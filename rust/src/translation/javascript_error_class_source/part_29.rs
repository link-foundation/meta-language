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
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18332) => {
                                    let source_ml_source2_data_property_value18332 = *source_ml_source2_data_property_value18332;
                                    source_ml_source2_data_property_value18332.clone()
                                }
                            }, (depth + 1f64));
                            {
                                let value_id = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    match match key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18333) => {
                                            let source_ml_source2_data_property_value18333 = *source_ml_source2_data_property_value18333;
                                            source_ml_source2_data_property_value18333.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18334, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18334
                                        }
                                    }
                                } else {
                                    match pattern.clone().expect("a proven present value") {
                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value18335, _, _) => {
                                            source_ml_source2_data_property_value18335
                                        }
                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value18336, _) => {
                                            source_ml_source2_data_property_value18336
                                        }
                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value18337, _, _) => {
                                            source_ml_source2_data_property_value18337
                                        }
                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value18338, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18338
                                        }
                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value18339, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18339
                                        }
                                    }
                                };
                                {
                                    let prepared = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        {
                                            let source_ml_source2_nominal_spread18343 = match key.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18340) => {
                                                    let source_ml_source2_data_property_value18340 = *source_ml_source2_data_property_value18340;
                                                    source_ml_source2_data_property_value18340.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field18344 = match source_ml_source2_nominal_spread18343.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18345, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value18345 = *source_ml_source2_data_property_value18345;
                                                        source_ml_source2_data_property_value18345.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18346 = match source_ml_source2_nominal_spread18343.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18347, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value18347 = *source_ml_source2_data_property_value18347;
                                                            source_ml_source2_data_property_value18347.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18348 = match source_ml_source2_nominal_spread18343.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18349, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18349.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18350 = match source_ml_source2_nominal_spread18343.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18351, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18351
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18352 = match source_ml_source2_nominal_spread18343.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18353, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18353
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18354 = match source_ml_source2_nominal_spread18343.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18355, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18355.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18356 = match source_ml_source2_nominal_spread18343.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18357, _, _, _) => {
                                                                                source_ml_source2_data_property_value18357
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18358 = match source_ml_source2_nominal_spread18343.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18359, _, _) => {
                                                                                    source_ml_source2_data_property_value18359.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18360 = match source_ml_source2_nominal_spread18343.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18361, _) => {
                                                                                        source_ml_source2_data_property_value18361.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18362 = match source_ml_source2_nominal_spread18343.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18363) => {
                                                                                            source_ml_source2_data_property_value18363.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override18364 = (match match key.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18341) => {
                                                                                                let source_ml_source2_data_property_value18341 = *source_ml_source2_data_property_value18341;
                                                                                                source_ml_source2_data_property_value18341.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18342, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18342
                                                                                            }
                                                                                        } + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18344.clone()), Box::new(source_ml_source2_nominal_field18346.clone()), source_ml_source2_nominal_field18348.clone(), source_ml_source2_nominal_override18364, source_ml_source2_nominal_field18352, source_ml_source2_nominal_field18354.clone(), source_ml_source2_nominal_field18356, source_ml_source2_nominal_field18358.clone(), source_ml_source2_nominal_field18360.clone(), source_ml_source2_nominal_field18362.clone())
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18365) => {
                                                let source_ml_source2_data_property_value18365 = *source_ml_source2_data_property_value18365;
                                                source_ml_source2_data_property_value18365.clone()
                                            }
                                        }
                                    };
                                    {
                                        let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, prepared.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                                        {
                                            let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                {
                                                    let source_ml_source2_constructor_field18367 = match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18366) => {
                                                            let source_ml_source2_data_property_value18366 = *source_ml_source2_data_property_value18366;
                                                            source_ml_source2_data_property_value18366.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_constructor_field18369 = self::JavaScriptCallSource::SourceInitializeCell(binding, Box::new(self::JavaScriptCallSource::SourceBinding(value_id)), Box::new(match body.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18368, _) => {
                                                                let source_ml_source2_data_property_value18368 = *source_ml_source2_data_property_value18368;
                                                                source_ml_source2_data_property_value18368.clone()
                                                            }
                                                        }));
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field18369.clone()), Box::new(source_ml_source2_constructor_field18367.clone()))
                                                    }
                                                }
                                            } else {
                                                self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match body.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18370) => {
                                                        let source_ml_source2_data_property_value18370 = *source_ml_source2_data_property_value18370;
                                                        source_ml_source2_data_property_value18370.clone()
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18371, _) => {
                                                        let source_ml_source2_data_property_value18371 = *source_ml_source2_data_property_value18371;
                                                        source_ml_source2_data_property_value18371.clone()
                                                    }
                                                }, mutable, 0f64, false)
                                            };
                                            {
                                                let current = match initialized.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18372) => {
                                                        let source_ml_source2_data_property_value18372 = *source_ml_source2_data_property_value18372;
                                                        source_ml_source2_data_property_value18372.clone()
                                                    }
                                                };
                                                {
                                                    let iteration = {
                                                        let source_ml_source2_constructor_field18376 = match match current.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18374, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18374 = *source_ml_source2_data_property_value18374;
                                                                source_ml_source2_data_property_value18374.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18375, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18375.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field18378 = match initialized.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18377, _) => {
                                                                    let source_ml_source2_data_property_value18377 = *source_ml_source2_data_property_value18377;
                                                                    source_ml_source2_data_property_value18377.clone()
                                                                }
                                                            };
                                                            self::JavaScriptCallSource::SourceLexicalScope(owned.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field18376.clone(), Box::new(source_ml_source2_constructor_field18378.clone()))
                                                        }
                                                    };
                                                    {
                                                        let temporary = match current.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18379, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18379
                                                            }
                                                        };
                                                        {
                                                            let current_2 = {
                                                                let source_ml_source2_nominal_field18381 = match current.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18382, _, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value18382 = *source_ml_source2_data_property_value18382;
                                                                        source_ml_source2_data_property_value18382.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18383 = match current.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18384, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18384 = *source_ml_source2_data_property_value18384;
                                                                            source_ml_source2_data_property_value18384.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18385 = match current.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18386, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18386.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18387 = match current.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18388, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18388
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18389 = match current.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18390, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18390
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18391 = match current.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18392, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value18392.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18393 = match current.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18394, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18394
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18395 = match current.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18396, _, _) => {
                                                                                                    source_ml_source2_data_property_value18396.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field18397 = match current.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18398, _) => {
                                                                                                        source_ml_source2_data_property_value18398.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field18399 = match current.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18400) => {
                                                                                                            source_ml_source2_data_property_value18400.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override18401 = (temporary + 1f64);
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18381.clone()), Box::new(source_ml_source2_nominal_field18383.clone()), source_ml_source2_nominal_field18385.clone(), source_ml_source2_nominal_override18401, source_ml_source2_nominal_field18389, source_ml_source2_nominal_field18391.clone(), source_ml_source2_nominal_field18393, source_ml_source2_nominal_field18395.clone(), source_ml_source2_nominal_field18397.clone(), source_ml_source2_nominal_field18399.clone())
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
                                                                    let source_ml_source2_constructor_field18407 = match input.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18406, _) => {
                                                                            let source_ml_source2_data_property_value18406 = *source_ml_source2_data_property_value18406;
                                                                            source_ml_source2_data_property_value18406.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field18409 = match key.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18408, _) => {
                                                                                let source_ml_source2_data_property_value18408 = *source_ml_source2_data_property_value18408;
                                                                                source_ml_source2_data_property_value18408.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field18413 = match match current_2.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18411, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value18411 = *source_ml_source2_data_property_value18411;
                                                                                    source_ml_source2_data_property_value18411.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18412, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18412.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_constructor_field18414 = Some(owned.clone());
                                                                                self::JavaScriptCallSource::SourceForOf(target, target, value_id, false, Box::new(source_ml_source2_constructor_field18407.clone()), Box::new(source_ml_source2_constructor_field18409.clone()), Box::new(iteration.clone()), source_ml_source2_constructor_field18413.clone(), Some((source_ml_source2_constructor_field18414.clone(),)))
                                                                            }
                                                                        }
                                                                    }
                                                                }, continuation.clone(), false);
                                                                {
                                                                    let source_2 = {
                                                                        let source_ml_source2_constructor_field18418 = match match current_2.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18416, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value18416 = *source_ml_source2_data_property_value18416;
                                                                                source_ml_source2_data_property_value18416.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18417, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18417.clone()
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSource::SourceLexicalScope(owned.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field18418.clone(), Box::new(source.clone()))
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
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18420) => {
                ((source_ml_source2_data_property_value18420.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value366 }; let ml_callback365 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18422, _, _, _, _) => {
                source_ml_source2_data_property_value18422
            }
        }) == identity) }) }; ml_values365.iter().cloned().enumerate().find_map(|(ml_index365, ml_value365)| if ml_callback365(ml_value365.clone()) { Some(ml_value365) } else { None }) };
        {
            let value = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
            {
                let value_3 = if !(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                    {
                        let parameter = match binding.clone().expect("a proven present value") {
                            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value18423) => {
                                ((source_ml_source2_data_property_value18423.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                            }
                        };
                        if {
                            let source_ml_source2_comparison_operand18425 = None::<()>;
                            !(match (parameter.clone(), source_ml_source2_comparison_operand18425.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
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
                    let source_ml_source2_s18426 = self::source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop241(context.clone(), depth, identities.clone(), statements.clone(), source_ml_source1_temporary_index238.clone());
                    match source_ml_source2_s18426.clone() {
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
                let source_ml_source2_comparison_operand18472 = ((variable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                !(match (source_ml_source2_comparison_operand18472.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
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
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18474) => {
                                        let source_ml_source2_data_property_value18474 = *source_ml_source2_data_property_value18474;
                                        source_ml_source2_data_property_value18474.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_id = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        match match key.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18475) => {
                                                let source_ml_source2_data_property_value18475 = *source_ml_source2_data_property_value18475;
                                                source_ml_source2_data_property_value18475.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18476, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18476
                                            }
                                        }
                                    } else {
                                        match pattern.clone().expect("a proven present value") {
                                            self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value18477, _, _) => {
                                                source_ml_source2_data_property_value18477
                                            }
                                            self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value18478, _) => {
                                                source_ml_source2_data_property_value18478
                                            }
                                            self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value18479, _, _) => {
                                                source_ml_source2_data_property_value18479
                                            }
                                            self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value18480, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18480
                                            }
                                            self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value18481, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18481
                                            }
                                        }
                                    };
                                    {
                                        let prepared = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source2_nominal_spread18485 = match key.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18482) => {
                                                        let source_ml_source2_data_property_value18482 = *source_ml_source2_data_property_value18482;
                                                        source_ml_source2_data_property_value18482.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18486 = match source_ml_source2_nominal_spread18485.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18487, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value18487 = *source_ml_source2_data_property_value18487;
                                                            source_ml_source2_data_property_value18487.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18488 = match source_ml_source2_nominal_spread18485.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18489, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18489 = *source_ml_source2_data_property_value18489;
                                                                source_ml_source2_data_property_value18489.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18490 = match source_ml_source2_nominal_spread18485.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18491, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18491.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18492 = match source_ml_source2_nominal_spread18485.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18493, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18493
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18494 = match source_ml_source2_nominal_spread18485.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18495, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18495
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18496 = match source_ml_source2_nominal_spread18485.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18497, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18497.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18498 = match source_ml_source2_nominal_spread18485.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18499, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18499
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18500 = match source_ml_source2_nominal_spread18485.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18501, _, _) => {
                                                                                        source_ml_source2_data_property_value18501.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18502 = match source_ml_source2_nominal_spread18485.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18503, _) => {
                                                                                            source_ml_source2_data_property_value18503.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18504 = match source_ml_source2_nominal_spread18485.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18505) => {
                                                                                                source_ml_source2_data_property_value18505.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override18506 = (match match key.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18483) => {
                                                                                                    let source_ml_source2_data_property_value18483 = *source_ml_source2_data_property_value18483;
                                                                                                    source_ml_source2_data_property_value18483.clone()
                                                                                                }
                                                                                            } {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18484, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value18484
                                                                                                }
                                                                                            } + 1f64);
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18486.clone()), Box::new(source_ml_source2_nominal_field18488.clone()), source_ml_source2_nominal_field18490.clone(), source_ml_source2_nominal_override18506, source_ml_source2_nominal_field18494, source_ml_source2_nominal_field18496.clone(), source_ml_source2_nominal_field18498, source_ml_source2_nominal_field18500.clone(), source_ml_source2_nominal_field18502.clone(), source_ml_source2_nominal_field18504.clone())
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
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18507) => {
                                                    let source_ml_source2_data_property_value18507 = *source_ml_source2_data_property_value18507;
                                                    source_ml_source2_data_property_value18507.clone()
                                                }
                                            }
                                        };
                                        {
                                            let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, prepared.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                                            {
                                                let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    {
                                                        let source_ml_source2_constructor_field18509 = match body.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18508) => {
                                                                let source_ml_source2_data_property_value18508 = *source_ml_source2_data_property_value18508;
                                                                source_ml_source2_data_property_value18508.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field18512 = if match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::JavaScriptCallSource::SourceInitializeCell(binding, Box::new(self::JavaScriptCallSource::SourceBinding(value_id)), Box::new(match body.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18510, _) => {
                                                                        let source_ml_source2_data_property_value18510 = *source_ml_source2_data_property_value18510;
                                                                        source_ml_source2_data_property_value18510.clone()
                                                                    }
                                                                }))
                                                            } else {
                                                                self::construct_java_script_source_existing_write(target.clone().expect("a proven present value"), self::JavaScriptCallSource::SourceBinding(value_id), match body.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18511, _) => {
                                                                        let source_ml_source2_data_property_value18511 = *source_ml_source2_data_property_value18511;
                                                                        source_ml_source2_data_property_value18511.clone()
                                                                    }
                                                                })
                                                            };
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field18512.clone()), Box::new(source_ml_source2_constructor_field18509.clone()))
                                                        }
                                                    }
                                                } else {
                                                    self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18513) => {
                                                            let source_ml_source2_data_property_value18513 = *source_ml_source2_data_property_value18513;
                                                            source_ml_source2_data_property_value18513.clone()
                                                        }
                                                    }, match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18514, _) => {
                                                            let source_ml_source2_data_property_value18514 = *source_ml_source2_data_property_value18514;
                                                            source_ml_source2_data_property_value18514.clone()
                                                        }
                                                    }, true, 0f64, false)
                                                };
                                                {
                                                    let temporary = match match initialized.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18515) => {
                                                            let source_ml_source2_data_property_value18515 = *source_ml_source2_data_property_value18515;
                                                            source_ml_source2_data_property_value18515.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18516, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value18516
                                                        }
                                                    };
                                                    {
                                                        let next = {
                                                            let source_ml_source2_nominal_spread18518 = match initialized.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18517) => {
                                                                    let source_ml_source2_data_property_value18517 = *source_ml_source2_data_property_value18517;
                                                                    source_ml_source2_data_property_value18517.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18519 = match source_ml_source2_nominal_spread18518.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18520, _, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value18520 = *source_ml_source2_data_property_value18520;
                                                                        source_ml_source2_data_property_value18520.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18521 = match source_ml_source2_nominal_spread18518.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18522, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18522 = *source_ml_source2_data_property_value18522;
                                                                            source_ml_source2_data_property_value18522.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18523 = match source_ml_source2_nominal_spread18518.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18524, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18524.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18525 = match source_ml_source2_nominal_spread18518.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18526, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18526
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18527 = match source_ml_source2_nominal_spread18518.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18528, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18528
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18529 = match source_ml_source2_nominal_spread18518.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18530, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value18530.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18531 = match source_ml_source2_nominal_spread18518.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18532, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18532
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18533 = match source_ml_source2_nominal_spread18518.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18534, _, _) => {
                                                                                                    source_ml_source2_data_property_value18534.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field18535 = match source_ml_source2_nominal_spread18518.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18536, _) => {
                                                                                                        source_ml_source2_data_property_value18536.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field18537 = match source_ml_source2_nominal_spread18518.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18538) => {
                                                                                                            source_ml_source2_data_property_value18538.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override18539 = (temporary + 1f64);
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18519.clone()), Box::new(source_ml_source2_nominal_field18521.clone()), source_ml_source2_nominal_field18523.clone(), source_ml_source2_nominal_override18539, source_ml_source2_nominal_field18527, source_ml_source2_nominal_field18529.clone(), source_ml_source2_nominal_field18531, source_ml_source2_nominal_field18533.clone(), source_ml_source2_nominal_field18535.clone(), source_ml_source2_nominal_field18537.clone())
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
                                                            let source_ml_source2_constructor_field18545 = match input.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18544, _) => {
                                                                    let source_ml_source2_data_property_value18544 = *source_ml_source2_data_property_value18544;
                                                                    source_ml_source2_data_property_value18544.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_constructor_field18547 = match key.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18546, _) => {
                                                                        let source_ml_source2_data_property_value18546 = *source_ml_source2_data_property_value18546;
                                                                        source_ml_source2_data_property_value18546.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_constructor_field18549 = match initialized.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18548, _) => {
                                                                            let source_ml_source2_data_property_value18548 = *source_ml_source2_data_property_value18548;
                                                                            source_ml_source2_data_property_value18548.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field18552 = match match next.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18550, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value18550 = *source_ml_source2_data_property_value18550;
                                                                                source_ml_source2_data_property_value18550.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18551, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18551.clone()
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSource::SourceForOf(statement_target, statement_target, value_id, false, Box::new(source_ml_source2_constructor_field18545.clone()), Box::new(source_ml_source2_constructor_field18547.clone()), Box::new(source_ml_source2_constructor_field18549.clone()), source_ml_source2_constructor_field18552.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>)
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
        self::JavaScriptSourceBindingTarget::SourceCatchBindingTarget(source_ml_source2_data_property_value18553, _) => {
            source_ml_source2_data_property_value18553
        }
    }, match target.clone() {
        self::JavaScriptSourceBindingTarget::SourceCatchBindingTarget(_, source_ml_source2_data_property_value18554) => {
            source_ml_source2_data_property_value18554
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
