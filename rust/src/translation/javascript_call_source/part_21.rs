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
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::push_java_script_call_source_control_scope;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_for_of(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(slot, input, iterator_key, body, realm) => {
            let input = *input;
            let iterator_key = *iterator_key;
            let body = *body;
            self::lower_java_script_call_source_iterator_scope(slot, realm.clone(), input.clone(), iterator_key.clone(), body.clone(), bindings.clone(), builder.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceForOf(slot_2, target, binding, mutable, input_2, iterator_key_2, source_body, realm_2, source_scope_bindings) => {
            let input_2 = *input_2;
            let iterator_key_2 = *iterator_key_2;
            let source_body = *source_body;
            {
                let scope_bindings = ((source_scope_bindings.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone();
                {
                    let body_2 = if !(match (scope_bindings.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                        {
                            let source_ml_source2_constructor_field14149 = Some(scope_bindings.clone().expect("a proven present value"));
                            self::JavaScriptCallSource::SourceIteratorLoop(slot_2, target, binding, mutable, Box::new(source_body.clone()), realm_2.clone(), Some((source_ml_source2_constructor_field14149.clone(),)))
                        }
                    } else {
                        self::JavaScriptCallSource::SourceIteratorLoop(slot_2, target, binding, mutable, Box::new(source_body.clone()), realm_2.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>)
                    };
                    self::lower_java_script_call_source_iterator_scope(slot_2, realm_2.clone(), input_2.clone(), iterator_key_2.clone(), body_2.clone(), bindings.clone(), builder.clone(), depth)
                }
            }
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_iterator_scope(slot: f64, realm: self::ml::Big, input_source: self::JavaScriptCallSource, iterator_key: self::JavaScriptCallSource, body: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((!(self::test_java_script_call_code_coordinate(slot, 65536f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
        self::refuse_java_script_call_source(builder.clone(), "for-of retains its actual bounded source region and realm")
    } else {
        {
            let input = self::lower_java_script_call_source_expression(input_source.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
            {
                let held = self::allocate_java_script_call_source_register(match input.clone() {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14156, _) => {
                        let source_ml_source2_data_property_value14156 = *source_ml_source2_data_property_value14156;
                        source_ml_source2_data_property_value14156.clone()
                    }
                });
                {
                    let saved = self::append_java_script_call_source_instruction(match held.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14157, _) => {
                            let source_ml_source2_data_property_value14157 = *source_ml_source2_data_property_value14157;
                            source_ml_source2_data_property_value14157.clone()
                        }
                    }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14158) => {
                            source_ml_source2_data_property_value14158
                        }
                    }, match input.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14159) => {
                            source_ml_source2_data_property_value14159
                        }
                    }));
                    {
                        let key = self::lower_java_script_call_source_expression(iterator_key.clone(), bindings.clone(), saved.clone(), (depth + 1f64));
                        {
                            let held_key = self::allocate_java_script_call_source_register(match key.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14160, _) => {
                                    let source_ml_source2_data_property_value14160 = *source_ml_source2_data_property_value14160;
                                    source_ml_source2_data_property_value14160.clone()
                                }
                            });
                            {
                                let keyed = self::append_java_script_call_source_instruction(match held_key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14161, _) => {
                                        let source_ml_source2_data_property_value14161 = *source_ml_source2_data_property_value14161;
                                        source_ml_source2_data_property_value14161.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match held_key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14162) => {
                                        source_ml_source2_data_property_value14162
                                    }
                                }, match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14163) => {
                                        source_ml_source2_data_property_value14163
                                    }
                                }));
                                {
                                    let value = self::allocate_java_script_call_source_register(keyed.clone());
                                    {
                                        let done = self::allocate_java_script_call_source_register(match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14164, _) => {
                                                let source_ml_source2_data_property_value14164 = *source_ml_source2_data_property_value14164;
                                                source_ml_source2_data_property_value14164.clone()
                                            }
                                        });
                                        {
                                            let opened = self::append_java_script_call_source_instruction(match done.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14165, _) => {
                                                    let source_ml_source2_data_property_value14165 = *source_ml_source2_data_property_value14165;
                                                    source_ml_source2_data_property_value14165.clone()
                                                }
                                            }, {
                                                let source_ml_source2_constructor_field14167 = match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14166) => {
                                                        source_ml_source2_data_property_value14166
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_constructor_field14169 = match done.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14168) => {
                                                            source_ml_source2_data_property_value14168
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_constructor_field14171 = match held.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14170) => {
                                                                source_ml_source2_data_property_value14170
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field14173 = match held_key.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14172) => {
                                                                    source_ml_source2_data_property_value14172
                                                                }
                                                            };
                                                            self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field14167, source_ml_source2_constructor_field14169, source_ml_source2_constructor_field14171, source_ml_source2_constructor_field14173, slot, String::from("acquire"), realm.clone(), None::<(Option<bool>,)>)
                                                        }
                                                    }
                                                }
                                            });
                                            {
                                                let finalizer = self::JavaScriptCallSource::SourceIteratorClose(slot, realm.clone());
                                                self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceFinally(Box::new(body.clone()), Box::new(finalizer.clone())), bindings.clone(), opened.clone(), (depth + 1f64))
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
pub fn lower_java_script_call_source_iterator_loop(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    {
        let binding_error = String::from("iterator loop retains its actual independent source binding");
        match source.clone() {
            self::JavaScriptCallSource::SourceConstant(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceNewTarget => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceArgument(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceDerivedReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceErrorIntrinsic => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceConstruct(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceThrow(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceReturn(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceControlJump(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceIteratorLoop(slot, target, binding, mutable, source_body, realm, scope_bindings) => {
                let source_body = *source_body;
                if ((((!(self::test_java_script_call_code_coordinate(binding, 65536f64)) || !(self::test_java_script_call_code_coordinate(slot, 65536f64))) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) || { let ml_values189 = { let ml_reference_value190 = bindings.borrow().clone(); ml_reference_value190 }; let ml_callback189 = { let binding = binding; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14355, _, _, _) => {
                        source_ml_source2_data_property_value14355
                    }
                }) == binding) }) }; ml_values189.iter().cloned().enumerate().any(|(ml_index189, ml_value189)| ml_callback189(ml_value189)) }) {
                    self::refuse_java_script_call_source(builder.clone(), &(binding_error))
                } else {
                    {
                        let scope = (((scope_bindings.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                        if ((({ let ml_reference_array191 = scope.clone(); let ml_reference_projection191 = ml_reference_array191.borrow().len() as f64; ml_reference_projection191 }) > (4096f64)) || { let ml_values192 = { let ml_reference_value193 = scope.borrow().clone(); ml_reference_value193 }; let ml_callback192 = { let scope = scope.clone(); let bindings = bindings.clone(); let binding = binding; std::rc::Rc::new(move |identity: f64, index: f64| -> bool { (((!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || ((self::ml_array::index_of(&{ let ml_reference_value194 = scope.borrow().clone(); ml_reference_value194 }, &identity, 0f64, false, |a, b| a == b)) != index)) || !({ let ml_values195 = { let ml_reference_value196 = bindings.borrow().clone(); ml_reference_value196 }; let ml_callback195 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14357, _, _, _) => {
                                source_ml_source2_data_property_value14357
                            }
                        }) == identity) }) }; ml_values195.iter().cloned().enumerate().any(|(ml_index195, ml_value195)| ml_callback195(ml_value195)) })) || (identity == binding)) }) }; ml_values192.iter().cloned().enumerate().any(|(ml_index192, ml_value192)| ml_callback192(ml_value192, (ml_index192 as f64))) }) {
                            self::refuse_java_script_call_source(builder.clone(), "iterator lexical scopes retain their actual outer TDZ cells and distinct received value")
                        } else {
                            {
                                let visible = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values197 = { let ml_reference_value198 = bindings.borrow().clone(); ml_reference_value198 }; let ml_callback197 = { let scope = scope.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { !(self::ml_array::includes(&{ let ml_reference_value199 = scope.borrow().clone(); ml_reference_value199 }, &match row.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14358, _, _, _) => {
                                        source_ml_source2_data_property_value14358
                                    }
                                }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values197.iter().cloned().enumerate().filter_map(|(ml_index197, ml_value197)| if ml_callback197(ml_value197.clone()) { Some(ml_value197) } else { None }).collect::<Vec<_>>() }));
                                {
                                    let scoped = self::push_java_script_call_source_control_scope(builder.clone(), target, "loop");
                                    {
                                        let value = self::allocate_java_script_call_source_register(scoped.clone());
                                        {
                                            let done = self::allocate_java_script_call_source_register(match value.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14359, _) => {
                                                    let source_ml_source2_data_property_value14359 = *source_ml_source2_data_property_value14359;
                                                    source_ml_source2_data_property_value14359.clone()
                                                }
                                            });
                                            {
                                                let start = { let ml_reference_array200 = match match done.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14360, _) => {
                                                        let source_ml_source2_data_property_value14360 = *source_ml_source2_data_property_value14360;
                                                        source_ml_source2_data_property_value14360.clone()
                                                    }
                                                } {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14361, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value14361.clone()
                                                    }
                                                }; let ml_reference_projection200 = ml_reference_array200.borrow().len() as f64; ml_reference_projection200 };
                                                {
                                                    let stepped = self::append_java_script_call_source_instruction(match done.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14362, _) => {
                                                            let source_ml_source2_data_property_value14362 = *source_ml_source2_data_property_value14362;
                                                            source_ml_source2_data_property_value14362.clone()
                                                        }
                                                    }, {
                                                        let source_ml_source2_constructor_field14364 = match value.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14363) => {
                                                                source_ml_source2_data_property_value14363
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field14366 = match done.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14365) => {
                                                                    source_ml_source2_data_property_value14365
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_constructor_field14367 = 0f64;
                                                                {
                                                                    let source_ml_source2_constructor_field14368 = 0f64;
                                                                    self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field14364, source_ml_source2_constructor_field14366, source_ml_source2_constructor_field14367, source_ml_source2_constructor_field14368, slot, String::from("step"), realm.clone(), None::<(Option<bool>,)>)
                                                                }
                                                            }
                                                        }
                                                    });
                                                    {
                                                        let branch = { let ml_reference_array201 = match stepped.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14372, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value14372.clone()
                                                            }
                                                        }; let ml_reference_projection201 = ml_reference_array201.borrow().len() as f64; ml_reference_projection201 };
                                                        {
                                                            let test = self::append_java_script_call_source_instruction(stepped.clone(), self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14373) => {
                                                                    source_ml_source2_data_property_value14373
                                                                }
                                                            }, 0f64, (branch + 1f64)));
                                                            {
                                                                let body = self::lower_java_script_call_source_expression(source_body.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value14379 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                        let source_ml_source2_constructor_field14376 = match value.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14375) => {
                                                                                source_ml_source2_data_property_value14375
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(binding, source_ml_source2_constructor_field14376, mutable, None::<(Option<bool>,)>)
                                                                    }]));
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value202 = visible.borrow().clone(); ml_reference_value202 }), { let ml_reference_value203 = source_ml_source2_concat_value14379.borrow().clone(); ml_reference_value203 })
                                                                })), test.clone(), (depth + 1f64));
                                                                {
                                                                    let loop_ = self::append_java_script_call_source_instruction(match body.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14380, _) => {
                                                                            let source_ml_source2_data_property_value14380 = *source_ml_source2_data_property_value14380;
                                                                            source_ml_source2_data_property_value14380.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Jump(start));
                                                                    {
                                                                        let end = { let ml_reference_array204 = match loop_.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14381, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value14381.clone()
                                                                            }
                                                                        }; let ml_reference_projection204 = ml_reference_array204.borrow().len() as f64; ml_reference_projection204 };
                                                                        {
                                                                            let patched = {
                                                                                let source_ml_source2_nominal_spread14384 = loop_.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field14385 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14386, _, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value14386.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field14387 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value14388, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value14388
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field14389 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value14390, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value14390
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field14391 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value14392, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value14392
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field14393 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value14394, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value14394
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field14395 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value14396, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value14396.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field14397 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value14398, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value14398.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14399 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value14400, _, _) => {
                                                                                                                        source_ml_source2_data_property_value14400.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14401 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14402, _) => {
                                                                                                                            source_ml_source2_data_property_value14402.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14403 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14404) => {
                                                                                                                                source_ml_source2_data_property_value14404.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override14405 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values205 = { let ml_reference_value206 = (match loop_.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14382, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value14382.clone()
                                                                                                                                }
                                                                                                                            }).borrow().clone(); ml_reference_value206 }; let ml_callback205 = { let branch = branch; let done = done.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                                                                                self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14383) => {
                                                                                                                                        source_ml_source2_data_property_value14383
                                                                                                                                    }
                                                                                                                                }, end, (branch + 1f64))
                                                                                                                            } else {
                                                                                                                                instruction.clone()
                                                                                                                            } }) }; ml_values205.iter().cloned().enumerate().map(|(ml_index205, ml_value205)| ml_callback205(ml_value205, (ml_index205 as f64))).collect::<Vec<_>>() }));
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override14405.clone(), source_ml_source2_nominal_field14387, source_ml_source2_nominal_field14389, source_ml_source2_nominal_field14391, source_ml_source2_nominal_field14393, source_ml_source2_nominal_field14395.clone(), source_ml_source2_nominal_field14397.clone(), source_ml_source2_nominal_field14399.clone(), source_ml_source2_nominal_field14401.clone(), source_ml_source2_nominal_field14403.clone())
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
                                                                            {
                                                                                let finished = self::finish_java_script_call_source_control_scope(patched.clone(), builder.clone(), target, end, start);
                                                                                self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined)), bindings.clone(), finished.clone(), (depth + 1f64))
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
            self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceFinally(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceTry(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceBinding(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceLet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceRecur(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceAllocate(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceObject(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceDelete(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceHas(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
            self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
        }
    }
}
