// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_compile_java_script_call_source_loop105;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_exception(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source2_f628) => {
            let source_ml_source2_f628 = *source_ml_source2_f628;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceArgument(source_ml_source2_f629) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceThrow(source_ml_source2_f630) => {
            let source_ml_source2_f630 = *source_ml_source2_f630;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceTry(identity, source_body, source_handler) => {
            let source_body = *source_body;
            let source_handler = *source_handler;
            if (!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || { let ml_values2 = bindings.clone(); let ml_callback2 = { let identity = identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value657, _) => {
                    source_ml_source2_data_property_value657
                }
            }) == identity) }) }; ml_values2.iter().cloned().enumerate().any(|(ml_index2, ml_value2)| ml_callback2(ml_value2)) }) {
                self::refuse_java_script_call_source(builder.clone(), "a catch binding owner must be unique in its lexical scope")
            } else {
                if ((match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value659, _) => {
                        source_ml_source2_data_property_value659
                    }
                }) >= (1024f64)) {
                    self::refuse_java_script_call_source(builder.clone(), "source exception handlers exceed their explicit active handler budget")
                } else {
                    {
                        let caught = self::allocate_java_script_call_source_register(builder.clone());
                        {
                            let result = self::allocate_java_script_call_source_register(match caught.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value660, _) => {
                                    let source_ml_source2_data_property_value660 = *source_ml_source2_data_property_value660;
                                    source_ml_source2_data_property_value660.clone()
                                }
                            });
                            {
                                let position = ((match match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value661, _) => {
                                        let source_ml_source2_data_property_value661 = *source_ml_source2_data_property_value661;
                                        source_ml_source2_data_property_value661.clone()
                                    }
                                } {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value662, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value662
                                    }
                                }).len() as f64);
                                {
                                    let protected_builder = self::append_java_script_call_source_instruction({
                                        let source_ml_source2_nominal_spread665 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value663, _) => {
                                                let source_ml_source2_data_property_value663 = *source_ml_source2_data_property_value663;
                                                source_ml_source2_data_property_value663.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field666 = match source_ml_source2_nominal_spread665.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value667, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value667
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field668 = match source_ml_source2_nominal_spread665.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value669, _, _, _, _) => {
                                                        source_ml_source2_data_property_value669
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field670 = match source_ml_source2_nominal_spread665.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value671, _, _, _) => {
                                                            source_ml_source2_data_property_value671
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field672 = match source_ml_source2_nominal_spread665.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value673, _, _) => {
                                                                source_ml_source2_data_property_value673
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field674 = match source_ml_source2_nominal_spread665.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value675, _) => {
                                                                    source_ml_source2_data_property_value675
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field676 = match source_ml_source2_nominal_spread665.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value677) => {
                                                                        source_ml_source2_data_property_value677.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override678 = (match builder.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value664, _) => {
                                                                            source_ml_source2_data_property_value664
                                                                        }
                                                                    } + 1f64);
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field666, source_ml_source2_nominal_field668, source_ml_source2_nominal_field670, source_ml_source2_nominal_field672, source_ml_source2_nominal_override678, source_ml_source2_nominal_field676.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }, self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value679) => {
                                            source_ml_source2_data_property_value679
                                        }
                                    }, 0f64));
                                    {
                                        let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                        {
                                            let popped = self::append_java_script_call_source_instruction({
                                                let source_ml_source2_nominal_spread682 = match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value680, _) => {
                                                        let source_ml_source2_data_property_value680 = *source_ml_source2_data_property_value680;
                                                        source_ml_source2_data_property_value680.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field683 = match source_ml_source2_nominal_spread682.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value684, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value684
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field685 = match source_ml_source2_nominal_spread682.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value686, _, _, _, _) => {
                                                                source_ml_source2_data_property_value686
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field687 = match source_ml_source2_nominal_spread682.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value688, _, _, _) => {
                                                                    source_ml_source2_data_property_value688
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field689 = match source_ml_source2_nominal_spread682.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value690, _, _) => {
                                                                        source_ml_source2_data_property_value690
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field691 = match source_ml_source2_nominal_spread682.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value692, _) => {
                                                                            source_ml_source2_data_property_value692
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field693 = match source_ml_source2_nominal_spread682.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value694) => {
                                                                                source_ml_source2_data_property_value694.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override695 = match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value681, _) => {
                                                                                    source_ml_source2_data_property_value681
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field683, source_ml_source2_nominal_field685, source_ml_source2_nominal_field687, source_ml_source2_nominal_field689, source_ml_source2_nominal_override695, source_ml_source2_nominal_field693.clone())
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
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value696) => {
                                                        source_ml_source2_data_property_value696
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value697) => {
                                                        source_ml_source2_data_property_value697
                                                    }
                                                }));
                                                {
                                                    let jump = ((match &copied { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value698, _, _, _, _, _) => source_ml_source2_data_property_value698.as_slice() }).len() as f64);
                                                    {
                                                        let alternative = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                        {
                                                            let target = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value699, _, _, _, _, _) => source_ml_source2_data_property_value699.as_slice() }).len() as f64);
                                                            {
                                                                let handler = self::lower_java_script_call_source_expression(source_handler.clone(), {
                                                                    let source_ml_source2_concat_value701 = bindings;
                                                                    {
                                                                        let source_ml_source2_concat_value702 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, match caught.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value700) => {
                                                                                source_ml_source2_data_property_value700
                                                                            }
                                                                        })];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value701), source_ml_source2_concat_value702)
                                                                    }
                                                                }, alternative.clone(), (depth + 1f64));
                                                                {
                                                                    let finished = self::append_java_script_call_source_instruction(match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value703, _) => {
                                                                            let source_ml_source2_data_property_value703 = *source_ml_source2_data_property_value703;
                                                                            source_ml_source2_data_property_value703.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value704) => {
                                                                            source_ml_source2_data_property_value704
                                                                        }
                                                                    }, match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value705) => {
                                                                            source_ml_source2_data_property_value705
                                                                        }
                                                                    }));
                                                                    {
                                                                        let end = ((match &finished { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value706, _, _, _, _, _) => source_ml_source2_data_property_value706.as_slice() }).len() as f64);
                                                                        if ((match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value708) => {
                                                                                source_ml_source2_data_property_value708.clone()
                                                                            }
                                                                        }) != "") {
                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(finished.clone()), 0f64)
                                                                        } else {
                                                                            {
                                                                                let instructions = { let ml_values3 = match finished.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value709, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value709
                                                                                    }
                                                                                }; let ml_callback3 = { let position = position; let caught = caught.clone(); let target = target; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                                    self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value710) => {
                                                                                            source_ml_source2_data_property_value710
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
                                                                                    let source_ml_source2_nominal_spread711 = finished.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field712 = match source_ml_source2_nominal_spread711.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value713, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value713
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field714 = match source_ml_source2_nominal_spread711.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value715, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value715
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field716 = match source_ml_source2_nominal_spread711.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value717, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value717
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field718 = match source_ml_source2_nominal_spread711.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value719, _, _) => {
                                                                                                            source_ml_source2_data_property_value719
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field720 = match source_ml_source2_nominal_spread711.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value721, _) => {
                                                                                                                source_ml_source2_data_property_value721
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field722 = match source_ml_source2_nominal_spread711.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value723) => {
                                                                                                                    source_ml_source2_data_property_value723.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override724 = instructions;
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override724, source_ml_source2_nominal_field714, source_ml_source2_nominal_field716, source_ml_source2_nominal_field718, source_ml_source2_nominal_field720, source_ml_source2_nominal_field722.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }), match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value725) => {
                                                                                        source_ml_source2_data_property_value725
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
        self::JavaScriptCallSource::SourceBinding(source_ml_source2_f631) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source2_f632, source_ml_source2_f633, source_ml_source2_f634) => {
            let source_ml_source2_f633 = *source_ml_source2_f633;
            let source_ml_source2_f634 = *source_ml_source2_f634;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIf(source_ml_source2_f635, source_ml_source2_f636, source_ml_source2_f637) => {
            let source_ml_source2_f635 = *source_ml_source2_f635;
            let source_ml_source2_f636 = *source_ml_source2_f636;
            let source_ml_source2_f637 = *source_ml_source2_f637;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f638, source_ml_source2_f639, source_ml_source2_f640) => {
            let source_ml_source2_f639 = *source_ml_source2_f639;
            let source_ml_source2_f640 = *source_ml_source2_f640;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source2_f641, source_ml_source2_f642) => {
            let source_ml_source2_f641 = *source_ml_source2_f641;
            let source_ml_source2_f642 = *source_ml_source2_f642;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source2_f643, source_ml_source2_f644) => {
            let source_ml_source2_f643 = *source_ml_source2_f643;
            let source_ml_source2_f644 = *source_ml_source2_f644;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source2_f645, source_ml_source2_f646) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source2_f647) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source2_f648, source_ml_source2_f649, source_ml_source2_f650) => {
            let source_ml_source2_f648 = *source_ml_source2_f648;
            let source_ml_source2_f649 = *source_ml_source2_f649;
            let source_ml_source2_f650 = *source_ml_source2_f650;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source2_f651, source_ml_source2_f652) => {
            let source_ml_source2_f651 = *source_ml_source2_f651;
            let source_ml_source2_f652 = *source_ml_source2_f652;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source2_f653, source_ml_source2_f654, source_ml_source2_f655) => {
            let source_ml_source2_f653 = *source_ml_source2_f653;
            let source_ml_source2_f654 = *source_ml_source2_f654;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source2_f726) => {
            let source_ml_source2_f726 = *source_ml_source2_f726;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceArgument(source_ml_source2_f727) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceThrow(source_ml_source2_f728) => {
            let source_ml_source2_f728 = *source_ml_source2_f728;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceTry(source_ml_source2_f729, source_ml_source2_f730, source_ml_source2_f731) => {
            let source_ml_source2_f730 = *source_ml_source2_f730;
            let source_ml_source2_f731 = *source_ml_source2_f731;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(source_ml_source2_f732) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source2_f733, source_ml_source2_f734, source_ml_source2_f735) => {
            let source_ml_source2_f734 = *source_ml_source2_f734;
            let source_ml_source2_f735 = *source_ml_source2_f735;
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value754, _) => {
                            let source_ml_source2_data_property_value754 = *source_ml_source2_data_property_value754;
                            source_ml_source2_data_property_value754.clone()
                        }
                    });
                    {
                        let branch = ((match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value755, _) => {
                                let source_ml_source2_data_property_value755 = *source_ml_source2_data_property_value755;
                                source_ml_source2_data_property_value755.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value756, _, _, _, _, _) => {
                                source_ml_source2_data_property_value756
                            }
                        }).len() as f64);
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value757, _) => {
                                    let source_ml_source2_data_property_value757 = *source_ml_source2_data_property_value757;
                                    source_ml_source2_data_property_value757.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value758) => {
                                    source_ml_source2_data_property_value758
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value759, _, _, _, _, _) => source_ml_source2_data_property_value759.as_slice() }).len() as f64);
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value760, _) => {
                                                let source_ml_source2_data_property_value760 = *source_ml_source2_data_property_value760;
                                                source_ml_source2_data_property_value760.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value761) => {
                                                source_ml_source2_data_property_value761
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value762) => {
                                                source_ml_source2_data_property_value762
                                            }
                                        }));
                                        {
                                            let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value763, _, _, _, _, _) => source_ml_source2_data_property_value763.as_slice() }).len() as f64);
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value764, _, _, _, _, _) => source_ml_source2_data_property_value764.as_slice() }).len() as f64);
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value765, _) => {
                                                                    let source_ml_source2_data_property_value765 = *source_ml_source2_data_property_value765;
                                                                    source_ml_source2_data_property_value765.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value766) => {
                                                                    source_ml_source2_data_property_value766
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value767) => {
                                                                    source_ml_source2_data_property_value767
                                                                }
                                                            }));
                                                            {
                                                                let end = ((match &copied_no { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value768, _, _, _, _, _) => source_ml_source2_data_property_value768.as_slice() }).len() as f64);
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value770) => {
                                                                        source_ml_source2_data_property_value770.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = { let ml_values4 = match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value771, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value771
                                                                            }
                                                                        }; let ml_callback4 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value772) => {
                                                                                    source_ml_source2_data_property_value772
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
                                                                            let source_ml_source2_nominal_spread773 = copied_no.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field774 = match source_ml_source2_nominal_spread773.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value775, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value775
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field776 = match source_ml_source2_nominal_spread773.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value777, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value777
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field778 = match source_ml_source2_nominal_spread773.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value779, _, _, _) => {
                                                                                                source_ml_source2_data_property_value779
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field780 = match source_ml_source2_nominal_spread773.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value781, _, _) => {
                                                                                                    source_ml_source2_data_property_value781
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field782 = match source_ml_source2_nominal_spread773.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value783, _) => {
                                                                                                        source_ml_source2_data_property_value783
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field784 = match source_ml_source2_nominal_spread773.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value785) => {
                                                                                                            source_ml_source2_data_property_value785.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override786 = instructions;
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override786, source_ml_source2_nominal_field776, source_ml_source2_nominal_field778, source_ml_source2_nominal_field780, source_ml_source2_nominal_field782, source_ml_source2_nominal_field784.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match result.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value787) => {
                                                                                source_ml_source2_data_property_value787
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
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f736, source_ml_source2_f737, source_ml_source2_f738) => {
            let source_ml_source2_f737 = *source_ml_source2_f737;
            let source_ml_source2_f738 = *source_ml_source2_f738;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source2_f739, source_ml_source2_f740) => {
            let source_ml_source2_f739 = *source_ml_source2_f739;
            let source_ml_source2_f740 = *source_ml_source2_f740;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source2_f741, source_ml_source2_f742) => {
            let source_ml_source2_f741 = *source_ml_source2_f741;
            let source_ml_source2_f742 = *source_ml_source2_f742;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source2_f743, source_ml_source2_f744) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source2_f745) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source2_f746, source_ml_source2_f747, source_ml_source2_f748) => {
            let source_ml_source2_f746 = *source_ml_source2_f746;
            let source_ml_source2_f747 = *source_ml_source2_f747;
            let source_ml_source2_f748 = *source_ml_source2_f748;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source2_f749, source_ml_source2_f750) => {
            let source_ml_source2_f749 = *source_ml_source2_f749;
            let source_ml_source2_f750 = *source_ml_source2_f750;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source2_f751, source_ml_source2_f752, source_ml_source2_f753) => {
            let source_ml_source2_f751 = *source_ml_source2_f751;
            let source_ml_source2_f752 = *source_ml_source2_f752;
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
                let source_ml_source2_s788 = self::source_ml_source1_temporary_compile_java_script_call_source_loop105(parameters, index);
                match source_ml_source2_s788.clone() {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Done => {
                        {
                            let bindings = { let ml_values5 = parameters.to_vec(); let ml_callback5 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5, (ml_index5 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, (parameters.len() as f64), 0f64, String::from("")), 0f64);
                                {
                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value789, _) => {
                                            let source_ml_source2_data_property_value789 = *source_ml_source2_data_property_value789;
                                            source_ml_source2_data_property_value789.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value790) => {
                                            source_ml_source2_data_property_value790
                                        }
                                    }));
                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value792) => {
                                            source_ml_source2_data_property_value792.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value793, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value793
                                            }
                                        }
                                    } else {
                                        Vec::<self::JavaScriptCallInstruction>::new()
                                    }, if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value795) => {
                                            source_ml_source2_data_property_value795.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value796, _, _, _, _) => {
                                                source_ml_source2_data_property_value796
                                            }
                                        }
                                    } else {
                                        0f64
                                    }, match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value797) => {
                                            source_ml_source2_data_property_value797.clone()
                                        }
                                    })
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Return(source_ml_source1_temporary_result105) => {
                        let source_ml_source1_temporary_result105 = *source_ml_source1_temporary_result105;
                        source_ml_source1_temporary_result105.clone()
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
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Done,
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(mut source: self::JavaScriptCallSource, mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut state: self::JavaScriptCallSourceBuilder, mut index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = self::ml_array::optional_at(&bindings, self::ml_array::number_index(index));
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                    let source_ml_source2_comparison_operand831 = {
                        let source_ml_source2_nullish_field_receiver825 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver825.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present826 = ((source_ml_source2_nullish_field_receiver825.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present826.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value827, _) => {
                                        source_ml_source2_data_property_value827
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand832 = match source.clone() {
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
                            self::JavaScriptCallSource::SourceTry(source_ml_source2_data_property_value828, _, _) => {
                                Some(source_ml_source2_data_property_value828)
                            }
                            self::JavaScriptCallSource::SourceBinding(source_ml_source2_data_property_value829) => {
                                Some(source_ml_source2_data_property_value829)
                            }
                            self::JavaScriptCallSource::SourceLet(source_ml_source2_data_property_value830, _, _) => {
                                Some(source_ml_source2_data_property_value830)
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
                        match (source_ml_source2_comparison_operand831, source_ml_source2_comparison_operand832.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                    }
                }) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), {
                        let source_ml_source2_nullish_field_receiver833 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver833.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present834 = ((source_ml_source2_nullish_field_receiver833.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present834.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value835) => {
                                        source_ml_source2_data_property_value835
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
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop100Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values97: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index97: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result {
    loop {
        return if (source_ml_source1_temporary_index97 < (self::ml::Big::from_u128(source_ml_source1_temporary_values97.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values97, source_ml_source1_temporary_index97.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value836, _) => {
                                let source_ml_source2_data_property_value836 = *source_ml_source2_data_property_value836;
                                source_ml_source2_data_property_value836.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value837) => {
                                    source_ml_source2_data_property_value837
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index97_2 = source_ml_source1_temporary_index97.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index97) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index97_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(Box::new(current.clone()), arguments_)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(mut current: self::JavaScriptCallSourceBuilder, arguments_: &[f64], mut index: f64) -> self::JavaScriptCallSourceBuilder {
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
