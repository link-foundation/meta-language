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
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17836, _, _, _) => {
                    source_ml_source2_data_property_value17836
                }
            }) || !(self::ml_array::includes(&{ let ml_reference_value335 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![43, 43]), self::ml_string::Text::from_units(vec![45, 45])]))).borrow().clone(); ml_reference_value335 }, &operator.clone(), 0f64, false, |a, b| a == b))) {
                self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 117, 112, 100, 97, 116, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 97, 110, 100, 32, 112, 114, 101, 102, 105, 120, 47, 112, 111, 115, 116, 102, 105, 120, 32, 111, 112, 101, 114, 97, 116, 111, 114])))
            } else {
                {
                    let reference = self::lower_java_script_source_reference(target.clone(), context.clone(), (depth + 1f64));
                    if ((match match reference.clone() {
                        self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17839) => {
                            let source_ml_source2_data_property_value17839 = *source_ml_source2_data_property_value17839;
                            source_ml_source2_data_property_value17839.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17840, _, _, _, _) => {
                            source_ml_source2_data_property_value17840.clone()
                        }
                    }) != (self::ml_string::Text::from_units(vec![]))) {
                        self::refuse_java_script_error_source(match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17841) => {
                                let source_ml_source2_data_property_value17841 = *source_ml_source2_data_property_value17841;
                                source_ml_source2_data_property_value17841.clone()
                            }
                        }, &(match match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17842) => {
                                let source_ml_source2_data_property_value17842 = *source_ml_source2_data_property_value17842;
                                source_ml_source2_data_property_value17842.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17843, _, _, _, _) => {
                                source_ml_source2_data_property_value17843.clone()
                            }
                        }))
                    } else {
                        {
                            let prior = match match reference.clone() {
                                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17844) => {
                                    let source_ml_source2_data_property_value17844 = *source_ml_source2_data_property_value17844;
                                    source_ml_source2_data_property_value17844.clone()
                                }
                            } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17845, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17845
                                }
                            };
                            {
                                let updated = (prior + 1f64);
                                {
                                    let effect = (updated + 1f64);
                                    {
                                        let current = {
                                            let source_ml_source2_nominal_spread17847 = match reference.clone() {
                                                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17846) => {
                                                    let source_ml_source2_data_property_value17846 = *source_ml_source2_data_property_value17846;
                                                    source_ml_source2_data_property_value17846.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17848 = match source_ml_source2_nominal_spread17847.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17849, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17849 = *source_ml_source2_data_property_value17849;
                                                        source_ml_source2_data_property_value17849.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17850 = match source_ml_source2_nominal_spread17847.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17851, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value17851 = *source_ml_source2_data_property_value17851;
                                                            source_ml_source2_data_property_value17851.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17852 = match source_ml_source2_nominal_spread17847.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17853, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17853.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17854 = match source_ml_source2_nominal_spread17847.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17855, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17855
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17856 = match source_ml_source2_nominal_spread17847.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17857, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value17857
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17858 = match source_ml_source2_nominal_spread17847.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17859, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value17859.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17860 = match source_ml_source2_nominal_spread17847.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17861, _, _, _) => {
                                                                                source_ml_source2_data_property_value17861
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17862 = match source_ml_source2_nominal_spread17847.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17863, _, _) => {
                                                                                    source_ml_source2_data_property_value17863.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17864 = match source_ml_source2_nominal_spread17847.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17865, _) => {
                                                                                        source_ml_source2_data_property_value17865.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17866 = match source_ml_source2_nominal_spread17847.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17867) => {
                                                                                            source_ml_source2_data_property_value17867.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override17868 = (effect + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17848.clone()), Box::new(source_ml_source2_nominal_field17850.clone()), source_ml_source2_nominal_field17852.clone(), source_ml_source2_nominal_override17868, source_ml_source2_nominal_field17856, source_ml_source2_nominal_field17858.clone(), source_ml_source2_nominal_field17860, source_ml_source2_nominal_field17862.clone(), source_ml_source2_nominal_field17864.clone(), source_ml_source2_nominal_field17866.clone())
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
                                                            self::JavaScriptSourceReference::SourceReference(_, _, source_ml_source2_data_property_value17869, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value17869 = *source_ml_source2_data_property_value17869;
                                                                source_ml_source2_data_property_value17869.clone()
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
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17946, _, _, _) => {
                    source_ml_source2_data_property_value17946
                }
            }) || !(self::ml_array::includes(&{ let ml_reference_value336 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![43, 61]), self::ml_string::Text::from_units(vec![45, 61]), self::ml_string::Text::from_units(vec![42, 61]), self::ml_string::Text::from_units(vec![47, 61]), self::ml_string::Text::from_units(vec![37, 61]), self::ml_string::Text::from_units(vec![42, 42, 61]), self::ml_string::Text::from_units(vec![38, 38, 61]), self::ml_string::Text::from_units(vec![124, 124, 61]), self::ml_string::Text::from_units(vec![63, 63, 61])]))).borrow().clone(); ml_reference_value336 }, &operator.clone(), 0f64, false, |a, b| a == b))) {
                self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 111, 117, 110, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 100, 47, 109, 111, 100, 105, 102, 121, 47, 119, 114, 105, 116, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101])))
            } else {
                {
                    let reference = self::lower_java_script_source_reference(target.clone(), context.clone(), (depth + 1f64));
                    if ((match match reference.clone() {
                        self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17949) => {
                            let source_ml_source2_data_property_value17949 = *source_ml_source2_data_property_value17949;
                            source_ml_source2_data_property_value17949.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17950, _, _, _, _) => {
                            source_ml_source2_data_property_value17950.clone()
                        }
                    }) != (self::ml_string::Text::from_units(vec![]))) {
                        self::refuse_java_script_error_source(match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17951) => {
                                let source_ml_source2_data_property_value17951 = *source_ml_source2_data_property_value17951;
                                source_ml_source2_data_property_value17951.clone()
                            }
                        }, &(match match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17952) => {
                                let source_ml_source2_data_property_value17952 = *source_ml_source2_data_property_value17952;
                                source_ml_source2_data_property_value17952.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17953, _, _, _, _) => {
                                source_ml_source2_data_property_value17953.clone()
                            }
                        }))
                    } else {
                        if self::ml_array::includes(&{ let ml_reference_value337 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![38, 38, 61]), self::ml_string::Text::from_units(vec![124, 124, 61]), self::ml_string::Text::from_units(vec![63, 63, 61])]))).borrow().clone(); ml_reference_value337 }, &operator.clone(), 0f64, false, |a, b| a == b) {
                            {
                                let prior = match match reference.clone() {
                                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17954) => {
                                        let source_ml_source2_data_property_value17954 = *source_ml_source2_data_property_value17954;
                                        source_ml_source2_data_property_value17954.clone()
                                    }
                                } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17955, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value17955
                                    }
                                };
                                {
                                    let rhs = self::lower_java_script_error_expression(expression_value.clone(), {
                                        let source_ml_source2_nominal_spread17957 = match reference.clone() {
                                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17956) => {
                                                let source_ml_source2_data_property_value17956 = *source_ml_source2_data_property_value17956;
                                                source_ml_source2_data_property_value17956.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field17958 = match source_ml_source2_nominal_spread17957.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17959, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value17959 = *source_ml_source2_data_property_value17959;
                                                    source_ml_source2_data_property_value17959.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17960 = match source_ml_source2_nominal_spread17957.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17961, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17961 = *source_ml_source2_data_property_value17961;
                                                        source_ml_source2_data_property_value17961.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17962 = match source_ml_source2_nominal_spread17957.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17963, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value17963.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17964 = match source_ml_source2_nominal_spread17957.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17965, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17965
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17966 = match source_ml_source2_nominal_spread17957.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17967, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17967
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17968 = match source_ml_source2_nominal_spread17957.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17969, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value17969.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17970 = match source_ml_source2_nominal_spread17957.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17971, _, _, _) => {
                                                                            source_ml_source2_data_property_value17971
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17972 = match source_ml_source2_nominal_spread17957.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17973, _, _) => {
                                                                                source_ml_source2_data_property_value17973.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17974 = match source_ml_source2_nominal_spread17957.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17975, _) => {
                                                                                    source_ml_source2_data_property_value17975.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17976 = match source_ml_source2_nominal_spread17957.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17977) => {
                                                                                        source_ml_source2_data_property_value17977.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override17978 = (prior + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17958.clone()), Box::new(source_ml_source2_nominal_field17960.clone()), source_ml_source2_nominal_field17962.clone(), source_ml_source2_nominal_override17978, source_ml_source2_nominal_field17966, source_ml_source2_nominal_field17968.clone(), source_ml_source2_nominal_field17970, source_ml_source2_nominal_field17972.clone(), source_ml_source2_nominal_field17974.clone(), source_ml_source2_nominal_field17976.clone())
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17979) => {
                                                let source_ml_source2_data_property_value17979 = *source_ml_source2_data_property_value17979;
                                                source_ml_source2_data_property_value17979.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17980, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value17980
                                            }
                                        };
                                        {
                                            let effect = (updated + 1f64);
                                            {
                                                let current = {
                                                    let source_ml_source2_nominal_spread17982 = match rhs.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17981) => {
                                                            let source_ml_source2_data_property_value17981 = *source_ml_source2_data_property_value17981;
                                                            source_ml_source2_data_property_value17981.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17983 = match source_ml_source2_nominal_spread17982.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17984, _, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value17984 = *source_ml_source2_data_property_value17984;
                                                                source_ml_source2_data_property_value17984.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17985 = match source_ml_source2_nominal_spread17982.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17986, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value17986 = *source_ml_source2_data_property_value17986;
                                                                    source_ml_source2_data_property_value17986.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17987 = match source_ml_source2_nominal_spread17982.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17988, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value17988.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17989 = match source_ml_source2_nominal_spread17982.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17990, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value17990
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17991 = match source_ml_source2_nominal_spread17982.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17992, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17992
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17993 = match source_ml_source2_nominal_spread17982.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17994, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17994.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17995 = match source_ml_source2_nominal_spread17982.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17996, _, _, _) => {
                                                                                        source_ml_source2_data_property_value17996
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17997 = match source_ml_source2_nominal_spread17982.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17998, _, _) => {
                                                                                            source_ml_source2_data_property_value17998.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17999 = match source_ml_source2_nominal_spread17982.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18000, _) => {
                                                                                                source_ml_source2_data_property_value18000.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18001 = match source_ml_source2_nominal_spread17982.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18002) => {
                                                                                                    source_ml_source2_data_property_value18002.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_override18003 = (effect + 1f64);
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17983.clone()), Box::new(source_ml_source2_nominal_field17985.clone()), source_ml_source2_nominal_field17987.clone(), source_ml_source2_nominal_override18003, source_ml_source2_nominal_field17991, source_ml_source2_nominal_field17993.clone(), source_ml_source2_nominal_field17995, source_ml_source2_nominal_field17997.clone(), source_ml_source2_nominal_field17999.clone(), source_ml_source2_nominal_field18001.clone())
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
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18004, _) => {
                                                                        let source_ml_source2_data_property_value18004 = *source_ml_source2_data_property_value18004;
                                                                        source_ml_source2_data_property_value18004.clone()
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
                                                                                self::JavaScriptSourceReference::SourceReference(_, _, source_ml_source2_data_property_value18005, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value18005 = *source_ml_source2_data_property_value18005;
                                                                                    source_ml_source2_data_property_value18005.clone()
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
                                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18006) => {
                                        let source_ml_source2_data_property_value18006 = *source_ml_source2_data_property_value18006;
                                        source_ml_source2_data_property_value18006.clone()
                                    }
                                } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18007, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value18007
                                    }
                                };
                                {
                                    let binary = self::lower_java_script_source_binary(self::JavaScriptErrorExpression::ErrorBinary({
                                        let source_ml_source2_string_range_operand18009 = 0f64;
                                        {
                                            let source_ml_source2_string_range_operand18010 = (-1f64);
                                            {
                                                let source_ml_source2_string_range_position18011 = source_ml_source2_string_range_operand18009;
                                                {
                                                    let source_ml_source2_string_range_position18012 = source_ml_source2_string_range_operand18010;
                                                    operator.slice(source_ml_source2_string_range_position18011, source_ml_source2_string_range_position18012)
                                                }
                                            }
                                        }
                                    }, Box::new(self::JavaScriptErrorExpression::ErrorBinding(prior_2, None::<(Option<bool>,)>)), Box::new(expression_value.clone()), self::ml_string::Text::from_units(vec![])), {
                                        let source_ml_source2_nominal_spread18015 = match reference.clone() {
                                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18014) => {
                                                let source_ml_source2_data_property_value18014 = *source_ml_source2_data_property_value18014;
                                                source_ml_source2_data_property_value18014.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field18016 = match source_ml_source2_nominal_spread18015.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18017, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18017 = *source_ml_source2_data_property_value18017;
                                                    source_ml_source2_data_property_value18017.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field18018 = match source_ml_source2_nominal_spread18015.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18019, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value18019 = *source_ml_source2_data_property_value18019;
                                                        source_ml_source2_data_property_value18019.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18020 = match source_ml_source2_nominal_spread18015.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18021, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value18021.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18022 = match source_ml_source2_nominal_spread18015.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18023, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18023
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18024 = match source_ml_source2_nominal_spread18015.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18025, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18025
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18026 = match source_ml_source2_nominal_spread18015.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18027, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18027.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18028 = match source_ml_source2_nominal_spread18015.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18029, _, _, _) => {
                                                                            source_ml_source2_data_property_value18029
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18030 = match source_ml_source2_nominal_spread18015.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18031, _, _) => {
                                                                                source_ml_source2_data_property_value18031.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18032 = match source_ml_source2_nominal_spread18015.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18033, _) => {
                                                                                    source_ml_source2_data_property_value18033.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18034 = match source_ml_source2_nominal_spread18015.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18035) => {
                                                                                        source_ml_source2_data_property_value18035.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override18036 = (prior_2 + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18016.clone()), Box::new(source_ml_source2_nominal_field18018.clone()), source_ml_source2_nominal_field18020.clone(), source_ml_source2_nominal_override18036, source_ml_source2_nominal_field18024, source_ml_source2_nominal_field18026.clone(), source_ml_source2_nominal_field18028, source_ml_source2_nominal_field18030.clone(), source_ml_source2_nominal_field18032.clone(), source_ml_source2_nominal_field18034.clone())
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18037) => {
                                                let source_ml_source2_data_property_value18037 = *source_ml_source2_data_property_value18037;
                                                source_ml_source2_data_property_value18037.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18038, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18038
                                            }
                                        };
                                        {
                                            let effect_2 = (updated_2 + 1f64);
                                            {
                                                let current_2 = {
                                                    let source_ml_source2_nominal_spread18040 = match binary.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18039) => {
                                                            let source_ml_source2_data_property_value18039 = *source_ml_source2_data_property_value18039;
                                                            source_ml_source2_data_property_value18039.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18041 = match source_ml_source2_nominal_spread18040.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18042, _, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18042 = *source_ml_source2_data_property_value18042;
                                                                source_ml_source2_data_property_value18042.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18043 = match source_ml_source2_nominal_spread18040.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18044, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value18044 = *source_ml_source2_data_property_value18044;
                                                                    source_ml_source2_data_property_value18044.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18045 = match source_ml_source2_nominal_spread18040.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18046, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18046.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18047 = match source_ml_source2_nominal_spread18040.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18048, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18048
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18049 = match source_ml_source2_nominal_spread18040.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18050, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18050
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18051 = match source_ml_source2_nominal_spread18040.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18052, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18052.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18053 = match source_ml_source2_nominal_spread18040.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18054, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18054
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18055 = match source_ml_source2_nominal_spread18040.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18056, _, _) => {
                                                                                            source_ml_source2_data_property_value18056.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18057 = match source_ml_source2_nominal_spread18040.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18058, _) => {
                                                                                                source_ml_source2_data_property_value18058.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18059 = match source_ml_source2_nominal_spread18040.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18060) => {
                                                                                                    source_ml_source2_data_property_value18060.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_override18061 = (effect_2 + 1f64);
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18041.clone()), Box::new(source_ml_source2_nominal_field18043.clone()), source_ml_source2_nominal_field18045.clone(), source_ml_source2_nominal_override18061, source_ml_source2_nominal_field18049, source_ml_source2_nominal_field18051.clone(), source_ml_source2_nominal_field18053, source_ml_source2_nominal_field18055.clone(), source_ml_source2_nominal_field18057.clone(), source_ml_source2_nominal_field18059.clone())
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
                                                                self::JavaScriptSourceReference::SourceReference(_, _, source_ml_source2_data_property_value18062, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value18062 = *source_ml_source2_data_property_value18062;
                                                                    source_ml_source2_data_property_value18062.clone()
                                                                }
                                                            }, self::construct_java_script_error_local_binding(current_2.clone(), updated_2, match binary.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18063, _) => {
                                                                    let source_ml_source2_data_property_value18063 = *source_ml_source2_data_property_value18063;
                                                                    source_ml_source2_data_property_value18063.clone()
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
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18064) => {
                ((source_ml_source2_data_property_value18064.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value339 }; let ml_callback338 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18066, _, _, _, _) => {
                source_ml_source2_data_property_value18066
            }
        }) == identity) }) }; ml_values338.iter().cloned().enumerate().any(|(ml_index338, ml_value338)| ml_callback338(ml_value338)) } {
            self::JavaScriptCallSource::SourceInitializeCell(identity, Box::new(value.clone()), Box::new(body.clone()))
        } else {
            {
                let shared = self::ml_array::includes(&{ let ml_reference_value340 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18068, _) => {
                        ((source_ml_source2_data_property_value18068.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value340 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()));
                {
                    let stored = if shared {
                        {
                            let source_ml_source2_constructor_field18069 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                            {
                                let source_ml_source2_constructor_field18072 = match match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18070, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value18070 = *source_ml_source2_data_property_value18070;
                                        source_ml_source2_data_property_value18070.clone()
                                    }
                                } {
                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18071, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value18071.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_constructor_field18075 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                        let source_ml_source2_constructor_field18073 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                        self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field18073.clone()), Box::new(value.clone()), None::<(Option<bool>,)>)
                                    }]));
                                    self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field18069.clone()), source_ml_source2_constructor_field18075.clone(), source_ml_source2_constructor_field18072.clone())
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
