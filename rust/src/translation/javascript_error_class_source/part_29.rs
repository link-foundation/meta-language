// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallCellInitializer;
use super::JavaScriptCallFunction;
use super::JavaScriptCallSource;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceBindingTarget;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceStatement;
use super::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop241Result;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_lexical_cell_read;
use super::lower_java_script_error_expression;
use super::lower_java_script_source_binding_pattern;
use super::lower_java_script_source_statements;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop241;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_lexical_for_of(statement: self::JavaScriptSourceStatement, context: self::JavaScriptErrorSourceContext, continuation: self::JavaScriptCallSource, owned: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match statement.clone() {
        self::JavaScriptSourceStatement::SourceForOfStatement(target, binding, mutable, _, _, statement_input, iterator_key, statement_body, statement_pattern) => {
            let statement_input = *statement_input;
            let iterator_key = *iterator_key;
            if (((({ let ml_reference_array360 = owned.clone(); let ml_reference_projection360 = ml_reference_array360.borrow().len() as f64; ml_reference_projection360 }) == (0f64)) || (({ let ml_reference_array361 = owned.clone(); let ml_reference_projection361 = ml_reference_array361.borrow().len() as f64; ml_reference_projection361 }) > (4096f64))) || { let ml_values362 = { let ml_reference_value363 = owned.borrow().clone(); ml_reference_value363 }; let ml_callback362 = { let owned = owned.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { ((self::ml_array::index_of(&{ let ml_reference_value364 = owned.borrow().clone(); ml_reference_value364 }, &identity, 0f64, false, |a, b| a == b)) != index) }) }; ml_values362.iter().cloned().enumerate().any(|(ml_index362, ml_value362)| ml_callback362(ml_value362, (ml_index362 as f64))) }) {
                self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 102, 111, 114, 45, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 111, 119, 110, 101, 100, 32, 104, 101, 97, 100, 32, 98, 105, 110, 100, 105, 110, 103, 115])))
            } else {
                {
                    let pattern = ((statement_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                    {
                        let input = self::lower_java_script_error_expression(statement_input.clone(), context.clone(), (depth + 1f64));
                        {
                            let key = self::lower_java_script_error_expression(iterator_key.clone(), match input.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18329) => {
                                    let source_ml_source2_data_property_value18329 = *source_ml_source2_data_property_value18329;
                                    source_ml_source2_data_property_value18329.clone()
                                }
                            }, (depth + 1f64));
                            {
                                let value_id = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    match match key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18330) => {
                                            let source_ml_source2_data_property_value18330 = *source_ml_source2_data_property_value18330;
                                            source_ml_source2_data_property_value18330.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18331, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18331
                                        }
                                    }
                                } else {
                                    match pattern.clone().expect("a proven present value") {
                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value18332, _, _) => {
                                            source_ml_source2_data_property_value18332
                                        }
                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value18333, _) => {
                                            source_ml_source2_data_property_value18333
                                        }
                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value18334, _, _) => {
                                            source_ml_source2_data_property_value18334
                                        }
                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value18335, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18335
                                        }
                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value18336, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18336
                                        }
                                    }
                                };
                                {
                                    let prepared = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        {
                                            let source_ml_source2_nominal_spread18340 = match key.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18337) => {
                                                    let source_ml_source2_data_property_value18337 = *source_ml_source2_data_property_value18337;
                                                    source_ml_source2_data_property_value18337.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field18341 = match source_ml_source2_nominal_spread18340.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18342, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value18342 = *source_ml_source2_data_property_value18342;
                                                        source_ml_source2_data_property_value18342.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18343 = match source_ml_source2_nominal_spread18340.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18344, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value18344 = *source_ml_source2_data_property_value18344;
                                                            source_ml_source2_data_property_value18344.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18345 = match source_ml_source2_nominal_spread18340.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18346, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18346.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18347 = match source_ml_source2_nominal_spread18340.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18348, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18348
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18349 = match source_ml_source2_nominal_spread18340.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18350, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18350
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18351 = match source_ml_source2_nominal_spread18340.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18352, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18352.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18353 = match source_ml_source2_nominal_spread18340.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18354, _, _, _) => {
                                                                                source_ml_source2_data_property_value18354
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18355 = match source_ml_source2_nominal_spread18340.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18356, _, _) => {
                                                                                    source_ml_source2_data_property_value18356.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18357 = match source_ml_source2_nominal_spread18340.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18358, _) => {
                                                                                        source_ml_source2_data_property_value18358.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18359 = match source_ml_source2_nominal_spread18340.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18360) => {
                                                                                            source_ml_source2_data_property_value18360.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override18361 = (match match key.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18338) => {
                                                                                                let source_ml_source2_data_property_value18338 = *source_ml_source2_data_property_value18338;
                                                                                                source_ml_source2_data_property_value18338.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18339, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18339
                                                                                            }
                                                                                        } + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18341.clone()), Box::new(source_ml_source2_nominal_field18343.clone()), source_ml_source2_nominal_field18345.clone(), source_ml_source2_nominal_override18361, source_ml_source2_nominal_field18349, source_ml_source2_nominal_field18351.clone(), source_ml_source2_nominal_field18353, source_ml_source2_nominal_field18355.clone(), source_ml_source2_nominal_field18357.clone(), source_ml_source2_nominal_field18359.clone())
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
                                        match key.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18362) => {
                                                let source_ml_source2_data_property_value18362 = *source_ml_source2_data_property_value18362;
                                                source_ml_source2_data_property_value18362.clone()
                                            }
                                        }
                                    };
                                    {
                                        let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, prepared.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                                        {
                                            let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                {
                                                    let source_ml_source2_constructor_field18364 = match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18363) => {
                                                            let source_ml_source2_data_property_value18363 = *source_ml_source2_data_property_value18363;
                                                            source_ml_source2_data_property_value18363.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_constructor_field18366 = self::JavaScriptCallSource::SourceInitializeCell(binding, Box::new(self::JavaScriptCallSource::SourceBinding(value_id)), Box::new(match body.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18365, _) => {
                                                                let source_ml_source2_data_property_value18365 = *source_ml_source2_data_property_value18365;
                                                                source_ml_source2_data_property_value18365.clone()
                                                            }
                                                        }));
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field18366.clone()), Box::new(source_ml_source2_constructor_field18364.clone()))
                                                    }
                                                }
                                            } else {
                                                self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match body.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18367) => {
                                                        let source_ml_source2_data_property_value18367 = *source_ml_source2_data_property_value18367;
                                                        source_ml_source2_data_property_value18367.clone()
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18368, _) => {
                                                        let source_ml_source2_data_property_value18368 = *source_ml_source2_data_property_value18368;
                                                        source_ml_source2_data_property_value18368.clone()
                                                    }
                                                }, mutable, 0f64, false)
                                            };
                                            {
                                                let current = match initialized.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18369) => {
                                                        let source_ml_source2_data_property_value18369 = *source_ml_source2_data_property_value18369;
                                                        source_ml_source2_data_property_value18369.clone()
                                                    }
                                                };
                                                {
                                                    let iteration = {
                                                        let source_ml_source2_constructor_field18373 = match match current.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18371, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18371 = *source_ml_source2_data_property_value18371;
                                                                source_ml_source2_data_property_value18371.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18372, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18372.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field18375 = match initialized.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18374, _) => {
                                                                    let source_ml_source2_data_property_value18374 = *source_ml_source2_data_property_value18374;
                                                                    source_ml_source2_data_property_value18374.clone()
                                                                }
                                                            };
                                                            self::JavaScriptCallSource::SourceLexicalScope(owned.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field18373.clone(), Box::new(source_ml_source2_constructor_field18375.clone()))
                                                        }
                                                    };
                                                    {
                                                        let temporary = match current.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18376, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18376
                                                            }
                                                        };
                                                        {
                                                            let current_2 = {
                                                                let source_ml_source2_nominal_field18378 = match current.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18379, _, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value18379 = *source_ml_source2_data_property_value18379;
                                                                        source_ml_source2_data_property_value18379.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18380 = match current.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18381, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18381 = *source_ml_source2_data_property_value18381;
                                                                            source_ml_source2_data_property_value18381.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18382 = match current.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18383, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18383.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18384 = match current.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18385, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18385
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18386 = match current.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18387, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18387
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18388 = match current.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18389, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value18389.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18390 = match current.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18391, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18391
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18392 = match current.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18393, _, _) => {
                                                                                                    source_ml_source2_data_property_value18393.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field18394 = match current.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18395, _) => {
                                                                                                        source_ml_source2_data_property_value18395.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field18396 = match current.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18397) => {
                                                                                                            source_ml_source2_data_property_value18397.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override18398 = (temporary + 1f64);
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18378.clone()), Box::new(source_ml_source2_nominal_field18380.clone()), source_ml_source2_nominal_field18382.clone(), source_ml_source2_nominal_override18398, source_ml_source2_nominal_field18386, source_ml_source2_nominal_field18388.clone(), source_ml_source2_nominal_field18390, source_ml_source2_nominal_field18392.clone(), source_ml_source2_nominal_field18394.clone(), source_ml_source2_nominal_field18396.clone())
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
                                                                let source = self::construct_java_script_error_local_binding(current_2.clone(), temporary, {
                                                                    let source_ml_source2_constructor_field18404 = match input.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18403, _) => {
                                                                            let source_ml_source2_data_property_value18403 = *source_ml_source2_data_property_value18403;
                                                                            source_ml_source2_data_property_value18403.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field18406 = match key.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18405, _) => {
                                                                                let source_ml_source2_data_property_value18405 = *source_ml_source2_data_property_value18405;
                                                                                source_ml_source2_data_property_value18405.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field18410 = match match current_2.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18408, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value18408 = *source_ml_source2_data_property_value18408;
                                                                                    source_ml_source2_data_property_value18408.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18409, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18409.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_constructor_field18411 = Some(owned.clone());
                                                                                self::JavaScriptCallSource::SourceForOf(target, target, value_id, false, Box::new(source_ml_source2_constructor_field18404.clone()), Box::new(source_ml_source2_constructor_field18406.clone()), Box::new(iteration.clone()), source_ml_source2_constructor_field18410.clone(), Some((source_ml_source2_constructor_field18411.clone(),)))
                                                                            }
                                                                        }
                                                                    }
                                                                }, continuation.clone(), false);
                                                                {
                                                                    let source_2 = {
                                                                        let source_ml_source2_constructor_field18415 = match match current_2.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18413, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value18413 = *source_ml_source2_data_property_value18413;
                                                                                source_ml_source2_data_property_value18413.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18414, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18414.clone()
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSource::SourceLexicalScope(owned.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field18415.clone(), Box::new(source.clone()))
                                                                    };
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_2.clone()), Box::new(current_2.clone()))
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
            self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 102, 111, 114, 45, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 111, 119, 110, 101, 100, 32, 104, 101, 97, 100, 32, 98, 105, 110, 100, 105, 110, 103, 115])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_source_variable_initializer(context: self::JavaScriptErrorSourceContext, identity: f64) -> self::JavaScriptCallCellInitializer {
    {
        let binding = { let ml_values365 = { let ml_reference_value366 = ((match context.clone() {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18417) => {
                ((source_ml_source2_data_property_value18417.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value366 }; let ml_callback365 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18419, _, _, _, _) => {
                source_ml_source2_data_property_value18419
            }
        }) == identity) }) }; ml_values365.iter().cloned().enumerate().find_map(|(ml_index365, ml_value365)| if ml_callback365(ml_value365.clone()) { Some(ml_value365) } else { None }) };
        {
            let value = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
            {
                let value_3 = if !(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                    {
                        let parameter = match binding.clone().expect("a proven present value") {
                            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value18420) => {
                                ((source_ml_source2_data_property_value18420.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                            }
                        };
                        if {
                            let source_ml_source2_comparison_operand18422 = None::<()>;
                            !(match (parameter.clone(), source_ml_source2_comparison_operand18422.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                        } {
                            {
                                let value_2 = self::construct_java_script_lexical_cell_read(context.clone(), parameter.clone().expect("a proven present value"));
                                value_2.clone()
                            }
                        } else {
                            value.clone()
                        }
                    }
                } else {
                    value.clone()
                };
                self::JavaScriptCallCellInitializer::SourceCellInitializer(identity, Box::new(value_3.clone()), true)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn collect_java_script_source_variable_declarations(statements: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, context: self::JavaScriptErrorSourceContext, depth: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (depth > (128f64)) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        {
            let identities = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let source_ml_source1_temporary_index238 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s18423 = self::source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop241(context.clone(), depth, identities.clone(), statements.clone(), source_ml_source1_temporary_index238.clone());
                    match source_ml_source2_s18423.clone() {
                        self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop241Result::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop241Done => {
                            identities.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_variable_for_of(statement: self::JavaScriptSourceStatement, context: self::JavaScriptErrorSourceContext, continuation: self::JavaScriptCallSource, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match statement.clone() {
        self::JavaScriptSourceStatement::SourceForOfStatement(statement_target, binding, _, variable, binding_target, statement_input, iterator_key, statement_body, statement_pattern) => {
            let statement_input = *statement_input;
            let iterator_key = *iterator_key;
            if {
                let source_ml_source2_comparison_operand18469 = ((variable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                !(match (source_ml_source2_comparison_operand18469.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
            } {
                self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![118, 97, 114, 32, 102, 111, 114, 45, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 45, 115, 99, 111, 112, 101, 100, 32, 104, 101, 97, 100])))
            } else {
                {
                    let pattern = ((statement_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                    {
                        let target = ((binding_target.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone();
                        {
                            let input = self::lower_java_script_error_expression(statement_input.clone(), context.clone(), (depth + 1f64));
                            {
                                let key = self::lower_java_script_error_expression(iterator_key.clone(), match input.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18471) => {
                                        let source_ml_source2_data_property_value18471 = *source_ml_source2_data_property_value18471;
                                        source_ml_source2_data_property_value18471.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_id = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        match match key.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18472) => {
                                                let source_ml_source2_data_property_value18472 = *source_ml_source2_data_property_value18472;
                                                source_ml_source2_data_property_value18472.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18473, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18473
                                            }
                                        }
                                    } else {
                                        match pattern.clone().expect("a proven present value") {
                                            self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value18474, _, _) => {
                                                source_ml_source2_data_property_value18474
                                            }
                                            self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value18475, _) => {
                                                source_ml_source2_data_property_value18475
                                            }
                                            self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value18476, _, _) => {
                                                source_ml_source2_data_property_value18476
                                            }
                                            self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value18477, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18477
                                            }
                                            self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value18478, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18478
                                            }
                                        }
                                    };
                                    {
                                        let prepared = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source2_nominal_spread18482 = match key.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18479) => {
                                                        let source_ml_source2_data_property_value18479 = *source_ml_source2_data_property_value18479;
                                                        source_ml_source2_data_property_value18479.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18483 = match source_ml_source2_nominal_spread18482.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18484, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value18484 = *source_ml_source2_data_property_value18484;
                                                            source_ml_source2_data_property_value18484.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18485 = match source_ml_source2_nominal_spread18482.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18486, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18486 = *source_ml_source2_data_property_value18486;
                                                                source_ml_source2_data_property_value18486.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18487 = match source_ml_source2_nominal_spread18482.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18488, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18488.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18489 = match source_ml_source2_nominal_spread18482.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18490, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18490
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18491 = match source_ml_source2_nominal_spread18482.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18492, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18492
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18493 = match source_ml_source2_nominal_spread18482.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18494, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18494.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18495 = match source_ml_source2_nominal_spread18482.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18496, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18496
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18497 = match source_ml_source2_nominal_spread18482.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18498, _, _) => {
                                                                                        source_ml_source2_data_property_value18498.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18499 = match source_ml_source2_nominal_spread18482.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18500, _) => {
                                                                                            source_ml_source2_data_property_value18500.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18501 = match source_ml_source2_nominal_spread18482.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18502) => {
                                                                                                source_ml_source2_data_property_value18502.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override18503 = (match match key.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18480) => {
                                                                                                    let source_ml_source2_data_property_value18480 = *source_ml_source2_data_property_value18480;
                                                                                                    source_ml_source2_data_property_value18480.clone()
                                                                                                }
                                                                                            } {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18481, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value18481
                                                                                                }
                                                                                            } + 1f64);
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18483.clone()), Box::new(source_ml_source2_nominal_field18485.clone()), source_ml_source2_nominal_field18487.clone(), source_ml_source2_nominal_override18503, source_ml_source2_nominal_field18491, source_ml_source2_nominal_field18493.clone(), source_ml_source2_nominal_field18495, source_ml_source2_nominal_field18497.clone(), source_ml_source2_nominal_field18499.clone(), source_ml_source2_nominal_field18501.clone())
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
                                            match key.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18504) => {
                                                    let source_ml_source2_data_property_value18504 = *source_ml_source2_data_property_value18504;
                                                    source_ml_source2_data_property_value18504.clone()
                                                }
                                            }
                                        };
                                        {
                                            let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, prepared.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                                            {
                                                let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    {
                                                        let source_ml_source2_constructor_field18506 = match body.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18505) => {
                                                                let source_ml_source2_data_property_value18505 = *source_ml_source2_data_property_value18505;
                                                                source_ml_source2_data_property_value18505.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field18509 = if match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::JavaScriptCallSource::SourceInitializeCell(binding, Box::new(self::JavaScriptCallSource::SourceBinding(value_id)), Box::new(match body.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18507, _) => {
                                                                        let source_ml_source2_data_property_value18507 = *source_ml_source2_data_property_value18507;
                                                                        source_ml_source2_data_property_value18507.clone()
                                                                    }
                                                                }))
                                                            } else {
                                                                self::construct_java_script_source_existing_write(target.clone().expect("a proven present value"), self::JavaScriptCallSource::SourceBinding(value_id), match body.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18508, _) => {
                                                                        let source_ml_source2_data_property_value18508 = *source_ml_source2_data_property_value18508;
                                                                        source_ml_source2_data_property_value18508.clone()
                                                                    }
                                                                })
                                                            };
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field18509.clone()), Box::new(source_ml_source2_constructor_field18506.clone()))
                                                        }
                                                    }
                                                } else {
                                                    self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18510) => {
                                                            let source_ml_source2_data_property_value18510 = *source_ml_source2_data_property_value18510;
                                                            source_ml_source2_data_property_value18510.clone()
                                                        }
                                                    }, match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18511, _) => {
                                                            let source_ml_source2_data_property_value18511 = *source_ml_source2_data_property_value18511;
                                                            source_ml_source2_data_property_value18511.clone()
                                                        }
                                                    }, true, 0f64, false)
                                                };
                                                {
                                                    let temporary = match match initialized.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18512) => {
                                                            let source_ml_source2_data_property_value18512 = *source_ml_source2_data_property_value18512;
                                                            source_ml_source2_data_property_value18512.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18513, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value18513
                                                        }
                                                    };
                                                    {
                                                        let next = {
                                                            let source_ml_source2_nominal_spread18515 = match initialized.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18514) => {
                                                                    let source_ml_source2_data_property_value18514 = *source_ml_source2_data_property_value18514;
                                                                    source_ml_source2_data_property_value18514.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18516 = match source_ml_source2_nominal_spread18515.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18517, _, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value18517 = *source_ml_source2_data_property_value18517;
                                                                        source_ml_source2_data_property_value18517.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18518 = match source_ml_source2_nominal_spread18515.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18519, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18519 = *source_ml_source2_data_property_value18519;
                                                                            source_ml_source2_data_property_value18519.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18520 = match source_ml_source2_nominal_spread18515.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18521, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18521.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18522 = match source_ml_source2_nominal_spread18515.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18523, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18523
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18524 = match source_ml_source2_nominal_spread18515.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18525, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18525
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18526 = match source_ml_source2_nominal_spread18515.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18527, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value18527.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18528 = match source_ml_source2_nominal_spread18515.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18529, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18529
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18530 = match source_ml_source2_nominal_spread18515.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18531, _, _) => {
                                                                                                    source_ml_source2_data_property_value18531.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field18532 = match source_ml_source2_nominal_spread18515.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18533, _) => {
                                                                                                        source_ml_source2_data_property_value18533.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field18534 = match source_ml_source2_nominal_spread18515.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18535) => {
                                                                                                            source_ml_source2_data_property_value18535.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override18536 = (temporary + 1f64);
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18516.clone()), Box::new(source_ml_source2_nominal_field18518.clone()), source_ml_source2_nominal_field18520.clone(), source_ml_source2_nominal_override18536, source_ml_source2_nominal_field18524, source_ml_source2_nominal_field18526.clone(), source_ml_source2_nominal_field18528, source_ml_source2_nominal_field18530.clone(), source_ml_source2_nominal_field18532.clone(), source_ml_source2_nominal_field18534.clone())
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
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), temporary, {
                                                            let source_ml_source2_constructor_field18542 = match input.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18541, _) => {
                                                                    let source_ml_source2_data_property_value18541 = *source_ml_source2_data_property_value18541;
                                                                    source_ml_source2_data_property_value18541.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_constructor_field18544 = match key.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18543, _) => {
                                                                        let source_ml_source2_data_property_value18543 = *source_ml_source2_data_property_value18543;
                                                                        source_ml_source2_data_property_value18543.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_constructor_field18546 = match initialized.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18545, _) => {
                                                                            let source_ml_source2_data_property_value18545 = *source_ml_source2_data_property_value18545;
                                                                            source_ml_source2_data_property_value18545.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field18549 = match match next.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18547, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value18547 = *source_ml_source2_data_property_value18547;
                                                                                source_ml_source2_data_property_value18547.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18548, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18548.clone()
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSource::SourceForOf(statement_target, statement_target, value_id, false, Box::new(source_ml_source2_constructor_field18542.clone()), Box::new(source_ml_source2_constructor_field18544.clone()), Box::new(source_ml_source2_constructor_field18546.clone()), source_ml_source2_constructor_field18549.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>)
                                                                    }
                                                                }
                                                            }
                                                        }, continuation.clone(), false)), Box::new(next.clone()))
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
            self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![118, 97, 114, 32, 102, 111, 114, 45, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 45, 115, 99, 111, 112, 101, 100, 32, 104, 101, 97, 100])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_source_existing_write(target: self::JavaScriptSourceBindingTarget, value: self::JavaScriptCallSource, body: self::JavaScriptCallSource) -> self::JavaScriptCallSource {
    self::JavaScriptCallSource::SourceWriteExisting(match target.clone() {
        self::JavaScriptSourceBindingTarget::SourceCatchBindingTarget(source_ml_source2_data_property_value18550, _) => {
            source_ml_source2_data_property_value18550
        }
    }, match target.clone() {
        self::JavaScriptSourceBindingTarget::SourceCatchBindingTarget(_, source_ml_source2_data_property_value18551) => {
            source_ml_source2_data_property_value18551
        }
    }, Box::new(value.clone()), Box::new(body.clone()))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ({ let ml_reference_array367 = units.clone(); let ml_reference_projection367 = ml_reference_array367.borrow().len() as f64; ml_reference_projection367 })) {
            {
                let unit = ({ let ml_reference_array368 = units.clone(); let ml_reference_position368 = at; let ml_reference_projection368 = self::ml_array::optional_at(&*ml_reference_array368.borrow(), self::ml_array::number_index(ml_reference_position368)); ml_reference_projection368 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Result {
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Result {
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Result {
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_call_code_functions_loop12(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut end: f64, mut index: f64) -> self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Result {
    loop {
        return if (index < end) {
            {
                let value = { let ml_reference_array369 = ordered.clone(); let ml_reference_position369 = index; let ml_reference_projection369 = self::ml_array::optional_at(&*ml_reference_array369.borrow(), self::ml_array::number_index(ml_reference_position369)); ml_reference_projection369 };
                if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value370 = functions.borrow().clone(); ml_reference_value370 }, 0f64, f64::INFINITY))))
                } else {
                    {
                        let merged_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value371 = merged.borrow().clone(); ml_reference_value371 }, vec![value.clone().expect("a proven present value")])));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (merged, index) = (merged_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Done(merged.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Result {
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}
