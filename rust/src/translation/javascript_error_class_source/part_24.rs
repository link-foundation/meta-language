// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptPatternBindingDeclarations;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceStatement;
use super::collect_java_script_pattern_binding_declarations;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_field_key;
use super::lower_java_script_error_expression;
use super::lower_java_script_lexical_for_of;
use super::lower_java_script_source_binding_pattern;
use super::lower_java_script_source_statements;
use super::lower_java_script_variable_for_of;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_lower_java_script_source_iteration_continuation_loop138;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_iteration_continuation(statement: self::JavaScriptSourceStatement, rest: self::JavaScriptErrorSourceExpression, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match statement.clone() {
        self::JavaScriptSourceStatement::SourceForOfStatement(target, binding, mutable, variable, _, statement_input, iterator_key, statement_body, statement_pattern) => {
            let statement_input = *statement_input;
            let iterator_key = *iterator_key;
            if {
                let source_ml_source2_comparison_operand17151 = ((variable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                match (source_ml_source2_comparison_operand17151.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            } {
                self::lower_java_script_variable_for_of(statement.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17153) => {
                        let source_ml_source2_data_property_value17153 = *source_ml_source2_data_property_value17153;
                        source_ml_source2_data_property_value17153.clone()
                    }
                }, match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17154, _) => {
                        let source_ml_source2_data_property_value17154 = *source_ml_source2_data_property_value17154;
                        source_ml_source2_data_property_value17154.clone()
                    }
                }, (depth + 1f64))
            } else {
                {
                    let pattern = ((statement_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                    {
                        let head = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            std::rc::Rc::new(std::cell::RefCell::new(vec![binding]))
                        } else {
                            std::rc::Rc::new(std::cell::RefCell::new({ let ml_values267 = { let ml_reference_value268 = (match self::collect_java_script_pattern_binding_declarations(pattern.clone().expect("a proven present value"), mutable, 0f64, std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))) {
                                self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value17155, _) => {
                                    source_ml_source2_data_property_value17155.clone()
                                }
                            }).borrow().clone(); ml_reference_value268 }; let ml_callback267 = std::rc::Rc::new(move |row: self::JavaScriptCallBindingDeclaration| -> f64 { match row.clone() {
                                self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value17156, _) => {
                                    source_ml_source2_data_property_value17156
                                }
                            } }); ml_values267.iter().cloned().enumerate().map(|(ml_index267, ml_value267)| ml_callback267(ml_value267)).collect::<Vec<_>>() }))
                        };
                        {
                            let owned = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values269 = { let ml_reference_value270 = head.borrow().clone(); ml_reference_value270 }; let ml_callback269 = { let rest = rest.clone(); std::rc::Rc::new(move |identity: f64| -> bool { { let ml_values271 = { let ml_reference_value272 = ((match match rest.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17157) => {
                                    let source_ml_source2_data_property_value17157 = *source_ml_source2_data_property_value17157;
                                    source_ml_source2_data_property_value17157.clone()
                                }
                            } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17158) => {
                                    ((source_ml_source2_data_property_value17158.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                }
                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value272 }; let ml_callback271 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17160, _, _, _, _) => {
                                    source_ml_source2_data_property_value17160
                                }
                            }) == identity) }) }; ml_values271.iter().cloned().enumerate().any(|(ml_index271, ml_value271)| ml_callback271(ml_value271)) } }) }; ml_values269.iter().cloned().enumerate().filter_map(|(ml_index269, ml_value269)| if ml_callback269(ml_value269.clone()) { Some(ml_value269) } else { None }).collect::<Vec<_>>() }));
                            if (({ let ml_reference_array273 = owned.clone(); let ml_reference_projection273 = ml_reference_array273.borrow().len() as f64; ml_reference_projection273 }) > (0f64)) {
                                self::lower_java_script_lexical_for_of(statement.clone(), match rest.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17161) => {
                                        let source_ml_source2_data_property_value17161 = *source_ml_source2_data_property_value17161;
                                        source_ml_source2_data_property_value17161.clone()
                                    }
                                }, match rest.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17162, _) => {
                                        let source_ml_source2_data_property_value17162 = *source_ml_source2_data_property_value17162;
                                        source_ml_source2_data_property_value17162.clone()
                                    }
                                }, owned.clone(), (depth + 1f64))
                            } else {
                                {
                                    let input = self::lower_java_script_error_expression(statement_input.clone(), match rest.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17163) => {
                                            let source_ml_source2_data_property_value17163 = *source_ml_source2_data_property_value17163;
                                            source_ml_source2_data_property_value17163.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let key = self::lower_java_script_error_expression(iterator_key.clone(), match input.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17164) => {
                                                let source_ml_source2_data_property_value17164 = *source_ml_source2_data_property_value17164;
                                                source_ml_source2_data_property_value17164.clone()
                                            }
                                        }, (depth + 1f64));
                                        {
                                            let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, match key.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17165) => {
                                                    let source_ml_source2_data_property_value17165 = *source_ml_source2_data_property_value17165;
                                                    source_ml_source2_data_property_value17165.clone()
                                                }
                                            }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                                            {
                                                let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    body.clone()
                                                } else {
                                                    self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17166) => {
                                                            let source_ml_source2_data_property_value17166 = *source_ml_source2_data_property_value17166;
                                                            source_ml_source2_data_property_value17166.clone()
                                                        }
                                                    }, match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17167, _) => {
                                                            let source_ml_source2_data_property_value17167 = *source_ml_source2_data_property_value17167;
                                                            source_ml_source2_data_property_value17167.clone()
                                                        }
                                                    }, mutable, 0f64, false)
                                                };
                                                {
                                                    let initialized_3 = if (match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && self::ml_array::includes(&{ let ml_reference_value274 = ((match match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17170) => {
                                                            let source_ml_source2_data_property_value17170 = *source_ml_source2_data_property_value17170;
                                                            source_ml_source2_data_property_value17170.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17171, _) => {
                                                            ((source_ml_source2_data_property_value17171.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value274 }, &binding, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                                                        {
                                                            let temporary = match match initialized.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17172) => {
                                                                    let source_ml_source2_data_property_value17172 = *source_ml_source2_data_property_value17172;
                                                                    source_ml_source2_data_property_value17172.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17173, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17173
                                                                }
                                                            };
                                                            {
                                                                let next = {
                                                                    let source_ml_source2_nominal_spread17175 = match initialized.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17174) => {
                                                                            let source_ml_source2_data_property_value17174 = *source_ml_source2_data_property_value17174;
                                                                            source_ml_source2_data_property_value17174.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17176 = match source_ml_source2_nominal_spread17175.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17177, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17177 = *source_ml_source2_data_property_value17177;
                                                                                source_ml_source2_data_property_value17177.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17178 = match source_ml_source2_nominal_spread17175.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17179, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17179 = *source_ml_source2_data_property_value17179;
                                                                                    source_ml_source2_data_property_value17179.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17180 = match source_ml_source2_nominal_spread17175.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17181, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value17181.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17182 = match source_ml_source2_nominal_spread17175.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17183, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value17183
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17184 = match source_ml_source2_nominal_spread17175.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17185, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value17185
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field17186 = match source_ml_source2_nominal_spread17175.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17187, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value17187.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field17188 = match source_ml_source2_nominal_spread17175.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17189, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value17189
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field17190 = match source_ml_source2_nominal_spread17175.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17191, _, _) => {
                                                                                                            source_ml_source2_data_property_value17191.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17192 = match source_ml_source2_nominal_spread17175.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17193, _) => {
                                                                                                                source_ml_source2_data_property_value17193.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field17194 = match source_ml_source2_nominal_spread17175.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17195) => {
                                                                                                                    source_ml_source2_data_property_value17195.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override17196 = (temporary + 1f64);
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17176.clone()), Box::new(source_ml_source2_nominal_field17178.clone()), source_ml_source2_nominal_field17180.clone(), source_ml_source2_nominal_override17196, source_ml_source2_nominal_field17184, source_ml_source2_nominal_field17186.clone(), source_ml_source2_nominal_field17188, source_ml_source2_nominal_field17190.clone(), source_ml_source2_nominal_field17192.clone(), source_ml_source2_nominal_field17194.clone())
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
                                                                    let cell = {
                                                                        let source_ml_source2_constructor_field17197 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                                                                        {
                                                                            let source_ml_source2_constructor_field17200 = match match next.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17198, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17198 = *source_ml_source2_data_property_value17198;
                                                                                    source_ml_source2_data_property_value17198.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17199, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17199.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_constructor_field17203 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                                    let source_ml_source2_constructor_field17201 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                                                                    {
                                                                                        let source_ml_source2_constructor_field17202 = self::JavaScriptCallSource::SourceBinding(binding);
                                                                                        self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field17201.clone()), Box::new(source_ml_source2_constructor_field17202.clone()), None::<(Option<bool>,)>)
                                                                                    }
                                                                                }]));
                                                                                self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field17197.clone()), source_ml_source2_constructor_field17203.clone(), source_ml_source2_constructor_field17200.clone())
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let initialized_2 = {
                                                                            let source_ml_source2_constructor_field17206 = self::construct_java_script_error_local_binding(next.clone(), temporary, self::JavaScriptCallSource::SourceAssignBinding(binding, Box::new(cell.clone())), match initialized.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17205, _) => {
                                                                                    let source_ml_source2_data_property_value17205 = *source_ml_source2_data_property_value17205;
                                                                                    source_ml_source2_data_property_value17205.clone()
                                                                                }
                                                                            }, false);
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field17206.clone()), Box::new(next.clone()))
                                                                        };
                                                                        initialized_2.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        initialized.clone()
                                                    };
                                                    {
                                                        let identity = match match initialized_3.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17207) => {
                                                                let source_ml_source2_data_property_value17207 = *source_ml_source2_data_property_value17207;
                                                                source_ml_source2_data_property_value17207.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17208, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17208
                                                            }
                                                        };
                                                        {
                                                            let next_2 = {
                                                                let source_ml_source2_nominal_spread17210 = match initialized_3.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17209) => {
                                                                        let source_ml_source2_data_property_value17209 = *source_ml_source2_data_property_value17209;
                                                                        source_ml_source2_data_property_value17209.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17211 = match source_ml_source2_nominal_spread17210.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17212, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value17212 = *source_ml_source2_data_property_value17212;
                                                                            source_ml_source2_data_property_value17212.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17213 = match source_ml_source2_nominal_spread17210.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17214, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17214 = *source_ml_source2_data_property_value17214;
                                                                                source_ml_source2_data_property_value17214.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17215 = match source_ml_source2_nominal_spread17210.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17216, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17216.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17217 = match source_ml_source2_nominal_spread17210.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17218, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value17218
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17219 = match source_ml_source2_nominal_spread17210.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17220, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value17220
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17221 = match source_ml_source2_nominal_spread17210.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17222, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value17222.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field17223 = match source_ml_source2_nominal_spread17210.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17224, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value17224
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field17225 = match source_ml_source2_nominal_spread17210.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17226, _, _) => {
                                                                                                        source_ml_source2_data_property_value17226.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field17227 = match source_ml_source2_nominal_spread17210.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17228, _) => {
                                                                                                            source_ml_source2_data_property_value17228.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17229 = match source_ml_source2_nominal_spread17210.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17230) => {
                                                                                                                source_ml_source2_data_property_value17230.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override17231 = (identity + 1f64);
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17211.clone()), Box::new(source_ml_source2_nominal_field17213.clone()), source_ml_source2_nominal_field17215.clone(), source_ml_source2_nominal_override17231, source_ml_source2_nominal_field17219, source_ml_source2_nominal_field17221.clone(), source_ml_source2_nominal_field17223, source_ml_source2_nominal_field17225.clone(), source_ml_source2_nominal_field17227.clone(), source_ml_source2_nominal_field17229.clone())
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
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next_2.clone(), identity, {
                                                                let source_ml_source2_constructor_field17237 = match input.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17236, _) => {
                                                                        let source_ml_source2_data_property_value17236 = *source_ml_source2_data_property_value17236;
                                                                        source_ml_source2_data_property_value17236.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_constructor_field17239 = match key.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17238, _) => {
                                                                            let source_ml_source2_data_property_value17238 = *source_ml_source2_data_property_value17238;
                                                                            source_ml_source2_data_property_value17238.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field17241 = match initialized_3.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17240, _) => {
                                                                                let source_ml_source2_data_property_value17240 = *source_ml_source2_data_property_value17240;
                                                                                source_ml_source2_data_property_value17240.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field17244 = match match next_2.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17242, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17242 = *source_ml_source2_data_property_value17242;
                                                                                    source_ml_source2_data_property_value17242.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17243, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17243.clone()
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSource::SourceForOf(target, target, binding, mutable, Box::new(source_ml_source2_constructor_field17237.clone()), Box::new(source_ml_source2_constructor_field17239.clone()), Box::new(source_ml_source2_constructor_field17241.clone()), source_ml_source2_constructor_field17244.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>)
                                                                        }
                                                                    }
                                                                }
                                                            }, match rest.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17245, _) => {
                                                                    let source_ml_source2_data_property_value17245 = *source_ml_source2_data_property_value17245;
                                                                    source_ml_source2_data_property_value17245.clone()
                                                                }
                                                            }, false)), Box::new(next_2.clone()))
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
        self::JavaScriptSourceStatement::SourceFinallyStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17246) => {
                    let source_ml_source2_data_property_value17246 = *source_ml_source2_data_property_value17246;
                    source_ml_source2_data_property_value17246.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceTryStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17247) => {
                    let source_ml_source2_data_property_value17247 = *source_ml_source2_data_property_value17247;
                    source_ml_source2_data_property_value17247.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceArrayBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17248) => {
                    let source_ml_source2_data_property_value17248 = *source_ml_source2_data_property_value17248;
                    source_ml_source2_data_property_value17248.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceObjectBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17249) => {
                    let source_ml_source2_data_property_value17249 = *source_ml_source2_data_property_value17249;
                    source_ml_source2_data_property_value17249.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceVariableStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17250) => {
                    let source_ml_source2_data_property_value17250 = *source_ml_source2_data_property_value17250;
                    source_ml_source2_data_property_value17250.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceConstStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17251) => {
                    let source_ml_source2_data_property_value17251 = *source_ml_source2_data_property_value17251;
                    source_ml_source2_data_property_value17251.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceMutableStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17252) => {
                    let source_ml_source2_data_property_value17252 = *source_ml_source2_data_property_value17252;
                    source_ml_source2_data_property_value17252.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceExpressionStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17253) => {
                    let source_ml_source2_data_property_value17253 = *source_ml_source2_data_property_value17253;
                    source_ml_source2_data_property_value17253.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17254) => {
                    let source_ml_source2_data_property_value17254 = *source_ml_source2_data_property_value17254;
                    source_ml_source2_data_property_value17254.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17255) => {
                    let source_ml_source2_data_property_value17255 = *source_ml_source2_data_property_value17255;
                    source_ml_source2_data_property_value17255.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceIfStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17256) => {
                    let source_ml_source2_data_property_value17256 = *source_ml_source2_data_property_value17256;
                    source_ml_source2_data_property_value17256.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceBlockStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17257) => {
                    let source_ml_source2_data_property_value17257 = *source_ml_source2_data_property_value17257;
                    source_ml_source2_data_property_value17257.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17258) => {
                    let source_ml_source2_data_property_value17258 = *source_ml_source2_data_property_value17258;
                    source_ml_source2_data_property_value17258.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceWhileStatement(statement_condition, statement_body_2, statement_target, statement_update, post_test, iteration_bindings) => {
            let statement_condition = *statement_condition;
            {
                let update_expression = ((statement_update.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorExpression>,))).0).clone();
                {
                    let target_2 = ((statement_target.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
                    {
                        let condition = self::lower_java_script_error_expression(statement_condition.clone(), match rest.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17259) => {
                                let source_ml_source2_data_property_value17259 = *source_ml_source2_data_property_value17259;
                                source_ml_source2_data_property_value17259.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let body_2 = self::lower_java_script_source_statements(statement_body_2.clone(), 0f64, match condition.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17260) => {
                                    let source_ml_source2_data_property_value17260 = *source_ml_source2_data_property_value17260;
                                    source_ml_source2_data_property_value17260.clone()
                                }
                            }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                            {
                                let update = if match (update_expression.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    body_2.clone()
                                } else {
                                    self::lower_java_script_error_expression(update_expression.clone().expect("a proven present value"), match body_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17261) => {
                                            let source_ml_source2_data_property_value17261 = *source_ml_source2_data_property_value17261;
                                            source_ml_source2_data_property_value17261.clone()
                                        }
                                    }, (depth + 1f64))
                                };
                                {
                                    let current = std::rc::Rc::new(std::cell::RefCell::new(match update.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17262) => {
                                            let source_ml_source2_data_property_value17262 = *source_ml_source2_data_property_value17262;
                                            source_ml_source2_data_property_value17262.clone()
                                        }
                                    }));
                                    {
                                        let iteration = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
                                        {
                                            let identities = (((iteration_bindings.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                            if (((({ let ml_reference_array275 = identities.clone(); let ml_reference_projection275 = ml_reference_array275.borrow().len() as f64; ml_reference_projection275 }) > (4096f64)) || { let ml_values276 = { let ml_reference_value277 = identities.borrow().clone(); ml_reference_value277 }; let ml_callback276 = { let current = current.clone(); let identities = identities.clone(); std::rc::Rc::new(move |identity_2: f64, index: f64| -> bool { (!(self::ml_array::includes(&{ let ml_reference_value278 = ((match { let ml_cell_value = current.borrow().clone(); ml_cell_value } {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17264, _) => {
                                                    ((source_ml_source2_data_property_value17264.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                }
                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value278 }, &identity_2, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) || ((self::ml_array::index_of(&{ let ml_reference_value279 = identities.borrow().clone(); ml_reference_value279 }, &identity_2, 0f64, false, |a, b| a == b)) != index)) }) }; ml_values276.iter().cloned().enumerate().any(|(ml_index276, ml_value276)| ml_callback276(ml_value276, (ml_index276 as f64))) }) || ((({ let ml_reference_array280 = identities.clone(); let ml_reference_projection280 = ml_reference_array280.borrow().len() as f64; ml_reference_projection280 }) > (0f64)) && {
                                                let source_ml_source2_comparison_operand17265 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                match (source_ml_source2_comparison_operand17265.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                            })) {
                                                self::refuse_java_script_error_source({ let ml_cell_value = current.borrow().clone(); ml_cell_value }, &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 99, 108, 111, 110, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 104, 97, 114, 101, 100, 32, 102, 111, 114, 45, 104, 101, 97, 100, 32, 98, 105, 110, 100, 105, 110, 103, 115])))
                                            } else {
                                                {
                                                    let at = std::rc::Rc::new(std::cell::RefCell::new(({ let ml_reference_array281 = identities.clone(); let ml_reference_projection281 = ml_reference_array281.borrow().len() as f64; ml_reference_projection281 } - 1f64)));
                                                    {
                                                        let iteration_2 = self::source_ml_source1_temporary_lower_java_script_source_iteration_continuation_loop138(current.clone(), iteration.clone(), identities.clone(), at.clone());
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new({
                                                            let source_ml_source1_temporary_constructor_field128 = match condition.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17267, _) => {
                                                                    let source_ml_source2_data_property_value17267 = *source_ml_source2_data_property_value17267;
                                                                    source_ml_source2_data_property_value17267.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_temporary_constructor_field129 = match body_2.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17268, _) => {
                                                                        let source_ml_source2_data_property_value17268 = *source_ml_source2_data_property_value17268;
                                                                        source_ml_source2_data_property_value17268.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_temporary_constructor_field130 = match rest.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17269, _) => {
                                                                            let source_ml_source2_data_property_value17269 = *source_ml_source2_data_property_value17269;
                                                                            source_ml_source2_data_property_value17269.clone()
                                                                        }
                                                                    };
                                                                    if {
                                                                        let source_ml_source2_comparison_operand17271 = None::<()>;
                                                                        !(match (target_2.clone(), source_ml_source2_comparison_operand17271.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                    } {
                                                                        {
                                                                            let source_ml_source1_temporary_constructor_field131 = target_2.clone().expect("a proven present value");
                                                                            if !(match (update_expression.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                {
                                                                                    let source_ml_source1_temporary_constructor_field132 = match update.clone() {
                                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17272, _) => {
                                                                                            let source_ml_source2_data_property_value17272 = *source_ml_source2_data_property_value17272;
                                                                                            source_ml_source2_data_property_value17272.clone()
                                                                                        }
                                                                                    };
                                                                                    if {
                                                                                        let source_ml_source2_comparison_operand17273 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                                                        match (source_ml_source2_comparison_operand17273.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                    } {
                                                                                        if { let ml_value = { let ml_reference_array283 = identities.clone(); let ml_reference_projection283 = ml_reference_array283.borrow().len() as f64; ml_reference_projection283 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17278 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field17279 = Some(source_ml_source1_temporary_constructor_field132.clone());
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field17280 = Some(true);
                                                                                                        self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17278.clone(),)), Some((source_ml_source2_constructor_field17279.clone(),)), Some((source_ml_source2_constructor_field17280.clone(),)), Some((iteration_2.clone(),)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17285 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field17286 = Some(source_ml_source1_temporary_constructor_field132.clone());
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field17287 = Some(true);
                                                                                                        self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17285.clone(),)), Some((source_ml_source2_constructor_field17286.clone(),)), Some((source_ml_source2_constructor_field17287.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        if { let ml_value = { let ml_reference_array285 = identities.clone(); let ml_reference_projection285 = ml_reference_array285.borrow().len() as f64; ml_reference_projection285 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17291 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field17292 = Some(source_ml_source1_temporary_constructor_field132.clone());
                                                                                                    self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17291.clone(),)), Some((source_ml_source2_constructor_field17292.clone(),)), None::<(Option<bool>,)>, Some((iteration_2.clone(),)))
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17297 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field17298 = Some(source_ml_source1_temporary_constructor_field132.clone());
                                                                                                    self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17297.clone(),)), Some((source_ml_source2_constructor_field17298.clone(),)), None::<(Option<bool>,)>, None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if {
                                                                                    let source_ml_source2_comparison_operand17299 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                                                    match (source_ml_source2_comparison_operand17299.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                } {
                                                                                    if { let ml_value = { let ml_reference_array287 = identities.clone(); let ml_reference_projection287 = ml_reference_array287.borrow().len() as f64; ml_reference_projection287 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17304 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17305 = Some(true);
                                                                                                self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17304.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>, Some((source_ml_source2_constructor_field17305.clone(),)), Some((iteration_2.clone(),)))
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17310 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17311 = Some(true);
                                                                                                self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17310.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>, Some((source_ml_source2_constructor_field17311.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    if { let ml_value = { let ml_reference_array289 = identities.clone(); let ml_reference_projection289 = ml_reference_array289.borrow().len() as f64; ml_reference_projection289 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17315 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                            self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17315.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>, None::<(Option<bool>,)>, Some((iteration_2.clone(),)))
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17320 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                            self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17320.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if !(match (update_expression.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            {
                                                                                let source_ml_source1_temporary_constructor_field135 = match update.clone() {
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17321, _) => {
                                                                                        let source_ml_source2_data_property_value17321 = *source_ml_source2_data_property_value17321;
                                                                                        source_ml_source2_data_property_value17321.clone()
                                                                                    }
                                                                                };
                                                                                if {
                                                                                    let source_ml_source2_comparison_operand17322 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                                                    match (source_ml_source2_comparison_operand17322.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                } {
                                                                                    if { let ml_value = { let ml_reference_array291 = identities.clone(); let ml_reference_projection291 = ml_reference_array291.borrow().len() as f64; ml_reference_projection291 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17327 = Some(source_ml_source1_temporary_constructor_field135.clone());
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17328 = Some(true);
                                                                                                self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, Some((source_ml_source2_constructor_field17327.clone(),)), Some((source_ml_source2_constructor_field17328.clone(),)), Some((iteration_2.clone(),)))
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17333 = Some(source_ml_source1_temporary_constructor_field135.clone());
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17334 = Some(true);
                                                                                                self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, Some((source_ml_source2_constructor_field17333.clone(),)), Some((source_ml_source2_constructor_field17334.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    if { let ml_value = { let ml_reference_array293 = identities.clone(); let ml_reference_projection293 = ml_reference_array293.borrow().len() as f64; ml_reference_projection293 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17338 = Some(source_ml_source1_temporary_constructor_field135.clone());
                                                                                            self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, Some((source_ml_source2_constructor_field17338.clone(),)), None::<(Option<bool>,)>, Some((iteration_2.clone(),)))
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17343 = Some(source_ml_source1_temporary_constructor_field135.clone());
                                                                                            self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, Some((source_ml_source2_constructor_field17343.clone(),)), None::<(Option<bool>,)>, None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if {
                                                                                let source_ml_source2_comparison_operand17344 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                                                match (source_ml_source2_comparison_operand17344.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                            } {
                                                                                if { let ml_value = { let ml_reference_array295 = identities.clone(); let ml_reference_projection295 = ml_reference_array295.borrow().len() as f64; ml_reference_projection295 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                    {
                                                                                        let source_ml_source2_constructor_field17349 = Some(true);
                                                                                        self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, None::<(Option<self::JavaScriptCallSource>,)>, Some((source_ml_source2_constructor_field17349.clone(),)), Some((iteration_2.clone(),)))
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_constructor_field17354 = Some(true);
                                                                                        self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, None::<(Option<self::JavaScriptCallSource>,)>, Some((source_ml_source2_constructor_field17354.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if { let ml_value = { let ml_reference_array297 = identities.clone(); let ml_reference_projection297 = ml_reference_array297.borrow().len() as f64; ml_reference_projection297 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                    self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, None::<(Option<self::JavaScriptCallSource>,)>, None::<(Option<bool>,)>, Some((iteration_2.clone(),)))
                                                                                } else {
                                                                                    self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, None::<(Option<self::JavaScriptCallSource>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }), Box::new({ let ml_cell_value = current.borrow().clone(); ml_cell_value }))
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
        self::JavaScriptSourceStatement::SourceControlBlockStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17362) => {
                    let source_ml_source2_data_property_value17362 = *source_ml_source2_data_property_value17362;
                    source_ml_source2_data_property_value17362.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17363) => {
                    let source_ml_source2_data_property_value17363 = *source_ml_source2_data_property_value17363;
                    source_ml_source2_data_property_value17363.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceSwitchStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17364) => {
                    let source_ml_source2_data_property_value17364 = *source_ml_source2_data_property_value17364;
                    source_ml_source2_data_property_value17364.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
    }
}
