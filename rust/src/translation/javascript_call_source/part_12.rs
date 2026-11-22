// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_argument_list;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop45;
use super::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop46;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_call_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::lower_java_script_call_source_argument_list(source.clone(), bindings.clone(), state.clone(), (depth + 1f64))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "argument spread requires its actual surrounding argument List")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClosure(owner, ordinal, source_captures, source_receiver) => {
            let source_receiver = *source_receiver;
            if ((((owner <= (self::ml::Big::from_i128(0))) || (owner > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::test_java_script_call_code_coordinate(ordinal, 4096f64))) || (({ let ml_reference_array82 = source_captures.clone(); let ml_reference_projection82 = ml_reference_array82.borrow().len() as f64; ml_reference_projection82 }) > (4096f64))) {
                self::refuse_java_script_call_source(state.clone(), "source closure requires its actual static owner, lexical site and bounded capture vector")
            } else {
                {
                    let receiver = self::lower_java_script_call_source_expression(source_receiver.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let saved_receiver = self::allocate_java_script_call_source_register(match receiver.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10560, _) => {
                                let source_ml_source2_data_property_value10560 = *source_ml_source2_data_property_value10560;
                                source_ml_source2_data_property_value10560.clone()
                            }
                        });
                        {
                            let current = self::append_java_script_call_source_instruction(match saved_receiver.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10561, _) => {
                                    let source_ml_source2_data_property_value10561 = *source_ml_source2_data_property_value10561;
                                    source_ml_source2_data_property_value10561.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved_receiver.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10562) => {
                                    source_ml_source2_data_property_value10562
                                }
                            }, match receiver.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10563) => {
                                    source_ml_source2_data_property_value10563
                                }
                            }));
                            {
                                let captures = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let source_ml_source1_temporary_index43 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s10564 = self::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop45(bindings.clone(), depth, current.clone(), captures.clone(), source_captures.clone(), source_ml_source1_temporary_index43.clone());
                                        match source_ml_source2_s10564.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop45Done(current_2, captures_2) => {
                                                let current_2 = *current_2;
                                                {
                                                    let result = self::allocate_java_script_call_source_register(current_2.clone());
                                                    {
                                                        let source_ml_source2_nominal_field10569 = match result.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10570, _) => {
                                                                let source_ml_source2_data_property_value10570 = *source_ml_source2_data_property_value10570;
                                                                source_ml_source2_data_property_value10570.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field10571 = match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10572) => {
                                                                    source_ml_source2_data_property_value10572
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override10573 = self::append_java_script_call_source_instruction(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10565, _) => {
                                                                        let source_ml_source2_data_property_value10565 = *source_ml_source2_data_property_value10565;
                                                                        source_ml_source2_data_property_value10565.clone()
                                                                    }
                                                                }, self::JavaScriptCallInstruction::ClosureAllocate(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10566) => {
                                                                        source_ml_source2_data_property_value10566
                                                                    }
                                                                }, owner.clone(), ordinal, captures_2.clone(), match saved_receiver.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10567) => {
                                                                        source_ml_source2_data_property_value10567
                                                                    }
                                                                }));
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override10573.clone()), source_ml_source2_nominal_field10571)
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
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source call lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCall(source_callee, source_receiver_2, arguments) => {
            let source_callee = *source_callee;
            let source_receiver_2 = *source_receiver_2;
            if { let ml_values83 = { let ml_reference_value84 = arguments.borrow().clone(); ml_reference_value84 }; let ml_callback83 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                    true
                }
                _ => {
                    false
                }
            } }); ml_values83.iter().cloned().enumerate().any(|(ml_index83, ml_value83)| ml_callback83(ml_value83)) } {
                self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee.clone()), Box::new(source_receiver_2.clone()), arguments.clone(), String::from("call"), false), bindings.clone(), state.clone(), (depth + 1f64))
            } else {
                if (({ let ml_reference_array85 = arguments.clone(); let ml_reference_projection85 = ml_reference_array85.borrow().len() as f64; ml_reference_projection85 }) > (4096f64)) {
                    self::refuse_java_script_call_source(state.clone(), "source call operands exceed their explicit budget")
                } else {
                    {
                        let callee = self::lower_java_script_call_source_operand(source_callee.clone(), bindings.clone(), state.clone(), (depth + 1f64), false);
                        {
                            let receiver_2 = self::lower_java_script_call_source_operand(source_receiver_2.clone(), bindings.clone(), match callee.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10942, _) => {
                                    let source_ml_source2_data_property_value10942 = *source_ml_source2_data_property_value10942;
                                    source_ml_source2_data_property_value10942.clone()
                                }
                            }, (depth + 1f64), false);
                            {
                                let current_3 = match receiver_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10943, _) => {
                                        let source_ml_source2_data_property_value10943 = *source_ml_source2_data_property_value10943;
                                        source_ml_source2_data_property_value10943.clone()
                                    }
                                };
                                {
                                    let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source1_temporary_index44 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s10944 = self::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop46(bindings.clone(), depth, false, current_3.clone(), arguments_.clone(), arguments.clone(), source_ml_source1_temporary_index44.clone());
                                            match source_ml_source2_s10944.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Done(current_4, arguments__2) => {
                                                    let current_4 = *current_4;
                                                    {
                                                        let result_2 = self::allocate_java_script_call_source_register(current_4.clone());
                                                        {
                                                            let source_ml_source2_nominal_field10950 = match result_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10951, _) => {
                                                                    let source_ml_source2_data_property_value10951 = *source_ml_source2_data_property_value10951;
                                                                    source_ml_source2_data_property_value10951.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field10952 = match result_2.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10953) => {
                                                                        source_ml_source2_data_property_value10953
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override10954 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10945, _) => {
                                                                            let source_ml_source2_data_property_value10945 = *source_ml_source2_data_property_value10945;
                                                                            source_ml_source2_data_property_value10945.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Call(match result_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10946) => {
                                                                            source_ml_source2_data_property_value10946
                                                                        }
                                                                    }, match callee.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10947) => {
                                                                            source_ml_source2_data_property_value10947
                                                                        }
                                                                    }, match receiver_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10948) => {
                                                                            source_ml_source2_data_property_value10948
                                                                        }
                                                                    }, arguments__2.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override10954.clone()), source_ml_source2_nominal_field10952)
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
        self::JavaScriptCallSource::SourceSnapshotCall(source_callee_2, source_receiver_3, arguments_2) => {
            let source_callee_2 = *source_callee_2;
            let source_receiver_3 = *source_receiver_3;
            if { let ml_values86 = { let ml_reference_value87 = arguments_2.borrow().clone(); ml_reference_value87 }; let ml_callback86 = std::rc::Rc::new(move |argument: self::JavaScriptCallSource| -> bool { match argument.clone() {
                self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                    true
                }
                _ => {
                    false
                }
            } }); ml_values86.iter().cloned().enumerate().any(|(ml_index86, ml_value86)| ml_callback86(ml_value86)) } {
                self::lower_java_script_call_source_argument_list(self::JavaScriptCallSource::SourceVariadicCall(Box::new(source_callee_2.clone()), Box::new(source_receiver_3.clone()), arguments_2.clone(), String::from("call"), false), bindings.clone(), state.clone(), (depth + 1f64))
            } else {
                if (({ let ml_reference_array88 = arguments_2.clone(); let ml_reference_projection88 = ml_reference_array88.borrow().len() as f64; ml_reference_projection88 }) > (4096f64)) {
                    self::refuse_java_script_call_source(state.clone(), "source call operands exceed their explicit budget")
                } else {
                    {
                        let callee_2 = self::lower_java_script_call_source_operand(source_callee_2.clone(), bindings.clone(), state.clone(), (depth + 1f64), true);
                        {
                            let receiver_3 = self::lower_java_script_call_source_operand(source_receiver_3.clone(), bindings.clone(), match callee_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11323, _) => {
                                    let source_ml_source2_data_property_value11323 = *source_ml_source2_data_property_value11323;
                                    source_ml_source2_data_property_value11323.clone()
                                }
                            }, (depth + 1f64), true);
                            {
                                let current_5 = match receiver_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11324, _) => {
                                        let source_ml_source2_data_property_value11324 = *source_ml_source2_data_property_value11324;
                                        source_ml_source2_data_property_value11324.clone()
                                    }
                                };
                                {
                                    let arguments__3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source1_temporary_index44_2 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s11325 = self::source_ml_source1_temporary_lower_java_script_call_source_call_expression_loop46(bindings.clone(), depth, true, current_5.clone(), arguments__3.clone(), arguments_2.clone(), source_ml_source1_temporary_index44_2.clone());
                                            match source_ml_source2_s11325.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Result::MlSource1TemporaryLowerJavaScriptCallSourceCallExpressionLoop46Done(current_6, arguments__4) => {
                                                    let current_6 = *current_6;
                                                    {
                                                        let result_3 = self::allocate_java_script_call_source_register(current_6.clone());
                                                        {
                                                            let source_ml_source2_nominal_field11331 = match result_3.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11332, _) => {
                                                                    let source_ml_source2_data_property_value11332 = *source_ml_source2_data_property_value11332;
                                                                    source_ml_source2_data_property_value11332.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field11333 = match result_3.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11334) => {
                                                                        source_ml_source2_data_property_value11334
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override11335 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11326, _) => {
                                                                            let source_ml_source2_data_property_value11326 = *source_ml_source2_data_property_value11326;
                                                                            source_ml_source2_data_property_value11326.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Call(match result_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11327) => {
                                                                            source_ml_source2_data_property_value11327
                                                                        }
                                                                    }, match callee_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11328) => {
                                                                            source_ml_source2_data_property_value11328
                                                                        }
                                                                    }, match receiver_3.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11329) => {
                                                                            source_ml_source2_data_property_value11329
                                                                        }
                                                                    }, arguments__4.clone()));
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override11335.clone()), source_ml_source2_nominal_field11333)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_operand(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64, snapshot: bool) -> self::JavaScriptCallSourceResult {
    {
        let value = self::lower_java_script_call_source_expression(source.clone(), bindings.clone(), builder.clone(), depth);
        if !snapshot {
            value.clone()
        } else {
            {
                let saved = self::allocate_java_script_call_source_register(match value.clone() {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11336, _) => {
                        let source_ml_source2_data_property_value11336 = *source_ml_source2_data_property_value11336;
                        source_ml_source2_data_property_value11336.clone()
                    }
                });
                {
                    let source_ml_source2_nominal_field11341 = match saved.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11342, _) => {
                            let source_ml_source2_data_property_value11342 = *source_ml_source2_data_property_value11342;
                            source_ml_source2_data_property_value11342.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field11343 = match saved.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11344) => {
                                source_ml_source2_data_property_value11344
                            }
                        };
                        {
                            let source_ml_source2_nominal_override11345 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11337, _) => {
                                    let source_ml_source2_data_property_value11337 = *source_ml_source2_data_property_value11337;
                                    source_ml_source2_data_property_value11337.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11338) => {
                                    source_ml_source2_data_property_value11338
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11339) => {
                                    source_ml_source2_data_property_value11339
                                }
                            }));
                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override11345.clone()), source_ml_source2_nominal_field11343)
                        }
                    }
                }
            }
        }
    }
}
