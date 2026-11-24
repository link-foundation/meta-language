// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_binding_read_expression_loop18;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_binding_read_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBinding(identity) => {
            if !(self::test_java_script_call_code_coordinate(identity, 65536f64)) {
                self::refuse_java_script_call_source(state.clone(), "source binding identities require exact nonnegative coordinates")
            } else {
                {
                    let index = ({ let ml_reference_array53 = bindings.clone(); let ml_reference_projection53 = ml_reference_array53.borrow().len() as f64; ml_reference_projection53 } - 1f64);
                    {
                        let source_ml_source2_s4391 = self::source_ml_source1_temporary_lower_java_script_call_source_binding_read_expression_loop18(bindings.clone(), state.clone(), identity, index);
                        match source_ml_source2_s4391.clone() {
                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Done => {
                                self::refuse_java_script_call_source(state.clone(), "a source binding reference has no actual lexical owner")
                            }
                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(source_ml_source1_temporary_result18) => {
                                let source_ml_source1_temporary_result18 = *source_ml_source1_temporary_result18;
                                source_ml_source1_temporary_result18.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceLet(source_identity, source_value, source_body) => {
            let source_value = *source_value;
            let source_body = *source_body;
            if (!(self::test_java_script_call_code_coordinate(source_identity, 65536f64)) || { let ml_values54 = { let ml_reference_value55 = bindings.borrow().clone(); ml_reference_value55 }; let ml_callback54 = { let source_identity = source_identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4393, _, _, _) => {
                    source_ml_source2_data_property_value4393
                }
            }) == source_identity) }) }; ml_values54.iter().cloned().enumerate().any(|(ml_index54, ml_value54)| ml_callback54(ml_value54)) }) {
                self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
            } else {
                {
                    let value = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    self::lower_java_script_call_source_expression(source_body.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value4582 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                            let source_ml_source2_constructor_field4579 = match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4578) => {
                                    source_ml_source2_data_property_value4578
                                }
                            };
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_identity, source_ml_source2_constructor_field4579, false, None::<(Option<bool>,)>)
                        }]));
                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value56 = bindings.borrow().clone(); ml_reference_value56 }), { let ml_reference_value57 = source_ml_source2_concat_value4582.borrow().clone(); ml_reference_value57 })
                    })), match value.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4583, _) => {
                            let source_ml_source2_data_property_value4583 = *source_ml_source2_data_property_value4583;
                            source_ml_source2_data_property_value4583.clone()
                        }
                    }, (depth + 1f64))
                }
            }
        }
        self::JavaScriptCallSource::SourceLocalLet(source_identity_2, source_value_2, source_body_2, source_ml_source2_f4328) => {
            let source_value_2 = *source_value_2;
            let source_body_2 = *source_body_2;
            if (!(self::test_java_script_call_code_coordinate(source_identity_2, 65536f64)) || { let ml_values58 = { let ml_reference_value59 = bindings.borrow().clone(); ml_reference_value59 }; let ml_callback58 = { let source_identity_2 = source_identity_2; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4585, _, _, _) => {
                    source_ml_source2_data_property_value4585
                }
            }) == source_identity_2) }) }; ml_values58.iter().cloned().enumerate().any(|(ml_index58, ml_value58)| ml_callback58(ml_value58)) }) {
                self::refuse_java_script_call_source(state.clone(), "a source binding owner must be unique in its lexical scope")
            } else {
                {
                    let value_2 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let local = self::allocate_java_script_call_source_register(match value_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4769, _) => {
                                let source_ml_source2_data_property_value4769 = *source_ml_source2_data_property_value4769;
                                source_ml_source2_data_property_value4769.clone()
                            }
                        });
                        {
                            let copied = self::append_java_script_call_source_instruction(match local.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4770, _) => {
                                    let source_ml_source2_data_property_value4770 = *source_ml_source2_data_property_value4770;
                                    source_ml_source2_data_property_value4770.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match local.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4771) => {
                                    source_ml_source2_data_property_value4771
                                }
                            }, match value_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4772) => {
                                    source_ml_source2_data_property_value4772
                                }
                            }));
                            self::lower_java_script_call_source_expression(source_body_2.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_concat_value4778 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                    let source_ml_source2_constructor_field4775 = match local.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4774) => {
                                            source_ml_source2_data_property_value4774
                                        }
                                    };
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_identity_2, source_ml_source2_constructor_field4775, source_ml_source2_f4328, None::<(Option<bool>,)>)
                                }]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value60 = bindings.borrow().clone(); ml_reference_value60 }), { let ml_reference_value61 = source_ml_source2_concat_value4778.borrow().clone(); ml_reference_value61 })
                            })), copied.clone(), (depth + 1f64))
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceArgumentRest(from, source_prototype, realm) => {
            let source_prototype = *source_prototype;
            if ((!(self::test_java_script_call_code_coordinate(from, 1048576f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), "argument rest retains its actual bounded formal start and realm")
            } else {
                {
                    let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let result = self::allocate_java_script_call_source_register(match prototype.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4779, _) => {
                                let source_ml_source2_data_property_value4779 = *source_ml_source2_data_property_value4779;
                                source_ml_source2_data_property_value4779.clone()
                            }
                        });
                        {
                            let source_ml_source2_nominal_field4784 = match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4785, _) => {
                                    let source_ml_source2_data_property_value4785 = *source_ml_source2_data_property_value4785;
                                    source_ml_source2_data_property_value4785.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field4786 = match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4787) => {
                                        source_ml_source2_data_property_value4787
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_override4788 = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4780, _) => {
                                            let source_ml_source2_data_property_value4780 = *source_ml_source2_data_property_value4780;
                                            source_ml_source2_data_property_value4780.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::ArgumentRest(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4781) => {
                                            source_ml_source2_data_property_value4781
                                        }
                                    }, match prototype.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4782) => {
                                            source_ml_source2_data_property_value4782
                                        }
                                    }, from, realm.clone()));
                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override4788.clone()), source_ml_source2_nominal_field4786)
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceParameterRest(identity_2, from_2, source_prototype_2, realm_2, body) => {
            let source_prototype_2 = *source_prototype_2;
            let body = *body;
            {
                let binding = { let ml_values62 = { let ml_reference_value63 = bindings.borrow().clone(); ml_reference_value63 }; let ml_callback62 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4790, _, _, _) => {
                        source_ml_source2_data_property_value4790
                    }
                }) == identity_2) }) }; ml_values62.iter().cloned().enumerate().find_map(|(ml_index62, ml_value62)| if ml_callback62(ml_value62.clone()) { Some(ml_value62) } else { None }) };
                if (((((({
                    let source_ml_source2_comparison_operand4792 = match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4791, _, _, _) => {
                            ((source_ml_source2_data_property_value4791.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand4792.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                } || match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((match binding.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4795, _, _) => {
                        source_ml_source2_data_property_value4795
                    }
                }) != ((from_2 + 1f64)))) || ((match binding.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4798, _, _) => {
                        source_ml_source2_data_property_value4798
                    }
                }) > (match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4799, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value4799
                    }
                }))) || !(self::test_java_script_call_code_coordinate(from_2, 1048577f64))) || (realm_2 <= (self::ml::Big::from_i128(0)))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::refuse_java_script_call_source(state.clone(), "rest parameters retain their actual formal coordinate and realm")
                } else {
                    {
                        let prototype_2 = self::lower_java_script_call_source_expression(source_prototype_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                        {
                            let copied_2 = self::append_java_script_call_source_instruction(match prototype_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4800, _) => {
                                    let source_ml_source2_data_property_value4800 = *source_ml_source2_data_property_value4800;
                                    source_ml_source2_data_property_value4800.clone()
                                }
                            }, self::JavaScriptCallInstruction::ArgumentRest(match binding.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4801, _, _) => {
                                    source_ml_source2_data_property_value4801
                                }
                            }, match prototype_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4802) => {
                                    source_ml_source2_data_property_value4802
                                }
                            }, from_2, realm_2.clone()));
                            self::lower_java_script_call_source_expression(body.clone(), bindings.clone(), copied_2.clone(), (depth + 1f64))
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceParameterDefault(identity_3, source_value_3, body_2) => {
            let source_value_3 = *source_value_3;
            let body_2 = *body_2;
            if {
                let source_ml_source2_comparison_operand4804 = match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4803, _, _, _) => {
                        ((source_ml_source2_data_property_value4803.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                !(match (source_ml_source2_comparison_operand4804.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
            } {
                self::refuse_java_script_call_source(state.clone(), "mutable source bindings require their snapshot compilation pass")
            } else {
                {
                    let binding_2 = { let ml_values64 = { let ml_reference_value65 = bindings.borrow().clone(); ml_reference_value65 }; let ml_callback64 = { let identity_3 = identity_3; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4807, _, _, _) => {
                            source_ml_source2_data_property_value4807
                        }
                    }) == identity_3) }) }; ml_values64.iter().cloned().enumerate().find_map(|(ml_index64, ml_value64)| if ml_callback64(ml_value64.clone()) { Some(ml_value64) } else { None }) };
                    if ((match (binding_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match binding_2.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4809, _, _) => {
                            source_ml_source2_data_property_value4809
                        }
                    }) < (1f64))) || ((match binding_2.clone().expect("a proven present value") {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4812, _, _) => {
                            source_ml_source2_data_property_value4812
                        }
                    }) > (match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4813, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value4813
                        }
                    }))) {
                        self::refuse_java_script_call_source(state.clone(), "a parameter default needs its actual initialized parameter coordinate")
                    } else {
                        {
                            let condition = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceStrictEqual(Box::new(self::JavaScriptCallSource::SourceBinding(identity_3)), Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined)))), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let branch = { let ml_reference_array66 = match match condition.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4814, _) => {
                                        let source_ml_source2_data_property_value4814 = *source_ml_source2_data_property_value4814;
                                        source_ml_source2_data_property_value4814.clone()
                                    }
                                } {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4815, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value4815.clone()
                                    }
                                }; let ml_reference_projection66 = ml_reference_array66.borrow().len() as f64; ml_reference_projection66 };
                                {
                                    let start = self::append_java_script_call_source_instruction(match condition.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4816, _) => {
                                            let source_ml_source2_data_property_value4816 = *source_ml_source2_data_property_value4816;
                                            source_ml_source2_data_property_value4816.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4817) => {
                                            source_ml_source2_data_property_value4817
                                        }
                                    }, 0f64, 0f64));
                                    {
                                        let value_3 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                        {
                                            let copied_3 = self::append_java_script_call_source_instruction(match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4818, _) => {
                                                    let source_ml_source2_data_property_value4818 = *source_ml_source2_data_property_value4818;
                                                    source_ml_source2_data_property_value4818.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Copy(match binding_2.clone().expect("a proven present value") {
                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4819, _, _) => {
                                                    source_ml_source2_data_property_value4819
                                                }
                                            }, match value_3.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4820) => {
                                                    source_ml_source2_data_property_value4820
                                                }
                                            }));
                                            {
                                                let end = { let ml_reference_array67 = match copied_3.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4821, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value4821.clone()
                                                    }
                                                }; let ml_reference_projection67 = ml_reference_array67.borrow().len() as f64; ml_reference_projection67 };
                                                {
                                                    let updated = {
                                                        let source_ml_source2_nominal_spread4824 = copied_3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field4825 = match source_ml_source2_nominal_spread4824.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4826, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value4826.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field4827 = match source_ml_source2_nominal_spread4824.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value4828, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value4828
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field4829 = match source_ml_source2_nominal_spread4824.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value4830, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value4830
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field4831 = match source_ml_source2_nominal_spread4824.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4832, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value4832
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field4833 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value4834, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value4834
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field4835 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value4836, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value4836.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field4837 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4838, _, _, _) => {
                                                                                            source_ml_source2_data_property_value4838.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4839 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4840, _, _) => {
                                                                                                source_ml_source2_data_property_value4840.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4841 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4842, _) => {
                                                                                                    source_ml_source2_data_property_value4842.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4843 = match source_ml_source2_nominal_spread4824.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4844) => {
                                                                                                        source_ml_source2_data_property_value4844.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override4845 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values68 = { let ml_reference_value69 = (match copied_3.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4822, _, _, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value4822.clone()
                                                                                                        }
                                                                                                    }).borrow().clone(); ml_reference_value69 }; let ml_callback68 = { let branch = branch; let condition = condition.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index_2: f64| -> self::JavaScriptCallInstruction { if (index_2 == branch) {
                                                                                                        self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4823) => {
                                                                                                                source_ml_source2_data_property_value4823
                                                                                                            }
                                                                                                        }, (branch + 1f64), end)
                                                                                                    } else {
                                                                                                        instruction.clone()
                                                                                                    } }) }; ml_values68.iter().cloned().enumerate().map(|(ml_index68, ml_value68)| ml_callback68(ml_value68, (ml_index68 as f64))).collect::<Vec<_>>() }));
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override4845.clone(), source_ml_source2_nominal_field4827, source_ml_source2_nominal_field4829, source_ml_source2_nominal_field4831, source_ml_source2_nominal_field4833, source_ml_source2_nominal_field4835.clone(), source_ml_source2_nominal_field4837.clone(), source_ml_source2_nominal_field4839.clone(), source_ml_source2_nominal_field4841.clone(), source_ml_source2_nominal_field4843.clone())
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
                                                        let continuation = if match (match copied_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4846, _, _) => {
                                                                ((source_ml_source2_data_property_value4846.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            updated.clone()
                                                        } else {
                                                            {
                                                                let source_ml_source2_nominal_field4850 = match updated.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4851, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value4851.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field4852 = match updated.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value4853, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value4853
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field4854 = match updated.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value4855, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value4855
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field4856 = match updated.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4857, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value4857
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field4858 = match updated.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value4859, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value4859
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field4860 = match updated.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value4861, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value4861.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4862 = match updated.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4863, _, _, _) => {
                                                                                                source_ml_source2_data_property_value4863.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4864 = match updated.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4865, _, _) => {
                                                                                                    source_ml_source2_data_property_value4865.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4866 = match updated.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4867, _) => {
                                                                                                        source_ml_source2_data_property_value4867.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field4868 = match updated.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4869) => {
                                                                                                            source_ml_source2_data_property_value4869.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override4870 = (match match condition.clone() {
                                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4847, _) => {
                                                                                                                let source_ml_source2_data_property_value4847 = *source_ml_source2_data_property_value4847;
                                                                                                                source_ml_source2_data_property_value4847.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4848, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value4848.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field4850.clone(), source_ml_source2_nominal_field4852, source_ml_source2_nominal_field4854, source_ml_source2_nominal_field4856, source_ml_source2_nominal_field4858, source_ml_source2_nominal_field4860.clone(), source_ml_source2_nominal_field4862.clone(), Some((Some(source_ml_source2_nominal_override4870.clone()),)), source_ml_source2_nominal_field4866.clone(), source_ml_source2_nominal_field4868.clone())
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
                                                        self::lower_java_script_call_source_expression(body_2.clone(), bindings.clone(), continuation.clone(), (depth + 1f64))
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
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingread lowering requires its actual expression kind")
        }
    }
}
