// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_string;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_source_argument_spread(builder: self::JavaScriptCallSourceBuilder, slot: f64, iterator: f64, realm: self::ml::Big, value: f64, done: f64, handlers: f64) -> self::JavaScriptCallSourceBuilder {
    {
        let protect = { let ml_reference_array92 = match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11720, _, _, _, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value11720.clone()
            }
        }; let ml_reference_projection92 = ml_reference_array92.borrow().len() as f64; ml_reference_projection92 };
        {
            let current = self::append_java_script_call_source_instruction(builder.clone(), self::JavaScriptCallInstruction::PushFinally(0f64));
            {
                let start = { let ml_reference_array93 = match current.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11721, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value11721.clone()
                    }
                }; let ml_reference_projection93 = ml_reference_array93.borrow().len() as f64; ml_reference_projection93 };
                {
                    let current_2 = self::append_java_script_call_source_instruction(current.clone(), {
                        let source_ml_source2_constructor_field11724 = 0f64;
                        {
                            let source_ml_source2_constructor_field11725 = 0f64;
                            self::JavaScriptCallInstruction::IteratorProtocol(value, done, source_ml_source2_constructor_field11724, source_ml_source2_constructor_field11725, iterator, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), None::<(Option<bool>,)>)
                        }
                    });
                    {
                        let branch = { let ml_reference_array94 = match current_2.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11729, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11729.clone()
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
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11730, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value11730.clone()
                                            }
                                        }; let ml_reference_projection95 = ml_reference_array95.borrow().len() as f64; ml_reference_projection95 };
                                        {
                                            let current_6 = self::append_java_script_call_source_instruction(current_5.clone(), self::JavaScriptCallInstruction::CompleteJump(0f64, handlers));
                                            {
                                                let cleanup = { let ml_reference_array96 = match current_6.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11731, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value11731.clone()
                                                    }
                                                }; let ml_reference_projection96 = ml_reference_array96.borrow().len() as f64; ml_reference_projection96 };
                                                {
                                                    let current_7 = self::append_java_script_call_source_instruction(current_6.clone(), {
                                                        let source_ml_source2_constructor_field11734 = 0f64;
                                                        {
                                                            let source_ml_source2_constructor_field11735 = 0f64;
                                                            self::JavaScriptCallInstruction::IteratorProtocol(value, done, source_ml_source2_constructor_field11734, source_ml_source2_constructor_field11735, iterator, self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), realm.clone(), None::<(Option<bool>,)>)
                                                        }
                                                    });
                                                    {
                                                        let current_8 = self::append_java_script_call_source_instruction(current_7.clone(), self::JavaScriptCallInstruction::FinishFinally);
                                                        {
                                                            let end = { let ml_reference_array97 = match current_8.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11739, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value11739.clone()
                                                                }
                                                            }; let ml_reference_projection97 = ml_reference_array97.borrow().len() as f64; ml_reference_projection97 };
                                                            {
                                                                let source_ml_source2_nominal_spread11741 = current_8.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field11742 = match source_ml_source2_nominal_spread11741.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11743, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value11743.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field11744 = match source_ml_source2_nominal_spread11741.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value11745, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value11745
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field11746 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value11747, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value11747
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field11748 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value11749, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value11749
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field11750 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11751, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value11751
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11752 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value11753, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value11753.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11754 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value11755, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value11755.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11756 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value11757, _, _) => {
                                                                                                        source_ml_source2_data_property_value11757.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11758 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value11759, _) => {
                                                                                                            source_ml_source2_data_property_value11759.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11760 = match source_ml_source2_nominal_spread11741.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11761) => {
                                                                                                                source_ml_source2_data_property_value11761.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override11762 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values98 = { let ml_reference_value99 = (match current_8.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11740, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value11740.clone()
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
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override11762.clone(), source_ml_source2_nominal_field11744, source_ml_source2_nominal_field11746, source_ml_source2_nominal_field11748, source_ml_source2_nominal_field11750, source_ml_source2_nominal_field11752.clone(), source_ml_source2_nominal_field11754.clone(), source_ml_source2_nominal_field11756.clone(), source_ml_source2_nominal_field11758.clone(), source_ml_source2_nominal_field11760.clone())
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
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceFinally(source_body, source_finalizer) => {
            let source_body = *source_body;
            let source_finalizer = *source_finalizer;
            if ((match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11946, _, _, _, _, _) => {
                    source_ml_source2_data_property_value11946
                }
            }) >= (1024f64)) {
                self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 104, 97, 110, 100, 108, 101, 114, 32, 98, 117, 100, 103, 101, 116])))
            } else {
                {
                    let result = self::allocate_java_script_call_source_register(builder.clone());
                    {
                        let position = { let ml_reference_array100 = match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11947, _) => {
                                let source_ml_source2_data_property_value11947 = *source_ml_source2_data_property_value11947;
                                source_ml_source2_data_property_value11947.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11948, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value11948.clone()
                            }
                        }; let ml_reference_projection100 = ml_reference_array100.borrow().len() as f64; ml_reference_projection100 };
                        {
                            let protected_builder = self::append_java_script_call_source_instruction({
                                let source_ml_source2_nominal_spread11951 = match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11949, _) => {
                                        let source_ml_source2_data_property_value11949 = *source_ml_source2_data_property_value11949;
                                        source_ml_source2_data_property_value11949.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field11952 = match source_ml_source2_nominal_spread11951.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11953, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value11953.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field11954 = match source_ml_source2_nominal_spread11951.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value11955, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value11955
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field11956 = match source_ml_source2_nominal_spread11951.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value11957, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value11957
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field11958 = match source_ml_source2_nominal_spread11951.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value11959, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value11959
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field11960 = match source_ml_source2_nominal_spread11951.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11961, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value11961
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field11962 = match source_ml_source2_nominal_spread11951.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value11963, _, _, _, _) => {
                                                                source_ml_source2_data_property_value11963.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field11964 = match source_ml_source2_nominal_spread11951.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value11965, _, _, _) => {
                                                                    source_ml_source2_data_property_value11965.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field11966 = match source_ml_source2_nominal_spread11951.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value11967, _, _) => {
                                                                        source_ml_source2_data_property_value11967.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field11968 = match source_ml_source2_nominal_spread11951.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value11969, _) => {
                                                                            source_ml_source2_data_property_value11969.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field11970 = match source_ml_source2_nominal_spread11951.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11971) => {
                                                                                source_ml_source2_data_property_value11971.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override11972 = (match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11950, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value11950
                                                                                }
                                                                            } + 1f64);
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field11952.clone(), source_ml_source2_nominal_field11954, source_ml_source2_nominal_field11956, source_ml_source2_nominal_field11958, source_ml_source2_nominal_override11972, source_ml_source2_nominal_field11962.clone(), source_ml_source2_nominal_field11964.clone(), source_ml_source2_nominal_field11966.clone(), source_ml_source2_nominal_field11968.clone(), source_ml_source2_nominal_field11970.clone())
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11973, _) => {
                                            let source_ml_source2_data_property_value11973 = *source_ml_source2_data_property_value11973;
                                            source_ml_source2_data_property_value11973.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11974) => {
                                            source_ml_source2_data_property_value11974
                                        }
                                    }, match body.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value11975) => {
                                            source_ml_source2_data_property_value11975
                                        }
                                    }));
                                    {
                                        let jump = { let ml_reference_array101 = match copied.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11976, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value11976.clone()
                                            }
                                        }; let ml_reference_projection101 = ml_reference_array101.borrow().len() as f64; ml_reference_projection101 };
                                        {
                                            let normal = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::CompleteJump(0f64, match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11977, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value11977
                                                }
                                            }));
                                            {
                                                let entry = { let ml_reference_array102 = match normal.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11978, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value11978.clone()
                                                    }
                                                }; let ml_reference_projection102 = ml_reference_array102.borrow().len() as f64; ml_reference_projection102 };
                                                {
                                                    let finalizer = self::lower_java_script_call_source_expression(source_finalizer.clone(), bindings.clone(), normal.clone(), (depth + 1f64));
                                                    {
                                                        let finished = self::append_java_script_call_source_instruction(match finalizer.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value11979, _) => {
                                                                let source_ml_source2_data_property_value11979 = *source_ml_source2_data_property_value11979;
                                                                source_ml_source2_data_property_value11979.clone()
                                                            }
                                                        }, self::JavaScriptCallInstruction::FinishFinally);
                                                        {
                                                            let end = { let ml_reference_array103 = match finished.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11980, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value11980.clone()
                                                                }
                                                            }; let ml_reference_projection103 = ml_reference_array103.borrow().len() as f64; ml_reference_projection103 };
                                                            {
                                                                let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values104 = { let ml_reference_value105 = (match finished.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11981, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value11981.clone()
                                                                    }
                                                                }).borrow().clone(); ml_reference_value105 }; let ml_callback104 = { let position = position; let entry = entry; let jump = jump; let end = end; let builder = builder.clone(); std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                    self::JavaScriptCallInstruction::PushFinally(entry)
                                                                } else {
                                                                    if (index == jump) {
                                                                        self::JavaScriptCallInstruction::CompleteJump(end, match builder.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11982, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value11982
                                                                            }
                                                                        })
                                                                    } else {
                                                                        instruction.clone()
                                                                    }
                                                                } }) }; ml_values104.iter().cloned().enumerate().map(|(ml_index104, ml_value104)| ml_callback104(ml_value104, (ml_index104 as f64))).collect::<Vec<_>>() }));
                                                                self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                                    let source_ml_source2_nominal_field11985 = match finished.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value11986, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value11986.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field11987 = match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value11988, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value11988
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field11989 = match finished.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value11990, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value11990
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field11991 = match finished.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value11992, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value11992
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field11993 = match finished.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11994, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value11994
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11995 = match finished.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value11996, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value11996.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11997 = match finished.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value11998, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value11998.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11999 = match finished.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12000, _, _) => {
                                                                                                        source_ml_source2_data_property_value12000.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12001 = match finished.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12002, _) => {
                                                                                                            source_ml_source2_data_property_value12002.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12003 = match finished.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12004) => {
                                                                                                                source_ml_source2_data_property_value12004.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override12006 = match builder.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value11983, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value11983
                                                                                                                }
                                                                                                            };
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field11987, source_ml_source2_nominal_field11989, source_ml_source2_nominal_field11991, source_ml_source2_nominal_override12006, source_ml_source2_nominal_field11995.clone(), source_ml_source2_nominal_field11997.clone(), source_ml_source2_nominal_field11999.clone(), source_ml_source2_nominal_field12001.clone(), source_ml_source2_nominal_field12003.clone())
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
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12007) => {
                                                                        source_ml_source2_data_property_value12007
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
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
    }
}
