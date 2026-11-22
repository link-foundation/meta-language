// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceControlScope;
use super::JavaScriptCallSourceControlTransfer;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result;
use super::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml_array;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop58;
use super::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop59;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_branch(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
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
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12577, _) => {
                            let source_ml_source2_data_property_value12577 = *source_ml_source2_data_property_value12577;
                            source_ml_source2_data_property_value12577.clone()
                        }
                    });
                    {
                        let branch = { let ml_reference_array119 = match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12578, _) => {
                                let source_ml_source2_data_property_value12578 = *source_ml_source2_data_property_value12578;
                                source_ml_source2_data_property_value12578.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12579, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12579.clone()
                            }
                        }; let ml_reference_projection119 = ml_reference_array119.borrow().len() as f64; ml_reference_projection119 };
                        {
                            let start = self::append_java_script_call_source_instruction(match result.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12580, _) => {
                                    let source_ml_source2_data_property_value12580 = *source_ml_source2_data_property_value12580;
                                    source_ml_source2_data_property_value12580.clone()
                                }
                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12581) => {
                                    source_ml_source2_data_property_value12581
                                }
                            }, 0f64, 0f64));
                            {
                                let yes_position = { let ml_reference_array120 = match start.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12582, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12582.clone()
                                    }
                                }; let ml_reference_projection120 = ml_reference_array120.borrow().len() as f64; ml_reference_projection120 };
                                {
                                    let yes = self::lower_java_script_call_source_expression(source_yes.clone(), bindings.clone(), start.clone(), (depth + 1f64));
                                    {
                                        let copied_yes = self::append_java_script_call_source_instruction(match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12583, _) => {
                                                let source_ml_source2_data_property_value12583 = *source_ml_source2_data_property_value12583;
                                                source_ml_source2_data_property_value12583.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12584) => {
                                                source_ml_source2_data_property_value12584
                                            }
                                        }, match yes.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12585) => {
                                                source_ml_source2_data_property_value12585
                                            }
                                        }));
                                        {
                                            let jump = { let ml_reference_array121 = match copied_yes.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12586, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12586.clone()
                                                }
                                            }; let ml_reference_projection121 = ml_reference_array121.borrow().len() as f64; ml_reference_projection121 };
                                            {
                                                let alternative = self::append_java_script_call_source_instruction(copied_yes.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                {
                                                    let no_position = { let ml_reference_array122 = match alternative.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12587, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12587.clone()
                                                        }
                                                    }; let ml_reference_projection122 = ml_reference_array122.borrow().len() as f64; ml_reference_projection122 };
                                                    {
                                                        let no = self::lower_java_script_call_source_expression(source_no.clone(), bindings.clone(), if match (match alternative.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12588, _, _) => {
                                                                ((source_ml_source2_data_property_value12588.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            alternative.clone()
                                                        } else {
                                                            {
                                                                let source_ml_source2_nominal_field12592 = match alternative.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12593, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12593.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12594 = match alternative.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12595, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12595
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12596 = match alternative.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12597, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12597
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12598 = match alternative.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12599, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12599
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12600 = match alternative.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12601, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12601
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12602 = match alternative.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12603, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12603.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12604 = match alternative.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12605, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12605.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12606 = match alternative.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12607, _, _) => {
                                                                                                    source_ml_source2_data_property_value12607.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12608 = match alternative.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12609, _) => {
                                                                                                        source_ml_source2_data_property_value12609.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12610 = match alternative.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12611) => {
                                                                                                            source_ml_source2_data_property_value12611.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override12612 = (match match condition.clone() {
                                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12589, _) => {
                                                                                                                let source_ml_source2_data_property_value12589 = *source_ml_source2_data_property_value12589;
                                                                                                                source_ml_source2_data_property_value12589.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12590, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value12590.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12592.clone(), source_ml_source2_nominal_field12594, source_ml_source2_nominal_field12596, source_ml_source2_nominal_field12598, source_ml_source2_nominal_field12600, source_ml_source2_nominal_field12602.clone(), source_ml_source2_nominal_field12604.clone(), Some((Some(source_ml_source2_nominal_override12612.clone()),)), source_ml_source2_nominal_field12608.clone(), source_ml_source2_nominal_field12610.clone())
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
                                                        }, (depth + 1f64));
                                                        {
                                                            let copied_no = self::append_java_script_call_source_instruction(match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12613, _) => {
                                                                    let source_ml_source2_data_property_value12613 = *source_ml_source2_data_property_value12613;
                                                                    source_ml_source2_data_property_value12613.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12614) => {
                                                                    source_ml_source2_data_property_value12614
                                                                }
                                                            }, match no.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12615) => {
                                                                    source_ml_source2_data_property_value12615
                                                                }
                                                            }));
                                                            {
                                                                let end = { let ml_reference_array123 = match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12616, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12616.clone()
                                                                    }
                                                                }; let ml_reference_projection123 = ml_reference_array123.borrow().len() as f64; ml_reference_projection123 };
                                                                if ((match copied_no.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12618, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12618.clone()
                                                                    }
                                                                }) != "") {
                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(copied_no.clone()), 0f64)
                                                                } else {
                                                                    {
                                                                        let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values124 = { let ml_reference_value125 = (match copied_no.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12619, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12619.clone()
                                                                            }
                                                                        }).borrow().clone(); ml_reference_value125 }; let ml_callback124 = { let branch = branch; let condition = condition.clone(); let yes_position = yes_position; let no_position = no_position; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                            self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12620) => {
                                                                                    source_ml_source2_data_property_value12620
                                                                                }
                                                                            }, yes_position, no_position)
                                                                        } else {
                                                                            if (index == jump) {
                                                                                self::JavaScriptCallInstruction::Jump(end)
                                                                            } else {
                                                                                entry.clone()
                                                                            }
                                                                        } }) }; ml_values124.iter().cloned().enumerate().map(|(ml_index124, ml_value124)| ml_callback124(ml_value124, (ml_index124 as f64))).collect::<Vec<_>>() }));
                                                                        {
                                                                            let updated = {
                                                                                let source_ml_source2_nominal_field12622 = match copied_no.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12623, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12623.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12624 = match copied_no.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12625, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12625
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12626 = match copied_no.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12627, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12627
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12628 = match copied_no.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12629, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12629
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12630 = match copied_no.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12631, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12631
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12632 = match copied_no.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12633, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12633.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12634 = match copied_no.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12635, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12635.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12636 = match copied_no.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12637, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12637.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12638 = match copied_no.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12639, _) => {
                                                                                                                        source_ml_source2_data_property_value12639.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12640 = match copied_no.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12641) => {
                                                                                                                            source_ml_source2_data_property_value12641.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field12624, source_ml_source2_nominal_field12626, source_ml_source2_nominal_field12628, source_ml_source2_nominal_field12630, source_ml_source2_nominal_field12632.clone(), source_ml_source2_nominal_field12634.clone(), source_ml_source2_nominal_field12636.clone(), source_ml_source2_nominal_field12638.clone(), source_ml_source2_nominal_field12640.clone())
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
                                                                                let continuation = if (match (match match yes.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12643, _) => {
                                                                                        let source_ml_source2_data_property_value12643 = *source_ml_source2_data_property_value12643;
                                                                                        source_ml_source2_data_property_value12643.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12644, _, _) => {
                                                                                        ((source_ml_source2_data_property_value12644.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && match (match match no.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12645, _) => {
                                                                                        let source_ml_source2_data_property_value12645 = *source_ml_source2_data_property_value12645;
                                                                                        source_ml_source2_data_property_value12645.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12646, _, _) => {
                                                                                        ((source_ml_source2_data_property_value12646.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    updated.clone()
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_nominal_spread12653 = updated.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12654 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12655, _, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12655.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12656 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12657, _, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12657
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12658 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12659, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12659
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12660 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12661, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12661
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12662 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12663, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12663
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12664 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12665, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value12665.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12666 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12667, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value12667.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12668 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12669, _, _) => {
                                                                                                                                source_ml_source2_data_property_value12669.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12670 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12671, _) => {
                                                                                                                                    source_ml_source2_data_property_value12671.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12672 = match source_ml_source2_nominal_spread12653.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12673) => {
                                                                                                                                        source_ml_source2_data_property_value12673.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override12674 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values126 = { let ml_reference_value127 = ((match match yes.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12647, _) => {
                                                                                                                                            let source_ml_source2_data_property_value12647 = *source_ml_source2_data_property_value12647;
                                                                                                                                            source_ml_source2_data_property_value12647.clone()
                                                                                                                                        }
                                                                                                                                    } {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12648, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value12648.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value127 }; let ml_callback126 = { let no = no.clone(); std::rc::Rc::new(move |identity: f64| -> bool { self::ml_array::includes(&{ let ml_reference_value128 = ((match match no.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12651, _) => {
                                                                                                                                            let source_ml_source2_data_property_value12651 = *source_ml_source2_data_property_value12651;
                                                                                                                                            source_ml_source2_data_property_value12651.clone()
                                                                                                                                        }
                                                                                                                                    } {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12652, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value12652.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value128 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) }) }; ml_values126.iter().cloned().enumerate().filter_map(|(ml_index126, ml_value126)| if ml_callback126(ml_value126.clone()) { Some(ml_value126) } else { None }).collect::<Vec<_>>() }));
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12654.clone(), source_ml_source2_nominal_field12656, source_ml_source2_nominal_field12658, source_ml_source2_nominal_field12660, source_ml_source2_nominal_field12662, source_ml_source2_nominal_field12664.clone(), source_ml_source2_nominal_field12666.clone(), Some((Some(source_ml_source2_nominal_override12674.clone()),)), source_ml_source2_nominal_field12670.clone(), source_ml_source2_nominal_field12672.clone())
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
                                                                                };
                                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new(continuation.clone()), match result.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12675) => {
                                                                                        source_ml_source2_data_property_value12675
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
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "a source branch compiler requires its actual conditional")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptCallSourceCompilation {
    self::compile_java_script_call_source_with_captures(source.clone(), parameters.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source_with_captures(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptCallSourceCompilation {
    {
        let first = self::compile_java_script_call_source_binding_reads(source.clone(), parameters.clone(), captures.clone(), false);
        if ((match first.clone() {
            self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value12677) => {
                source_ml_source2_data_property_value12677.clone()
            }
        }) == "mutable source bindings require their snapshot compilation pass") {
            self::compile_java_script_call_source_binding_reads(source.clone(), parameters.clone(), captures.clone(), true)
        } else {
            first.clone()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_mutable_call_source_with_captures(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptCallSourceCompilation {
    self::compile_java_script_call_source_binding_reads(source.clone(), parameters.clone(), captures.clone(), true)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source_binding_reads(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, snapshot_mutable_bindings: bool) -> self::JavaScriptCallSourceCompilation {
    if ((({ let ml_reference_array129 = parameters.clone(); let ml_reference_projection129 = ml_reference_array129.borrow().len() as f64; ml_reference_projection129 } + { let ml_reference_array130 = captures.clone(); let ml_reference_projection130 = ml_reference_array130.borrow().len() as f64; ml_reference_projection130 })) >= (4096f64)) {
        self::JavaScriptCallSourceCompilation::SourceCompiled(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new())), 0f64, String::from("source parameters exceed their explicit register budget"))
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source2_s12678 = self::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop58(parameters.clone(), index);
                match source_ml_source2_s12678.clone() {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Done => {
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s12679 = self::source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop59(parameters.clone(), captures.clone(), index_2);
                                match source_ml_source2_s12679.clone() {
                                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Done => {
                                        {
                                            let bindings = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values131 = { let ml_reference_value132 = (std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value133 = parameters.borrow().clone(); ml_reference_value133 }), { let ml_reference_value134 = captures.borrow().clone(); ml_reference_value134 })))).borrow().clone(); ml_reference_value132 }; let ml_callback131 = { let parameters = parameters.clone(); std::rc::Rc::new(move |identity: f64, index_3: f64| -> self::JavaScriptCallSourceBinding { {
                                                let source_ml_source2_constructor_field12682 = identity;
                                                {
                                                    let source_ml_source2_constructor_field12683 = (index_3 + 1f64);
                                                    {
                                                        let source_ml_source2_constructor_field12684 = (index_3 < ({ let ml_reference_array135 = parameters.clone(); let ml_reference_projection135 = ml_reference_array135.borrow().len() as f64; ml_reference_projection135 }));
                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_constructor_field12682, source_ml_source2_constructor_field12683, source_ml_source2_constructor_field12684, None::<(Option<bool>,)>)
                                                    }
                                                }
                                            } }) }; ml_values131.iter().cloned().enumerate().map(|(ml_index131, ml_value131)| ml_callback131(ml_value131, (ml_index131 as f64))).collect::<Vec<_>>() }));
                                            {
                                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings.clone(), {
                                                    let source_ml_source1_temporary_constructor_field52 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
                                                    {
                                                        let source_ml_source1_temporary_constructor_field53 = (({ let ml_reference_array136 = parameters.clone(); let ml_reference_projection136 = ml_reference_array136.borrow().len() as f64; ml_reference_projection136 } + { let ml_reference_array137 = captures.clone(); let ml_reference_projection137 = ml_reference_array137.borrow().len() as f64; ml_reference_projection137 }) + 1f64);
                                                        {
                                                            let source_ml_source1_temporary_constructor_field54 = 0f64;
                                                            {
                                                                let source_ml_source1_temporary_constructor_field55 = { let ml_reference_array138 = parameters.clone(); let ml_reference_projection138 = ml_reference_array138.borrow().len() as f64; ml_reference_projection138 };
                                                                {
                                                                    let source_ml_source1_temporary_constructor_field56 = 0f64;
                                                                    if snapshot_mutable_bindings {
                                                                        {
                                                                            let source_ml_source2_constructor_field12691 = Some(true);
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_temporary_constructor_field52.clone(), source_ml_source1_temporary_constructor_field53, source_ml_source1_temporary_constructor_field54, source_ml_source1_temporary_constructor_field55, source_ml_source1_temporary_constructor_field56, String::from(""), Some((source_ml_source2_constructor_field12691.clone(),)), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,)>)
                                                                        }
                                                                    } else {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_temporary_constructor_field52.clone(), source_ml_source1_temporary_constructor_field53, source_ml_source1_temporary_constructor_field54, source_ml_source1_temporary_constructor_field55, source_ml_source1_temporary_constructor_field56, String::from(""), None::<(Option<bool>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,)>)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }, 0f64);
                                                {
                                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12698, _) => {
                                                            let source_ml_source2_data_property_value12698 = *source_ml_source2_data_property_value12698;
                                                            source_ml_source2_data_property_value12698.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12699) => {
                                                            source_ml_source2_data_property_value12699
                                                        }
                                                    }));
                                                    {
                                                        let builder_3 = if { let ml_value = { let ml_reference_array140 = (match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12700) => {
                                                                ((source_ml_source2_data_property_value12700.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,))).0).clone()
                                                            }
                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlTransfer>::new()))); let ml_reference_projection140 = ml_reference_array140.borrow().len() as f64; ml_reference_projection140 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                            {
                                                                let builder_2 = {
                                                                    let source_ml_source2_nominal_field12702 = match builder.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12703, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12703.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12704 = match builder.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12705, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12705
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12706 = match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12707, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12707
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12708 = match builder.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12709, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12709
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12710 = match builder.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12711, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12711
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12712 = match builder.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12713, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12713.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12714 = match builder.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12715, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12715.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12716 = match builder.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12717, _, _) => {
                                                                                                        source_ml_source2_data_property_value12717.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12718 = match builder.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12719, _) => {
                                                                                                            source_ml_source2_data_property_value12719.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12720 = match builder.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12721) => {
                                                                                                                source_ml_source2_data_property_value12721.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12702.clone(), source_ml_source2_nominal_field12704, source_ml_source2_nominal_field12706, source_ml_source2_nominal_field12708, source_ml_source2_nominal_field12710, String::from("completed source compilation cannot retain unresolved control transfers"), source_ml_source2_nominal_field12714.clone(), source_ml_source2_nominal_field12716.clone(), source_ml_source2_nominal_field12718.clone(), source_ml_source2_nominal_field12720.clone())
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
                                                                builder_2.clone()
                                                            }
                                                        } else {
                                                            builder.clone()
                                                        };
                                                        self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12724, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12724.clone()
                                                            }
                                                        }) == "") {
                                                            match builder_3.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12725, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12725.clone()
                                                                }
                                                            }
                                                        } else {
                                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()))
                                                        }, if ((match builder_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12727, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12727.clone()
                                                            }
                                                        }) == "") {
                                                            match builder_3.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12728, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12728
                                                                }
                                                            }
                                                        } else {
                                                            0f64
                                                        }, match builder_3.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12729, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12729.clone()
                                                            }
                                                        })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Return(source_ml_source1_temporary_result59) => {
                                        let source_ml_source1_temporary_result59 = *source_ml_source1_temporary_result59;
                                        source_ml_source1_temporary_result59.clone()
                                    }
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Return(source_ml_source1_temporary_result59_2) => {
                        let source_ml_source1_temporary_result59_2 = *source_ml_source1_temporary_result59_2;
                        source_ml_source1_temporary_result59_2.clone()
                    }
                }
            }
        }
    }
}
