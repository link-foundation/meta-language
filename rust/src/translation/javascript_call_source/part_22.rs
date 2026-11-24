// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_operand;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_binding_scope_loop73;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_binding_scope(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    {
        let declaration_error = String::from("binding output scopes retain their actual bounded declarations");
        match source.clone() {
            self::JavaScriptCallSource::SourceConstant(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceNewTarget => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceArgument(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceDerivedReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceErrorIntrinsic => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceConstruct(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceThrow(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceReturn(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceControlJump(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceBindingScope(declarations, initializer, body) => {
                let initializer = *initializer;
                let body = *body;
                if (({ let ml_reference_array207 = declarations.clone(); let ml_reference_projection207 = ml_reference_array207.borrow().len() as f64; ml_reference_projection207 }) > (4096f64)) {
                    self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
                } else {
                    {
                        let current = builder.clone();
                        {
                            let entries = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceBinding>::new()));
                            {
                                let source_ml_source1_temporary_values72 = declarations.clone();
                                {
                                    let source_ml_source1_temporary_index72 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s14587 = self::source_ml_source1_temporary_lower_java_script_call_source_binding_scope_loop73(bindings.clone(), current.clone(), entries.clone(), source_ml_source1_temporary_values72.clone(), source_ml_source1_temporary_index72.clone());
                                        match source_ml_source2_s14587.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Done(current_2) => {
                                                let current_2 = *current_2;
                                                {
                                                    let initialized = self::lower_java_script_call_source_expression(initializer.clone(), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value208 = bindings.borrow().clone(); ml_reference_value208 }), { let ml_reference_value209 = entries.borrow().clone(); ml_reference_value209 }))), current_2.clone(), (depth + 1f64));
                                                    if { let ml_values210 = { let ml_reference_value211 = entries.borrow().clone(); ml_reference_value211 }; let ml_callback210 = { let initialized = initialized.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { !(self::ml_array::includes(&{ let ml_reference_value212 = ((match match initialized.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14592, _) => {
                                                            let source_ml_source2_data_property_value14592 = *source_ml_source2_data_property_value14592;
                                                            source_ml_source2_data_property_value14592.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value14593, _, _) => {
                                                            ((source_ml_source2_data_property_value14593.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value212 }, &match row.clone() {
                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14594, _, _, _) => {
                                                            source_ml_source2_data_property_value14594
                                                        }
                                                    }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values210.iter().cloned().enumerate().any(|(ml_index210, ml_value210)| ml_callback210(ml_value210)) } {
                                                        self::refuse_java_script_call_source(match initialized.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14595, _) => {
                                                                let source_ml_source2_data_property_value14595 = *source_ml_source2_data_property_value14595;
                                                                source_ml_source2_data_property_value14595.clone()
                                                            }
                                                        }, "every binding output must complete initialization before its body")
                                                    } else {
                                                        self::lower_java_script_call_source_expression(body.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value14602 = bindings.clone();
                                                            {
                                                                let source_ml_source2_concat_value14603 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values213 = { let ml_reference_value214 = entries.borrow().clone(); ml_reference_value214 }; let ml_callback213 = std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> self::JavaScriptCallSourceBinding { {
                                                                    let source_ml_source2_constructor_field14597 = match row.clone() {
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14596, _, _, _) => {
                                                                            source_ml_source2_data_property_value14596
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field14599 = match row.clone() {
                                                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value14598, _, _) => {
                                                                                source_ml_source2_data_property_value14598
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field14601 = match row.clone() {
                                                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value14600, _) => {
                                                                                    source_ml_source2_data_property_value14600
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_constructor_field14597, source_ml_source2_constructor_field14599, source_ml_source2_constructor_field14601, None::<(Option<bool>,)>)
                                                                        }
                                                                    }
                                                                } }); ml_values213.iter().cloned().enumerate().map(|(ml_index213, ml_value213)| ml_callback213(ml_value213)).collect::<Vec<_>>() }));
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value215 = source_ml_source2_concat_value14602.borrow().clone(); ml_reference_value215 }), { let ml_reference_value216 = source_ml_source2_concat_value14603.borrow().clone(); ml_reference_value216 })
                                                            }
                                                        })), match initialized.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14604, _) => {
                                                                let source_ml_source2_data_property_value14604 = *source_ml_source2_data_property_value14604;
                                                                source_ml_source2_data_property_value14604.clone()
                                                            }
                                                        }, (depth + 1f64))
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Return(source_ml_source1_temporary_result73) => {
                                                let source_ml_source1_temporary_result73 = *source_ml_source1_temporary_result73;
                                                source_ml_source1_temporary_result73.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceFinally(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceTry(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceBinding(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceLet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceRecur(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceAllocate(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceObject(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceDelete(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceHas(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
            self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_iterator_remainder(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    {
        let region_error = String::from("iterator rest retains its actual opened region and realm");
        match source.clone() {
            self::JavaScriptCallSource::SourceConstant(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceNewTarget => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceArgument(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceDerivedReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceErrorIntrinsic => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceConstruct(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceThrow(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceReturn(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceControlJump(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceIteratorRemainder(slot, source_prototype, realm) => {
                let source_prototype = *source_prototype;
                if ((!(self::test_java_script_call_code_coordinate(slot, 65536f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::refuse_java_script_call_source(builder.clone(), &(region_error))
                } else {
                    {
                        let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                        {
                            let object = self::allocate_java_script_call_source_register(match prototype.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14786, _) => {
                                    let source_ml_source2_data_property_value14786 = *source_ml_source2_data_property_value14786;
                                    source_ml_source2_data_property_value14786.clone()
                                }
                            });
                            {
                                let created = self::append_java_script_call_source_instruction(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14787, _) => {
                                        let source_ml_source2_data_property_value14787 = *source_ml_source2_data_property_value14787;
                                        source_ml_source2_data_property_value14787.clone()
                                    }
                                }, self::JavaScriptCallInstruction::ArrayAllocate(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14788) => {
                                        source_ml_source2_data_property_value14788
                                    }
                                }, match prototype.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14789) => {
                                        source_ml_source2_data_property_value14789
                                    }
                                }, 0f64));
                                {
                                    let index = self::allocate_java_script_call_source_register(created.clone());
                                    {
                                        let initial = self::append_java_script_call_source_instruction(match index.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14790, _) => {
                                                let source_ml_source2_data_property_value14790 = *source_ml_source2_data_property_value14790;
                                                source_ml_source2_data_property_value14790.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Constant(match index.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14791) => {
                                                source_ml_source2_data_property_value14791
                                            }
                                        }, Box::new(self::JavaScriptSemanticValue::Number(0f64))));
                                        {
                                            let value = self::allocate_java_script_call_source_register(initial.clone());
                                            {
                                                let done = self::allocate_java_script_call_source_register(match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14792, _) => {
                                                        let source_ml_source2_data_property_value14792 = *source_ml_source2_data_property_value14792;
                                                        source_ml_source2_data_property_value14792.clone()
                                                    }
                                                });
                                                {
                                                    let key = self::allocate_java_script_call_source_register(match done.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14793, _) => {
                                                            let source_ml_source2_data_property_value14793 = *source_ml_source2_data_property_value14793;
                                                            source_ml_source2_data_property_value14793.clone()
                                                        }
                                                    });
                                                    {
                                                        let written = self::allocate_java_script_call_source_register(match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14794, _) => {
                                                                let source_ml_source2_data_property_value14794 = *source_ml_source2_data_property_value14794;
                                                                source_ml_source2_data_property_value14794.clone()
                                                            }
                                                        });
                                                        {
                                                            let start = { let ml_reference_array217 = match match written.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14795, _) => {
                                                                    let source_ml_source2_data_property_value14795 = *source_ml_source2_data_property_value14795;
                                                                    source_ml_source2_data_property_value14795.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14796, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value14796.clone()
                                                                }
                                                            }; let ml_reference_projection217 = ml_reference_array217.borrow().len() as f64; ml_reference_projection217 };
                                                            {
                                                                let stepped = self::append_java_script_call_source_instruction(match written.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14797, _) => {
                                                                        let source_ml_source2_data_property_value14797 = *source_ml_source2_data_property_value14797;
                                                                        source_ml_source2_data_property_value14797.clone()
                                                                    }
                                                                }, {
                                                                    let source_ml_source2_constructor_field14799 = match value.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14798) => {
                                                                            source_ml_source2_data_property_value14798
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field14801 = match done.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14800) => {
                                                                                source_ml_source2_data_property_value14800
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field14802 = 0f64;
                                                                            {
                                                                                let source_ml_source2_constructor_field14803 = 0f64;
                                                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field14799, source_ml_source2_constructor_field14801, source_ml_source2_constructor_field14802, source_ml_source2_constructor_field14803, slot, String::from("step"), realm.clone(), None::<(Option<bool>,)>)
                                                                            }
                                                                        }
                                                                    }
                                                                });
                                                                {
                                                                    let branch = { let ml_reference_array218 = match stepped.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14807, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value14807.clone()
                                                                        }
                                                                    }; let ml_reference_projection218 = ml_reference_array218.borrow().len() as f64; ml_reference_projection218 };
                                                                    {
                                                                        let test = self::append_java_script_call_source_instruction(stepped.clone(), self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14808) => {
                                                                                source_ml_source2_data_property_value14808
                                                                            }
                                                                        }, 0f64, (branch + 1f64)));
                                                                        {
                                                                            let keyed = self::append_java_script_call_source_instruction(test.clone(), self::JavaScriptCallInstruction::PropertyKey(match key.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14809) => {
                                                                                    source_ml_source2_data_property_value14809
                                                                                }
                                                                            }, match index.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14810) => {
                                                                                    source_ml_source2_data_property_value14810
                                                                                }
                                                                            }, realm.clone()));
                                                                            {
                                                                                let defined = self::append_java_script_call_source_instruction(keyed.clone(), self::JavaScriptCallInstruction::OrdinaryDataProperty(match written.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14811) => {
                                                                                        source_ml_source2_data_property_value14811
                                                                                    }
                                                                                }, match object.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14812) => {
                                                                                        source_ml_source2_data_property_value14812
                                                                                    }
                                                                                }, match key.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14813) => {
                                                                                        source_ml_source2_data_property_value14813
                                                                                    }
                                                                                }, match value.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14814) => {
                                                                                        source_ml_source2_data_property_value14814
                                                                                    }
                                                                                }, realm.clone()));
                                                                                {
                                                                                    let advanced = self::append_java_script_call_source_instruction(defined.clone(), self::JavaScriptCallInstruction::Primitive(match index.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14815) => {
                                                                                            source_ml_source2_data_property_value14815
                                                                                        }
                                                                                    }, String::from("increment numeric value"), std::rc::Rc::new(std::cell::RefCell::new(vec![match index.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14816) => {
                                                                                            source_ml_source2_data_property_value14816
                                                                                        }
                                                                                    }]))));
                                                                                    {
                                                                                        let loop_ = self::append_java_script_call_source_instruction(advanced.clone(), self::JavaScriptCallInstruction::Jump(start));
                                                                                        {
                                                                                            let end = { let ml_reference_array219 = match loop_.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14817, _, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value14817.clone()
                                                                                                }
                                                                                            }; let ml_reference_projection219 = ml_reference_array219.borrow().len() as f64; ml_reference_projection219 };
                                                                                            {
                                                                                                let source_ml_source2_constructor_field14819 = match object.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14818) => {
                                                                                                        source_ml_source2_data_property_value14818
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field14844 = {
                                                                                                        let source_ml_source2_nominal_spread14822 = loop_.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field14823 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14824, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value14824.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14825 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value14826, _, _, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value14826
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14827 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value14828, _, _, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value14828
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14829 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value14830, _, _, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value14830
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field14831 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value14832, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value14832
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field14833 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value14834, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value14834.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field14835 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value14836, _, _, _) => {
                                                                                                                                            source_ml_source2_data_property_value14836.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field14837 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value14838, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value14838.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field14839 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14840, _) => {
                                                                                                                                                    source_ml_source2_data_property_value14840.clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field14841 = match source_ml_source2_nominal_spread14822.clone() {
                                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14842) => {
                                                                                                                                                        source_ml_source2_data_property_value14842.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override14843 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values220 = { let ml_reference_value221 = (match loop_.clone() {
                                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14820, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                            source_ml_source2_data_property_value14820.clone()
                                                                                                                                                        }
                                                                                                                                                    }).borrow().clone(); ml_reference_value221 }; let ml_callback220 = { let branch = branch; let done = done.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, at: f64| -> self::JavaScriptCallInstruction { if (at == branch) {
                                                                                                                                                        self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14821) => {
                                                                                                                                                                source_ml_source2_data_property_value14821
                                                                                                                                                            }
                                                                                                                                                        }, end, (branch + 1f64))
                                                                                                                                                    } else {
                                                                                                                                                        instruction.clone()
                                                                                                                                                    } }) }; ml_values220.iter().cloned().enumerate().map(|(ml_index220, ml_value220)| ml_callback220(ml_value220, (ml_index220 as f64))).collect::<Vec<_>>() }));
                                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override14843.clone(), source_ml_source2_nominal_field14825, source_ml_source2_nominal_field14827, source_ml_source2_nominal_field14829, source_ml_source2_nominal_field14831, source_ml_source2_nominal_field14833.clone(), source_ml_source2_nominal_field14835.clone(), source_ml_source2_nominal_field14837.clone(), source_ml_source2_nominal_field14839.clone(), source_ml_source2_nominal_field14841.clone())
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
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field14844.clone()), source_ml_source2_constructor_field14819)
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
                }
            }
            self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceFinally(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceTry(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceBinding(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceLet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceRecur(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceAllocate(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceObject(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceDelete(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceHas(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
            self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
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
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ({ let ml_reference_array222 = units.clone(); let ml_reference_projection222 = ml_reference_array222.borrow().len() as f64; ml_reference_projection222 })) {
            {
                let unit = ({ let ml_reference_array223 = units.clone(); let ml_reference_position223 = at; let ml_reference_projection223 = self::ml_array::optional_at(&*ml_reference_array223.borrow(), self::ml_array::number_index(ml_reference_position223)); ml_reference_projection223 }).clone().unwrap_or_else(|| (-1f64));
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
pub enum MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result {
    MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop15(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut snapshot: bool, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values13: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index13: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result {
    loop {
        return if (source_ml_source1_temporary_index13 < ({ let ml_reference_array224 = source_ml_source1_temporary_values13.clone(); let ml_reference_projection224 = self::ml::Big::from_u128(ml_reference_array224.borrow().len() as u128); ml_reference_projection224 })) {
            {
                let argument = { let ml_reference_array225 = source_ml_source1_temporary_values13.clone(); let ml_reference_position225 = source_ml_source1_temporary_index13.clone(); let ml_reference_projection225 = self::ml_array::at(&*ml_reference_array225.borrow(), ml_reference_position225.to_index()); ml_reference_projection225 };
                {
                    let value = self::lower_java_script_call_source_operand(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64), snapshot);
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15047, _) => {
                                let source_ml_source2_data_property_value15047 = *source_ml_source2_data_property_value15047;
                                source_ml_source2_data_property_value15047.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value226 = arguments_.borrow().clone(); ml_reference_value226 }, vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15048) => {
                                    source_ml_source2_data_property_value15048
                                }
                            }])));
                            {
                                let source_ml_source1_temporary_index13_2 = source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index13) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index13_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result {
    MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}
