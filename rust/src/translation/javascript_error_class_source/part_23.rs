// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceStatement;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_field_key;
use super::lower_java_script_source_binding_pattern;
use super::lower_java_script_source_statements;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_error_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_exception_continuation(statement: self::JavaScriptSourceStatement, rest: self::JavaScriptErrorSourceExpression, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match statement.clone() {
        self::JavaScriptSourceStatement::SourceForOfStatement(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16995) => {
                    let source_ml_source2_data_property_value16995 = *source_ml_source2_data_property_value16995;
                    source_ml_source2_data_property_value16995.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceFinallyStatement(statement_body, statement_finalizer) => {
            {
                let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16996) => {
                        let source_ml_source2_data_property_value16996 = *source_ml_source2_data_property_value16996;
                        source_ml_source2_data_property_value16996.clone()
                    }
                }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                {
                    let finalizer = self::lower_java_script_source_statements(statement_finalizer.clone(), 0f64, match body.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16997) => {
                            let source_ml_source2_data_property_value16997 = *source_ml_source2_data_property_value16997;
                            source_ml_source2_data_property_value16997.clone()
                        }
                    }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                    {
                        let identity = match match finalizer.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16998) => {
                                let source_ml_source2_data_property_value16998 = *source_ml_source2_data_property_value16998;
                                source_ml_source2_data_property_value16998.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16999, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value16999
                            }
                        };
                        {
                            let next = {
                                let source_ml_source2_nominal_spread17001 = match finalizer.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17000) => {
                                        let source_ml_source2_data_property_value17000 = *source_ml_source2_data_property_value17000;
                                        source_ml_source2_data_property_value17000.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field17002 = match source_ml_source2_nominal_spread17001.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17003, _, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value17003 = *source_ml_source2_data_property_value17003;
                                            source_ml_source2_data_property_value17003.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field17004 = match source_ml_source2_nominal_spread17001.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17005, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value17005 = *source_ml_source2_data_property_value17005;
                                                source_ml_source2_data_property_value17005.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field17006 = match source_ml_source2_nominal_spread17001.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17007, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value17007.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17008 = match source_ml_source2_nominal_spread17001.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17009, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value17009
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17010 = match source_ml_source2_nominal_spread17001.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17011, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value17011
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17012 = match source_ml_source2_nominal_spread17001.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17013, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17013.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17014 = match source_ml_source2_nominal_spread17001.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17015, _, _, _) => {
                                                                    source_ml_source2_data_property_value17015
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17016 = match source_ml_source2_nominal_spread17001.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17017, _, _) => {
                                                                        source_ml_source2_data_property_value17017.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17018 = match source_ml_source2_nominal_spread17001.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17019, _) => {
                                                                            source_ml_source2_data_property_value17019.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17020 = match source_ml_source2_nominal_spread17001.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17021) => {
                                                                                source_ml_source2_data_property_value17021.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override17022 = (identity + 1f64);
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17002.clone()), Box::new(source_ml_source2_nominal_field17004.clone()), source_ml_source2_nominal_field17006.clone(), source_ml_source2_nominal_override17022, source_ml_source2_nominal_field17010, source_ml_source2_nominal_field17012.clone(), source_ml_source2_nominal_field17014, source_ml_source2_nominal_field17016.clone(), source_ml_source2_nominal_field17018.clone(), source_ml_source2_nominal_field17020.clone())
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
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), identity, self::JavaScriptCallSource::SourceFinally(Box::new(match body.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17023, _) => {
                                    let source_ml_source2_data_property_value17023 = *source_ml_source2_data_property_value17023;
                                    source_ml_source2_data_property_value17023.clone()
                                }
                            }), Box::new(match finalizer.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17024, _) => {
                                    let source_ml_source2_data_property_value17024 = *source_ml_source2_data_property_value17024;
                                    source_ml_source2_data_property_value17024.clone()
                                }
                            })), match rest.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17025, _) => {
                                    let source_ml_source2_data_property_value17025 = *source_ml_source2_data_property_value17025;
                                    source_ml_source2_data_property_value17025.clone()
                                }
                            }, false)), Box::new(next.clone()))
                        }
                    }
                }
            }
        }
        self::JavaScriptSourceStatement::SourceTryStatement(statement_identity, statement_body_2, statement_handler, statement_pattern) => {
            {
                let pattern = ((statement_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                {
                    let body_2 = self::lower_java_script_source_statements(statement_body_2.clone(), 0f64, match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17026) => {
                            let source_ml_source2_data_property_value17026 = *source_ml_source2_data_property_value17026;
                            source_ml_source2_data_property_value17026.clone()
                        }
                    }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                    {
                        let handler = self::lower_java_script_source_statements(statement_handler.clone(), 0f64, match body_2.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17027) => {
                                let source_ml_source2_data_property_value17027 = *source_ml_source2_data_property_value17027;
                                source_ml_source2_data_property_value17027.clone()
                            }
                        }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                        {
                            let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                handler.clone()
                            } else {
                                self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match handler.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17028) => {
                                        let source_ml_source2_data_property_value17028 = *source_ml_source2_data_property_value17028;
                                        source_ml_source2_data_property_value17028.clone()
                                    }
                                }, match handler.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17029, _) => {
                                        let source_ml_source2_data_property_value17029 = *source_ml_source2_data_property_value17029;
                                        source_ml_source2_data_property_value17029.clone()
                                    }
                                }, true, 0f64, false)
                            };
                            {
                                let initialized_3 = if (match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && self::ml_array::includes(&{ let ml_reference_value266 = ((match match handler.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17032) => {
                                        let source_ml_source2_data_property_value17032 = *source_ml_source2_data_property_value17032;
                                        source_ml_source2_data_property_value17032.clone()
                                    }
                                } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17033, _) => {
                                        ((source_ml_source2_data_property_value17033.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value266 }, &statement_identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                                    {
                                        let temporary = match match initialized.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17034) => {
                                                let source_ml_source2_data_property_value17034 = *source_ml_source2_data_property_value17034;
                                                source_ml_source2_data_property_value17034.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17035, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value17035
                                            }
                                        };
                                        {
                                            let next_2 = {
                                                let source_ml_source2_nominal_spread17037 = match initialized.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17036) => {
                                                        let source_ml_source2_data_property_value17036 = *source_ml_source2_data_property_value17036;
                                                        source_ml_source2_data_property_value17036.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17038 = match source_ml_source2_nominal_spread17037.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17039, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value17039 = *source_ml_source2_data_property_value17039;
                                                            source_ml_source2_data_property_value17039.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17040 = match source_ml_source2_nominal_spread17037.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17041, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value17041 = *source_ml_source2_data_property_value17041;
                                                                source_ml_source2_data_property_value17041.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17042 = match source_ml_source2_nominal_spread17037.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17043, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17043.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17044 = match source_ml_source2_nominal_spread17037.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17045, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value17045
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17046 = match source_ml_source2_nominal_spread17037.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17047, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value17047
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17048 = match source_ml_source2_nominal_spread17037.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17049, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17049.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17050 = match source_ml_source2_nominal_spread17037.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17051, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17051
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17052 = match source_ml_source2_nominal_spread17037.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17053, _, _) => {
                                                                                        source_ml_source2_data_property_value17053.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17054 = match source_ml_source2_nominal_spread17037.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17055, _) => {
                                                                                            source_ml_source2_data_property_value17055.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17056 = match source_ml_source2_nominal_spread17037.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17057) => {
                                                                                                source_ml_source2_data_property_value17057.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override17058 = (temporary + 1f64);
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17038.clone()), Box::new(source_ml_source2_nominal_field17040.clone()), source_ml_source2_nominal_field17042.clone(), source_ml_source2_nominal_override17058, source_ml_source2_nominal_field17046, source_ml_source2_nominal_field17048.clone(), source_ml_source2_nominal_field17050, source_ml_source2_nominal_field17052.clone(), source_ml_source2_nominal_field17054.clone(), source_ml_source2_nominal_field17056.clone())
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
                                                    let source_ml_source2_constructor_field17059 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                                                    {
                                                        let source_ml_source2_constructor_field17062 = match match next_2.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17060, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value17060 = *source_ml_source2_data_property_value17060;
                                                                source_ml_source2_data_property_value17060.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17061, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17061.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field17065 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                let source_ml_source2_constructor_field17063 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                                                {
                                                                    let source_ml_source2_constructor_field17064 = self::JavaScriptCallSource::SourceBinding(statement_identity);
                                                                    self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field17063.clone()), Box::new(source_ml_source2_constructor_field17064.clone()), None::<(Option<bool>,)>)
                                                                }
                                                            }]));
                                                            self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field17059.clone()), source_ml_source2_constructor_field17065.clone(), source_ml_source2_constructor_field17062.clone())
                                                        }
                                                    }
                                                };
                                                {
                                                    let initialized_2 = {
                                                        let source_ml_source2_constructor_field17068 = self::construct_java_script_error_local_binding(next_2.clone(), temporary, self::JavaScriptCallSource::SourceAssignBinding(statement_identity, Box::new(cell.clone())), match initialized.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17067, _) => {
                                                                let source_ml_source2_data_property_value17067 = *source_ml_source2_data_property_value17067;
                                                                source_ml_source2_data_property_value17067.clone()
                                                            }
                                                        }, false);
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field17068.clone()), Box::new(next_2.clone()))
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
                                    let identity_2 = match match initialized_3.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17069) => {
                                            let source_ml_source2_data_property_value17069 = *source_ml_source2_data_property_value17069;
                                            source_ml_source2_data_property_value17069.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17070, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value17070
                                        }
                                    };
                                    {
                                        let next_3 = {
                                            let source_ml_source2_nominal_spread17072 = match initialized_3.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17071) => {
                                                    let source_ml_source2_data_property_value17071 = *source_ml_source2_data_property_value17071;
                                                    source_ml_source2_data_property_value17071.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17073 = match source_ml_source2_nominal_spread17072.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17074, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17074 = *source_ml_source2_data_property_value17074;
                                                        source_ml_source2_data_property_value17074.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17075 = match source_ml_source2_nominal_spread17072.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17076, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value17076 = *source_ml_source2_data_property_value17076;
                                                            source_ml_source2_data_property_value17076.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17077 = match source_ml_source2_nominal_spread17072.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17078, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17078.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17079 = match source_ml_source2_nominal_spread17072.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17080, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17080
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17081 = match source_ml_source2_nominal_spread17072.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17082, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value17082
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17083 = match source_ml_source2_nominal_spread17072.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17084, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value17084.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17085 = match source_ml_source2_nominal_spread17072.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17086, _, _, _) => {
                                                                                source_ml_source2_data_property_value17086
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17087 = match source_ml_source2_nominal_spread17072.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17088, _, _) => {
                                                                                    source_ml_source2_data_property_value17088.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17089 = match source_ml_source2_nominal_spread17072.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17090, _) => {
                                                                                        source_ml_source2_data_property_value17090.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17091 = match source_ml_source2_nominal_spread17072.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17092) => {
                                                                                            source_ml_source2_data_property_value17092.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override17093 = (identity_2 + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17073.clone()), Box::new(source_ml_source2_nominal_field17075.clone()), source_ml_source2_nominal_field17077.clone(), source_ml_source2_nominal_override17093, source_ml_source2_nominal_field17081, source_ml_source2_nominal_field17083.clone(), source_ml_source2_nominal_field17085, source_ml_source2_nominal_field17087.clone(), source_ml_source2_nominal_field17089.clone(), source_ml_source2_nominal_field17091.clone())
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
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next_3.clone(), identity_2, self::JavaScriptCallSource::SourceTry(statement_identity, Box::new(match body_2.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17094, _) => {
                                                let source_ml_source2_data_property_value17094 = *source_ml_source2_data_property_value17094;
                                                source_ml_source2_data_property_value17094.clone()
                                            }
                                        }), Box::new(match initialized_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17095, _) => {
                                                let source_ml_source2_data_property_value17095 = *source_ml_source2_data_property_value17095;
                                                source_ml_source2_data_property_value17095.clone()
                                            }
                                        })), match rest.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17096, _) => {
                                                let source_ml_source2_data_property_value17096 = *source_ml_source2_data_property_value17096;
                                                source_ml_source2_data_property_value17096.clone()
                                            }
                                        }, false)), Box::new(next_3.clone()))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSourceStatement::SourceArrayBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17097) => {
                    let source_ml_source2_data_property_value17097 = *source_ml_source2_data_property_value17097;
                    source_ml_source2_data_property_value17097.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceObjectBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17098) => {
                    let source_ml_source2_data_property_value17098 = *source_ml_source2_data_property_value17098;
                    source_ml_source2_data_property_value17098.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceVariableStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17099) => {
                    let source_ml_source2_data_property_value17099 = *source_ml_source2_data_property_value17099;
                    source_ml_source2_data_property_value17099.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceConstStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17100) => {
                    let source_ml_source2_data_property_value17100 = *source_ml_source2_data_property_value17100;
                    source_ml_source2_data_property_value17100.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceMutableStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17101) => {
                    let source_ml_source2_data_property_value17101 = *source_ml_source2_data_property_value17101;
                    source_ml_source2_data_property_value17101.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceExpressionStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17102) => {
                    let source_ml_source2_data_property_value17102 = *source_ml_source2_data_property_value17102;
                    source_ml_source2_data_property_value17102.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17103) => {
                    let source_ml_source2_data_property_value17103 = *source_ml_source2_data_property_value17103;
                    source_ml_source2_data_property_value17103.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17104) => {
                    let source_ml_source2_data_property_value17104 = *source_ml_source2_data_property_value17104;
                    source_ml_source2_data_property_value17104.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceIfStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17105) => {
                    let source_ml_source2_data_property_value17105 = *source_ml_source2_data_property_value17105;
                    source_ml_source2_data_property_value17105.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceBlockStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17106) => {
                    let source_ml_source2_data_property_value17106 = *source_ml_source2_data_property_value17106;
                    source_ml_source2_data_property_value17106.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17107) => {
                    let source_ml_source2_data_property_value17107 = *source_ml_source2_data_property_value17107;
                    source_ml_source2_data_property_value17107.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceWhileStatement(_, _, _, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17108) => {
                    let source_ml_source2_data_property_value17108 = *source_ml_source2_data_property_value17108;
                    source_ml_source2_data_property_value17108.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlBlockStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17109) => {
                    let source_ml_source2_data_property_value17109 = *source_ml_source2_data_property_value17109;
                    source_ml_source2_data_property_value17109.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17110) => {
                    let source_ml_source2_data_property_value17110 = *source_ml_source2_data_property_value17110;
                    source_ml_source2_data_property_value17110.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceSwitchStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17111) => {
                    let source_ml_source2_data_property_value17111 = *source_ml_source2_data_property_value17111;
                    source_ml_source2_data_property_value17111.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
    }
}
