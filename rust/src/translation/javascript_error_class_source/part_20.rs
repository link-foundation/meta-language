// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallArrayPart;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceArrayElement;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorObjectProperty;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
use super::MlSource1TemporaryLowerJavaScriptErrorLiteralExpressionLoop118Result;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::lower_java_script_error_expression;
use super::ml;
use super::ml_string;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop109;
use super::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop111;
use super::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop114;
use super::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop116;
use super::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop118;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_error_computed_assignment(expression: self::JavaScriptErrorExpression, next: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    {
        let constant = std::rc::Rc::new(self::construct_java_script_error_constant);
        match expression.clone() {
            self::JavaScriptErrorExpression::ErrorComputedSet(expression_object, expression_key, expression_value) => {
                let expression_object = *expression_object;
                let expression_key = *expression_key;
                let expression_value = *expression_value;
                {
                    let object = self::lower_java_script_error_expression(expression_object.clone(), next.clone(), (depth + 1f64));
                    {
                        let key = self::lower_java_script_error_expression(expression_key.clone(), match object.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16232) => {
                                let source_ml_source2_data_property_value16232 = *source_ml_source2_data_property_value16232;
                                source_ml_source2_data_property_value16232.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let receiver_id = match match key.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16233) => {
                                    let source_ml_source2_data_property_value16233 = *source_ml_source2_data_property_value16233;
                                    source_ml_source2_data_property_value16233.clone()
                                }
                            } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16234, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value16234
                                }
                            };
                            {
                                let key_id = (receiver_id + 1f64);
                                {
                                    let value_id = (key_id + 1f64);
                                    {
                                        let value = self::lower_java_script_error_expression(expression_value.clone(), {
                                            let source_ml_source2_nominal_spread16236 = match key.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16235) => {
                                                    let source_ml_source2_data_property_value16235 = *source_ml_source2_data_property_value16235;
                                                    source_ml_source2_data_property_value16235.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16237 = match source_ml_source2_nominal_spread16236.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16238, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value16238 = *source_ml_source2_data_property_value16238;
                                                        source_ml_source2_data_property_value16238.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16239 = match source_ml_source2_nominal_spread16236.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16240, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value16240 = *source_ml_source2_data_property_value16240;
                                                            source_ml_source2_data_property_value16240.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16241 = match source_ml_source2_nominal_spread16236.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16242, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value16242.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16243 = match source_ml_source2_nominal_spread16236.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16244, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value16244
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16245 = match source_ml_source2_nominal_spread16236.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16246, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value16246
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16247 = match source_ml_source2_nominal_spread16236.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16248, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value16248.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field16249 = match source_ml_source2_nominal_spread16236.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16250, _, _, _) => {
                                                                                source_ml_source2_data_property_value16250
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field16251 = match source_ml_source2_nominal_spread16236.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16252, _, _) => {
                                                                                    source_ml_source2_data_property_value16252.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field16253 = match source_ml_source2_nominal_spread16236.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16254, _) => {
                                                                                        source_ml_source2_data_property_value16254.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field16255 = match source_ml_source2_nominal_spread16236.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16256) => {
                                                                                            source_ml_source2_data_property_value16256.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override16257 = (value_id + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16237.clone()), Box::new(source_ml_source2_nominal_field16239.clone()), source_ml_source2_nominal_field16241.clone(), source_ml_source2_nominal_override16257, source_ml_source2_nominal_field16245, source_ml_source2_nominal_field16247.clone(), source_ml_source2_nominal_field16249, source_ml_source2_nominal_field16251.clone(), source_ml_source2_nominal_field16253.clone(), source_ml_source2_nominal_field16255.clone())
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
                                            let receiver = self::JavaScriptCallSource::SourceBinding(receiver_id);
                                            {
                                                let raw_key = self::JavaScriptCallSource::SourceBinding(key_id);
                                                {
                                                    let assigned = self::JavaScriptCallSource::SourceBinding(value_id);
                                                    {
                                                        let failure = self::JavaScriptCallSource::SourceCall(Box::new((constant.clone())(match match next.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16258, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value16258 = *source_ml_source2_data_property_value16258;
                                                                source_ml_source2_data_property_value16258.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16259) => {
                                                                let source_ml_source2_data_property_value16259 = *source_ml_source2_data_property_value16259;
                                                                source_ml_source2_data_property_value16259.clone()
                                                            }
                                                        })), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![raw_key.clone(), assigned.clone()])));
                                                        {
                                                            let converted = self::JavaScriptCallSource::SourcePropertyKey(Box::new(raw_key.clone()), match match next.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16260, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value16260 = *source_ml_source2_data_property_value16260;
                                                                    source_ml_source2_data_property_value16260.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16261, _, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value16261.clone()
                                                                }
                                                            });
                                                            {
                                                                let checked_key = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Null)))), Box::new(failure.clone()), Box::new(self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)))), Box::new(failure.clone()), Box::new(converted.clone()))));
                                                                {
                                                                    let write = self::JavaScriptCallSource::SourceSnapshotSet(Box::new(receiver.clone()), Box::new(checked_key.clone()), Box::new(assigned.clone()));
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16262) => {
                                                                            let source_ml_source2_data_property_value16262 = *source_ml_source2_data_property_value16262;
                                                                            source_ml_source2_data_property_value16262.clone()
                                                                        }
                                                                    }, receiver_id, match object.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16263, _) => {
                                                                            let source_ml_source2_data_property_value16263 = *source_ml_source2_data_property_value16263;
                                                                            source_ml_source2_data_property_value16263.clone()
                                                                        }
                                                                    }, self::construct_java_script_error_local_binding(match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16264) => {
                                                                            let source_ml_source2_data_property_value16264 = *source_ml_source2_data_property_value16264;
                                                                            source_ml_source2_data_property_value16264.clone()
                                                                        }
                                                                    }, key_id, match key.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16265, _) => {
                                                                            let source_ml_source2_data_property_value16265 = *source_ml_source2_data_property_value16265;
                                                                            source_ml_source2_data_property_value16265.clone()
                                                                        }
                                                                    }, self::construct_java_script_error_local_binding(match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16266) => {
                                                                            let source_ml_source2_data_property_value16266 = *source_ml_source2_data_property_value16266;
                                                                            source_ml_source2_data_property_value16266.clone()
                                                                        }
                                                                    }, value_id, match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16267, _) => {
                                                                            let source_ml_source2_data_property_value16267 = *source_ml_source2_data_property_value16267;
                                                                            source_ml_source2_data_property_value16267.clone()
                                                                        }
                                                                    }, write.clone(), false), false), false)), Box::new(match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16268) => {
                                                                            let source_ml_source2_data_property_value16268 = *source_ml_source2_data_property_value16268;
                                                                            source_ml_source2_data_property_value16268.clone()
                                                                        }
                                                                    }))
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
                self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_error_literal_expression(expression: self::JavaScriptErrorExpression, next: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    {
        let constant = std::rc::Rc::new(self::construct_java_script_error_constant);
        match expression.clone() {
            self::JavaScriptErrorExpression::ErrorArray(factory, items) => {
                {
                    let owner = { let ml_values251 = { let ml_reference_value252 = (match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16340, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16340.clone()
                        }
                    }).borrow().clone(); ml_reference_value252 }; let ml_callback251 = { let factory = factory.clone(); std::rc::Rc::new(move |owner: self::JavaScriptSourceCapabilityOwner| -> bool { ((match owner.clone() {
                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value16342, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16342.clone()
                        }
                    }) == factory) }) }; ml_values251.iter().cloned().enumerate().find_map(|(ml_index251, ml_value251)| if ml_callback251(ml_value251.clone()) { Some(ml_value251) } else { None }) };
                    if (match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match owner.clone().expect("a proven present value") {
                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value16343) => {
                            source_ml_source2_data_property_value16343
                        }
                    })) {
                        self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 105, 116, 101, 114, 97, 108, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 114, 101, 97, 108, 109, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121])))
                    } else {
                        {
                            let current = next.clone();
                            {
                                let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()));
                                {
                                    let source_ml_source1_temporary_index104 = self::ml::Big::from_i128(0);
                                    {
                                        let current_2 = self::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop109(depth, current.clone(), arguments_.clone(), items.clone(), source_ml_source1_temporary_index104.clone());
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceCall(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object(match owner.clone().expect("a proven present value") {
                                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value16344, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value16344.clone()
                                            }
                                        }, match owner.clone().expect("a proven present value") {
                                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value16345, _) => {
                                                source_ml_source2_data_property_value16345
                                            }
                                        }))), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)), arguments_.clone())), Box::new(current_2.clone()))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptErrorExpression::ErrorArrayAccumulation(expression_parts, expression_iterator_key) => {
                let expression_iterator_key = *expression_iterator_key;
                {
                    let prototype = match match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16346, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value16346 = *source_ml_source2_data_property_value16346;
                            source_ml_source2_data_property_value16346.clone()
                        }
                    } {
                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, source_ml_source2_data_property_value16347, _, _, _, _, _, _, _, _) => {
                            ((source_ml_source2_data_property_value16347.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    };
                    if !(match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16348, _, _, _) => {
                            source_ml_source2_data_property_value16348
                        }
                    }) {
                        self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 45, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109])))
                    } else {
                        if match (prototype.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 45, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109])))
                        } else {
                            {
                                let iterator_key = self::lower_java_script_error_expression(expression_iterator_key.clone(), next.clone(), (depth + 1f64));
                                {
                                    let current_3 = match iterator_key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16349) => {
                                            let source_ml_source2_data_property_value16349 = *source_ml_source2_data_property_value16349;
                                            source_ml_source2_data_property_value16349.clone()
                                        }
                                    };
                                    {
                                        let parts = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArrayPart>::new()));
                                        {
                                            let source_ml_source1_temporary_index105 = self::ml::Big::from_i128(0);
                                            {
                                                let current_4 = self::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop111(depth, current_3.clone(), parts.clone(), expression_parts.clone(), source_ml_source1_temporary_index105.clone());
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceArrayAccumulation(Box::new((constant.clone())(prototype.clone().expect("a proven present value"))), parts.clone(), Box::new(match iterator_key.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16350, _) => {
                                                        let source_ml_source2_data_property_value16350 = *source_ml_source2_data_property_value16350;
                                                        source_ml_source2_data_property_value16350.clone()
                                                    }
                                                }), match match next.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16351, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value16351 = *source_ml_source2_data_property_value16351;
                                                        source_ml_source2_data_property_value16351.clone()
                                                    }
                                                } {
                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16352, _, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value16352.clone()
                                                    }
                                                })), Box::new(current_4.clone()))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptErrorExpression::ErrorArrayLiteral(length, expression_elements) => {
                {
                    let prototype_2 = match match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16353, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value16353 = *source_ml_source2_data_property_value16353;
                            source_ml_source2_data_property_value16353.clone()
                        }
                    } {
                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, source_ml_source2_data_property_value16354, _, _, _, _, _, _, _, _) => {
                            ((source_ml_source2_data_property_value16354.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    };
                    if !(match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16355, _, _, _) => {
                            source_ml_source2_data_property_value16355
                        }
                    }) {
                        self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 108, 105, 116, 101, 114, 97, 108, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 45, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))
                    } else {
                        if match (prototype_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 108, 105, 116, 101, 114, 97, 108, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 45, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))
                        } else {
                            {
                                let current_5 = next.clone();
                                {
                                    let elements = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceArrayElement>::new()));
                                    {
                                        let source_ml_source1_temporary_index106 = self::ml::Big::from_i128(0);
                                        {
                                            let current_6 = self::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop114(depth, current_5.clone(), elements.clone(), expression_elements.clone(), source_ml_source1_temporary_index106.clone());
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceArray(Box::new((constant.clone())(prototype_2.clone().expect("a proven present value"))), length, elements.clone(), match match next.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16356, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value16356 = *source_ml_source2_data_property_value16356;
                                                    source_ml_source2_data_property_value16356.clone()
                                                }
                                            } {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16357, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value16357.clone()
                                                }
                                            })), Box::new(current_6.clone()))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptErrorExpression::ErrorObject(expression_properties) => {
                {
                    let current_7 = next.clone();
                    {
                        let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceProperty>::new()));
                        {
                            let source_ml_source1_temporary_index107 = self::ml::Big::from_i128(0);
                            {
                                let current_8 = self::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop116(depth, current_7.clone(), properties.clone(), expression_properties.clone(), source_ml_source1_temporary_index107.clone());
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceObject(Box::new((constant.clone())(match match next.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16358, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value16358 = *source_ml_source2_data_property_value16358;
                                        source_ml_source2_data_property_value16358.clone()
                                    }
                                } {
                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, source_ml_source2_data_property_value16359, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value16359 = *source_ml_source2_data_property_value16359;
                                        source_ml_source2_data_property_value16359.clone()
                                    }
                                })), properties.clone(), match match next.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16360, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value16360 = *source_ml_source2_data_property_value16360;
                                        source_ml_source2_data_property_value16360.clone()
                                    }
                                } {
                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16361, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value16361.clone()
                                    }
                                })), Box::new(current_8.clone()))
                            }
                        }
                    }
                }
            }
            self::JavaScriptErrorExpression::ErrorTemplate(parts_2) => {
                {
                    let current_9 = next.clone();
                    {
                        let source = (constant.clone())(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                        {
                            let source_ml_source1_temporary_index108 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s16362 = self::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop118(depth, constant.clone(), current_9.clone(), source.clone(), parts_2.clone(), source_ml_source1_temporary_index108.clone());
                                match source_ml_source2_s16362.clone() {
                                    self::MlSource1TemporaryLowerJavaScriptErrorLiteralExpressionLoop118Result::MlSource1TemporaryLowerJavaScriptErrorLiteralExpressionLoop118Done(current_10, source_2) => {
                                        let current_10 = *current_10;
                                        let source_2 = *source_2;
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_2.clone()), Box::new(current_10.clone()))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_error_object_property(source: self::JavaScriptErrorObjectProperty, context: self::JavaScriptErrorSourceContext, depth: f64) -> std::rc::Rc<std::cell::RefCell<(self::JavaScriptErrorSourceContext, self::JavaScriptCallSourceProperty)>> {
    match source.clone() {
        self::JavaScriptErrorObjectProperty::ErrorObjectProperty(source_key, source_value) => {
            let source_value = *source_value;
            {
                let key = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(source_key.clone()));
                {
                    let value = self::lower_java_script_error_expression(source_value.clone(), context.clone(), depth);
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field16366 = {
                            let source_ml_source2_constructor_field16365 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16364, _) => {
                                    let source_ml_source2_data_property_value16364 = *source_ml_source2_data_property_value16364;
                                    source_ml_source2_data_property_value16364.clone()
                                }
                            };
                            self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(key.clone()), Box::new(source_ml_source2_constructor_field16365.clone()), None::<(Option<bool>,)>)
                        };
                        {
                            let source_ml_source2_field16368 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16367) => {
                                    let source_ml_source2_data_property_value16367 = *source_ml_source2_data_property_value16367;
                                    source_ml_source2_data_property_value16367.clone()
                                }
                            };
                            (source_ml_source2_field16368.clone(), source_ml_source2_field16366.clone())
                        }
                    }))
                }
            }
        }
        self::JavaScriptErrorObjectProperty::ErrorObjectAccessor(source_key_2, source_value_2, kind) => {
            let source_value_2 = *source_value_2;
            {
                let key_2 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(source_key_2.clone()));
                {
                    let value_2 = self::lower_java_script_error_expression(source_value_2.clone(), context.clone(), depth);
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field16373 = {
                            let source_ml_source2_constructor_field16371 = match value_2.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16370, _) => {
                                    let source_ml_source2_data_property_value16370 = *source_ml_source2_data_property_value16370;
                                    source_ml_source2_data_property_value16370.clone()
                                }
                            };
                            self::JavaScriptCallSourceProperty::SourceObjectAccessor(Box::new(key_2.clone()), Box::new(source_ml_source2_constructor_field16371.clone()), kind.clone(), None::<(Option<bool>,)>)
                        };
                        {
                            let source_ml_source2_field16375 = match value_2.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16374) => {
                                    let source_ml_source2_data_property_value16374 = *source_ml_source2_data_property_value16374;
                                    source_ml_source2_data_property_value16374.clone()
                                }
                            };
                            (source_ml_source2_field16375.clone(), source_ml_source2_field16373.clone())
                        }
                    }))
                }
            }
        }
        self::JavaScriptErrorObjectProperty::ErrorLiteralPrototype(source_value_3) => {
            let source_value_3 = *source_value_3;
            {
                let value_3 = self::lower_java_script_error_expression(source_value_3.clone(), context.clone(), depth);
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_field16377 = self::JavaScriptCallSourceProperty::SourceLiteralPrototype(Box::new(match value_3.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16376, _) => {
                            let source_ml_source2_data_property_value16376 = *source_ml_source2_data_property_value16376;
                            source_ml_source2_data_property_value16376.clone()
                        }
                    }));
                    {
                        let source_ml_source2_field16379 = match value_3.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16378) => {
                                let source_ml_source2_data_property_value16378 = *source_ml_source2_data_property_value16378;
                                source_ml_source2_data_property_value16378.clone()
                            }
                        };
                        (source_ml_source2_field16379.clone(), source_ml_source2_field16377.clone())
                    }
                }))
            }
        }
        self::JavaScriptErrorObjectProperty::ErrorComputedObjectProperty(source_key_3, source_value_4, infer_name, source_kind) => {
            let source_key_3 = *source_key_3;
            let source_value_4 = *source_value_4;
            {
                let key_3 = self::lower_java_script_error_expression(source_key_3.clone(), context.clone(), depth);
                {
                    let value_4 = self::lower_java_script_error_expression(source_value_4.clone(), match key_3.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16380) => {
                            let source_ml_source2_data_property_value16380 = *source_ml_source2_data_property_value16380;
                            source_ml_source2_data_property_value16380.clone()
                        }
                    }, depth);
                    {
                        let kind_2 = (((source_kind.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        if (kind_2 != (self::ml_string::Text::from_units(vec![]))) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field16392 = if match ((infer_name.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                    {
                                        let source_ml_source2_constructor_field16382 = match key_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16381, _) => {
                                                let source_ml_source2_data_property_value16381 = *source_ml_source2_data_property_value16381;
                                                source_ml_source2_data_property_value16381.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16384 = match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16383, _) => {
                                                    let source_ml_source2_data_property_value16383 = *source_ml_source2_data_property_value16383;
                                                    source_ml_source2_data_property_value16383.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field16386 = Some(true);
                                                self::JavaScriptCallSourceProperty::SourceObjectAccessor(Box::new(source_ml_source2_constructor_field16382.clone()), Box::new(source_ml_source2_constructor_field16384.clone()), kind_2.clone(), Some((source_ml_source2_constructor_field16386.clone(),)))
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let source_ml_source2_constructor_field16388 = match key_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16387, _) => {
                                                let source_ml_source2_data_property_value16387 = *source_ml_source2_data_property_value16387;
                                                source_ml_source2_data_property_value16387.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16390 = match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16389, _) => {
                                                    let source_ml_source2_data_property_value16389 = *source_ml_source2_data_property_value16389;
                                                    source_ml_source2_data_property_value16389.clone()
                                                }
                                            };
                                            self::JavaScriptCallSourceProperty::SourceObjectAccessor(Box::new(source_ml_source2_constructor_field16388.clone()), Box::new(source_ml_source2_constructor_field16390.clone()), kind_2.clone(), None::<(Option<bool>,)>)
                                        }
                                    }
                                };
                                {
                                    let source_ml_source2_field16394 = match value_4.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16393) => {
                                            let source_ml_source2_data_property_value16393 = *source_ml_source2_data_property_value16393;
                                            source_ml_source2_data_property_value16393.clone()
                                        }
                                    };
                                    (source_ml_source2_field16394.clone(), source_ml_source2_field16392.clone())
                                }
                            }))
                        } else {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field16404 = if match ((infer_name.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                    {
                                        let source_ml_source2_constructor_field16396 = match key_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16395, _) => {
                                                let source_ml_source2_data_property_value16395 = *source_ml_source2_data_property_value16395;
                                                source_ml_source2_data_property_value16395.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16398 = match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16397, _) => {
                                                    let source_ml_source2_data_property_value16397 = *source_ml_source2_data_property_value16397;
                                                    source_ml_source2_data_property_value16397.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field16399 = Some(true);
                                                self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field16396.clone()), Box::new(source_ml_source2_constructor_field16398.clone()), Some((source_ml_source2_constructor_field16399.clone(),)))
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let source_ml_source2_constructor_field16401 = match key_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16400, _) => {
                                                let source_ml_source2_data_property_value16400 = *source_ml_source2_data_property_value16400;
                                                source_ml_source2_data_property_value16400.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16403 = match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16402, _) => {
                                                    let source_ml_source2_data_property_value16402 = *source_ml_source2_data_property_value16402;
                                                    source_ml_source2_data_property_value16402.clone()
                                                }
                                            };
                                            self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field16401.clone()), Box::new(source_ml_source2_constructor_field16403.clone()), None::<(Option<bool>,)>)
                                        }
                                    }
                                };
                                {
                                    let source_ml_source2_field16406 = match value_4.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16405) => {
                                            let source_ml_source2_data_property_value16405 = *source_ml_source2_data_property_value16405;
                                            source_ml_source2_data_property_value16405.clone()
                                        }
                                    };
                                    (source_ml_source2_field16406.clone(), source_ml_source2_field16404.clone())
                                }
                            }))
                        }
                    }
                }
            }
        }
        self::JavaScriptErrorObjectProperty::ErrorObjectSpread(source_input) => {
            let source_input = *source_input;
            {
                let input = self::lower_java_script_error_expression(source_input.clone(), context.clone(), depth);
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_field16409 = {
                        let source_ml_source2_constructor_field16408 = match input.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16407, _) => {
                                let source_ml_source2_data_property_value16407 = *source_ml_source2_data_property_value16407;
                                source_ml_source2_data_property_value16407.clone()
                            }
                        };
                        self::JavaScriptCallSourceProperty::SourceObjectSpread(Box::new(source_ml_source2_constructor_field16408.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,)>)
                    };
                    {
                        let source_ml_source2_field16411 = match input.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16410) => {
                                let source_ml_source2_data_property_value16410 = *source_ml_source2_data_property_value16410;
                                source_ml_source2_data_property_value16410.clone()
                            }
                        };
                        (source_ml_source2_field16411.clone(), source_ml_source2_field16409.clone())
                    }
                }))
            }
        }
    }
}
