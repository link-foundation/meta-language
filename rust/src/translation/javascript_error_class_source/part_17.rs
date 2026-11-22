// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallSource;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorFunctionOwner;
use super::JavaScriptErrorModuleRegistry;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
use super::JavaScriptSourceLexicalBinding;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_lexical_cell_read;
use super::construct_java_script_source_field_key;
use super::lower_java_script_error_construction_expression;
use super::lower_java_script_error_literal_expression;
use super::lower_java_script_error_property_expression;
use super::lower_java_script_lexical_cell_write;
use super::lower_java_script_source_binary;
use super::lower_java_script_source_compound_assignment;
use super::lower_java_script_source_delete;
use super::lower_java_script_source_presence;
use super::lower_java_script_source_update;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::refuse_java_script_error_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_error_expression(expression: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    {
        let next = {
            let source_ml_source2_nominal_field15279 = match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15280, _, _, _, _, _, _, _, _, _) => {
                    let source_ml_source2_data_property_value15280 = *source_ml_source2_data_property_value15280;
                    source_ml_source2_data_property_value15280.clone()
                }
            };
            {
                let source_ml_source2_nominal_field15281 = match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15282, _, _, _, _, _, _, _, _) => {
                        let source_ml_source2_data_property_value15282 = *source_ml_source2_data_property_value15282;
                        source_ml_source2_data_property_value15282.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field15283 = match context.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15284, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15284.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field15285 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15286, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15286
                            }
                        };
                        {
                            let source_ml_source2_nominal_field15287 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15288, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value15288
                                }
                            };
                            {
                                let source_ml_source2_nominal_field15289 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15290, _, _, _, _) => {
                                        source_ml_source2_data_property_value15290.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field15291 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15292, _, _, _) => {
                                            source_ml_source2_data_property_value15292
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field15293 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15294, _, _) => {
                                                source_ml_source2_data_property_value15294.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15295 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15296, _) => {
                                                    source_ml_source2_data_property_value15296.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field15297 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15298) => {
                                                        source_ml_source2_data_property_value15298.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override15299 = (match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15277, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value15277
                                                        }
                                                    } + 1f64);
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15279.clone()), Box::new(source_ml_source2_nominal_field15281.clone()), source_ml_source2_nominal_field15283.clone(), source_ml_source2_nominal_field15285, source_ml_source2_nominal_override15299, source_ml_source2_nominal_field15289.clone(), source_ml_source2_nominal_field15291, source_ml_source2_nominal_field15293.clone(), source_ml_source2_nominal_field15295.clone(), source_ml_source2_nominal_field15297.clone())
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
        if ((match next.clone() {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15301, _, _, _, _) => {
                source_ml_source2_data_property_value15301.clone()
            }
        }) != (self::ml_string::Text::from_units(vec![]))) {
            self::refuse_java_script_error_source(next.clone(), &(match next.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15302, _, _, _, _) => {
                    source_ml_source2_data_property_value15302.clone()
                }
            }))
        } else {
            if ((depth > (128f64)) || ((match next.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15304, _, _, _, _, _) => {
                    source_ml_source2_data_property_value15304
                }
            }) > (65536f64))) {
                self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 32, 98, 117, 100, 103, 101, 116])))
            } else {
                {
                    let constant = std::rc::Rc::new(self::construct_java_script_error_constant);
                    match expression.clone() {
                        self::JavaScriptErrorExpression::ErrorConstant(value) => {
                            let value = *value;
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new((constant.clone())(value.clone())), Box::new(next.clone()))
                        }
                        self::JavaScriptErrorExpression::ErrorClosure(_, _, _, _) => {
                            self::lower_java_script_error_construction_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorArgumentSpread(expression_value, iterator_key) => {
                            let expression_value = *expression_value;
                            let iterator_key = *iterator_key;
                            {
                                let value_2 = self::lower_java_script_error_expression(expression_value.clone(), next.clone(), (depth + 1f64));
                                {
                                    let key = self::lower_java_script_error_expression(iterator_key.clone(), match value_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15359) => {
                                            let source_ml_source2_data_property_value15359 = *source_ml_source2_data_property_value15359;
                                            source_ml_source2_data_property_value15359.clone()
                                        }
                                    }, (depth + 1f64));
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceArgumentSpread(Box::new(match value_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15360, _) => {
                                            let source_ml_source2_data_property_value15360 = *source_ml_source2_data_property_value15360;
                                            source_ml_source2_data_property_value15360.clone()
                                        }
                                    }), Box::new(match key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15361, _) => {
                                            let source_ml_source2_data_property_value15361 = *source_ml_source2_data_property_value15361;
                                            source_ml_source2_data_property_value15361.clone()
                                        }
                                    }), match match next.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15362, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value15362 = *source_ml_source2_data_property_value15362;
                                            source_ml_source2_data_property_value15362.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value15363, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value15363.clone()
                                        }
                                    })), Box::new(match key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15364) => {
                                            let source_ml_source2_data_property_value15364 = *source_ml_source2_data_property_value15364;
                                            source_ml_source2_data_property_value15364.clone()
                                        }
                                    }))
                                }
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorInstanceOf(expression_left, expression_right, expression_key) => {
                            let expression_left = *expression_left;
                            let expression_right = *expression_right;
                            let expression_key = *expression_key;
                            {
                                let left = self::lower_java_script_error_expression(expression_left.clone(), next.clone(), (depth + 1f64));
                                {
                                    let right = self::lower_java_script_error_expression(expression_right.clone(), match left.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15365) => {
                                            let source_ml_source2_data_property_value15365 = *source_ml_source2_data_property_value15365;
                                            source_ml_source2_data_property_value15365.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let key_2 = self::lower_java_script_error_expression(expression_key.clone(), match right.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15366) => {
                                                let source_ml_source2_data_property_value15366 = *source_ml_source2_data_property_value15366;
                                                source_ml_source2_data_property_value15366.clone()
                                            }
                                        }, (depth + 1f64));
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceInstanceOf(Box::new(match left.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15367, _) => {
                                                let source_ml_source2_data_property_value15367 = *source_ml_source2_data_property_value15367;
                                                source_ml_source2_data_property_value15367.clone()
                                            }
                                        }), Box::new(match right.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15368, _) => {
                                                let source_ml_source2_data_property_value15368 = *source_ml_source2_data_property_value15368;
                                                source_ml_source2_data_property_value15368.clone()
                                            }
                                        }), Box::new(match key_2.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15369, _) => {
                                                let source_ml_source2_data_property_value15369 = *source_ml_source2_data_property_value15369;
                                                source_ml_source2_data_property_value15369.clone()
                                            }
                                        }))), Box::new(match key_2.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15370) => {
                                                let source_ml_source2_data_property_value15370 = *source_ml_source2_data_property_value15370;
                                                source_ml_source2_data_property_value15370.clone()
                                            }
                                        }))
                                    }
                                }
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorDelete(_) => {
                            self::lower_java_script_source_delete(expression.clone(), next.clone(), (depth + 1f64))
                        }
                        self::JavaScriptErrorExpression::ErrorIn(_, _) => {
                            self::lower_java_script_source_presence(expression.clone(), next.clone(), (depth + 1f64))
                        }
                        self::JavaScriptErrorExpression::ErrorBinding(identity, _) => {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceBinding(identity)), Box::new(next.clone()))
                        }
                        self::JavaScriptErrorExpression::ErrorModuleBinding(identity_2) => {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceBinding(identity_2)), Box::new(next.clone()))
                        }
                        self::JavaScriptErrorExpression::ErrorThis => {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceDerivedReceiver), Box::new(next.clone()))
                        }
                        self::JavaScriptErrorExpression::ErrorConditional(expression_condition, expression_yes, expression_no) => {
                            let expression_condition = *expression_condition;
                            let expression_yes = *expression_yes;
                            let expression_no = *expression_no;
                            {
                                let condition = self::lower_java_script_error_expression(expression_condition.clone(), next.clone(), (depth + 1f64));
                                {
                                    let yes = self::lower_java_script_error_expression(expression_yes.clone(), match condition.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15371) => {
                                            let source_ml_source2_data_property_value15371 = *source_ml_source2_data_property_value15371;
                                            source_ml_source2_data_property_value15371.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let no = self::lower_java_script_error_expression(expression_no.clone(), match yes.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15372) => {
                                                let source_ml_source2_data_property_value15372 = *source_ml_source2_data_property_value15372;
                                                source_ml_source2_data_property_value15372.clone()
                                            }
                                        }, (depth + 1f64));
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceIf(Box::new(match condition.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15373, _) => {
                                                let source_ml_source2_data_property_value15373 = *source_ml_source2_data_property_value15373;
                                                source_ml_source2_data_property_value15373.clone()
                                            }
                                        }), Box::new(match yes.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15374, _) => {
                                                let source_ml_source2_data_property_value15374 = *source_ml_source2_data_property_value15374;
                                                source_ml_source2_data_property_value15374.clone()
                                            }
                                        }), Box::new(match no.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15375, _) => {
                                                let source_ml_source2_data_property_value15375 = *source_ml_source2_data_property_value15375;
                                                source_ml_source2_data_property_value15375.clone()
                                            }
                                        }))), Box::new(match no.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15376) => {
                                                let source_ml_source2_data_property_value15376 = *source_ml_source2_data_property_value15376;
                                                source_ml_source2_data_property_value15376.clone()
                                            }
                                        }))
                                    }
                                }
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorModuleCell(identity_3) => {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_lexical_cell_read(next.clone(), identity_3)), Box::new(next.clone()))
                        }
                        self::JavaScriptErrorExpression::ErrorModuleCellSet(identity_4, expression_value_2) => {
                            let expression_value_2 = *expression_value_2;
                            {
                                let value_3 = self::lower_java_script_error_expression(expression_value_2.clone(), next.clone(), (depth + 1f64));
                                if { let ml_values223 = { let ml_reference_value224 = ((match next.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15377) => {
                                        ((source_ml_source2_data_property_value15377.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value224 }; let ml_callback223 = { let identity_4 = identity_4; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value15379, _, _, _, _) => {
                                        source_ml_source2_data_property_value15379
                                    }
                                }) == identity_4) }) }; ml_values223.iter().cloned().enumerate().any(|(ml_index223, ml_value223)| ml_callback223(ml_value223)) } {
                                    self::lower_java_script_lexical_cell_write(match value_3.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15380) => {
                                            let source_ml_source2_data_property_value15380 = *source_ml_source2_data_property_value15380;
                                            source_ml_source2_data_property_value15380.clone()
                                        }
                                    }, identity_4, match value_3.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15381, _) => {
                                            let source_ml_source2_data_property_value15381 = *source_ml_source2_data_property_value15381;
                                            source_ml_source2_data_property_value15381.clone()
                                        }
                                    }, false)
                                } else {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceSet(Box::new(self::JavaScriptCallSource::SourceBinding(identity_4)), Box::new(self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))), Box::new(match value_3.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15382, _) => {
                                            let source_ml_source2_data_property_value15382 = *source_ml_source2_data_property_value15382;
                                            source_ml_source2_data_property_value15382.clone()
                                        }
                                    }))), Box::new(match value_3.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15383) => {
                                            let source_ml_source2_data_property_value15383 = *source_ml_source2_data_property_value15383;
                                            source_ml_source2_data_property_value15383.clone()
                                        }
                                    }))
                                }
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorUnary(operator, expression_value_3) => {
                            let expression_value_3 = *expression_value_3;
                            {
                                let value_4 = self::lower_java_script_error_expression(expression_value_3.clone(), next.clone(), (depth + 1f64));
                                if (operator == (self::ml_string::Text::from_units(vec![118, 111, 105, 100]))) {
                                    {
                                        let identity_5 = match match value_4.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15384) => {
                                                let source_ml_source2_data_property_value15384 = *source_ml_source2_data_property_value15384;
                                                source_ml_source2_data_property_value15384.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15385, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value15385
                                            }
                                        };
                                        {
                                            let current = {
                                                let source_ml_source2_nominal_spread15387 = match value_4.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15386) => {
                                                        let source_ml_source2_data_property_value15386 = *source_ml_source2_data_property_value15386;
                                                        source_ml_source2_data_property_value15386.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field15388 = match source_ml_source2_nominal_spread15387.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15389, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value15389 = *source_ml_source2_data_property_value15389;
                                                            source_ml_source2_data_property_value15389.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field15390 = match source_ml_source2_nominal_spread15387.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15391, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value15391 = *source_ml_source2_data_property_value15391;
                                                                source_ml_source2_data_property_value15391.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field15392 = match source_ml_source2_nominal_spread15387.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15393, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value15393.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field15394 = match source_ml_source2_nominal_spread15387.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15395, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value15395
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field15396 = match source_ml_source2_nominal_spread15387.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15397, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value15397
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field15398 = match source_ml_source2_nominal_spread15387.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15399, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value15399.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field15400 = match source_ml_source2_nominal_spread15387.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15401, _, _, _) => {
                                                                                    source_ml_source2_data_property_value15401
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field15402 = match source_ml_source2_nominal_spread15387.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15403, _, _) => {
                                                                                        source_ml_source2_data_property_value15403.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field15404 = match source_ml_source2_nominal_spread15387.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15405, _) => {
                                                                                            source_ml_source2_data_property_value15405.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15406 = match source_ml_source2_nominal_spread15387.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15407) => {
                                                                                                source_ml_source2_data_property_value15407.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override15408 = (identity_5 + 1f64);
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15388.clone()), Box::new(source_ml_source2_nominal_field15390.clone()), source_ml_source2_nominal_field15392.clone(), source_ml_source2_nominal_override15408, source_ml_source2_nominal_field15396, source_ml_source2_nominal_field15398.clone(), source_ml_source2_nominal_field15400, source_ml_source2_nominal_field15402.clone(), source_ml_source2_nominal_field15404.clone(), source_ml_source2_nominal_field15406.clone())
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(current.clone(), identity_5, match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15409, _) => {
                                                    let source_ml_source2_data_property_value15409 = *source_ml_source2_data_property_value15409;
                                                    source_ml_source2_data_property_value15409.clone()
                                                }
                                            }, (constant.clone())(self::JavaScriptSemanticValue::Undefined), false)), Box::new(current.clone()))
                                        }
                                    }
                                } else {
                                    if (operator == (self::ml_string::Text::from_units(vec![33]))) {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceIf(Box::new(match value_4.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15410, _) => {
                                                let source_ml_source2_data_property_value15410 = *source_ml_source2_data_property_value15410;
                                                source_ml_source2_data_property_value15410.clone()
                                            }
                                        }), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(false))), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(true))))), Box::new(match value_4.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15411) => {
                                                let source_ml_source2_data_property_value15411 = *source_ml_source2_data_property_value15411;
                                                source_ml_source2_data_property_value15411.clone()
                                            }
                                        }))
                                    } else {
                                        if (operator == (self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102]))) {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15412, _) => {
                                                    let source_ml_source2_data_property_value15412 = *source_ml_source2_data_property_value15412;
                                                    source_ml_source2_data_property_value15412.clone()
                                                }
                                            }])))), Box::new(match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15413) => {
                                                    let source_ml_source2_data_property_value15413 = *source_ml_source2_data_property_value15413;
                                                    source_ml_source2_data_property_value15413.clone()
                                                }
                                            }))
                                        } else {
                                            if (operator == (self::ml_string::Text::from_units(vec![43]))) {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![match value_4.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15414, _) => {
                                                        let source_ml_source2_data_property_value15414 = *source_ml_source2_data_property_value15414;
                                                        source_ml_source2_data_property_value15414.clone()
                                                    }
                                                }])))), Box::new(match value_4.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15415) => {
                                                        let source_ml_source2_data_property_value15415 = *source_ml_source2_data_property_value15415;
                                                        source_ml_source2_data_property_value15415.clone()
                                                    }
                                                }))
                                            } else {
                                                if (operator == (self::ml_string::Text::from_units(vec![45]))) {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99]), std::rc::Rc::new(std::cell::RefCell::new(vec![match value_4.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15416, _) => {
                                                            let source_ml_source2_data_property_value15416 = *source_ml_source2_data_property_value15416;
                                                            source_ml_source2_data_property_value15416.clone()
                                                        }
                                                    }])))])))), Box::new(match value_4.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15417) => {
                                                            let source_ml_source2_data_property_value15417 = *source_ml_source2_data_property_value15417;
                                                            source_ml_source2_data_property_value15417.clone()
                                                        }
                                                    }))
                                                } else {
                                                    self::refuse_java_script_error_source(match value_4.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15418) => {
                                                            let source_ml_source2_data_property_value15418 = *source_ml_source2_data_property_value15418;
                                                            source_ml_source2_data_property_value15418.clone()
                                                        }
                                                    }, &(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 117, 110, 97, 114, 121, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 111, 119, 110, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 112, 114, 111, 102, 105, 108, 101])))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorUpdate(_, _, _) => {
                            self::lower_java_script_source_update(expression.clone(), next.clone(), (depth + 1f64))
                        }
                        self::JavaScriptErrorExpression::ErrorCompoundAssignment(_, _, _) => {
                            self::lower_java_script_source_compound_assignment(expression.clone(), next.clone(), (depth + 1f64))
                        }
                        self::JavaScriptErrorExpression::ErrorBinary(_, _, _, _) => {
                            self::lower_java_script_source_binary(expression.clone(), next.clone(), (depth + 1f64))
                        }
                        self::JavaScriptErrorExpression::ErrorArray(_, _) => {
                            self::lower_java_script_error_literal_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorArrayAccumulation(_, _) => {
                            self::lower_java_script_error_literal_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorArrayLiteral(_, _) => {
                            self::lower_java_script_error_literal_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorObject(_) => {
                            self::lower_java_script_error_literal_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorTemplate(_) => {
                            self::lower_java_script_error_literal_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorCall(_, _, _, _) => {
                            self::lower_java_script_error_construction_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorFunctionReference(identity_6) => {
                            {
                                let owner = { let ml_values225 = { let ml_reference_value226 = (match match next.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15419, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value15419 = *source_ml_source2_data_property_value15419;
                                        source_ml_source2_data_property_value15419.clone()
                                    }
                                } {
                                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value15420, _, _, _, _) => {
                                        source_ml_source2_data_property_value15420.clone()
                                    }
                                }).borrow().clone(); ml_reference_value226 }; let ml_callback225 = { let identity_6 = identity_6; std::rc::Rc::new(move |owner: self::JavaScriptErrorFunctionOwner| -> bool { ((match owner.clone() {
                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value15424, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value15424
                                    }
                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value15425, _, _, _) => {
                                        source_ml_source2_data_property_value15425
                                    }
                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value15426, _, _) => {
                                        source_ml_source2_data_property_value15426
                                    }
                                }) == identity_6) }) }; ml_values225.iter().cloned().enumerate().find_map(|(ml_index225, ml_value225)| if ml_callback225(ml_value225.clone()) { Some(ml_value225) } else { None }) };
                                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121])))
                                } else {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object(match owner.clone().expect("a proven present value") {
                                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value15427, _, _, _, _) => {
                                            source_ml_source2_data_property_value15427.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value15428, _, _) => {
                                            source_ml_source2_data_property_value15428.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value15429, _) => {
                                            source_ml_source2_data_property_value15429.clone()
                                        }
                                    }, true))), Box::new(next.clone()))
                                }
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorNativeReference(source) => {
                            {
                                let owners = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values227 = { let ml_reference_value228 = (match next.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15430, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value15430.clone()
                                    }
                                }).borrow().clone(); ml_reference_value228 }; let ml_callback227 = { let source = source.clone(); std::rc::Rc::new(move |owner_2: self::JavaScriptSourceCapabilityOwner| -> bool { ((match owner_2.clone() {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value15432, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value15432.clone()
                                    }
                                }) == source) }) }; ml_values227.iter().cloned().enumerate().filter_map(|(ml_index227, ml_value227)| if ml_callback227(ml_value227.clone()) { Some(ml_value227) } else { None }).collect::<Vec<_>>() }));
                                {
                                    let owner_2 = {
                                        let source_ml_source2_array_read_index15434 = 0f64;
                                        { let ml_reference_array229 = owners.clone(); let ml_reference_position229 = source_ml_source2_array_read_index15434; let ml_reference_projection229 = self::ml_array::optional_at(&*ml_reference_array229.borrow(), self::ml_array::number_index(ml_reference_position229)); ml_reference_projection229 }
                                    };
                                    if ((({ let ml_reference_array230 = owners.clone(); let ml_reference_projection230 = ml_reference_array230.borrow().len() as f64; ml_reference_projection230 }) != (1f64)) || match (owner_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 110, 97, 116, 105, 118, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 45, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 111, 119, 110, 101, 114])))
                                    } else {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object({
                                            let source_ml_source2_nullish_field_receiver15435 = (owner_2.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver15435.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present15436 = ((source_ml_source2_nullish_field_receiver15435.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present15436.clone() {
                                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value15437, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value15437.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }, {
                                            let source_ml_source2_nullish_field_receiver15438 = (owner_2.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver15438.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present15439 = ((source_ml_source2_nullish_field_receiver15438.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present15439.clone() {
                                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value15440, _) => {
                                                            source_ml_source2_data_property_value15440
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }))), Box::new(next.clone()))
                                    }
                                }
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorNewTarget => {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceNewTarget), Box::new(next.clone()))
                        }
                        self::JavaScriptErrorExpression::ErrorGenericConstruction(_, _) => {
                            self::lower_java_script_error_construction_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorReceiver => {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceReceiver), Box::new(next.clone()))
                        }
                        self::JavaScriptErrorExpression::ErrorOrdinaryConstruction(_, _) => {
                            self::lower_java_script_error_construction_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorCollectionConstruction(_, _) => {
                            self::lower_java_script_error_construction_expression(expression.clone(), next.clone(), depth)
                        }
                        self::JavaScriptErrorExpression::ErrorBindingSet(identity_7, expression_value_4, immutable) => {
                            let expression_value_4 = *expression_value_4;
                            if !(match next.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15441, _, _, _) => {
                                    source_ml_source2_data_property_value15441
                                }
                            }) {
                                self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101, 32, 108, 101, 120, 105, 99, 97, 108, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101])))
                            } else {
                                {
                                    let value_5 = self::lower_java_script_error_expression(expression_value_4.clone(), next.clone(), (depth + 1f64));
                                    {
                                        let source_ml_source2_nominal_field15445 = match value_5.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15446, _) => {
                                                let source_ml_source2_data_property_value15446 = *source_ml_source2_data_property_value15446;
                                                source_ml_source2_data_property_value15446.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15447 = match value_5.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15448) => {
                                                    let source_ml_source2_data_property_value15448 = *source_ml_source2_data_property_value15448;
                                                    source_ml_source2_data_property_value15448.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override15449 = if match ((immutable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                                    self::JavaScriptCallSource::SourceImmutableBindingWrite(Box::new(match value_5.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15442, _) => {
                                                            let source_ml_source2_data_property_value15442 = *source_ml_source2_data_property_value15442;
                                                            source_ml_source2_data_property_value15442.clone()
                                                        }
                                                    }))
                                                } else {
                                                    self::JavaScriptCallSource::SourceAssignBinding(identity_7, Box::new(match value_5.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15443, _) => {
                                                            let source_ml_source2_data_property_value15443 = *source_ml_source2_data_property_value15443;
                                                            source_ml_source2_data_property_value15443.clone()
                                                        }
                                                    }))
                                                };
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override15449.clone()), Box::new(source_ml_source2_nominal_field15447.clone()))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorSpecialNumber(form) => {
                            if ((form == (self::ml_string::Text::from_units(vec![78, 97, 78]))) || (form == (self::ml_string::Text::from_units(vec![73, 110, 102, 105, 110, 105, 116, 121])))) {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new((constant.clone())(self::JavaScriptSemanticValue::Number(if (form == (self::ml_string::Text::from_units(vec![78, 97, 78]))) {
                                    f64::NAN
                                } else {
                                    f64::INFINITY
                                }))), Box::new(next.clone()))
                            } else {
                                self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 112, 101, 99, 105, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 104, 97, 100, 111, 119, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 103, 108, 111, 98, 97, 108])))
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorStrictEqual(expression_left_2, expression_right_2, negate) => {
                            let expression_left_2 = *expression_left_2;
                            let expression_right_2 = *expression_right_2;
                            {
                                let left_2 = self::lower_java_script_error_expression(expression_left_2.clone(), next.clone(), (depth + 1f64));
                                {
                                    let right_2 = self::lower_java_script_error_expression(expression_right_2.clone(), match left_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15450) => {
                                            let source_ml_source2_data_property_value15450 = *source_ml_source2_data_property_value15450;
                                            source_ml_source2_data_property_value15450.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let source_2 = if match next.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15451, _, _, _) => {
                                                source_ml_source2_data_property_value15451
                                            }
                                        } {
                                            self::JavaScriptCallSource::SourceSnapshotStrictEqual(Box::new(match left_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15452, _) => {
                                                    let source_ml_source2_data_property_value15452 = *source_ml_source2_data_property_value15452;
                                                    source_ml_source2_data_property_value15452.clone()
                                                }
                                            }), Box::new(match right_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15453, _) => {
                                                    let source_ml_source2_data_property_value15453 = *source_ml_source2_data_property_value15453;
                                                    source_ml_source2_data_property_value15453.clone()
                                                }
                                            }))
                                        } else {
                                            self::JavaScriptCallSource::SourceStrictEqual(Box::new(match left_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15454, _) => {
                                                    let source_ml_source2_data_property_value15454 = *source_ml_source2_data_property_value15454;
                                                    source_ml_source2_data_property_value15454.clone()
                                                }
                                            }), Box::new(match right_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15455, _) => {
                                                    let source_ml_source2_data_property_value15455 = *source_ml_source2_data_property_value15455;
                                                    source_ml_source2_data_property_value15455.clone()
                                                }
                                            }))
                                        };
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(if negate {
                                            self::JavaScriptCallSource::SourceIf(Box::new(source_2.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(false))), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(true))))
                                        } else {
                                            source_2.clone()
                                        }), Box::new(match right_2.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15456) => {
                                                let source_ml_source2_data_property_value15456 = *source_ml_source2_data_property_value15456;
                                                source_ml_source2_data_property_value15456.clone()
                                            }
                                        }))
                                    }
                                }
                            }
                        }
                        _ => {
                            self::lower_java_script_error_property_expression(expression.clone(), next.clone(), depth)
                        }
                    }
                }
            }
        }
    }
}
