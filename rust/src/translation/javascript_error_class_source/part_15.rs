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
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14156, _) => {
                        let source_ml_source2_data_property_value14156 = *source_ml_source2_data_property_value14156;
                        source_ml_source2_data_property_value14156.clone()
                    }
                });
                {
                    let saved = self::append_java_script_call_source_instruction(match held.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14157, _) => {
                            let source_ml_source2_data_property_value14157 = *source_ml_source2_data_property_value14157;
                            source_ml_source2_data_property_value14157.clone()
                        }
                    }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14158) => {
                            source_ml_source2_data_property_value14158
                        }
                    }, match input.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14159) => {
                            source_ml_source2_data_property_value14159
                        }
                    }));
                    {
                        let key = self::lower_java_script_call_source_expression(iterator_key.clone(), bindings.clone(), saved.clone(), (depth + 1f64));
                        {
                            let held_key = self::allocate_java_script_call_source_register(match key.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14160, _) => {
                                    let source_ml_source2_data_property_value14160 = *source_ml_source2_data_property_value14160;
                                    source_ml_source2_data_property_value14160.clone()
                                }
                            });
                            {
                                let keyed = self::append_java_script_call_source_instruction(match held_key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14161, _) => {
                                        let source_ml_source2_data_property_value14161 = *source_ml_source2_data_property_value14161;
                                        source_ml_source2_data_property_value14161.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match held_key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14162) => {
                                        source_ml_source2_data_property_value14162
                                    }
                                }, match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14163) => {
                                        source_ml_source2_data_property_value14163
                                    }
                                }));
                                {
                                    let value = self::allocate_java_script_call_source_register(keyed.clone());
                                    {
                                        let done = self::allocate_java_script_call_source_register(match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14164, _) => {
                                                let source_ml_source2_data_property_value14164 = *source_ml_source2_data_property_value14164;
                                                source_ml_source2_data_property_value14164.clone()
                                            }
                                        });
                                        {
                                            let opened = self::append_java_script_call_source_instruction(match done.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14165, _) => {
                                                    let source_ml_source2_data_property_value14165 = *source_ml_source2_data_property_value14165;
                                                    source_ml_source2_data_property_value14165.clone()
                                                }
                                            }, {
                                                let source_ml_source2_constructor_field14167 = match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14166) => {
                                                        source_ml_source2_data_property_value14166
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_constructor_field14169 = match done.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14168) => {
                                                            source_ml_source2_data_property_value14168
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_constructor_field14171 = match held.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14170) => {
                                                                source_ml_source2_data_property_value14170
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field14173 = match held_key.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14172) => {
                                                                    source_ml_source2_data_property_value14172
                                                                }
                                                            };
                                                            self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field14167, source_ml_source2_constructor_field14169, source_ml_source2_constructor_field14171, source_ml_source2_constructor_field14173, slot, self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), realm.clone(), None::<(Option<bool>,)>)
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
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14355, _, _, _) => {
                        source_ml_source2_data_property_value14355
                    }
                }) == binding) }) }; ml_values190.iter().cloned().enumerate().any(|(ml_index190, ml_value190)| ml_callback190(ml_value190)) }) {
                    self::refuse_java_script_call_source(builder.clone(), &(binding_error))
                } else {
                    {
                        let scope = (((scope_bindings.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                        if ((({ let ml_reference_array192 = scope.clone(); let ml_reference_projection192 = ml_reference_array192.borrow().len() as f64; ml_reference_projection192 }) > (4096f64)) || { let ml_values193 = { let ml_reference_value194 = scope.borrow().clone(); ml_reference_value194 }; let ml_callback193 = { let scope = scope.clone(); let bindings = bindings.clone(); let binding = binding; std::rc::Rc::new(move |identity: f64, index: f64| -> bool { (((!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || ((self::ml_array::index_of(&{ let ml_reference_value195 = scope.borrow().clone(); ml_reference_value195 }, &identity, 0f64, false, |a, b| a == b)) != index)) || !({ let ml_values196 = { let ml_reference_value197 = bindings.borrow().clone(); ml_reference_value197 }; let ml_callback196 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14357, _, _, _) => {
                                source_ml_source2_data_property_value14357
                            }
                        }) == identity) }) }; ml_values196.iter().cloned().enumerate().any(|(ml_index196, ml_value196)| ml_callback196(ml_value196)) })) || (identity == binding)) }) }; ml_values193.iter().cloned().enumerate().any(|(ml_index193, ml_value193)| ml_callback193(ml_value193, (ml_index193 as f64))) }) {
                            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 99, 111, 112, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 117, 116, 101, 114, 32, 84, 68, 90, 32, 99, 101, 108, 108, 115, 32, 97, 110, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 100, 32, 118, 97, 108, 117, 101])))
                        } else {
                            {
                                let visible = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values198 = { let ml_reference_value199 = bindings.borrow().clone(); ml_reference_value199 }; let ml_callback198 = { let scope = scope.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { !(self::ml_array::includes(&{ let ml_reference_value200 = scope.borrow().clone(); ml_reference_value200 }, &match row.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14358, _, _, _) => {
                                        source_ml_source2_data_property_value14358
                                    }
                                }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values198.iter().cloned().enumerate().filter_map(|(ml_index198, ml_value198)| if ml_callback198(ml_value198.clone()) { Some(ml_value198) } else { None }).collect::<Vec<_>>() }));
                                {
                                    let scoped = self::push_java_script_call_source_control_scope(builder.clone(), target, &(self::ml_string::Text::from_units(vec![108, 111, 111, 112])));
                                    {
                                        let value = self::allocate_java_script_call_source_register(scoped.clone());
                                        {
                                            let done = self::allocate_java_script_call_source_register(match value.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14359, _) => {
                                                    let source_ml_source2_data_property_value14359 = *source_ml_source2_data_property_value14359;
                                                    source_ml_source2_data_property_value14359.clone()
                                                }
                                            });
                                            {
                                                let start = { let ml_reference_array201 = match match done.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14360, _) => {
                                                        let source_ml_source2_data_property_value14360 = *source_ml_source2_data_property_value14360;
                                                        source_ml_source2_data_property_value14360.clone()
                                                    }
                                                } {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14361, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value14361.clone()
                                                    }
                                                }; let ml_reference_projection201 = ml_reference_array201.borrow().len() as f64; ml_reference_projection201 };
                                                {
                                                    let stepped = self::append_java_script_call_source_instruction(match done.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14362, _) => {
                                                            let source_ml_source2_data_property_value14362 = *source_ml_source2_data_property_value14362;
                                                            source_ml_source2_data_property_value14362.clone()
                                                        }
                                                    }, {
                                                        let source_ml_source2_constructor_field14364 = match value.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14363) => {
                                                                source_ml_source2_data_property_value14363
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field14366 = match done.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14365) => {
                                                                    source_ml_source2_data_property_value14365
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_constructor_field14367 = 0f64;
                                                                {
                                                                    let source_ml_source2_constructor_field14368 = 0f64;
                                                                    self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field14364, source_ml_source2_constructor_field14366, source_ml_source2_constructor_field14367, source_ml_source2_constructor_field14368, slot, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), None::<(Option<bool>,)>)
                                                                }
                                                            }
                                                        }
                                                    });
                                                    {
                                                        let branch = { let ml_reference_array202 = match stepped.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14372, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value14372.clone()
                                                            }
                                                        }; let ml_reference_projection202 = ml_reference_array202.borrow().len() as f64; ml_reference_projection202 };
                                                        {
                                                            let test = self::append_java_script_call_source_instruction(stepped.clone(), self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14373) => {
                                                                    source_ml_source2_data_property_value14373
                                                                }
                                                            }, 0f64, (branch + 1f64)));
                                                            {
                                                                let body = self::lower_java_script_call_source_expression(source_body.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value14379 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                        let source_ml_source2_constructor_field14376 = match value.clone() {
                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14375) => {
                                                                                source_ml_source2_data_property_value14375
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(binding, source_ml_source2_constructor_field14376, mutable, None::<(Option<bool>,)>)
                                                                    }]));
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value203 = visible.borrow().clone(); ml_reference_value203 }), { let ml_reference_value204 = source_ml_source2_concat_value14379.borrow().clone(); ml_reference_value204 })
                                                                })), test.clone(), (depth + 1f64));
                                                                {
                                                                    let loop_ = self::append_java_script_call_source_instruction(match body.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14380, _) => {
                                                                            let source_ml_source2_data_property_value14380 = *source_ml_source2_data_property_value14380;
                                                                            source_ml_source2_data_property_value14380.clone()
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Jump(start));
                                                                    {
                                                                        let end = { let ml_reference_array205 = match loop_.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14381, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value14381.clone()
                                                                            }
                                                                        }; let ml_reference_projection205 = ml_reference_array205.borrow().len() as f64; ml_reference_projection205 };
                                                                        {
                                                                            let patched = {
                                                                                let source_ml_source2_nominal_spread14384 = loop_.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field14385 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14386, _, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value14386.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field14387 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value14388, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value14388
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field14389 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value14390, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value14390
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field14391 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value14392, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value14392
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field14393 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value14394, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value14394
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field14395 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value14396, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value14396.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field14397 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value14398, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value14398.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14399 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value14400, _, _) => {
                                                                                                                        source_ml_source2_data_property_value14400.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14401 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14402, _) => {
                                                                                                                            source_ml_source2_data_property_value14402.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14403 = match source_ml_source2_nominal_spread14384.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14404) => {
                                                                                                                                source_ml_source2_data_property_value14404.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override14405 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values206 = { let ml_reference_value207 = (match loop_.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14382, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value14382.clone()
                                                                                                                                }
                                                                                                                            }).borrow().clone(); ml_reference_value207 }; let ml_callback206 = { let branch = branch; let done = done.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                                                                                self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14383) => {
                                                                                                                                        source_ml_source2_data_property_value14383
                                                                                                                                    }
                                                                                                                                }, end, (branch + 1f64))
                                                                                                                            } else {
                                                                                                                                instruction.clone()
                                                                                                                            } }) }; ml_values206.iter().cloned().enumerate().map(|(ml_index206, ml_value206)| ml_callback206(ml_value206, (ml_index206 as f64))).collect::<Vec<_>>() }));
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override14405.clone(), source_ml_source2_nominal_field14387, source_ml_source2_nominal_field14389, source_ml_source2_nominal_field14391, source_ml_source2_nominal_field14393, source_ml_source2_nominal_field14395.clone(), source_ml_source2_nominal_field14397.clone(), source_ml_source2_nominal_field14399.clone(), source_ml_source2_nominal_field14401.clone(), source_ml_source2_nominal_field14403.clone())
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
                                        let source_ml_source2_s14587 = self::source_ml_source1_temporary_lower_java_script_call_source_binding_scope_loop73(bindings.clone(), current.clone(), entries.clone(), source_ml_source1_temporary_values72.clone(), source_ml_source1_temporary_index72.clone());
                                        match source_ml_source2_s14587.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Done(current_2) => {
                                                let current_2 = *current_2;
                                                {
                                                    let initialized = self::lower_java_script_call_source_expression(initializer.clone(), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value209 = bindings.borrow().clone(); ml_reference_value209 }), { let ml_reference_value210 = entries.borrow().clone(); ml_reference_value210 }))), current_2.clone(), (depth + 1f64));
                                                    if { let ml_values211 = { let ml_reference_value212 = entries.borrow().clone(); ml_reference_value212 }; let ml_callback211 = { let initialized = initialized.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { !(self::ml_array::includes(&{ let ml_reference_value213 = ((match match initialized.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14592, _) => {
                                                            let source_ml_source2_data_property_value14592 = *source_ml_source2_data_property_value14592;
                                                            source_ml_source2_data_property_value14592.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value14593, _, _) => {
                                                            ((source_ml_source2_data_property_value14593.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value213 }, &match row.clone() {
                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value14594, _, _, _) => {
                                                            source_ml_source2_data_property_value14594
                                                        }
                                                    }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values211.iter().cloned().enumerate().any(|(ml_index211, ml_value211)| ml_callback211(ml_value211)) } {
                                                        self::refuse_java_script_call_source(match initialized.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14595, _) => {
                                                                let source_ml_source2_data_property_value14595 = *source_ml_source2_data_property_value14595;
                                                                source_ml_source2_data_property_value14595.clone()
                                                            }
                                                        }, &(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 98, 105, 110, 100, 105, 110, 103, 32, 111, 117, 116, 112, 117, 116, 32, 109, 117, 115, 116, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 98, 111, 100, 121])))
                                                    } else {
                                                        self::lower_java_script_call_source_expression(body.clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value14602 = bindings.clone();
                                                            {
                                                                let source_ml_source2_concat_value14603 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values214 = { let ml_reference_value215 = entries.borrow().clone(); ml_reference_value215 }; let ml_callback214 = std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> self::JavaScriptCallSourceBinding { {
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
                                                                } }); ml_values214.iter().cloned().enumerate().map(|(ml_index214, ml_value214)| ml_callback214(ml_value214)).collect::<Vec<_>>() }));
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceBinding>::new(), { let ml_reference_value216 = source_ml_source2_concat_value14602.borrow().clone(); ml_reference_value216 }), { let ml_reference_value217 = source_ml_source2_concat_value14603.borrow().clone(); ml_reference_value217 })
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
                                                            let start = { let ml_reference_array218 = match match written.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value14795, _) => {
                                                                    let source_ml_source2_data_property_value14795 = *source_ml_source2_data_property_value14795;
                                                                    source_ml_source2_data_property_value14795.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14796, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value14796.clone()
                                                                }
                                                            }; let ml_reference_projection218 = ml_reference_array218.borrow().len() as f64; ml_reference_projection218 };
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
                                                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field14799, source_ml_source2_constructor_field14801, source_ml_source2_constructor_field14802, source_ml_source2_constructor_field14803, slot, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), None::<(Option<bool>,)>)
                                                                            }
                                                                        }
                                                                    }
                                                                });
                                                                {
                                                                    let branch = { let ml_reference_array219 = match stepped.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14807, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value14807.clone()
                                                                        }
                                                                    }; let ml_reference_projection219 = ml_reference_array219.borrow().len() as f64; ml_reference_projection219 };
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
                                                                                    }, self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match index.clone() {
                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14816) => {
                                                                                            source_ml_source2_data_property_value14816
                                                                                        }
                                                                                    }]))));
                                                                                    {
                                                                                        let loop_ = self::append_java_script_call_source_instruction(advanced.clone(), self::JavaScriptCallInstruction::Jump(start));
                                                                                        {
                                                                                            let end = { let ml_reference_array220 = match loop_.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14817, _, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value14817.clone()
                                                                                                }
                                                                                            }; let ml_reference_projection220 = ml_reference_array220.borrow().len() as f64; ml_reference_projection220 };
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
                                                                                                                                                    let source_ml_source2_nominal_override14843 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values221 = { let ml_reference_value222 = (match loop_.clone() {
                                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value14820, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                            source_ml_source2_data_property_value14820.clone()
                                                                                                                                                        }
                                                                                                                                                    }).borrow().clone(); ml_reference_value222 }; let ml_callback221 = { let branch = branch; let done = done.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, at: f64| -> self::JavaScriptCallInstruction { if (at == branch) {
                                                                                                                                                        self::JavaScriptCallInstruction::Branch(match done.clone() {
                                                                                                                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value14821) => {
                                                                                                                                                                source_ml_source2_data_property_value14821
                                                                                                                                                            }
                                                                                                                                                        }, end, (branch + 1f64))
                                                                                                                                                    } else {
                                                                                                                                                        instruction.clone()
                                                                                                                                                    } }) }; ml_values221.iter().cloned().enumerate().map(|(ml_index221, ml_value221)| ml_callback221(ml_value221, (ml_index221 as f64))).collect::<Vec<_>>() }));
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
        let source_ml_source2_nominal_field14846 = match context.clone() {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value14847, _, _, _, _, _, _, _, _, _) => {
                let source_ml_source2_data_property_value14847 = *source_ml_source2_data_property_value14847;
                source_ml_source2_data_property_value14847.clone()
            }
        };
        {
            let source_ml_source2_nominal_field14848 = match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14849, _, _, _, _, _, _, _, _) => {
                    let source_ml_source2_data_property_value14849 = *source_ml_source2_data_property_value14849;
                    source_ml_source2_data_property_value14849.clone()
                }
            };
            {
                let source_ml_source2_nominal_field14850 = match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value14851, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value14851.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field14852 = match context.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value14853, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value14853
                        }
                    };
                    {
                        let source_ml_source2_nominal_field14854 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value14855, _, _, _, _, _) => {
                                source_ml_source2_data_property_value14855
                            }
                        };
                        {
                            let source_ml_source2_nominal_field14856 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value14857, _, _, _, _) => {
                                    source_ml_source2_data_property_value14857.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field14858 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value14859, _, _, _) => {
                                        source_ml_source2_data_property_value14859
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field14860 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value14861, _, _) => {
                                            source_ml_source2_data_property_value14861.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field14862 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14863, _) => {
                                                source_ml_source2_data_property_value14863.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field14864 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14865) => {
                                                    source_ml_source2_data_property_value14865.clone()
                                                }
                                            };
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field14846.clone()), Box::new(source_ml_source2_nominal_field14848.clone()), source_ml_source2_nominal_field14850.clone(), source_ml_source2_nominal_field14852, source_ml_source2_nominal_field14854, error.clone(), source_ml_source2_nominal_field14858, source_ml_source2_nominal_field14860.clone(), source_ml_source2_nominal_field14862.clone(), source_ml_source2_nominal_field14864.clone())
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
