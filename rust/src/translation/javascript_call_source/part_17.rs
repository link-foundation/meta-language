// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceControlScope;
use super::JavaScriptCallSourceControlTransfer;
use super::JavaScriptCallSourceResult;
use super::append_java_script_call_source_instruction;
use super::ml_array;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_control_jump(source: self::JavaScriptCallSource, builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    {
        let target_error = String::from("a control jump requires its actual source target");
        match source.clone() {
            self::JavaScriptCallSource::SourceConstant(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceNewTarget => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArgument(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceDerivedReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceErrorIntrinsic => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceConstruct(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceThrow(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceReturn(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceControlJump(target, continuing) => {
                {
                    let scope = { let ml_values141 = { let ml_reference_value142 = ((match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12858, _) => {
                            ((source_ml_source2_data_property_value12858.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlScope>::new())))).borrow().clone(); ml_reference_value142 }; let ml_callback141 = { let target = target; std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlScope| -> bool { ((match row.clone() {
                        self::JavaScriptCallSourceControlScope::SourceControlScope(source_ml_source2_data_property_value12860, _, _) => {
                            source_ml_source2_data_property_value12860
                        }
                    }) == target) }) }; ml_values141.iter().cloned().enumerate().find_map(|(ml_index141, ml_value141)| if ml_callback141(ml_value141.clone()) { Some(ml_value141) } else { None }) };
                    if ((match (scope.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (continuing && ((match scope.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceControlScope::SourceControlScope(_, source_ml_source2_data_property_value12862, _) => {
                            source_ml_source2_data_property_value12862.clone()
                        }
                    }) != "loop"))) || ((match scope.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceControlScope::SourceControlScope(_, _, source_ml_source2_data_property_value12865) => {
                            source_ml_source2_data_property_value12865
                        }
                    }) > (match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12866, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12866
                        }
                    }))) {
                        self::refuse_java_script_call_source(builder.clone(), "break and continue retain an active actual target and handler depth")
                    } else {
                        {
                            let current = builder.clone();
                            {
                                let position = { let ml_reference_array143 = match current.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12867, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12867.clone()
                                    }
                                }; let ml_reference_projection143 = ml_reference_array143.borrow().len() as f64; ml_reference_projection143 };
                                {
                                    let current_2 = self::append_java_script_call_source_instruction(current.clone(), if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12870, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value12870
                                        }
                                    }) > (match scope.clone().expect("a proven present value") {
                                        self::JavaScriptCallSourceControlScope::SourceControlScope(_, _, source_ml_source2_data_property_value12871) => {
                                            source_ml_source2_data_property_value12871
                                        }
                                    })) {
                                        self::JavaScriptCallInstruction::CompleteJump(0f64, match scope.clone().expect("a proven present value") {
                                            self::JavaScriptCallSourceControlScope::SourceControlScope(_, _, source_ml_source2_data_property_value12872) => {
                                                source_ml_source2_data_property_value12872
                                            }
                                        })
                                    } else {
                                        self::JavaScriptCallInstruction::Jump(0f64)
                                    });
                                    {
                                        let transfers = (match current_2.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12873) => {
                                                ((source_ml_source2_data_property_value12873.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlTransfer>::new())));
                                        if (({ let ml_reference_array144 = transfers.clone(); let ml_reference_projection144 = ml_reference_array144.borrow().len() as f64; ml_reference_projection144 }) >= (65536f64)) {
                                            self::refuse_java_script_call_source(current_2.clone(), "control transfers exceed their explicit transfer budget")
                                        } else {
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                let source_ml_source2_nominal_field12877 = match current_2.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12878, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12878.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12879 = match current_2.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12880, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12880
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12881 = match current_2.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12882, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12882
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12883 = match current_2.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12884, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12884
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12885 = match current_2.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12886, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12886
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12887 = match current_2.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12888, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12888.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12889 = match current_2.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12890, _, _, _) => {
                                                                                source_ml_source2_data_property_value12890.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12891 = match current_2.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12892, _, _) => {
                                                                                    source_ml_source2_data_property_value12892.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12893 = match current_2.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12894, _) => {
                                                                                        source_ml_source2_data_property_value12894.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12895 = match current_2.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12896) => {
                                                                                            source_ml_source2_data_property_value12896.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override12897 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source2_concat_value12875 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(position, target, continuing)]));
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceControlTransfer>::new(), { let ml_reference_value145 = transfers.borrow().clone(); ml_reference_value145 }), { let ml_reference_value146 = source_ml_source2_concat_value12875.borrow().clone(); ml_reference_value146 })
                                                                                        }));
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12877.clone(), source_ml_source2_nominal_field12879, source_ml_source2_nominal_field12881, source_ml_source2_nominal_field12883, source_ml_source2_nominal_field12885, source_ml_source2_nominal_field12887.clone(), source_ml_source2_nominal_field12889.clone(), source_ml_source2_nominal_field12891.clone(), source_ml_source2_nominal_field12893.clone(), Some((Some(source_ml_source2_nominal_override12897.clone()),)))
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
                                            }), 0f64)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceFinally(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceTry(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceBinding(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceLet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceRecur(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceAllocate(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceObject(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceDelete(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceHas(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_call_source_control_scope(builder: self::JavaScriptCallSourceBuilder, target: f64, kind: &str) -> self::JavaScriptCallSourceBuilder {
    {
        let scopes = (match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12898, _) => {
                ((source_ml_source2_data_property_value12898.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlScope>::new())));
        if (((!(self::ml_array::includes(&{ let ml_reference_value147 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("loop"), String::from("switch"), String::from("block")]))).borrow().clone(); ml_reference_value147 }, &kind.to_owned(), 0f64, false, |a, b| a == b)) || !(self::test_java_script_call_code_coordinate(target, 65536f64))) || (({ let ml_reference_array148 = scopes.clone(); let ml_reference_projection148 = ml_reference_array148.borrow().len() as f64; ml_reference_projection148 }) >= (128f64))) || { let ml_values149 = { let ml_reference_value150 = scopes.borrow().clone(); ml_reference_value150 }; let ml_callback149 = { let target = target; std::rc::Rc::new(move |scope: self::JavaScriptCallSourceControlScope| -> bool { ((match scope.clone() {
            self::JavaScriptCallSourceControlScope::SourceControlScope(source_ml_source2_data_property_value12900, _, _) => {
                source_ml_source2_data_property_value12900
            }
        }) == target) }) }; ml_values149.iter().cloned().enumerate().any(|(ml_index149, ml_value149)| ml_callback149(ml_value149)) }) {
            {
                let source_ml_source2_nominal_field12902 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12903, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value12903.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field12904 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12905, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12905
                        }
                    };
                    {
                        let source_ml_source2_nominal_field12906 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12907, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12907
                            }
                        };
                        {
                            let source_ml_source2_nominal_field12908 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12909, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value12909
                                }
                            };
                            {
                                let source_ml_source2_nominal_field12910 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12911, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12911
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field12912 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12913, _, _, _, _) => {
                                            source_ml_source2_data_property_value12913.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field12914 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12915, _, _, _) => {
                                                source_ml_source2_data_property_value12915.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12916 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12917, _, _) => {
                                                    source_ml_source2_data_property_value12917.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12918 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12919, _) => {
                                                        source_ml_source2_data_property_value12919.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12920 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12921) => {
                                                            source_ml_source2_data_property_value12921.clone()
                                                        }
                                                    };
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12902.clone(), source_ml_source2_nominal_field12904, source_ml_source2_nominal_field12906, source_ml_source2_nominal_field12908, source_ml_source2_nominal_field12910, String::from("control scopes require unique actual source targets within their lexical budget"), source_ml_source2_nominal_field12914.clone(), source_ml_source2_nominal_field12916.clone(), source_ml_source2_nominal_field12918.clone(), source_ml_source2_nominal_field12920.clone())
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
        } else {
            {
                let source_ml_source2_nominal_field12927 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12928, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value12928.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field12929 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12930, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12930
                        }
                    };
                    {
                        let source_ml_source2_nominal_field12931 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12932, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12932
                            }
                        };
                        {
                            let source_ml_source2_nominal_field12933 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12934, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value12934
                                }
                            };
                            {
                                let source_ml_source2_nominal_field12935 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12936, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12936
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field12937 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12938, _, _, _, _) => {
                                            source_ml_source2_data_property_value12938.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field12939 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12940, _, _, _) => {
                                                source_ml_source2_data_property_value12940.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12941 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12942, _, _) => {
                                                    source_ml_source2_data_property_value12942.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12943 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12944, _) => {
                                                        source_ml_source2_data_property_value12944.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12945 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12946) => {
                                                            source_ml_source2_data_property_value12946.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override12947 = std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value12925 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSourceControlScope::SourceControlScope(target, kind.to_owned(), match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12923, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12923
                                                                }
                                                            })]));
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceControlScope>::new(), { let ml_reference_value151 = scopes.borrow().clone(); ml_reference_value151 }), { let ml_reference_value152 = source_ml_source2_concat_value12925.borrow().clone(); ml_reference_value152 })
                                                        }));
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12927.clone(), source_ml_source2_nominal_field12929, source_ml_source2_nominal_field12931, source_ml_source2_nominal_field12933, source_ml_source2_nominal_field12935, source_ml_source2_nominal_field12937.clone(), source_ml_source2_nominal_field12939.clone(), source_ml_source2_nominal_field12941.clone(), Some((Some(source_ml_source2_nominal_override12947.clone()),)), source_ml_source2_nominal_field12945.clone())
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
