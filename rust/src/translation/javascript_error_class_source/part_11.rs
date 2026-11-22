// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_finally(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceFinally(source_body, source_finalizer) => {
            let source_body = *source_body;
            let source_finalizer = *source_finalizer;
            if ((match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12000, _, _, _, _, _) => {
                    source_ml_source2_data_property_value12000
                }
            }) >= (1024f64)) {
                self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 104, 97, 110, 100, 108, 101, 114, 32, 98, 117, 100, 103, 101, 116])))
            } else {
                {
                    let result = self::allocate_java_script_call_source_register(builder.clone());
                    {
                        let position = { let ml_reference_array101 = match match result.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12001, _) => {
                                let source_ml_source2_data_property_value12001 = *source_ml_source2_data_property_value12001;
                                source_ml_source2_data_property_value12001.clone()
                            }
                        } {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12002, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12002.clone()
                            }
                        }; let ml_reference_projection101 = ml_reference_array101.borrow().len() as f64; ml_reference_projection101 };
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
                                        let jump = { let ml_reference_array102 = match copied.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12030, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value12030.clone()
                                            }
                                        }; let ml_reference_projection102 = ml_reference_array102.borrow().len() as f64; ml_reference_projection102 };
                                        {
                                            let normal = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::CompleteJump(0f64, match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12031, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12031
                                                }
                                            }));
                                            {
                                                let entry = { let ml_reference_array103 = match normal.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12032, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12032.clone()
                                                    }
                                                }; let ml_reference_projection103 = ml_reference_array103.borrow().len() as f64; ml_reference_projection103 };
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
                                                            let end = { let ml_reference_array104 = match finished.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12034, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12034.clone()
                                                                }
                                                            }; let ml_reference_projection104 = ml_reference_array104.borrow().len() as f64; ml_reference_projection104 };
                                                            {
                                                                let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values105 = { let ml_reference_value106 = (match finished.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12035, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12035.clone()
                                                                    }
                                                                }).borrow().clone(); ml_reference_value106 }; let ml_callback105 = { let position = position; let entry = entry; let jump = jump; let end = end; let builder = builder.clone(); std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
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
                                                                } }) }; ml_values105.iter().cloned().enumerate().map(|(ml_index105, ml_value105)| ml_callback105(ml_value105, (ml_index105 as f64))).collect::<Vec<_>>() }));
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
        _ => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 108, 121, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 105, 122, 101, 114])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_exception(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceTry(source_identity, source_body, source_handler) => {
            let source_body = *source_body;
            let source_handler = *source_handler;
            if (!(self::test_java_script_call_code_coordinate(source_identity, 65536f64)) || { let ml_values107 = { let ml_reference_value108 = bindings.borrow().clone(); ml_reference_value108 }; let ml_callback107 = { let source_identity = source_identity; std::rc::Rc::new(move |entry: self::JavaScriptCallSourceBinding| -> bool { ((match entry.clone() {
                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value12244, _, _, _) => {
                    source_ml_source2_data_property_value12244
                }
            }) == source_identity) }) }; ml_values107.iter().cloned().enumerate().any(|(ml_index107, ml_value107)| ml_callback107(ml_value107)) }) {
                self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103, 32, 111, 119, 110, 101, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 105, 110, 32, 105, 116, 115, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 99, 111, 112, 101])))
            } else {
                if ((match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12246, _, _, _, _, _) => {
                        source_ml_source2_data_property_value12246
                    }
                }) >= (1024f64)) {
                    self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114, 32, 98, 117, 100, 103, 101, 116])))
                } else {
                    {
                        let caught = self::allocate_java_script_call_source_register(builder.clone());
                        {
                            let result = self::allocate_java_script_call_source_register(match caught.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12247, _) => {
                                    let source_ml_source2_data_property_value12247 = *source_ml_source2_data_property_value12247;
                                    source_ml_source2_data_property_value12247.clone()
                                }
                            });
                            {
                                let position = { let ml_reference_array109 = match match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12248, _) => {
                                        let source_ml_source2_data_property_value12248 = *source_ml_source2_data_property_value12248;
                                        source_ml_source2_data_property_value12248.clone()
                                    }
                                } {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12249, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12249.clone()
                                    }
                                }; let ml_reference_projection109 = ml_reference_array109.borrow().len() as f64; ml_reference_projection109 };
                                {
                                    let protected_builder = self::append_java_script_call_source_instruction({
                                        let source_ml_source2_nominal_spread12252 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12250, _) => {
                                                let source_ml_source2_data_property_value12250 = *source_ml_source2_data_property_value12250;
                                                source_ml_source2_data_property_value12250.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12253 = match source_ml_source2_nominal_spread12252.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12254, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12254.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12255 = match source_ml_source2_nominal_spread12252.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12256, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12256
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12257 = match source_ml_source2_nominal_spread12252.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12258, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12258
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12259 = match source_ml_source2_nominal_spread12252.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12260, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12260
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12261 = match source_ml_source2_nominal_spread12252.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12262, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12262
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12263 = match source_ml_source2_nominal_spread12252.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12264, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12264.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12265 = match source_ml_source2_nominal_spread12252.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12266, _, _, _) => {
                                                                            source_ml_source2_data_property_value12266.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12267 = match source_ml_source2_nominal_spread12252.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12268, _, _) => {
                                                                                source_ml_source2_data_property_value12268.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12269 = match source_ml_source2_nominal_spread12252.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12270, _) => {
                                                                                    source_ml_source2_data_property_value12270.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12271 = match source_ml_source2_nominal_spread12252.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12272) => {
                                                                                        source_ml_source2_data_property_value12272.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override12273 = (match builder.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12251, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12251
                                                                                        }
                                                                                    } + 1f64);
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12253.clone(), source_ml_source2_nominal_field12255, source_ml_source2_nominal_field12257, source_ml_source2_nominal_field12259, source_ml_source2_nominal_override12273, source_ml_source2_nominal_field12263.clone(), source_ml_source2_nominal_field12265.clone(), source_ml_source2_nominal_field12267.clone(), source_ml_source2_nominal_field12269.clone(), source_ml_source2_nominal_field12271.clone())
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
                                    }, self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12274) => {
                                            source_ml_source2_data_property_value12274
                                        }
                                    }, 0f64));
                                    {
                                        let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), protected_builder.clone(), (depth + 1f64));
                                        {
                                            let popped = self::append_java_script_call_source_instruction({
                                                let source_ml_source2_nominal_spread12277 = match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12275, _) => {
                                                        let source_ml_source2_data_property_value12275 = *source_ml_source2_data_property_value12275;
                                                        source_ml_source2_data_property_value12275.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12278 = match source_ml_source2_nominal_spread12277.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12279, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12279.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12280 = match source_ml_source2_nominal_spread12277.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12281, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12281
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12282 = match source_ml_source2_nominal_spread12277.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12283, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12283
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12284 = match source_ml_source2_nominal_spread12277.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12285, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12285
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12286 = match source_ml_source2_nominal_spread12277.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12287, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12287
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12288 = match source_ml_source2_nominal_spread12277.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12289, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12289.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12290 = match source_ml_source2_nominal_spread12277.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12291, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12291.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12292 = match source_ml_source2_nominal_spread12277.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12293, _, _) => {
                                                                                        source_ml_source2_data_property_value12293.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12294 = match source_ml_source2_nominal_spread12277.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12295, _) => {
                                                                                            source_ml_source2_data_property_value12295.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12296 = match source_ml_source2_nominal_spread12277.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12297) => {
                                                                                                source_ml_source2_data_property_value12297.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override12298 = match builder.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12276, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12276
                                                                                                }
                                                                                            };
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12278.clone(), source_ml_source2_nominal_field12280, source_ml_source2_nominal_field12282, source_ml_source2_nominal_field12284, source_ml_source2_nominal_override12298, source_ml_source2_nominal_field12288.clone(), source_ml_source2_nominal_field12290.clone(), source_ml_source2_nominal_field12292.clone(), source_ml_source2_nominal_field12294.clone(), source_ml_source2_nominal_field12296.clone())
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
                                            }, self::JavaScriptCallInstruction::PopHandler);
                                            {
                                                let copied = self::append_java_script_call_source_instruction(popped.clone(), self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12299) => {
                                                        source_ml_source2_data_property_value12299
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12300) => {
                                                        source_ml_source2_data_property_value12300
                                                    }
                                                }));
                                                {
                                                    let jump = { let ml_reference_array110 = match copied.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12301, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12301.clone()
                                                        }
                                                    }; let ml_reference_projection110 = ml_reference_array110.borrow().len() as f64; ml_reference_projection110 };
                                                    {
                                                        let alternative = self::append_java_script_call_source_instruction(copied.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                        {
                                                            let target = { let ml_reference_array111 = match alternative.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12302, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12302.clone()
                                                                }
                                                            }; let ml_reference_projection111 = ml_reference_array111.borrow().len() as f64; ml_reference_projection111 };
                                                            {
                                                                let handler = self::lower_java_script_call_source_expression(source_handler.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value12308 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                        let source_ml_source2_constructor_field12305 = match caught.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12304) => {
                                                                                source_ml_source2_data_property_value12304
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_identity, source_ml_source2_constructor_field12305, true, None::<(Option<bool>,)>)
                                                                    }]));
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value112 = bindings.borrow().clone(); ml_reference_value112 }), { let ml_reference_value113 = source_ml_source2_concat_value12308.borrow().clone(); ml_reference_value113 })
                                                                })), if match (match alternative.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12309, _, _) => {
                                                                        ((source_ml_source2_data_property_value12309.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    alternative.clone()
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_nominal_field12312 = match alternative.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12313, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12313.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12314 = match alternative.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12315, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value12315
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12316 = match alternative.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12317, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12317
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12318 = match alternative.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12319, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value12319
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12320 = match alternative.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12321, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12321
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12322 = match alternative.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12323, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12323.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12324 = match alternative.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12325, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12325.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12326 = match alternative.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12327, _, _) => {
                                                                                                            source_ml_source2_data_property_value12327.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12328 = match alternative.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12329, _) => {
                                                                                                                source_ml_source2_data_property_value12329.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12330 = match alternative.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12331) => {
                                                                                                                    source_ml_source2_data_property_value12331.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override12332 = (match builder.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12310, _, _) => {
                                                                                                                        ((source_ml_source2_data_property_value12310.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                    }
                                                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12312.clone(), source_ml_source2_nominal_field12314, source_ml_source2_nominal_field12316, source_ml_source2_nominal_field12318, source_ml_source2_nominal_field12320, source_ml_source2_nominal_field12322.clone(), source_ml_source2_nominal_field12324.clone(), Some((Some(source_ml_source2_nominal_override12332.clone()),)), source_ml_source2_nominal_field12328.clone(), source_ml_source2_nominal_field12330.clone())
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
                                                                    let finished = self::append_java_script_call_source_instruction(match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12333, _) => {
                                                                            let source_ml_source2_data_property_value12333 = *source_ml_source2_data_property_value12333;
                                                                            source_ml_source2_data_property_value12333.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Copy(match result.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12334) => {
                                                                            source_ml_source2_data_property_value12334
                                                                        }
                                                                    }, match handler.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12335) => {
                                                                            source_ml_source2_data_property_value12335
                                                                        }
                                                                    }));
                                                                    {
                                                                        let end = { let ml_reference_array114 = match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12336, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12336.clone()
                                                                            }
                                                                        }; let ml_reference_projection114 = ml_reference_array114.borrow().len() as f64; ml_reference_projection114 };
                                                                        if ((match finished.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12338, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value12338.clone()
                                                                            }
                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(finished.clone()), 0f64)
                                                                        } else {
                                                                            {
                                                                                let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values115 = { let ml_reference_value116 = (match finished.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12339, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value12339.clone()
                                                                                    }
                                                                                }).borrow().clone(); ml_reference_value116 }; let ml_callback115 = { let position = position; let caught = caught.clone(); let target = target; let jump = jump; let end = end; std::rc::Rc::new(move |entry: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == position) {
                                                                                    self::JavaScriptCallInstruction::PushHandler(match caught.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12340) => {
                                                                                            source_ml_source2_data_property_value12340
                                                                                        }
                                                                                    }, target)
                                                                                } else {
                                                                                    if (index == jump) {
                                                                                        self::JavaScriptCallInstruction::Jump(end)
                                                                                    } else {
                                                                                        entry.clone()
                                                                                    }
                                                                                } }) }; ml_values115.iter().cloned().enumerate().map(|(ml_index115, ml_value115)| ml_callback115(ml_value115, (ml_index115 as f64))).collect::<Vec<_>>() }));
                                                                                {
                                                                                    let updated = {
                                                                                        let source_ml_source2_nominal_field12342 = match finished.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12343, _, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value12343.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12344 = match finished.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12345, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value12345
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12346 = match finished.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12347, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value12347
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12348 = match finished.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12349, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12349
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12350 = match finished.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12351, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12351
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12352 = match finished.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12353, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12353.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12354 = match finished.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12355, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value12355.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12356 = match finished.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12357, _, _) => {
                                                                                                                            source_ml_source2_data_property_value12357.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12358 = match finished.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12359, _) => {
                                                                                                                                source_ml_source2_data_property_value12359.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12360 = match finished.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12361) => {
                                                                                                                                    source_ml_source2_data_property_value12361.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field12344, source_ml_source2_nominal_field12346, source_ml_source2_nominal_field12348, source_ml_source2_nominal_field12350, source_ml_source2_nominal_field12352.clone(), source_ml_source2_nominal_field12354.clone(), source_ml_source2_nominal_field12356.clone(), source_ml_source2_nominal_field12358.clone(), source_ml_source2_nominal_field12360.clone())
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
                                                                                        let continuation = if (match (match match body.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12363, _) => {
                                                                                                let source_ml_source2_data_property_value12363 = *source_ml_source2_data_property_value12363;
                                                                                                source_ml_source2_data_property_value12363.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12364, _, _) => {
                                                                                                ((source_ml_source2_data_property_value12364.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && match (match match handler.clone() {
                                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12365, _) => {
                                                                                                let source_ml_source2_data_property_value12365 = *source_ml_source2_data_property_value12365;
                                                                                                source_ml_source2_data_property_value12365.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12366, _, _) => {
                                                                                                ((source_ml_source2_data_property_value12366.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                            updated.clone()
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source2_nominal_spread12373 = updated.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12374 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12375, _, _, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value12375.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12376 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12377, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value12377
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12378 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12379, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value12379
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12380 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12381, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value12381
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12382 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12383, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value12383
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12384 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12385, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value12385.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12386 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12387, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value12387.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12388 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12389, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value12389.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12390 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12391, _) => {
                                                                                                                                            source_ml_source2_data_property_value12391.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12392 = match source_ml_source2_nominal_spread12373.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12393) => {
                                                                                                                                                source_ml_source2_data_property_value12393.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override12394 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values117 = { let ml_reference_value118 = ((match match body.clone() {
                                                                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12367, _) => {
                                                                                                                                                    let source_ml_source2_data_property_value12367 = *source_ml_source2_data_property_value12367;
                                                                                                                                                    source_ml_source2_data_property_value12367.clone()
                                                                                                                                                }
                                                                                                                                            } {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12368, _, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value12368.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value118 }; let ml_callback117 = { let handler = handler.clone(); std::rc::Rc::new(move |identity: f64| -> bool { self::ml_array::includes(&{ let ml_reference_value119 = ((match match handler.clone() {
                                                                                                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value12371, _) => {
                                                                                                                                                    let source_ml_source2_data_property_value12371 = *source_ml_source2_data_property_value12371;
                                                                                                                                                    source_ml_source2_data_property_value12371.clone()
                                                                                                                                                }
                                                                                                                                            } {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12372, _, _) => {
                                                                                                                                                    ((source_ml_source2_data_property_value12372.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                                                                }
                                                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value119 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())) }) }; ml_values117.iter().cloned().enumerate().filter_map(|(ml_index117, ml_value117)| if ml_callback117(ml_value117.clone()) { Some(ml_value117) } else { None }).collect::<Vec<_>>() }));
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12374.clone(), source_ml_source2_nominal_field12376, source_ml_source2_nominal_field12378, source_ml_source2_nominal_field12380, source_ml_source2_nominal_field12382, source_ml_source2_nominal_field12384.clone(), source_ml_source2_nominal_field12386.clone(), Some((Some(source_ml_source2_nominal_override12394.clone()),)), source_ml_source2_nominal_field12390.clone(), source_ml_source2_nominal_field12392.clone())
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
                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value12395) => {
                                                                                                source_ml_source2_data_property_value12395
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
            }
        }
        _ => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 99, 111, 109, 112, 105, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 97, 116, 99, 104, 32, 98, 105, 110, 100, 105, 110, 103])))
        }
    }
}
