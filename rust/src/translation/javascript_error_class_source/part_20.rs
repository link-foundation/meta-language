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
use super::ml_source_exception;
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
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16316) => {
                                let source_ml_source2_data_property_value16316 = *source_ml_source2_data_property_value16316;
                                source_ml_source2_data_property_value16316.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let receiver_id = match match key.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16317) => {
                                    let source_ml_source2_data_property_value16317 = *source_ml_source2_data_property_value16317;
                                    source_ml_source2_data_property_value16317.clone()
                                }
                            } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16318, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value16318
                                }
                            };
                            {
                                let key_id = (receiver_id + 1f64);
                                {
                                    let value_id = (key_id + 1f64);
                                    {
                                        let value = self::lower_java_script_error_expression(expression_value.clone(), {
                                            let source_ml_source2_nominal_spread16320 = match key.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16319) => {
                                                    let source_ml_source2_data_property_value16319 = *source_ml_source2_data_property_value16319;
                                                    source_ml_source2_data_property_value16319.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16321 = match source_ml_source2_nominal_spread16320.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16322, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value16322 = *source_ml_source2_data_property_value16322;
                                                        source_ml_source2_data_property_value16322.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16323 = match source_ml_source2_nominal_spread16320.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16324, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value16324 = *source_ml_source2_data_property_value16324;
                                                            source_ml_source2_data_property_value16324.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16325 = match source_ml_source2_nominal_spread16320.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16326, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value16326.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16327 = match source_ml_source2_nominal_spread16320.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16328, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value16328
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16329 = match source_ml_source2_nominal_spread16320.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16330, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value16330
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16331 = match source_ml_source2_nominal_spread16320.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16332, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value16332.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field16333 = match source_ml_source2_nominal_spread16320.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16334, _, _, _) => {
                                                                                source_ml_source2_data_property_value16334
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field16335 = match source_ml_source2_nominal_spread16320.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16336, _, _) => {
                                                                                    source_ml_source2_data_property_value16336.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field16337 = match source_ml_source2_nominal_spread16320.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16338, _) => {
                                                                                        source_ml_source2_data_property_value16338.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field16339 = match source_ml_source2_nominal_spread16320.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16340) => {
                                                                                            source_ml_source2_data_property_value16340.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override16341 = (value_id + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16321.clone()), Box::new(source_ml_source2_nominal_field16323.clone()), source_ml_source2_nominal_field16325.clone(), source_ml_source2_nominal_override16341, source_ml_source2_nominal_field16329, source_ml_source2_nominal_field16331.clone(), source_ml_source2_nominal_field16333, source_ml_source2_nominal_field16335.clone(), source_ml_source2_nominal_field16337.clone(), source_ml_source2_nominal_field16339.clone())
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
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16342, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value16342 = *source_ml_source2_data_property_value16342;
                                                                source_ml_source2_data_property_value16342.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16343) => {
                                                                let source_ml_source2_data_property_value16343 = *source_ml_source2_data_property_value16343;
                                                                source_ml_source2_data_property_value16343.clone()
                                                            }
                                                        })), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![raw_key.clone(), assigned.clone()])));
                                                        {
                                                            let converted = self::JavaScriptCallSource::SourcePropertyKey(Box::new(raw_key.clone()), match match next.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16344, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value16344 = *source_ml_source2_data_property_value16344;
                                                                    source_ml_source2_data_property_value16344.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16345, _, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value16345.clone()
                                                                }
                                                            });
                                                            {
                                                                let checked_key = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Null)))), Box::new(failure.clone()), Box::new(self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)))), Box::new(failure.clone()), Box::new(converted.clone()))));
                                                                {
                                                                    let write = self::JavaScriptCallSource::SourceSnapshotSet(Box::new(receiver.clone()), Box::new(checked_key.clone()), Box::new(assigned.clone()));
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16346) => {
                                                                            let source_ml_source2_data_property_value16346 = *source_ml_source2_data_property_value16346;
                                                                            source_ml_source2_data_property_value16346.clone()
                                                                        }
                                                                    }, receiver_id, match object.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16347, _) => {
                                                                            let source_ml_source2_data_property_value16347 = *source_ml_source2_data_property_value16347;
                                                                            source_ml_source2_data_property_value16347.clone()
                                                                        }
                                                                    }, self::construct_java_script_error_local_binding(match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16348) => {
                                                                            let source_ml_source2_data_property_value16348 = *source_ml_source2_data_property_value16348;
                                                                            source_ml_source2_data_property_value16348.clone()
                                                                        }
                                                                    }, key_id, match key.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16349, _) => {
                                                                            let source_ml_source2_data_property_value16349 = *source_ml_source2_data_property_value16349;
                                                                            source_ml_source2_data_property_value16349.clone()
                                                                        }
                                                                    }, self::construct_java_script_error_local_binding(match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16350) => {
                                                                            let source_ml_source2_data_property_value16350 = *source_ml_source2_data_property_value16350;
                                                                            source_ml_source2_data_property_value16350.clone()
                                                                        }
                                                                    }, value_id, match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16351, _) => {
                                                                            let source_ml_source2_data_property_value16351 = *source_ml_source2_data_property_value16351;
                                                                            source_ml_source2_data_property_value16351.clone()
                                                                        }
                                                                    }, write.clone(), false), false), false)), Box::new(match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16352) => {
                                                                            let source_ml_source2_data_property_value16352 = *source_ml_source2_data_property_value16352;
                                                                            source_ml_source2_data_property_value16352.clone()
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
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16424, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16424.clone()
                        }
                    }).borrow().clone(); ml_reference_value252 }; let ml_callback251 = { let factory = factory.clone(); std::rc::Rc::new(move |owner: self::JavaScriptSourceCapabilityOwner| -> bool { ((match owner.clone() {
                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value16426, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16426.clone()
                        }
                    }) == factory) }) }; ml_values251.iter().cloned().enumerate().find_map(|(ml_index251, ml_value251)| if ml_callback251(ml_value251.clone()) { Some(ml_value251) } else { None }) };
                    if (match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !({
                        let source_ml_source2_nullish_field_receiver16427 = (owner.clone(),);
                        if match (source_ml_source2_nullish_field_receiver16427.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present16428 = ((source_ml_source2_nullish_field_receiver16427.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present16428.clone() {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value16429) => {
                                        source_ml_source2_data_property_value16429
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
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
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceCall(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object({
                                            let source_ml_source2_nullish_field_receiver16430 = (owner.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver16430.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present16431 = ((source_ml_source2_nullish_field_receiver16430.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present16431.clone() {
                                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value16432, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value16432.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }, {
                                            let source_ml_source2_nullish_field_receiver16433 = (owner.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver16433.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present16434 = ((source_ml_source2_nullish_field_receiver16433.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present16434.clone() {
                                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value16435, _) => {
                                                            source_ml_source2_data_property_value16435
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
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
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16436, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value16436 = *source_ml_source2_data_property_value16436;
                            source_ml_source2_data_property_value16436.clone()
                        }
                    } {
                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, source_ml_source2_data_property_value16437, _, _, _, _, _, _, _, _) => {
                            ((source_ml_source2_data_property_value16437.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    };
                    if !(match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16438, _, _, _) => {
                            source_ml_source2_data_property_value16438
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
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16439) => {
                                            let source_ml_source2_data_property_value16439 = *source_ml_source2_data_property_value16439;
                                            source_ml_source2_data_property_value16439.clone()
                                        }
                                    };
                                    {
                                        let parts = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArrayPart>::new()));
                                        {
                                            let source_ml_source1_temporary_index105 = self::ml::Big::from_i128(0);
                                            {
                                                let current_4 = self::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop111(depth, current_3.clone(), parts.clone(), expression_parts.clone(), source_ml_source1_temporary_index105.clone());
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceArrayAccumulation(Box::new((constant.clone())(prototype.clone().expect("a proven present value"))), parts.clone(), Box::new(match iterator_key.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16440, _) => {
                                                        let source_ml_source2_data_property_value16440 = *source_ml_source2_data_property_value16440;
                                                        source_ml_source2_data_property_value16440.clone()
                                                    }
                                                }), match match next.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16441, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value16441 = *source_ml_source2_data_property_value16441;
                                                        source_ml_source2_data_property_value16441.clone()
                                                    }
                                                } {
                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16442, _, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value16442.clone()
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
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16443, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value16443 = *source_ml_source2_data_property_value16443;
                            source_ml_source2_data_property_value16443.clone()
                        }
                    } {
                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, source_ml_source2_data_property_value16444, _, _, _, _, _, _, _, _) => {
                            ((source_ml_source2_data_property_value16444.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    };
                    if !(match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16445, _, _, _) => {
                            source_ml_source2_data_property_value16445
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
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16446, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value16446 = *source_ml_source2_data_property_value16446;
                                                    source_ml_source2_data_property_value16446.clone()
                                                }
                                            } {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16447, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value16447.clone()
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
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16448, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value16448 = *source_ml_source2_data_property_value16448;
                                        source_ml_source2_data_property_value16448.clone()
                                    }
                                } {
                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, source_ml_source2_data_property_value16449, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value16449 = *source_ml_source2_data_property_value16449;
                                        source_ml_source2_data_property_value16449.clone()
                                    }
                                })), properties.clone(), match match next.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16450, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value16450 = *source_ml_source2_data_property_value16450;
                                        source_ml_source2_data_property_value16450.clone()
                                    }
                                } {
                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16451, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value16451.clone()
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
                                let source_ml_source2_s16452 = self::source_ml_source1_temporary_lower_java_script_error_literal_expression_loop118(depth, constant.clone(), current_9.clone(), source.clone(), parts_2.clone(), source_ml_source1_temporary_index108.clone());
                                match source_ml_source2_s16452.clone() {
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
                        let source_ml_source2_field16456 = {
                            let source_ml_source2_constructor_field16455 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16454, _) => {
                                    let source_ml_source2_data_property_value16454 = *source_ml_source2_data_property_value16454;
                                    source_ml_source2_data_property_value16454.clone()
                                }
                            };
                            self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(key.clone()), Box::new(source_ml_source2_constructor_field16455.clone()), None::<(Option<bool>,)>)
                        };
                        {
                            let source_ml_source2_field16458 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16457) => {
                                    let source_ml_source2_data_property_value16457 = *source_ml_source2_data_property_value16457;
                                    source_ml_source2_data_property_value16457.clone()
                                }
                            };
                            (source_ml_source2_field16458.clone(), source_ml_source2_field16456.clone())
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
                        let source_ml_source2_field16463 = {
                            let source_ml_source2_constructor_field16461 = match value_2.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16460, _) => {
                                    let source_ml_source2_data_property_value16460 = *source_ml_source2_data_property_value16460;
                                    source_ml_source2_data_property_value16460.clone()
                                }
                            };
                            self::JavaScriptCallSourceProperty::SourceObjectAccessor(Box::new(key_2.clone()), Box::new(source_ml_source2_constructor_field16461.clone()), kind.clone(), None::<(Option<bool>,)>)
                        };
                        {
                            let source_ml_source2_field16465 = match value_2.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16464) => {
                                    let source_ml_source2_data_property_value16464 = *source_ml_source2_data_property_value16464;
                                    source_ml_source2_data_property_value16464.clone()
                                }
                            };
                            (source_ml_source2_field16465.clone(), source_ml_source2_field16463.clone())
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
                    let source_ml_source2_field16467 = self::JavaScriptCallSourceProperty::SourceLiteralPrototype(Box::new(match value_3.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16466, _) => {
                            let source_ml_source2_data_property_value16466 = *source_ml_source2_data_property_value16466;
                            source_ml_source2_data_property_value16466.clone()
                        }
                    }));
                    {
                        let source_ml_source2_field16469 = match value_3.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16468) => {
                                let source_ml_source2_data_property_value16468 = *source_ml_source2_data_property_value16468;
                                source_ml_source2_data_property_value16468.clone()
                            }
                        };
                        (source_ml_source2_field16469.clone(), source_ml_source2_field16467.clone())
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
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16470) => {
                            let source_ml_source2_data_property_value16470 = *source_ml_source2_data_property_value16470;
                            source_ml_source2_data_property_value16470.clone()
                        }
                    }, depth);
                    {
                        let kind_2 = (((source_kind.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        if (kind_2 != (self::ml_string::Text::from_units(vec![]))) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field16482 = if match ((infer_name.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                    {
                                        let source_ml_source2_constructor_field16472 = match key_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16471, _) => {
                                                let source_ml_source2_data_property_value16471 = *source_ml_source2_data_property_value16471;
                                                source_ml_source2_data_property_value16471.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16474 = match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16473, _) => {
                                                    let source_ml_source2_data_property_value16473 = *source_ml_source2_data_property_value16473;
                                                    source_ml_source2_data_property_value16473.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field16476 = Some(true);
                                                self::JavaScriptCallSourceProperty::SourceObjectAccessor(Box::new(source_ml_source2_constructor_field16472.clone()), Box::new(source_ml_source2_constructor_field16474.clone()), kind_2.clone(), Some((source_ml_source2_constructor_field16476.clone(),)))
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let source_ml_source2_constructor_field16478 = match key_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16477, _) => {
                                                let source_ml_source2_data_property_value16477 = *source_ml_source2_data_property_value16477;
                                                source_ml_source2_data_property_value16477.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16480 = match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16479, _) => {
                                                    let source_ml_source2_data_property_value16479 = *source_ml_source2_data_property_value16479;
                                                    source_ml_source2_data_property_value16479.clone()
                                                }
                                            };
                                            self::JavaScriptCallSourceProperty::SourceObjectAccessor(Box::new(source_ml_source2_constructor_field16478.clone()), Box::new(source_ml_source2_constructor_field16480.clone()), kind_2.clone(), None::<(Option<bool>,)>)
                                        }
                                    }
                                };
                                {
                                    let source_ml_source2_field16484 = match value_4.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16483) => {
                                            let source_ml_source2_data_property_value16483 = *source_ml_source2_data_property_value16483;
                                            source_ml_source2_data_property_value16483.clone()
                                        }
                                    };
                                    (source_ml_source2_field16484.clone(), source_ml_source2_field16482.clone())
                                }
                            }))
                        } else {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field16494 = if match ((infer_name.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                    {
                                        let source_ml_source2_constructor_field16486 = match key_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16485, _) => {
                                                let source_ml_source2_data_property_value16485 = *source_ml_source2_data_property_value16485;
                                                source_ml_source2_data_property_value16485.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16488 = match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16487, _) => {
                                                    let source_ml_source2_data_property_value16487 = *source_ml_source2_data_property_value16487;
                                                    source_ml_source2_data_property_value16487.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field16489 = Some(true);
                                                self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field16486.clone()), Box::new(source_ml_source2_constructor_field16488.clone()), Some((source_ml_source2_constructor_field16489.clone(),)))
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let source_ml_source2_constructor_field16491 = match key_3.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16490, _) => {
                                                let source_ml_source2_data_property_value16490 = *source_ml_source2_data_property_value16490;
                                                source_ml_source2_data_property_value16490.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16493 = match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16492, _) => {
                                                    let source_ml_source2_data_property_value16492 = *source_ml_source2_data_property_value16492;
                                                    source_ml_source2_data_property_value16492.clone()
                                                }
                                            };
                                            self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field16491.clone()), Box::new(source_ml_source2_constructor_field16493.clone()), None::<(Option<bool>,)>)
                                        }
                                    }
                                };
                                {
                                    let source_ml_source2_field16496 = match value_4.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16495) => {
                                            let source_ml_source2_data_property_value16495 = *source_ml_source2_data_property_value16495;
                                            source_ml_source2_data_property_value16495.clone()
                                        }
                                    };
                                    (source_ml_source2_field16496.clone(), source_ml_source2_field16494.clone())
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
                    let source_ml_source2_field16499 = {
                        let source_ml_source2_constructor_field16498 = match input.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16497, _) => {
                                let source_ml_source2_data_property_value16497 = *source_ml_source2_data_property_value16497;
                                source_ml_source2_data_property_value16497.clone()
                            }
                        };
                        self::JavaScriptCallSourceProperty::SourceObjectSpread(Box::new(source_ml_source2_constructor_field16498.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>>,)>)
                    };
                    {
                        let source_ml_source2_field16501 = match input.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16500) => {
                                let source_ml_source2_data_property_value16500 = *source_ml_source2_data_property_value16500;
                                source_ml_source2_data_property_value16500.clone()
                            }
                        };
                        (source_ml_source2_field16501.clone(), source_ml_source2_field16499.clone())
                    }
                }))
            }
        }
    }
}
