// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallCellInitializer;
use super::JavaScriptCallSource;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceBindingTarget;
use super::JavaScriptSourceStatement;
use super::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123;
use super::MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Result;
use super::collect_java_script_source_lexical_declarations;
use super::collect_java_script_source_variable_declarations;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_existing_write;
use super::lower_java_script_error_expression;
use super::lower_java_script_lexical_cell_write;
use super::lower_java_script_source_binding_pattern;
use super::lower_java_script_source_exception_continuation;
use super::lower_java_script_source_iteration_continuation;
use super::lower_java_script_source_local_declaration;
use super::lower_java_script_source_selection_continuation;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_lower_java_script_source_statements_loop124;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_statements(statements: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, first: f64, context: self::JavaScriptErrorSourceContext, continuation: self::JavaScriptCallSource, depth: f64, instantiate: bool) -> self::JavaScriptErrorSourceExpression {
    {
        let source_ml_source2_s16632 = if ((first == (0f64)) && instantiate) {
            {
                let hoisted = if (depth == (0f64)) {
                    self::collect_java_script_source_variable_declarations(statements.clone(), context.clone(), 0f64)
                } else {
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                };
                {
                    let declarations = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value16634 = self::collect_java_script_source_lexical_declarations(statements.clone(), context.clone(), 0f64);
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value259 = hoisted.borrow().clone(); ml_reference_value259 }), { let ml_reference_value260 = source_ml_source2_concat_value16634.borrow().clone(); ml_reference_value260 })
                    }));
                    if (({ let ml_reference_array261 = declarations.clone(); let ml_reference_projection261 = ml_reference_array261.borrow().len() as f64; ml_reference_projection261 }) > (0f64)) {
                        {
                            let body = self::lower_java_script_source_statements(statements.clone(), first, context.clone(), continuation.clone(), depth, false);
                            {
                                let initializers = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallCellInitializer>::new()));
                                {
                                    let source_ml_source1_temporary_index120 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s16635 = self::source_ml_source1_temporary_lower_java_script_source_statements_loop124(body.clone(), initializers.clone(), hoisted.clone(), source_ml_source1_temporary_index120.clone());
                                        match source_ml_source2_s16635.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Result::MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Done => {
                                                self::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123Return(Box::new({
                                                    let source_ml_source2_nominal_field16649 = match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16650, _) => {
                                                            let source_ml_source2_data_property_value16650 = *source_ml_source2_data_property_value16650;
                                                            source_ml_source2_data_property_value16650.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16651 = match body.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16652) => {
                                                                let source_ml_source2_data_property_value16652 = *source_ml_source2_data_property_value16652;
                                                                source_ml_source2_data_property_value16652.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override16653 = {
                                                                let source_ml_source1_temporary_constructor_field122 = match match match body.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16636) => {
                                                                        let source_ml_source2_data_property_value16636 = *source_ml_source2_data_property_value16636;
                                                                        source_ml_source2_data_property_value16636.clone()
                                                                    }
                                                                } {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16637, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value16637 = *source_ml_source2_data_property_value16637;
                                                                        source_ml_source2_data_property_value16637.clone()
                                                                    }
                                                                } {
                                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16638, _, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value16638.clone()
                                                                    }
                                                                };
                                                                if { let ml_value = { let ml_reference_array263 = hoisted.clone(); let ml_reference_projection263 = ml_reference_array263.borrow().len() as f64; ml_reference_projection263 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                    {
                                                                        let source_ml_source2_constructor_field16641 = Some(initializers.clone());
                                                                        {
                                                                            let source_ml_source2_constructor_field16643 = match body.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16642, _) => {
                                                                                    let source_ml_source2_data_property_value16642 = *source_ml_source2_data_property_value16642;
                                                                                    source_ml_source2_data_property_value16642.clone()
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSource::SourceLexicalScope(declarations.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Some((source_ml_source2_constructor_field16641.clone(),)), source_ml_source1_temporary_constructor_field122.clone(), Box::new(source_ml_source2_constructor_field16643.clone()))
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_constructor_field16647 = match body.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16646, _) => {
                                                                                let source_ml_source2_data_property_value16646 = *source_ml_source2_data_property_value16646;
                                                                                source_ml_source2_data_property_value16646.clone()
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSource::SourceLexicalScope(declarations.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source1_temporary_constructor_field122.clone(), Box::new(source_ml_source2_constructor_field16647.clone()))
                                                                    }
                                                                }
                                                            };
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override16653.clone()), Box::new(source_ml_source2_nominal_field16651.clone()))
                                                        }
                                                    }
                                                }))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123Next
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123Next
        };
        match source_ml_source2_s16632.clone() {
            self::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123Next => {
                if (((depth > (128f64)) || (first > (65536f64))) || (({ let ml_reference_array264 = statements.clone(); let ml_reference_projection264 = ml_reference_array264.borrow().len() as f64; ml_reference_projection264 }) > (65536f64))) {
                    self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 115, 116, 97, 116, 101, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 32, 98, 117, 100, 103, 101, 116])))
                } else {
                    {
                        let statement = { let ml_reference_array265 = statements.clone(); let ml_reference_position265 = first; let ml_reference_projection265 = self::ml_array::optional_at(&*ml_reference_array265.borrow(), self::ml_array::number_index(ml_reference_position265)); ml_reference_projection265 };
                        if match (statement.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(continuation.clone()), Box::new(context.clone()))
                        } else {
                            match statement.clone().expect("a proven present value") {
                                self::JavaScriptSourceStatement::SourceForOfStatement(_, _, _, _, _, _, _, _, _) => {
                                    {
                                        let rest = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceFinallyStatement(_, _) => {
                                    {
                                        let rest_2 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_2.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceTryStatement(_, _, _, _) => {
                                    {
                                        let rest_3 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_3.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceArrayBindingStatement(_, _, _, _) => {
                                    {
                                        let rest_4 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_4.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceObjectBindingStatement(_, _, _, _) => {
                                    {
                                        let rest_5 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_5.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceVariableStatement(_, _, _, _) => {
                                    {
                                        let rest_6 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_6.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceConstStatement(_, _) => {
                                    {
                                        let rest_7 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_7.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceMutableStatement(_, _, _) => {
                                    {
                                        let rest_8 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_8.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceExpressionStatement(_) => {
                                    {
                                        let rest_9 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_9.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceReturnStatement(statement_value) => {
                                    let statement_value = *statement_value;
                                    {
                                        let value = self::lower_java_script_error_expression(statement_value.clone(), context.clone(), (depth + 1f64));
                                        {
                                            let source_ml_source2_nominal_field16707 = match value.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16708, _) => {
                                                    let source_ml_source2_data_property_value16708 = *source_ml_source2_data_property_value16708;
                                                    source_ml_source2_data_property_value16708.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16709 = match value.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16710) => {
                                                        let source_ml_source2_data_property_value16710 = *source_ml_source2_data_property_value16710;
                                                        source_ml_source2_data_property_value16710.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override16711 = self::JavaScriptCallSource::SourceReturn(Box::new(match value.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16705, _) => {
                                                            let source_ml_source2_data_property_value16705 = *source_ml_source2_data_property_value16705;
                                                            source_ml_source2_data_property_value16705.clone()
                                                        }
                                                    }));
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override16711.clone()), Box::new(source_ml_source2_nominal_field16709.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceThrowStatement(statement_value_2) => {
                                    let statement_value_2 = *statement_value_2;
                                    {
                                        let value_2 = self::lower_java_script_error_expression(statement_value_2.clone(), context.clone(), (depth + 1f64));
                                        {
                                            let source_ml_source2_nominal_field16714 = match value_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16715, _) => {
                                                    let source_ml_source2_data_property_value16715 = *source_ml_source2_data_property_value16715;
                                                    source_ml_source2_data_property_value16715.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16716 = match value_2.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16717) => {
                                                        let source_ml_source2_data_property_value16717 = *source_ml_source2_data_property_value16717;
                                                        source_ml_source2_data_property_value16717.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override16718 = self::JavaScriptCallSource::SourceThrow(Box::new(match value_2.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16712, _) => {
                                                            let source_ml_source2_data_property_value16712 = *source_ml_source2_data_property_value16712;
                                                            source_ml_source2_data_property_value16712.clone()
                                                        }
                                                    }));
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override16718.clone()), Box::new(source_ml_source2_nominal_field16716.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceIfStatement(_, _, _) => {
                                    {
                                        let rest_10 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_10.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceBlockStatement(_) => {
                                    {
                                        let rest_11 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_11.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(_) => {
                                    {
                                        let rest_12 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_12.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceWhileStatement(_, _, _, _, _, _) => {
                                    {
                                        let rest_13 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_13.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceControlBlockStatement(_, _) => {
                                    {
                                        let rest_14 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_14.clone(), depth)
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceControlJumpStatement(target, continuing) => {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceControlJump(target, continuing)), Box::new(context.clone()))
                                }
                                self::JavaScriptSourceStatement::SourceSwitchStatement(_, _, _) => {
                                    {
                                        let rest_15 = self::lower_java_script_source_statements(statements.clone(), (first + 1f64), context.clone(), continuation.clone(), (depth + 1f64), true);
                                        self::lower_java_script_source_statement_continuation(statement.clone().expect("a proven present value"), rest_15.clone(), depth)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123Return(source_ml_source1_temporary_result123) => {
                let source_ml_source1_temporary_result123 = *source_ml_source1_temporary_result123;
                source_ml_source1_temporary_result123.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_statement_continuation(statement: self::JavaScriptSourceStatement, rest: self::JavaScriptErrorSourceExpression, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match statement.clone() {
        self::JavaScriptSourceStatement::SourceFinallyStatement(_, _) => {
            self::lower_java_script_source_exception_continuation(statement.clone(), rest.clone(), depth)
        }
        self::JavaScriptSourceStatement::SourceTryStatement(_, _, _, _) => {
            self::lower_java_script_source_exception_continuation(statement.clone(), rest.clone(), depth)
        }
        self::JavaScriptSourceStatement::SourceArrayBindingStatement(pattern, statement_value, mutable, _) => {
            let pattern = *pattern;
            let statement_value = *statement_value;
            {
                let value = self::lower_java_script_error_expression(statement_value.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16752) => {
                        let source_ml_source2_data_property_value16752 = *source_ml_source2_data_property_value16752;
                        source_ml_source2_data_property_value16752.clone()
                    }
                }, (depth + 1f64));
                {
                    let initialized = self::lower_java_script_source_binding_pattern(pattern.clone(), match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16753) => {
                            let source_ml_source2_data_property_value16753 = *source_ml_source2_data_property_value16753;
                            source_ml_source2_data_property_value16753.clone()
                        }
                    }, match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16754, _) => {
                            let source_ml_source2_data_property_value16754 = *source_ml_source2_data_property_value16754;
                            source_ml_source2_data_property_value16754.clone()
                        }
                    }, mutable, 0f64, false);
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(match initialized.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16755) => {
                            let source_ml_source2_data_property_value16755 = *source_ml_source2_data_property_value16755;
                            source_ml_source2_data_property_value16755.clone()
                        }
                    }, match pattern.clone() {
                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value16756, _, _) => {
                            source_ml_source2_data_property_value16756
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value16757, _) => {
                            source_ml_source2_data_property_value16757
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value16758, _, _) => {
                            source_ml_source2_data_property_value16758
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value16759, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16759
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value16760, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16760
                        }
                    }, match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16761, _) => {
                            let source_ml_source2_data_property_value16761 = *source_ml_source2_data_property_value16761;
                            source_ml_source2_data_property_value16761.clone()
                        }
                    }, match initialized.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16762, _) => {
                            let source_ml_source2_data_property_value16762 = *source_ml_source2_data_property_value16762;
                            source_ml_source2_data_property_value16762.clone()
                        }
                    }, false)), Box::new(match initialized.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16763) => {
                            let source_ml_source2_data_property_value16763 = *source_ml_source2_data_property_value16763;
                            source_ml_source2_data_property_value16763.clone()
                        }
                    }))
                }
            }
        }
        self::JavaScriptSourceStatement::SourceObjectBindingStatement(pattern_2, statement_value_2, mutable_2, _) => {
            let pattern_2 = *pattern_2;
            let statement_value_2 = *statement_value_2;
            {
                let value_2 = self::lower_java_script_error_expression(statement_value_2.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16764) => {
                        let source_ml_source2_data_property_value16764 = *source_ml_source2_data_property_value16764;
                        source_ml_source2_data_property_value16764.clone()
                    }
                }, (depth + 1f64));
                {
                    let initialized_2 = self::lower_java_script_source_binding_pattern(pattern_2.clone(), match value_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16765) => {
                            let source_ml_source2_data_property_value16765 = *source_ml_source2_data_property_value16765;
                            source_ml_source2_data_property_value16765.clone()
                        }
                    }, match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16766, _) => {
                            let source_ml_source2_data_property_value16766 = *source_ml_source2_data_property_value16766;
                            source_ml_source2_data_property_value16766.clone()
                        }
                    }, mutable_2, 0f64, false);
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(match initialized_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16767) => {
                            let source_ml_source2_data_property_value16767 = *source_ml_source2_data_property_value16767;
                            source_ml_source2_data_property_value16767.clone()
                        }
                    }, match pattern_2.clone() {
                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value16768, _, _) => {
                            source_ml_source2_data_property_value16768
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value16769, _) => {
                            source_ml_source2_data_property_value16769
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value16770, _, _) => {
                            source_ml_source2_data_property_value16770
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value16771, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16771
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value16772, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16772
                        }
                    }, match value_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16773, _) => {
                            let source_ml_source2_data_property_value16773 = *source_ml_source2_data_property_value16773;
                            source_ml_source2_data_property_value16773.clone()
                        }
                    }, match initialized_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16774, _) => {
                            let source_ml_source2_data_property_value16774 = *source_ml_source2_data_property_value16774;
                            source_ml_source2_data_property_value16774.clone()
                        }
                    }, false)), Box::new(match initialized_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16775) => {
                            let source_ml_source2_data_property_value16775 = *source_ml_source2_data_property_value16775;
                            source_ml_source2_data_property_value16775.clone()
                        }
                    }))
                }
            }
        }
        self::JavaScriptSourceStatement::SourceVariableStatement(identity, statement_value_3, has_initializer, statement_target) => {
            let statement_value_3 = *statement_value_3;
            if !has_initializer {
                rest.clone()
            } else {
                {
                    let value_3 = self::lower_java_script_error_expression(statement_value_3.clone(), match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16776) => {
                            let source_ml_source2_data_property_value16776 = *source_ml_source2_data_property_value16776;
                            source_ml_source2_data_property_value16776.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let target = ((statement_target.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone();
                        if !(match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_source_existing_write(target.clone().expect("a proven present value"), match value_3.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16777, _) => {
                                    let source_ml_source2_data_property_value16777 = *source_ml_source2_data_property_value16777;
                                    source_ml_source2_data_property_value16777.clone()
                                }
                            }, match rest.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16778, _) => {
                                    let source_ml_source2_data_property_value16778 = *source_ml_source2_data_property_value16778;
                                    source_ml_source2_data_property_value16778.clone()
                                }
                            })), Box::new(match value_3.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16779) => {
                                    let source_ml_source2_data_property_value16779 = *source_ml_source2_data_property_value16779;
                                    source_ml_source2_data_property_value16779.clone()
                                }
                            }))
                        } else {
                            {
                                let write = self::lower_java_script_lexical_cell_write(match value_3.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16780) => {
                                        let source_ml_source2_data_property_value16780 = *source_ml_source2_data_property_value16780;
                                        source_ml_source2_data_property_value16780.clone()
                                    }
                                }, identity, match value_3.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16781, _) => {
                                        let source_ml_source2_data_property_value16781 = *source_ml_source2_data_property_value16781;
                                        source_ml_source2_data_property_value16781.clone()
                                    }
                                }, false);
                                {
                                    let temporary = match match write.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16782) => {
                                            let source_ml_source2_data_property_value16782 = *source_ml_source2_data_property_value16782;
                                            source_ml_source2_data_property_value16782.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16783, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value16783
                                        }
                                    };
                                    {
                                        let next = {
                                            let source_ml_source2_nominal_spread16785 = match write.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16784) => {
                                                    let source_ml_source2_data_property_value16784 = *source_ml_source2_data_property_value16784;
                                                    source_ml_source2_data_property_value16784.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16786 = match source_ml_source2_nominal_spread16785.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16787, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value16787 = *source_ml_source2_data_property_value16787;
                                                        source_ml_source2_data_property_value16787.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16788 = match source_ml_source2_nominal_spread16785.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16789, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value16789 = *source_ml_source2_data_property_value16789;
                                                            source_ml_source2_data_property_value16789.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16790 = match source_ml_source2_nominal_spread16785.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16791, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value16791.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16792 = match source_ml_source2_nominal_spread16785.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16793, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value16793
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16794 = match source_ml_source2_nominal_spread16785.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16795, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value16795
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16796 = match source_ml_source2_nominal_spread16785.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16797, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value16797.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field16798 = match source_ml_source2_nominal_spread16785.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16799, _, _, _) => {
                                                                                source_ml_source2_data_property_value16799
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field16800 = match source_ml_source2_nominal_spread16785.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16801, _, _) => {
                                                                                    source_ml_source2_data_property_value16801.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field16802 = match source_ml_source2_nominal_spread16785.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16803, _) => {
                                                                                        source_ml_source2_data_property_value16803.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field16804 = match source_ml_source2_nominal_spread16785.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16805) => {
                                                                                            source_ml_source2_data_property_value16805.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override16806 = (temporary + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16786.clone()), Box::new(source_ml_source2_nominal_field16788.clone()), source_ml_source2_nominal_field16790.clone(), source_ml_source2_nominal_override16806, source_ml_source2_nominal_field16794, source_ml_source2_nominal_field16796.clone(), source_ml_source2_nominal_field16798, source_ml_source2_nominal_field16800.clone(), source_ml_source2_nominal_field16802.clone(), source_ml_source2_nominal_field16804.clone())
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
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), temporary, match write.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16807, _) => {
                                                let source_ml_source2_data_property_value16807 = *source_ml_source2_data_property_value16807;
                                                source_ml_source2_data_property_value16807.clone()
                                            }
                                        }, match rest.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16808, _) => {
                                                let source_ml_source2_data_property_value16808 = *source_ml_source2_data_property_value16808;
                                                source_ml_source2_data_property_value16808.clone()
                                            }
                                        }, false)), Box::new(next.clone()))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSourceStatement::SourceConstStatement(identity_2, value_4) => {
            let value_4 = *value_4;
            self::lower_java_script_source_local_declaration(identity_2, value_4.clone(), false, false, rest.clone(), (depth + 1f64))
        }
        self::JavaScriptSourceStatement::SourceMutableStatement(identity_3, value_5, shared) => {
            let value_5 = *value_5;
            self::lower_java_script_source_local_declaration(identity_3, value_5.clone(), true, {
                let source_ml_source2_comparison_operand16809 = ((shared.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                match (source_ml_source2_comparison_operand16809.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }, rest.clone(), (depth + 1f64))
        }
        self::JavaScriptSourceStatement::SourceExpressionStatement(statement_value_4) => {
            let statement_value_4 = *statement_value_4;
            {
                let value_6 = self::lower_java_script_error_expression(statement_value_4.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16811) => {
                        let source_ml_source2_data_property_value16811 = *source_ml_source2_data_property_value16811;
                        source_ml_source2_data_property_value16811.clone()
                    }
                }, (depth + 1f64));
                {
                    let identity_4 = match match value_6.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16812) => {
                            let source_ml_source2_data_property_value16812 = *source_ml_source2_data_property_value16812;
                            source_ml_source2_data_property_value16812.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16813, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16813
                        }
                    };
                    {
                        let next_2 = {
                            let source_ml_source2_nominal_spread16815 = match value_6.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16814) => {
                                    let source_ml_source2_data_property_value16814 = *source_ml_source2_data_property_value16814;
                                    source_ml_source2_data_property_value16814.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field16816 = match source_ml_source2_nominal_spread16815.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16817, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value16817 = *source_ml_source2_data_property_value16817;
                                        source_ml_source2_data_property_value16817.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field16818 = match source_ml_source2_nominal_spread16815.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16819, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value16819 = *source_ml_source2_data_property_value16819;
                                            source_ml_source2_data_property_value16819.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field16820 = match source_ml_source2_nominal_spread16815.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16821, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value16821.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field16822 = match source_ml_source2_nominal_spread16815.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16823, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value16823
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16824 = match source_ml_source2_nominal_spread16815.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16825, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value16825
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16826 = match source_ml_source2_nominal_spread16815.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16827, _, _, _, _) => {
                                                            source_ml_source2_data_property_value16827.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16828 = match source_ml_source2_nominal_spread16815.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16829, _, _, _) => {
                                                                source_ml_source2_data_property_value16829
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16830 = match source_ml_source2_nominal_spread16815.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16831, _, _) => {
                                                                    source_ml_source2_data_property_value16831.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16832 = match source_ml_source2_nominal_spread16815.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16833, _) => {
                                                                        source_ml_source2_data_property_value16833.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16834 = match source_ml_source2_nominal_spread16815.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16835) => {
                                                                            source_ml_source2_data_property_value16835.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override16836 = (identity_4 + 1f64);
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16816.clone()), Box::new(source_ml_source2_nominal_field16818.clone()), source_ml_source2_nominal_field16820.clone(), source_ml_source2_nominal_override16836, source_ml_source2_nominal_field16824, source_ml_source2_nominal_field16826.clone(), source_ml_source2_nominal_field16828, source_ml_source2_nominal_field16830.clone(), source_ml_source2_nominal_field16832.clone(), source_ml_source2_nominal_field16834.clone())
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
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next_2.clone(), identity_4, match value_6.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16837, _) => {
                                let source_ml_source2_data_property_value16837 = *source_ml_source2_data_property_value16837;
                                source_ml_source2_data_property_value16837.clone()
                            }
                        }, match rest.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16838, _) => {
                                let source_ml_source2_data_property_value16838 = *source_ml_source2_data_property_value16838;
                                source_ml_source2_data_property_value16838.clone()
                            }
                        }, false)), Box::new(next_2.clone()))
                    }
                }
            }
        }
        self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16839) => {
                    let source_ml_source2_data_property_value16839 = *source_ml_source2_data_property_value16839;
                    source_ml_source2_data_property_value16839.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16840) => {
                    let source_ml_source2_data_property_value16840 = *source_ml_source2_data_property_value16840;
                    source_ml_source2_data_property_value16840.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceIfStatement(_, _, _) => {
            self::lower_java_script_source_selection_continuation(statement.clone(), rest.clone(), depth)
        }
        self::JavaScriptSourceStatement::SourceBlockStatement(statements) => {
            self::lower_java_script_source_statements(statements.clone(), 0f64, match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16841) => {
                    let source_ml_source2_data_property_value16841 = *source_ml_source2_data_property_value16841;
                    source_ml_source2_data_property_value16841.clone()
                }
            }, match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16842, _) => {
                    let source_ml_source2_data_property_value16842 = *source_ml_source2_data_property_value16842;
                    source_ml_source2_data_property_value16842.clone()
                }
            }, (depth + 1f64), true)
        }
        self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(statements_2) => {
            self::lower_java_script_source_statements(statements_2.clone(), 0f64, match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16843) => {
                    let source_ml_source2_data_property_value16843 = *source_ml_source2_data_property_value16843;
                    source_ml_source2_data_property_value16843.clone()
                }
            }, match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16844, _) => {
                    let source_ml_source2_data_property_value16844 = *source_ml_source2_data_property_value16844;
                    source_ml_source2_data_property_value16844.clone()
                }
            }, (depth + 1f64), false)
        }
        self::JavaScriptSourceStatement::SourceControlBlockStatement(target_2, statement_body) => {
            {
                let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16845) => {
                        let source_ml_source2_data_property_value16845 = *source_ml_source2_data_property_value16845;
                        source_ml_source2_data_property_value16845.clone()
                    }
                }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceControlBlock(target_2, Box::new(match body.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16846, _) => {
                        let source_ml_source2_data_property_value16846 = *source_ml_source2_data_property_value16846;
                        source_ml_source2_data_property_value16846.clone()
                    }
                }), Box::new(match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16847, _) => {
                        let source_ml_source2_data_property_value16847 = *source_ml_source2_data_property_value16847;
                        source_ml_source2_data_property_value16847.clone()
                    }
                }))), Box::new(match body.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16848) => {
                        let source_ml_source2_data_property_value16848 = *source_ml_source2_data_property_value16848;
                        source_ml_source2_data_property_value16848.clone()
                    }
                }))
            }
        }
        self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16849) => {
                    let source_ml_source2_data_property_value16849 = *source_ml_source2_data_property_value16849;
                    source_ml_source2_data_property_value16849.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceSwitchStatement(_, _, _) => {
            self::lower_java_script_source_selection_continuation(statement.clone(), rest.clone(), depth)
        }
        _ => {
            self::lower_java_script_source_iteration_continuation(statement.clone(), rest.clone(), depth)
        }
    }
}
