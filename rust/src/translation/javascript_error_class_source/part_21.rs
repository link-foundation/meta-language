// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallSource;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
use super::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_ordering_comparison;
use super::lower_java_script_error_expression;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::refuse_java_script_error_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_binary(expression: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match expression.clone() {
        self::JavaScriptErrorExpression::ErrorBinary(expression_operator, expression_left, expression_right, port) => {
            let expression_left = *expression_left;
            let expression_right = *expression_right;
            {
                let left = self::lower_java_script_error_expression(expression_left.clone(), context.clone(), (depth + 1f64));
                {
                    let first = match match left.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16577) => {
                            let source_ml_source2_data_property_value16577 = *source_ml_source2_data_property_value16577;
                            source_ml_source2_data_property_value16577.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16578, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16578
                        }
                    };
                    {
                        let right = self::lower_java_script_error_expression(expression_right.clone(), {
                            let source_ml_source2_nominal_spread16580 = match left.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16579) => {
                                    let source_ml_source2_data_property_value16579 = *source_ml_source2_data_property_value16579;
                                    source_ml_source2_data_property_value16579.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field16581 = match source_ml_source2_nominal_spread16580.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16582, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value16582 = *source_ml_source2_data_property_value16582;
                                        source_ml_source2_data_property_value16582.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field16583 = match source_ml_source2_nominal_spread16580.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16584, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value16584 = *source_ml_source2_data_property_value16584;
                                            source_ml_source2_data_property_value16584.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field16585 = match source_ml_source2_nominal_spread16580.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16586, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value16586.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field16587 = match source_ml_source2_nominal_spread16580.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16588, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value16588
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16589 = match source_ml_source2_nominal_spread16580.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16590, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value16590
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16591 = match source_ml_source2_nominal_spread16580.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16592, _, _, _, _) => {
                                                            source_ml_source2_data_property_value16592.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16593 = match source_ml_source2_nominal_spread16580.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16594, _, _, _) => {
                                                                source_ml_source2_data_property_value16594
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16595 = match source_ml_source2_nominal_spread16580.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16596, _, _) => {
                                                                    source_ml_source2_data_property_value16596.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16597 = match source_ml_source2_nominal_spread16580.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16598, _) => {
                                                                        source_ml_source2_data_property_value16598.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16599 = match source_ml_source2_nominal_spread16580.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16600) => {
                                                                            source_ml_source2_data_property_value16600.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override16601 = (first + 1f64);
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16581.clone()), Box::new(source_ml_source2_nominal_field16583.clone()), source_ml_source2_nominal_field16585.clone(), source_ml_source2_nominal_override16601, source_ml_source2_nominal_field16589, source_ml_source2_nominal_field16591.clone(), source_ml_source2_nominal_field16593, source_ml_source2_nominal_field16595.clone(), source_ml_source2_nominal_field16597.clone(), source_ml_source2_nominal_field16599.clone())
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
                            let second = match match right.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16602) => {
                                    let source_ml_source2_data_property_value16602 = *source_ml_source2_data_property_value16602;
                                    source_ml_source2_data_property_value16602.clone()
                                }
                            } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16603, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value16603
                                }
                            };
                            {
                                let next = {
                                    let source_ml_source2_nominal_spread16605 = match right.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16604) => {
                                            let source_ml_source2_data_property_value16604 = *source_ml_source2_data_property_value16604;
                                            source_ml_source2_data_property_value16604.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field16606 = match source_ml_source2_nominal_spread16605.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16607, _, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value16607 = *source_ml_source2_data_property_value16607;
                                                source_ml_source2_data_property_value16607.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field16608 = match source_ml_source2_nominal_spread16605.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16609, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value16609 = *source_ml_source2_data_property_value16609;
                                                    source_ml_source2_data_property_value16609.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16610 = match source_ml_source2_nominal_spread16605.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16611, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value16611.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16612 = match source_ml_source2_nominal_spread16605.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16613, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value16613
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16614 = match source_ml_source2_nominal_spread16605.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16615, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value16615
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16616 = match source_ml_source2_nominal_spread16605.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16617, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value16617.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16618 = match source_ml_source2_nominal_spread16605.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16619, _, _, _) => {
                                                                        source_ml_source2_data_property_value16619
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16620 = match source_ml_source2_nominal_spread16605.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16621, _, _) => {
                                                                            source_ml_source2_data_property_value16621.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field16622 = match source_ml_source2_nominal_spread16605.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16623, _) => {
                                                                                source_ml_source2_data_property_value16623.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field16624 = match source_ml_source2_nominal_spread16605.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16625) => {
                                                                                    source_ml_source2_data_property_value16625.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override16626 = (second + 1f64);
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16606.clone()), Box::new(source_ml_source2_nominal_field16608.clone()), source_ml_source2_nominal_field16610.clone(), source_ml_source2_nominal_override16626, source_ml_source2_nominal_field16614, source_ml_source2_nominal_field16616.clone(), source_ml_source2_nominal_field16618, source_ml_source2_nominal_field16620.clone(), source_ml_source2_nominal_field16622.clone(), source_ml_source2_nominal_field16624.clone())
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
                                    let a = self::JavaScriptCallSource::SourceBinding(first);
                                    {
                                        let b = self::JavaScriptCallSource::SourceBinding(second);
                                        {
                                            let constant = std::rc::Rc::new(self::construct_java_script_error_constant);
                                            if (expression_operator == (self::ml_string::Text::from_units(vec![44]))) {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), first, match left.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16627, _) => {
                                                        let source_ml_source2_data_property_value16627 = *source_ml_source2_data_property_value16627;
                                                        source_ml_source2_data_property_value16627.clone()
                                                    }
                                                }, match right.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16628, _) => {
                                                        let source_ml_source2_data_property_value16628 = *source_ml_source2_data_property_value16628;
                                                        source_ml_source2_data_property_value16628.clone()
                                                    }
                                                }, false)), Box::new(next.clone()))
                                            } else {
                                                if (expression_operator == (self::ml_string::Text::from_units(vec![63, 63]))) {
                                                    {
                                                        let nullish = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(a.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Null)))), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(true))), Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(a.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)))));
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), first, match left.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16629, _) => {
                                                                let source_ml_source2_data_property_value16629 = *source_ml_source2_data_property_value16629;
                                                                source_ml_source2_data_property_value16629.clone()
                                                            }
                                                        }, self::JavaScriptCallSource::SourceIf(Box::new(nullish.clone()), Box::new(match right.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16630, _) => {
                                                                let source_ml_source2_data_property_value16630 = *source_ml_source2_data_property_value16630;
                                                                source_ml_source2_data_property_value16630.clone()
                                                            }
                                                        }), Box::new(a.clone())), false)), Box::new(next.clone()))
                                                    }
                                                } else {
                                                    if ((expression_operator == (self::ml_string::Text::from_units(vec![38, 38]))) || (expression_operator == (self::ml_string::Text::from_units(vec![124, 124])))) {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), first, match left.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16631, _) => {
                                                                let source_ml_source2_data_property_value16631 = *source_ml_source2_data_property_value16631;
                                                                source_ml_source2_data_property_value16631.clone()
                                                            }
                                                        }, self::JavaScriptCallSource::SourceIf(Box::new(a.clone()), Box::new(if (expression_operator == (self::ml_string::Text::from_units(vec![38, 38]))) {
                                                            match right.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16632, _) => {
                                                                    let source_ml_source2_data_property_value16632 = *source_ml_source2_data_property_value16632;
                                                                    source_ml_source2_data_property_value16632.clone()
                                                                }
                                                            }
                                                        } else {
                                                            a.clone()
                                                        }), Box::new(if (expression_operator == (self::ml_string::Text::from_units(vec![38, 38]))) {
                                                            a.clone()
                                                        } else {
                                                            match right.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16633, _) => {
                                                                    let source_ml_source2_data_property_value16633 = *source_ml_source2_data_property_value16633;
                                                                    source_ml_source2_data_property_value16633.clone()
                                                                }
                                                            }
                                                        })), false)), Box::new(next.clone()))
                                                    } else {
                                                        if (match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16634, _, _, _) => {
                                                                source_ml_source2_data_property_value16634
                                                            }
                                                        } && self::ml_array::includes(&{ let ml_reference_value253 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![43]), self::ml_string::Text::from_units(vec![45]), self::ml_string::Text::from_units(vec![42]), self::ml_string::Text::from_units(vec![47]), self::ml_string::Text::from_units(vec![37]), self::ml_string::Text::from_units(vec![42, 42])]))).borrow().clone(); ml_reference_value253 }, &expression_operator.clone(), 0f64, false, |a, b| a == b)) {
                                                            {
                                                                let first_converted = match next.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16635, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value16635
                                                                    }
                                                                };
                                                                {
                                                                    let second_converted = (first_converted + 1f64);
                                                                    {
                                                                        let complete = {
                                                                            let source_ml_source2_nominal_field16637 = match next.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16638, _, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value16638 = *source_ml_source2_data_property_value16638;
                                                                                    source_ml_source2_data_property_value16638.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field16639 = match next.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16640, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value16640 = *source_ml_source2_data_property_value16640;
                                                                                        source_ml_source2_data_property_value16640.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field16641 = match next.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16642, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value16642.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16643 = match next.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16644, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value16644
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16645 = match next.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16646, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value16646
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16647 = match next.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16648, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value16648.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16649 = match next.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16650, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value16650
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16651 = match next.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16652, _, _) => {
                                                                                                                source_ml_source2_data_property_value16652.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16653 = match next.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16654, _) => {
                                                                                                                    source_ml_source2_data_property_value16654.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16655 = match next.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16656) => {
                                                                                                                        source_ml_source2_data_property_value16656.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override16657 = (second_converted + 1f64);
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16637.clone()), Box::new(source_ml_source2_nominal_field16639.clone()), source_ml_source2_nominal_field16641.clone(), source_ml_source2_nominal_override16657, source_ml_source2_nominal_field16645, source_ml_source2_nominal_field16647.clone(), source_ml_source2_nominal_field16649, source_ml_source2_nominal_field16651.clone(), source_ml_source2_nominal_field16653.clone(), source_ml_source2_nominal_field16655.clone())
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
                                                                            let convert = if (expression_operator == (self::ml_string::Text::from_units(vec![43]))) {
                                                                                self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99])
                                                                            };
                                                                            {
                                                                                let operation = if (expression_operator == (self::ml_string::Text::from_units(vec![43]))) {
                                                                                    self::ml_string::Text::from_units(vec![97, 100, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101, 115])
                                                                                } else {
                                                                                    if (expression_operator == (self::ml_string::Text::from_units(vec![45]))) {
                                                                                        self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115])
                                                                                    } else {
                                                                                        if (expression_operator == (self::ml_string::Text::from_units(vec![42]))) {
                                                                                            self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115])
                                                                                        } else {
                                                                                            if (expression_operator == (self::ml_string::Text::from_units(vec![47]))) {
                                                                                                self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115])
                                                                                            } else {
                                                                                                if (expression_operator == (self::ml_string::Text::from_units(vec![37]))) {
                                                                                                    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])
                                                                                                } else {
                                                                                                    self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111, 119, 101, 114])
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let converted_a = self::JavaScriptCallSource::SourceBinding(first_converted);
                                                                                    {
                                                                                        let converted_b = self::JavaScriptCallSource::SourceBinding(second_converted);
                                                                                        {
                                                                                            let body = self::construct_java_script_error_local_binding(complete.clone(), first_converted, self::JavaScriptCallSource::SourcePrimitive(convert.clone(), std::rc::Rc::new(std::cell::RefCell::new(vec![a.clone()]))), self::construct_java_script_error_local_binding(complete.clone(), second_converted, self::JavaScriptCallSource::SourcePrimitive(convert.clone(), std::rc::Rc::new(std::cell::RefCell::new(vec![b.clone()]))), self::JavaScriptCallSource::SourcePrimitive(operation.clone(), std::rc::Rc::new(std::cell::RefCell::new(vec![converted_a.clone(), converted_b.clone()]))), false), false);
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(complete.clone(), first, match left.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16658, _) => {
                                                                                                    let source_ml_source2_data_property_value16658 = *source_ml_source2_data_property_value16658;
                                                                                                    source_ml_source2_data_property_value16658.clone()
                                                                                                }
                                                                                            }, self::construct_java_script_error_local_binding(complete.clone(), second, match right.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16659, _) => {
                                                                                                    let source_ml_source2_data_property_value16659 = *source_ml_source2_data_property_value16659;
                                                                                                    source_ml_source2_data_property_value16659.clone()
                                                                                                }
                                                                                            }, body.clone(), false), false)), Box::new(complete.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if (match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16660, _, _, _) => {
                                                                    source_ml_source2_data_property_value16660
                                                                }
                                                            } && self::ml_array::includes(&{ let ml_reference_value254 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![60]), self::ml_string::Text::from_units(vec![60, 61]), self::ml_string::Text::from_units(vec![62]), self::ml_string::Text::from_units(vec![62, 61])]))).borrow().clone(); ml_reference_value254 }, &expression_operator.clone(), 0f64, false, |a, b| a == b)) {
                                                                {
                                                                    let first_converted_2 = match next.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16661, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value16661
                                                                        }
                                                                    };
                                                                    {
                                                                        let second_converted_2 = (first_converted_2 + 1f64);
                                                                        {
                                                                            let comparison = (second_converted_2 + 1f64);
                                                                            {
                                                                                let complete_2 = {
                                                                                    let source_ml_source2_nominal_field16663 = match next.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16664, _, _, _, _, _, _, _, _, _) => {
                                                                                            let source_ml_source2_data_property_value16664 = *source_ml_source2_data_property_value16664;
                                                                                            source_ml_source2_data_property_value16664.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16665 = match next.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16666, _, _, _, _, _, _, _, _) => {
                                                                                                let source_ml_source2_data_property_value16666 = *source_ml_source2_data_property_value16666;
                                                                                                source_ml_source2_data_property_value16666.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16667 = match next.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16668, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value16668.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16669 = match next.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16670, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value16670
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16671 = match next.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16672, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value16672
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16673 = match next.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16674, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value16674.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16675 = match next.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16676, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value16676
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16677 = match next.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16678, _, _) => {
                                                                                                                        source_ml_source2_data_property_value16678.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16679 = match next.clone() {
                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16680, _) => {
                                                                                                                            source_ml_source2_data_property_value16680.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16681 = match next.clone() {
                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16682) => {
                                                                                                                                source_ml_source2_data_property_value16682.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override16683 = (comparison + 1f64);
                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16663.clone()), Box::new(source_ml_source2_nominal_field16665.clone()), source_ml_source2_nominal_field16667.clone(), source_ml_source2_nominal_override16683, source_ml_source2_nominal_field16671, source_ml_source2_nominal_field16673.clone(), source_ml_source2_nominal_field16675, source_ml_source2_nominal_field16677.clone(), source_ml_source2_nominal_field16679.clone(), source_ml_source2_nominal_field16681.clone())
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
                                                                                    let ordering = self::JavaScriptCallSource::SourceBinding(comparison);
                                                                                    {
                                                                                        let chosen = if (expression_operator == (self::ml_string::Text::from_units(vec![60]))) {
                                                                                            self::construct_java_script_source_ordering_comparison(ordering.clone(), (-1f64))
                                                                                        } else {
                                                                                            if (expression_operator == (self::ml_string::Text::from_units(vec![62]))) {
                                                                                                self::construct_java_script_source_ordering_comparison(ordering.clone(), 1f64)
                                                                                            } else {
                                                                                                self::JavaScriptCallSource::SourceIf(Box::new(self::construct_java_script_source_ordering_comparison(ordering.clone(), 0f64)), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(true))), Box::new(self::construct_java_script_source_ordering_comparison(ordering.clone(), if (expression_operator == (self::ml_string::Text::from_units(vec![60, 61]))) {
                                                                                                    (-1f64)
                                                                                                } else {
                                                                                                    1f64
                                                                                                })))
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let compared = self::construct_java_script_error_local_binding(complete_2.clone(), comparison, self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101, 115]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSource::SourceBinding(first_converted_2), self::JavaScriptCallSource::SourceBinding(second_converted_2)]))), chosen.clone(), false);
                                                                                            {
                                                                                                let converted = self::construct_java_script_error_local_binding(complete_2.clone(), first_converted_2, self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 110, 117, 109, 98, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![a.clone()]))), self::construct_java_script_error_local_binding(complete_2.clone(), second_converted_2, self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 110, 117, 109, 98, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![b.clone()]))), compared.clone(), false), false);
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(complete_2.clone(), first, match left.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16684, _) => {
                                                                                                        let source_ml_source2_data_property_value16684 = *source_ml_source2_data_property_value16684;
                                                                                                        source_ml_source2_data_property_value16684.clone()
                                                                                                    }
                                                                                                }, self::construct_java_script_error_local_binding(complete_2.clone(), second, match right.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16685, _) => {
                                                                                                        let source_ml_source2_data_property_value16685 = *source_ml_source2_data_property_value16685;
                                                                                                        source_ml_source2_data_property_value16685.clone()
                                                                                                    }
                                                                                                }, converted.clone(), false), false)), Box::new(complete_2.clone()))
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
                                                                    let body_2 = (constant.clone())(self::JavaScriptSemanticValue::Undefined);
                                                                    {
                                                                        let source_ml_source2_s16696 = if (expression_operator == (self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102]))) {
                                                                            {
                                                                                let owner = { let ml_values255 = { let ml_reference_value256 = (match context.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16697, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value16697.clone()
                                                                                    }
                                                                                }).borrow().clone(); ml_reference_value256 }; let ml_callback255 = { let port = port.clone(); std::rc::Rc::new(move |owner: self::JavaScriptSourceCapabilityOwner| -> bool { ((match owner.clone() {
                                                                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value16699, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value16699.clone()
                                                                                    }
                                                                                }) == port) }) }; ml_values255.iter().cloned().enumerate().find_map(|(ml_index255, ml_value255)| if ml_callback255(ml_value255.clone()) { Some(ml_value255) } else { None }) };
                                                                                if (match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !({
                                                                                    let source_ml_source2_nullish_field_receiver16700 = (owner.clone(),);
                                                                                    if match (source_ml_source2_nullish_field_receiver16700.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        {
                                                                                            let source_ml_source2_nullish_field_present16701 = ((source_ml_source2_nullish_field_receiver16700.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source2_nullish_field_present16701.clone() {
                                                                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value16702) => {
                                                                                                    source_ml_source2_data_property_value16702
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                                                    }
                                                                                })) {
                                                                                    self::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Return(Box::new(self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 47, 111, 112, 101, 114, 97, 116, 111, 114, 32, 111, 119, 110, 101, 114])))))
                                                                                } else {
                                                                                    {
                                                                                        let body_3 = self::JavaScriptCallSource::SourceCall(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object({
                                                                                            let source_ml_source2_nullish_field_receiver16703 = (owner.clone(),);
                                                                                            if match (source_ml_source2_nullish_field_receiver16703.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                {
                                                                                                    let source_ml_source2_nullish_field_present16704 = ((source_ml_source2_nullish_field_receiver16703.clone()).0).clone().expect("a proven present value");
                                                                                                    match source_ml_source2_nullish_field_present16704.clone() {
                                                                                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value16705, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value16705.clone()
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                                                                            }
                                                                                        }, true))), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)), std::rc::Rc::new(std::cell::RefCell::new(vec![a.clone(), b.clone()])));
                                                                                        self::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Next(Box::new(body_3.clone()))
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if ((expression_operator == (self::ml_string::Text::from_units(vec![43]))) && match context.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16706, _, _, _) => {
                                                                                    source_ml_source2_data_property_value16706
                                                                                }
                                                                            }) {
                                                                                {
                                                                                    let body_4 = self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![97, 100, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101, 115]), std::rc::Rc::new(std::cell::RefCell::new(vec![a.clone(), b.clone()])));
                                                                                    self::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Next(Box::new(body_4.clone()))
                                                                                }
                                                                            } else {
                                                                                if self::ml_array::includes(&{ let ml_reference_value257 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![42]), self::ml_string::Text::from_units(vec![47]), self::ml_string::Text::from_units(vec![45]), self::ml_string::Text::from_units(vec![37]), self::ml_string::Text::from_units(vec![43])]))).borrow().clone(); ml_reference_value257 }, &expression_operator.clone(), 0f64, false, |a, b| a == b) {
                                                                                    {
                                                                                        let operator = if (expression_operator == (self::ml_string::Text::from_units(vec![42]))) {
                                                                                            self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121])
                                                                                        } else {
                                                                                            if (expression_operator == (self::ml_string::Text::from_units(vec![47]))) {
                                                                                                self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101])
                                                                                            } else {
                                                                                                if (expression_operator == (self::ml_string::Text::from_units(vec![45]))) {
                                                                                                    self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116])
                                                                                                } else {
                                                                                                    if (expression_operator == (self::ml_string::Text::from_units(vec![37]))) {
                                                                                                        self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![97, 100, 100])
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let body_5 = self::JavaScriptCallSource::SourceNumberBinary(operator.clone(), Box::new(a.clone()), Box::new(b.clone()));
                                                                                            self::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Next(Box::new(body_5.clone()))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    if self::ml_array::includes(&{ let ml_reference_value258 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![60]), self::ml_string::Text::from_units(vec![60, 61]), self::ml_string::Text::from_units(vec![62]), self::ml_string::Text::from_units(vec![62, 61])]))).borrow().clone(); ml_reference_value258 }, &expression_operator.clone(), 0f64, false, |a, b| a == b) {
                                                                                        {
                                                                                            let swapped = ((expression_operator == (self::ml_string::Text::from_units(vec![62]))) || (expression_operator == (self::ml_string::Text::from_units(vec![62, 61]))));
                                                                                            {
                                                                                                let less = self::JavaScriptCallSource::SourceNumberBinary(self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), Box::new(if swapped {
                                                                                                    b.clone()
                                                                                                } else {
                                                                                                    a.clone()
                                                                                                }), Box::new(if swapped {
                                                                                                    a.clone()
                                                                                                } else {
                                                                                                    b.clone()
                                                                                                }));
                                                                                                {
                                                                                                    let body_6 = if ((expression_operator == (self::ml_string::Text::from_units(vec![60]))) || (expression_operator == (self::ml_string::Text::from_units(vec![62])))) {
                                                                                                        less.clone()
                                                                                                    } else {
                                                                                                        self::JavaScriptCallSource::SourceIf(Box::new(less.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(true))), Box::new(self::JavaScriptCallSource::SourceNumberBinary(self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108]), Box::new(a.clone()), Box::new(b.clone()))))
                                                                                                    };
                                                                                                    self::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Next(Box::new(body_6.clone()))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        self::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Return(Box::new(self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 97, 114, 121, 32, 111, 112, 101, 114, 97, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 112, 114, 111, 102, 105, 108, 101])))))
                                                                                    }
                                                                                }
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s16696.clone() {
                                                                            self::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Next(body_7) => {
                                                                                let body_7 = *body_7;
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), first, match left.clone() {
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16707, _) => {
                                                                                        let source_ml_source2_data_property_value16707 = *source_ml_source2_data_property_value16707;
                                                                                        source_ml_source2_data_property_value16707.clone()
                                                                                    }
                                                                                }, self::construct_java_script_error_local_binding(next.clone(), second, match right.clone() {
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16708, _) => {
                                                                                        let source_ml_source2_data_property_value16708 = *source_ml_source2_data_property_value16708;
                                                                                        source_ml_source2_data_property_value16708.clone()
                                                                                    }
                                                                                }, body_7.clone(), false), false)), Box::new(next.clone()))
                                                                            }
                                                                            self::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119::MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Return(source_ml_source1_temporary_result119) => {
                                                                                let source_ml_source1_temporary_result119 = *source_ml_source1_temporary_result119;
                                                                                source_ml_source1_temporary_result119.clone()
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
                    }
                }
            }
        }
        _ => {
            self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121, 32, 115, 111, 117, 114, 99, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 111, 114, 32, 110, 111, 100, 101])))
        }
    }
}
