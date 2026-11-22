// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_property_query_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStringGet(source_input, source_key, source_prototype) => {
            let source_input = *source_input;
            let source_key = *source_key;
            let source_prototype = *source_prototype;
            {
                let input = self::lower_java_script_call_source_expression(source_input.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let held = self::allocate_java_script_call_source_register(match input.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9167, _) => {
                            let source_ml_source2_data_property_value9167 = *source_ml_source2_data_property_value9167;
                            source_ml_source2_data_property_value9167.clone()
                        }
                    });
                    {
                        let saved = self::append_java_script_call_source_instruction(match held.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9168, _) => {
                                let source_ml_source2_data_property_value9168 = *source_ml_source2_data_property_value9168;
                                source_ml_source2_data_property_value9168.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9169) => {
                                source_ml_source2_data_property_value9169
                            }
                        }, match input.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9170) => {
                                source_ml_source2_data_property_value9170
                            }
                        }));
                        {
                            let key = self::lower_java_script_call_source_expression(source_key.clone(), bindings.clone(), saved.clone(), (depth + 1f64));
                            {
                                let held_key = self::allocate_java_script_call_source_register(match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9171, _) => {
                                        let source_ml_source2_data_property_value9171 = *source_ml_source2_data_property_value9171;
                                        source_ml_source2_data_property_value9171.clone()
                                    }
                                });
                                {
                                    let saved_key = self::append_java_script_call_source_instruction(match held_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9172, _) => {
                                            let source_ml_source2_data_property_value9172 = *source_ml_source2_data_property_value9172;
                                            source_ml_source2_data_property_value9172.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Copy(match held_key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9173) => {
                                            source_ml_source2_data_property_value9173
                                        }
                                    }, match key.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9174) => {
                                            source_ml_source2_data_property_value9174
                                        }
                                    }));
                                    {
                                        let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), saved_key.clone(), (depth + 1f64));
                                        {
                                            let condition = self::allocate_java_script_call_source_register(match prototype.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9175, _) => {
                                                    let source_ml_source2_data_property_value9175 = *source_ml_source2_data_property_value9175;
                                                    source_ml_source2_data_property_value9175.clone()
                                                }
                                            });
                                            {
                                                let result = self::allocate_java_script_call_source_register(match condition.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9176, _) => {
                                                        let source_ml_source2_data_property_value9176 = *source_ml_source2_data_property_value9176;
                                                        source_ml_source2_data_property_value9176.clone()
                                                    }
                                                });
                                                {
                                                    let tested = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9177, _) => {
                                                            let source_ml_source2_data_property_value9177 = *source_ml_source2_data_property_value9177;
                                                            source_ml_source2_data_property_value9177.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Primitive(match condition.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9178) => {
                                                            source_ml_source2_data_property_value9178
                                                        }
                                                    }, String::from("test String own key"), std::rc::Rc::new(std::cell::RefCell::new(vec![match held.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9179) => {
                                                            source_ml_source2_data_property_value9179
                                                        }
                                                    }, match held_key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9180) => {
                                                            source_ml_source2_data_property_value9180
                                                        }
                                                    }]))));
                                                    {
                                                        let branch = { let ml_reference_array73 = match tested.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9181, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value9181.clone()
                                                            }
                                                        }; let ml_reference_projection73 = ml_reference_array73.borrow().len() as f64; ml_reference_projection73 };
                                                        {
                                                            let selected = self::append_java_script_call_source_instruction(tested.clone(), self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9182) => {
                                                                    source_ml_source2_data_property_value9182
                                                                }
                                                            }, (branch + 1f64), 0f64));
                                                            {
                                                                let own = self::append_java_script_call_source_instruction(selected.clone(), self::JavaScriptCallInstruction::Primitive(match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9183) => {
                                                                        source_ml_source2_data_property_value9183
                                                                    }
                                                                }, String::from("read String own key"), std::rc::Rc::new(std::cell::RefCell::new(vec![match held.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9184) => {
                                                                        source_ml_source2_data_property_value9184
                                                                    }
                                                                }, match held_key.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9185) => {
                                                                        source_ml_source2_data_property_value9185
                                                                    }
                                                                }]))));
                                                                {
                                                                    let jump = { let ml_reference_array74 = match own.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9186, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value9186.clone()
                                                                        }
                                                                    }; let ml_reference_projection74 = ml_reference_array74.borrow().len() as f64; ml_reference_projection74 };
                                                                    {
                                                                        let skipped = self::append_java_script_call_source_instruction(own.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                                        {
                                                                            let get = { let ml_reference_array75 = match skipped.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9187, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value9187.clone()
                                                                                }
                                                                            }; let ml_reference_projection75 = ml_reference_array75.borrow().len() as f64; ml_reference_projection75 };
                                                                            {
                                                                                let got = self::append_java_script_call_source_instruction(skipped.clone(), self::JavaScriptCallInstruction::OrdinaryGet(match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9188) => {
                                                                                        source_ml_source2_data_property_value9188
                                                                                    }
                                                                                }, match prototype.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9189) => {
                                                                                        source_ml_source2_data_property_value9189
                                                                                    }
                                                                                }, match held_key.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9190) => {
                                                                                        source_ml_source2_data_property_value9190
                                                                                    }
                                                                                }, match held.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9191) => {
                                                                                        source_ml_source2_data_property_value9191
                                                                                    }
                                                                                }));
                                                                                {
                                                                                    let end = { let ml_reference_array76 = match got.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9192, _, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value9192.clone()
                                                                                        }
                                                                                    }; let ml_reference_projection76 = ml_reference_array76.borrow().len() as f64; ml_reference_projection76 };
                                                                                    {
                                                                                        let source_ml_source2_constructor_field9194 = match result.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9193) => {
                                                                                                source_ml_source2_data_property_value9193
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_constructor_field9219 = {
                                                                                                let source_ml_source2_nominal_spread9197 = got.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9198 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9199, _, _, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value9199.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9200 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value9201, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value9201
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9202 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value9203, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value9203
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9204 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value9205, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value9205
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9206 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value9207, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value9207
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9208 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value9209, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value9209.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9210 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value9211, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value9211.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field9212 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value9213, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value9213.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field9214 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value9215, _) => {
                                                                                                                                            source_ml_source2_data_property_value9215.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field9216 = match source_ml_source2_nominal_spread9197.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9217) => {
                                                                                                                                                source_ml_source2_data_property_value9217.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override9218 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values77 = { let ml_reference_value78 = (match got.clone() {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value9195, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                    source_ml_source2_data_property_value9195.clone()
                                                                                                                                                }
                                                                                                                                            }).borrow().clone(); ml_reference_value78 }; let ml_callback77 = { let branch = branch; let condition = condition.clone(); let get = get; let jump = jump; let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, at: f64| -> self::JavaScriptCallInstruction { if (at == branch) {
                                                                                                                                                self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9196) => {
                                                                                                                                                        source_ml_source2_data_property_value9196
                                                                                                                                                    }
                                                                                                                                                }, (branch + 1f64), get)
                                                                                                                                            } else {
                                                                                                                                                if (at == jump) {
                                                                                                                                                    self::JavaScriptCallInstruction::Jump(end)
                                                                                                                                                } else {
                                                                                                                                                    instruction.clone()
                                                                                                                                                }
                                                                                                                                            } }) }; ml_values77.iter().cloned().enumerate().map(|(ml_index77, ml_value77)| ml_callback77(ml_value77, (ml_index77 as f64))).collect::<Vec<_>>() }));
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override9218.clone(), source_ml_source2_nominal_field9200, source_ml_source2_nominal_field9202, source_ml_source2_nominal_field9204, source_ml_source2_nominal_field9206, source_ml_source2_nominal_field9208.clone(), source_ml_source2_nominal_field9210.clone(), source_ml_source2_nominal_field9212.clone(), source_ml_source2_nominal_field9214.clone(), source_ml_source2_nominal_field9216.clone())
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
                                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field9219.clone()), source_ml_source2_constructor_field9194)
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
        self::JavaScriptCallSource::SourceGet(source_object, source_key_2, source_receiver) => {
            let source_object = *source_object;
            let source_key_2 = *source_key_2;
            {
                let object = self::lower_java_script_call_source_expression(source_object.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let key_2 = self::lower_java_script_call_source_expression(source_key_2.clone(), bindings.clone(), match object.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9220, _) => {
                            let source_ml_source2_data_property_value9220 = *source_ml_source2_data_property_value9220;
                            source_ml_source2_data_property_value9220.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let supplied_receiver = ((source_receiver.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSource>,))).0).clone();
                        {
                            let receiver = if match (supplied_receiver.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                {
                                    let source_ml_source2_constructor_field9222 = match object.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9221) => {
                                            source_ml_source2_data_property_value9221
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field9224 = match key_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9223, _) => {
                                                let source_ml_source2_data_property_value9223 = *source_ml_source2_data_property_value9223;
                                                source_ml_source2_data_property_value9223.clone()
                                            }
                                        };
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field9224.clone()), source_ml_source2_constructor_field9222)
                                    }
                                }
                            } else {
                                self::lower_java_script_call_source_expression(supplied_receiver.clone().expect("a proven present value"), bindings.clone(), match key_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9225, _) => {
                                        let source_ml_source2_data_property_value9225 = *source_ml_source2_data_property_value9225;
                                        source_ml_source2_data_property_value9225.clone()
                                    }
                                }, (depth + 1f64))
                            };
                            {
                                let result_2 = self::allocate_java_script_call_source_register(match receiver.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9226, _) => {
                                        let source_ml_source2_data_property_value9226 = *source_ml_source2_data_property_value9226;
                                        source_ml_source2_data_property_value9226.clone()
                                    }
                                });
                                {
                                    let source_ml_source2_nominal_field9233 = match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9234, _) => {
                                            let source_ml_source2_data_property_value9234 = *source_ml_source2_data_property_value9234;
                                            source_ml_source2_data_property_value9234.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field9235 = match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9236) => {
                                                source_ml_source2_data_property_value9236
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override9237 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9227, _) => {
                                                    let source_ml_source2_data_property_value9227 = *source_ml_source2_data_property_value9227;
                                                    source_ml_source2_data_property_value9227.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::OrdinaryGet(match result_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9228) => {
                                                    source_ml_source2_data_property_value9228
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9229) => {
                                                    source_ml_source2_data_property_value9229
                                                }
                                            }, match key_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9230) => {
                                                    source_ml_source2_data_property_value9230
                                                }
                                            }, match receiver.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9231) => {
                                                    source_ml_source2_data_property_value9231
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override9237.clone()), source_ml_source2_nominal_field9235)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDelete(source_object_2, source_key_3) => {
            let source_object_2 = *source_object_2;
            let source_key_3 = *source_key_3;
            {
                let raw_ = self::lower_java_script_call_source_expression(source_object_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_2 = self::allocate_java_script_call_source_register(match raw_.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9238, _) => {
                            let source_ml_source2_data_property_value9238 = *source_ml_source2_data_property_value9238;
                            source_ml_source2_data_property_value9238.clone()
                        }
                    });
                    {
                        let saved_2 = self::append_java_script_call_source_instruction(match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9239, _) => {
                                let source_ml_source2_data_property_value9239 = *source_ml_source2_data_property_value9239;
                                source_ml_source2_data_property_value9239.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match object_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9240) => {
                                source_ml_source2_data_property_value9240
                            }
                        }, match raw_.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9241) => {
                                source_ml_source2_data_property_value9241
                            }
                        }));
                        {
                            let key_3 = self::lower_java_script_call_source_expression(source_key_3.clone(), bindings.clone(), saved_2.clone(), (depth + 1f64));
                            {
                                let result_3 = self::allocate_java_script_call_source_register(match key_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9242, _) => {
                                        let source_ml_source2_data_property_value9242 = *source_ml_source2_data_property_value9242;
                                        source_ml_source2_data_property_value9242.clone()
                                    }
                                });
                                {
                                    let current = self::append_java_script_call_source_instruction(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9243, _) => {
                                            let source_ml_source2_data_property_value9243 = *source_ml_source2_data_property_value9243;
                                            source_ml_source2_data_property_value9243.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::OrdinaryDelete(match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9428) => {
                                            source_ml_source2_data_property_value9428
                                        }
                                    }, match object_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9429) => {
                                            source_ml_source2_data_property_value9429
                                        }
                                    }, match key_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9430) => {
                                            source_ml_source2_data_property_value9430
                                        }
                                    }));
                                    {
                                        let current_3 = {
                                            let checked = self::allocate_java_script_call_source_register(current.clone());
                                            {
                                                let current_2 = self::append_java_script_call_source_instruction(match checked.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9618, _) => {
                                                        let source_ml_source2_data_property_value9618 = *source_ml_source2_data_property_value9618;
                                                        source_ml_source2_data_property_value9618.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::Primitive(match checked.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9619) => {
                                                        source_ml_source2_data_property_value9619
                                                    }
                                                }, String::from("require delete success"), std::rc::Rc::new(std::cell::RefCell::new(vec![match result_3.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9620) => {
                                                        source_ml_source2_data_property_value9620
                                                    }
                                                }]))));
                                                current_2.clone()
                                            }
                                        };
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(current_3.clone()), match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9621) => {
                                                source_ml_source2_data_property_value9621
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
        self::JavaScriptCallSource::SourceHas(source_object_3, source_key_4) => {
            let source_object_3 = *source_object_3;
            let source_key_4 = *source_key_4;
            {
                let raw_2 = self::lower_java_script_call_source_expression(source_object_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let object_3 = self::allocate_java_script_call_source_register(match raw_2.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9622, _) => {
                            let source_ml_source2_data_property_value9622 = *source_ml_source2_data_property_value9622;
                            source_ml_source2_data_property_value9622.clone()
                        }
                    });
                    {
                        let saved_3 = self::append_java_script_call_source_instruction(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9623, _) => {
                                let source_ml_source2_data_property_value9623 = *source_ml_source2_data_property_value9623;
                                source_ml_source2_data_property_value9623.clone()
                            }
                        }, self::JavaScriptCallInstruction::Copy(match object_3.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9624) => {
                                source_ml_source2_data_property_value9624
                            }
                        }, match raw_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9625) => {
                                source_ml_source2_data_property_value9625
                            }
                        }));
                        {
                            let key_4 = self::lower_java_script_call_source_expression(source_key_4.clone(), bindings.clone(), saved_3.clone(), (depth + 1f64));
                            {
                                let result_4 = self::allocate_java_script_call_source_register(match key_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9626, _) => {
                                        let source_ml_source2_data_property_value9626 = *source_ml_source2_data_property_value9626;
                                        source_ml_source2_data_property_value9626.clone()
                                    }
                                });
                                {
                                    let current_4 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value9627, _) => {
                                            let source_ml_source2_data_property_value9627 = *source_ml_source2_data_property_value9627;
                                            source_ml_source2_data_property_value9627.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::OrdinaryHas(match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9815) => {
                                            source_ml_source2_data_property_value9815
                                        }
                                    }, match object_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9816) => {
                                            source_ml_source2_data_property_value9816
                                        }
                                    }, match key_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value9817) => {
                                            source_ml_source2_data_property_value9817
                                        }
                                    }));
                                    {
                                        let current_5 = current_4.clone();
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(current_5.clone()), match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value10002) => {
                                                source_ml_source2_data_property_value10002
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
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source propertyquery lowering requires its actual expression kind")
        }
    }
}
