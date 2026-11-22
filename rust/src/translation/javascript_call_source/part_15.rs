// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml_array;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_exception(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceTry(source_identity, source_body, source_handler) => {
            let source_body = *source_body;
            let source_handler = *source_handler;
            if (!(self::test_java_script_call_code_coordinate(source_identity, 65536f64)) || { let ml_values106 = { let ml_reference_value107 = bindings.borrow().clone(); ml_reference_value107 }; let ml_callback106 = { let source_identity = source_identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value12190, _, _, _) => {
                    source_ml_source2_data_property_value12190
                }
            }) == source_identity) }) }; ml_values106.iter().cloned().enumerate().any(|(ml_index106, ml_value106)| ml_callback106(ml_value106)) }) {
                self::refuse_java_script_call_source(builder.clone(), "a catch binding owner must be unique in its lexical scope")
            } else {
                if ((match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12192, _, _, _, _, _) => {
                        source_ml_source2_data_property_value12192
                    }
                }) >= (1024f64)) {
                    self::refuse_java_script_call_source(builder.clone(), "source exception handlers exceed their explicit active handler budget")
                } else {
                    {
                        let caught = self::allocate_java_script_call_source_register(builder.clone());
                        {
                            let result = self::allocate_java_script_call_source_register(match caught.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12193, _) => {
                                    let source_ml_source2_data_property_value12193 = *source_ml_source2_data_property_value12193;
                                    source_ml_source2_data_property_value12193.clone()
                                }
                            });
                            {
                                let position = { let ml_reference_array108 = match match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12194, _) => {
                                        let source_ml_source2_data_property_value12194 = *source_ml_source2_data_property_value12194;
                                        source_ml_source2_data_property_value12194.clone()
                                    }
                                } {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12195, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12195.clone()
                                    }
                                }; let ml_reference_projection108 = ml_reference_array108.borrow().len() as f64; ml_reference_projection108 };
                                {
                                    let protected_builder = self::append_java_script_call_source_instruction({
                                        let source_ml_source2_nominal_spread12198 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12196, _) => {
                                                let source_ml_source2_data_property_value12196 = *source_ml_source2_data_property_value12196;
                                                source_ml_source2_data_property_value12196.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12199 = match source_ml_source2_nominal_spread12198.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12200, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12200.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12201 = match source_ml_source2_nominal_spread12198.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12202, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12202
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12203 = match source_ml_source2_nominal_spread12198.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12204, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12204
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12205 = match source_ml_source2_nominal_spread12198.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12206, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12206
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12207 = match source_ml_source2_nominal_spread12198.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12208, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12208
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12209 = match source_ml_source2_nominal_spread12198.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12210, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12210.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12211 = match source_ml_source2_nominal_spread12198.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12212, _, _, _) => {
                                                                            source_ml_source2_data_property_value12212.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12213 = match source_ml_source2_nominal_spread12198.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12214, _, _) => {
                                                                                source_ml_source2_data_property_value12214.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12215 = match source_ml_source2_nominal_spread12198.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12216, _) => {
                                                                                    source_ml_source2_data_property_value12216.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12217 = match source_ml_source2_nominal_spread12198.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12218) => {
                                                                                        source_ml_source2_data_property_value12218.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override12219 = (match builder.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12197, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12197
                                                                                        }
                                                                                    } + 1f64);
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12199.clone(), source_ml_source2_nominal_field12201, source_ml_source2_nominal_field12203, source_ml_source2_nominal_field12205, source_ml_source2_nominal_override12219, source_ml_source2_nominal_field12209.clone(), source_ml_source2_nominal_field12211.clone(), source_ml_source2_nominal_field12213.clone(), source_ml_source2_nominal_field12215.clone(), source_ml_source2_nominal_field12217.clone())
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
                                    }, self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12220) => {
                                            source_ml_source2_data_property_value12220
                                        }
                                    }, 0f64));
                                    {
                                        let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                        {
                                            let popped = self::append_java_script_call_source_instruction({
                                                let source_ml_source2_nominal_spread12223 = match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12221, _) => {
                                                        let source_ml_source2_data_property_value12221 = *source_ml_source2_data_property_value12221;
                                                        source_ml_source2_data_property_value12221.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12224 = match source_ml_source2_nominal_spread12223.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12225, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12225.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12226 = match source_ml_source2_nominal_spread12223.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12227, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12227
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12228 = match source_ml_source2_nominal_spread12223.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12229, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12229
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12230 = match source_ml_source2_nominal_spread12223.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12231, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12231
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12232 = match source_ml_source2_nominal_spread12223.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12233, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12233
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12234 = match source_ml_source2_nominal_spread12223.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12235, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12235.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12236 = match source_ml_source2_nominal_spread12223.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12237, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12237.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12238 = match source_ml_source2_nominal_spread12223.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12239, _, _) => {
                                                                                        source_ml_source2_data_property_value12239.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12240 = match source_ml_source2_nominal_spread12223.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12241, _) => {
                                                                                            source_ml_source2_data_property_value12241.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12242 = match source_ml_source2_nominal_spread12223.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12243) => {
                                                                                                source_ml_source2_data_property_value12243.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override12244 = match builder.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12222, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12222
                                                                                                }
                                                                                            };
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12224.clone(), source_ml_source2_nominal_field12226, source_ml_source2_nominal_field12228, source_ml_source2_nominal_field12230, source_ml_source2_nominal_override12244, source_ml_source2_nominal_field12234.clone(), source_ml_source2_nominal_field12236.clone(), source_ml_source2_nominal_field12238.clone(), source_ml_source2_nominal_field12240.clone(), source_ml_source2_nominal_field12242.clone())
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
                                            }, self::JavaScriptCallInstruction::PopHandler);
                                            {
                                                let copied = self::append_java_script_call_source_instruction(popped.clone(), self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12245) => {
                                                        source_ml_source2_data_property_value12245
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12246) => {
                                                        source_ml_source2_data_property_value12246
                                                    }
                                                }));
                                                {
                                                    let jump = { let ml_reference_array109 = match copied.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12247, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12247.clone()
                                                        }
                                                    }; let ml_reference_projection109 = ml_reference_array109.borrow().len() as f64; ml_reference_projection109 };
                                                    {
                                                        let alternative = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                        {
                                                            let target = { let ml_reference_array110 = match alternative.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12248, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12248.clone()
                                                                }
                                                            }; let ml_reference_projection110 = ml_reference_array110.borrow().len() as f64; ml_reference_projection110 };
                                                            {
                                                                let handler = self::lower_java_script_call_source_expression(source_handler.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value12254 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                        let source_ml_source2_constructor_field12251 = match caught.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12250) => {
                                                                                source_ml_source2_data_property_value12250
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_identity, source_ml_source2_constructor_field12251, true, None::<(Option<bool>,)>)
                                                                    }]));
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value111 = bindings.borrow().clone(); ml_reference_value111 }), { let ml_reference_value112 = source_ml_source2_concat_value12254.borrow().clone(); ml_reference_value112 })
                                                                })), if match (match alternative.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12255, _, _) => {
                                                                        ((source_ml_source2_data_property_value12255.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    alternative.clone()
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_nominal_field12258 = match alternative.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12259, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12259.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12260 = match alternative.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12261, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12261
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12262 = match alternative.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12263, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12263
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12264 = match alternative.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12265, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12265
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12266 = match alternative.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12267, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12267
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12268 = match alternative.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12269, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12269.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12270 = match alternative.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12271, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12271.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12272 = match alternative.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12273, _, _) => {
                                                                                                            source_ml_source2_data_property_value12273.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12274 = match alternative.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12275, _) => {
                                                                                                                source_ml_source2_data_property_value12275.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12276 = match alternative.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12277) => {
                                                                                                                    source_ml_source2_data_property_value12277.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override12278 = (match builder.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12256, _, _) => {
                                                                                                                        ((source_ml_source2_data_property_value12256.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                    }
                                                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12258.clone(), source_ml_source2_nominal_field12260, source_ml_source2_nominal_field12262, source_ml_source2_nominal_field12264, source_ml_source2_nominal_field12266, source_ml_source2_nominal_field12268.clone(), source_ml_source2_nominal_field12270.clone(), Some((Some(source_ml_source2_nominal_override12278.clone()),)), source_ml_source2_nominal_field12274.clone(), source_ml_source2_nominal_field12276.clone())
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
                                                                }, (depth + 1f64));
                                                                {
                                                                    let finished = self::append_java_script_call_source_instruction(match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12279, _) => {
                                                                            let source_ml_source2_data_property_value12279 = *source_ml_source2_data_property_value12279;
                                                                            source_ml_source2_data_property_value12279.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12280) => {
                                                                            source_ml_source2_data_property_value12280
                                                                        }
                                                                    }, match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12281) => {
                                                                            source_ml_source2_data_property_value12281
                                                                        }
                                                                    }));
                                                                    {
                                                                        let end = { let ml_reference_array113 = match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12282, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12282.clone()
                                                                            }
                                                                        }; let ml_reference_projection113 = ml_reference_array113.borrow().len() as f64; ml_reference_projection113 };
                                                                        if ((match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12284, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12284.clone()
                                                                            }
                                                                        }) != "") {
                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(finished.clone()), 0f64)
                                                                        } else {
                                                                            {
                                                                                let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values114 = { let ml_reference_value115 = (match finished.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12285, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12285.clone()
                                                                                    }
                                                                                }).borrow().clone(); ml_reference_value115 }; let ml_callback114 = { let position = position; let caught = caught.clone(); let target = target; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                                    self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12286) => {
                                                                                            source_ml_source2_data_property_value12286
                                                                                        }
                                                                                    }, target)
                                                                                } else {
                                                                                    if (index == jump) {
                                                                                        self::JavaScriptCallInstruction::Jump(end)
                                                                                    } else {
                                                                                        entry.clone()
                                                                                    }
                                                                                } }) }; ml_values114.iter().cloned().enumerate().map(|(ml_index114, ml_value114)| ml_callback114(ml_value114, (ml_index114 as f64))).collect::<Vec<_>>() }));
                                                                                {
                                                                                    let updated = {
                                                                                        let source_ml_source2_nominal_field12288 = match finished.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12289, _, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12289.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12290 = match finished.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12291, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12291
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12292 = match finished.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12293, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12293
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12294 = match finished.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12295, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12295
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12296 = match finished.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12297, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12297
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12298 = match finished.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12299, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12299.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12300 = match finished.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12301, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value12301.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12302 = match finished.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12303, _, _) => {
                                                                                                                            source_ml_source2_data_property_value12303.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12304 = match finished.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12305, _) => {
                                                                                                                                source_ml_source2_data_property_value12305.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12306 = match finished.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12307) => {
                                                                                                                                    source_ml_source2_data_property_value12307.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field12290, source_ml_source2_nominal_field12292, source_ml_source2_nominal_field12294, source_ml_source2_nominal_field12296, source_ml_source2_nominal_field12298.clone(), source_ml_source2_nominal_field12300.clone(), source_ml_source2_nominal_field12302.clone(), source_ml_source2_nominal_field12304.clone(), source_ml_source2_nominal_field12306.clone())
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
                                                                                    {
                                                                                        let continuation = if (match (match match body.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12309, _) => {
                                                                                                let source_ml_source2_data_property_value12309 = *source_ml_source2_data_property_value12309;
                                                                                                source_ml_source2_data_property_value12309.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12310, _, _) => {
                                                                                                ((source_ml_source2_data_property_value12310.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && match (match match handler.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12311, _) => {
                                                                                                let source_ml_source2_data_property_value12311 = *source_ml_source2_data_property_value12311;
                                                                                                source_ml_source2_data_property_value12311.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12312, _, _) => {
                                                                                                ((source_ml_source2_data_property_value12312.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                            updated.clone()
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source2_nominal_spread12319 = updated.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12320 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12321, _, _, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12321.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12322 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12323, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12323
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12324 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12325, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12325
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12326 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12327, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value12327
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12328 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12329, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value12329
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12330 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12331, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value12331.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12332 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12333, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value12333.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12334 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12335, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value12335.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12336 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12337, _) => {
                                                                                                                                            source_ml_source2_data_property_value12337.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12338 = match source_ml_source2_nominal_spread12319.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12339) => {
                                                                                                                                                source_ml_source2_data_property_value12339.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override12340 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values116 = { let ml_reference_value117 = ((match match body.clone() {
                                                                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12313, _) => {
                                                                                                                                                    let source_ml_source2_data_property_value12313 = *source_ml_source2_data_property_value12313;
                                                                                                                                                    source_ml_source2_data_property_value12313.clone()
                                                                                                                                                }
                                                                                                                                            } {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12314, _, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value12314.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value117 }; let ml_callback116 = { let handler = handler.clone(); std::rc::Rc::new(move |identity: f64| -> bool { self::ml_array::includes(&{ let ml_reference_value118 = ((match match handler.clone() {
                                                                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12317, _) => {
                                                                                                                                                    let source_ml_source2_data_property_value12317 = *source_ml_source2_data_property_value12317;
                                                                                                                                                    source_ml_source2_data_property_value12317.clone()
                                                                                                                                                }
                                                                                                                                            } {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12318, _, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value12318.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value118 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) }) }; ml_values116.iter().cloned().enumerate().filter_map(|(ml_index116, ml_value116)| if ml_callback116(ml_value116.clone()) { Some(ml_value116) } else { None }).collect::<Vec<_>>() }));
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12320.clone(), source_ml_source2_nominal_field12322, source_ml_source2_nominal_field12324, source_ml_source2_nominal_field12326, source_ml_source2_nominal_field12328, source_ml_source2_nominal_field12330.clone(), source_ml_source2_nominal_field12332.clone(), Some((Some(source_ml_source2_nominal_override12340.clone()),)), source_ml_source2_nominal_field12336.clone(), source_ml_source2_nominal_field12338.clone())
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
                                                                                        };
                                                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(continuation.clone()), match result.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12341) => {
                                                                                                source_ml_source2_data_property_value12341
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
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
    }
}
