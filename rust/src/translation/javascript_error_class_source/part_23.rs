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
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16897) => {
                    let source_ml_source2_data_property_value16897 = *source_ml_source2_data_property_value16897;
                    source_ml_source2_data_property_value16897.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceFinallyStatement(statement_body, statement_finalizer) => {
            {
                let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16898) => {
                        let source_ml_source2_data_property_value16898 = *source_ml_source2_data_property_value16898;
                        source_ml_source2_data_property_value16898.clone()
                    }
                }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                {
                    let finalizer = self::lower_java_script_source_statements(statement_finalizer.clone(), 0f64, match body.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16899) => {
                            let source_ml_source2_data_property_value16899 = *source_ml_source2_data_property_value16899;
                            source_ml_source2_data_property_value16899.clone()
                        }
                    }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                    {
                        let identity = match match finalizer.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16900) => {
                                let source_ml_source2_data_property_value16900 = *source_ml_source2_data_property_value16900;
                                source_ml_source2_data_property_value16900.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16901, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value16901
                            }
                        };
                        {
                            let next = {
                                let source_ml_source2_nominal_spread16903 = match finalizer.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16902) => {
                                        let source_ml_source2_data_property_value16902 = *source_ml_source2_data_property_value16902;
                                        source_ml_source2_data_property_value16902.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field16904 = match source_ml_source2_nominal_spread16903.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16905, _, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value16905 = *source_ml_source2_data_property_value16905;
                                            source_ml_source2_data_property_value16905.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field16906 = match source_ml_source2_nominal_spread16903.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16907, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value16907 = *source_ml_source2_data_property_value16907;
                                                source_ml_source2_data_property_value16907.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field16908 = match source_ml_source2_nominal_spread16903.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16909, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value16909.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16910 = match source_ml_source2_nominal_spread16903.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16911, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value16911
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16912 = match source_ml_source2_nominal_spread16903.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16913, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value16913
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16914 = match source_ml_source2_nominal_spread16903.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16915, _, _, _, _) => {
                                                                source_ml_source2_data_property_value16915.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16916 = match source_ml_source2_nominal_spread16903.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16917, _, _, _) => {
                                                                    source_ml_source2_data_property_value16917
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16918 = match source_ml_source2_nominal_spread16903.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16919, _, _) => {
                                                                        source_ml_source2_data_property_value16919.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16920 = match source_ml_source2_nominal_spread16903.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16921, _) => {
                                                                            source_ml_source2_data_property_value16921.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field16922 = match source_ml_source2_nominal_spread16903.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16923) => {
                                                                                source_ml_source2_data_property_value16923.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override16924 = (identity + 1f64);
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16904.clone()), Box::new(source_ml_source2_nominal_field16906.clone()), source_ml_source2_nominal_field16908.clone(), source_ml_source2_nominal_override16924, source_ml_source2_nominal_field16912, source_ml_source2_nominal_field16914.clone(), source_ml_source2_nominal_field16916, source_ml_source2_nominal_field16918.clone(), source_ml_source2_nominal_field16920.clone(), source_ml_source2_nominal_field16922.clone())
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
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16925, _) => {
                                    let source_ml_source2_data_property_value16925 = *source_ml_source2_data_property_value16925;
                                    source_ml_source2_data_property_value16925.clone()
                                }
                            }), Box::new(match finalizer.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16926, _) => {
                                    let source_ml_source2_data_property_value16926 = *source_ml_source2_data_property_value16926;
                                    source_ml_source2_data_property_value16926.clone()
                                }
                            })), match rest.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16927, _) => {
                                    let source_ml_source2_data_property_value16927 = *source_ml_source2_data_property_value16927;
                                    source_ml_source2_data_property_value16927.clone()
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
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16928) => {
                            let source_ml_source2_data_property_value16928 = *source_ml_source2_data_property_value16928;
                            source_ml_source2_data_property_value16928.clone()
                        }
                    }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                    {
                        let handler = self::lower_java_script_source_statements(statement_handler.clone(), 0f64, match body_2.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16929) => {
                                let source_ml_source2_data_property_value16929 = *source_ml_source2_data_property_value16929;
                                source_ml_source2_data_property_value16929.clone()
                            }
                        }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                        {
                            let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                handler.clone()
                            } else {
                                self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match handler.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16930) => {
                                        let source_ml_source2_data_property_value16930 = *source_ml_source2_data_property_value16930;
                                        source_ml_source2_data_property_value16930.clone()
                                    }
                                }, match handler.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16931, _) => {
                                        let source_ml_source2_data_property_value16931 = *source_ml_source2_data_property_value16931;
                                        source_ml_source2_data_property_value16931.clone()
                                    }
                                }, true, 0f64, false)
                            };
                            {
                                let initialized_3 = if (match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && self::ml_array::includes(&{ let ml_reference_value266 = ((match match handler.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16934) => {
                                        let source_ml_source2_data_property_value16934 = *source_ml_source2_data_property_value16934;
                                        source_ml_source2_data_property_value16934.clone()
                                    }
                                } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16935, _) => {
                                        ((source_ml_source2_data_property_value16935.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value266 }, &statement_identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                                    {
                                        let temporary = match match initialized.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16936) => {
                                                let source_ml_source2_data_property_value16936 = *source_ml_source2_data_property_value16936;
                                                source_ml_source2_data_property_value16936.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16937, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value16937
                                            }
                                        };
                                        {
                                            let next_2 = {
                                                let source_ml_source2_nominal_spread16939 = match initialized.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16938) => {
                                                        let source_ml_source2_data_property_value16938 = *source_ml_source2_data_property_value16938;
                                                        source_ml_source2_data_property_value16938.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16940 = match source_ml_source2_nominal_spread16939.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16941, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value16941 = *source_ml_source2_data_property_value16941;
                                                            source_ml_source2_data_property_value16941.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16942 = match source_ml_source2_nominal_spread16939.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16943, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value16943 = *source_ml_source2_data_property_value16943;
                                                                source_ml_source2_data_property_value16943.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16944 = match source_ml_source2_nominal_spread16939.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16945, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value16945.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16946 = match source_ml_source2_nominal_spread16939.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16947, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value16947
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16948 = match source_ml_source2_nominal_spread16939.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16949, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value16949
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field16950 = match source_ml_source2_nominal_spread16939.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16951, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value16951.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field16952 = match source_ml_source2_nominal_spread16939.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16953, _, _, _) => {
                                                                                    source_ml_source2_data_property_value16953
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field16954 = match source_ml_source2_nominal_spread16939.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16955, _, _) => {
                                                                                        source_ml_source2_data_property_value16955.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field16956 = match source_ml_source2_nominal_spread16939.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16957, _) => {
                                                                                            source_ml_source2_data_property_value16957.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16958 = match source_ml_source2_nominal_spread16939.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16959) => {
                                                                                                source_ml_source2_data_property_value16959.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override16960 = (temporary + 1f64);
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16940.clone()), Box::new(source_ml_source2_nominal_field16942.clone()), source_ml_source2_nominal_field16944.clone(), source_ml_source2_nominal_override16960, source_ml_source2_nominal_field16948, source_ml_source2_nominal_field16950.clone(), source_ml_source2_nominal_field16952, source_ml_source2_nominal_field16954.clone(), source_ml_source2_nominal_field16956.clone(), source_ml_source2_nominal_field16958.clone())
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
                                                    let source_ml_source2_constructor_field16961 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                                                    {
                                                        let source_ml_source2_constructor_field16964 = match match next_2.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16962, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value16962 = *source_ml_source2_data_property_value16962;
                                                                source_ml_source2_data_property_value16962.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16963, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value16963.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field16967 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                let source_ml_source2_constructor_field16965 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                                                {
                                                                    let source_ml_source2_constructor_field16966 = self::JavaScriptCallSource::SourceBinding(statement_identity);
                                                                    self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field16965.clone()), Box::new(source_ml_source2_constructor_field16966.clone()), None::<(Option<bool>,)>)
                                                                }
                                                            }]));
                                                            self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field16961.clone()), source_ml_source2_constructor_field16967.clone(), source_ml_source2_constructor_field16964.clone())
                                                        }
                                                    }
                                                };
                                                {
                                                    let initialized_2 = {
                                                        let source_ml_source2_constructor_field16970 = self::construct_java_script_error_local_binding(next_2.clone(), temporary, self::JavaScriptCallSource::SourceAssignBinding(statement_identity, Box::new(cell.clone())), match initialized.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16969, _) => {
                                                                let source_ml_source2_data_property_value16969 = *source_ml_source2_data_property_value16969;
                                                                source_ml_source2_data_property_value16969.clone()
                                                            }
                                                        }, false);
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field16970.clone()), Box::new(next_2.clone()))
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
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16971) => {
                                            let source_ml_source2_data_property_value16971 = *source_ml_source2_data_property_value16971;
                                            source_ml_source2_data_property_value16971.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16972, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value16972
                                        }
                                    };
                                    {
                                        let next_3 = {
                                            let source_ml_source2_nominal_spread16974 = match initialized_3.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16973) => {
                                                    let source_ml_source2_data_property_value16973 = *source_ml_source2_data_property_value16973;
                                                    source_ml_source2_data_property_value16973.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16975 = match source_ml_source2_nominal_spread16974.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16976, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value16976 = *source_ml_source2_data_property_value16976;
                                                        source_ml_source2_data_property_value16976.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16977 = match source_ml_source2_nominal_spread16974.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16978, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value16978 = *source_ml_source2_data_property_value16978;
                                                            source_ml_source2_data_property_value16978.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16979 = match source_ml_source2_nominal_spread16974.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16980, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value16980.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16981 = match source_ml_source2_nominal_spread16974.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16982, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value16982
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16983 = match source_ml_source2_nominal_spread16974.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16984, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value16984
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16985 = match source_ml_source2_nominal_spread16974.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16986, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value16986.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field16987 = match source_ml_source2_nominal_spread16974.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16988, _, _, _) => {
                                                                                source_ml_source2_data_property_value16988
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field16989 = match source_ml_source2_nominal_spread16974.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16990, _, _) => {
                                                                                    source_ml_source2_data_property_value16990.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field16991 = match source_ml_source2_nominal_spread16974.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16992, _) => {
                                                                                        source_ml_source2_data_property_value16992.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field16993 = match source_ml_source2_nominal_spread16974.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16994) => {
                                                                                            source_ml_source2_data_property_value16994.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override16995 = (identity_2 + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16975.clone()), Box::new(source_ml_source2_nominal_field16977.clone()), source_ml_source2_nominal_field16979.clone(), source_ml_source2_nominal_override16995, source_ml_source2_nominal_field16983, source_ml_source2_nominal_field16985.clone(), source_ml_source2_nominal_field16987, source_ml_source2_nominal_field16989.clone(), source_ml_source2_nominal_field16991.clone(), source_ml_source2_nominal_field16993.clone())
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16996, _) => {
                                                let source_ml_source2_data_property_value16996 = *source_ml_source2_data_property_value16996;
                                                source_ml_source2_data_property_value16996.clone()
                                            }
                                        }), Box::new(match initialized_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16997, _) => {
                                                let source_ml_source2_data_property_value16997 = *source_ml_source2_data_property_value16997;
                                                source_ml_source2_data_property_value16997.clone()
                                            }
                                        })), match rest.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16998, _) => {
                                                let source_ml_source2_data_property_value16998 = *source_ml_source2_data_property_value16998;
                                                source_ml_source2_data_property_value16998.clone()
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
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16999) => {
                    let source_ml_source2_data_property_value16999 = *source_ml_source2_data_property_value16999;
                    source_ml_source2_data_property_value16999.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceObjectBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17000) => {
                    let source_ml_source2_data_property_value17000 = *source_ml_source2_data_property_value17000;
                    source_ml_source2_data_property_value17000.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceVariableStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17001) => {
                    let source_ml_source2_data_property_value17001 = *source_ml_source2_data_property_value17001;
                    source_ml_source2_data_property_value17001.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceConstStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17002) => {
                    let source_ml_source2_data_property_value17002 = *source_ml_source2_data_property_value17002;
                    source_ml_source2_data_property_value17002.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceMutableStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17003) => {
                    let source_ml_source2_data_property_value17003 = *source_ml_source2_data_property_value17003;
                    source_ml_source2_data_property_value17003.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceExpressionStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17004) => {
                    let source_ml_source2_data_property_value17004 = *source_ml_source2_data_property_value17004;
                    source_ml_source2_data_property_value17004.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17005) => {
                    let source_ml_source2_data_property_value17005 = *source_ml_source2_data_property_value17005;
                    source_ml_source2_data_property_value17005.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17006) => {
                    let source_ml_source2_data_property_value17006 = *source_ml_source2_data_property_value17006;
                    source_ml_source2_data_property_value17006.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceIfStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17007) => {
                    let source_ml_source2_data_property_value17007 = *source_ml_source2_data_property_value17007;
                    source_ml_source2_data_property_value17007.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceBlockStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17008) => {
                    let source_ml_source2_data_property_value17008 = *source_ml_source2_data_property_value17008;
                    source_ml_source2_data_property_value17008.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17009) => {
                    let source_ml_source2_data_property_value17009 = *source_ml_source2_data_property_value17009;
                    source_ml_source2_data_property_value17009.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceWhileStatement(_, _, _, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17010) => {
                    let source_ml_source2_data_property_value17010 = *source_ml_source2_data_property_value17010;
                    source_ml_source2_data_property_value17010.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlBlockStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17011) => {
                    let source_ml_source2_data_property_value17011 = *source_ml_source2_data_property_value17011;
                    source_ml_source2_data_property_value17011.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17012) => {
                    let source_ml_source2_data_property_value17012 = *source_ml_source2_data_property_value17012;
                    source_ml_source2_data_property_value17012.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceSwitchStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17013) => {
                    let source_ml_source2_data_property_value17013 = *source_ml_source2_data_property_value17013;
                    source_ml_source2_data_property_value17013.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
    }
}
