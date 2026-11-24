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
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17839, _, _, _) => {
                    source_ml_source2_data_property_value17839
                }
            }) || !(self::ml_array::includes(&{ let ml_reference_value335 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![43, 43]), self::ml_string::Text::from_units(vec![45, 45])]))).borrow().clone(); ml_reference_value335 }, &operator.clone(), 0f64, false, |a, b| a == b))) {
                self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 117, 112, 100, 97, 116, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 97, 110, 100, 32, 112, 114, 101, 102, 105, 120, 47, 112, 111, 115, 116, 102, 105, 120, 32, 111, 112, 101, 114, 97, 116, 111, 114])))
            } else {
                {
                    let reference = self::lower_java_script_source_reference(target.clone(), context.clone(), (depth + 1f64));
                    if ((match match reference.clone() {
                        self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17842) => {
                            let source_ml_source2_data_property_value17842 = *source_ml_source2_data_property_value17842;
                            source_ml_source2_data_property_value17842.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17843, _, _, _, _) => {
                            source_ml_source2_data_property_value17843.clone()
                        }
                    }) != (self::ml_string::Text::from_units(vec![]))) {
                        self::refuse_java_script_error_source(match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17844) => {
                                let source_ml_source2_data_property_value17844 = *source_ml_source2_data_property_value17844;
                                source_ml_source2_data_property_value17844.clone()
                            }
                        }, &(match match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17845) => {
                                let source_ml_source2_data_property_value17845 = *source_ml_source2_data_property_value17845;
                                source_ml_source2_data_property_value17845.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17846, _, _, _, _) => {
                                source_ml_source2_data_property_value17846.clone()
                            }
                        }))
                    } else {
                        {
                            let prior = match match reference.clone() {
                                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17847) => {
                                    let source_ml_source2_data_property_value17847 = *source_ml_source2_data_property_value17847;
                                    source_ml_source2_data_property_value17847.clone()
                                }
                            } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17848, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17848
                                }
                            };
                            {
                                let updated = (prior + 1f64);
                                {
                                    let effect = (updated + 1f64);
                                    {
                                        let current = {
                                            let source_ml_source2_nominal_spread17850 = match reference.clone() {
                                                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17849) => {
                                                    let source_ml_source2_data_property_value17849 = *source_ml_source2_data_property_value17849;
                                                    source_ml_source2_data_property_value17849.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17851 = match source_ml_source2_nominal_spread17850.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17852, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17852 = *source_ml_source2_data_property_value17852;
                                                        source_ml_source2_data_property_value17852.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17853 = match source_ml_source2_nominal_spread17850.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17854, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value17854 = *source_ml_source2_data_property_value17854;
                                                            source_ml_source2_data_property_value17854.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17855 = match source_ml_source2_nominal_spread17850.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17856, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17856.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17857 = match source_ml_source2_nominal_spread17850.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17858, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17858
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17859 = match source_ml_source2_nominal_spread17850.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17860, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value17860
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17861 = match source_ml_source2_nominal_spread17850.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17862, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value17862.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17863 = match source_ml_source2_nominal_spread17850.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17864, _, _, _) => {
                                                                                source_ml_source2_data_property_value17864
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17865 = match source_ml_source2_nominal_spread17850.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17866, _, _) => {
                                                                                    source_ml_source2_data_property_value17866.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17867 = match source_ml_source2_nominal_spread17850.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17868, _) => {
                                                                                        source_ml_source2_data_property_value17868.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17869 = match source_ml_source2_nominal_spread17850.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17870) => {
                                                                                            source_ml_source2_data_property_value17870.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override17871 = (effect + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17851.clone()), Box::new(source_ml_source2_nominal_field17853.clone()), source_ml_source2_nominal_field17855.clone(), source_ml_source2_nominal_override17871, source_ml_source2_nominal_field17859, source_ml_source2_nominal_field17861.clone(), source_ml_source2_nominal_field17863, source_ml_source2_nominal_field17865.clone(), source_ml_source2_nominal_field17867.clone(), source_ml_source2_nominal_field17869.clone())
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
                                                            self::JavaScriptSourceReference::SourceReference(_, _, source_ml_source2_data_property_value17872, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value17872 = *source_ml_source2_data_property_value17872;
                                                                source_ml_source2_data_property_value17872.clone()
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
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17949, _, _, _) => {
                    source_ml_source2_data_property_value17949
                }
            }) || !(self::ml_array::includes(&{ let ml_reference_value336 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![43, 61]), self::ml_string::Text::from_units(vec![45, 61]), self::ml_string::Text::from_units(vec![42, 61]), self::ml_string::Text::from_units(vec![47, 61]), self::ml_string::Text::from_units(vec![37, 61]), self::ml_string::Text::from_units(vec![42, 42, 61]), self::ml_string::Text::from_units(vec![38, 38, 61]), self::ml_string::Text::from_units(vec![124, 124, 61]), self::ml_string::Text::from_units(vec![63, 63, 61])]))).borrow().clone(); ml_reference_value336 }, &operator.clone(), 0f64, false, |a, b| a == b))) {
                self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 111, 117, 110, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 100, 47, 109, 111, 100, 105, 102, 121, 47, 119, 114, 105, 116, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101])))
            } else {
                {
                    let reference = self::lower_java_script_source_reference(target.clone(), context.clone(), (depth + 1f64));
                    if ((match match reference.clone() {
                        self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17952) => {
                            let source_ml_source2_data_property_value17952 = *source_ml_source2_data_property_value17952;
                            source_ml_source2_data_property_value17952.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17953, _, _, _, _) => {
                            source_ml_source2_data_property_value17953.clone()
                        }
                    }) != (self::ml_string::Text::from_units(vec![]))) {
                        self::refuse_java_script_error_source(match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17954) => {
                                let source_ml_source2_data_property_value17954 = *source_ml_source2_data_property_value17954;
                                source_ml_source2_data_property_value17954.clone()
                            }
                        }, &(match match reference.clone() {
                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17955) => {
                                let source_ml_source2_data_property_value17955 = *source_ml_source2_data_property_value17955;
                                source_ml_source2_data_property_value17955.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17956, _, _, _, _) => {
                                source_ml_source2_data_property_value17956.clone()
                            }
                        }))
                    } else {
                        if self::ml_array::includes(&{ let ml_reference_value337 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![38, 38, 61]), self::ml_string::Text::from_units(vec![124, 124, 61]), self::ml_string::Text::from_units(vec![63, 63, 61])]))).borrow().clone(); ml_reference_value337 }, &operator.clone(), 0f64, false, |a, b| a == b) {
                            {
                                let prior = match match reference.clone() {
                                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17957) => {
                                        let source_ml_source2_data_property_value17957 = *source_ml_source2_data_property_value17957;
                                        source_ml_source2_data_property_value17957.clone()
                                    }
                                } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17958, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value17958
                                    }
                                };
                                {
                                    let rhs = self::lower_java_script_error_expression(expression_value.clone(), {
                                        let source_ml_source2_nominal_spread17960 = match reference.clone() {
                                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17959) => {
                                                let source_ml_source2_data_property_value17959 = *source_ml_source2_data_property_value17959;
                                                source_ml_source2_data_property_value17959.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field17961 = match source_ml_source2_nominal_spread17960.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17962, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value17962 = *source_ml_source2_data_property_value17962;
                                                    source_ml_source2_data_property_value17962.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17963 = match source_ml_source2_nominal_spread17960.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17964, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17964 = *source_ml_source2_data_property_value17964;
                                                        source_ml_source2_data_property_value17964.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17965 = match source_ml_source2_nominal_spread17960.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17966, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value17966.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17967 = match source_ml_source2_nominal_spread17960.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17968, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17968
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17969 = match source_ml_source2_nominal_spread17960.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17970, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17970
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17971 = match source_ml_source2_nominal_spread17960.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17972, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value17972.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17973 = match source_ml_source2_nominal_spread17960.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17974, _, _, _) => {
                                                                            source_ml_source2_data_property_value17974
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17975 = match source_ml_source2_nominal_spread17960.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17976, _, _) => {
                                                                                source_ml_source2_data_property_value17976.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17977 = match source_ml_source2_nominal_spread17960.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17978, _) => {
                                                                                    source_ml_source2_data_property_value17978.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17979 = match source_ml_source2_nominal_spread17960.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17980) => {
                                                                                        source_ml_source2_data_property_value17980.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override17981 = (prior + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17961.clone()), Box::new(source_ml_source2_nominal_field17963.clone()), source_ml_source2_nominal_field17965.clone(), source_ml_source2_nominal_override17981, source_ml_source2_nominal_field17969, source_ml_source2_nominal_field17971.clone(), source_ml_source2_nominal_field17973, source_ml_source2_nominal_field17975.clone(), source_ml_source2_nominal_field17977.clone(), source_ml_source2_nominal_field17979.clone())
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17982) => {
                                                let source_ml_source2_data_property_value17982 = *source_ml_source2_data_property_value17982;
                                                source_ml_source2_data_property_value17982.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17983, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value17983
                                            }
                                        };
                                        {
                                            let effect = (updated + 1f64);
                                            {
                                                let current = {
                                                    let source_ml_source2_nominal_spread17985 = match rhs.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17984) => {
                                                            let source_ml_source2_data_property_value17984 = *source_ml_source2_data_property_value17984;
                                                            source_ml_source2_data_property_value17984.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17986 = match source_ml_source2_nominal_spread17985.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17987, _, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value17987 = *source_ml_source2_data_property_value17987;
                                                                source_ml_source2_data_property_value17987.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17988 = match source_ml_source2_nominal_spread17985.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17989, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value17989 = *source_ml_source2_data_property_value17989;
                                                                    source_ml_source2_data_property_value17989.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17990 = match source_ml_source2_nominal_spread17985.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17991, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value17991.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17992 = match source_ml_source2_nominal_spread17985.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17993, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value17993
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17994 = match source_ml_source2_nominal_spread17985.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17995, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17995
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17996 = match source_ml_source2_nominal_spread17985.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17997, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17997.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17998 = match source_ml_source2_nominal_spread17985.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17999, _, _, _) => {
                                                                                        source_ml_source2_data_property_value17999
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18000 = match source_ml_source2_nominal_spread17985.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18001, _, _) => {
                                                                                            source_ml_source2_data_property_value18001.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18002 = match source_ml_source2_nominal_spread17985.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18003, _) => {
                                                                                                source_ml_source2_data_property_value18003.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18004 = match source_ml_source2_nominal_spread17985.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18005) => {
                                                                                                    source_ml_source2_data_property_value18005.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_override18006 = (effect + 1f64);
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17986.clone()), Box::new(source_ml_source2_nominal_field17988.clone()), source_ml_source2_nominal_field17990.clone(), source_ml_source2_nominal_override18006, source_ml_source2_nominal_field17994, source_ml_source2_nominal_field17996.clone(), source_ml_source2_nominal_field17998, source_ml_source2_nominal_field18000.clone(), source_ml_source2_nominal_field18002.clone(), source_ml_source2_nominal_field18004.clone())
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
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18007, _) => {
                                                                        let source_ml_source2_data_property_value18007 = *source_ml_source2_data_property_value18007;
                                                                        source_ml_source2_data_property_value18007.clone()
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
                                                                                self::JavaScriptSourceReference::SourceReference(_, _, source_ml_source2_data_property_value18008, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value18008 = *source_ml_source2_data_property_value18008;
                                                                                    source_ml_source2_data_property_value18008.clone()
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
                                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18009) => {
                                        let source_ml_source2_data_property_value18009 = *source_ml_source2_data_property_value18009;
                                        source_ml_source2_data_property_value18009.clone()
                                    }
                                } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18010, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value18010
                                    }
                                };
                                {
                                    let binary = self::lower_java_script_source_binary(self::JavaScriptErrorExpression::ErrorBinary({
                                        let source_ml_source2_string_range_operand18012 = 0f64;
                                        {
                                            let source_ml_source2_string_range_operand18013 = (-1f64);
                                            {
                                                let source_ml_source2_string_range_position18014 = source_ml_source2_string_range_operand18012;
                                                {
                                                    let source_ml_source2_string_range_position18015 = source_ml_source2_string_range_operand18013;
                                                    operator.slice(source_ml_source2_string_range_position18014, source_ml_source2_string_range_position18015)
                                                }
                                            }
                                        }
                                    }, Box::new(self::JavaScriptErrorExpression::ErrorBinding(prior_2, None::<(Option<bool>,)>)), Box::new(expression_value.clone()), self::ml_string::Text::from_units(vec![])), {
                                        let source_ml_source2_nominal_spread18018 = match reference.clone() {
                                            self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value18017) => {
                                                let source_ml_source2_data_property_value18017 = *source_ml_source2_data_property_value18017;
                                                source_ml_source2_data_property_value18017.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field18019 = match source_ml_source2_nominal_spread18018.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18020, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value18020 = *source_ml_source2_data_property_value18020;
                                                    source_ml_source2_data_property_value18020.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field18021 = match source_ml_source2_nominal_spread18018.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18022, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value18022 = *source_ml_source2_data_property_value18022;
                                                        source_ml_source2_data_property_value18022.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18023 = match source_ml_source2_nominal_spread18018.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18024, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value18024.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18025 = match source_ml_source2_nominal_spread18018.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18026, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18026
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18027 = match source_ml_source2_nominal_spread18018.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18028, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18028
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18029 = match source_ml_source2_nominal_spread18018.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18030, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18030.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18031 = match source_ml_source2_nominal_spread18018.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18032, _, _, _) => {
                                                                            source_ml_source2_data_property_value18032
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18033 = match source_ml_source2_nominal_spread18018.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18034, _, _) => {
                                                                                source_ml_source2_data_property_value18034.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18035 = match source_ml_source2_nominal_spread18018.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18036, _) => {
                                                                                    source_ml_source2_data_property_value18036.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18037 = match source_ml_source2_nominal_spread18018.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18038) => {
                                                                                        source_ml_source2_data_property_value18038.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override18039 = (prior_2 + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18019.clone()), Box::new(source_ml_source2_nominal_field18021.clone()), source_ml_source2_nominal_field18023.clone(), source_ml_source2_nominal_override18039, source_ml_source2_nominal_field18027, source_ml_source2_nominal_field18029.clone(), source_ml_source2_nominal_field18031, source_ml_source2_nominal_field18033.clone(), source_ml_source2_nominal_field18035.clone(), source_ml_source2_nominal_field18037.clone())
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18040) => {
                                                let source_ml_source2_data_property_value18040 = *source_ml_source2_data_property_value18040;
                                                source_ml_source2_data_property_value18040.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18041, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18041
                                            }
                                        };
                                        {
                                            let effect_2 = (updated_2 + 1f64);
                                            {
                                                let current_2 = {
                                                    let source_ml_source2_nominal_spread18043 = match binary.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18042) => {
                                                            let source_ml_source2_data_property_value18042 = *source_ml_source2_data_property_value18042;
                                                            source_ml_source2_data_property_value18042.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18044 = match source_ml_source2_nominal_spread18043.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18045, _, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18045 = *source_ml_source2_data_property_value18045;
                                                                source_ml_source2_data_property_value18045.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18046 = match source_ml_source2_nominal_spread18043.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18047, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value18047 = *source_ml_source2_data_property_value18047;
                                                                    source_ml_source2_data_property_value18047.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18048 = match source_ml_source2_nominal_spread18043.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18049, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18049.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18050 = match source_ml_source2_nominal_spread18043.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18051, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18051
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18052 = match source_ml_source2_nominal_spread18043.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18053, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18053
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18054 = match source_ml_source2_nominal_spread18043.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18055, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18055.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18056 = match source_ml_source2_nominal_spread18043.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18057, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18057
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18058 = match source_ml_source2_nominal_spread18043.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18059, _, _) => {
                                                                                            source_ml_source2_data_property_value18059.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18060 = match source_ml_source2_nominal_spread18043.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18061, _) => {
                                                                                                source_ml_source2_data_property_value18061.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18062 = match source_ml_source2_nominal_spread18043.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18063) => {
                                                                                                    source_ml_source2_data_property_value18063.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_override18064 = (effect_2 + 1f64);
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18044.clone()), Box::new(source_ml_source2_nominal_field18046.clone()), source_ml_source2_nominal_field18048.clone(), source_ml_source2_nominal_override18064, source_ml_source2_nominal_field18052, source_ml_source2_nominal_field18054.clone(), source_ml_source2_nominal_field18056, source_ml_source2_nominal_field18058.clone(), source_ml_source2_nominal_field18060.clone(), source_ml_source2_nominal_field18062.clone())
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
                                                                self::JavaScriptSourceReference::SourceReference(_, _, source_ml_source2_data_property_value18065, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value18065 = *source_ml_source2_data_property_value18065;
                                                                    source_ml_source2_data_property_value18065.clone()
                                                                }
                                                            }, self::construct_java_script_error_local_binding(current_2.clone(), updated_2, match binary.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18066, _) => {
                                                                    let source_ml_source2_data_property_value18066 = *source_ml_source2_data_property_value18066;
                                                                    source_ml_source2_data_property_value18066.clone()
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
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18067) => {
                ((source_ml_source2_data_property_value18067.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value339 }; let ml_callback338 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18069, _, _, _, _) => {
                source_ml_source2_data_property_value18069
            }
        }) == identity) }) }; ml_values338.iter().cloned().enumerate().any(|(ml_index338, ml_value338)| ml_callback338(ml_value338)) } {
            self::JavaScriptCallSource::SourceInitializeCell(identity, Box::new(value.clone()), Box::new(body.clone()))
        } else {
            {
                let shared = self::ml_array::includes(&{ let ml_reference_value340 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18071, _) => {
                        ((source_ml_source2_data_property_value18071.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value340 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()));
                {
                    let stored = if shared {
                        {
                            let source_ml_source2_constructor_field18072 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                            {
                                let source_ml_source2_constructor_field18075 = match match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18073, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value18073 = *source_ml_source2_data_property_value18073;
                                        source_ml_source2_data_property_value18073.clone()
                                    }
                                } {
                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18074, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value18074.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_constructor_field18078 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                        let source_ml_source2_constructor_field18076 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                        self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field18076.clone()), Box::new(value.clone()), None::<(Option<bool>,)>)
                                    }]));
                                    self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field18072.clone()), source_ml_source2_constructor_field18078.clone(), source_ml_source2_constructor_field18075.clone())
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
