// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_math;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop33;
use super::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop34;
use super::source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop41;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_array_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(source_prototype, parts, source_iterator_key, realm) => {
            let source_prototype = *source_prototype;
            let source_iterator_key = *source_iterator_key;
            if (((({ let ml_reference_array79 = parts.clone(); let ml_reference_projection79 = ml_reference_array79.borrow().len() as f64; ml_reference_projection79 }) > (4096f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), "Array accumulation retains its bounded parts and actual realm")
            } else {
                {
                    let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let object = self::allocate_java_script_call_source_register(match prototype.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10125, _) => {
                                let source_ml_source2_data_property_value10125 = *source_ml_source2_data_property_value10125;
                                source_ml_source2_data_property_value10125.clone()
                            }
                        });
                        {
                            let current = self::append_java_script_call_source_instruction(match object.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10126, _) => {
                                    let source_ml_source2_data_property_value10126 = *source_ml_source2_data_property_value10126;
                                    source_ml_source2_data_property_value10126.clone()
                                }
                            }, self::JavaScriptCallInstruction::ArrayAllocate(match object.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10127) => {
                                    source_ml_source2_data_property_value10127
                                }
                            }, match prototype.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10128) => {
                                    source_ml_source2_data_property_value10128
                                }
                            }, 0f64));
                            {
                                let iterator_key = self::lower_java_script_call_source_expression(source_iterator_key.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                {
                                    let saved_key = self::allocate_java_script_call_source_register(match iterator_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10129, _) => {
                                            let source_ml_source2_data_property_value10129 = *source_ml_source2_data_property_value10129;
                                            source_ml_source2_data_property_value10129.clone()
                                        }
                                    });
                                    {
                                        let keyed = self::append_java_script_call_source_instruction(match saved_key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10130, _) => {
                                                let source_ml_source2_data_property_value10130 = *source_ml_source2_data_property_value10130;
                                                source_ml_source2_data_property_value10130.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved_key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10131) => {
                                                source_ml_source2_data_property_value10131
                                            }
                                        }, match iterator_key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10132) => {
                                                source_ml_source2_data_property_value10132
                                            }
                                        }));
                                        {
                                            let index = self::allocate_java_script_call_source_register(keyed.clone());
                                            {
                                                let current_2 = self::append_java_script_call_source_instruction(match index.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10133, _) => {
                                                        let source_ml_source2_data_property_value10133 = *source_ml_source2_data_property_value10133;
                                                        source_ml_source2_data_property_value10133.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Constant(match index.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10134) => {
                                                        source_ml_source2_data_property_value10134
                                                    }
                                                }, Box::new(self::JavaScriptSemanticValue::Number(0f64))));
                                                {
                                                    let source_ml_source1_temporary_index31 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s10135 = self::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop33(bindings.clone(), depth, realm.clone(), object.clone(), current_2.clone(), saved_key.clone(), index.clone(), parts.clone(), source_ml_source1_temporary_index31.clone());
                                                        match source_ml_source2_s10135.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Done(current_3) => {
                                                                let current_3 = *current_3;
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(current_3.clone()), match object.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10136) => {
                                                                        source_ml_source2_data_property_value10136
                                                                    }
                                                                })
                                                            }
                                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Return(source_ml_source1_temporary_result33) => {
                                                                let source_ml_source1_temporary_result33 = *source_ml_source1_temporary_result33;
                                                                source_ml_source1_temporary_result33.clone()
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
        self::JavaScriptCallSource::SourceArray(source_prototype_2, length, elements, realm_2) => {
            let source_prototype_2 = *source_prototype_2;
            if ((((((!(self::ml_math::is_integer(length)) || (length < (0f64))) || (length > (4294967295f64))) || {
                let source_ml_source2_same_value_operand10138 = (-0f64);
                (((length == source_ml_source2_same_value_operand10138) && ((length != (0f64)) || (((1f64 / length)) == ((1f64 / source_ml_source2_same_value_operand10138))))) || ((length != length) && (source_ml_source2_same_value_operand10138 != source_ml_source2_same_value_operand10138)))
            }) || (({ let ml_reference_array80 = elements.clone(); let ml_reference_projection80 = ml_reference_array80.borrow().len() as f64; ml_reference_projection80 }) > (4096f64))) || (realm_2 <= (self::ml::Big::from_i128(0)))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), "source Array construction requires its actual sparse length, bounded elements and realm")
            } else {
                {
                    let prototype_2 = self::lower_java_script_call_source_expression(source_prototype_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                    {
                        let object_2 = self::allocate_java_script_call_source_register(match prototype_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10139, _) => {
                                let source_ml_source2_data_property_value10139 = *source_ml_source2_data_property_value10139;
                                source_ml_source2_data_property_value10139.clone()
                            }
                        });
                        {
                            let current_4 = self::append_java_script_call_source_instruction(match object_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10140, _) => {
                                    let source_ml_source2_data_property_value10140 = *source_ml_source2_data_property_value10140;
                                    source_ml_source2_data_property_value10140.clone()
                                }
                            }, self::JavaScriptCallInstruction::ArrayAllocate(match object_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10141) => {
                                    source_ml_source2_data_property_value10141
                                }
                            }, match prototype_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10142) => {
                                    source_ml_source2_data_property_value10142
                                }
                            }, length));
                            {
                                let previous = (-1f64);
                                {
                                    let source_ml_source1_temporary_index32 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s10143 = self::source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop34(bindings.clone(), depth, realm_2.clone(), length, object_2.clone(), current_4.clone(), previous, elements.clone(), source_ml_source1_temporary_index32.clone());
                                        match source_ml_source2_s10143.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Done(current_5) => {
                                                let current_5 = *current_5;
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(current_5.clone()), match object_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10144) => {
                                                        source_ml_source2_data_property_value10144
                                                    }
                                                })
                                            }
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Return(source_ml_source1_temporary_result34) => {
                                                let source_ml_source1_temporary_result34 = *source_ml_source1_temporary_result34;
                                                source_ml_source1_temporary_result34.clone()
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
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source array lowering requires its actual expression kind")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_object_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceObject(source_prototype, properties, realm) => {
            let source_prototype = *source_prototype;
            if ((realm <= (self::ml::Big::from_i128(0))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), "source object construction requires its actual unsigned64 realm identity")
            } else {
                if (({ let ml_reference_array81 = properties.clone(); let ml_reference_projection81 = ml_reference_array81.borrow().len() as f64; ml_reference_projection81 }) > (4096f64)) {
                    self::refuse_java_script_call_source(state.clone(), "source object properties exceed their explicit structural budget")
                } else {
                    {
                        let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                        {
                            let object = self::allocate_java_script_call_source_register(match prototype.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10326, _) => {
                                    let source_ml_source2_data_property_value10326 = *source_ml_source2_data_property_value10326;
                                    source_ml_source2_data_property_value10326.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value10327, _) => {
                                        let source_ml_source2_data_property_value10327 = *source_ml_source2_data_property_value10327;
                                        source_ml_source2_data_property_value10327.clone()
                                    }
                                }, self::JavaScriptCallInstruction::OrdinaryAllocate(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10328) => {
                                        source_ml_source2_data_property_value10328
                                    }
                                }, match prototype.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10329) => {
                                        source_ml_source2_data_property_value10329
                                    }
                                }));
                                {
                                    let source_ml_source1_temporary_index35 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s10330 = self::source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop41(bindings.clone(), depth, realm.clone(), object.clone(), current.clone(), properties.clone(), source_ml_source1_temporary_index35.clone());
                                        match source_ml_source2_s10330.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Done(current_2) => {
                                                let current_2 = *current_2;
                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(current_2.clone()), match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10331) => {
                                                        source_ml_source2_data_property_value10331
                                                    }
                                                })
                                            }
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Return(source_ml_source1_temporary_result41) => {
                                                let source_ml_source1_temporary_result41 = *source_ml_source1_temporary_result41;
                                                source_ml_source1_temporary_result41.clone()
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
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source object lowering requires its actual expression kind")
        }
    }
}
