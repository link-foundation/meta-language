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
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value12244, _, _, _) => {
                    source_ml_source2_data_property_value12244
                }
            }) == source_identity) }) }; ml_values106.iter().cloned().enumerate().any(|(ml_index106, ml_value106)| ml_callback106(ml_value106)) }) {
                self::refuse_java_script_call_source(builder.clone(), "a catch binding owner must be unique in its lexical scope")
            } else {
                if ((match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12246, _, _, _, _, _) => {
                        source_ml_source2_data_property_value12246
                    }
                }) >= (1024f64)) {
                    self::refuse_java_script_call_source(builder.clone(), "source exception handlers exceed their explicit active handler budget")
                } else {
                    {
                        let caught = self::allocate_java_script_call_source_register(builder.clone());
                        {
                            let result = self::allocate_java_script_call_source_register(match caught.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12247, _) => {
                                    let source_ml_source2_data_property_value12247 = *source_ml_source2_data_property_value12247;
                                    source_ml_source2_data_property_value12247.clone()
                                }
                            });
                            {
                                let position = { let ml_reference_array108 = match match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12248, _) => {
                                        let source_ml_source2_data_property_value12248 = *source_ml_source2_data_property_value12248;
                                        source_ml_source2_data_property_value12248.clone()
                                    }
                                } {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12249, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12249.clone()
                                    }
                                }; let ml_reference_projection108 = ml_reference_array108.borrow().len() as f64; ml_reference_projection108 };
                                {
                                    let protected_builder = self::append_java_script_call_source_instruction({
                                        let source_ml_source2_nominal_spread12252 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12250, _) => {
                                                let source_ml_source2_data_property_value12250 = *source_ml_source2_data_property_value12250;
                                                source_ml_source2_data_property_value12250.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12253 = match source_ml_source2_nominal_spread12252.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12254, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12254.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12255 = match source_ml_source2_nominal_spread12252.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12256, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12256
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12257 = match source_ml_source2_nominal_spread12252.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12258, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12258
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12259 = match source_ml_source2_nominal_spread12252.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12260, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12260
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12261 = match source_ml_source2_nominal_spread12252.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12262, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12262
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12263 = match source_ml_source2_nominal_spread12252.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12264, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12264.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12265 = match source_ml_source2_nominal_spread12252.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12266, _, _, _) => {
                                                                            source_ml_source2_data_property_value12266.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12267 = match source_ml_source2_nominal_spread12252.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12268, _, _) => {
                                                                                source_ml_source2_data_property_value12268.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12269 = match source_ml_source2_nominal_spread12252.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12270, _) => {
                                                                                    source_ml_source2_data_property_value12270.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12271 = match source_ml_source2_nominal_spread12252.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12272) => {
                                                                                        source_ml_source2_data_property_value12272.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override12273 = (match builder.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12251, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12251
                                                                                        }
                                                                                    } + 1f64);
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12253.clone(), source_ml_source2_nominal_field12255, source_ml_source2_nominal_field12257, source_ml_source2_nominal_field12259, source_ml_source2_nominal_override12273, source_ml_source2_nominal_field12263.clone(), source_ml_source2_nominal_field12265.clone(), source_ml_source2_nominal_field12267.clone(), source_ml_source2_nominal_field12269.clone(), source_ml_source2_nominal_field12271.clone())
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
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12274) => {
                                            source_ml_source2_data_property_value12274
                                        }
                                    }, 0f64));
                                    {
                                        let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                        {
                                            let popped = self::append_java_script_call_source_instruction({
                                                let source_ml_source2_nominal_spread12277 = match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12275, _) => {
                                                        let source_ml_source2_data_property_value12275 = *source_ml_source2_data_property_value12275;
                                                        source_ml_source2_data_property_value12275.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12278 = match source_ml_source2_nominal_spread12277.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12279, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12279.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12280 = match source_ml_source2_nominal_spread12277.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12281, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12281
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12282 = match source_ml_source2_nominal_spread12277.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12283, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12283
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12284 = match source_ml_source2_nominal_spread12277.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12285, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12285
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12286 = match source_ml_source2_nominal_spread12277.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12287, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12287
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12288 = match source_ml_source2_nominal_spread12277.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12289, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12289.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12290 = match source_ml_source2_nominal_spread12277.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12291, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12291.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12292 = match source_ml_source2_nominal_spread12277.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12293, _, _) => {
                                                                                        source_ml_source2_data_property_value12293.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12294 = match source_ml_source2_nominal_spread12277.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12295, _) => {
                                                                                            source_ml_source2_data_property_value12295.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12296 = match source_ml_source2_nominal_spread12277.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12297) => {
                                                                                                source_ml_source2_data_property_value12297.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override12298 = match builder.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12276, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12276
                                                                                                }
                                                                                            };
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12278.clone(), source_ml_source2_nominal_field12280, source_ml_source2_nominal_field12282, source_ml_source2_nominal_field12284, source_ml_source2_nominal_override12298, source_ml_source2_nominal_field12288.clone(), source_ml_source2_nominal_field12290.clone(), source_ml_source2_nominal_field12292.clone(), source_ml_source2_nominal_field12294.clone(), source_ml_source2_nominal_field12296.clone())
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
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12299) => {
                                                        source_ml_source2_data_property_value12299
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12300) => {
                                                        source_ml_source2_data_property_value12300
                                                    }
                                                }));
                                                {
                                                    let jump = { let ml_reference_array109 = match copied.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12301, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12301.clone()
                                                        }
                                                    }; let ml_reference_projection109 = ml_reference_array109.borrow().len() as f64; ml_reference_projection109 };
                                                    {
                                                        let alternative = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                        {
                                                            let target = { let ml_reference_array110 = match alternative.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12302, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12302.clone()
                                                                }
                                                            }; let ml_reference_projection110 = ml_reference_array110.borrow().len() as f64; ml_reference_projection110 };
                                                            {
                                                                let handler = self::lower_java_script_call_source_expression(source_handler.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value12308 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                        let source_ml_source2_constructor_field12305 = match caught.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12304) => {
                                                                                source_ml_source2_data_property_value12304
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_identity, source_ml_source2_constructor_field12305, true, None::<(Option<bool>,)>)
                                                                    }]));
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value111 = bindings.borrow().clone(); ml_reference_value111 }), { let ml_reference_value112 = source_ml_source2_concat_value12308.borrow().clone(); ml_reference_value112 })
                                                                })), if match (match alternative.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12309, _, _) => {
                                                                        ((source_ml_source2_data_property_value12309.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    alternative.clone()
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_nominal_field12312 = match alternative.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12313, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12313.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12314 = match alternative.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12315, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12315
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12316 = match alternative.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12317, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12317
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12318 = match alternative.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12319, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12319
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12320 = match alternative.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12321, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12321
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12322 = match alternative.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12323, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12323.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12324 = match alternative.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12325, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12325.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12326 = match alternative.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12327, _, _) => {
                                                                                                            source_ml_source2_data_property_value12327.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12328 = match alternative.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12329, _) => {
                                                                                                                source_ml_source2_data_property_value12329.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12330 = match alternative.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12331) => {
                                                                                                                    source_ml_source2_data_property_value12331.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override12332 = (match builder.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12310, _, _) => {
                                                                                                                        ((source_ml_source2_data_property_value12310.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                    }
                                                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12312.clone(), source_ml_source2_nominal_field12314, source_ml_source2_nominal_field12316, source_ml_source2_nominal_field12318, source_ml_source2_nominal_field12320, source_ml_source2_nominal_field12322.clone(), source_ml_source2_nominal_field12324.clone(), Some((Some(source_ml_source2_nominal_override12332.clone()),)), source_ml_source2_nominal_field12328.clone(), source_ml_source2_nominal_field12330.clone())
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
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12333, _) => {
                                                                            let source_ml_source2_data_property_value12333 = *source_ml_source2_data_property_value12333;
                                                                            source_ml_source2_data_property_value12333.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12334) => {
                                                                            source_ml_source2_data_property_value12334
                                                                        }
                                                                    }, match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12335) => {
                                                                            source_ml_source2_data_property_value12335
                                                                        }
                                                                    }));
                                                                    {
                                                                        let end = { let ml_reference_array113 = match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12336, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12336.clone()
                                                                            }
                                                                        }; let ml_reference_projection113 = ml_reference_array113.borrow().len() as f64; ml_reference_projection113 };
                                                                        if ((match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12338, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12338.clone()
                                                                            }
                                                                        }) != "") {
                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(finished.clone()), 0f64)
                                                                        } else {
                                                                            {
                                                                                let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values114 = { let ml_reference_value115 = (match finished.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12339, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12339.clone()
                                                                                    }
                                                                                }).borrow().clone(); ml_reference_value115 }; let ml_callback114 = { let position = position; let caught = caught.clone(); let target = target; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                                    self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12340) => {
                                                                                            source_ml_source2_data_property_value12340
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
                                                                                        let source_ml_source2_nominal_field12342 = match finished.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12343, _, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12343.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12344 = match finished.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12345, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12345
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12346 = match finished.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12347, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12347
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12348 = match finished.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12349, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12349
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12350 = match finished.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12351, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12351
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12352 = match finished.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12353, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12353.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12354 = match finished.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12355, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value12355.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12356 = match finished.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12357, _, _) => {
                                                                                                                            source_ml_source2_data_property_value12357.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12358 = match finished.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12359, _) => {
                                                                                                                                source_ml_source2_data_property_value12359.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12360 = match finished.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12361) => {
                                                                                                                                    source_ml_source2_data_property_value12361.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field12344, source_ml_source2_nominal_field12346, source_ml_source2_nominal_field12348, source_ml_source2_nominal_field12350, source_ml_source2_nominal_field12352.clone(), source_ml_source2_nominal_field12354.clone(), source_ml_source2_nominal_field12356.clone(), source_ml_source2_nominal_field12358.clone(), source_ml_source2_nominal_field12360.clone())
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
                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12363, _) => {
                                                                                                let source_ml_source2_data_property_value12363 = *source_ml_source2_data_property_value12363;
                                                                                                source_ml_source2_data_property_value12363.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12364, _, _) => {
                                                                                                ((source_ml_source2_data_property_value12364.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && match (match match handler.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12365, _) => {
                                                                                                let source_ml_source2_data_property_value12365 = *source_ml_source2_data_property_value12365;
                                                                                                source_ml_source2_data_property_value12365.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12366, _, _) => {
                                                                                                ((source_ml_source2_data_property_value12366.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                            updated.clone()
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source2_nominal_spread12373 = updated.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12374 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12375, _, _, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12375.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12376 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12377, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12377
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12378 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12379, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12379
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12380 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12381, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value12381
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12382 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12383, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value12383
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12384 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12385, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value12385.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12386 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12387, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value12387.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12388 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12389, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value12389.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12390 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12391, _) => {
                                                                                                                                            source_ml_source2_data_property_value12391.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12392 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12393) => {
                                                                                                                                                source_ml_source2_data_property_value12393.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override12394 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values116 = { let ml_reference_value117 = ((match match body.clone() {
                                                                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12367, _) => {
                                                                                                                                                    let source_ml_source2_data_property_value12367 = *source_ml_source2_data_property_value12367;
                                                                                                                                                    source_ml_source2_data_property_value12367.clone()
                                                                                                                                                }
                                                                                                                                            } {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12368, _, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value12368.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value117 }; let ml_callback116 = { let handler = handler.clone(); std::rc::Rc::new(move |identity: f64| -> bool { self::ml_array::includes(&{ let ml_reference_value118 = ((match match handler.clone() {
                                                                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12371, _) => {
                                                                                                                                                    let source_ml_source2_data_property_value12371 = *source_ml_source2_data_property_value12371;
                                                                                                                                                    source_ml_source2_data_property_value12371.clone()
                                                                                                                                                }
                                                                                                                                            } {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12372, _, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value12372.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value118 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) }) }; ml_values116.iter().cloned().enumerate().filter_map(|(ml_index116, ml_value116)| if ml_callback116(ml_value116.clone()) { Some(ml_value116) } else { None }).collect::<Vec<_>>() }));
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12374.clone(), source_ml_source2_nominal_field12376, source_ml_source2_nominal_field12378, source_ml_source2_nominal_field12380, source_ml_source2_nominal_field12382, source_ml_source2_nominal_field12384.clone(), source_ml_source2_nominal_field12386.clone(), Some((Some(source_ml_source2_nominal_override12394.clone()),)), source_ml_source2_nominal_field12390.clone(), source_ml_source2_nominal_field12392.clone())
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
                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12395) => {
                                                                                                source_ml_source2_data_property_value12395
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
