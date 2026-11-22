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
use super::ml_string;
use super::refuse_java_script_error_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_error_expression(expression: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    {
        let next = {
            let source_ml_source2_nominal_field15211 = match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15212, _, _, _, _, _, _, _, _, _) => {
                    let source_ml_source2_data_property_value15212 = *source_ml_source2_data_property_value15212;
                    source_ml_source2_data_property_value15212.clone()
                }
            };
            {
                let source_ml_source2_nominal_field15213 = match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15214, _, _, _, _, _, _, _, _) => {
                        let source_ml_source2_data_property_value15214 = *source_ml_source2_data_property_value15214;
                        source_ml_source2_data_property_value15214.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field15215 = match context.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15216, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15216.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field15217 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15218, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15218
                            }
                        };
                        {
                            let source_ml_source2_nominal_field15219 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15220, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value15220
                                }
                            };
                            {
                                let source_ml_source2_nominal_field15221 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15222, _, _, _, _) => {
                                        source_ml_source2_data_property_value15222.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field15223 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15224, _, _, _) => {
                                            source_ml_source2_data_property_value15224
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field15225 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15226, _, _) => {
                                                source_ml_source2_data_property_value15226.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15227 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15228, _) => {
                                                    source_ml_source2_data_property_value15228.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field15229 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15230) => {
                                                        source_ml_source2_data_property_value15230.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override15231 = (match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15209, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value15209
                                                        }
                                                    } + 1f64);
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15211.clone()), Box::new(source_ml_source2_nominal_field15213.clone()), source_ml_source2_nominal_field15215.clone(), source_ml_source2_nominal_field15217, source_ml_source2_nominal_override15231, source_ml_source2_nominal_field15221.clone(), source_ml_source2_nominal_field15223, source_ml_source2_nominal_field15225.clone(), source_ml_source2_nominal_field15227.clone(), source_ml_source2_nominal_field15229.clone())
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
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15233, _, _, _, _) => {
                source_ml_source2_data_property_value15233.clone()
            }
        }) != (self::ml_string::Text::from_units(vec![]))) {
            self::refuse_java_script_error_source(next.clone(), &(match next.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15234, _, _, _, _) => {
                    source_ml_source2_data_property_value15234.clone()
                }
            }))
        } else {
            if ((depth > (128f64)) || ((match next.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15236, _, _, _, _, _) => {
                    source_ml_source2_data_property_value15236
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
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15291) => {
                                            let source_ml_source2_data_property_value15291 = *source_ml_source2_data_property_value15291;
                                            source_ml_source2_data_property_value15291.clone()
                                        }
                                    }, (depth + 1f64));
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceArgumentSpread(Box::new(match value_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15292, _) => {
                                            let source_ml_source2_data_property_value15292 = *source_ml_source2_data_property_value15292;
                                            source_ml_source2_data_property_value15292.clone()
                                        }
                                    }), Box::new(match key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15293, _) => {
                                            let source_ml_source2_data_property_value15293 = *source_ml_source2_data_property_value15293;
                                            source_ml_source2_data_property_value15293.clone()
                                        }
                                    }), match match next.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15294, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value15294 = *source_ml_source2_data_property_value15294;
                                            source_ml_source2_data_property_value15294.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value15295, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value15295.clone()
                                        }
                                    })), Box::new(match key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15296) => {
                                            let source_ml_source2_data_property_value15296 = *source_ml_source2_data_property_value15296;
                                            source_ml_source2_data_property_value15296.clone()
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
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15297) => {
                                            let source_ml_source2_data_property_value15297 = *source_ml_source2_data_property_value15297;
                                            source_ml_source2_data_property_value15297.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let key_2 = self::lower_java_script_error_expression(expression_key.clone(), match right.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15298) => {
                                                let source_ml_source2_data_property_value15298 = *source_ml_source2_data_property_value15298;
                                                source_ml_source2_data_property_value15298.clone()
                                            }
                                        }, (depth + 1f64));
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceInstanceOf(Box::new(match left.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15299, _) => {
                                                let source_ml_source2_data_property_value15299 = *source_ml_source2_data_property_value15299;
                                                source_ml_source2_data_property_value15299.clone()
                                            }
                                        }), Box::new(match right.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15300, _) => {
                                                let source_ml_source2_data_property_value15300 = *source_ml_source2_data_property_value15300;
                                                source_ml_source2_data_property_value15300.clone()
                                            }
                                        }), Box::new(match key_2.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15301, _) => {
                                                let source_ml_source2_data_property_value15301 = *source_ml_source2_data_property_value15301;
                                                source_ml_source2_data_property_value15301.clone()
                                            }
                                        }))), Box::new(match key_2.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15302) => {
                                                let source_ml_source2_data_property_value15302 = *source_ml_source2_data_property_value15302;
                                                source_ml_source2_data_property_value15302.clone()
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
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15303) => {
                                            let source_ml_source2_data_property_value15303 = *source_ml_source2_data_property_value15303;
                                            source_ml_source2_data_property_value15303.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let no = self::lower_java_script_error_expression(expression_no.clone(), match yes.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15304) => {
                                                let source_ml_source2_data_property_value15304 = *source_ml_source2_data_property_value15304;
                                                source_ml_source2_data_property_value15304.clone()
                                            }
                                        }, (depth + 1f64));
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceIf(Box::new(match condition.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15305, _) => {
                                                let source_ml_source2_data_property_value15305 = *source_ml_source2_data_property_value15305;
                                                source_ml_source2_data_property_value15305.clone()
                                            }
                                        }), Box::new(match yes.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15306, _) => {
                                                let source_ml_source2_data_property_value15306 = *source_ml_source2_data_property_value15306;
                                                source_ml_source2_data_property_value15306.clone()
                                            }
                                        }), Box::new(match no.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15307, _) => {
                                                let source_ml_source2_data_property_value15307 = *source_ml_source2_data_property_value15307;
                                                source_ml_source2_data_property_value15307.clone()
                                            }
                                        }))), Box::new(match no.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15308) => {
                                                let source_ml_source2_data_property_value15308 = *source_ml_source2_data_property_value15308;
                                                source_ml_source2_data_property_value15308.clone()
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
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15309) => {
                                        ((source_ml_source2_data_property_value15309.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value224 }; let ml_callback223 = { let identity_4 = identity_4; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value15311, _, _, _, _) => {
                                        source_ml_source2_data_property_value15311
                                    }
                                }) == identity_4) }) }; ml_values223.iter().cloned().enumerate().any(|(ml_index223, ml_value223)| ml_callback223(ml_value223)) } {
                                    self::lower_java_script_lexical_cell_write(match value_3.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15312) => {
                                            let source_ml_source2_data_property_value15312 = *source_ml_source2_data_property_value15312;
                                            source_ml_source2_data_property_value15312.clone()
                                        }
                                    }, identity_4, match value_3.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15313, _) => {
                                            let source_ml_source2_data_property_value15313 = *source_ml_source2_data_property_value15313;
                                            source_ml_source2_data_property_value15313.clone()
                                        }
                                    }, false)
                                } else {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceSet(Box::new(self::JavaScriptCallSource::SourceBinding(identity_4)), Box::new(self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))), Box::new(match value_3.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15314, _) => {
                                            let source_ml_source2_data_property_value15314 = *source_ml_source2_data_property_value15314;
                                            source_ml_source2_data_property_value15314.clone()
                                        }
                                    }))), Box::new(match value_3.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15315) => {
                                            let source_ml_source2_data_property_value15315 = *source_ml_source2_data_property_value15315;
                                            source_ml_source2_data_property_value15315.clone()
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15316) => {
                                                let source_ml_source2_data_property_value15316 = *source_ml_source2_data_property_value15316;
                                                source_ml_source2_data_property_value15316.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15317, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value15317
                                            }
                                        };
                                        {
                                            let current = {
                                                let source_ml_source2_nominal_spread15319 = match value_4.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15318) => {
                                                        let source_ml_source2_data_property_value15318 = *source_ml_source2_data_property_value15318;
                                                        source_ml_source2_data_property_value15318.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field15320 = match source_ml_source2_nominal_spread15319.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15321, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value15321 = *source_ml_source2_data_property_value15321;
                                                            source_ml_source2_data_property_value15321.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field15322 = match source_ml_source2_nominal_spread15319.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15323, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value15323 = *source_ml_source2_data_property_value15323;
                                                                source_ml_source2_data_property_value15323.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field15324 = match source_ml_source2_nominal_spread15319.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15325, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value15325.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field15326 = match source_ml_source2_nominal_spread15319.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15327, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value15327
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field15328 = match source_ml_source2_nominal_spread15319.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15329, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value15329
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field15330 = match source_ml_source2_nominal_spread15319.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15331, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value15331.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field15332 = match source_ml_source2_nominal_spread15319.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15333, _, _, _) => {
                                                                                    source_ml_source2_data_property_value15333
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field15334 = match source_ml_source2_nominal_spread15319.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15335, _, _) => {
                                                                                        source_ml_source2_data_property_value15335.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field15336 = match source_ml_source2_nominal_spread15319.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15337, _) => {
                                                                                            source_ml_source2_data_property_value15337.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15338 = match source_ml_source2_nominal_spread15319.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15339) => {
                                                                                                source_ml_source2_data_property_value15339.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override15340 = (identity_5 + 1f64);
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15320.clone()), Box::new(source_ml_source2_nominal_field15322.clone()), source_ml_source2_nominal_field15324.clone(), source_ml_source2_nominal_override15340, source_ml_source2_nominal_field15328, source_ml_source2_nominal_field15330.clone(), source_ml_source2_nominal_field15332, source_ml_source2_nominal_field15334.clone(), source_ml_source2_nominal_field15336.clone(), source_ml_source2_nominal_field15338.clone())
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
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15341, _) => {
                                                    let source_ml_source2_data_property_value15341 = *source_ml_source2_data_property_value15341;
                                                    source_ml_source2_data_property_value15341.clone()
                                                }
                                            }, (constant.clone())(self::JavaScriptSemanticValue::Undefined), false)), Box::new(current.clone()))
                                        }
                                    }
                                } else {
                                    if (operator == (self::ml_string::Text::from_units(vec![33]))) {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceIf(Box::new(match value_4.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15342, _) => {
                                                let source_ml_source2_data_property_value15342 = *source_ml_source2_data_property_value15342;
                                                source_ml_source2_data_property_value15342.clone()
                                            }
                                        }), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(false))), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(true))))), Box::new(match value_4.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15343) => {
                                                let source_ml_source2_data_property_value15343 = *source_ml_source2_data_property_value15343;
                                                source_ml_source2_data_property_value15343.clone()
                                            }
                                        }))
                                    } else {
                                        if (operator == (self::ml_string::Text::from_units(vec![116, 121, 112, 101, 111, 102]))) {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15344, _) => {
                                                    let source_ml_source2_data_property_value15344 = *source_ml_source2_data_property_value15344;
                                                    source_ml_source2_data_property_value15344.clone()
                                                }
                                            }])))), Box::new(match value_4.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15345) => {
                                                    let source_ml_source2_data_property_value15345 = *source_ml_source2_data_property_value15345;
                                                    source_ml_source2_data_property_value15345.clone()
                                                }
                                            }))
                                        } else {
                                            if (operator == (self::ml_string::Text::from_units(vec![43]))) {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![match value_4.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15346, _) => {
                                                        let source_ml_source2_data_property_value15346 = *source_ml_source2_data_property_value15346;
                                                        source_ml_source2_data_property_value15346.clone()
                                                    }
                                                }])))), Box::new(match value_4.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15347) => {
                                                        let source_ml_source2_data_property_value15347 = *source_ml_source2_data_property_value15347;
                                                        source_ml_source2_data_property_value15347.clone()
                                                    }
                                                }))
                                            } else {
                                                if (operator == (self::ml_string::Text::from_units(vec![45]))) {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99]), std::rc::Rc::new(std::cell::RefCell::new(vec![match value_4.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15348, _) => {
                                                            let source_ml_source2_data_property_value15348 = *source_ml_source2_data_property_value15348;
                                                            source_ml_source2_data_property_value15348.clone()
                                                        }
                                                    }])))])))), Box::new(match value_4.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15349) => {
                                                            let source_ml_source2_data_property_value15349 = *source_ml_source2_data_property_value15349;
                                                            source_ml_source2_data_property_value15349.clone()
                                                        }
                                                    }))
                                                } else {
                                                    self::refuse_java_script_error_source(match value_4.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15350) => {
                                                            let source_ml_source2_data_property_value15350 = *source_ml_source2_data_property_value15350;
                                                            source_ml_source2_data_property_value15350.clone()
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
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15351, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value15351 = *source_ml_source2_data_property_value15351;
                                        source_ml_source2_data_property_value15351.clone()
                                    }
                                } {
                                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value15352, _, _, _, _) => {
                                        source_ml_source2_data_property_value15352.clone()
                                    }
                                }).borrow().clone(); ml_reference_value226 }; let ml_callback225 = { let identity_6 = identity_6; std::rc::Rc::new(move |owner: self::JavaScriptErrorFunctionOwner| -> bool { ((match owner.clone() {
                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value15356, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value15356
                                    }
                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value15357, _, _, _) => {
                                        source_ml_source2_data_property_value15357
                                    }
                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value15358, _, _) => {
                                        source_ml_source2_data_property_value15358
                                    }
                                }) == identity_6) }) }; ml_values225.iter().cloned().enumerate().find_map(|(ml_index225, ml_value225)| if ml_callback225(ml_value225.clone()) { Some(ml_value225) } else { None }) };
                                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121])))
                                } else {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object(match owner.clone().expect("a proven present value") {
                                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value15359, _, _, _, _) => {
                                            source_ml_source2_data_property_value15359.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value15360, _, _) => {
                                            source_ml_source2_data_property_value15360.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value15361, _) => {
                                            source_ml_source2_data_property_value15361.clone()
                                        }
                                    }, true))), Box::new(next.clone()))
                                }
                            }
                        }
                        self::JavaScriptErrorExpression::ErrorNativeReference(source) => {
                            {
                                let owners = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values227 = { let ml_reference_value228 = (match next.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15362, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value15362.clone()
                                    }
                                }).borrow().clone(); ml_reference_value228 }; let ml_callback227 = { let source = source.clone(); std::rc::Rc::new(move |owner_2: self::JavaScriptSourceCapabilityOwner| -> bool { ((match owner_2.clone() {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value15364, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value15364.clone()
                                    }
                                }) == source) }) }; ml_values227.iter().cloned().enumerate().filter_map(|(ml_index227, ml_value227)| if ml_callback227(ml_value227.clone()) { Some(ml_value227) } else { None }).collect::<Vec<_>>() }));
                                {
                                    let owner_2 = {
                                        let source_ml_source2_array_read_index15366 = 0f64;
                                        { let ml_reference_array229 = owners.clone(); let ml_reference_position229 = source_ml_source2_array_read_index15366; let ml_reference_projection229 = self::ml_array::optional_at(&*ml_reference_array229.borrow(), self::ml_array::number_index(ml_reference_position229)); ml_reference_projection229 }
                                    };
                                    if ((({ let ml_reference_array230 = owners.clone(); let ml_reference_projection230 = ml_reference_array230.borrow().len() as f64; ml_reference_projection230 }) != (1f64)) || match (owner_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 110, 97, 116, 105, 118, 101, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 45, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 111, 119, 110, 101, 114])))
                                    } else {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object(match owner_2.clone().expect("a proven present value") {
                                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value15367, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value15367.clone()
                                            }
                                        }, match owner_2.clone().expect("a proven present value") {
                                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value15368, _) => {
                                                source_ml_source2_data_property_value15368
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
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15369, _, _, _) => {
                                    source_ml_source2_data_property_value15369
                                }
                            }) {
                                self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101, 32, 108, 101, 120, 105, 99, 97, 108, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101])))
                            } else {
                                {
                                    let value_5 = self::lower_java_script_error_expression(expression_value_4.clone(), next.clone(), (depth + 1f64));
                                    {
                                        let source_ml_source2_nominal_field15373 = match value_5.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15374, _) => {
                                                let source_ml_source2_data_property_value15374 = *source_ml_source2_data_property_value15374;
                                                source_ml_source2_data_property_value15374.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field15375 = match value_5.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15376) => {
                                                    let source_ml_source2_data_property_value15376 = *source_ml_source2_data_property_value15376;
                                                    source_ml_source2_data_property_value15376.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_override15377 = if match ((immutable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                                    self::JavaScriptCallSource::SourceImmutableBindingWrite(Box::new(match value_5.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15370, _) => {
                                                            let source_ml_source2_data_property_value15370 = *source_ml_source2_data_property_value15370;
                                                            source_ml_source2_data_property_value15370.clone()
                                                        }
                                                    }))
                                                } else {
                                                    self::JavaScriptCallSource::SourceAssignBinding(identity_7, Box::new(match value_5.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15371, _) => {
                                                            let source_ml_source2_data_property_value15371 = *source_ml_source2_data_property_value15371;
                                                            source_ml_source2_data_property_value15371.clone()
                                                        }
                                                    }))
                                                };
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override15377.clone()), Box::new(source_ml_source2_nominal_field15375.clone()))
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
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15378) => {
                                            let source_ml_source2_data_property_value15378 = *source_ml_source2_data_property_value15378;
                                            source_ml_source2_data_property_value15378.clone()
                                        }
                                    }, (depth + 1f64));
                                    {
                                        let source_2 = if match next.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15379, _, _, _) => {
                                                source_ml_source2_data_property_value15379
                                            }
                                        } {
                                            self::JavaScriptCallSource::SourceSnapshotStrictEqual(Box::new(match left_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15380, _) => {
                                                    let source_ml_source2_data_property_value15380 = *source_ml_source2_data_property_value15380;
                                                    source_ml_source2_data_property_value15380.clone()
                                                }
                                            }), Box::new(match right_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15381, _) => {
                                                    let source_ml_source2_data_property_value15381 = *source_ml_source2_data_property_value15381;
                                                    source_ml_source2_data_property_value15381.clone()
                                                }
                                            }))
                                        } else {
                                            self::JavaScriptCallSource::SourceStrictEqual(Box::new(match left_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15382, _) => {
                                                    let source_ml_source2_data_property_value15382 = *source_ml_source2_data_property_value15382;
                                                    source_ml_source2_data_property_value15382.clone()
                                                }
                                            }), Box::new(match right_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15383, _) => {
                                                    let source_ml_source2_data_property_value15383 = *source_ml_source2_data_property_value15383;
                                                    source_ml_source2_data_property_value15383.clone()
                                                }
                                            }))
                                        };
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(if negate {
                                            self::JavaScriptCallSource::SourceIf(Box::new(source_2.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(false))), Box::new((constant.clone())(self::JavaScriptSemanticValue::Boolean(true))))
                                        } else {
                                            source_2.clone()
                                        }), Box::new(match right_2.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15384) => {
                                                let source_ml_source2_data_property_value15384 = *source_ml_source2_data_property_value15384;
                                                source_ml_source2_data_property_value15384.clone()
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
