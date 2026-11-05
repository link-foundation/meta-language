// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_compile_java_script_call_source_loop108;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_exception(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source2_f665) => {
            let source_ml_source2_f665 = *source_ml_source2_f665;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceArgument(source_ml_source2_f666) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceThrow(source_ml_source2_f667) => {
            let source_ml_source2_f667 = *source_ml_source2_f667;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceTry(identity, source_body, source_handler) => {
            let source_body = *source_body;
            let source_handler = *source_handler;
            if (!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || { let ml_values2 = bindings.clone(); let ml_callback2 = { let identity = identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value694, _) => {
                    source_ml_source2_data_property_value694
                }
            }) == identity) }) }; ml_values2.iter().cloned().enumerate().any(|(ml_index2, ml_value2)| ml_callback2(ml_value2)) }) {
                self::refuse_java_script_call_source(builder.clone(), "a catch binding owner must be unique in its lexical scope")
            } else {
                if ((match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value696, _) => {
                        source_ml_source2_data_property_value696
                    }
                }) >= (1024f64)) {
                    self::refuse_java_script_call_source(builder.clone(), "source exception handlers exceed their explicit active handler budget")
                } else {
                    {
                        let caught = self::allocate_java_script_call_source_register(builder.clone());
                        {
                            let result = self::allocate_java_script_call_source_register(match caught.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value697, _) => {
                                    let source_ml_source2_data_property_value697 = *source_ml_source2_data_property_value697;
                                    source_ml_source2_data_property_value697.clone()
                                }
                            });
                            {
                                let position = ((match match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value698, _) => {
                                        let source_ml_source2_data_property_value698 = *source_ml_source2_data_property_value698;
                                        source_ml_source2_data_property_value698.clone()
                                    }
                                } {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value699, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value699
                                    }
                                }).len() as f64);
                                {
                                    let protected_builder = self::append_java_script_call_source_instruction({
                                        let source_ml_source2_nominal_spread702 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value700, _) => {
                                                let source_ml_source2_data_property_value700 = *source_ml_source2_data_property_value700;
                                                source_ml_source2_data_property_value700.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field703 = match source_ml_source2_nominal_spread702.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value704, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value704
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field705 = match source_ml_source2_nominal_spread702.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value706, _, _, _, _) => {
                                                        source_ml_source2_data_property_value706
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field707 = match source_ml_source2_nominal_spread702.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value708, _, _, _) => {
                                                            source_ml_source2_data_property_value708
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field709 = match source_ml_source2_nominal_spread702.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value710, _, _) => {
                                                                source_ml_source2_data_property_value710
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field711 = match source_ml_source2_nominal_spread702.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value712, _) => {
                                                                    source_ml_source2_data_property_value712
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field713 = match source_ml_source2_nominal_spread702.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value714) => {
                                                                        source_ml_source2_data_property_value714.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override715 = (match builder.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value701, _) => {
                                                                            source_ml_source2_data_property_value701
                                                                        }
                                                                    } + 1f64);
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field703, source_ml_source2_nominal_field705, source_ml_source2_nominal_field707, source_ml_source2_nominal_field709, source_ml_source2_nominal_override715, source_ml_source2_nominal_field713.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }, self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value716) => {
                                            source_ml_source2_data_property_value716
                                        }
                                    }, 0f64));
                                    {
                                        let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                        {
                                            let popped = self::append_java_script_call_source_instruction({
                                                let source_ml_source2_nominal_spread719 = match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value717, _) => {
                                                        let source_ml_source2_data_property_value717 = *source_ml_source2_data_property_value717;
                                                        source_ml_source2_data_property_value717.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field720 = match source_ml_source2_nominal_spread719.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value721, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value721
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field722 = match source_ml_source2_nominal_spread719.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value723, _, _, _, _) => {
                                                                source_ml_source2_data_property_value723
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field724 = match source_ml_source2_nominal_spread719.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value725, _, _, _) => {
                                                                    source_ml_source2_data_property_value725
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field726 = match source_ml_source2_nominal_spread719.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value727, _, _) => {
                                                                        source_ml_source2_data_property_value727
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field728 = match source_ml_source2_nominal_spread719.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value729, _) => {
                                                                            source_ml_source2_data_property_value729
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field730 = match source_ml_source2_nominal_spread719.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value731) => {
                                                                                source_ml_source2_data_property_value731.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override732 = match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value718, _) => {
                                                                                    source_ml_source2_data_property_value718
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field720, source_ml_source2_nominal_field722, source_ml_source2_nominal_field724, source_ml_source2_nominal_field726, source_ml_source2_nominal_override732, source_ml_source2_nominal_field730.clone())
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
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value733) => {
                                                        source_ml_source2_data_property_value733
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value734) => {
                                                        source_ml_source2_data_property_value734
                                                    }
                                                }));
                                                {
                                                    let jump = ((match &copied { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value735, _, _, _, _, _) => source_ml_source2_data_property_value735.as_slice() }).len() as f64);
                                                    {
                                                        let alternative = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                        {
                                                            let target = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value736, _, _, _, _, _) => source_ml_source2_data_property_value736.as_slice() }).len() as f64);
                                                            {
                                                                let handler = self::lower_java_script_call_source_expression(source_handler.clone(), {
                                                                    let source_ml_source2_concat_value738 = bindings;
                                                                    {
                                                                        let source_ml_source2_concat_value739 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, match caught.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value737) => {
                                                                                source_ml_source2_data_property_value737
                                                                            }
                                                                        })];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value738), source_ml_source2_concat_value739)
                                                                    }
                                                                }, alternative.clone(), (depth + 1f64));
                                                                {
                                                                    let finished = self::append_java_script_call_source_instruction(match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value740, _) => {
                                                                            let source_ml_source2_data_property_value740 = *source_ml_source2_data_property_value740;
                                                                            source_ml_source2_data_property_value740.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value741) => {
                                                                            source_ml_source2_data_property_value741
                                                                        }
                                                                    }, match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value742) => {
                                                                            source_ml_source2_data_property_value742
                                                                        }
                                                                    }));
                                                                    {
                                                                        let end = ((match &finished { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value743, _, _, _, _, _) => source_ml_source2_data_property_value743.as_slice() }).len() as f64);
                                                                        if ((match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value745) => {
                                                                                source_ml_source2_data_property_value745.clone()
                                                                            }
                                                                        }) != "") {
                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(finished.clone()), 0f64)
                                                                        } else {
                                                                            {
                                                                                let instructions = { let ml_values3 = match finished.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value746, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value746
                                                                                    }
                                                                                }; let ml_callback3 = { let position = position; let caught = caught.clone(); let target = target; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                                    self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value747) => {
                                                                                            source_ml_source2_data_property_value747
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
                                                                                    let source_ml_source2_nominal_spread748 = finished.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field749 = match source_ml_source2_nominal_spread748.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value750, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value750
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field751 = match source_ml_source2_nominal_spread748.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value752, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value752
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field753 = match source_ml_source2_nominal_spread748.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value754, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value754
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field755 = match source_ml_source2_nominal_spread748.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value756, _, _) => {
                                                                                                            source_ml_source2_data_property_value756
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field757 = match source_ml_source2_nominal_spread748.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value758, _) => {
                                                                                                                source_ml_source2_data_property_value758
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field759 = match source_ml_source2_nominal_spread748.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value760) => {
                                                                                                                    source_ml_source2_data_property_value760.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override761 = instructions;
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override761, source_ml_source2_nominal_field751, source_ml_source2_nominal_field753, source_ml_source2_nominal_field755, source_ml_source2_nominal_field757, source_ml_source2_nominal_field759.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }), match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value762) => {
                                                                                        source_ml_source2_data_property_value762
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
        self::JavaScriptCallSource::SourceBinding(source_ml_source2_f668) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source2_f669, source_ml_source2_f670, source_ml_source2_f671) => {
            let source_ml_source2_f670 = *source_ml_source2_f670;
            let source_ml_source2_f671 = *source_ml_source2_f671;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIf(source_ml_source2_f672, source_ml_source2_f673, source_ml_source2_f674) => {
            let source_ml_source2_f672 = *source_ml_source2_f672;
            let source_ml_source2_f673 = *source_ml_source2_f673;
            let source_ml_source2_f674 = *source_ml_source2_f674;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f675, source_ml_source2_f676, source_ml_source2_f677) => {
            let source_ml_source2_f676 = *source_ml_source2_f676;
            let source_ml_source2_f677 = *source_ml_source2_f677;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source2_f678, source_ml_source2_f679) => {
            let source_ml_source2_f678 = *source_ml_source2_f678;
            let source_ml_source2_f679 = *source_ml_source2_f679;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source2_f680, source_ml_source2_f681) => {
            let source_ml_source2_f680 = *source_ml_source2_f680;
            let source_ml_source2_f681 = *source_ml_source2_f681;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source2_f682, source_ml_source2_f683) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source2_f684) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source2_f685, source_ml_source2_f686, source_ml_source2_f687) => {
            let source_ml_source2_f685 = *source_ml_source2_f685;
            let source_ml_source2_f686 = *source_ml_source2_f686;
            let source_ml_source2_f687 = *source_ml_source2_f687;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source2_f688, source_ml_source2_f689) => {
            let source_ml_source2_f688 = *source_ml_source2_f688;
            let source_ml_source2_f689 = *source_ml_source2_f689;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source2_f690, source_ml_source2_f691, source_ml_source2_f692) => {
            let source_ml_source2_f690 = *source_ml_source2_f690;
            let source_ml_source2_f691 = *source_ml_source2_f691;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source2_f763) => {
            let source_ml_source2_f763 = *source_ml_source2_f763;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceArgument(source_ml_source2_f764) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceThrow(source_ml_source2_f765) => {
            let source_ml_source2_f765 = *source_ml_source2_f765;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceTry(source_ml_source2_f766, source_ml_source2_f767, source_ml_source2_f768) => {
            let source_ml_source2_f767 = *source_ml_source2_f767;
            let source_ml_source2_f768 = *source_ml_source2_f768;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(source_ml_source2_f769) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source2_f770, source_ml_source2_f771, source_ml_source2_f772) => {
            let source_ml_source2_f771 = *source_ml_source2_f771;
            let source_ml_source2_f772 = *source_ml_source2_f772;
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value791, _) => {
                            let source_ml_source2_data_property_value791 = *source_ml_source2_data_property_value791;
                            source_ml_source2_data_property_value791.clone()
                        }
                    });
                    {
                        let branch = ((match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value792, _) => {
                                let source_ml_source2_data_property_value792 = *source_ml_source2_data_property_value792;
                                source_ml_source2_data_property_value792.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value793, _, _, _, _, _) => {
                                source_ml_source2_data_property_value793
                            }
                        }).len() as f64);
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value794, _) => {
                                    let source_ml_source2_data_property_value794 = *source_ml_source2_data_property_value794;
                                    source_ml_source2_data_property_value794.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value795) => {
                                    source_ml_source2_data_property_value795
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value796, _, _, _, _, _) => source_ml_source2_data_property_value796.as_slice() }).len() as f64);
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value797, _) => {
                                                let source_ml_source2_data_property_value797 = *source_ml_source2_data_property_value797;
                                                source_ml_source2_data_property_value797.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value798) => {
                                                source_ml_source2_data_property_value798
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value799) => {
                                                source_ml_source2_data_property_value799
                                            }
                                        }));
                                        {
                                            let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value800, _, _, _, _, _) => source_ml_source2_data_property_value800.as_slice() }).len() as f64);
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value801, _, _, _, _, _) => source_ml_source2_data_property_value801.as_slice() }).len() as f64);
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value802, _) => {
                                                                    let source_ml_source2_data_property_value802 = *source_ml_source2_data_property_value802;
                                                                    source_ml_source2_data_property_value802.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value803) => {
                                                                    source_ml_source2_data_property_value803
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value804) => {
                                                                    source_ml_source2_data_property_value804
                                                                }
                                                            }));
                                                            {
                                                                let end = ((match &copied_no { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value805, _, _, _, _, _) => source_ml_source2_data_property_value805.as_slice() }).len() as f64);
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value807) => {
                                                                        source_ml_source2_data_property_value807.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = { let ml_values4 = match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value808, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value808
                                                                            }
                                                                        }; let ml_callback4 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value809) => {
                                                                                    source_ml_source2_data_property_value809
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
                                                                            let source_ml_source2_nominal_spread810 = copied_no.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field811 = match source_ml_source2_nominal_spread810.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value812, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value812
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field813 = match source_ml_source2_nominal_spread810.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value814, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value814
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field815 = match source_ml_source2_nominal_spread810.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value816, _, _, _) => {
                                                                                                source_ml_source2_data_property_value816
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field817 = match source_ml_source2_nominal_spread810.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value818, _, _) => {
                                                                                                    source_ml_source2_data_property_value818
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field819 = match source_ml_source2_nominal_spread810.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value820, _) => {
                                                                                                        source_ml_source2_data_property_value820
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field821 = match source_ml_source2_nominal_spread810.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value822) => {
                                                                                                            source_ml_source2_data_property_value822.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override823 = instructions;
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override823, source_ml_source2_nominal_field813, source_ml_source2_nominal_field815, source_ml_source2_nominal_field817, source_ml_source2_nominal_field819, source_ml_source2_nominal_field821.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match result.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value824) => {
                                                                                source_ml_source2_data_property_value824
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
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f773, source_ml_source2_f774, source_ml_source2_f775) => {
            let source_ml_source2_f774 = *source_ml_source2_f774;
            let source_ml_source2_f775 = *source_ml_source2_f775;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source2_f776, source_ml_source2_f777) => {
            let source_ml_source2_f776 = *source_ml_source2_f776;
            let source_ml_source2_f777 = *source_ml_source2_f777;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source2_f778, source_ml_source2_f779) => {
            let source_ml_source2_f778 = *source_ml_source2_f778;
            let source_ml_source2_f779 = *source_ml_source2_f779;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source2_f780, source_ml_source2_f781) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source2_f782) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source2_f783, source_ml_source2_f784, source_ml_source2_f785) => {
            let source_ml_source2_f783 = *source_ml_source2_f783;
            let source_ml_source2_f784 = *source_ml_source2_f784;
            let source_ml_source2_f785 = *source_ml_source2_f785;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source2_f786, source_ml_source2_f787) => {
            let source_ml_source2_f786 = *source_ml_source2_f786;
            let source_ml_source2_f787 = *source_ml_source2_f787;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source2_f788, source_ml_source2_f789, source_ml_source2_f790) => {
            let source_ml_source2_f788 = *source_ml_source2_f788;
            let source_ml_source2_f789 = *source_ml_source2_f789;
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
                let source_ml_source2_s825 = self::source_ml_source1_temporary_compile_java_script_call_source_loop108(parameters, index);
                match source_ml_source2_s825.clone() {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Done => {
                        {
                            let bindings = { let ml_values5 = parameters.to_vec(); let ml_callback5 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5, (ml_index5 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, (parameters.len() as f64), 0f64, String::from("")), 0f64);
                                {
                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value826, _) => {
                                            let source_ml_source2_data_property_value826 = *source_ml_source2_data_property_value826;
                                            source_ml_source2_data_property_value826.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value827) => {
                                            source_ml_source2_data_property_value827
                                        }
                                    }));
                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value829) => {
                                            source_ml_source2_data_property_value829.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value830, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value830
                                            }
                                        }
                                    } else {
                                        Vec::<self::JavaScriptCallInstruction>::new()
                                    }, if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value832) => {
                                            source_ml_source2_data_property_value832.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value833, _, _, _, _) => {
                                                source_ml_source2_data_property_value833
                                            }
                                        }
                                    } else {
                                        0f64
                                    }, match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value834) => {
                                            source_ml_source2_data_property_value834.clone()
                                        }
                                    })
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Return(source_ml_source1_temporary_result108) => {
                        let source_ml_source1_temporary_result108 = *source_ml_source1_temporary_result108;
                        source_ml_source1_temporary_result108.clone()
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
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done,
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(mut source: self::JavaScriptCallSource, mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut state: self::JavaScriptCallSourceBuilder, mut index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = self::ml_array::optional_at(&bindings, self::ml_array::number_index(index));
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                    let source_ml_source2_comparison_operand868 = {
                        let source_ml_source2_nullish_field_receiver862 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver862.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present863 = ((source_ml_source2_nullish_field_receiver862.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present863.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value864, _) => {
                                        source_ml_source2_data_property_value864
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand869 = match source.clone() {
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
                            self::JavaScriptCallSource::SourceTry(source_ml_source2_data_property_value865, _, _) => {
                                Some(source_ml_source2_data_property_value865)
                            }
                            self::JavaScriptCallSource::SourceBinding(source_ml_source2_data_property_value866) => {
                                Some(source_ml_source2_data_property_value866)
                            }
                            self::JavaScriptCallSource::SourceLet(source_ml_source2_data_property_value867, _, _) => {
                                Some(source_ml_source2_data_property_value867)
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
                        match (source_ml_source2_comparison_operand868, source_ml_source2_comparison_operand869.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                    }
                }) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), {
                        let source_ml_source2_nullish_field_receiver870 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver870.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present871 = ((source_ml_source2_nullish_field_receiver870.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present871.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value872) => {
                                        source_ml_source2_data_property_value872
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
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop104(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values100: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index100: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result {
    loop {
        return if (source_ml_source1_temporary_index100 < (self::ml::Big::from_u128(source_ml_source1_temporary_values100.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values100, source_ml_source1_temporary_index100.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value873, _) => {
                                let source_ml_source2_data_property_value873 = *source_ml_source2_data_property_value873;
                                source_ml_source2_data_property_value873.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value874) => {
                                    source_ml_source2_data_property_value874
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index100_2 = source_ml_source1_temporary_index100.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index100) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index100_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(Box::new(current.clone()), arguments_)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop106(mut current: self::JavaScriptCallSourceBuilder, arguments_: &[f64], mut index: f64) -> self::JavaScriptCallSourceBuilder {
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
