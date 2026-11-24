// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallSource;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryConstructJavaScriptSourceFieldKeyLoop79Result;
use super::construct_java_script_error_constant;
use super::lower_java_script_error_expression;
use super::ml;
use super::ml_string;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_construct_java_script_error_property_read_loop76;
use super::source_ml_source1_temporary_construct_java_script_source_field_key_loop79;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_error_property_read(receiver: self::JavaScriptCallSource, key: self::JavaScriptCallSource, context: self::JavaScriptErrorSourceContext) -> self::JavaScriptCallSource {
    {
        let legacy = self::JavaScriptCallSource::SourceCall(Box::new(self::construct_java_script_error_constant(match match context.clone() {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14867, _, _, _, _, _, _, _, _) => {
                let source_ml_source2_data_property_value14867 = *source_ml_source2_data_property_value14867;
                source_ml_source2_data_property_value14867.clone()
            }
        } {
            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14868, _) => {
                let source_ml_source2_data_property_value14868 = *source_ml_source2_data_property_value14868;
                source_ml_source2_data_property_value14868.clone()
            }
        })), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone()])));
        {
            let string_prototype = match match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14869, _, _, _, _, _, _, _, _) => {
                    let source_ml_source2_data_property_value14869 = *source_ml_source2_data_property_value14869;
                    source_ml_source2_data_property_value14869.clone()
                }
            } {
                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, source_ml_source2_data_property_value14870, _, _, _, _, _, _) => {
                    ((source_ml_source2_data_property_value14870.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                }
            };
            {
                let primitive = if match (string_prototype.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    legacy.clone()
                } else {
                    self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![receiver.clone()])))), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![115f64, 116f64, 114f64, 105f64, 110f64, 103f64]))))))), Box::new(self::JavaScriptCallSource::SourceStringGet(Box::new(receiver.clone()), Box::new(key.clone()), Box::new(self::construct_java_script_error_constant(string_prototype.clone().expect("a proven present value"))))), Box::new(legacy.clone()))
                };
                {
                    let source_ml_source1_temporary_values75 = std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field14874 = match match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14872, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value14872 = *source_ml_source2_data_property_value14872;
                                source_ml_source2_data_property_value14872.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, source_ml_source2_data_property_value14873, _, _, _, _, _) => {
                                ((source_ml_source2_data_property_value14873.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        };
                        (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), source_ml_source2_field14874.clone())
                    })), std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field14878 = match match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14876, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value14876 = *source_ml_source2_data_property_value14876;
                                source_ml_source2_data_property_value14876.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, source_ml_source2_data_property_value14877, _, _, _, _) => {
                                ((source_ml_source2_data_property_value14877.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        };
                        (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), source_ml_source2_field14878.clone())
                    })), std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field14882 = match match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14880, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value14880 = *source_ml_source2_data_property_value14880;
                                source_ml_source2_data_property_value14880.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, source_ml_source2_data_property_value14881, _, _, _) => {
                                ((source_ml_source2_data_property_value14881.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        };
                        (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]), source_ml_source2_field14882.clone())
                    })), std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field14886 = match match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14884, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value14884 = *source_ml_source2_data_property_value14884;
                                source_ml_source2_data_property_value14884.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14885, _, _) => {
                                ((source_ml_source2_data_property_value14885.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        };
                        (self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108]), source_ml_source2_field14886.clone())
                    }))]));
                    {
                        let source_ml_source1_temporary_index75 = self::ml::Big::from_i128(0);
                        {
                            let primitive_2 = self::source_ml_source1_temporary_construct_java_script_error_property_read_loop76(receiver.clone(), key.clone(), primitive.clone(), source_ml_source1_temporary_values75.clone(), source_ml_source1_temporary_index75.clone());
                            self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![receiver.clone()])))), Box::new(self::JavaScriptCallSource::SourceGet(Box::new(receiver.clone()), Box::new(key.clone()), None::<(Option<self::JavaScriptCallSource>,)>)), Box::new(primitive_2.clone()))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_source_computed_read(receiver: self::JavaScriptCallSource, key: self::JavaScriptCallSource, context: self::JavaScriptErrorSourceContext) -> self::JavaScriptCallSource {
    {
        let failure = self::JavaScriptCallSource::SourceCall(Box::new(self::construct_java_script_error_constant(match match context.clone() {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14889, _, _, _, _, _, _, _, _) => {
                let source_ml_source2_data_property_value14889 = *source_ml_source2_data_property_value14889;
                source_ml_source2_data_property_value14889.clone()
            }
        } {
            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14890, _) => {
                let source_ml_source2_data_property_value14890 = *source_ml_source2_data_property_value14890;
                source_ml_source2_data_property_value14890.clone()
            }
        })), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone()])));
        {
            let read = self::construct_java_script_error_property_read(receiver.clone(), self::JavaScriptCallSource::SourcePropertyKey(Box::new(key.clone()), match match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14891, _, _, _, _, _, _, _, _) => {
                    let source_ml_source2_data_property_value14891 = *source_ml_source2_data_property_value14891;
                    source_ml_source2_data_property_value14891.clone()
                }
            } {
                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value14892, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value14892.clone()
                }
            }), context.clone());
            self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null)))), Box::new(failure.clone()), Box::new(self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)))), Box::new(failure.clone()), Box::new(read.clone()))))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_source_field_key(key: &self::ml_string::Text) -> self::JavaScriptCallSource {
    {
        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = 0f64;
            {
                let source_ml_source2_s14893 = self::source_ml_source1_temporary_construct_java_script_source_field_key_loop79(key, units.clone(), at);
                match source_ml_source2_s14893.clone() {
                    self::MlSource1TemporaryConstructJavaScriptSourceFieldKeyLoop79Result::MlSource1TemporaryConstructJavaScriptSourceFieldKeyLoop79Done => {
                        self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(units.clone()))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_delete(expression: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match expression.clone() {
        self::JavaScriptErrorExpression::ErrorDelete(target) => {
            let target = *target;
            match target.clone() {
                self::JavaScriptErrorExpression::ErrorBinding(_, _) => {
                    self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 32, 115, 111, 117, 114, 99, 101, 32, 99, 97, 110, 110, 111, 116, 32, 100, 101, 108, 101, 116, 101, 32, 97, 110, 32, 117, 110, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 98, 105, 110, 100, 105, 110, 103, 32, 82, 101, 102, 101, 114, 101, 110, 99, 101])))
                }
                self::JavaScriptErrorExpression::ErrorGet(object, key) => {
                    let object = *object;
                    let key = *key;
                    self::lower_java_script_source_property_delete(object.clone(), key.clone(), false, context.clone(), depth)
                }
                self::JavaScriptErrorExpression::ErrorComputedGet(object_2, key_2) => {
                    let object_2 = *object_2;
                    let key_2 = *key_2;
                    self::lower_java_script_source_property_delete(object_2.clone(), key_2.clone(), false, context.clone(), depth)
                }
                self::JavaScriptErrorExpression::ErrorOptionalGet(object_3, key_3) => {
                    let object_3 = *object_3;
                    let key_3 = *key_3;
                    self::lower_java_script_source_property_delete(object_3.clone(), key_3.clone(), true, context.clone(), depth)
                }
                self::JavaScriptErrorExpression::ErrorModuleCell(_) => {
                    self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 32, 115, 111, 117, 114, 99, 101, 32, 99, 97, 110, 110, 111, 116, 32, 100, 101, 108, 101, 116, 101, 32, 97, 110, 32, 117, 110, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 98, 105, 110, 100, 105, 110, 103, 32, 82, 101, 102, 101, 114, 101, 110, 99, 101])))
                }
                self::JavaScriptErrorExpression::ErrorFunctionReference(_) => {
                    self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 32, 115, 111, 117, 114, 99, 101, 32, 99, 97, 110, 110, 111, 116, 32, 100, 101, 108, 101, 116, 101, 32, 97, 110, 32, 117, 110, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 98, 105, 110, 100, 105, 110, 103, 32, 82, 101, 102, 101, 114, 101, 110, 99, 101])))
                }
                self::JavaScriptErrorExpression::ErrorNativeReference(_) => {
                    self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 32, 115, 111, 117, 114, 99, 101, 32, 99, 97, 110, 110, 111, 116, 32, 100, 101, 108, 101, 116, 101, 32, 97, 110, 32, 117, 110, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 98, 105, 110, 100, 105, 110, 103, 32, 82, 101, 102, 101, 114, 101, 110, 99, 101])))
                }
                _ => {
                    self::lower_java_script_source_value_delete(target.clone(), context.clone(), depth)
                }
            }
        }
        _ => {
            self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 116, 97, 114, 103, 101, 116])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_value_delete(target: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    {
        let value = self::lower_java_script_error_expression(target.clone(), context.clone(), (depth + 1f64));
        {
            let identity = match match value.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15045) => {
                    let source_ml_source2_data_property_value15045 = *source_ml_source2_data_property_value15045;
                    source_ml_source2_data_property_value15045.clone()
                }
            } {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15046, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value15046
                }
            };
            {
                let current = {
                    let source_ml_source2_nominal_spread15048 = match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15047) => {
                            let source_ml_source2_data_property_value15047 = *source_ml_source2_data_property_value15047;
                            source_ml_source2_data_property_value15047.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field15049 = match source_ml_source2_nominal_spread15048.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15050, _, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value15050 = *source_ml_source2_data_property_value15050;
                                source_ml_source2_data_property_value15050.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field15051 = match source_ml_source2_nominal_spread15048.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15052, _, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value15052 = *source_ml_source2_data_property_value15052;
                                    source_ml_source2_data_property_value15052.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field15053 = match source_ml_source2_nominal_spread15048.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15054, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value15054.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field15055 = match source_ml_source2_nominal_spread15048.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15056, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value15056
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field15057 = match source_ml_source2_nominal_spread15048.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15058, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value15058
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15059 = match source_ml_source2_nominal_spread15048.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15060, _, _, _, _) => {
                                                    source_ml_source2_data_property_value15060.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field15061 = match source_ml_source2_nominal_spread15048.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15062, _, _, _) => {
                                                        source_ml_source2_data_property_value15062
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field15063 = match source_ml_source2_nominal_spread15048.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15064, _, _) => {
                                                            source_ml_source2_data_property_value15064.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field15065 = match source_ml_source2_nominal_spread15048.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15066, _) => {
                                                                source_ml_source2_data_property_value15066.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field15067 = match source_ml_source2_nominal_spread15048.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15068) => {
                                                                    source_ml_source2_data_property_value15068.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override15069 = (identity + 1f64);
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15049.clone()), Box::new(source_ml_source2_nominal_field15051.clone()), source_ml_source2_nominal_field15053.clone(), source_ml_source2_nominal_override15069, source_ml_source2_nominal_field15057, source_ml_source2_nominal_field15059.clone(), source_ml_source2_nominal_field15061, source_ml_source2_nominal_field15063.clone(), source_ml_source2_nominal_field15065.clone(), source_ml_source2_nominal_field15067.clone())
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
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(current.clone(), identity, match value.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15070, _) => {
                        let source_ml_source2_data_property_value15070 = *source_ml_source2_data_property_value15070;
                        source_ml_source2_data_property_value15070.clone()
                    }
                }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Boolean(true)), false)), Box::new(current.clone()))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_property_delete(object_expression: self::JavaScriptErrorExpression, key_expression: self::JavaScriptErrorExpression, optional: bool, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    {
        let constant = std::rc::Rc::new(self::construct_java_script_error_constant);
        {
            let object = self::lower_java_script_error_expression(object_expression.clone(), context.clone(), (depth + 1f64));
            {
                let key = self::lower_java_script_error_expression(key_expression.clone(), match object.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15071) => {
                        let source_ml_source2_data_property_value15071 = *source_ml_source2_data_property_value15071;
                        source_ml_source2_data_property_value15071.clone()
                    }
                }, (depth + 1f64));
                {
                    let object_id = match match key.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15072) => {
                            let source_ml_source2_data_property_value15072 = *source_ml_source2_data_property_value15072;
                            source_ml_source2_data_property_value15072.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15073, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15073
                        }
                    };
                    {
                        let key_id = (object_id + 1f64);
                        {
                            let checked_id = (key_id + 1f64);
                            {
                                let current = {
                                    let source_ml_source2_nominal_spread15075 = match key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15074) => {
                                            let source_ml_source2_data_property_value15074 = *source_ml_source2_data_property_value15074;
                                            source_ml_source2_data_property_value15074.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field15076 = match source_ml_source2_nominal_spread15075.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15077, _, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value15077 = *source_ml_source2_data_property_value15077;
                                                source_ml_source2_data_property_value15077.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15078 = match source_ml_source2_nominal_spread15075.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15079, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value15079 = *source_ml_source2_data_property_value15079;
                                                    source_ml_source2_data_property_value15079.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field15080 = match source_ml_source2_nominal_spread15075.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15081, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value15081.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field15082 = match source_ml_source2_nominal_spread15075.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15083, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value15083
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field15084 = match source_ml_source2_nominal_spread15075.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15085, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value15085
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field15086 = match source_ml_source2_nominal_spread15075.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15087, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value15087.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field15088 = match source_ml_source2_nominal_spread15075.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15089, _, _, _) => {
                                                                        source_ml_source2_data_property_value15089
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field15090 = match source_ml_source2_nominal_spread15075.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15091, _, _) => {
                                                                            source_ml_source2_data_property_value15091.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field15092 = match source_ml_source2_nominal_spread15075.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15093, _) => {
                                                                                source_ml_source2_data_property_value15093.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field15094 = match source_ml_source2_nominal_spread15075.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15095) => {
                                                                                    source_ml_source2_data_property_value15095.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override15096 = (checked_id + 1f64);
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15076.clone()), Box::new(source_ml_source2_nominal_field15078.clone()), source_ml_source2_nominal_field15080.clone(), source_ml_source2_nominal_override15096, source_ml_source2_nominal_field15084, source_ml_source2_nominal_field15086.clone(), source_ml_source2_nominal_field15088, source_ml_source2_nominal_field15090.clone(), source_ml_source2_nominal_field15092.clone(), source_ml_source2_nominal_field15094.clone())
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
                                    let receiver = self::JavaScriptCallSource::SourceBinding(object_id);
                                    {
                                        let raw_key = self::JavaScriptCallSource::SourceBinding(key_id);
                                        {
                                            let source = self::construct_java_script_error_local_binding(current.clone(), key_id, match key.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15097, _) => {
                                                    let source_ml_source2_data_property_value15097 = *source_ml_source2_data_property_value15097;
                                                    source_ml_source2_data_property_value15097.clone()
                                                }
                                            }, self::construct_java_script_error_local_binding(current.clone(), checked_id, self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 98, 106, 101, 99, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![receiver.clone()]))), self::JavaScriptCallSource::SourceDelete(Box::new(receiver.clone()), Box::new(self::JavaScriptCallSource::SourcePropertyKey(Box::new(raw_key.clone()), match match current.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15098, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value15098 = *source_ml_source2_data_property_value15098;
                                                    source_ml_source2_data_property_value15098.clone()
                                                }
                                            } {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value15099, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value15099.clone()
                                                }
                                            }))), false), false);
                                            {
                                                let source_3 = if optional {
                                                    {
                                                        let truth = (constant.clone())(self::JavaScriptSemanticValue::Boolean(true));
                                                        {
                                                            let source_2 = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Null)))), Box::new(truth.clone()), Box::new(self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)))), Box::new(truth.clone()), Box::new(source.clone()))));
                                                            source_2.clone()
                                                        }
                                                    }
                                                } else {
                                                    source.clone()
                                                };
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(current.clone(), object_id, match object.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15100, _) => {
                                                        let source_ml_source2_data_property_value15100 = *source_ml_source2_data_property_value15100;
                                                        source_ml_source2_data_property_value15100.clone()
                                                    }
                                                }, source_3.clone(), false)), Box::new(current.clone()))
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
pub fn lower_java_script_source_presence(expression: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match expression.clone() {
        self::JavaScriptErrorExpression::ErrorIn(expression_key, expression_object) => {
            let expression_key = *expression_key;
            let expression_object = *expression_object;
            {
                let key = self::lower_java_script_error_expression(expression_key.clone(), context.clone(), (depth + 1f64));
                {
                    let object = self::lower_java_script_error_expression(expression_object.clone(), match key.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15178) => {
                            let source_ml_source2_data_property_value15178 = *source_ml_source2_data_property_value15178;
                            source_ml_source2_data_property_value15178.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let key_id = match match object.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15179) => {
                                let source_ml_source2_data_property_value15179 = *source_ml_source2_data_property_value15179;
                                source_ml_source2_data_property_value15179.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15180, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15180
                            }
                        };
                        {
                            let object_id = (key_id + 1f64);
                            {
                                let checked_id = (object_id + 1f64);
                                {
                                    let current = {
                                        let source_ml_source2_nominal_spread15182 = match object.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15181) => {
                                                let source_ml_source2_data_property_value15181 = *source_ml_source2_data_property_value15181;
                                                source_ml_source2_data_property_value15181.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15183 = match source_ml_source2_nominal_spread15182.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15184, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value15184 = *source_ml_source2_data_property_value15184;
                                                    source_ml_source2_data_property_value15184.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field15185 = match source_ml_source2_nominal_spread15182.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15186, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value15186 = *source_ml_source2_data_property_value15186;
                                                        source_ml_source2_data_property_value15186.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field15187 = match source_ml_source2_nominal_spread15182.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15188, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value15188.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field15189 = match source_ml_source2_nominal_spread15182.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15190, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value15190
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field15191 = match source_ml_source2_nominal_spread15182.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15192, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value15192
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field15193 = match source_ml_source2_nominal_spread15182.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15194, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value15194.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field15195 = match source_ml_source2_nominal_spread15182.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15196, _, _, _) => {
                                                                            source_ml_source2_data_property_value15196
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field15197 = match source_ml_source2_nominal_spread15182.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15198, _, _) => {
                                                                                source_ml_source2_data_property_value15198.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field15199 = match source_ml_source2_nominal_spread15182.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15200, _) => {
                                                                                    source_ml_source2_data_property_value15200.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field15201 = match source_ml_source2_nominal_spread15182.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15202) => {
                                                                                        source_ml_source2_data_property_value15202.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override15203 = (checked_id + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15183.clone()), Box::new(source_ml_source2_nominal_field15185.clone()), source_ml_source2_nominal_field15187.clone(), source_ml_source2_nominal_override15203, source_ml_source2_nominal_field15191, source_ml_source2_nominal_field15193.clone(), source_ml_source2_nominal_field15195, source_ml_source2_nominal_field15197.clone(), source_ml_source2_nominal_field15199.clone(), source_ml_source2_nominal_field15201.clone())
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
                                        let receiver = self::JavaScriptCallSource::SourceBinding(object_id);
                                        {
                                            let raw_key = self::JavaScriptCallSource::SourceBinding(key_id);
                                            {
                                                let source = self::construct_java_script_error_local_binding(current.clone(), key_id, match key.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15204, _) => {
                                                        let source_ml_source2_data_property_value15204 = *source_ml_source2_data_property_value15204;
                                                        source_ml_source2_data_property_value15204.clone()
                                                    }
                                                }, self::construct_java_script_error_local_binding(current.clone(), object_id, match object.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15205, _) => {
                                                        let source_ml_source2_data_property_value15205 = *source_ml_source2_data_property_value15205;
                                                        source_ml_source2_data_property_value15205.clone()
                                                    }
                                                }, self::construct_java_script_error_local_binding(current.clone(), checked_id, self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 102, 111, 114, 32, 105, 110]), std::rc::Rc::new(std::cell::RefCell::new(vec![receiver.clone()]))), self::JavaScriptCallSource::SourceHas(Box::new(receiver.clone()), Box::new(self::JavaScriptCallSource::SourcePropertyKey(Box::new(raw_key.clone()), match match current.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15206, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value15206 = *source_ml_source2_data_property_value15206;
                                                        source_ml_source2_data_property_value15206.clone()
                                                    }
                                                } {
                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value15207, _, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value15207.clone()
                                                    }
                                                }))), false), false), false);
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source.clone()), Box::new(current.clone()))
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
            self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![105, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 107, 101, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_error_local_binding(context: self::JavaScriptErrorSourceContext, identity: f64, value: self::JavaScriptCallSource, body: self::JavaScriptCallSource, mutable: bool) -> self::JavaScriptCallSource {
    if match context.clone() {
        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15208, _, _, _) => {
            source_ml_source2_data_property_value15208
        }
    } {
        self::JavaScriptCallSource::SourceLocalLet(identity, Box::new(value.clone()), Box::new(body.clone()), mutable)
    } else {
        self::JavaScriptCallSource::SourceLet(identity, Box::new(value.clone()), Box::new(body.clone()))
    }
}
