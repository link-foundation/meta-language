// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_compile_java_script_call_source_loop106;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_exception(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source2_f646) => {
            let source_ml_source2_f646 = *source_ml_source2_f646;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceArgument(source_ml_source2_f647) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceThrow(source_ml_source2_f648) => {
            let source_ml_source2_f648 = *source_ml_source2_f648;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceTry(identity, source_body, source_handler) => {
            let source_body = *source_body;
            let source_handler = *source_handler;
            if (!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || { let ml_values2 = bindings.clone(); let ml_callback2 = { let identity = identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value675, _) => {
                    source_ml_source2_data_property_value675
                }
            }) == identity) }) }; ml_values2.iter().cloned().enumerate().any(|(ml_index2, ml_value2)| ml_callback2(ml_value2)) }) {
                self::refuse_java_script_call_source(builder.clone(), "a catch binding owner must be unique in its lexical scope")
            } else {
                if ((match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value677, _) => {
                        source_ml_source2_data_property_value677
                    }
                }) >= (1024f64)) {
                    self::refuse_java_script_call_source(builder.clone(), "source exception handlers exceed their explicit active handler budget")
                } else {
                    {
                        let caught = self::allocate_java_script_call_source_register(builder.clone());
                        {
                            let result = self::allocate_java_script_call_source_register(match caught.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value678, _) => {
                                    let source_ml_source2_data_property_value678 = *source_ml_source2_data_property_value678;
                                    source_ml_source2_data_property_value678.clone()
                                }
                            });
                            {
                                let position = ((match match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value679, _) => {
                                        let source_ml_source2_data_property_value679 = *source_ml_source2_data_property_value679;
                                        source_ml_source2_data_property_value679.clone()
                                    }
                                } {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value680, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value680
                                    }
                                }).len() as f64);
                                {
                                    let protected_builder = self::append_java_script_call_source_instruction({
                                        let source_ml_source2_nominal_spread683 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value681, _) => {
                                                let source_ml_source2_data_property_value681 = *source_ml_source2_data_property_value681;
                                                source_ml_source2_data_property_value681.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field684 = match source_ml_source2_nominal_spread683.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value685, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value685
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field686 = match source_ml_source2_nominal_spread683.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value687, _, _, _, _) => {
                                                        source_ml_source2_data_property_value687
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field688 = match source_ml_source2_nominal_spread683.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value689, _, _, _) => {
                                                            source_ml_source2_data_property_value689
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field690 = match source_ml_source2_nominal_spread683.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value691, _, _) => {
                                                                source_ml_source2_data_property_value691
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field692 = match source_ml_source2_nominal_spread683.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value693, _) => {
                                                                    source_ml_source2_data_property_value693
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field694 = match source_ml_source2_nominal_spread683.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value695) => {
                                                                        source_ml_source2_data_property_value695.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override696 = (match builder.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value682, _) => {
                                                                            source_ml_source2_data_property_value682
                                                                        }
                                                                    } + 1f64);
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field684, source_ml_source2_nominal_field686, source_ml_source2_nominal_field688, source_ml_source2_nominal_field690, source_ml_source2_nominal_override696, source_ml_source2_nominal_field694.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }, self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value697) => {
                                            source_ml_source2_data_property_value697
                                        }
                                    }, 0f64));
                                    {
                                        let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                        {
                                            let popped = self::append_java_script_call_source_instruction({
                                                let source_ml_source2_nominal_spread700 = match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value698, _) => {
                                                        let source_ml_source2_data_property_value698 = *source_ml_source2_data_property_value698;
                                                        source_ml_source2_data_property_value698.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field701 = match source_ml_source2_nominal_spread700.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value702, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value702
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field703 = match source_ml_source2_nominal_spread700.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value704, _, _, _, _) => {
                                                                source_ml_source2_data_property_value704
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field705 = match source_ml_source2_nominal_spread700.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value706, _, _, _) => {
                                                                    source_ml_source2_data_property_value706
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field707 = match source_ml_source2_nominal_spread700.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value708, _, _) => {
                                                                        source_ml_source2_data_property_value708
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field709 = match source_ml_source2_nominal_spread700.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value710, _) => {
                                                                            source_ml_source2_data_property_value710
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field711 = match source_ml_source2_nominal_spread700.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value712) => {
                                                                                source_ml_source2_data_property_value712.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override713 = match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value699, _) => {
                                                                                    source_ml_source2_data_property_value699
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field701, source_ml_source2_nominal_field703, source_ml_source2_nominal_field705, source_ml_source2_nominal_field707, source_ml_source2_nominal_override713, source_ml_source2_nominal_field711.clone())
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
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value714) => {
                                                        source_ml_source2_data_property_value714
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value715) => {
                                                        source_ml_source2_data_property_value715
                                                    }
                                                }));
                                                {
                                                    let jump = ((match &copied { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value716, _, _, _, _, _) => source_ml_source2_data_property_value716.as_slice() }).len() as f64);
                                                    {
                                                        let alternative = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                        {
                                                            let target = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value717, _, _, _, _, _) => source_ml_source2_data_property_value717.as_slice() }).len() as f64);
                                                            {
                                                                let handler = self::lower_java_script_call_source_expression(source_handler.clone(), {
                                                                    let source_ml_source2_concat_value719 = bindings;
                                                                    {
                                                                        let source_ml_source2_concat_value720 = vec![self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, match caught.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value718) => {
                                                                                source_ml_source2_data_property_value718
                                                                            }
                                                                        })];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), source_ml_source2_concat_value719), source_ml_source2_concat_value720)
                                                                    }
                                                                }, alternative.clone(), (depth + 1f64));
                                                                {
                                                                    let finished = self::append_java_script_call_source_instruction(match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value721, _) => {
                                                                            let source_ml_source2_data_property_value721 = *source_ml_source2_data_property_value721;
                                                                            source_ml_source2_data_property_value721.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value722) => {
                                                                            source_ml_source2_data_property_value722
                                                                        }
                                                                    }, match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value723) => {
                                                                            source_ml_source2_data_property_value723
                                                                        }
                                                                    }));
                                                                    {
                                                                        let end = ((match &finished { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value724, _, _, _, _, _) => source_ml_source2_data_property_value724.as_slice() }).len() as f64);
                                                                        if ((match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value726) => {
                                                                                source_ml_source2_data_property_value726.clone()
                                                                            }
                                                                        }) != "") {
                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(finished.clone()), 0f64)
                                                                        } else {
                                                                            {
                                                                                let instructions = { let ml_values3 = match finished.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value727, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value727
                                                                                    }
                                                                                }; let ml_callback3 = { let position = position; let caught = caught.clone(); let target = target; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                                    self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value728) => {
                                                                                            source_ml_source2_data_property_value728
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
                                                                                    let source_ml_source2_nominal_spread729 = finished.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field730 = match source_ml_source2_nominal_spread729.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value731, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value731
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field732 = match source_ml_source2_nominal_spread729.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value733, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value733
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field734 = match source_ml_source2_nominal_spread729.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value735, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value735
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field736 = match source_ml_source2_nominal_spread729.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value737, _, _) => {
                                                                                                            source_ml_source2_data_property_value737
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field738 = match source_ml_source2_nominal_spread729.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value739, _) => {
                                                                                                                source_ml_source2_data_property_value739
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field740 = match source_ml_source2_nominal_spread729.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value741) => {
                                                                                                                    source_ml_source2_data_property_value741.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override742 = instructions;
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override742, source_ml_source2_nominal_field732, source_ml_source2_nominal_field734, source_ml_source2_nominal_field736, source_ml_source2_nominal_field738, source_ml_source2_nominal_field740.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }), match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value743) => {
                                                                                        source_ml_source2_data_property_value743
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
        self::JavaScriptCallSource::SourceBinding(source_ml_source2_f649) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source2_f650, source_ml_source2_f651, source_ml_source2_f652) => {
            let source_ml_source2_f651 = *source_ml_source2_f651;
            let source_ml_source2_f652 = *source_ml_source2_f652;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceIf(source_ml_source2_f653, source_ml_source2_f654, source_ml_source2_f655) => {
            let source_ml_source2_f653 = *source_ml_source2_f653;
            let source_ml_source2_f654 = *source_ml_source2_f654;
            let source_ml_source2_f655 = *source_ml_source2_f655;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f656, source_ml_source2_f657, source_ml_source2_f658) => {
            let source_ml_source2_f657 = *source_ml_source2_f657;
            let source_ml_source2_f658 = *source_ml_source2_f658;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source2_f659, source_ml_source2_f660) => {
            let source_ml_source2_f659 = *source_ml_source2_f659;
            let source_ml_source2_f660 = *source_ml_source2_f660;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source2_f661, source_ml_source2_f662) => {
            let source_ml_source2_f661 = *source_ml_source2_f661;
            let source_ml_source2_f662 = *source_ml_source2_f662;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source2_f663, source_ml_source2_f664) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source2_f665) => {
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source2_f666, source_ml_source2_f667, source_ml_source2_f668) => {
            let source_ml_source2_f666 = *source_ml_source2_f666;
            let source_ml_source2_f667 = *source_ml_source2_f667;
            let source_ml_source2_f668 = *source_ml_source2_f668;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source2_f669, source_ml_source2_f670) => {
            let source_ml_source2_f669 = *source_ml_source2_f669;
            let source_ml_source2_f670 = *source_ml_source2_f670;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source2_f671, source_ml_source2_f672, source_ml_source2_f673) => {
            let source_ml_source2_f671 = *source_ml_source2_f671;
            let source_ml_source2_f672 = *source_ml_source2_f672;
            self::refuse_java_script_call_source(builder.clone(), "an exception compiler requires its actual protected body and catch binding")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: Vec<self::JavaScriptCallSourceBinding>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(source_ml_source2_f744) => {
            let source_ml_source2_f744 = *source_ml_source2_f744;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceArgument(source_ml_source2_f745) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceThrow(source_ml_source2_f746) => {
            let source_ml_source2_f746 = *source_ml_source2_f746;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceTry(source_ml_source2_f747, source_ml_source2_f748, source_ml_source2_f749) => {
            let source_ml_source2_f748 = *source_ml_source2_f748;
            let source_ml_source2_f749 = *source_ml_source2_f749;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(source_ml_source2_f750) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(source_ml_source2_f751, source_ml_source2_f752, source_ml_source2_f753) => {
            let source_ml_source2_f752 = *source_ml_source2_f752;
            let source_ml_source2_f753 = *source_ml_source2_f753;
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value772, _) => {
                            let source_ml_source2_data_property_value772 = *source_ml_source2_data_property_value772;
                            source_ml_source2_data_property_value772.clone()
                        }
                    });
                    {
                        let branch = ((match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value773, _) => {
                                let source_ml_source2_data_property_value773 = *source_ml_source2_data_property_value773;
                                source_ml_source2_data_property_value773.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value774, _, _, _, _, _) => {
                                source_ml_source2_data_property_value774
                            }
                        }).len() as f64);
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value775, _) => {
                                    let source_ml_source2_data_property_value775 = *source_ml_source2_data_property_value775;
                                    source_ml_source2_data_property_value775.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value776) => {
                                    source_ml_source2_data_property_value776
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = ((match &start { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value777, _, _, _, _, _) => source_ml_source2_data_property_value777.as_slice() }).len() as f64);
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value778, _) => {
                                                let source_ml_source2_data_property_value778 = *source_ml_source2_data_property_value778;
                                                source_ml_source2_data_property_value778.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value779) => {
                                                source_ml_source2_data_property_value779
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value780) => {
                                                source_ml_source2_data_property_value780
                                            }
                                        }));
                                        {
                                            let jump = ((match &copied_yes { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value781, _, _, _, _, _) => source_ml_source2_data_property_value781.as_slice() }).len() as f64);
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = ((match &alternative { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value782, _, _, _, _, _) => source_ml_source2_data_property_value782.as_slice() }).len() as f64);
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings, alternative.clone(), (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value783, _) => {
                                                                    let source_ml_source2_data_property_value783 = *source_ml_source2_data_property_value783;
                                                                    source_ml_source2_data_property_value783.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value784) => {
                                                                    source_ml_source2_data_property_value784
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value785) => {
                                                                    source_ml_source2_data_property_value785
                                                                }
                                                            }));
                                                            {
                                                                let end = ((match &copied_no { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value786, _, _, _, _, _) => source_ml_source2_data_property_value786.as_slice() }).len() as f64);
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value788) => {
                                                                        source_ml_source2_data_property_value788.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = { let ml_values4 = match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value789, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value789
                                                                            }
                                                                        }; let ml_callback4 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value790) => {
                                                                                    source_ml_source2_data_property_value790
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
                                                                            let source_ml_source2_nominal_spread791 = copied_no.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field792 = match source_ml_source2_nominal_spread791.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value793, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value793
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field794 = match source_ml_source2_nominal_spread791.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value795, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value795
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field796 = match source_ml_source2_nominal_spread791.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value797, _, _, _) => {
                                                                                                source_ml_source2_data_property_value797
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field798 = match source_ml_source2_nominal_spread791.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value799, _, _) => {
                                                                                                    source_ml_source2_data_property_value799
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field800 = match source_ml_source2_nominal_spread791.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value801, _) => {
                                                                                                        source_ml_source2_data_property_value801
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field802 = match source_ml_source2_nominal_spread791.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value803) => {
                                                                                                            source_ml_source2_data_property_value803.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override804 = instructions;
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override804, source_ml_source2_nominal_field794, source_ml_source2_nominal_field796, source_ml_source2_nominal_field798, source_ml_source2_nominal_field800, source_ml_source2_nominal_field802.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match result.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value805) => {
                                                                                source_ml_source2_data_property_value805
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
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f754, source_ml_source2_f755, source_ml_source2_f756) => {
            let source_ml_source2_f755 = *source_ml_source2_f755;
            let source_ml_source2_f756 = *source_ml_source2_f756;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(source_ml_source2_f757, source_ml_source2_f758) => {
            let source_ml_source2_f757 = *source_ml_source2_f757;
            let source_ml_source2_f758 = *source_ml_source2_f758;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_ml_source2_f759, source_ml_source2_f760) => {
            let source_ml_source2_f759 = *source_ml_source2_f759;
            let source_ml_source2_f760 = *source_ml_source2_f760;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourcePrimitive(source_ml_source2_f761, source_ml_source2_f762) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceRecur(source_ml_source2_f763) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSet(source_ml_source2_f764, source_ml_source2_f765, source_ml_source2_f766) => {
            let source_ml_source2_f764 = *source_ml_source2_f764;
            let source_ml_source2_f765 = *source_ml_source2_f765;
            let source_ml_source2_f766 = *source_ml_source2_f766;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceGet(source_ml_source2_f767, source_ml_source2_f768) => {
            let source_ml_source2_f767 = *source_ml_source2_f767;
            let source_ml_source2_f768 = *source_ml_source2_f768;
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(source_ml_source2_f769, source_ml_source2_f770, source_ml_source2_f771) => {
            let source_ml_source2_f769 = *source_ml_source2_f769;
            let source_ml_source2_f770 = *source_ml_source2_f770;
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
                let source_ml_source2_s806 = self::source_ml_source1_temporary_compile_java_script_call_source_loop106(parameters, index);
                match source_ml_source2_s806.clone() {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Done => {
                        {
                            let bindings = { let ml_values5 = parameters.to_vec(); let ml_callback5 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values5.iter().cloned().enumerate().map(|(ml_index5, ml_value5)| ml_callback5(ml_value5, (ml_index5 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, (parameters.len() as f64), 0f64, String::from("")), 0f64);
                                {
                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value807, _) => {
                                            let source_ml_source2_data_property_value807 = *source_ml_source2_data_property_value807;
                                            source_ml_source2_data_property_value807.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value808) => {
                                            source_ml_source2_data_property_value808
                                        }
                                    }));
                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value810) => {
                                            source_ml_source2_data_property_value810.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value811, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value811
                                            }
                                        }
                                    } else {
                                        Vec::<self::JavaScriptCallInstruction>::new()
                                    }, if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value813) => {
                                            source_ml_source2_data_property_value813.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value814, _, _, _, _) => {
                                                source_ml_source2_data_property_value814
                                            }
                                        }
                                    } else {
                                        0f64
                                    }, match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value815) => {
                                            source_ml_source2_data_property_value815.clone()
                                        }
                                    })
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Return(source_ml_source1_temporary_result106) => {
                        let source_ml_source1_temporary_result106 = *source_ml_source1_temporary_result106;
                        source_ml_source1_temporary_result106.clone()
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
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done,
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(mut source: self::JavaScriptCallSource, mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut state: self::JavaScriptCallSourceBuilder, mut index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = self::ml_array::optional_at(&bindings, self::ml_array::number_index(index));
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                    let source_ml_source2_comparison_operand849 = {
                        let source_ml_source2_nullish_field_receiver843 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver843.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present844 = ((source_ml_source2_nullish_field_receiver843.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present844.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value845, _) => {
                                        source_ml_source2_data_property_value845
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand850 = match source.clone() {
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
                            self::JavaScriptCallSource::SourceTry(source_ml_source2_data_property_value846, _, _) => {
                                Some(source_ml_source2_data_property_value846)
                            }
                            self::JavaScriptCallSource::SourceBinding(source_ml_source2_data_property_value847) => {
                                Some(source_ml_source2_data_property_value847)
                            }
                            self::JavaScriptCallSource::SourceLet(source_ml_source2_data_property_value848, _, _) => {
                                Some(source_ml_source2_data_property_value848)
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
                        match (source_ml_source2_comparison_operand849, source_ml_source2_comparison_operand850.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                    }
                }) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), {
                        let source_ml_source2_nullish_field_receiver851 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver851.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present852 = ((source_ml_source2_nullish_field_receiver851.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present852.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value853) => {
                                        source_ml_source2_data_property_value853
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
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop102(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values98: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index98: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Result {
    loop {
        return if (source_ml_source1_temporary_index98 < (self::ml::Big::from_u128(source_ml_source1_temporary_values98.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values98, source_ml_source1_temporary_index98.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value854, _) => {
                                let source_ml_source2_data_property_value854 = *source_ml_source2_data_property_value854;
                                source_ml_source2_data_property_value854.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value855) => {
                                    source_ml_source2_data_property_value855
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index98_2 = source_ml_source1_temporary_index98.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index98) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index98_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop102Done(Box::new(current.clone()), arguments_)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop104(mut current: self::JavaScriptCallSourceBuilder, arguments_: &[f64], mut index: f64) -> self::JavaScriptCallSourceBuilder {
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
