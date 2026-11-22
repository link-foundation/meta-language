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
                let source_ml_source2_comparison_operand17053 = ((variable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                match (source_ml_source2_comparison_operand17053.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            } {
                self::lower_java_script_variable_for_of(statement.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17055) => {
                        let source_ml_source2_data_property_value17055 = *source_ml_source2_data_property_value17055;
                        source_ml_source2_data_property_value17055.clone()
                    }
                }, match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17056, _) => {
                        let source_ml_source2_data_property_value17056 = *source_ml_source2_data_property_value17056;
                        source_ml_source2_data_property_value17056.clone()
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
                                self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value17057, _) => {
                                    source_ml_source2_data_property_value17057.clone()
                                }
                            }).borrow().clone(); ml_reference_value268 }; let ml_callback267 = std::rc::Rc::new(move |row: self::JavaScriptCallBindingDeclaration| -> f64 { match row.clone() {
                                self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value17058, _) => {
                                    source_ml_source2_data_property_value17058
                                }
                            } }); ml_values267.iter().cloned().enumerate().map(|(ml_index267, ml_value267)| ml_callback267(ml_value267)).collect::<Vec<_>>() }))
                        };
                        {
                            let owned = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values269 = { let ml_reference_value270 = head.borrow().clone(); ml_reference_value270 }; let ml_callback269 = { let rest = rest.clone(); std::rc::Rc::new(move |identity: f64| -> bool { { let ml_values271 = { let ml_reference_value272 = ((match match rest.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17059) => {
                                    let source_ml_source2_data_property_value17059 = *source_ml_source2_data_property_value17059;
                                    source_ml_source2_data_property_value17059.clone()
                                }
                            } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17060) => {
                                    ((source_ml_source2_data_property_value17060.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                }
                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value272 }; let ml_callback271 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17062, _, _, _, _) => {
                                    source_ml_source2_data_property_value17062
                                }
                            }) == identity) }) }; ml_values271.iter().cloned().enumerate().any(|(ml_index271, ml_value271)| ml_callback271(ml_value271)) } }) }; ml_values269.iter().cloned().enumerate().filter_map(|(ml_index269, ml_value269)| if ml_callback269(ml_value269.clone()) { Some(ml_value269) } else { None }).collect::<Vec<_>>() }));
                            if (({ let ml_reference_array273 = owned.clone(); let ml_reference_projection273 = ml_reference_array273.borrow().len() as f64; ml_reference_projection273 }) > (0f64)) {
                                self::lower_java_script_lexical_for_of(statement.clone(), match rest.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17063) => {
                                        let source_ml_source2_data_property_value17063 = *source_ml_source2_data_property_value17063;
                                        source_ml_source2_data_property_value17063.clone()
                                    }
                                }, match rest.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17064, _) => {
                                        let source_ml_source2_data_property_value17064 = *source_ml_source2_data_property_value17064;
                                        source_ml_source2_data_property_value17064.clone()
                                    }
                                }, owned.clone(), (depth + 1f64))
                            } else {
                                {
                                    let input = self::lower_java_script_error_expression(statement_input.clone(), match rest.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17065) => {
                                            let source_ml_source2_data_property_value17065 = *source_ml_source2_data_property_value17065;
                                            source_ml_source2_data_property_value17065.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let key = self::lower_java_script_error_expression(iterator_key.clone(), match input.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17066) => {
                                                let source_ml_source2_data_property_value17066 = *source_ml_source2_data_property_value17066;
                                                source_ml_source2_data_property_value17066.clone()
                                            }
                                        }, (depth + 1f64));
                                        {
                                            let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, match key.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17067) => {
                                                    let source_ml_source2_data_property_value17067 = *source_ml_source2_data_property_value17067;
                                                    source_ml_source2_data_property_value17067.clone()
                                                }
                                            }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                                            {
                                                let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    body.clone()
                                                } else {
                                                    self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17068) => {
                                                            let source_ml_source2_data_property_value17068 = *source_ml_source2_data_property_value17068;
                                                            source_ml_source2_data_property_value17068.clone()
                                                        }
                                                    }, match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17069, _) => {
                                                            let source_ml_source2_data_property_value17069 = *source_ml_source2_data_property_value17069;
                                                            source_ml_source2_data_property_value17069.clone()
                                                        }
                                                    }, mutable, 0f64, false)
                                                };
                                                {
                                                    let initialized_3 = if (match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && self::ml_array::includes(&{ let ml_reference_value274 = ((match match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17072) => {
                                                            let source_ml_source2_data_property_value17072 = *source_ml_source2_data_property_value17072;
                                                            source_ml_source2_data_property_value17072.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17073, _) => {
                                                            ((source_ml_source2_data_property_value17073.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value274 }, &binding, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                                                        {
                                                            let temporary = match match initialized.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17074) => {
                                                                    let source_ml_source2_data_property_value17074 = *source_ml_source2_data_property_value17074;
                                                                    source_ml_source2_data_property_value17074.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17075, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17075
                                                                }
                                                            };
                                                            {
                                                                let next = {
                                                                    let source_ml_source2_nominal_spread17077 = match initialized.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17076) => {
                                                                            let source_ml_source2_data_property_value17076 = *source_ml_source2_data_property_value17076;
                                                                            source_ml_source2_data_property_value17076.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17078 = match source_ml_source2_nominal_spread17077.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17079, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17079 = *source_ml_source2_data_property_value17079;
                                                                                source_ml_source2_data_property_value17079.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17080 = match source_ml_source2_nominal_spread17077.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17081, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17081 = *source_ml_source2_data_property_value17081;
                                                                                    source_ml_source2_data_property_value17081.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17082 = match source_ml_source2_nominal_spread17077.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17083, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value17083.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17084 = match source_ml_source2_nominal_spread17077.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17085, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value17085
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17086 = match source_ml_source2_nominal_spread17077.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17087, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value17087
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field17088 = match source_ml_source2_nominal_spread17077.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17089, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value17089.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field17090 = match source_ml_source2_nominal_spread17077.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17091, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value17091
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field17092 = match source_ml_source2_nominal_spread17077.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17093, _, _) => {
                                                                                                            source_ml_source2_data_property_value17093.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17094 = match source_ml_source2_nominal_spread17077.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17095, _) => {
                                                                                                                source_ml_source2_data_property_value17095.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field17096 = match source_ml_source2_nominal_spread17077.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17097) => {
                                                                                                                    source_ml_source2_data_property_value17097.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override17098 = (temporary + 1f64);
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17078.clone()), Box::new(source_ml_source2_nominal_field17080.clone()), source_ml_source2_nominal_field17082.clone(), source_ml_source2_nominal_override17098, source_ml_source2_nominal_field17086, source_ml_source2_nominal_field17088.clone(), source_ml_source2_nominal_field17090, source_ml_source2_nominal_field17092.clone(), source_ml_source2_nominal_field17094.clone(), source_ml_source2_nominal_field17096.clone())
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
                                                                        let source_ml_source2_constructor_field17099 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                                                                        {
                                                                            let source_ml_source2_constructor_field17102 = match match next.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17100, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17100 = *source_ml_source2_data_property_value17100;
                                                                                    source_ml_source2_data_property_value17100.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17101, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17101.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_constructor_field17105 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                                    let source_ml_source2_constructor_field17103 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                                                                    {
                                                                                        let source_ml_source2_constructor_field17104 = self::JavaScriptCallSource::SourceBinding(binding);
                                                                                        self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field17103.clone()), Box::new(source_ml_source2_constructor_field17104.clone()), None::<(Option<bool>,)>)
                                                                                    }
                                                                                }]));
                                                                                self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field17099.clone()), source_ml_source2_constructor_field17105.clone(), source_ml_source2_constructor_field17102.clone())
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let initialized_2 = {
                                                                            let source_ml_source2_constructor_field17108 = self::construct_java_script_error_local_binding(next.clone(), temporary, self::JavaScriptCallSource::SourceAssignBinding(binding, Box::new(cell.clone())), match initialized.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17107, _) => {
                                                                                    let source_ml_source2_data_property_value17107 = *source_ml_source2_data_property_value17107;
                                                                                    source_ml_source2_data_property_value17107.clone()
                                                                                }
                                                                            }, false);
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field17108.clone()), Box::new(next.clone()))
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
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17109) => {
                                                                let source_ml_source2_data_property_value17109 = *source_ml_source2_data_property_value17109;
                                                                source_ml_source2_data_property_value17109.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17110, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17110
                                                            }
                                                        };
                                                        {
                                                            let next_2 = {
                                                                let source_ml_source2_nominal_spread17112 = match initialized_3.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17111) => {
                                                                        let source_ml_source2_data_property_value17111 = *source_ml_source2_data_property_value17111;
                                                                        source_ml_source2_data_property_value17111.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17113 = match source_ml_source2_nominal_spread17112.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17114, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value17114 = *source_ml_source2_data_property_value17114;
                                                                            source_ml_source2_data_property_value17114.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17115 = match source_ml_source2_nominal_spread17112.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17116, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17116 = *source_ml_source2_data_property_value17116;
                                                                                source_ml_source2_data_property_value17116.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17117 = match source_ml_source2_nominal_spread17112.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17118, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17118.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17119 = match source_ml_source2_nominal_spread17112.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17120, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value17120
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17121 = match source_ml_source2_nominal_spread17112.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17122, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value17122
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17123 = match source_ml_source2_nominal_spread17112.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17124, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value17124.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field17125 = match source_ml_source2_nominal_spread17112.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17126, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value17126
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field17127 = match source_ml_source2_nominal_spread17112.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17128, _, _) => {
                                                                                                        source_ml_source2_data_property_value17128.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field17129 = match source_ml_source2_nominal_spread17112.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17130, _) => {
                                                                                                            source_ml_source2_data_property_value17130.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17131 = match source_ml_source2_nominal_spread17112.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17132) => {
                                                                                                                source_ml_source2_data_property_value17132.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override17133 = (identity + 1f64);
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17113.clone()), Box::new(source_ml_source2_nominal_field17115.clone()), source_ml_source2_nominal_field17117.clone(), source_ml_source2_nominal_override17133, source_ml_source2_nominal_field17121, source_ml_source2_nominal_field17123.clone(), source_ml_source2_nominal_field17125, source_ml_source2_nominal_field17127.clone(), source_ml_source2_nominal_field17129.clone(), source_ml_source2_nominal_field17131.clone())
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
                                                                let source_ml_source2_constructor_field17139 = match input.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17138, _) => {
                                                                        let source_ml_source2_data_property_value17138 = *source_ml_source2_data_property_value17138;
                                                                        source_ml_source2_data_property_value17138.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_constructor_field17141 = match key.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17140, _) => {
                                                                            let source_ml_source2_data_property_value17140 = *source_ml_source2_data_property_value17140;
                                                                            source_ml_source2_data_property_value17140.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field17143 = match initialized_3.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17142, _) => {
                                                                                let source_ml_source2_data_property_value17142 = *source_ml_source2_data_property_value17142;
                                                                                source_ml_source2_data_property_value17142.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field17146 = match match next_2.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17144, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17144 = *source_ml_source2_data_property_value17144;
                                                                                    source_ml_source2_data_property_value17144.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17145, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17145.clone()
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSource::SourceForOf(target, target, binding, mutable, Box::new(source_ml_source2_constructor_field17139.clone()), Box::new(source_ml_source2_constructor_field17141.clone()), Box::new(source_ml_source2_constructor_field17143.clone()), source_ml_source2_constructor_field17146.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>)
                                                                        }
                                                                    }
                                                                }
                                                            }, match rest.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17147, _) => {
                                                                    let source_ml_source2_data_property_value17147 = *source_ml_source2_data_property_value17147;
                                                                    source_ml_source2_data_property_value17147.clone()
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
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17148) => {
                    let source_ml_source2_data_property_value17148 = *source_ml_source2_data_property_value17148;
                    source_ml_source2_data_property_value17148.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceTryStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17149) => {
                    let source_ml_source2_data_property_value17149 = *source_ml_source2_data_property_value17149;
                    source_ml_source2_data_property_value17149.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceArrayBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17150) => {
                    let source_ml_source2_data_property_value17150 = *source_ml_source2_data_property_value17150;
                    source_ml_source2_data_property_value17150.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceObjectBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17151) => {
                    let source_ml_source2_data_property_value17151 = *source_ml_source2_data_property_value17151;
                    source_ml_source2_data_property_value17151.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceVariableStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17152) => {
                    let source_ml_source2_data_property_value17152 = *source_ml_source2_data_property_value17152;
                    source_ml_source2_data_property_value17152.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceConstStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17153) => {
                    let source_ml_source2_data_property_value17153 = *source_ml_source2_data_property_value17153;
                    source_ml_source2_data_property_value17153.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceMutableStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17154) => {
                    let source_ml_source2_data_property_value17154 = *source_ml_source2_data_property_value17154;
                    source_ml_source2_data_property_value17154.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceExpressionStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17155) => {
                    let source_ml_source2_data_property_value17155 = *source_ml_source2_data_property_value17155;
                    source_ml_source2_data_property_value17155.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17156) => {
                    let source_ml_source2_data_property_value17156 = *source_ml_source2_data_property_value17156;
                    source_ml_source2_data_property_value17156.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17157) => {
                    let source_ml_source2_data_property_value17157 = *source_ml_source2_data_property_value17157;
                    source_ml_source2_data_property_value17157.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceIfStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17158) => {
                    let source_ml_source2_data_property_value17158 = *source_ml_source2_data_property_value17158;
                    source_ml_source2_data_property_value17158.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceBlockStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17159) => {
                    let source_ml_source2_data_property_value17159 = *source_ml_source2_data_property_value17159;
                    source_ml_source2_data_property_value17159.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17160) => {
                    let source_ml_source2_data_property_value17160 = *source_ml_source2_data_property_value17160;
                    source_ml_source2_data_property_value17160.clone()
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
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17161) => {
                                let source_ml_source2_data_property_value17161 = *source_ml_source2_data_property_value17161;
                                source_ml_source2_data_property_value17161.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let body_2 = self::lower_java_script_source_statements(statement_body_2.clone(), 0f64, match condition.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17162) => {
                                    let source_ml_source2_data_property_value17162 = *source_ml_source2_data_property_value17162;
                                    source_ml_source2_data_property_value17162.clone()
                                }
                            }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                            {
                                let update = if match (update_expression.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    body_2.clone()
                                } else {
                                    self::lower_java_script_error_expression(update_expression.clone().expect("a proven present value"), match body_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17163) => {
                                            let source_ml_source2_data_property_value17163 = *source_ml_source2_data_property_value17163;
                                            source_ml_source2_data_property_value17163.clone()
                                        }
                                    }, (depth + 1f64))
                                };
                                {
                                    let current = std::rc::Rc::new(std::cell::RefCell::new(match update.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17164) => {
                                            let source_ml_source2_data_property_value17164 = *source_ml_source2_data_property_value17164;
                                            source_ml_source2_data_property_value17164.clone()
                                        }
                                    }));
                                    {
                                        let iteration = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
                                        {
                                            let identities = (((iteration_bindings.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                            if (((({ let ml_reference_array275 = identities.clone(); let ml_reference_projection275 = ml_reference_array275.borrow().len() as f64; ml_reference_projection275 }) > (4096f64)) || { let ml_values276 = { let ml_reference_value277 = identities.borrow().clone(); ml_reference_value277 }; let ml_callback276 = { let current = current.clone(); let identities = identities.clone(); std::rc::Rc::new(move |identity_2: f64, index: f64| -> bool { (!(self::ml_array::includes(&{ let ml_reference_value278 = ((match { let ml_cell_value = current.borrow().clone(); ml_cell_value } {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17166, _) => {
                                                    ((source_ml_source2_data_property_value17166.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                }
                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value278 }, &identity_2, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) || ((self::ml_array::index_of(&{ let ml_reference_value279 = identities.borrow().clone(); ml_reference_value279 }, &identity_2, 0f64, false, |a, b| a == b)) != index)) }) }; ml_values276.iter().cloned().enumerate().any(|(ml_index276, ml_value276)| ml_callback276(ml_value276, (ml_index276 as f64))) }) || ((({ let ml_reference_array280 = identities.clone(); let ml_reference_projection280 = ml_reference_array280.borrow().len() as f64; ml_reference_projection280 }) > (0f64)) && {
                                                let source_ml_source2_comparison_operand17167 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                match (source_ml_source2_comparison_operand17167.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                            })) {
                                                self::refuse_java_script_error_source({ let ml_cell_value = current.borrow().clone(); ml_cell_value }, &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 99, 108, 111, 110, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 104, 97, 114, 101, 100, 32, 102, 111, 114, 45, 104, 101, 97, 100, 32, 98, 105, 110, 100, 105, 110, 103, 115])))
                                            } else {
                                                {
                                                    let at = std::rc::Rc::new(std::cell::RefCell::new(({ let ml_reference_array281 = identities.clone(); let ml_reference_projection281 = ml_reference_array281.borrow().len() as f64; ml_reference_projection281 } - 1f64)));
                                                    {
                                                        let iteration_2 = self::source_ml_source1_temporary_lower_java_script_source_iteration_continuation_loop138(current.clone(), iteration.clone(), identities.clone(), at.clone());
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new({
                                                            let source_ml_source1_temporary_constructor_field128 = match condition.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17169, _) => {
                                                                    let source_ml_source2_data_property_value17169 = *source_ml_source2_data_property_value17169;
                                                                    source_ml_source2_data_property_value17169.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_temporary_constructor_field129 = match body_2.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17170, _) => {
                                                                        let source_ml_source2_data_property_value17170 = *source_ml_source2_data_property_value17170;
                                                                        source_ml_source2_data_property_value17170.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_temporary_constructor_field130 = match rest.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17171, _) => {
                                                                            let source_ml_source2_data_property_value17171 = *source_ml_source2_data_property_value17171;
                                                                            source_ml_source2_data_property_value17171.clone()
                                                                        }
                                                                    };
                                                                    if {
                                                                        let source_ml_source2_comparison_operand17173 = None::<()>;
                                                                        !(match (target_2.clone(), source_ml_source2_comparison_operand17173.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                    } {
                                                                        {
                                                                            let source_ml_source1_temporary_constructor_field131 = target_2.clone().expect("a proven present value");
                                                                            if !(match (update_expression.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                {
                                                                                    let source_ml_source1_temporary_constructor_field132 = match update.clone() {
                                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17174, _) => {
                                                                                            let source_ml_source2_data_property_value17174 = *source_ml_source2_data_property_value17174;
                                                                                            source_ml_source2_data_property_value17174.clone()
                                                                                        }
                                                                                    };
                                                                                    if {
                                                                                        let source_ml_source2_comparison_operand17175 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                                                        match (source_ml_source2_comparison_operand17175.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                    } {
                                                                                        if { let ml_value = { let ml_reference_array283 = identities.clone(); let ml_reference_projection283 = ml_reference_array283.borrow().len() as f64; ml_reference_projection283 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17180 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field17181 = Some(source_ml_source1_temporary_constructor_field132.clone());
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field17182 = Some(true);
                                                                                                        self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17180.clone(),)), Some((source_ml_source2_constructor_field17181.clone(),)), Some((source_ml_source2_constructor_field17182.clone(),)), Some((iteration_2.clone(),)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17187 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field17188 = Some(source_ml_source1_temporary_constructor_field132.clone());
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field17189 = Some(true);
                                                                                                        self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17187.clone(),)), Some((source_ml_source2_constructor_field17188.clone(),)), Some((source_ml_source2_constructor_field17189.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        if { let ml_value = { let ml_reference_array285 = identities.clone(); let ml_reference_projection285 = ml_reference_array285.borrow().len() as f64; ml_reference_projection285 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17193 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field17194 = Some(source_ml_source1_temporary_constructor_field132.clone());
                                                                                                    self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17193.clone(),)), Some((source_ml_source2_constructor_field17194.clone(),)), None::<(Option<bool>,)>, Some((iteration_2.clone(),)))
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17199 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field17200 = Some(source_ml_source1_temporary_constructor_field132.clone());
                                                                                                    self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17199.clone(),)), Some((source_ml_source2_constructor_field17200.clone(),)), None::<(Option<bool>,)>, None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if {
                                                                                    let source_ml_source2_comparison_operand17201 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                                                    match (source_ml_source2_comparison_operand17201.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                } {
                                                                                    if { let ml_value = { let ml_reference_array287 = identities.clone(); let ml_reference_projection287 = ml_reference_array287.borrow().len() as f64; ml_reference_projection287 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17206 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17207 = Some(true);
                                                                                                self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17206.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>, Some((source_ml_source2_constructor_field17207.clone(),)), Some((iteration_2.clone(),)))
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17212 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17213 = Some(true);
                                                                                                self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17212.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>, Some((source_ml_source2_constructor_field17213.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    if { let ml_value = { let ml_reference_array289 = identities.clone(); let ml_reference_projection289 = ml_reference_array289.borrow().len() as f64; ml_reference_projection289 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17217 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                            self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17217.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>, None::<(Option<bool>,)>, Some((iteration_2.clone(),)))
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17222 = Some(source_ml_source1_temporary_constructor_field131);
                                                                                            self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), Some((source_ml_source2_constructor_field17222.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if !(match (update_expression.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            {
                                                                                let source_ml_source1_temporary_constructor_field135 = match update.clone() {
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17223, _) => {
                                                                                        let source_ml_source2_data_property_value17223 = *source_ml_source2_data_property_value17223;
                                                                                        source_ml_source2_data_property_value17223.clone()
                                                                                    }
                                                                                };
                                                                                if {
                                                                                    let source_ml_source2_comparison_operand17224 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                                                    match (source_ml_source2_comparison_operand17224.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                } {
                                                                                    if { let ml_value = { let ml_reference_array291 = identities.clone(); let ml_reference_projection291 = ml_reference_array291.borrow().len() as f64; ml_reference_projection291 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17229 = Some(source_ml_source1_temporary_constructor_field135.clone());
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17230 = Some(true);
                                                                                                self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, Some((source_ml_source2_constructor_field17229.clone(),)), Some((source_ml_source2_constructor_field17230.clone(),)), Some((iteration_2.clone(),)))
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17235 = Some(source_ml_source1_temporary_constructor_field135.clone());
                                                                                            {
                                                                                                let source_ml_source2_constructor_field17236 = Some(true);
                                                                                                self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, Some((source_ml_source2_constructor_field17235.clone(),)), Some((source_ml_source2_constructor_field17236.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    if { let ml_value = { let ml_reference_array293 = identities.clone(); let ml_reference_projection293 = ml_reference_array293.borrow().len() as f64; ml_reference_projection293 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17240 = Some(source_ml_source1_temporary_constructor_field135.clone());
                                                                                            self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, Some((source_ml_source2_constructor_field17240.clone(),)), None::<(Option<bool>,)>, Some((iteration_2.clone(),)))
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field17245 = Some(source_ml_source1_temporary_constructor_field135.clone());
                                                                                            self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, Some((source_ml_source2_constructor_field17245.clone(),)), None::<(Option<bool>,)>, None::<(Option<self::JavaScriptCallSource>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if {
                                                                                let source_ml_source2_comparison_operand17246 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                                                match (source_ml_source2_comparison_operand17246.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                            } {
                                                                                if { let ml_value = { let ml_reference_array295 = identities.clone(); let ml_reference_projection295 = ml_reference_array295.borrow().len() as f64; ml_reference_projection295 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                                    {
                                                                                        let source_ml_source2_constructor_field17251 = Some(true);
                                                                                        self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, None::<(Option<self::JavaScriptCallSource>,)>, Some((source_ml_source2_constructor_field17251.clone(),)), Some((iteration_2.clone(),)))
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_constructor_field17256 = Some(true);
                                                                                        self::JavaScriptCallSource::SourceWhile(Box::new(source_ml_source1_temporary_constructor_field128.clone()), Box::new(source_ml_source1_temporary_constructor_field129.clone()), Box::new(source_ml_source1_temporary_constructor_field130.clone()), None::<(Option<f64>,)>, None::<(Option<self::JavaScriptCallSource>,)>, Some((source_ml_source2_constructor_field17256.clone(),)), None::<(Option<self::JavaScriptCallSource>,)>)
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
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17264) => {
                    let source_ml_source2_data_property_value17264 = *source_ml_source2_data_property_value17264;
                    source_ml_source2_data_property_value17264.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17265) => {
                    let source_ml_source2_data_property_value17265 = *source_ml_source2_data_property_value17265;
                    source_ml_source2_data_property_value17265.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceSwitchStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17266) => {
                    let source_ml_source2_data_property_value17266 = *source_ml_source2_data_property_value17266;
                    source_ml_source2_data_property_value17266.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
    }
}
