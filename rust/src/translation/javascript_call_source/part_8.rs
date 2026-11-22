// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_for_of;
use super::lower_java_script_call_source_iterator_loop;
use super::lower_java_script_call_source_iterator_remainder;
use super::ml;
use super::ml_array;
use super::read_java_script_call_primitive_arity_error;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop28;
use super::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop29;
use super::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop30;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_iterator_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::lower_java_script_call_source_for_of(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorStep(slot, realm, read_value) => {
            if ((!(self::test_java_script_call_code_coordinate(slot, 65536f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), "binding steps retain their actual iterator region, realm and value choice")
            } else {
                {
                    let value = self::allocate_java_script_call_source_register(state.clone());
                    {
                        let done = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5104, _) => {
                                let source_ml_source2_data_property_value5104 = *source_ml_source2_data_property_value5104;
                                source_ml_source2_data_property_value5104.clone()
                            }
                        });
                        {
                            let source_ml_source2_constructor_field5106 = match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5105) => {
                                    source_ml_source2_data_property_value5105
                                }
                            };
                            {
                                let source_ml_source2_constructor_field5125 = self::append_java_script_call_source_instruction(match done.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5107, _) => {
                                        let source_ml_source2_data_property_value5107 = *source_ml_source2_data_property_value5107;
                                        source_ml_source2_data_property_value5107.clone()
                                    }
                                }, {
                                    let source_ml_source1_temporary_constructor_field19 = match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5108) => {
                                            source_ml_source2_data_property_value5108
                                        }
                                    };
                                    {
                                        let source_ml_source1_temporary_constructor_field20 = match done.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5109) => {
                                                source_ml_source2_data_property_value5109
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_constructor_field21 = 0f64;
                                            {
                                                let source_ml_source1_temporary_constructor_field22 = 0f64;
                                                if read_value {
                                                    self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source1_temporary_constructor_field19, source_ml_source1_temporary_constructor_field20, source_ml_source1_temporary_constructor_field21, source_ml_source1_temporary_constructor_field22, slot, String::from("step"), realm.clone(), None::<(Option<bool>,)>)
                                                } else {
                                                    {
                                                        let source_ml_source2_constructor_field5124 = Some(false);
                                                        self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source1_temporary_constructor_field19, source_ml_source1_temporary_constructor_field20, source_ml_source1_temporary_constructor_field21, source_ml_source1_temporary_constructor_field22, slot, String::from("step"), realm.clone(), Some((source_ml_source2_constructor_field5124.clone(),)))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                });
                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field5125.clone()), source_ml_source2_constructor_field5106)
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::lower_java_script_call_source_iterator_remainder(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_for_of(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_iterator_loop(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorClose(slot_2, realm_2) => {
            if ((!(self::test_java_script_call_code_coordinate(slot_2, 65536f64)) || (realm_2 <= (self::ml::Big::from_i128(0)))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), "iterator closing retains its actual source slot and realm")
            } else {
                {
                    let value_2 = self::allocate_java_script_call_source_register(state.clone());
                    {
                        let done_2 = self::allocate_java_script_call_source_register(match value_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5126, _) => {
                                let source_ml_source2_data_property_value5126 = *source_ml_source2_data_property_value5126;
                                source_ml_source2_data_property_value5126.clone()
                            }
                        });
                        {
                            let source_ml_source2_constructor_field5128 = match value_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5127) => {
                                    source_ml_source2_data_property_value5127
                                }
                            };
                            {
                                let source_ml_source2_constructor_field5139 = self::append_java_script_call_source_instruction(match done_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5129, _) => {
                                        let source_ml_source2_data_property_value5129 = *source_ml_source2_data_property_value5129;
                                        source_ml_source2_data_property_value5129.clone()
                                    }
                                }, {
                                    let source_ml_source2_constructor_field5131 = match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5130) => {
                                            source_ml_source2_data_property_value5130
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field5133 = match done_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5132) => {
                                                source_ml_source2_data_property_value5132
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field5134 = 0f64;
                                            {
                                                let source_ml_source2_constructor_field5135 = 0f64;
                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field5131, source_ml_source2_constructor_field5133, source_ml_source2_constructor_field5134, source_ml_source2_constructor_field5135, slot_2, String::from("close"), realm_2.clone(), None::<(Option<bool>,)>)
                                            }
                                        }
                                    }
                                });
                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field5139.clone()), source_ml_source2_constructor_field5128)
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source iterator lowering requires its actual expression kind")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_comparison_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f5265, source_left, source_right) => {
            let source_left = *source_left;
            let source_right = *source_right;
            {
                let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let left_2 = left.clone();
                    {
                        let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings.clone(), match left_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5497, _) => {
                                let source_ml_source2_data_property_value5497 = *source_ml_source2_data_property_value5497;
                                source_ml_source2_data_property_value5497.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result = self::allocate_java_script_call_source_register(match right.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5498, _) => {
                                    let source_ml_source2_data_property_value5498 = *source_ml_source2_data_property_value5498;
                                    source_ml_source2_data_property_value5498.clone()
                                }
                            });
                            if !(self::ml_array::includes(&{ let ml_reference_value70 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("add"), String::from("subtract"), String::from("multiply"), String::from("divide"), String::from("remainder"), String::from("power"), String::from("lessThan"), String::from("equal")]))).borrow().clone(); ml_reference_value70 }, &source_ml_source2_f5265.clone(), 0f64, false, |a, b| a == b)) {
                                self::refuse_java_script_call_source(match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5682, _) => {
                                        let source_ml_source2_data_property_value5682 = *source_ml_source2_data_property_value5682;
                                        source_ml_source2_data_property_value5682.clone()
                                    }
                                }, "source Number lowering requires its exact registered operator")
                            } else {
                                {
                                    let source_ml_source2_nominal_field5688 = match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5689, _) => {
                                            let source_ml_source2_data_property_value5689 = *source_ml_source2_data_property_value5689;
                                            source_ml_source2_data_property_value5689.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field5690 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5691) => {
                                                source_ml_source2_data_property_value5691
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override5692 = self::append_java_script_call_source_instruction(match result.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5683, _) => {
                                                    let source_ml_source2_data_property_value5683 = *source_ml_source2_data_property_value5683;
                                                    source_ml_source2_data_property_value5683.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::NumberBinary(match result.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5684) => {
                                                    source_ml_source2_data_property_value5684
                                                }
                                            }, match left_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5685) => {
                                                    source_ml_source2_data_property_value5685
                                                }
                                            }, match right.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5686) => {
                                                    source_ml_source2_data_property_value5686
                                                }
                                            }, source_ml_source2_f5265.clone()));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override5692.clone()), source_ml_source2_nominal_field5690)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceSameValue(source_left_2, source_right_2) => {
            let source_left_2 = *source_left_2;
            let source_right_2 = *source_right_2;
            {
                let left_3 = self::lower_java_script_call_source_expression(source_left_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let left_4 = left_3.clone();
                    {
                        let right_2 = self::lower_java_script_call_source_expression(source_right_2.clone(), bindings.clone(), match left_4.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5877, _) => {
                                let source_ml_source2_data_property_value5877 = *source_ml_source2_data_property_value5877;
                                source_ml_source2_data_property_value5877.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result_2 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5878, _) => {
                                    let source_ml_source2_data_property_value5878 = *source_ml_source2_data_property_value5878;
                                    source_ml_source2_data_property_value5878.clone()
                                }
                            });
                            {
                                let source_ml_source2_nominal_field6445 = match result_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value6446, _) => {
                                        let source_ml_source2_data_property_value6446 = *source_ml_source2_data_property_value6446;
                                        source_ml_source2_data_property_value6446.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field6447 = match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value6448) => {
                                            source_ml_source2_data_property_value6448
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override6449 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value6440, _) => {
                                                let source_ml_source2_data_property_value6440 = *source_ml_source2_data_property_value6440;
                                                source_ml_source2_data_property_value6440.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::SameValue(match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value6441) => {
                                                source_ml_source2_data_property_value6441
                                            }
                                        }, match left_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value6442) => {
                                                source_ml_source2_data_property_value6442
                                            }
                                        }, match right_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value6443) => {
                                                source_ml_source2_data_property_value6443
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override6449.clone()), source_ml_source2_nominal_field6447)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_left_3, source_right_3) => {
            let source_left_3 = *source_left_3;
            let source_right_3 = *source_right_3;
            {
                let left_5 = self::lower_java_script_call_source_expression(source_left_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let left_6 = left_5.clone();
                    {
                        let right_3 = self::lower_java_script_call_source_expression(source_right_3.clone(), bindings.clone(), match left_6.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value6634, _) => {
                                let source_ml_source2_data_property_value6634 = *source_ml_source2_data_property_value6634;
                                source_ml_source2_data_property_value6634.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result_3 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value6635, _) => {
                                    let source_ml_source2_data_property_value6635 = *source_ml_source2_data_property_value6635;
                                    source_ml_source2_data_property_value6635.clone()
                                }
                            });
                            {
                                let source_ml_source2_nominal_field7192 = match result_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7193, _) => {
                                        let source_ml_source2_data_property_value7193 = *source_ml_source2_data_property_value7193;
                                        source_ml_source2_data_property_value7193.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field7194 = match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7195) => {
                                            source_ml_source2_data_property_value7195
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override7196 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7187, _) => {
                                                let source_ml_source2_data_property_value7187 = *source_ml_source2_data_property_value7187;
                                                source_ml_source2_data_property_value7187.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::StrictEqual(match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7188) => {
                                                source_ml_source2_data_property_value7188
                                            }
                                        }, match left_6.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7189) => {
                                                source_ml_source2_data_property_value7189
                                            }
                                        }, match right_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7190) => {
                                                source_ml_source2_data_property_value7190
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override7196.clone()), source_ml_source2_nominal_field7194)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(source_left_4, source_right_4) => {
            let source_left_4 = *source_left_4;
            let source_right_4 = *source_right_4;
            {
                let left_7 = self::lower_java_script_call_source_expression(source_left_4.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let left_9 = {
                        let saved = self::allocate_java_script_call_source_register(match left_7.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7391, _) => {
                                let source_ml_source2_data_property_value7391 = *source_ml_source2_data_property_value7391;
                                source_ml_source2_data_property_value7391.clone()
                            }
                        });
                        {
                            let left_8 = {
                                let source_ml_source2_nominal_field7396 = match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7397, _) => {
                                        let source_ml_source2_data_property_value7397 = *source_ml_source2_data_property_value7397;
                                        source_ml_source2_data_property_value7397.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field7398 = match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7399) => {
                                            source_ml_source2_data_property_value7399
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override7400 = self::append_java_script_call_source_instruction(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7392, _) => {
                                                let source_ml_source2_data_property_value7392 = *source_ml_source2_data_property_value7392;
                                                source_ml_source2_data_property_value7392.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7393) => {
                                                source_ml_source2_data_property_value7393
                                            }
                                        }, match left_7.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7394) => {
                                                source_ml_source2_data_property_value7394
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override7400.clone()), source_ml_source2_nominal_field7398)
                                    }
                                }
                            };
                            left_8.clone()
                        }
                    };
                    {
                        let right_4 = self::lower_java_script_call_source_expression(source_right_4.clone(), bindings.clone(), match left_9.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7401, _) => {
                                let source_ml_source2_data_property_value7401 = *source_ml_source2_data_property_value7401;
                                source_ml_source2_data_property_value7401.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result_4 = self::allocate_java_script_call_source_register(match right_4.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7402, _) => {
                                    let source_ml_source2_data_property_value7402 = *source_ml_source2_data_property_value7402;
                                    source_ml_source2_data_property_value7402.clone()
                                }
                            });
                            {
                                let source_ml_source2_nominal_field7959 = match result_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7960, _) => {
                                        let source_ml_source2_data_property_value7960 = *source_ml_source2_data_property_value7960;
                                        source_ml_source2_data_property_value7960.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field7961 = match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7962) => {
                                            source_ml_source2_data_property_value7962
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override7963 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7954, _) => {
                                                let source_ml_source2_data_property_value7954 = *source_ml_source2_data_property_value7954;
                                                source_ml_source2_data_property_value7954.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::StrictEqual(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7955) => {
                                                source_ml_source2_data_property_value7955
                                            }
                                        }, match left_9.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7956) => {
                                                source_ml_source2_data_property_value7956
                                            }
                                        }, match right_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7957) => {
                                                source_ml_source2_data_property_value7957
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override7963.clone()), source_ml_source2_nominal_field7961)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourcePrimitive(operation, arguments) => {
            {
                let error = self::read_java_script_call_primitive_arity_error(&(operation), { let ml_reference_array71 = arguments.clone(); let ml_reference_projection71 = ml_reference_array71.borrow().len() as f64; ml_reference_projection71 });
                if (error != "") {
                    self::refuse_java_script_call_source(state.clone(), &(error))
                } else {
                    {
                        let current = state.clone();
                        {
                            let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            {
                                let source_ml_source1_temporary_index26 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s7974 = self::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop28(bindings.clone(), depth, current.clone(), arguments_.clone(), arguments.clone(), source_ml_source1_temporary_index26.clone());
                                    match source_ml_source2_s7974.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Done(current_2, arguments__2) => {
                                            let current_2 = *current_2;
                                            {
                                                let result_5 = self::allocate_java_script_call_source_register(current_2.clone());
                                                {
                                                    let source_ml_source2_nominal_field7978 = match result_5.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7979, _) => {
                                                            let source_ml_source2_data_property_value7979 = *source_ml_source2_data_property_value7979;
                                                            source_ml_source2_data_property_value7979.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field7980 = match result_5.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7981) => {
                                                                source_ml_source2_data_property_value7981
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override7982 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7975, _) => {
                                                                    let source_ml_source2_data_property_value7975 = *source_ml_source2_data_property_value7975;
                                                                    source_ml_source2_data_property_value7975.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Primitive(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7976) => {
                                                                    source_ml_source2_data_property_value7976
                                                                }
                                                            }, operation.clone(), arguments__2.clone()));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override7982.clone()), source_ml_source2_nominal_field7980)
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
        self::JavaScriptCallSource::SourceRecur(arguments_2) => {
            if ((match state.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value7984, _, _, _, _, _) => {
                    source_ml_source2_data_property_value7984
                }
            }) != (0f64)) {
                self::refuse_java_script_call_source(state.clone(), "source recurrence through an active exception handler requires explicit unwinding")
            } else {
                if (({ let ml_reference_array72 = arguments_2.clone(); let ml_reference_projection72 = ml_reference_array72.borrow().len() as f64; ml_reference_projection72 }) != (match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value7986, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value7986
                    }
                })) {
                    self::refuse_java_script_call_source(state.clone(), "loop updates require every actual parameter operand")
                } else {
                    {
                        let current_3 = state.clone();
                        {
                            let arguments__3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            {
                                let source_ml_source1_temporary_index27 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s7987 = self::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop29(bindings.clone(), depth, current_3.clone(), arguments__3.clone(), arguments_2.clone(), source_ml_source1_temporary_index27.clone());
                                    match source_ml_source2_s7987.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Done(current_4, arguments__4) => {
                                            let current_4 = *current_4;
                                            {
                                                let index = 0f64;
                                                {
                                                    let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop30(current_4.clone(), arguments__4.clone(), index);
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(current_5.clone(), self::JavaScriptCallInstruction::Jump(0f64))), 0f64)
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
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source comparison lowering requires its actual expression kind")
        }
    }
}
