// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_argument_spread;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_operand;
use super::ml;
use super::ml_array;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_find_java_script_call_source_spread_iterator_slot_loop51;
use super::source_ml_source1_temporary_lower_java_script_call_source_argument_list_loop49;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_argument_list(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceVariadicCall(source_callee, receiver, arguments, mode, receiver_after_arguments) => {
            let source_callee = *source_callee;
            let receiver = *receiver;
            if (!(self::ml_array::includes(&{ let ml_reference_value89 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("call"), String::from("collection"), String::from("error class"), String::from("ordinary class"), String::from("construct")]))).borrow().clone(); ml_reference_value89 }, &mode.clone(), 0f64, false, |a, b| a == b)) || (receiver_after_arguments && (mode != "ordinary class"))) {
                self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
            } else {
                if (({ let ml_reference_array90 = arguments.clone(); let ml_reference_projection90 = ml_reference_array90.borrow().len() as f64; ml_reference_projection90 }) > (4096f64)) {
                    self::refuse_java_script_call_source(builder.clone(), "variadic source exceeds its authored argument expression budget")
                } else {
                    {
                        let selected = self::lower_java_script_call_source_expression(source_callee.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                        {
                            let callee = self::allocate_java_script_call_source_register(match selected.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11471, _) => {
                                    let source_ml_source2_data_property_value11471 = *source_ml_source2_data_property_value11471;
                                    source_ml_source2_data_property_value11471.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match callee.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11472, _) => {
                                        let source_ml_source2_data_property_value11472 = *source_ml_source2_data_property_value11472;
                                        source_ml_source2_data_property_value11472.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match callee.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11473) => {
                                        source_ml_source2_data_property_value11473
                                    }
                                }, match selected.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11474) => {
                                        source_ml_source2_data_property_value11474
                                    }
                                }));
                                {
                                    let receiver_register = 0f64;
                                    {
                                        let source_ml_source2_s11481 = if (mode == "construct") {
                                            {
                                                let receiver_register_2 = match callee.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11482) => {
                                                        source_ml_source2_data_property_value11482
                                                    }
                                                };
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(Box::new(current.clone()), receiver_register_2)
                                            }
                                        } else {
                                            if !receiver_after_arguments {
                                                {
                                                    let raw_ = self::lower_java_script_call_source_expression(receiver.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                                    {
                                                        let held = self::allocate_java_script_call_source_register(match raw_.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11483, _) => {
                                                                let source_ml_source2_data_property_value11483 = *source_ml_source2_data_property_value11483;
                                                                source_ml_source2_data_property_value11483.clone()
                                                            }
                                                        });
                                                        {
                                                            let current_2 = self::append_java_script_call_source_instruction(match held.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11484, _) => {
                                                                    let source_ml_source2_data_property_value11484 = *source_ml_source2_data_property_value11484;
                                                                    source_ml_source2_data_property_value11484.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11485) => {
                                                                    source_ml_source2_data_property_value11485
                                                                }
                                                            }, match raw_.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11486) => {
                                                                    source_ml_source2_data_property_value11486
                                                                }
                                                            }));
                                                            {
                                                                let receiver_register_3 = match held.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11487) => {
                                                                        source_ml_source2_data_property_value11487
                                                                    }
                                                                };
                                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(Box::new(current_2.clone()), receiver_register_3)
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(Box::new(current.clone()), receiver_register)
                                            }
                                        };
                                        match source_ml_source2_s11481.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin48Next(current_3, receiver_register_4) => {
                                                let current_3 = *current_3;
                                                {
                                                    let slot = { let ml_reference_array91 = match current_3.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11488, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value11488.clone()
                                                        }
                                                    }; let ml_reference_projection91 = ml_reference_array91.borrow().len() as f64; ml_reference_projection91 };
                                                    {
                                                        let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::ArgumentListOpen(slot));
                                                        {
                                                            let source_ml_source1_temporary_index47 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s11489 = self::source_ml_source1_temporary_lower_java_script_call_source_argument_list_loop49(bindings.clone(), builder.clone(), depth, current_4.clone(), slot, arguments.clone(), source_ml_source1_temporary_index47.clone());
                                                                match source_ml_source2_s11489.clone() {
                                                                    self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Done(current_5) => {
                                                                        let current_5 = *current_5;
                                                                        {
                                                                            let source_ml_source2_s11495 = if receiver_after_arguments {
                                                                                {
                                                                                    let raw_2 = self::lower_java_script_call_source_expression(receiver.clone(), bindings.clone(), current_5.clone(), (depth + 1f64));
                                                                                    {
                                                                                        let held_2 = self::allocate_java_script_call_source_register(match raw_2.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11496, _) => {
                                                                                                let source_ml_source2_data_property_value11496 = *source_ml_source2_data_property_value11496;
                                                                                                source_ml_source2_data_property_value11496.clone()
                                                                                            }
                                                                                        });
                                                                                        {
                                                                                            let current_6 = self::append_java_script_call_source_instruction(match held_2.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11497, _) => {
                                                                                                    let source_ml_source2_data_property_value11497 = *source_ml_source2_data_property_value11497;
                                                                                                    source_ml_source2_data_property_value11497.clone()
                                                                                                }
                                                                                            }, self::JavaScriptCallInstruction::Copy(match held_2.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11498) => {
                                                                                                    source_ml_source2_data_property_value11498
                                                                                                }
                                                                                            }, match raw_2.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11499) => {
                                                                                                    source_ml_source2_data_property_value11499
                                                                                                }
                                                                                            }));
                                                                                            {
                                                                                                let receiver_register_5 = match held_2.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11500) => {
                                                                                                        source_ml_source2_data_property_value11500
                                                                                                    }
                                                                                                };
                                                                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50Next(Box::new(current_6.clone()), receiver_register_5)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50Next(Box::new(current_5.clone()), receiver_register_4)
                                                                            };
                                                                            match source_ml_source2_s11495.clone() {
                                                                                self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListJoin50Next(current_7, receiver_register_6) => {
                                                                                    let current_7 = *current_7;
                                                                                    {
                                                                                        let result = self::allocate_java_script_call_source_register(current_7.clone());
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11505 = match result.clone() {
                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11506, _) => {
                                                                                                    let source_ml_source2_data_property_value11506 = *source_ml_source2_data_property_value11506;
                                                                                                    source_ml_source2_data_property_value11506.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11507 = match result.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11508) => {
                                                                                                        source_ml_source2_data_property_value11508
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override11509 = self::append_java_script_call_source_instruction(match result.clone() {
                                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11501, _) => {
                                                                                                            let source_ml_source2_data_property_value11501 = *source_ml_source2_data_property_value11501;
                                                                                                            source_ml_source2_data_property_value11501.clone()
                                                                                                        }
                                                                                                    }, self::JavaScriptCallInstruction::ArgumentListCall(match result.clone() {
                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11502) => {
                                                                                                            source_ml_source2_data_property_value11502
                                                                                                        }
                                                                                                    }, slot, match callee.clone() {
                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11503) => {
                                                                                                            source_ml_source2_data_property_value11503
                                                                                                        }
                                                                                                    }, receiver_register_6, mode.clone()));
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override11509.clone()), source_ml_source2_nominal_field11507)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Result::MlSource1TemporaryLowerJavaScriptCallSourceArgumentListLoop49Return(source_ml_source1_temporary_result50) => {
                                                                        let source_ml_source1_temporary_result50 = *source_ml_source1_temporary_result50;
                                                                        source_ml_source1_temporary_result50.clone()
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
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "variadic source retains its actual invocation and receiver allocation policy")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_argument(argument: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64, slot: f64, handlers: f64) -> std::rc::Rc<std::cell::RefCell<(self::JavaScriptCallSourceBuilder, bool)>> {
    match argument.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceArgumentSpread(value, iterator_key, realm) => {
            let value = *value;
            let iterator_key = *iterator_key;
            self::lower_java_script_call_source_spread_argument(value.clone(), iterator_key.clone(), realm.clone(), bindings.clone(), builder.clone(), depth, slot, handlers)
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::lower_java_script_call_source_plain_argument(argument.clone(), bindings.clone(), builder.clone(), depth, slot)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_plain_argument(argument: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64, slot: f64) -> std::rc::Rc<std::cell::RefCell<(self::JavaScriptCallSourceBuilder, bool)>> {
    {
        let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), builder.clone(), depth);
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_field11693 = self::append_java_script_call_source_instruction(match value.clone() {
                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11691, _) => {
                    let source_ml_source2_data_property_value11691 = *source_ml_source2_data_property_value11691;
                    source_ml_source2_data_property_value11691.clone()
                }
            }, self::JavaScriptCallInstruction::ArgumentListAppend(slot, match value.clone() {
                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11692) => {
                    source_ml_source2_data_property_value11692
                }
            }));
            (source_ml_source2_field11693.clone(), false)
        }))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_spread_argument(argument: self::JavaScriptCallSource, iterator_key: self::JavaScriptCallSource, realm: self::ml::Big, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64, slot: f64, handlers: f64) -> std::rc::Rc<std::cell::RefCell<(self::JavaScriptCallSourceBuilder, bool)>> {
    {
        let input = self::lower_java_script_call_source_operand(argument.clone(), bindings.clone(), builder.clone(), depth, true);
        {
            let key = self::lower_java_script_call_source_operand(iterator_key.clone(), bindings.clone(), match input.clone() {
                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11695, _) => {
                    let source_ml_source2_data_property_value11695 = *source_ml_source2_data_property_value11695;
                    source_ml_source2_data_property_value11695.clone()
                }
            }, depth, true);
            {
                let value = self::allocate_java_script_call_source_register(match key.clone() {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11696, _) => {
                        let source_ml_source2_data_property_value11696 = *source_ml_source2_data_property_value11696;
                        source_ml_source2_data_property_value11696.clone()
                    }
                });
                {
                    let done = self::allocate_java_script_call_source_register(match value.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11697, _) => {
                            let source_ml_source2_data_property_value11697 = *source_ml_source2_data_property_value11697;
                            source_ml_source2_data_property_value11697.clone()
                        }
                    });
                    {
                        let iterator = self::find_java_script_call_source_spread_iterator_slot(match match done.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11698, _) => {
                                let source_ml_source2_data_property_value11698 = *source_ml_source2_data_property_value11698;
                                source_ml_source2_data_property_value11698.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11699, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11699.clone()
                            }
                        });
                        if ((iterator >= (65536f64)) || ((match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11701, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11701
                            }
                        }) >= (1024f64))) {
                            std::rc::Rc::new(std::cell::RefCell::new((builder.clone(), true)))
                        } else {
                            {
                                let acquired = self::append_java_script_call_source_instruction(match done.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11704, _) => {
                                        let source_ml_source2_data_property_value11704 = *source_ml_source2_data_property_value11704;
                                        source_ml_source2_data_property_value11704.clone()
                                    }
                                }, {
                                    let source_ml_source2_constructor_field11706 = match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11705) => {
                                            source_ml_source2_data_property_value11705
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field11708 = match done.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11707) => {
                                                source_ml_source2_data_property_value11707
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field11710 = match input.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11709) => {
                                                    source_ml_source2_data_property_value11709
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field11712 = match key.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11711) => {
                                                        source_ml_source2_data_property_value11711
                                                    }
                                                };
                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field11706, source_ml_source2_constructor_field11708, source_ml_source2_constructor_field11710, source_ml_source2_constructor_field11712, iterator, String::from("acquire"), realm.clone(), None::<(Option<bool>,)>)
                                            }
                                        }
                                    }
                                });
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field11718 = self::append_java_script_call_source_argument_spread(acquired.clone(), slot, iterator, realm.clone(), match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11716) => {
                                            source_ml_source2_data_property_value11716
                                        }
                                    }, match done.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11717) => {
                                            source_ml_source2_data_property_value11717
                                        }
                                    }, handlers);
                                    (source_ml_source2_field11718.clone(), false)
                                }))
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
pub fn find_java_script_call_source_spread_iterator_slot(instructions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>) -> f64 {
    {
        let iterator = 0f64;
        {
            let iterator_2 = self::source_ml_source1_temporary_find_java_script_call_source_spread_iterator_slot_loop51(instructions.clone(), iterator);
            iterator_2
        }
    }
}
