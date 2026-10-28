// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_compile_java_script_call_source_loop102;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_exception(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source2_f570) => {
            let source_ml_source2_f570 = *source_ml_source2_f570;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceArgument(source_ml_source2_f571) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceThrow(source_ml_source2_f572) => {
            let source_ml_source2_f572 = *source_ml_source2_f572;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceTry(identity, source_body, source_handler) => {
            let source_body = *source_body;
            let source_handler = *source_handler;
            if (!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || { let ml_values2 = bindings.clone(); let ml_callback2 = { let identity = identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value599, _) => {
                    source_ml_source2_data_property_value599
                }
            }) == identity) }) }; ml_values2.iter().cloned().enumerate().any(|(ml_index2, ml_value2)| ml_callback2(ml_value2)) }) {
                self::refuse_java_script_call_source(builder.clone(), "a catch binding owner must be unique in its lexical scope")
            } else {
                if ((match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value601, _) => {
                        source_ml_source2_data_property_value601
                    }
                }) >= (1024f64)) {
                    self::refuse_java_script_call_source(builder.clone(), "source exception handlers exceed their explicit active handler budget")
                } else {
                    {
                        let caught = self::allocate_java_script_call_source_register(builder.clone());
                        {
                            let result = self::allocate_java_script_call_source_register(match caught.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value602, _) => {
                                    let source_ml_source2_data_property_value602 = *source_ml_source2_data_property_value602;
                                    source_ml_source2_data_property_value602.clone()
                                }
                            });
                            {
                                let position = ((match match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value603, _) => {
                                        let source_ml_source2_data_property_value603 = *source_ml_source2_data_property_value603;
                                        source_ml_source2_data_property_value603.clone()
                                    }
                                } {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value604, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value604
                                    }
                                }).len() as f64);
                                {
                                    let protected_builder = self::append_java_script_call_source_instruction({
                                        let source_ml_source2_nominal_spread607 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value605, _) => {
                                                let source_ml_source2_data_property_value605 = *source_ml_source2_data_property_value605;
                                                source_ml_source2_data_property_value605.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field608 = match source_ml_source2_nominal_spread607.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value609, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value609
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field610 = match source_ml_source2_nominal_spread607.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value611, _, _, _, _) => {
                                                        source_ml_source2_data_property_value611
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field612 = match source_ml_source2_nominal_spread607.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value613, _, _, _) => {
                                                            source_ml_source2_data_property_value613
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field614 = match source_ml_source2_nominal_spread607.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value615, _, _) => {
                                                                source_ml_source2_data_property_value615
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field616 = match source_ml_source2_nominal_spread607.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value617, _) => {
                                                                    source_ml_source2_data_property_value617
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field618 = match source_ml_source2_nominal_spread607.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value619) => {
                                                                        source_ml_source2_data_property_value619.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override620 = (match builder.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value606, _) => {
                                                                            source_ml_source2_data_property_value606
                                                                        }
                                                                    } + 1f64);
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field608, source_ml_source2_nominal_field610, source_ml_source2_nominal_field612, source_ml_source2_nominal_field614, source_ml_source2_nominal_override620, source_ml_source2_nominal_field618.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }, self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value621) => {
                                            source_ml_source2_data_property_value621
                                        }
                                    }, 0f64));
                                    {
                                        let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                        {
                                            let popped = self::append_java_script_call_source_instruction({
                                                let source_ml_source2_nominal_spread624 = match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value622, _) => {
                                                        let source_ml_source2_data_property_value622 = *source_ml_source2_data_property_value622;
                                                        source_ml_source2_data_property_value622.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field625 = match source_ml_source2_nominal_spread624.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value626, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value626
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field627 = match source_ml_source2_nominal_spread624.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value628, _, _, _, _) => {
                                                                source_ml_source2_data_property_value628
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field629 = match source_ml_source2_nominal_spread624.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value630, _, _, _) => {
                                                                    source_ml_source2_data_property_value630
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field631 = match source_ml_source2_nominal_spread624.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value632, _, _) => {
                                                                        source_ml_source2_data_property_value632
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field633 = match source_ml_source2_nominal_spread624.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value634, _) => {
                                                                            source_ml_source2_data_property_value634
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field635 = match source_ml_source2_nominal_spread624.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value636) => {
                                                                                source_ml_source2_data_property_value636.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override637 = match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value623, _) => {
                                                                                    source_ml_source2_data_property_value623
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field625, source_ml_source2_nominal_field627, source_ml_source2_nominal_field629, source_ml_source2_nominal_field631, source_ml_source2_nominal_override637, source_ml_source2_nominal_field635.clone())
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
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value638) => {
                                                        source_ml_source2_data_property_value638
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value639) => {
                                                        source_ml_source2_data_property_value639
                                                    }
                                                }));
                                                {
                                                    let jump = ((match &copied { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value640, _, _, _, _, _) => source_ml_source2_data_property_value640.as_slice() }).len() as f64);
                                                    {
                                                        let alternative = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                        {
                                                            let target = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value641, _, _, _, _, _) => source_ml_source2_data_property_value641.as_slice() }).len() as f64);
                                                            {
                                                                let handler = self::lower_java_script_call_source_expression(source_handler.clone(), {
                                                                    let source_ml_source2_concat_value643 = bindings;
                                                                    {
                                                                        let source_ml_source2_concat_value644 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, match caught.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value642) => {
                                                                                source_ml_source2_data_property_value642
                                                                            }
                                                                        })];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value643), source_ml_source2_concat_value644)
                                                                    }
                                                                }, alternative.clone(), (depth + 1f64));
                                                                {
                                                                    let finished = self::append_java_script_call_source_instruction(match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value645, _) => {
                                                                            let source_ml_source2_data_property_value645 = *source_ml_source2_data_property_value645;
                                                                            source_ml_source2_data_property_value645.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value646) => {
                                                                            source_ml_source2_data_property_value646
                                                                        }
                                                                    }, match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value647) => {
                                                                            source_ml_source2_data_property_value647
                                                                        }
                                                                    }));
                                                                    {
                                                                        let end = ((match &finished { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value648, _, _, _, _, _) => source_ml_source2_data_property_value648.as_slice() }).len() as f64);
                                                                        if ((match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value650) => {
                                                                                source_ml_source2_data_property_value650.clone()
                                                                            }
                                                                        }) != "") {
                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(finished.clone()), 0f64)
                                                                        } else {
                                                                            {
                                                                                let instructions = { let ml_values3 = match finished.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value651, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value651
                                                                                    }
                                                                                }; let ml_callback3 = { let position = position; let caught = caught.clone(); let target = target; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                                    self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value652) => {
                                                                                            source_ml_source2_data_property_value652
                                                                                        }
                                                                                    }, target)
                                                                                } else {
                                                                                    if (index == jump) {
                                                                                        self::JavaScriptCallInstruction::Jump(end)
                                                                                    } else {
                                                                                        entry.clone()
                                                                                    }
                                                                                } }) }; ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() };
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                                                    let source_ml_source2_nominal_spread653 = finished.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field654 = match source_ml_source2_nominal_spread653.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value655, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value655
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field656 = match source_ml_source2_nominal_spread653.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value657, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value657
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field658 = match source_ml_source2_nominal_spread653.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value659, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value659
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field660 = match source_ml_source2_nominal_spread653.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value661, _, _) => {
                                                                                                            source_ml_source2_data_property_value661
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field662 = match source_ml_source2_nominal_spread653.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value663, _) => {
                                                                                                                source_ml_source2_data_property_value663
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field664 = match source_ml_source2_nominal_spread653.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value665) => {
                                                                                                                    source_ml_source2_data_property_value665.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override666 = instructions;
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override666, source_ml_source2_nominal_field656, source_ml_source2_nominal_field658, source_ml_source2_nominal_field660, source_ml_source2_nominal_field662, source_ml_source2_nominal_field664.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }), match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value667) => {
                                                                                        source_ml_source2_data_property_value667
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
        self::JavaScriptCallSource::SourceBinding(source_ml_source2_f573) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source2_f574, source_ml_source2_f575, source_ml_source2_f576) => {
            let source_ml_source2_f575 = *source_ml_source2_f575;
            let source_ml_source2_f576 = *source_ml_source2_f576;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIf(source_ml_source2_f577, source_ml_source2_f578, source_ml_source2_f579) => {
            let source_ml_source2_f577 = *source_ml_source2_f577;
            let source_ml_source2_f578 = *source_ml_source2_f578;
            let source_ml_source2_f579 = *source_ml_source2_f579;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f580, source_ml_source2_f581, source_ml_source2_f582) => {
            let source_ml_source2_f581 = *source_ml_source2_f581;
            let source_ml_source2_f582 = *source_ml_source2_f582;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source2_f583, source_ml_source2_f584) => {
            let source_ml_source2_f583 = *source_ml_source2_f583;
            let source_ml_source2_f584 = *source_ml_source2_f584;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source2_f585, source_ml_source2_f586) => {
            let source_ml_source2_f585 = *source_ml_source2_f585;
            let source_ml_source2_f586 = *source_ml_source2_f586;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source2_f587, source_ml_source2_f588) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source2_f589) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source2_f590, source_ml_source2_f591, source_ml_source2_f592) => {
            let source_ml_source2_f590 = *source_ml_source2_f590;
            let source_ml_source2_f591 = *source_ml_source2_f591;
            let source_ml_source2_f592 = *source_ml_source2_f592;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source2_f593, source_ml_source2_f594) => {
            let source_ml_source2_f593 = *source_ml_source2_f593;
            let source_ml_source2_f594 = *source_ml_source2_f594;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source2_f595, source_ml_source2_f596, source_ml_source2_f597) => {
            let source_ml_source2_f595 = *source_ml_source2_f595;
            let source_ml_source2_f596 = *source_ml_source2_f596;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source2_f668) => {
            let source_ml_source2_f668 = *source_ml_source2_f668;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceArgument(source_ml_source2_f669) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceThrow(source_ml_source2_f670) => {
            let source_ml_source2_f670 = *source_ml_source2_f670;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceTry(source_ml_source2_f671, source_ml_source2_f672, source_ml_source2_f673) => {
            let source_ml_source2_f672 = *source_ml_source2_f672;
            let source_ml_source2_f673 = *source_ml_source2_f673;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(source_ml_source2_f674) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source2_f675, source_ml_source2_f676, source_ml_source2_f677) => {
            let source_ml_source2_f676 = *source_ml_source2_f676;
            let source_ml_source2_f677 = *source_ml_source2_f677;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceIf(source_condition, source_yes, source_no) => {
            let source_condition = *source_condition;
            let source_yes = *source_yes;
            let source_no = *source_no;
            {
                let condition = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                {
                    let result = self::allocate_java_script_call_source_register(match condition.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value696, _) => {
                            let source_ml_source2_data_property_value696 = *source_ml_source2_data_property_value696;
                            source_ml_source2_data_property_value696.clone()
                        }
                    });
                    {
                        let branch = ((match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value697, _) => {
                                let source_ml_source2_data_property_value697 = *source_ml_source2_data_property_value697;
                                source_ml_source2_data_property_value697.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value698, _, _, _, _, _) => {
                                source_ml_source2_data_property_value698
                            }
                        }).len() as f64);
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value699, _) => {
                                    let source_ml_source2_data_property_value699 = *source_ml_source2_data_property_value699;
                                    source_ml_source2_data_property_value699.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value700) => {
                                    source_ml_source2_data_property_value700
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value701, _, _, _, _, _) => source_ml_source2_data_property_value701.as_slice() }).len() as f64);
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value702, _) => {
                                                let source_ml_source2_data_property_value702 = *source_ml_source2_data_property_value702;
                                                source_ml_source2_data_property_value702.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value703) => {
                                                source_ml_source2_data_property_value703
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value704) => {
                                                source_ml_source2_data_property_value704
                                            }
                                        }));
                                        {
                                            let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value705, _, _, _, _, _) => source_ml_source2_data_property_value705.as_slice() }).len() as f64);
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value706, _, _, _, _, _) => source_ml_source2_data_property_value706.as_slice() }).len() as f64);
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value707, _) => {
                                                                    let source_ml_source2_data_property_value707 = *source_ml_source2_data_property_value707;
                                                                    source_ml_source2_data_property_value707.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value708) => {
                                                                    source_ml_source2_data_property_value708
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value709) => {
                                                                    source_ml_source2_data_property_value709
                                                                }
                                                            }));
                                                            {
                                                                let end = ((match &copied_no { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value710, _, _, _, _, _) => source_ml_source2_data_property_value710.as_slice() }).len() as f64);
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value712) => {
                                                                        source_ml_source2_data_property_value712.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = { let ml_values4 = match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value713, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value713
                                                                            }
                                                                        }; let ml_callback4 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value714) => {
                                                                                    source_ml_source2_data_property_value714
                                                                                }
                                                                            }, yes_position, no_position)
                                                                        } else {
                                                                            if (index == jump) {
                                                                                self::JavaScriptCallInstruction::Jump(end)
                                                                            } else {
                                                                                entry.clone()
                                                                            }
                                                                        } }) }; ml_values4.iter().cloned().enumerate().map(|(ml_index4, ml_value4)| ml_callback4(ml_value4, (ml_index4 as f64))).collect::<Vec<_>>() };
                                                                        self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                                            let source_ml_source2_nominal_spread715 = copied_no.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field716 = match source_ml_source2_nominal_spread715.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value717, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value717
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field718 = match source_ml_source2_nominal_spread715.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value719, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value719
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field720 = match source_ml_source2_nominal_spread715.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value721, _, _, _) => {
                                                                                                source_ml_source2_data_property_value721
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field722 = match source_ml_source2_nominal_spread715.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value723, _, _) => {
                                                                                                    source_ml_source2_data_property_value723
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field724 = match source_ml_source2_nominal_spread715.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value725, _) => {
                                                                                                        source_ml_source2_data_property_value725
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field726 = match source_ml_source2_nominal_spread715.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value727) => {
                                                                                                            source_ml_source2_data_property_value727.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override728 = instructions;
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override728, source_ml_source2_nominal_field718, source_ml_source2_nominal_field720, source_ml_source2_nominal_field722, source_ml_source2_nominal_field724, source_ml_source2_nominal_field726.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match result.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value729) => {
                                                                                source_ml_source2_data_property_value729
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
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f678, source_ml_source2_f679, source_ml_source2_f680) => {
            let source_ml_source2_f679 = *source_ml_source2_f679;
            let source_ml_source2_f680 = *source_ml_source2_f680;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source2_f681, source_ml_source2_f682) => {
            let source_ml_source2_f681 = *source_ml_source2_f681;
            let source_ml_source2_f682 = *source_ml_source2_f682;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source2_f683, source_ml_source2_f684) => {
            let source_ml_source2_f683 = *source_ml_source2_f683;
            let source_ml_source2_f684 = *source_ml_source2_f684;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source2_f685, source_ml_source2_f686) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source2_f687) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source2_f688, source_ml_source2_f689, source_ml_source2_f690) => {
            let source_ml_source2_f688 = *source_ml_source2_f688;
            let source_ml_source2_f689 = *source_ml_source2_f689;
            let source_ml_source2_f690 = *source_ml_source2_f690;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source2_f691, source_ml_source2_f692) => {
            let source_ml_source2_f691 = *source_ml_source2_f691;
            let source_ml_source2_f692 = *source_ml_source2_f692;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source2_f693, source_ml_source2_f694, source_ml_source2_f695) => {
            let source_ml_source2_f693 = *source_ml_source2_f693;
            let source_ml_source2_f694 = *source_ml_source2_f694;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source(source: self::JavaScriptCallSource, parameters: &[f64]) -> self::JavaScriptCallSourceCompilation {
    if (((parameters.len() as f64)) >= (4096f64)) {
        self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters exceed their explicit register budget"))
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s730 = self::source_ml_source1_temporary_compile_java_script_call_source_loop102(parameters, index);
                match source_ml_source2_s730.clone() {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Done => {
                        {
                            let bindings = { let ml_values5 = parameters.to_vec(); let ml_callback5 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5, (ml_index5 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, (parameters.len() as f64), 0f64, String::from("")), 0f64);
                                {
                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value731, _) => {
                                            let source_ml_source2_data_property_value731 = *source_ml_source2_data_property_value731;
                                            source_ml_source2_data_property_value731.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value732) => {
                                            source_ml_source2_data_property_value732
                                        }
                                    }));
                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value734) => {
                                            source_ml_source2_data_property_value734.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value735, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value735
                                            }
                                        }
                                    } else {
                                        Vec::<self::JavaScriptCallInstruction>::new()
                                    }, if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value737) => {
                                            source_ml_source2_data_property_value737.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value738, _, _, _, _) => {
                                                source_ml_source2_data_property_value738
                                            }
                                        }
                                    } else {
                                        0f64
                                    }, match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value739) => {
                                            source_ml_source2_data_property_value739.clone()
                                        }
                                    })
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Return(source_ml_source1_temporary_result102) => {
                        let source_ml_source1_temporary_result102 = *source_ml_source1_temporary_result102;
                        source_ml_source1_temporary_result102.clone()
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Done,
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop97(mut source: self::JavaScriptCallSource, mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut state: self::JavaScriptCallSourceBuilder, mut index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = self::ml_array::optional_at(&bindings, self::ml_array::number_index(index));
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                    let source_ml_source2_comparison_operand763 = {
                        let source_ml_source2_nullish_field_receiver757 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver757.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present758 = ((source_ml_source2_nullish_field_receiver757.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present758.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value759, _) => {
                                        source_ml_source2_data_property_value759
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand764 = match source.clone() {
                            self::JavaScriptCallSource::SourceConstant(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceReceiver => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceArgument(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceThrow(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceTry(source_ml_source2_data_property_value760, _, _) => {
                                Some(source_ml_source2_data_property_value760)
                            }
                            self::JavaScriptCallSource::SourceBinding(source_ml_source2_data_property_value761) => {
                                Some(source_ml_source2_data_property_value761)
                            }
                            self::JavaScriptCallSource::SourceLet(source_ml_source2_data_property_value762, _, _) => {
                                Some(source_ml_source2_data_property_value762)
                            }
                            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceRecur(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceGet(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                                None::<f64>
                            }
                        };
                        match (source_ml_source2_comparison_operand763, source_ml_source2_comparison_operand764.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                    }
                }) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), {
                        let source_ml_source2_nullish_field_receiver765 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver765.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present766 = ((source_ml_source2_nullish_field_receiver765.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present766.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value767) => {
                                        source_ml_source2_data_property_value767
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    })))
                } else {
                    {
                        let index_2 = (index - 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop98(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values94: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index94: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result {
    loop {
        return if (source_ml_source1_temporary_index94 < (self::ml::Big::from_u128(source_ml_source1_temporary_values94.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values94, source_ml_source1_temporary_index94.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value768, _) => {
                                let source_ml_source2_data_property_value768 = *source_ml_source2_data_property_value768;
                                source_ml_source2_data_property_value768.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value769) => {
                                    source_ml_source2_data_property_value769
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index94_2 = source_ml_source1_temporary_index94.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index94) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index94_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Done(Box::new(current.clone()), arguments_)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(mut current: self::JavaScriptCallSourceBuilder, arguments_: &[f64], mut index: f64) -> self::JavaScriptCallSourceBuilder {
    loop {
        return if (index < ((arguments_.len() as f64))) {
            {
                let current_2 = self::append_java_script_call_source_instruction(current.clone(), self::JavaScriptCallInstruction::Copy((index + 1f64), (self::ml_array::optional_at(arguments_, self::ml_array::number_index(index))).clone().unwrap_or_else(|| 0f64)));
                {
                    let index_2 = (index + 1f64);
                    {
                        (current, index) = (current_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}
