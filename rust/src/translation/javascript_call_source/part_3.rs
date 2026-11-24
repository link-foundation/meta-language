// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::finish_java_script_call_source_control_scope;
use super::lower_java_script_call_source_argument_list;
use super::lower_java_script_call_source_array_expression;
use super::lower_java_script_call_source_binding_read_expression;
use super::lower_java_script_call_source_binding_scope;
use super::lower_java_script_call_source_binding_write_expression;
use super::lower_java_script_call_source_branch;
use super::lower_java_script_call_source_call_expression;
use super::lower_java_script_call_source_comparison_expression;
use super::lower_java_script_call_source_constructor_expression;
use super::lower_java_script_call_source_control_jump;
use super::lower_java_script_call_source_exception;
use super::lower_java_script_call_source_finally;
use super::lower_java_script_call_source_iterator_expression;
use super::lower_java_script_call_source_lexical_cell_expression;
use super::lower_java_script_call_source_loop;
use super::lower_java_script_call_source_object_expression;
use super::lower_java_script_call_source_property_expression;
use super::lower_java_script_call_source_property_query_expression;
use super::lower_java_script_call_source_switch;
use super::push_java_script_call_source_control_scope;
use super::read_java_script_call_constant_error;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value822, _, _, _, _) => {
            source_ml_source2_data_property_value822.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((depth > (128f64)) || ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value824, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value824
            }
        }) >= (65536f64))) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its structural budget")
        } else {
            {
                let state = {
                    let source_ml_source2_nominal_field827 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value828, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value828.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field829 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value830, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value830
                            }
                        };
                        {
                            let source_ml_source2_nominal_field831 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value832, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value832
                                }
                            };
                            {
                                let source_ml_source2_nominal_field833 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value834, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value834
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field835 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value836, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value836
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field837 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value838, _, _, _, _) => {
                                                source_ml_source2_data_property_value838.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field839 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value840, _, _, _) => {
                                                    source_ml_source2_data_property_value840.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field841 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value842, _, _) => {
                                                        source_ml_source2_data_property_value842.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field843 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value844, _) => {
                                                            source_ml_source2_data_property_value844.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field845 = match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value846) => {
                                                                source_ml_source2_data_property_value846.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override847 = (match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value825, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value825
                                                                }
                                                            } + 1f64);
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field827.clone(), source_ml_source2_nominal_field829, source_ml_source2_nominal_override847, source_ml_source2_nominal_field833, source_ml_source2_nominal_field835, source_ml_source2_nominal_field837.clone(), source_ml_source2_nominal_field839.clone(), source_ml_source2_nominal_field841.clone(), source_ml_source2_nominal_field843.clone(), source_ml_source2_nominal_field845.clone())
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
                                        let source_ml_source2_nominal_field1026 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1027, _) => {
                                                let source_ml_source2_data_property_value1027 = *source_ml_source2_data_property_value1027;
                                                source_ml_source2_data_property_value1027.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field1028 = match result.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1029) => {
                                                    source_ml_source2_data_property_value1029
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override1030 = self::append_java_script_call_source_instruction(match result.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1023, _) => {
                                                        let source_ml_source2_data_property_value1023 = *source_ml_source2_data_property_value1023;
                                                        source_ml_source2_data_property_value1023.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Constant(match result.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1024) => {
                                                        source_ml_source2_data_property_value1024
                                                    }
                                                }, Box::new(value.clone())));
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1030.clone()), source_ml_source2_nominal_field1028)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceNewTarget => {
                        {
                            let result_2 = self::allocate_java_script_call_source_register(state.clone());
                            {
                                let source_ml_source2_nominal_field1034 = match result_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1035, _) => {
                                        let source_ml_source2_data_property_value1035 = *source_ml_source2_data_property_value1035;
                                        source_ml_source2_data_property_value1035.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1036 = match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1037) => {
                                            source_ml_source2_data_property_value1037
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1038 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1031, _) => {
                                                let source_ml_source2_data_property_value1031 = *source_ml_source2_data_property_value1031;
                                                source_ml_source2_data_property_value1031.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::ReadNewTarget(match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1032) => {
                                                source_ml_source2_data_property_value1032
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1038.clone()), source_ml_source2_nominal_field1036)
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
                                let result_3 = self::allocate_java_script_call_source_register(state.clone());
                                {
                                    let source_ml_source2_nominal_field1042 = match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1043, _) => {
                                            let source_ml_source2_data_property_value1043 = *source_ml_source2_data_property_value1043;
                                            source_ml_source2_data_property_value1043.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field1044 = match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1045) => {
                                                source_ml_source2_data_property_value1045
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override1046 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1039, _) => {
                                                    let source_ml_source2_data_property_value1039 = *source_ml_source2_data_property_value1039;
                                                    source_ml_source2_data_property_value1039.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Argument(match result_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1040) => {
                                                    source_ml_source2_data_property_value1040
                                                }
                                            }, index));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1046.clone()), source_ml_source2_nominal_field1044)
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceDerivedReceiver => {
                        {
                            let result_4 = self::allocate_java_script_call_source_register(state.clone());
                            {
                                let source_ml_source2_nominal_field1235 = match result_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1236, _) => {
                                        let source_ml_source2_data_property_value1236 = *source_ml_source2_data_property_value1236;
                                        source_ml_source2_data_property_value1236.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1237 = match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1238) => {
                                            source_ml_source2_data_property_value1238
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1239 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1047, _) => {
                                                let source_ml_source2_data_property_value1047 = *source_ml_source2_data_property_value1047;
                                                source_ml_source2_data_property_value1047.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::ReadDerivedReceiver(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1232) => {
                                                source_ml_source2_data_property_value1232
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1239.clone()), source_ml_source2_nominal_field1237)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceErrorIntrinsic => {
                        {
                            let result_5 = self::allocate_java_script_call_source_register(state.clone());
                            {
                                let source_ml_source2_nominal_field1428 = match result_5.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1429, _) => {
                                        let source_ml_source2_data_property_value1429 = *source_ml_source2_data_property_value1429;
                                        source_ml_source2_data_property_value1429.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1430 = match result_5.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1431) => {
                                            source_ml_source2_data_property_value1431
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1432 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1240, _) => {
                                                let source_ml_source2_data_property_value1240 = *source_ml_source2_data_property_value1240;
                                                source_ml_source2_data_property_value1240.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::InvokeErrorIntrinsic(match result_5.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1426) => {
                                                source_ml_source2_data_property_value1426
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1432.clone()), source_ml_source2_nominal_field1430)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceConstruct(callee, arguments) => {
                        let callee = *callee;
                        self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(callee.clone()), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined))), arguments.clone(), String::from("construct"), false), bindings.clone(), state.clone(), (depth + 1f64))
                    }
                    self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                        self::lower_java_script_call_source_constructor_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                        self::lower_java_script_call_source_constructor_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                        self::lower_java_script_call_source_constructor_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                        self::lower_java_script_call_source_constructor_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceThrow(source_value) => {
                        let source_value = *source_value;
                        {
                            let value_2 = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let source_ml_source2_nominal_field1621 = match value_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1622, _) => {
                                        let source_ml_source2_data_property_value1622 = *source_ml_source2_data_property_value1622;
                                        source_ml_source2_data_property_value1622.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1623 = match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1624) => {
                                            source_ml_source2_data_property_value1624
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1625 = self::append_java_script_call_source_instruction(match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1433, _) => {
                                                let source_ml_source2_data_property_value1433 = *source_ml_source2_data_property_value1433;
                                                source_ml_source2_data_property_value1433.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Throw(match value_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1618) => {
                                                source_ml_source2_data_property_value1618
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1625.clone()), source_ml_source2_nominal_field1623)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceReturn(source_value_2) => {
                        let source_value_2 = *source_value_2;
                        {
                            let value_3 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let source_ml_source2_nominal_field1814 = match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1815, _) => {
                                        let source_ml_source2_data_property_value1815 = *source_ml_source2_data_property_value1815;
                                        source_ml_source2_data_property_value1815.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field1816 = match value_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1817) => {
                                            source_ml_source2_data_property_value1817
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override1818 = self::append_java_script_call_source_instruction(match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1626, _) => {
                                                let source_ml_source2_data_property_value1626 = *source_ml_source2_data_property_value1626;
                                                source_ml_source2_data_property_value1626.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Return(match value_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value1812) => {
                                                source_ml_source2_data_property_value1812
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override1818.clone()), source_ml_source2_nominal_field1816)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                        self::lower_java_script_call_source_loop(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceControlJump(_, _) => {
                        self::lower_java_script_call_source_control_jump(source.clone(), state.clone())
                    }
                    self::JavaScriptCallSource::SourceControlBlock(target, source_body, after) => {
                        let source_body = *source_body;
                        let after = *after;
                        {
                            let scoped = self::push_java_script_call_source_control_scope(state.clone(), target, "block");
                            {
                                let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), scoped.clone(), (depth + 1f64));
                                {
                                    let finished = self::finish_java_script_call_source_control_scope(match body.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1819, _) => {
                                            let source_ml_source2_data_property_value1819 = *source_ml_source2_data_property_value1819;
                                            source_ml_source2_data_property_value1819.clone()
                                        }
                                    }, state.clone(), target, { let ml_reference_array13 = match match body.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value1820, _) => {
                                            let source_ml_source2_data_property_value1820 = *source_ml_source2_data_property_value1820;
                                            source_ml_source2_data_property_value1820.clone()
                                        }
                                    } {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value1821, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1821.clone()
                                        }
                                    }; let ml_reference_projection13 = ml_reference_array13.borrow().len() as f64; ml_reference_projection13 }, (-1f64));
                                    self::lower_java_script_call_source_expression(after.clone(), bindings.clone(), finished.clone(), (depth + 1f64))
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
                        self::lower_java_script_call_source_switch(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
                        self::lower_java_script_call_source_binding_scope(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                        self::lower_java_script_call_source_lexical_cell_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                        self::lower_java_script_call_source_lexical_cell_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                        self::lower_java_script_call_source_binding_write_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
                        self::lower_java_script_call_source_iterator_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
                        self::lower_java_script_call_source_iterator_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
                        self::lower_java_script_call_source_iterator_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
                        self::lower_java_script_call_source_iterator_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
                        self::lower_java_script_call_source_iterator_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
                        self::lower_java_script_call_source_iterator_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceFinally(_, _) => {
                        self::lower_java_script_call_source_finally(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceTry(_, _, _) => {
                        self::lower_java_script_call_source_exception(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceBinding(_) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceLet(_, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                        self::lower_java_script_call_source_binding_read_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceIf(_, _, _) => {
                        self::lower_java_script_call_source_branch(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSameValue(_, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceRecur(_) => {
                        self::lower_java_script_call_source_comparison_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceAllocate(_) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceObject(_, _, _) => {
                        self::lower_java_script_call_source_object_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                        self::lower_java_script_call_source_array_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                        self::lower_java_script_call_source_array_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSet(_, _, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                        self::lower_java_script_call_source_property_query_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceGet(_, _, _) => {
                        self::lower_java_script_call_source_property_query_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                        self::lower_java_script_call_source_property_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceDelete(_, _) => {
                        self::lower_java_script_call_source_property_query_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceHas(_, _) => {
                        self::lower_java_script_call_source_property_query_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceCall(_, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                    self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                        self::lower_java_script_call_source_call_expression(source.clone(), bindings.clone(), state.clone(), depth)
                    }
                }
            }
        }
    }
}
