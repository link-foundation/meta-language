// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_source_argument_spread(builder: self::JavaScriptCallSourceBuilder, slot: f64, iterator: f64, realm: self::ml::Big, value: f64, done: f64, handlers: f64) -> self::JavaScriptCallSourceBuilder {
    {
        let protect = { let ml_reference_array92 = match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11774, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value11774.clone()
            }
        }; let ml_reference_projection92 = ml_reference_array92.borrow().len() as f64; ml_reference_projection92 };
        {
            let current = self::append_java_script_call_source_instruction(builder.clone(), self::JavaScriptCallInstruction::PushFinally(0f64));
            {
                let start = { let ml_reference_array93 = match current.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11775, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value11775.clone()
                    }
                }; let ml_reference_projection93 = ml_reference_array93.borrow().len() as f64; ml_reference_projection93 };
                {
                    let current_2 = self::append_java_script_call_source_instruction(current.clone(), {
                        let source_ml_source2_constructor_field11778 = 0f64;
                        {
                            let source_ml_source2_constructor_field11779 = 0f64;
                            self::JavaScriptCallInstruction::IteratorProtocol(value, done, source_ml_source2_constructor_field11778, source_ml_source2_constructor_field11779, iterator, String::from("step"), realm.clone(), None::<(Option<bool>,)>)
                        }
                    });
                    {
                        let branch = { let ml_reference_array94 = match current_2.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11783, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11783.clone()
                            }
                        }; let ml_reference_projection94 = ml_reference_array94.borrow().len() as f64; ml_reference_projection94 };
                        {
                            let current_3 = self::append_java_script_call_source_instruction(current_2.clone(), self::JavaScriptCallInstruction::Branch(done, 0f64, (branch + 1f64)));
                            {
                                let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::ArgumentListAppend(slot, value));
                                {
                                    let current_5 = self::append_java_script_call_source_instruction(current_4.clone(), self::JavaScriptCallInstruction::Jump(start));
                                    {
                                        let leave = { let ml_reference_array95 = match current_5.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11784, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value11784.clone()
                                            }
                                        }; let ml_reference_projection95 = ml_reference_array95.borrow().len() as f64; ml_reference_projection95 };
                                        {
                                            let current_6 = self::append_java_script_call_source_instruction(current_5.clone(), self::JavaScriptCallInstruction::CompleteJump(0f64, handlers));
                                            {
                                                let cleanup = { let ml_reference_array96 = match current_6.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11785, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value11785.clone()
                                                    }
                                                }; let ml_reference_projection96 = ml_reference_array96.borrow().len() as f64; ml_reference_projection96 };
                                                {
                                                    let current_7 = self::append_java_script_call_source_instruction(current_6.clone(), {
                                                        let source_ml_source2_constructor_field11788 = 0f64;
                                                        {
                                                            let source_ml_source2_constructor_field11789 = 0f64;
                                                            self::JavaScriptCallInstruction::IteratorProtocol(value, done, source_ml_source2_constructor_field11788, source_ml_source2_constructor_field11789, iterator, String::from("close"), realm.clone(), None::<(Option<bool>,)>)
                                                        }
                                                    });
                                                    {
                                                        let current_8 = self::append_java_script_call_source_instruction(current_7.clone(), self::JavaScriptCallInstruction::FinishFinally);
                                                        {
                                                            let end = { let ml_reference_array97 = match current_8.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11793, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value11793.clone()
                                                                }
                                                            }; let ml_reference_projection97 = ml_reference_array97.borrow().len() as f64; ml_reference_projection97 };
                                                            {
                                                                let source_ml_source2_nominal_spread11795 = current_8.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field11796 = match source_ml_source2_nominal_spread11795.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11797, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value11797.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field11798 = match source_ml_source2_nominal_spread11795.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value11799, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value11799
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field11800 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value11801, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value11801
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field11802 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value11803, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value11803
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field11804 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11805, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value11805
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11806 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value11807, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value11807.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11808 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value11809, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value11809.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11810 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value11811, _, _) => {
                                                                                                        source_ml_source2_data_property_value11811.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11812 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value11813, _) => {
                                                                                                            source_ml_source2_data_property_value11813.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11814 = match source_ml_source2_nominal_spread11795.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11815) => {
                                                                                                                source_ml_source2_data_property_value11815.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override11816 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values98 = { let ml_reference_value99 = (match current_8.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11794, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value11794.clone()
                                                                                                                }
                                                                                                            }).borrow().clone(); ml_reference_value99 }; let ml_callback98 = { let protect = protect; let cleanup = cleanup; let branch = branch; let done = done; let leave = leave; let end = end; let handlers = handlers; std::rc::Rc::new(move |row: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == protect) {
                                                                                                                self::JavaScriptCallInstruction::PushFinally(cleanup)
                                                                                                            } else {
                                                                                                                if (index == branch) {
                                                                                                                    self::JavaScriptCallInstruction::Branch(done, leave, (branch + 1f64))
                                                                                                                } else {
                                                                                                                    if (index == leave) {
                                                                                                                        self::JavaScriptCallInstruction::CompleteJump(end, handlers)
                                                                                                                    } else {
                                                                                                                        row.clone()
                                                                                                                    }
                                                                                                                }
                                                                                                            } }) }; ml_values98.iter().cloned().enumerate().map(|(ml_index98, ml_value98)| ml_callback98(ml_value98, (ml_index98 as f64))).collect::<Vec<_>>() }));
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override11816.clone(), source_ml_source2_nominal_field11798, source_ml_source2_nominal_field11800, source_ml_source2_nominal_field11802, source_ml_source2_nominal_field11804, source_ml_source2_nominal_field11806.clone(), source_ml_source2_nominal_field11808.clone(), source_ml_source2_nominal_field11810.clone(), source_ml_source2_nominal_field11812.clone(), source_ml_source2_nominal_field11814.clone())
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_finally(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceFinally(source_body, source_finalizer) => {
            let source_body = *source_body;
            let source_finalizer = *source_finalizer;
            if ((match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12000, _, _, _, _, _) => {
                    source_ml_source2_data_property_value12000
                }
            }) >= (1024f64)) {
                self::refuse_java_script_call_source(builder.clone(), "finally exceeds its actual shared handler budget")
            } else {
                {
                    let result = self::allocate_java_script_call_source_register(builder.clone());
                    {
                        let position = { let ml_reference_array100 = match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12001, _) => {
                                let source_ml_source2_data_property_value12001 = *source_ml_source2_data_property_value12001;
                                source_ml_source2_data_property_value12001.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12002, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12002.clone()
                            }
                        }; let ml_reference_projection100 = ml_reference_array100.borrow().len() as f64; ml_reference_projection100 };
                        {
                            let protected_builder = self::append_java_script_call_source_instruction({
                                let source_ml_source2_nominal_spread12005 = match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12003, _) => {
                                        let source_ml_source2_data_property_value12003 = *source_ml_source2_data_property_value12003;
                                        source_ml_source2_data_property_value12003.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field12006 = match source_ml_source2_nominal_spread12005.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12007, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value12007.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field12008 = match source_ml_source2_nominal_spread12005.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12009, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value12009
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12010 = match source_ml_source2_nominal_spread12005.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12011, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12011
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12012 = match source_ml_source2_nominal_spread12005.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12013, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12013
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12014 = match source_ml_source2_nominal_spread12005.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12015, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12015
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12016 = match source_ml_source2_nominal_spread12005.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12017, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12017.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12018 = match source_ml_source2_nominal_spread12005.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12019, _, _, _) => {
                                                                    source_ml_source2_data_property_value12019.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12020 = match source_ml_source2_nominal_spread12005.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12021, _, _) => {
                                                                        source_ml_source2_data_property_value12021.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12022 = match source_ml_source2_nominal_spread12005.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12023, _) => {
                                                                            source_ml_source2_data_property_value12023.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12024 = match source_ml_source2_nominal_spread12005.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12025) => {
                                                                                source_ml_source2_data_property_value12025.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override12026 = (match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12004, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12004
                                                                                }
                                                                            } + 1f64);
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12006.clone(), source_ml_source2_nominal_field12008, source_ml_source2_nominal_field12010, source_ml_source2_nominal_field12012, source_ml_source2_nominal_override12026, source_ml_source2_nominal_field12016.clone(), source_ml_source2_nominal_field12018.clone(), source_ml_source2_nominal_field12020.clone(), source_ml_source2_nominal_field12022.clone(), source_ml_source2_nominal_field12024.clone())
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
                            }, self::JavaScriptCallInstruction::PushFinally(0f64));
                            {
                                let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                {
                                    let copied = self::append_java_script_call_source_instruction(match body.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12027, _) => {
                                            let source_ml_source2_data_property_value12027 = *source_ml_source2_data_property_value12027;
                                            source_ml_source2_data_property_value12027.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12028) => {
                                            source_ml_source2_data_property_value12028
                                        }
                                    }, match body.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12029) => {
                                            source_ml_source2_data_property_value12029
                                        }
                                    }));
                                    {
                                        let jump = { let ml_reference_array101 = match copied.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12030, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value12030.clone()
                                            }
                                        }; let ml_reference_projection101 = ml_reference_array101.borrow().len() as f64; ml_reference_projection101 };
                                        {
                                            let normal = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::CompleteJump(0f64, match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12031, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12031
                                                }
                                            }));
                                            {
                                                let entry = { let ml_reference_array102 = match normal.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12032, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12032.clone()
                                                    }
                                                }; let ml_reference_projection102 = ml_reference_array102.borrow().len() as f64; ml_reference_projection102 };
                                                {
                                                    let finalizer = self::lower_java_script_call_source_expression(source_finalizer.clone(), bindings.clone(), normal.clone(), (depth + 1f64));
                                                    {
                                                        let finished = self::append_java_script_call_source_instruction(match finalizer.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12033, _) => {
                                                                let source_ml_source2_data_property_value12033 = *source_ml_source2_data_property_value12033;
                                                                source_ml_source2_data_property_value12033.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::FinishFinally);
                                                        {
                                                            let end = { let ml_reference_array103 = match finished.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12034, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12034.clone()
                                                                }
                                                            }; let ml_reference_projection103 = ml_reference_array103.borrow().len() as f64; ml_reference_projection103 };
                                                            {
                                                                let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values104 = { let ml_reference_value105 = (match finished.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12035, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12035.clone()
                                                                    }
                                                                }).borrow().clone(); ml_reference_value105 }; let ml_callback104 = { let position = position; let entry = entry; let jump = jump; let end = end; let builder = builder.clone(); std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                    self::JavaScriptCallInstruction::PushFinally(entry)
                                                                } else {
                                                                    if (index == jump) {
                                                                        self::JavaScriptCallInstruction::CompleteJump(end, match builder.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12036, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12036
                                                                            }
                                                                        })
                                                                    } else {
                                                                        instruction.clone()
                                                                    }
                                                                } }) }; ml_values104.iter().cloned().enumerate().map(|(ml_index104, ml_value104)| ml_callback104(ml_value104, (ml_index104 as f64))).collect::<Vec<_>>() }));
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                                    let source_ml_source2_nominal_field12039 = match finished.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12040, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12040.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12041 = match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12042, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12042
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12043 = match finished.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12044, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12044
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12045 = match finished.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12046, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12046
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12047 = match finished.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12048, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12048
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12049 = match finished.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12050, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12050.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12051 = match finished.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12052, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12052.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12053 = match finished.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12054, _, _) => {
                                                                                                        source_ml_source2_data_property_value12054.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12055 = match finished.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12056, _) => {
                                                                                                            source_ml_source2_data_property_value12056.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12057 = match finished.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12058) => {
                                                                                                                source_ml_source2_data_property_value12058.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override12060 = match builder.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12037, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12037
                                                                                                                }
                                                                                                            };
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field12041, source_ml_source2_nominal_field12043, source_ml_source2_nominal_field12045, source_ml_source2_nominal_override12060, source_ml_source2_nominal_field12049.clone(), source_ml_source2_nominal_field12051.clone(), source_ml_source2_nominal_field12053.clone(), source_ml_source2_nominal_field12055.clone(), source_ml_source2_nominal_field12057.clone())
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
                                                                }), match result.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12061) => {
                                                                        source_ml_source2_data_property_value12061
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
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), "finally lowering retains its actual protected body and finalizer")
        }
    }
}
