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
        let source_ml_source2_s16730 = if ((first == (0f64)) && instantiate) {
            {
                let hoisted = if (depth == (0f64)) {
                    self::collect_java_script_source_variable_declarations(statements.clone(), context.clone(), 0f64)
                } else {
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                };
                {
                    let declarations = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value16732 = self::collect_java_script_source_lexical_declarations(statements.clone(), context.clone(), 0f64);
                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value259 = hoisted.borrow().clone(); ml_reference_value259 }), { let ml_reference_value260 = source_ml_source2_concat_value16732.borrow().clone(); ml_reference_value260 })
                    }));
                    if (({ let ml_reference_array261 = declarations.clone(); let ml_reference_projection261 = ml_reference_array261.borrow().len() as f64; ml_reference_projection261 }) > (0f64)) {
                        {
                            let body = self::lower_java_script_source_statements(statements.clone(), first, context.clone(), continuation.clone(), depth, false);
                            {
                                let initializers = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallCellInitializer>::new()));
                                {
                                    let source_ml_source1_temporary_index120 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s16733 = self::source_ml_source1_temporary_lower_java_script_source_statements_loop124(body.clone(), initializers.clone(), hoisted.clone(), source_ml_source1_temporary_index120.clone());
                                        match source_ml_source2_s16733.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Result::MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Done => {
                                                self::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123::MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123Return(Box::new({
                                                    let source_ml_source2_nominal_field16747 = match body.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16748, _) => {
                                                            let source_ml_source2_data_property_value16748 = *source_ml_source2_data_property_value16748;
                                                            source_ml_source2_data_property_value16748.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16749 = match body.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16750) => {
                                                                let source_ml_source2_data_property_value16750 = *source_ml_source2_data_property_value16750;
                                                                source_ml_source2_data_property_value16750.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override16751 = {
                                                                let source_ml_source1_temporary_constructor_field122 = match match match body.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16734) => {
                                                                        let source_ml_source2_data_property_value16734 = *source_ml_source2_data_property_value16734;
                                                                        source_ml_source2_data_property_value16734.clone()
                                                                    }
                                                                } {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16735, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value16735 = *source_ml_source2_data_property_value16735;
                                                                        source_ml_source2_data_property_value16735.clone()
                                                                    }
                                                                } {
                                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value16736, _, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value16736.clone()
                                                                    }
                                                                };
                                                                if { let ml_value = { let ml_reference_array263 = hoisted.clone(); let ml_reference_projection263 = ml_reference_array263.borrow().len() as f64; ml_reference_projection263 }; ml_value != 0f64 && !ml_value.is_nan() } {
                                                                    {
                                                                        let source_ml_source2_constructor_field16739 = Some(initializers.clone());
                                                                        {
                                                                            let source_ml_source2_constructor_field16741 = match body.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16740, _) => {
                                                                                    let source_ml_source2_data_property_value16740 = *source_ml_source2_data_property_value16740;
                                                                                    source_ml_source2_data_property_value16740.clone()
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSource::SourceLexicalScope(declarations.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Some((source_ml_source2_constructor_field16739.clone(),)), source_ml_source1_temporary_constructor_field122.clone(), Box::new(source_ml_source2_constructor_field16741.clone()))
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_constructor_field16745 = match body.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16744, _) => {
                                                                                let source_ml_source2_data_property_value16744 = *source_ml_source2_data_property_value16744;
                                                                                source_ml_source2_data_property_value16744.clone()
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallSource::SourceLexicalScope(declarations.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source1_temporary_constructor_field122.clone(), Box::new(source_ml_source2_constructor_field16745.clone()))
                                                                    }
                                                                }
                                                            };
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override16751.clone()), Box::new(source_ml_source2_nominal_field16749.clone()))
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
        match source_ml_source2_s16730.clone() {
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
                                            let source_ml_source2_nominal_field16805 = match value.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16806, _) => {
                                                    let source_ml_source2_data_property_value16806 = *source_ml_source2_data_property_value16806;
                                                    source_ml_source2_data_property_value16806.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16807 = match value.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16808) => {
                                                        let source_ml_source2_data_property_value16808 = *source_ml_source2_data_property_value16808;
                                                        source_ml_source2_data_property_value16808.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override16809 = self::JavaScriptCallSource::SourceReturn(Box::new(match value.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16803, _) => {
                                                            let source_ml_source2_data_property_value16803 = *source_ml_source2_data_property_value16803;
                                                            source_ml_source2_data_property_value16803.clone()
                                                        }
                                                    }));
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override16809.clone()), Box::new(source_ml_source2_nominal_field16807.clone()))
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
                                            let source_ml_source2_nominal_field16812 = match value_2.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16813, _) => {
                                                    let source_ml_source2_data_property_value16813 = *source_ml_source2_data_property_value16813;
                                                    source_ml_source2_data_property_value16813.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16814 = match value_2.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16815) => {
                                                        let source_ml_source2_data_property_value16815 = *source_ml_source2_data_property_value16815;
                                                        source_ml_source2_data_property_value16815.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_override16816 = self::JavaScriptCallSource::SourceThrow(Box::new(match value_2.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16810, _) => {
                                                            let source_ml_source2_data_property_value16810 = *source_ml_source2_data_property_value16810;
                                                            source_ml_source2_data_property_value16810.clone()
                                                        }
                                                    }));
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override16816.clone()), Box::new(source_ml_source2_nominal_field16814.clone()))
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
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16850) => {
                        let source_ml_source2_data_property_value16850 = *source_ml_source2_data_property_value16850;
                        source_ml_source2_data_property_value16850.clone()
                    }
                }, (depth + 1f64));
                {
                    let initialized = self::lower_java_script_source_binding_pattern(pattern.clone(), match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16851) => {
                            let source_ml_source2_data_property_value16851 = *source_ml_source2_data_property_value16851;
                            source_ml_source2_data_property_value16851.clone()
                        }
                    }, match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16852, _) => {
                            let source_ml_source2_data_property_value16852 = *source_ml_source2_data_property_value16852;
                            source_ml_source2_data_property_value16852.clone()
                        }
                    }, mutable, 0f64, false);
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(match initialized.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16853) => {
                            let source_ml_source2_data_property_value16853 = *source_ml_source2_data_property_value16853;
                            source_ml_source2_data_property_value16853.clone()
                        }
                    }, match pattern.clone() {
                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value16854, _, _) => {
                            source_ml_source2_data_property_value16854
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value16855, _) => {
                            source_ml_source2_data_property_value16855
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value16856, _, _) => {
                            source_ml_source2_data_property_value16856
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value16857, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16857
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value16858, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16858
                        }
                    }, match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16859, _) => {
                            let source_ml_source2_data_property_value16859 = *source_ml_source2_data_property_value16859;
                            source_ml_source2_data_property_value16859.clone()
                        }
                    }, match initialized.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16860, _) => {
                            let source_ml_source2_data_property_value16860 = *source_ml_source2_data_property_value16860;
                            source_ml_source2_data_property_value16860.clone()
                        }
                    }, false)), Box::new(match initialized.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16861) => {
                            let source_ml_source2_data_property_value16861 = *source_ml_source2_data_property_value16861;
                            source_ml_source2_data_property_value16861.clone()
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
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16862) => {
                        let source_ml_source2_data_property_value16862 = *source_ml_source2_data_property_value16862;
                        source_ml_source2_data_property_value16862.clone()
                    }
                }, (depth + 1f64));
                {
                    let initialized_2 = self::lower_java_script_source_binding_pattern(pattern_2.clone(), match value_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16863) => {
                            let source_ml_source2_data_property_value16863 = *source_ml_source2_data_property_value16863;
                            source_ml_source2_data_property_value16863.clone()
                        }
                    }, match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16864, _) => {
                            let source_ml_source2_data_property_value16864 = *source_ml_source2_data_property_value16864;
                            source_ml_source2_data_property_value16864.clone()
                        }
                    }, mutable_2, 0f64, false);
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(match initialized_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16865) => {
                            let source_ml_source2_data_property_value16865 = *source_ml_source2_data_property_value16865;
                            source_ml_source2_data_property_value16865.clone()
                        }
                    }, match pattern_2.clone() {
                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value16866, _, _) => {
                            source_ml_source2_data_property_value16866
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value16867, _) => {
                            source_ml_source2_data_property_value16867
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value16868, _, _) => {
                            source_ml_source2_data_property_value16868
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value16869, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16869
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value16870, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16870
                        }
                    }, match value_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16871, _) => {
                            let source_ml_source2_data_property_value16871 = *source_ml_source2_data_property_value16871;
                            source_ml_source2_data_property_value16871.clone()
                        }
                    }, match initialized_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16872, _) => {
                            let source_ml_source2_data_property_value16872 = *source_ml_source2_data_property_value16872;
                            source_ml_source2_data_property_value16872.clone()
                        }
                    }, false)), Box::new(match initialized_2.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16873) => {
                            let source_ml_source2_data_property_value16873 = *source_ml_source2_data_property_value16873;
                            source_ml_source2_data_property_value16873.clone()
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
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16874) => {
                            let source_ml_source2_data_property_value16874 = *source_ml_source2_data_property_value16874;
                            source_ml_source2_data_property_value16874.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let target = ((statement_target.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone();
                        if !(match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_source_existing_write(target.clone().expect("a proven present value"), match value_3.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16875, _) => {
                                    let source_ml_source2_data_property_value16875 = *source_ml_source2_data_property_value16875;
                                    source_ml_source2_data_property_value16875.clone()
                                }
                            }, match rest.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16876, _) => {
                                    let source_ml_source2_data_property_value16876 = *source_ml_source2_data_property_value16876;
                                    source_ml_source2_data_property_value16876.clone()
                                }
                            })), Box::new(match value_3.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16877) => {
                                    let source_ml_source2_data_property_value16877 = *source_ml_source2_data_property_value16877;
                                    source_ml_source2_data_property_value16877.clone()
                                }
                            }))
                        } else {
                            {
                                let write = self::lower_java_script_lexical_cell_write(match value_3.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16878) => {
                                        let source_ml_source2_data_property_value16878 = *source_ml_source2_data_property_value16878;
                                        source_ml_source2_data_property_value16878.clone()
                                    }
                                }, identity, match value_3.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16879, _) => {
                                        let source_ml_source2_data_property_value16879 = *source_ml_source2_data_property_value16879;
                                        source_ml_source2_data_property_value16879.clone()
                                    }
                                }, false);
                                {
                                    let temporary = match match write.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16880) => {
                                            let source_ml_source2_data_property_value16880 = *source_ml_source2_data_property_value16880;
                                            source_ml_source2_data_property_value16880.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16881, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value16881
                                        }
                                    };
                                    {
                                        let next = {
                                            let source_ml_source2_nominal_spread16883 = match write.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16882) => {
                                                    let source_ml_source2_data_property_value16882 = *source_ml_source2_data_property_value16882;
                                                    source_ml_source2_data_property_value16882.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16884 = match source_ml_source2_nominal_spread16883.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16885, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value16885 = *source_ml_source2_data_property_value16885;
                                                        source_ml_source2_data_property_value16885.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16886 = match source_ml_source2_nominal_spread16883.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16887, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value16887 = *source_ml_source2_data_property_value16887;
                                                            source_ml_source2_data_property_value16887.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16888 = match source_ml_source2_nominal_spread16883.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16889, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value16889.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16890 = match source_ml_source2_nominal_spread16883.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16891, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value16891
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16892 = match source_ml_source2_nominal_spread16883.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16893, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value16893
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16894 = match source_ml_source2_nominal_spread16883.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16895, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value16895.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field16896 = match source_ml_source2_nominal_spread16883.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16897, _, _, _) => {
                                                                                source_ml_source2_data_property_value16897
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field16898 = match source_ml_source2_nominal_spread16883.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16899, _, _) => {
                                                                                    source_ml_source2_data_property_value16899.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field16900 = match source_ml_source2_nominal_spread16883.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16901, _) => {
                                                                                        source_ml_source2_data_property_value16901.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field16902 = match source_ml_source2_nominal_spread16883.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16903) => {
                                                                                            source_ml_source2_data_property_value16903.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override16904 = (temporary + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16884.clone()), Box::new(source_ml_source2_nominal_field16886.clone()), source_ml_source2_nominal_field16888.clone(), source_ml_source2_nominal_override16904, source_ml_source2_nominal_field16892, source_ml_source2_nominal_field16894.clone(), source_ml_source2_nominal_field16896, source_ml_source2_nominal_field16898.clone(), source_ml_source2_nominal_field16900.clone(), source_ml_source2_nominal_field16902.clone())
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16905, _) => {
                                                let source_ml_source2_data_property_value16905 = *source_ml_source2_data_property_value16905;
                                                source_ml_source2_data_property_value16905.clone()
                                            }
                                        }, match rest.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16906, _) => {
                                                let source_ml_source2_data_property_value16906 = *source_ml_source2_data_property_value16906;
                                                source_ml_source2_data_property_value16906.clone()
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
                let source_ml_source2_comparison_operand16907 = ((shared.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                match (source_ml_source2_comparison_operand16907.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }, rest.clone(), (depth + 1f64))
        }
        self::JavaScriptSourceStatement::SourceExpressionStatement(statement_value_4) => {
            let statement_value_4 = *statement_value_4;
            {
                let value_6 = self::lower_java_script_error_expression(statement_value_4.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16909) => {
                        let source_ml_source2_data_property_value16909 = *source_ml_source2_data_property_value16909;
                        source_ml_source2_data_property_value16909.clone()
                    }
                }, (depth + 1f64));
                {
                    let identity_4 = match match value_6.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16910) => {
                            let source_ml_source2_data_property_value16910 = *source_ml_source2_data_property_value16910;
                            source_ml_source2_data_property_value16910.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16911, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value16911
                        }
                    };
                    {
                        let next_2 = {
                            let source_ml_source2_nominal_spread16913 = match value_6.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16912) => {
                                    let source_ml_source2_data_property_value16912 = *source_ml_source2_data_property_value16912;
                                    source_ml_source2_data_property_value16912.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field16914 = match source_ml_source2_nominal_spread16913.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value16915, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value16915 = *source_ml_source2_data_property_value16915;
                                        source_ml_source2_data_property_value16915.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field16916 = match source_ml_source2_nominal_spread16913.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value16917, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value16917 = *source_ml_source2_data_property_value16917;
                                            source_ml_source2_data_property_value16917.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field16918 = match source_ml_source2_nominal_spread16913.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value16919, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value16919.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field16920 = match source_ml_source2_nominal_spread16913.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value16921, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value16921
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field16922 = match source_ml_source2_nominal_spread16913.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value16923, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value16923
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field16924 = match source_ml_source2_nominal_spread16913.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value16925, _, _, _, _) => {
                                                            source_ml_source2_data_property_value16925.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field16926 = match source_ml_source2_nominal_spread16913.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value16927, _, _, _) => {
                                                                source_ml_source2_data_property_value16927
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field16928 = match source_ml_source2_nominal_spread16913.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value16929, _, _) => {
                                                                    source_ml_source2_data_property_value16929.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field16930 = match source_ml_source2_nominal_spread16913.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value16931, _) => {
                                                                        source_ml_source2_data_property_value16931.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field16932 = match source_ml_source2_nominal_spread16913.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16933) => {
                                                                            source_ml_source2_data_property_value16933.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override16934 = (identity_4 + 1f64);
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field16914.clone()), Box::new(source_ml_source2_nominal_field16916.clone()), source_ml_source2_nominal_field16918.clone(), source_ml_source2_nominal_override16934, source_ml_source2_nominal_field16922, source_ml_source2_nominal_field16924.clone(), source_ml_source2_nominal_field16926, source_ml_source2_nominal_field16928.clone(), source_ml_source2_nominal_field16930.clone(), source_ml_source2_nominal_field16932.clone())
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
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16935, _) => {
                                let source_ml_source2_data_property_value16935 = *source_ml_source2_data_property_value16935;
                                source_ml_source2_data_property_value16935.clone()
                            }
                        }, match rest.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16936, _) => {
                                let source_ml_source2_data_property_value16936 = *source_ml_source2_data_property_value16936;
                                source_ml_source2_data_property_value16936.clone()
                            }
                        }, false)), Box::new(next_2.clone()))
                    }
                }
            }
        }
        self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16937) => {
                    let source_ml_source2_data_property_value16937 = *source_ml_source2_data_property_value16937;
                    source_ml_source2_data_property_value16937.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16938) => {
                    let source_ml_source2_data_property_value16938 = *source_ml_source2_data_property_value16938;
                    source_ml_source2_data_property_value16938.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceIfStatement(_, _, _) => {
            self::lower_java_script_source_selection_continuation(statement.clone(), rest.clone(), depth)
        }
        self::JavaScriptSourceStatement::SourceBlockStatement(statements) => {
            self::lower_java_script_source_statements(statements.clone(), 0f64, match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16939) => {
                    let source_ml_source2_data_property_value16939 = *source_ml_source2_data_property_value16939;
                    source_ml_source2_data_property_value16939.clone()
                }
            }, match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16940, _) => {
                    let source_ml_source2_data_property_value16940 = *source_ml_source2_data_property_value16940;
                    source_ml_source2_data_property_value16940.clone()
                }
            }, (depth + 1f64), true)
        }
        self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(statements_2) => {
            self::lower_java_script_source_statements(statements_2.clone(), 0f64, match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16941) => {
                    let source_ml_source2_data_property_value16941 = *source_ml_source2_data_property_value16941;
                    source_ml_source2_data_property_value16941.clone()
                }
            }, match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16942, _) => {
                    let source_ml_source2_data_property_value16942 = *source_ml_source2_data_property_value16942;
                    source_ml_source2_data_property_value16942.clone()
                }
            }, (depth + 1f64), false)
        }
        self::JavaScriptSourceStatement::SourceControlBlockStatement(target_2, statement_body) => {
            {
                let body = self::lower_java_script_source_statements(statement_body.clone(), 0f64, match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16943) => {
                        let source_ml_source2_data_property_value16943 = *source_ml_source2_data_property_value16943;
                        source_ml_source2_data_property_value16943.clone()
                    }
                }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), true);
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceControlBlock(target_2, Box::new(match body.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16944, _) => {
                        let source_ml_source2_data_property_value16944 = *source_ml_source2_data_property_value16944;
                        source_ml_source2_data_property_value16944.clone()
                    }
                }), Box::new(match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value16945, _) => {
                        let source_ml_source2_data_property_value16945 = *source_ml_source2_data_property_value16945;
                        source_ml_source2_data_property_value16945.clone()
                    }
                }))), Box::new(match body.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16946) => {
                        let source_ml_source2_data_property_value16946 = *source_ml_source2_data_property_value16946;
                        source_ml_source2_data_property_value16946.clone()
                    }
                }))
            }
        }
        self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value16947) => {
                    let source_ml_source2_data_property_value16947 = *source_ml_source2_data_property_value16947;
                    source_ml_source2_data_property_value16947.clone()
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
