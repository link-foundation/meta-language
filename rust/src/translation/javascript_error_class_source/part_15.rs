// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::finish_java_script_call_source_control_scope;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call_source_control_scope;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_binding_scope_loop73;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_iterator_scope(slot: f64, realm: self::ml::Big, input_source: self::JavaScriptCallSource, iterator_key: self::JavaScriptCallSource, body: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    if ((!(self::test_java_script_call_code_coordinate(slot, 65536f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
        self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![102, 111, 114, 45, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 103, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
    } else {
        {
            let input = self::lower_java_script_call_source_expression(input_source.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
            {
                let held = self::allocate_java_script_call_source_register(match input.clone() {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14224, _) => {
                        let source_ml_source2_data_property_value14224 = *source_ml_source2_data_property_value14224;
                        source_ml_source2_data_property_value14224.clone()
                    }
                });
                {
                    let saved = self::append_java_script_call_source_instruction(match held.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14225, _) => {
                            let source_ml_source2_data_property_value14225 = *source_ml_source2_data_property_value14225;
                            source_ml_source2_data_property_value14225.clone()
                        }
                    }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14226) => {
                            source_ml_source2_data_property_value14226
                        }
                    }, match input.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14227) => {
                            source_ml_source2_data_property_value14227
                        }
                    }));
                    {
                        let key = self::lower_java_script_call_source_expression(iterator_key.clone(), bindings.clone(), saved.clone(), (depth + 1f64));
                        {
                            let held_key = self::allocate_java_script_call_source_register(match key.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14228, _) => {
                                    let source_ml_source2_data_property_value14228 = *source_ml_source2_data_property_value14228;
                                    source_ml_source2_data_property_value14228.clone()
                                }
                            });
                            {
                                let keyed = self::append_java_script_call_source_instruction(match held_key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14229, _) => {
                                        let source_ml_source2_data_property_value14229 = *source_ml_source2_data_property_value14229;
                                        source_ml_source2_data_property_value14229.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match held_key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14230) => {
                                        source_ml_source2_data_property_value14230
                                    }
                                }, match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14231) => {
                                        source_ml_source2_data_property_value14231
                                    }
                                }));
                                {
                                    let value = self::allocate_java_script_call_source_register(keyed.clone());
                                    {
                                        let done = self::allocate_java_script_call_source_register(match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14232, _) => {
                                                let source_ml_source2_data_property_value14232 = *source_ml_source2_data_property_value14232;
                                                source_ml_source2_data_property_value14232.clone()
                                            }
                                        });
                                        {
                                            let opened = self::append_java_script_call_source_instruction(match done.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14233, _) => {
                                                    let source_ml_source2_data_property_value14233 = *source_ml_source2_data_property_value14233;
                                                    source_ml_source2_data_property_value14233.clone()
                                                }
                                            }, {
                                                let source_ml_source2_constructor_field14235 = match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14234) => {
                                                        source_ml_source2_data_property_value14234
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_constructor_field14237 = match done.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14236) => {
                                                            source_ml_source2_data_property_value14236
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_constructor_field14239 = match held.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14238) => {
                                                                source_ml_source2_data_property_value14238
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field14241 = match held_key.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14240) => {
                                                                    source_ml_source2_data_property_value14240
                                                                }
                                                            };
                                                            self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field14235, source_ml_source2_constructor_field14237, source_ml_source2_constructor_field14239, source_ml_source2_constructor_field14241, slot, self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), realm.clone(), None::<(Option<bool>,)>)
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
        let binding_error = self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 111, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110, 116, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103]);
        match source.clone() {
            self::JavaScriptCallSource::SourceIteratorLoop(slot, target, binding, mutable, source_body, realm, scope_bindings) => {
                let source_body = *source_body;
                if ((((!(self::test_java_script_call_code_coordinate(binding, 65536f64)) || !(self::test_java_script_call_code_coordinate(slot, 65536f64))) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) || { let ml_values190 = { let ml_reference_value191 = bindings.borrow().clone(); ml_reference_value191 }; let ml_callback190 = { let binding = binding; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14423, _, _, _) => {
                        source_ml_source2_data_property_value14423
                    }
                }) == binding) }) }; ml_values190.iter().cloned().enumerate().any(|(ml_index190, ml_value190)| ml_callback190(ml_value190)) }) {
                    self::refuse_java_script_call_source(builder.clone(), &(binding_error))
                } else {
                    {
                        let scope = (((scope_bindings.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                        if ((({ let ml_reference_array192 = scope.clone(); let ml_reference_projection192 = ml_reference_array192.borrow().len() as f64; ml_reference_projection192 }) > (4096f64)) || { let ml_values193 = { let ml_reference_value194 = scope.borrow().clone(); ml_reference_value194 }; let ml_callback193 = { let scope = scope.clone(); let bindings = bindings.clone(); let binding = binding; std::rc::Rc::new(move |identity: f64, index: f64| -> bool { (((!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || ((self::ml_array::index_of(&{ let ml_reference_value195 = scope.borrow().clone(); ml_reference_value195 }, &identity, 0f64, false, |a, b| a == b)) != index)) || !({ let ml_values196 = { let ml_reference_value197 = bindings.borrow().clone(); ml_reference_value197 }; let ml_callback196 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14425, _, _, _) => {
                                source_ml_source2_data_property_value14425
                            }
                        }) == identity) }) }; ml_values196.iter().cloned().enumerate().any(|(ml_index196, ml_value196)| ml_callback196(ml_value196)) })) || (identity == binding)) }) }; ml_values193.iter().cloned().enumerate().any(|(ml_index193, ml_value193)| ml_callback193(ml_value193, (ml_index193 as f64))) }) {
                            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 99, 111, 112, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 117, 116, 101, 114, 32, 84, 68, 90, 32, 99, 101, 108, 108, 115, 32, 97, 110, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 100, 32, 118, 97, 108, 117, 101])))
                        } else {
                            {
                                let visible = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values198 = { let ml_reference_value199 = bindings.borrow().clone(); ml_reference_value199 }; let ml_callback198 = { let scope = scope.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { !(self::ml_array::includes(&{ let ml_reference_value200 = scope.borrow().clone(); ml_reference_value200 }, &match row.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14426, _, _, _) => {
                                        source_ml_source2_data_property_value14426
                                    }
                                }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values198.iter().cloned().enumerate().filter_map(|(ml_index198, ml_value198)| if ml_callback198(ml_value198.clone()) { Some(ml_value198) } else { None }).collect::<Vec<_>>() }));
                                {
                                    let scoped = self::push_java_script_call_source_control_scope(builder.clone(), target, &(self::ml_string::Text::from_units(vec![108, 111, 111, 112])));
                                    {
                                        let value = self::allocate_java_script_call_source_register(scoped.clone());
                                        {
                                            let done = self::allocate_java_script_call_source_register(match value.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14427, _) => {
                                                    let source_ml_source2_data_property_value14427 = *source_ml_source2_data_property_value14427;
                                                    source_ml_source2_data_property_value14427.clone()
                                                }
                                            });
                                            {
                                                let start = { let ml_reference_array201 = match match done.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14428, _) => {
                                                        let source_ml_source2_data_property_value14428 = *source_ml_source2_data_property_value14428;
                                                        source_ml_source2_data_property_value14428.clone()
                                                    }
                                                } {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14429, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value14429.clone()
                                                    }
                                                }; let ml_reference_projection201 = ml_reference_array201.borrow().len() as f64; ml_reference_projection201 };
                                                {
                                                    let stepped = self::append_java_script_call_source_instruction(match done.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14430, _) => {
                                                            let source_ml_source2_data_property_value14430 = *source_ml_source2_data_property_value14430;
                                                            source_ml_source2_data_property_value14430.clone()
                                                        }
                                                    }, {
                                                        let source_ml_source2_constructor_field14432 = match value.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14431) => {
                                                                source_ml_source2_data_property_value14431
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field14434 = match done.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14433) => {
                                                                    source_ml_source2_data_property_value14433
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_constructor_field14435 = 0f64;
                                                                {
                                                                    let source_ml_source2_constructor_field14436 = 0f64;
                                                                    self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field14432, source_ml_source2_constructor_field14434, source_ml_source2_constructor_field14435, source_ml_source2_constructor_field14436, slot, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), None::<(Option<bool>,)>)
                                                                }
                                                            }
                                                        }
                                                    });
                                                    {
                                                        let branch = { let ml_reference_array202 = match stepped.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14440, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value14440.clone()
                                                            }
                                                        }; let ml_reference_projection202 = ml_reference_array202.borrow().len() as f64; ml_reference_projection202 };
                                                        {
                                                            let test = self::append_java_script_call_source_instruction(stepped.clone(), self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14441) => {
                                                                    source_ml_source2_data_property_value14441
                                                                }
                                                            }, 0f64, (branch + 1f64)));
                                                            {
                                                                let body = self::lower_java_script_call_source_expression(source_body.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value14447 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                        let source_ml_source2_constructor_field14444 = match value.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14443) => {
                                                                                source_ml_source2_data_property_value14443
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(binding, source_ml_source2_constructor_field14444, mutable, None::<(Option<bool>,)>)
                                                                    }]));
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value203 = visible.borrow().clone(); ml_reference_value203 }), { let ml_reference_value204 = source_ml_source2_concat_value14447.borrow().clone(); ml_reference_value204 })
                                                                })), test.clone(), (depth + 1f64));
                                                                {
                                                                    let loop_ = self::append_java_script_call_source_instruction(match body.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14448, _) => {
                                                                            let source_ml_source2_data_property_value14448 = *source_ml_source2_data_property_value14448;
                                                                            source_ml_source2_data_property_value14448.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Jump(start));
                                                                    {
                                                                        let end = { let ml_reference_array205 = match loop_.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14449, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value14449.clone()
                                                                            }
                                                                        }; let ml_reference_projection205 = ml_reference_array205.borrow().len() as f64; ml_reference_projection205 };
                                                                        {
                                                                            let patched = {
                                                                                let source_ml_source2_nominal_spread14452 = loop_.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field14453 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14454, _, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value14454.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field14455 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value14456, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value14456
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field14457 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value14458, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value14458
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field14459 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value14460, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value14460
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field14461 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value14462, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value14462
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field14463 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value14464, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value14464.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field14465 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value14466, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value14466.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14467 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value14468, _, _) => {
                                                                                                                        source_ml_source2_data_property_value14468.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14469 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14470, _) => {
                                                                                                                            source_ml_source2_data_property_value14470.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14471 = match source_ml_source2_nominal_spread14452.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14472) => {
                                                                                                                                source_ml_source2_data_property_value14472.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override14473 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values206 = { let ml_reference_value207 = (match loop_.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14450, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value14450.clone()
                                                                                                                                }
                                                                                                                            }).borrow().clone(); ml_reference_value207 }; let ml_callback206 = { let branch = branch; let done = done.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                                                                                self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14451) => {
                                                                                                                                        source_ml_source2_data_property_value14451
                                                                                                                                    }
                                                                                                                                }, end, (branch + 1f64))
                                                                                                                            } else {
                                                                                                                                instruction.clone()
                                                                                                                            } }) }; ml_values206.iter().cloned().enumerate().map(|(ml_index206, ml_value206)| ml_callback206(ml_value206, (ml_index206 as f64))).collect::<Vec<_>>() }));
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override14473.clone(), source_ml_source2_nominal_field14455, source_ml_source2_nominal_field14457, source_ml_source2_nominal_field14459, source_ml_source2_nominal_field14461, source_ml_source2_nominal_field14463.clone(), source_ml_source2_nominal_field14465.clone(), source_ml_source2_nominal_field14467.clone(), source_ml_source2_nominal_field14469.clone(), source_ml_source2_nominal_field14471.clone())
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
            _ => {
                self::refuse_java_script_call_source(builder.clone(), &(binding_error))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_binding_scope(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    {
        let declaration_error = self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 111, 117, 116, 112, 117, 116, 32, 115, 99, 111, 112, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115]);
        match source.clone() {
            self::JavaScriptCallSource::SourceBindingScope(declarations, initializer, body) => {
                let initializer = *initializer;
                let body = *body;
                if (({ let ml_reference_array208 = declarations.clone(); let ml_reference_projection208 = ml_reference_array208.borrow().len() as f64; ml_reference_projection208 }) > (4096f64)) {
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
                                        let source_ml_source2_s14655 = self::source_ml_source1_temporary_lower_java_script_call_source_binding_scope_loop73(bindings.clone(), current.clone(), entries.clone(), source_ml_source1_temporary_values72.clone(), source_ml_source1_temporary_index72.clone());
                                        match source_ml_source2_s14655.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Done(current_2) => {
                                                let current_2 = *current_2;
                                                {
                                                    let initialized = self::lower_java_script_call_source_expression(initializer.clone(), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value209 = bindings.borrow().clone(); ml_reference_value209 }), { let ml_reference_value210 = entries.borrow().clone(); ml_reference_value210 }))), current_2.clone(), (depth + 1f64));
                                                    if { let ml_values211 = { let ml_reference_value212 = entries.borrow().clone(); ml_reference_value212 }; let ml_callback211 = { let initialized = initialized.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { !(self::ml_array::includes(&{ let ml_reference_value213 = ((match match initialized.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14660, _) => {
                                                            let source_ml_source2_data_property_value14660 = *source_ml_source2_data_property_value14660;
                                                            source_ml_source2_data_property_value14660.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value14661, _, _) => {
                                                            ((source_ml_source2_data_property_value14661.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value213 }, &match row.clone() {
                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14662, _, _, _) => {
                                                            source_ml_source2_data_property_value14662
                                                        }
                                                    }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values211.iter().cloned().enumerate().any(|(ml_index211, ml_value211)| ml_callback211(ml_value211)) } {
                                                        self::refuse_java_script_call_source(match initialized.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14663, _) => {
                                                                let source_ml_source2_data_property_value14663 = *source_ml_source2_data_property_value14663;
                                                                source_ml_source2_data_property_value14663.clone()
                                                            }
                                                        }, &(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 98, 105, 110, 100, 105, 110, 103, 32, 111, 117, 116, 112, 117, 116, 32, 109, 117, 115, 116, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 98, 111, 100, 121])))
                                                    } else {
                                                        self::lower_java_script_call_source_expression(body.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value14670 = bindings.clone();
                                                            {
                                                                let source_ml_source2_concat_value14671 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values214 = { let ml_reference_value215 = entries.borrow().clone(); ml_reference_value215 }; let ml_callback214 = std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> self::JavaScriptCallSourceBinding { {
                                                                    let source_ml_source2_constructor_field14665 = match row.clone() {
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14664, _, _, _) => {
                                                                            source_ml_source2_data_property_value14664
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field14667 = match row.clone() {
                                                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value14666, _, _) => {
                                                                                source_ml_source2_data_property_value14666
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field14669 = match row.clone() {
                                                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value14668, _) => {
                                                                                    source_ml_source2_data_property_value14668
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_constructor_field14665, source_ml_source2_constructor_field14667, source_ml_source2_constructor_field14669, None::<(Option<bool>,)>)
                                                                        }
                                                                    }
                                                                } }); ml_values214.iter().cloned().enumerate().map(|(ml_index214, ml_value214)| ml_callback214(ml_value214)).collect::<Vec<_>>() }));
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value216 = source_ml_source2_concat_value14670.borrow().clone(); ml_reference_value216 }), { let ml_reference_value217 = source_ml_source2_concat_value14671.borrow().clone(); ml_reference_value217 })
                                                            }
                                                        })), match initialized.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14672, _) => {
                                                                let source_ml_source2_data_property_value14672 = *source_ml_source2_data_property_value14672;
                                                                source_ml_source2_data_property_value14672.clone()
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
            _ => {
                self::refuse_java_script_call_source(builder.clone(), &(declaration_error))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_iterator_remainder(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    {
        let region_error = self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 110, 101, 100, 32, 114, 101, 103, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]);
        match source.clone() {
            self::JavaScriptCallSource::SourceIteratorRemainder(slot, source_prototype, realm) => {
                let source_prototype = *source_prototype;
                if ((!(self::test_java_script_call_code_coordinate(slot, 65536f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::refuse_java_script_call_source(builder.clone(), &(region_error))
                } else {
                    {
                        let prototype = self::lower_java_script_call_source_expression(source_prototype.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                        {
                            let object = self::allocate_java_script_call_source_register(match prototype.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14854, _) => {
                                    let source_ml_source2_data_property_value14854 = *source_ml_source2_data_property_value14854;
                                    source_ml_source2_data_property_value14854.clone()
                                }
                            });
                            {
                                let created = self::append_java_script_call_source_instruction(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14855, _) => {
                                        let source_ml_source2_data_property_value14855 = *source_ml_source2_data_property_value14855;
                                        source_ml_source2_data_property_value14855.clone()
                                    }
                                }, self::JavaScriptCallInstruction::ArrayAllocate(match object.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14856) => {
                                        source_ml_source2_data_property_value14856
                                    }
                                }, match prototype.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14857) => {
                                        source_ml_source2_data_property_value14857
                                    }
                                }, 0f64));
                                {
                                    let index = self::allocate_java_script_call_source_register(created.clone());
                                    {
                                        let initial = self::append_java_script_call_source_instruction(match index.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14858, _) => {
                                                let source_ml_source2_data_property_value14858 = *source_ml_source2_data_property_value14858;
                                                source_ml_source2_data_property_value14858.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Constant(match index.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14859) => {
                                                source_ml_source2_data_property_value14859
                                            }
                                        }, Box::new(self::JavaScriptSemanticValue::Number(0f64))));
                                        {
                                            let value = self::allocate_java_script_call_source_register(initial.clone());
                                            {
                                                let done = self::allocate_java_script_call_source_register(match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14860, _) => {
                                                        let source_ml_source2_data_property_value14860 = *source_ml_source2_data_property_value14860;
                                                        source_ml_source2_data_property_value14860.clone()
                                                    }
                                                });
                                                {
                                                    let key = self::allocate_java_script_call_source_register(match done.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14861, _) => {
                                                            let source_ml_source2_data_property_value14861 = *source_ml_source2_data_property_value14861;
                                                            source_ml_source2_data_property_value14861.clone()
                                                        }
                                                    });
                                                    {
                                                        let written = self::allocate_java_script_call_source_register(match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14862, _) => {
                                                                let source_ml_source2_data_property_value14862 = *source_ml_source2_data_property_value14862;
                                                                source_ml_source2_data_property_value14862.clone()
                                                            }
                                                        });
                                                        {
                                                            let start = { let ml_reference_array218 = match match written.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14863, _) => {
                                                                    let source_ml_source2_data_property_value14863 = *source_ml_source2_data_property_value14863;
                                                                    source_ml_source2_data_property_value14863.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14864, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value14864.clone()
                                                                }
                                                            }; let ml_reference_projection218 = ml_reference_array218.borrow().len() as f64; ml_reference_projection218 };
                                                            {
                                                                let stepped = self::append_java_script_call_source_instruction(match written.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14865, _) => {
                                                                        let source_ml_source2_data_property_value14865 = *source_ml_source2_data_property_value14865;
                                                                        source_ml_source2_data_property_value14865.clone()
                                                                    }
                                                                }, {
                                                                    let source_ml_source2_constructor_field14867 = match value.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14866) => {
                                                                            source_ml_source2_data_property_value14866
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field14869 = match done.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14868) => {
                                                                                source_ml_source2_data_property_value14868
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field14870 = 0f64;
                                                                            {
                                                                                let source_ml_source2_constructor_field14871 = 0f64;
                                                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field14867, source_ml_source2_constructor_field14869, source_ml_source2_constructor_field14870, source_ml_source2_constructor_field14871, slot, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), None::<(Option<bool>,)>)
                                                                            }
                                                                        }
                                                                    }
                                                                });
                                                                {
                                                                    let branch = { let ml_reference_array219 = match stepped.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14875, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value14875.clone()
                                                                        }
                                                                    }; let ml_reference_projection219 = ml_reference_array219.borrow().len() as f64; ml_reference_projection219 };
                                                                    {
                                                                        let test = self::append_java_script_call_source_instruction(stepped.clone(), self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14876) => {
                                                                                source_ml_source2_data_property_value14876
                                                                            }
                                                                        }, 0f64, (branch + 1f64)));
                                                                        {
                                                                            let keyed = self::append_java_script_call_source_instruction(test.clone(), self::JavaScriptCallInstruction::PropertyKey(match key.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14877) => {
                                                                                    source_ml_source2_data_property_value14877
                                                                                }
                                                                            }, match index.clone() {
                                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14878) => {
                                                                                    source_ml_source2_data_property_value14878
                                                                                }
                                                                            }, realm.clone()));
                                                                            {
                                                                                let defined = self::append_java_script_call_source_instruction(keyed.clone(), self::JavaScriptCallInstruction::OrdinaryDataProperty(match written.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14879) => {
                                                                                        source_ml_source2_data_property_value14879
                                                                                    }
                                                                                }, match object.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14880) => {
                                                                                        source_ml_source2_data_property_value14880
                                                                                    }
                                                                                }, match key.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14881) => {
                                                                                        source_ml_source2_data_property_value14881
                                                                                    }
                                                                                }, match value.clone() {
                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14882) => {
                                                                                        source_ml_source2_data_property_value14882
                                                                                    }
                                                                                }, realm.clone()));
                                                                                {
                                                                                    let advanced = self::append_java_script_call_source_instruction(defined.clone(), self::JavaScriptCallInstruction::Primitive(match index.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14883) => {
                                                                                            source_ml_source2_data_property_value14883
                                                                                        }
                                                                                    }, self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match index.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14884) => {
                                                                                            source_ml_source2_data_property_value14884
                                                                                        }
                                                                                    }]))));
                                                                                    {
                                                                                        let loop_ = self::append_java_script_call_source_instruction(advanced.clone(), self::JavaScriptCallInstruction::Jump(start));
                                                                                        {
                                                                                            let end = { let ml_reference_array220 = match loop_.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14885, _, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value14885.clone()
                                                                                                }
                                                                                            }; let ml_reference_projection220 = ml_reference_array220.borrow().len() as f64; ml_reference_projection220 };
                                                                                            {
                                                                                                let source_ml_source2_constructor_field14887 = match object.clone() {
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14886) => {
                                                                                                        source_ml_source2_data_property_value14886
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field14912 = {
                                                                                                        let source_ml_source2_nominal_spread14890 = loop_.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field14891 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14892, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value14892.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14893 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value14894, _, _, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value14894
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14895 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value14896, _, _, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value14896
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14897 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value14898, _, _, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value14898
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field14899 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value14900, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value14900
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field14901 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value14902, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value14902.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field14903 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value14904, _, _, _) => {
                                                                                                                                            source_ml_source2_data_property_value14904.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field14905 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value14906, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value14906.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field14907 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14908, _) => {
                                                                                                                                                    source_ml_source2_data_property_value14908.clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field14909 = match source_ml_source2_nominal_spread14890.clone() {
                                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14910) => {
                                                                                                                                                        source_ml_source2_data_property_value14910.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override14911 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values221 = { let ml_reference_value222 = (match loop_.clone() {
                                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14888, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                            source_ml_source2_data_property_value14888.clone()
                                                                                                                                                        }
                                                                                                                                                    }).borrow().clone(); ml_reference_value222 }; let ml_callback221 = { let branch = branch; let done = done.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, at: f64| -> self::JavaScriptCallInstruction { if (at == branch) {
                                                                                                                                                        self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14889) => {
                                                                                                                                                                source_ml_source2_data_property_value14889
                                                                                                                                                            }
                                                                                                                                                        }, end, (branch + 1f64))
                                                                                                                                                    } else {
                                                                                                                                                        instruction.clone()
                                                                                                                                                    } }) }; ml_values221.iter().cloned().enumerate().map(|(ml_index221, ml_value221)| ml_callback221(ml_value221, (ml_index221 as f64))).collect::<Vec<_>>() }));
                                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override14911.clone(), source_ml_source2_nominal_field14893, source_ml_source2_nominal_field14895, source_ml_source2_nominal_field14897, source_ml_source2_nominal_field14899, source_ml_source2_nominal_field14901.clone(), source_ml_source2_nominal_field14903.clone(), source_ml_source2_nominal_field14905.clone(), source_ml_source2_nominal_field14907.clone(), source_ml_source2_nominal_field14909.clone())
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
                                                                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field14912.clone()), source_ml_source2_constructor_field14887)
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
            _ => {
                self::refuse_java_script_call_source(builder.clone(), &(region_error))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_error_constant(value: self::JavaScriptSemanticValue) -> self::JavaScriptCallSource {
    self::JavaScriptCallSource::SourceConstant(Box::new(value.clone()))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_error_source(context: self::JavaScriptErrorSourceContext, error: &self::ml_string::Text) -> self::JavaScriptErrorSourceExpression {
    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)), Box::new({
        let source_ml_source2_nominal_field14914 = match context.clone() {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value14915, _, _, _, _, _, _, _, _, _) => {
                let source_ml_source2_data_property_value14915 = *source_ml_source2_data_property_value14915;
                source_ml_source2_data_property_value14915.clone()
            }
        };
        {
            let source_ml_source2_nominal_field14916 = match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14917, _, _, _, _, _, _, _, _) => {
                    let source_ml_source2_data_property_value14917 = *source_ml_source2_data_property_value14917;
                    source_ml_source2_data_property_value14917.clone()
                }
            };
            {
                let source_ml_source2_nominal_field14918 = match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value14919, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value14919.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field14920 = match context.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value14921, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value14921
                        }
                    };
                    {
                        let source_ml_source2_nominal_field14922 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value14923, _, _, _, _, _) => {
                                source_ml_source2_data_property_value14923
                            }
                        };
                        {
                            let source_ml_source2_nominal_field14924 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value14925, _, _, _, _) => {
                                    source_ml_source2_data_property_value14925.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field14926 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value14927, _, _, _) => {
                                        source_ml_source2_data_property_value14927
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field14928 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value14929, _, _) => {
                                            source_ml_source2_data_property_value14929.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field14930 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14931, _) => {
                                                source_ml_source2_data_property_value14931.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field14932 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14933) => {
                                                    source_ml_source2_data_property_value14933.clone()
                                                }
                                            };
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field14914.clone()), Box::new(source_ml_source2_nominal_field14916.clone()), source_ml_source2_nominal_field14918.clone(), source_ml_source2_nominal_field14920, source_ml_source2_nominal_field14922, error.clone(), source_ml_source2_nominal_field14926, source_ml_source2_nominal_field14928.clone(), source_ml_source2_nominal_field14930.clone(), source_ml_source2_nominal_field14932.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }))
}
