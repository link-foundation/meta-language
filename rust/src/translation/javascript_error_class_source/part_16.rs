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
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14935, _, _, _, _, _, _, _, _) => {
                let source_ml_source2_data_property_value14935 = *source_ml_source2_data_property_value14935;
                source_ml_source2_data_property_value14935.clone()
            }
        } {
            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14936, _) => {
                let source_ml_source2_data_property_value14936 = *source_ml_source2_data_property_value14936;
                source_ml_source2_data_property_value14936.clone()
            }
        })), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone()])));
        {
            let string_prototype = match match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14937, _, _, _, _, _, _, _, _) => {
                    let source_ml_source2_data_property_value14937 = *source_ml_source2_data_property_value14937;
                    source_ml_source2_data_property_value14937.clone()
                }
            } {
                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, source_ml_source2_data_property_value14938, _, _, _, _, _, _) => {
                    ((source_ml_source2_data_property_value14938.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
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
                        let source_ml_source2_field14942 = match match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14940, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value14940 = *source_ml_source2_data_property_value14940;
                                source_ml_source2_data_property_value14940.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, source_ml_source2_data_property_value14941, _, _, _, _, _) => {
                                ((source_ml_source2_data_property_value14941.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        };
                        (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), source_ml_source2_field14942.clone())
                    })), std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field14946 = match match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14944, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value14944 = *source_ml_source2_data_property_value14944;
                                source_ml_source2_data_property_value14944.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, source_ml_source2_data_property_value14945, _, _, _, _) => {
                                ((source_ml_source2_data_property_value14945.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        };
                        (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), source_ml_source2_field14946.clone())
                    })), std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field14950 = match match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14948, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value14948 = *source_ml_source2_data_property_value14948;
                                source_ml_source2_data_property_value14948.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, source_ml_source2_data_property_value14949, _, _, _) => {
                                ((source_ml_source2_data_property_value14949.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        };
                        (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]), source_ml_source2_field14950.clone())
                    })), std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field14954 = match match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14952, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value14952 = *source_ml_source2_data_property_value14952;
                                source_ml_source2_data_property_value14952.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14953, _, _) => {
                                ((source_ml_source2_data_property_value14953.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        };
                        (self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108]), source_ml_source2_field14954.clone())
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
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14957, _, _, _, _, _, _, _, _) => {
                let source_ml_source2_data_property_value14957 = *source_ml_source2_data_property_value14957;
                source_ml_source2_data_property_value14957.clone()
            }
        } {
            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14958, _) => {
                let source_ml_source2_data_property_value14958 = *source_ml_source2_data_property_value14958;
                source_ml_source2_data_property_value14958.clone()
            }
        })), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![key.clone()])));
        {
            let read = self::construct_java_script_error_property_read(receiver.clone(), self::JavaScriptCallSource::SourcePropertyKey(Box::new(key.clone()), match match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value14959, _, _, _, _, _, _, _, _) => {
                    let source_ml_source2_data_property_value14959 = *source_ml_source2_data_property_value14959;
                    source_ml_source2_data_property_value14959.clone()
                }
            } {
                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value14960, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value14960.clone()
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
                let source_ml_source2_s14961 = self::source_ml_source1_temporary_construct_java_script_source_field_key_loop79(key, units.clone(), at);
                match source_ml_source2_s14961.clone() {
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
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15113) => {
                    let source_ml_source2_data_property_value15113 = *source_ml_source2_data_property_value15113;
                    source_ml_source2_data_property_value15113.clone()
                }
            } {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15114, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value15114
                }
            };
            {
                let current = {
                    let source_ml_source2_nominal_spread15116 = match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15115) => {
                            let source_ml_source2_data_property_value15115 = *source_ml_source2_data_property_value15115;
                            source_ml_source2_data_property_value15115.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field15117 = match source_ml_source2_nominal_spread15116.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15118, _, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value15118 = *source_ml_source2_data_property_value15118;
                                source_ml_source2_data_property_value15118.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field15119 = match source_ml_source2_nominal_spread15116.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15120, _, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value15120 = *source_ml_source2_data_property_value15120;
                                    source_ml_source2_data_property_value15120.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field15121 = match source_ml_source2_nominal_spread15116.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15122, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value15122.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field15123 = match source_ml_source2_nominal_spread15116.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15124, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value15124
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field15125 = match source_ml_source2_nominal_spread15116.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15126, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value15126
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15127 = match source_ml_source2_nominal_spread15116.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15128, _, _, _, _) => {
                                                    source_ml_source2_data_property_value15128.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field15129 = match source_ml_source2_nominal_spread15116.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15130, _, _, _) => {
                                                        source_ml_source2_data_property_value15130
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field15131 = match source_ml_source2_nominal_spread15116.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15132, _, _) => {
                                                            source_ml_source2_data_property_value15132.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field15133 = match source_ml_source2_nominal_spread15116.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15134, _) => {
                                                                source_ml_source2_data_property_value15134.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field15135 = match source_ml_source2_nominal_spread15116.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15136) => {
                                                                    source_ml_source2_data_property_value15136.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override15137 = (identity + 1f64);
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15117.clone()), Box::new(source_ml_source2_nominal_field15119.clone()), source_ml_source2_nominal_field15121.clone(), source_ml_source2_nominal_override15137, source_ml_source2_nominal_field15125, source_ml_source2_nominal_field15127.clone(), source_ml_source2_nominal_field15129, source_ml_source2_nominal_field15131.clone(), source_ml_source2_nominal_field15133.clone(), source_ml_source2_nominal_field15135.clone())
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
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15138, _) => {
                        let source_ml_source2_data_property_value15138 = *source_ml_source2_data_property_value15138;
                        source_ml_source2_data_property_value15138.clone()
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
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15139) => {
                        let source_ml_source2_data_property_value15139 = *source_ml_source2_data_property_value15139;
                        source_ml_source2_data_property_value15139.clone()
                    }
                }, (depth + 1f64));
                {
                    let object_id = match match key.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15140) => {
                            let source_ml_source2_data_property_value15140 = *source_ml_source2_data_property_value15140;
                            source_ml_source2_data_property_value15140.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15141, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15141
                        }
                    };
                    {
                        let key_id = (object_id + 1f64);
                        {
                            let checked_id = (key_id + 1f64);
                            {
                                let current = {
                                    let source_ml_source2_nominal_spread15143 = match key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15142) => {
                                            let source_ml_source2_data_property_value15142 = *source_ml_source2_data_property_value15142;
                                            source_ml_source2_data_property_value15142.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field15144 = match source_ml_source2_nominal_spread15143.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15145, _, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value15145 = *source_ml_source2_data_property_value15145;
                                                source_ml_source2_data_property_value15145.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15146 = match source_ml_source2_nominal_spread15143.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15147, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value15147 = *source_ml_source2_data_property_value15147;
                                                    source_ml_source2_data_property_value15147.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field15148 = match source_ml_source2_nominal_spread15143.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15149, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value15149.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field15150 = match source_ml_source2_nominal_spread15143.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15151, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value15151
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field15152 = match source_ml_source2_nominal_spread15143.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15153, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value15153
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field15154 = match source_ml_source2_nominal_spread15143.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15155, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value15155.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field15156 = match source_ml_source2_nominal_spread15143.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15157, _, _, _) => {
                                                                        source_ml_source2_data_property_value15157
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field15158 = match source_ml_source2_nominal_spread15143.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15159, _, _) => {
                                                                            source_ml_source2_data_property_value15159.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field15160 = match source_ml_source2_nominal_spread15143.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15161, _) => {
                                                                                source_ml_source2_data_property_value15161.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field15162 = match source_ml_source2_nominal_spread15143.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15163) => {
                                                                                    source_ml_source2_data_property_value15163.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override15164 = (checked_id + 1f64);
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15144.clone()), Box::new(source_ml_source2_nominal_field15146.clone()), source_ml_source2_nominal_field15148.clone(), source_ml_source2_nominal_override15164, source_ml_source2_nominal_field15152, source_ml_source2_nominal_field15154.clone(), source_ml_source2_nominal_field15156, source_ml_source2_nominal_field15158.clone(), source_ml_source2_nominal_field15160.clone(), source_ml_source2_nominal_field15162.clone())
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
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15165, _) => {
                                                    let source_ml_source2_data_property_value15165 = *source_ml_source2_data_property_value15165;
                                                    source_ml_source2_data_property_value15165.clone()
                                                }
                                            }, self::construct_java_script_error_local_binding(current.clone(), checked_id, self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 98, 106, 101, 99, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![receiver.clone()]))), self::JavaScriptCallSource::SourceDelete(Box::new(receiver.clone()), Box::new(self::JavaScriptCallSource::SourcePropertyKey(Box::new(raw_key.clone()), match match current.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15166, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value15166 = *source_ml_source2_data_property_value15166;
                                                    source_ml_source2_data_property_value15166.clone()
                                                }
                                            } {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value15167, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value15167.clone()
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
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15168, _) => {
                                                        let source_ml_source2_data_property_value15168 = *source_ml_source2_data_property_value15168;
                                                        source_ml_source2_data_property_value15168.clone()
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
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15246) => {
                            let source_ml_source2_data_property_value15246 = *source_ml_source2_data_property_value15246;
                            source_ml_source2_data_property_value15246.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let key_id = match match object.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15247) => {
                                let source_ml_source2_data_property_value15247 = *source_ml_source2_data_property_value15247;
                                source_ml_source2_data_property_value15247.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15248, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15248
                            }
                        };
                        {
                            let object_id = (key_id + 1f64);
                            {
                                let checked_id = (object_id + 1f64);
                                {
                                    let current = {
                                        let source_ml_source2_nominal_spread15250 = match object.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15249) => {
                                                let source_ml_source2_data_property_value15249 = *source_ml_source2_data_property_value15249;
                                                source_ml_source2_data_property_value15249.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15251 = match source_ml_source2_nominal_spread15250.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15252, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value15252 = *source_ml_source2_data_property_value15252;
                                                    source_ml_source2_data_property_value15252.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field15253 = match source_ml_source2_nominal_spread15250.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15254, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value15254 = *source_ml_source2_data_property_value15254;
                                                        source_ml_source2_data_property_value15254.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field15255 = match source_ml_source2_nominal_spread15250.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15256, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value15256.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field15257 = match source_ml_source2_nominal_spread15250.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15258, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value15258
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field15259 = match source_ml_source2_nominal_spread15250.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15260, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value15260
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field15261 = match source_ml_source2_nominal_spread15250.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15262, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value15262.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field15263 = match source_ml_source2_nominal_spread15250.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15264, _, _, _) => {
                                                                            source_ml_source2_data_property_value15264
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field15265 = match source_ml_source2_nominal_spread15250.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15266, _, _) => {
                                                                                source_ml_source2_data_property_value15266.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field15267 = match source_ml_source2_nominal_spread15250.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15268, _) => {
                                                                                    source_ml_source2_data_property_value15268.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field15269 = match source_ml_source2_nominal_spread15250.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15270) => {
                                                                                        source_ml_source2_data_property_value15270.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override15271 = (checked_id + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15251.clone()), Box::new(source_ml_source2_nominal_field15253.clone()), source_ml_source2_nominal_field15255.clone(), source_ml_source2_nominal_override15271, source_ml_source2_nominal_field15259, source_ml_source2_nominal_field15261.clone(), source_ml_source2_nominal_field15263, source_ml_source2_nominal_field15265.clone(), source_ml_source2_nominal_field15267.clone(), source_ml_source2_nominal_field15269.clone())
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
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15272, _) => {
                                                        let source_ml_source2_data_property_value15272 = *source_ml_source2_data_property_value15272;
                                                        source_ml_source2_data_property_value15272.clone()
                                                    }
                                                }, self::construct_java_script_error_local_binding(current.clone(), object_id, match object.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15273, _) => {
                                                        let source_ml_source2_data_property_value15273 = *source_ml_source2_data_property_value15273;
                                                        source_ml_source2_data_property_value15273.clone()
                                                    }
                                                }, self::construct_java_script_error_local_binding(current.clone(), checked_id, self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 102, 111, 114, 32, 105, 110]), std::rc::Rc::new(std::cell::RefCell::new(vec![receiver.clone()]))), self::JavaScriptCallSource::SourceHas(Box::new(receiver.clone()), Box::new(self::JavaScriptCallSource::SourcePropertyKey(Box::new(raw_key.clone()), match match current.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15274, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value15274 = *source_ml_source2_data_property_value15274;
                                                        source_ml_source2_data_property_value15274.clone()
                                                    }
                                                } {
                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value15275, _, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value15275.clone()
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
        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15276, _, _, _) => {
            source_ml_source2_data_property_value15276
        }
    } {
        self::JavaScriptCallSource::SourceLocalLet(identity, Box::new(value.clone()), Box::new(body.clone()), mutable)
    } else {
        self::JavaScriptCallSource::SourceLet(identity, Box::new(value.clone()), Box::new(body.clone()))
    }
}
