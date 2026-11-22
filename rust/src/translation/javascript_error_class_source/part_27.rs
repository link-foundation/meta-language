// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceBindingTarget;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceReference;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_existing_write;
use super::construct_java_script_source_field_key;
use super::construct_java_script_source_reference_captures;
use super::construct_java_script_source_reference_write;
use super::lower_java_script_error_expression;
use super::lower_java_script_source_array_binding;
use super::lower_java_script_source_binary;
use super::lower_java_script_source_object_binding;
use super::lower_java_script_source_reference;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_error_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_update(expression: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match expression.clone() {
        self::JavaScriptErrorExpression::ErrorUpdate(operator, target, prefix) => {
            let target = *target;
            if (!(match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17935, _, _, _) => {
                    source_ml_source2_data_property_value17935
                }
            }) || !(self::ml_array::includes(&{ let ml_reference_value335 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![43, 43]), self::ml_string::Text::from_units(vec![45, 45])]))).borrow().clone(); ml_reference_value335 }, &operator.clone(), 0f64, false, |a, b| a == b))) {
                self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 117, 112, 100, 97, 116, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 97, 110, 100, 32, 112, 114, 101, 102, 105, 120, 47, 112, 111, 115, 116, 102, 105, 120, 32, 111, 112, 101, 114, 97, 116, 111, 114])))
            } else {
                {
                    let reference = self::lower_java_script_source_reference(target.clone(), context.clone(), (depth + 1f64));
                    if ((match match reference.clone() {
                        self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17938) => {
                            let source_ml_source2_data_property_value17938 = *source_ml_source2_data_property_value17938;
                            source_ml_source2_data_property_value17938.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17939, _, _, _, _) => {
                            source_ml_source2_data_property_value17939.clone()
                        }
                    }) != (self::ml_string::Text::from_units(vec![]))) {
                        self::refuse_java_script_error_source(match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17940) => {
                                let source_ml_source2_data_property_value17940 = *source_ml_source2_data_property_value17940;
                                source_ml_source2_data_property_value17940.clone()
                            }
                        }, &(match match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17941) => {
                                let source_ml_source2_data_property_value17941 = *source_ml_source2_data_property_value17941;
                                source_ml_source2_data_property_value17941.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17942, _, _, _, _) => {
                                source_ml_source2_data_property_value17942.clone()
                            }
                        }))
                    } else {
                        {
                            let prior = match match reference.clone() {
                                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17943) => {
                                    let source_ml_source2_data_property_value17943 = *source_ml_source2_data_property_value17943;
                                    source_ml_source2_data_property_value17943.clone()
                                }
                            } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17944, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17944
                                }
                            };
                            {
                                let updated = (prior + 1f64);
                                {
                                    let effect = (updated + 1f64);
                                    {
                                        let current = {
                                            let source_ml_source2_nominal_spread17946 = match reference.clone() {
                                                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17945) => {
                                                    let source_ml_source2_data_property_value17945 = *source_ml_source2_data_property_value17945;
                                                    source_ml_source2_data_property_value17945.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17947 = match source_ml_source2_nominal_spread17946.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17948, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17948 = *source_ml_source2_data_property_value17948;
                                                        source_ml_source2_data_property_value17948.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17949 = match source_ml_source2_nominal_spread17946.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17950, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value17950 = *source_ml_source2_data_property_value17950;
                                                            source_ml_source2_data_property_value17950.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17951 = match source_ml_source2_nominal_spread17946.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17952, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17952.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17953 = match source_ml_source2_nominal_spread17946.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17954, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17954
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17955 = match source_ml_source2_nominal_spread17946.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17956, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value17956
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17957 = match source_ml_source2_nominal_spread17946.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17958, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value17958.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17959 = match source_ml_source2_nominal_spread17946.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17960, _, _, _) => {
                                                                                source_ml_source2_data_property_value17960
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17961 = match source_ml_source2_nominal_spread17946.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17962, _, _) => {
                                                                                    source_ml_source2_data_property_value17962.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17963 = match source_ml_source2_nominal_spread17946.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17964, _) => {
                                                                                        source_ml_source2_data_property_value17964.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17965 = match source_ml_source2_nominal_spread17946.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17966) => {
                                                                                            source_ml_source2_data_property_value17966.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override17967 = (effect + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17947.clone()), Box::new(source_ml_source2_nominal_field17949.clone()), source_ml_source2_nominal_field17951.clone(), source_ml_source2_nominal_override17967, source_ml_source2_nominal_field17955, source_ml_source2_nominal_field17957.clone(), source_ml_source2_nominal_field17959, source_ml_source2_nominal_field17961.clone(), source_ml_source2_nominal_field17963.clone(), source_ml_source2_nominal_field17965.clone())
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
                                            let old = self::JavaScriptCallSource::SourceBinding(prior);
                                            {
                                                let value = self::JavaScriptCallSource::SourceBinding(updated);
                                                {
                                                    let write = self::construct_java_script_source_reference_write(reference.clone(), value.clone());
                                                    {
                                                        let body = self::construct_java_script_error_local_binding(current.clone(), prior, self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99]), std::rc::Rc::new(std::cell::RefCell::new(vec![match reference.clone() {
                                                            self::JavaScriptSourceReference::SourceReference(_, _, source_ml_source2_data_property_value17968, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value17968 = *source_ml_source2_data_property_value17968;
                                                                source_ml_source2_data_property_value17968.clone()
                                                            }
                                                        }]))), self::construct_java_script_error_local_binding(current.clone(), updated, self::JavaScriptCallSource::SourcePrimitive(if (operator == (self::ml_string::Text::from_units(vec![43, 43]))) {
                                                            self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101])
                                                        }, std::rc::Rc::new(std::cell::RefCell::new(vec![old.clone()]))), self::construct_java_script_error_local_binding(current.clone(), effect, write.clone(), if prefix {
                                                            value.clone()
                                                        } else {
                                                            old.clone()
                                                        }, false), false), false);
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_source_reference_captures(reference.clone(), current.clone(), body.clone())), Box::new(current.clone()))
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
            self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 117, 112, 100, 97, 116, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 97, 110, 100, 32, 112, 114, 101, 102, 105, 120, 47, 112, 111, 115, 116, 102, 105, 120, 32, 111, 112, 101, 114, 97, 116, 111, 114])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_compound_assignment(expression: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match expression.clone() {
        self::JavaScriptErrorExpression::ErrorCompoundAssignment(operator, target, expression_value) => {
            let target = *target;
            let expression_value = *expression_value;
            if (!(match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18045, _, _, _) => {
                    source_ml_source2_data_property_value18045
                }
            }) || !(self::ml_array::includes(&{ let ml_reference_value336 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![43, 61]), self::ml_string::Text::from_units(vec![45, 61]), self::ml_string::Text::from_units(vec![42, 61]), self::ml_string::Text::from_units(vec![47, 61]), self::ml_string::Text::from_units(vec![37, 61]), self::ml_string::Text::from_units(vec![42, 42, 61]), self::ml_string::Text::from_units(vec![38, 38, 61]), self::ml_string::Text::from_units(vec![124, 124, 61]), self::ml_string::Text::from_units(vec![63, 63, 61])]))).borrow().clone(); ml_reference_value336 }, &operator.clone(), 0f64, false, |a, b| a == b))) {
                self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 111, 117, 110, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 100, 47, 109, 111, 100, 105, 102, 121, 47, 119, 114, 105, 116, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101])))
            } else {
                {
                    let reference = self::lower_java_script_source_reference(target.clone(), context.clone(), (depth + 1f64));
                    if ((match match reference.clone() {
                        self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18048) => {
                            let source_ml_source2_data_property_value18048 = *source_ml_source2_data_property_value18048;
                            source_ml_source2_data_property_value18048.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18049, _, _, _, _) => {
                            source_ml_source2_data_property_value18049.clone()
                        }
                    }) != (self::ml_string::Text::from_units(vec![]))) {
                        self::refuse_java_script_error_source(match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18050) => {
                                let source_ml_source2_data_property_value18050 = *source_ml_source2_data_property_value18050;
                                source_ml_source2_data_property_value18050.clone()
                            }
                        }, &(match match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18051) => {
                                let source_ml_source2_data_property_value18051 = *source_ml_source2_data_property_value18051;
                                source_ml_source2_data_property_value18051.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18052, _, _, _, _) => {
                                source_ml_source2_data_property_value18052.clone()
                            }
                        }))
                    } else {
                        if self::ml_array::includes(&{ let ml_reference_value337 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![38, 38, 61]), self::ml_string::Text::from_units(vec![124, 124, 61]), self::ml_string::Text::from_units(vec![63, 63, 61])]))).borrow().clone(); ml_reference_value337 }, &operator.clone(), 0f64, false, |a, b| a == b) {
                            {
                                let prior = match match reference.clone() {
                                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18053) => {
                                        let source_ml_source2_data_property_value18053 = *source_ml_source2_data_property_value18053;
                                        source_ml_source2_data_property_value18053.clone()
                                    }
                                } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18054, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value18054
                                    }
                                };
                                {
                                    let rhs = self::lower_java_script_error_expression(expression_value.clone(), {
                                        let source_ml_source2_nominal_spread18056 = match reference.clone() {
                                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18055) => {
                                                let source_ml_source2_data_property_value18055 = *source_ml_source2_data_property_value18055;
                                                source_ml_source2_data_property_value18055.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field18057 = match source_ml_source2_nominal_spread18056.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18058, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18058 = *source_ml_source2_data_property_value18058;
                                                    source_ml_source2_data_property_value18058.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field18059 = match source_ml_source2_nominal_spread18056.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18060, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value18060 = *source_ml_source2_data_property_value18060;
                                                        source_ml_source2_data_property_value18060.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18061 = match source_ml_source2_nominal_spread18056.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18062, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value18062.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18063 = match source_ml_source2_nominal_spread18056.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18064, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18064
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18065 = match source_ml_source2_nominal_spread18056.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18066, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18066
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18067 = match source_ml_source2_nominal_spread18056.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18068, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18068.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18069 = match source_ml_source2_nominal_spread18056.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18070, _, _, _) => {
                                                                            source_ml_source2_data_property_value18070
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18071 = match source_ml_source2_nominal_spread18056.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18072, _, _) => {
                                                                                source_ml_source2_data_property_value18072.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18073 = match source_ml_source2_nominal_spread18056.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18074, _) => {
                                                                                    source_ml_source2_data_property_value18074.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18075 = match source_ml_source2_nominal_spread18056.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18076) => {
                                                                                        source_ml_source2_data_property_value18076.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override18077 = (prior + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18057.clone()), Box::new(source_ml_source2_nominal_field18059.clone()), source_ml_source2_nominal_field18061.clone(), source_ml_source2_nominal_override18077, source_ml_source2_nominal_field18065, source_ml_source2_nominal_field18067.clone(), source_ml_source2_nominal_field18069, source_ml_source2_nominal_field18071.clone(), source_ml_source2_nominal_field18073.clone(), source_ml_source2_nominal_field18075.clone())
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
                                        let updated = match match rhs.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18078) => {
                                                let source_ml_source2_data_property_value18078 = *source_ml_source2_data_property_value18078;
                                                source_ml_source2_data_property_value18078.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18079, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18079
                                            }
                                        };
                                        {
                                            let effect = (updated + 1f64);
                                            {
                                                let current = {
                                                    let source_ml_source2_nominal_spread18081 = match rhs.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18080) => {
                                                            let source_ml_source2_data_property_value18080 = *source_ml_source2_data_property_value18080;
                                                            source_ml_source2_data_property_value18080.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18082 = match source_ml_source2_nominal_spread18081.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18083, _, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18083 = *source_ml_source2_data_property_value18083;
                                                                source_ml_source2_data_property_value18083.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18084 = match source_ml_source2_nominal_spread18081.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18085, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value18085 = *source_ml_source2_data_property_value18085;
                                                                    source_ml_source2_data_property_value18085.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18086 = match source_ml_source2_nominal_spread18081.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18087, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18087.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18088 = match source_ml_source2_nominal_spread18081.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18089, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18089
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18090 = match source_ml_source2_nominal_spread18081.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18091, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18091
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18092 = match source_ml_source2_nominal_spread18081.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18093, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18093.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18094 = match source_ml_source2_nominal_spread18081.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18095, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18095
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18096 = match source_ml_source2_nominal_spread18081.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18097, _, _) => {
                                                                                            source_ml_source2_data_property_value18097.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18098 = match source_ml_source2_nominal_spread18081.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18099, _) => {
                                                                                                source_ml_source2_data_property_value18099.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18100 = match source_ml_source2_nominal_spread18081.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18101) => {
                                                                                                    source_ml_source2_data_property_value18101.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_override18102 = (effect + 1f64);
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18082.clone()), Box::new(source_ml_source2_nominal_field18084.clone()), source_ml_source2_nominal_field18086.clone(), source_ml_source2_nominal_override18102, source_ml_source2_nominal_field18090, source_ml_source2_nominal_field18092.clone(), source_ml_source2_nominal_field18094, source_ml_source2_nominal_field18096.clone(), source_ml_source2_nominal_field18098.clone(), source_ml_source2_nominal_field18100.clone())
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
                                                    let old = self::JavaScriptCallSource::SourceBinding(prior);
                                                    {
                                                        let value = self::JavaScriptCallSource::SourceBinding(updated);
                                                        {
                                                            let constant = std::rc::Rc::new(self::construct_java_script_error_constant);
                                                            {
                                                                let selected = self::construct_java_script_error_local_binding(current.clone(), updated, match rhs.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18103, _) => {
                                                                        let source_ml_source2_data_property_value18103 = *source_ml_source2_data_property_value18103;
                                                                        source_ml_source2_data_property_value18103.clone()
                                                                    }
                                                                }, self::construct_java_script_error_local_binding(current.clone(), effect, self::construct_java_script_source_reference_write(reference.clone(), value.clone()), value.clone(), false), false);
                                                                {
                                                                    let nullish = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(old.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Null)))), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(true))), Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(old.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)))));
                                                                    {
                                                                        let choice = if (operator == (self::ml_string::Text::from_units(vec![38, 38, 61]))) {
                                                                            self::JavaScriptCallSource::SourceIf(Box::new(old.clone()), Box::new(selected.clone()), Box::new(old.clone()))
                                                                        } else {
                                                                            if (operator == (self::ml_string::Text::from_units(vec![124, 124, 61]))) {
                                                                                self::JavaScriptCallSource::SourceIf(Box::new(old.clone()), Box::new(old.clone()), Box::new(selected.clone()))
                                                                            } else {
                                                                                self::JavaScriptCallSource::SourceIf(Box::new(nullish.clone()), Box::new(selected.clone()), Box::new(old.clone()))
                                                                            }
                                                                        };
                                                                        {
                                                                            let body = self::construct_java_script_error_local_binding(current.clone(), prior, match reference.clone() {
                                                                                self::JavaScriptSourceReference::SourceReference(_, _, source_ml_source2_data_property_value18104, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value18104 = *source_ml_source2_data_property_value18104;
                                                                                    source_ml_source2_data_property_value18104.clone()
                                                                                }
                                                                            }, choice.clone(), false);
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_source_reference_captures(reference.clone(), current.clone(), body.clone())), Box::new(current.clone()))
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
                        } else {
                            {
                                let prior_2 = match match reference.clone() {
                                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18105) => {
                                        let source_ml_source2_data_property_value18105 = *source_ml_source2_data_property_value18105;
                                        source_ml_source2_data_property_value18105.clone()
                                    }
                                } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18106, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value18106
                                    }
                                };
                                {
                                    let binary = self::lower_java_script_source_binary(self::JavaScriptErrorExpression::ErrorBinary({
                                        let source_ml_source2_string_range_operand18108 = 0f64;
                                        {
                                            let source_ml_source2_string_range_operand18109 = (-1f64);
                                            {
                                                let source_ml_source2_string_range_position18110 = source_ml_source2_string_range_operand18108;
                                                {
                                                    let source_ml_source2_string_range_position18111 = source_ml_source2_string_range_operand18109;
                                                    operator.slice(source_ml_source2_string_range_position18110, source_ml_source2_string_range_position18111)
                                                }
                                            }
                                        }
                                    }, Box::new(self::JavaScriptErrorExpression::ErrorBinding(prior_2, None::<(Option<bool>,)>)), Box::new(expression_value.clone()), self::ml_string::Text::from_units(vec![])), {
                                        let source_ml_source2_nominal_spread18114 = match reference.clone() {
                                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18113) => {
                                                let source_ml_source2_data_property_value18113 = *source_ml_source2_data_property_value18113;
                                                source_ml_source2_data_property_value18113.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field18115 = match source_ml_source2_nominal_spread18114.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18116, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18116 = *source_ml_source2_data_property_value18116;
                                                    source_ml_source2_data_property_value18116.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field18117 = match source_ml_source2_nominal_spread18114.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18118, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value18118 = *source_ml_source2_data_property_value18118;
                                                        source_ml_source2_data_property_value18118.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18119 = match source_ml_source2_nominal_spread18114.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18120, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value18120.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18121 = match source_ml_source2_nominal_spread18114.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18122, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18122
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18123 = match source_ml_source2_nominal_spread18114.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18124, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18124
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18125 = match source_ml_source2_nominal_spread18114.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18126, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18126.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18127 = match source_ml_source2_nominal_spread18114.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18128, _, _, _) => {
                                                                            source_ml_source2_data_property_value18128
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18129 = match source_ml_source2_nominal_spread18114.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18130, _, _) => {
                                                                                source_ml_source2_data_property_value18130.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18131 = match source_ml_source2_nominal_spread18114.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18132, _) => {
                                                                                    source_ml_source2_data_property_value18132.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18133 = match source_ml_source2_nominal_spread18114.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18134) => {
                                                                                        source_ml_source2_data_property_value18134.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override18135 = (prior_2 + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18115.clone()), Box::new(source_ml_source2_nominal_field18117.clone()), source_ml_source2_nominal_field18119.clone(), source_ml_source2_nominal_override18135, source_ml_source2_nominal_field18123, source_ml_source2_nominal_field18125.clone(), source_ml_source2_nominal_field18127, source_ml_source2_nominal_field18129.clone(), source_ml_source2_nominal_field18131.clone(), source_ml_source2_nominal_field18133.clone())
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
                                        let updated_2 = match match binary.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18136) => {
                                                let source_ml_source2_data_property_value18136 = *source_ml_source2_data_property_value18136;
                                                source_ml_source2_data_property_value18136.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18137, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18137
                                            }
                                        };
                                        {
                                            let effect_2 = (updated_2 + 1f64);
                                            {
                                                let current_2 = {
                                                    let source_ml_source2_nominal_spread18139 = match binary.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18138) => {
                                                            let source_ml_source2_data_property_value18138 = *source_ml_source2_data_property_value18138;
                                                            source_ml_source2_data_property_value18138.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18140 = match source_ml_source2_nominal_spread18139.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18141, _, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18141 = *source_ml_source2_data_property_value18141;
                                                                source_ml_source2_data_property_value18141.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18142 = match source_ml_source2_nominal_spread18139.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18143, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value18143 = *source_ml_source2_data_property_value18143;
                                                                    source_ml_source2_data_property_value18143.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18144 = match source_ml_source2_nominal_spread18139.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18145, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18145.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18146 = match source_ml_source2_nominal_spread18139.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18147, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18147
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18148 = match source_ml_source2_nominal_spread18139.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18149, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18149
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18150 = match source_ml_source2_nominal_spread18139.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18151, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18151.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18152 = match source_ml_source2_nominal_spread18139.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18153, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18153
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18154 = match source_ml_source2_nominal_spread18139.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18155, _, _) => {
                                                                                            source_ml_source2_data_property_value18155.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18156 = match source_ml_source2_nominal_spread18139.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18157, _) => {
                                                                                                source_ml_source2_data_property_value18157.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18158 = match source_ml_source2_nominal_spread18139.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18159) => {
                                                                                                    source_ml_source2_data_property_value18159.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_override18160 = (effect_2 + 1f64);
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18140.clone()), Box::new(source_ml_source2_nominal_field18142.clone()), source_ml_source2_nominal_field18144.clone(), source_ml_source2_nominal_override18160, source_ml_source2_nominal_field18148, source_ml_source2_nominal_field18150.clone(), source_ml_source2_nominal_field18152, source_ml_source2_nominal_field18154.clone(), source_ml_source2_nominal_field18156.clone(), source_ml_source2_nominal_field18158.clone())
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
                                                    let value_2 = self::JavaScriptCallSource::SourceBinding(updated_2);
                                                    {
                                                        let write = self::construct_java_script_source_reference_write(reference.clone(), value_2.clone());
                                                        {
                                                            let body_2 = self::construct_java_script_error_local_binding(current_2.clone(), prior_2, match reference.clone() {
                                                                self::JavaScriptSourceReference::SourceReference(_, _, source_ml_source2_data_property_value18161, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value18161 = *source_ml_source2_data_property_value18161;
                                                                    source_ml_source2_data_property_value18161.clone()
                                                                }
                                                            }, self::construct_java_script_error_local_binding(current_2.clone(), updated_2, match binary.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18162, _) => {
                                                                    let source_ml_source2_data_property_value18162 = *source_ml_source2_data_property_value18162;
                                                                    source_ml_source2_data_property_value18162.clone()
                                                                }
                                                            }, self::construct_java_script_error_local_binding(current_2.clone(), effect_2, write.clone(), value_2.clone(), false), false), false);
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_source_reference_captures(reference.clone(), current_2.clone(), body_2.clone())), Box::new(current_2.clone()))
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
            self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 111, 117, 110, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 100, 47, 109, 111, 100, 105, 102, 121, 47, 119, 114, 105, 116, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_source_ordering_comparison(ordering: self::JavaScriptCallSource, code: f64) -> self::JavaScriptCallSource {
    self::JavaScriptCallSource::SourceNumberBinary(self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108]), Box::new(ordering.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Number(code))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_source_pattern_binding(context: self::JavaScriptErrorSourceContext, identity: f64, value: self::JavaScriptCallSource, body: self::JavaScriptCallSource, mutable: bool, initialize_existing: bool, target: Option<self::JavaScriptSourceBindingTarget>) -> self::JavaScriptCallSource {
    if !(match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
        self::construct_java_script_source_existing_write(target.clone().expect("a proven present value"), value.clone(), body.clone())
    } else {
        if { let ml_values338 = { let ml_reference_value339 = ((match context.clone() {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18163) => {
                ((source_ml_source2_data_property_value18163.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value339 }; let ml_callback338 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18165, _, _, _, _) => {
                source_ml_source2_data_property_value18165
            }
        }) == identity) }) }; ml_values338.iter().cloned().enumerate().any(|(ml_index338, ml_value338)| ml_callback338(ml_value338)) } {
            self::JavaScriptCallSource::SourceInitializeCell(identity, Box::new(value.clone()), Box::new(body.clone()))
        } else {
            {
                let shared = self::ml_array::includes(&{ let ml_reference_value340 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18167, _) => {
                        ((source_ml_source2_data_property_value18167.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value340 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()));
                {
                    let stored = if shared {
                        {
                            let source_ml_source2_constructor_field18168 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                            {
                                let source_ml_source2_constructor_field18171 = match match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18169, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value18169 = *source_ml_source2_data_property_value18169;
                                        source_ml_source2_data_property_value18169.clone()
                                    }
                                } {
                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18170, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value18170.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_constructor_field18174 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                        let source_ml_source2_constructor_field18172 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                        self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field18172.clone()), Box::new(value.clone()), None::<(Option<bool>,)>)
                                    }]));
                                    self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field18168.clone()), source_ml_source2_constructor_field18174.clone(), source_ml_source2_constructor_field18171.clone())
                                }
                            }
                        }
                    } else {
                        value.clone()
                    };
                    if initialize_existing {
                        self::JavaScriptCallSource::SourceInitializeBinding(identity, Box::new(stored.clone()), Box::new(body.clone()))
                    } else {
                        self::construct_java_script_error_local_binding(context.clone(), identity, stored.clone(), body.clone(), (mutable && !shared))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_binding_pattern(pattern: self::JavaScriptErrorParameter, context: self::JavaScriptErrorSourceContext, continuation: self::JavaScriptCallSource, mutable: bool, depth: f64, initialize_existing: bool) -> self::JavaScriptErrorSourceExpression {
    if match pattern.clone() {
        self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
            true
        }
        _ => {
            false
        }
    } {
        self::lower_java_script_source_array_binding(pattern.clone(), context.clone(), continuation.clone(), mutable, depth, initialize_existing)
    } else {
        self::lower_java_script_source_object_binding(pattern.clone(), context.clone(), continuation.clone(), mutable, depth, initialize_existing)
    }
}
