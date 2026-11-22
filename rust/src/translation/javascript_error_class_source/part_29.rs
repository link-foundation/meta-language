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
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18430) => {
                                    let source_ml_source2_data_property_value18430 = *source_ml_source2_data_property_value18430;
                                    source_ml_source2_data_property_value18430.clone()
                                }
                            }, (depth + 1f64));
                            {
                                let value_id = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    match match key.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18431) => {
                                            let source_ml_source2_data_property_value18431 = *source_ml_source2_data_property_value18431;
                                            source_ml_source2_data_property_value18431.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18432, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18432
                                        }
                                    }
                                } else {
                                    match pattern.clone().expect("a proven present value") {
                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value18433, _, _) => {
                                            source_ml_source2_data_property_value18433
                                        }
                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value18434, _) => {
                                            source_ml_source2_data_property_value18434
                                        }
                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value18435, _, _) => {
                                            source_ml_source2_data_property_value18435
                                        }
                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value18436, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18436
                                        }
                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value18437, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18437
                                        }
                                    }
                                };
                                {
                                    let prepared = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        {
                                            let source_ml_source2_nominal_spread18441 = match key.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18438) => {
                                                    let source_ml_source2_data_property_value18438 = *source_ml_source2_data_property_value18438;
                                                    source_ml_source2_data_property_value18438.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field18442 = match source_ml_source2_nominal_spread18441.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18443, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value18443 = *source_ml_source2_data_property_value18443;
                                                        source_ml_source2_data_property_value18443.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18444 = match source_ml_source2_nominal_spread18441.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18445, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value18445 = *source_ml_source2_data_property_value18445;
                                                            source_ml_source2_data_property_value18445.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18446 = match source_ml_source2_nominal_spread18441.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18447, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18447.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18448 = match source_ml_source2_nominal_spread18441.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18449, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18449
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18450 = match source_ml_source2_nominal_spread18441.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18451, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18451
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18452 = match source_ml_source2_nominal_spread18441.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18453, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18453.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18454 = match source_ml_source2_nominal_spread18441.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18455, _, _, _) => {
                                                                                source_ml_source2_data_property_value18455
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18456 = match source_ml_source2_nominal_spread18441.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18457, _, _) => {
                                                                                    source_ml_source2_data_property_value18457.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18458 = match source_ml_source2_nominal_spread18441.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18459, _) => {
                                                                                        source_ml_source2_data_property_value18459.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18460 = match source_ml_source2_nominal_spread18441.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18461) => {
                                                                                            source_ml_source2_data_property_value18461.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override18462 = (match match key.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18439) => {
                                                                                                let source_ml_source2_data_property_value18439 = *source_ml_source2_data_property_value18439;
                                                                                                source_ml_source2_data_property_value18439.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18440, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18440
                                                                                            }
                                                                                        } + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18442.clone()), Box::new(source_ml_source2_nominal_field18444.clone()), source_ml_source2_nominal_field18446.clone(), source_ml_source2_nominal_override18462, source_ml_source2_nominal_field18450, source_ml_source2_nominal_field18452.clone(), source_ml_source2_nominal_field18454, source_ml_source2_nominal_field18456.clone(), source_ml_source2_nominal_field18458.clone(), source_ml_source2_nominal_field18460.clone())
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18463) => {
                                                let source_ml_source2_data_property_value18463 = *source_ml_source2_data_property_value18463;
                                                source_ml_source2_data_property_value18463.clone()
                                            }
                                        }
                                    };
                                    {
                                        let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, prepared.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                                        {
                                            let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                {
                                                    let source_ml_source2_constructor_field18465 = match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18464) => {
                                                            let source_ml_source2_data_property_value18464 = *source_ml_source2_data_property_value18464;
                                                            source_ml_source2_data_property_value18464.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_constructor_field18467 = self::JavaScriptCallSource::SourceInitializeCell(binding, Box::new(self::JavaScriptCallSource::SourceBinding(value_id)), Box::new(match body.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18466, _) => {
                                                                let source_ml_source2_data_property_value18466 = *source_ml_source2_data_property_value18466;
                                                                source_ml_source2_data_property_value18466.clone()
                                                            }
                                                        }));
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field18467.clone()), Box::new(source_ml_source2_constructor_field18465.clone()))
                                                    }
                                                }
                                            } else {
                                                self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match body.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18468) => {
                                                        let source_ml_source2_data_property_value18468 = *source_ml_source2_data_property_value18468;
                                                        source_ml_source2_data_property_value18468.clone()
                                                    }
                                                }, match body.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18469, _) => {
                                                        let source_ml_source2_data_property_value18469 = *source_ml_source2_data_property_value18469;
                                                        source_ml_source2_data_property_value18469.clone()
                                                    }
                                                }, mutable, 0f64, false)
                                            };
                                            {
                                                let current = match initialized.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18470) => {
                                                        let source_ml_source2_data_property_value18470 = *source_ml_source2_data_property_value18470;
                                                        source_ml_source2_data_property_value18470.clone()
                                                    }
                                                };
                                                {
                                                    let iteration = {
                                                        let source_ml_source2_constructor_field18474 = match match current.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18472, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18472 = *source_ml_source2_data_property_value18472;
                                                                source_ml_source2_data_property_value18472.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18473, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18473.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field18476 = match initialized.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18475, _) => {
                                                                    let source_ml_source2_data_property_value18475 = *source_ml_source2_data_property_value18475;
                                                                    source_ml_source2_data_property_value18475.clone()
                                                                }
                                                            };
                                                            self::JavaScriptCallSource::SourceLexicalScope(owned.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field18474.clone(), Box::new(source_ml_source2_constructor_field18476.clone()))
                                                        }
                                                    };
                                                    {
                                                        let temporary = match current.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18477, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18477
                                                            }
                                                        };
                                                        {
                                                            let current_2 = {
                                                                let source_ml_source2_nominal_field18479 = match current.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18480, _, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value18480 = *source_ml_source2_data_property_value18480;
                                                                        source_ml_source2_data_property_value18480.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18481 = match current.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18482, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18482 = *source_ml_source2_data_property_value18482;
                                                                            source_ml_source2_data_property_value18482.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18483 = match current.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18484, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18484.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18485 = match current.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18486, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18486
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18487 = match current.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18488, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18488
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18489 = match current.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18490, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value18490.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18491 = match current.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18492, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18492
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18493 = match current.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18494, _, _) => {
                                                                                                    source_ml_source2_data_property_value18494.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field18495 = match current.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18496, _) => {
                                                                                                        source_ml_source2_data_property_value18496.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field18497 = match current.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18498) => {
                                                                                                            source_ml_source2_data_property_value18498.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override18499 = (temporary + 1f64);
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18479.clone()), Box::new(source_ml_source2_nominal_field18481.clone()), source_ml_source2_nominal_field18483.clone(), source_ml_source2_nominal_override18499, source_ml_source2_nominal_field18487, source_ml_source2_nominal_field18489.clone(), source_ml_source2_nominal_field18491, source_ml_source2_nominal_field18493.clone(), source_ml_source2_nominal_field18495.clone(), source_ml_source2_nominal_field18497.clone())
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
                                                                    let source_ml_source2_constructor_field18505 = match input.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18504, _) => {
                                                                            let source_ml_source2_data_property_value18504 = *source_ml_source2_data_property_value18504;
                                                                            source_ml_source2_data_property_value18504.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field18507 = match key.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18506, _) => {
                                                                                let source_ml_source2_data_property_value18506 = *source_ml_source2_data_property_value18506;
                                                                                source_ml_source2_data_property_value18506.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field18511 = match match current_2.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18509, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value18509 = *source_ml_source2_data_property_value18509;
                                                                                    source_ml_source2_data_property_value18509.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18510, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18510.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_constructor_field18512 = Some(owned.clone());
                                                                                self::JavaScriptCallSource::SourceForOf(target, target, value_id, false, Box::new(source_ml_source2_constructor_field18505.clone()), Box::new(source_ml_source2_constructor_field18507.clone()), Box::new(iteration.clone()), source_ml_source2_constructor_field18511.clone(), Some((source_ml_source2_constructor_field18512.clone(),)))
                                                                            }
                                                                        }
                                                                    }
                                                                }, continuation.clone(), false);
                                                                {
                                                                    let source_2 = {
                                                                        let source_ml_source2_constructor_field18516 = match match current_2.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18514, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value18514 = *source_ml_source2_data_property_value18514;
                                                                                source_ml_source2_data_property_value18514.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18515, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18515.clone()
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSource::SourceLexicalScope(owned.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field18516.clone(), Box::new(source.clone()))
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
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18518) => {
                ((source_ml_source2_data_property_value18518.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value366 }; let ml_callback365 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18520, _, _, _, _) => {
                source_ml_source2_data_property_value18520
            }
        }) == identity) }) }; ml_values365.iter().cloned().enumerate().find_map(|(ml_index365, ml_value365)| if ml_callback365(ml_value365.clone()) { Some(ml_value365) } else { None }) };
        {
            let value = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
            {
                let value_3 = if !(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                    {
                        let parameter = match binding.clone().expect("a proven present value") {
                            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value18521) => {
                                ((source_ml_source2_data_property_value18521.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                            }
                        };
                        if {
                            let source_ml_source2_comparison_operand18523 = None::<()>;
                            !(match (parameter.clone(), source_ml_source2_comparison_operand18523.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
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
                    let source_ml_source2_s18524 = self::source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop241(context.clone(), depth, identities.clone(), statements.clone(), source_ml_source1_temporary_index238.clone());
                    match source_ml_source2_s18524.clone() {
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
                let source_ml_source2_comparison_operand18570 = ((variable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                !(match (source_ml_source2_comparison_operand18570.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
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
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18572) => {
                                        let source_ml_source2_data_property_value18572 = *source_ml_source2_data_property_value18572;
                                        source_ml_source2_data_property_value18572.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let value_id = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        match match key.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18573) => {
                                                let source_ml_source2_data_property_value18573 = *source_ml_source2_data_property_value18573;
                                                source_ml_source2_data_property_value18573.clone()
                                            }
                                        } {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18574, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18574
                                            }
                                        }
                                    } else {
                                        match pattern.clone().expect("a proven present value") {
                                            self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value18575, _, _) => {
                                                source_ml_source2_data_property_value18575
                                            }
                                            self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value18576, _) => {
                                                source_ml_source2_data_property_value18576
                                            }
                                            self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value18577, _, _) => {
                                                source_ml_source2_data_property_value18577
                                            }
                                            self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value18578, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18578
                                            }
                                            self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value18579, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18579
                                            }
                                        }
                                    };
                                    {
                                        let prepared = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            {
                                                let source_ml_source2_nominal_spread18583 = match key.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18580) => {
                                                        let source_ml_source2_data_property_value18580 = *source_ml_source2_data_property_value18580;
                                                        source_ml_source2_data_property_value18580.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18584 = match source_ml_source2_nominal_spread18583.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18585, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value18585 = *source_ml_source2_data_property_value18585;
                                                            source_ml_source2_data_property_value18585.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18586 = match source_ml_source2_nominal_spread18583.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18587, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18587 = *source_ml_source2_data_property_value18587;
                                                                source_ml_source2_data_property_value18587.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18588 = match source_ml_source2_nominal_spread18583.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18589, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18589.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18590 = match source_ml_source2_nominal_spread18583.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18591, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18591
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18592 = match source_ml_source2_nominal_spread18583.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18593, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18593
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18594 = match source_ml_source2_nominal_spread18583.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18595, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18595.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18596 = match source_ml_source2_nominal_spread18583.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18597, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18597
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18598 = match source_ml_source2_nominal_spread18583.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18599, _, _) => {
                                                                                        source_ml_source2_data_property_value18599.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18600 = match source_ml_source2_nominal_spread18583.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18601, _) => {
                                                                                            source_ml_source2_data_property_value18601.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18602 = match source_ml_source2_nominal_spread18583.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18603) => {
                                                                                                source_ml_source2_data_property_value18603.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_override18604 = (match match key.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18581) => {
                                                                                                    let source_ml_source2_data_property_value18581 = *source_ml_source2_data_property_value18581;
                                                                                                    source_ml_source2_data_property_value18581.clone()
                                                                                                }
                                                                                            } {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18582, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value18582
                                                                                                }
                                                                                            } + 1f64);
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18584.clone()), Box::new(source_ml_source2_nominal_field18586.clone()), source_ml_source2_nominal_field18588.clone(), source_ml_source2_nominal_override18604, source_ml_source2_nominal_field18592, source_ml_source2_nominal_field18594.clone(), source_ml_source2_nominal_field18596, source_ml_source2_nominal_field18598.clone(), source_ml_source2_nominal_field18600.clone(), source_ml_source2_nominal_field18602.clone())
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
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18605) => {
                                                    let source_ml_source2_data_property_value18605 = *source_ml_source2_data_property_value18605;
                                                    source_ml_source2_data_property_value18605.clone()
                                                }
                                            }
                                        };
                                        {
                                            let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, prepared.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                                            {
                                                let initialized = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    {
                                                        let source_ml_source2_constructor_field18607 = match body.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18606) => {
                                                                let source_ml_source2_data_property_value18606 = *source_ml_source2_data_property_value18606;
                                                                source_ml_source2_data_property_value18606.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field18610 = if match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::JavaScriptCallSource::SourceInitializeCell(binding, Box::new(self::JavaScriptCallSource::SourceBinding(value_id)), Box::new(match body.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18608, _) => {
                                                                        let source_ml_source2_data_property_value18608 = *source_ml_source2_data_property_value18608;
                                                                        source_ml_source2_data_property_value18608.clone()
                                                                    }
                                                                }))
                                                            } else {
                                                                self::construct_java_script_source_existing_write(target.clone().expect("a proven present value"), self::JavaScriptCallSource::SourceBinding(value_id), match body.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18609, _) => {
                                                                        let source_ml_source2_data_property_value18609 = *source_ml_source2_data_property_value18609;
                                                                        source_ml_source2_data_property_value18609.clone()
                                                                    }
                                                                })
                                                            };
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_constructor_field18610.clone()), Box::new(source_ml_source2_constructor_field18607.clone()))
                                                        }
                                                    }
                                                } else {
                                                    self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18611) => {
                                                            let source_ml_source2_data_property_value18611 = *source_ml_source2_data_property_value18611;
                                                            source_ml_source2_data_property_value18611.clone()
                                                        }
                                                    }, match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18612, _) => {
                                                            let source_ml_source2_data_property_value18612 = *source_ml_source2_data_property_value18612;
                                                            source_ml_source2_data_property_value18612.clone()
                                                        }
                                                    }, true, 0f64, false)
                                                };
                                                {
                                                    let temporary = match match initialized.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18613) => {
                                                            let source_ml_source2_data_property_value18613 = *source_ml_source2_data_property_value18613;
                                                            source_ml_source2_data_property_value18613.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18614, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value18614
                                                        }
                                                    };
                                                    {
                                                        let next = {
                                                            let source_ml_source2_nominal_spread18616 = match initialized.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18615) => {
                                                                    let source_ml_source2_data_property_value18615 = *source_ml_source2_data_property_value18615;
                                                                    source_ml_source2_data_property_value18615.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field18617 = match source_ml_source2_nominal_spread18616.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18618, _, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value18618 = *source_ml_source2_data_property_value18618;
                                                                        source_ml_source2_data_property_value18618.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field18619 = match source_ml_source2_nominal_spread18616.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18620, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18620 = *source_ml_source2_data_property_value18620;
                                                                            source_ml_source2_data_property_value18620.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field18621 = match source_ml_source2_nominal_spread18616.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18622, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18622.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18623 = match source_ml_source2_nominal_spread18616.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18624, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value18624
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18625 = match source_ml_source2_nominal_spread18616.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18626, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18626
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18627 = match source_ml_source2_nominal_spread18616.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18628, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value18628.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18629 = match source_ml_source2_nominal_spread18616.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18630, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18630
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18631 = match source_ml_source2_nominal_spread18616.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18632, _, _) => {
                                                                                                    source_ml_source2_data_property_value18632.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field18633 = match source_ml_source2_nominal_spread18616.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18634, _) => {
                                                                                                        source_ml_source2_data_property_value18634.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field18635 = match source_ml_source2_nominal_spread18616.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18636) => {
                                                                                                            source_ml_source2_data_property_value18636.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override18637 = (temporary + 1f64);
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18617.clone()), Box::new(source_ml_source2_nominal_field18619.clone()), source_ml_source2_nominal_field18621.clone(), source_ml_source2_nominal_override18637, source_ml_source2_nominal_field18625, source_ml_source2_nominal_field18627.clone(), source_ml_source2_nominal_field18629, source_ml_source2_nominal_field18631.clone(), source_ml_source2_nominal_field18633.clone(), source_ml_source2_nominal_field18635.clone())
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
                                                            let source_ml_source2_constructor_field18643 = match input.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18642, _) => {
                                                                    let source_ml_source2_data_property_value18642 = *source_ml_source2_data_property_value18642;
                                                                    source_ml_source2_data_property_value18642.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_constructor_field18645 = match key.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18644, _) => {
                                                                        let source_ml_source2_data_property_value18644 = *source_ml_source2_data_property_value18644;
                                                                        source_ml_source2_data_property_value18644.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_constructor_field18647 = match initialized.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18646, _) => {
                                                                            let source_ml_source2_data_property_value18646 = *source_ml_source2_data_property_value18646;
                                                                            source_ml_source2_data_property_value18646.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_constructor_field18650 = match match next.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18648, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value18648 = *source_ml_source2_data_property_value18648;
                                                                                source_ml_source2_data_property_value18648.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18649, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value18649.clone()
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSource::SourceForOf(statement_target, statement_target, value_id, false, Box::new(source_ml_source2_constructor_field18643.clone()), Box::new(source_ml_source2_constructor_field18645.clone()), Box::new(source_ml_source2_constructor_field18647.clone()), source_ml_source2_constructor_field18650.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>)
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
        self::JavaScriptSourceBindingTarget::SourceCatchBindingTarget(source_ml_source2_data_property_value18651, _) => {
            source_ml_source2_data_property_value18651
        }
    }, match target.clone() {
        self::JavaScriptSourceBindingTarget::SourceCatchBindingTarget(_, source_ml_source2_data_property_value18652) => {
            source_ml_source2_data_property_value18652
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
