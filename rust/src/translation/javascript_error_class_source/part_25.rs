// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallCellInitializer;
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceSwitchClause;
use super::JavaScriptCallSourceUnit;
use super::JavaScriptCallTemplate;
use super::JavaScriptErrorArrowTemplatePlan;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorModuleBinding;
use super::JavaScriptErrorModuleCompilation;
use super::JavaScriptErrorModulePlan;
use super::JavaScriptErrorModuleRegistry;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceBindingTarget;
use super::JavaScriptSourceCapabilityOwner;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceStatement;
use super::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result;
use super::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result;
use super::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result;
use super::MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Result;
use super::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result;
use super::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result;
use super::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Result;
use super::compile_java_script_error_module_templates;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_field_key;
use super::construct_java_script_source_pattern_binding;
use super::lower_java_script_error_expression;
use super::lower_java_script_source_statements;
use super::ml;
use super::ml_string;
use super::order_java_script_call_code_functions;
use super::read_java_script_error_module_plan_error;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop171;
use super::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop172;
use super::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop173;
use super::source_ml_source1_temporary_lower_java_script_source_object_binding_loop149;
use super::source_ml_source1_temporary_lower_java_script_source_object_binding_loop152;
use super::source_ml_source1_temporary_lower_java_script_source_parameters_loop157;
use super::source_ml_source1_temporary_lower_java_script_source_parameters_loop158;
use super::source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop142;
use super::source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop145;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_selection_continuation(statement: self::JavaScriptSourceStatement, rest: self::JavaScriptErrorSourceExpression, depth: f64) -> self::JavaScriptErrorSourceExpression {
    match statement.clone() {
        self::JavaScriptSourceStatement::SourceForOfStatement(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17412) => {
                    let source_ml_source2_data_property_value17412 = *source_ml_source2_data_property_value17412;
                    source_ml_source2_data_property_value17412.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceFinallyStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17413) => {
                    let source_ml_source2_data_property_value17413 = *source_ml_source2_data_property_value17413;
                    source_ml_source2_data_property_value17413.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceTryStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17414) => {
                    let source_ml_source2_data_property_value17414 = *source_ml_source2_data_property_value17414;
                    source_ml_source2_data_property_value17414.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceArrayBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17415) => {
                    let source_ml_source2_data_property_value17415 = *source_ml_source2_data_property_value17415;
                    source_ml_source2_data_property_value17415.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceObjectBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17416) => {
                    let source_ml_source2_data_property_value17416 = *source_ml_source2_data_property_value17416;
                    source_ml_source2_data_property_value17416.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceVariableStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17417) => {
                    let source_ml_source2_data_property_value17417 = *source_ml_source2_data_property_value17417;
                    source_ml_source2_data_property_value17417.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceConstStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17418) => {
                    let source_ml_source2_data_property_value17418 = *source_ml_source2_data_property_value17418;
                    source_ml_source2_data_property_value17418.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceMutableStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17419) => {
                    let source_ml_source2_data_property_value17419 = *source_ml_source2_data_property_value17419;
                    source_ml_source2_data_property_value17419.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceExpressionStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17420) => {
                    let source_ml_source2_data_property_value17420 = *source_ml_source2_data_property_value17420;
                    source_ml_source2_data_property_value17420.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17421) => {
                    let source_ml_source2_data_property_value17421 = *source_ml_source2_data_property_value17421;
                    source_ml_source2_data_property_value17421.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17422) => {
                    let source_ml_source2_data_property_value17422 = *source_ml_source2_data_property_value17422;
                    source_ml_source2_data_property_value17422.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceIfStatement(statement_condition, statement_yes, statement_no) => {
            let statement_condition = *statement_condition;
            {
                let condition = self::lower_java_script_error_expression(statement_condition.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17423) => {
                        let source_ml_source2_data_property_value17423 = *source_ml_source2_data_property_value17423;
                        source_ml_source2_data_property_value17423.clone()
                    }
                }, (depth + 1f64));
                {
                    let yes = self::lower_java_script_source_statements(statement_yes.clone(), 0f64, match condition.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17424) => {
                            let source_ml_source2_data_property_value17424 = *source_ml_source2_data_property_value17424;
                            source_ml_source2_data_property_value17424.clone()
                        }
                    }, match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17425, _) => {
                            let source_ml_source2_data_property_value17425 = *source_ml_source2_data_property_value17425;
                            source_ml_source2_data_property_value17425.clone()
                        }
                    }, (depth + 1f64), true);
                    {
                        let no = self::lower_java_script_source_statements(statement_no.clone(), 0f64, match yes.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17426) => {
                                let source_ml_source2_data_property_value17426 = *source_ml_source2_data_property_value17426;
                                source_ml_source2_data_property_value17426.clone()
                            }
                        }, match rest.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17427, _) => {
                                let source_ml_source2_data_property_value17427 = *source_ml_source2_data_property_value17427;
                                source_ml_source2_data_property_value17427.clone()
                            }
                        }, (depth + 1f64), true);
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceIf(Box::new(match condition.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17428, _) => {
                                let source_ml_source2_data_property_value17428 = *source_ml_source2_data_property_value17428;
                                source_ml_source2_data_property_value17428.clone()
                            }
                        }), Box::new(match yes.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17429, _) => {
                                let source_ml_source2_data_property_value17429 = *source_ml_source2_data_property_value17429;
                                source_ml_source2_data_property_value17429.clone()
                            }
                        }), Box::new(match no.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17430, _) => {
                                let source_ml_source2_data_property_value17430 = *source_ml_source2_data_property_value17430;
                                source_ml_source2_data_property_value17430.clone()
                            }
                        }))), Box::new(match no.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17431) => {
                                let source_ml_source2_data_property_value17431 = *source_ml_source2_data_property_value17431;
                                source_ml_source2_data_property_value17431.clone()
                            }
                        }))
                    }
                }
            }
        }
        self::JavaScriptSourceStatement::SourceBlockStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17432) => {
                    let source_ml_source2_data_property_value17432 = *source_ml_source2_data_property_value17432;
                    source_ml_source2_data_property_value17432.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17433) => {
                    let source_ml_source2_data_property_value17433 = *source_ml_source2_data_property_value17433;
                    source_ml_source2_data_property_value17433.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceWhileStatement(_, _, _, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17434) => {
                    let source_ml_source2_data_property_value17434 = *source_ml_source2_data_property_value17434;
                    source_ml_source2_data_property_value17434.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlBlockStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17435) => {
                    let source_ml_source2_data_property_value17435 = *source_ml_source2_data_property_value17435;
                    source_ml_source2_data_property_value17435.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17436) => {
                    let source_ml_source2_data_property_value17436 = *source_ml_source2_data_property_value17436;
                    source_ml_source2_data_property_value17436.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceSwitchStatement(target, statement_value, statement_clauses) => {
            let statement_value = *statement_value;
            {
                let value = self::lower_java_script_error_expression(statement_value.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17437) => {
                        let source_ml_source2_data_property_value17437 = *source_ml_source2_data_property_value17437;
                        source_ml_source2_data_property_value17437.clone()
                    }
                }, (depth + 1f64));
                {
                    let current = match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17438) => {
                            let source_ml_source2_data_property_value17438 = *source_ml_source2_data_property_value17438;
                            source_ml_source2_data_property_value17438.clone()
                        }
                    };
                    {
                        let clauses = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceSwitchClause>::new()));
                        {
                            let source_ml_source1_temporary_index139 = self::ml::Big::from_i128(0);
                            {
                                let current_2 = self::source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop142(depth, current.clone(), clauses.clone(), statement_clauses.clone(), source_ml_source1_temporary_index139.clone());
                                {
                                    let declarations = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                    {
                                        let source_ml_source1_temporary_index140 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s17439 = self::source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop145(current_2.clone(), declarations.clone(), statement_clauses.clone(), source_ml_source1_temporary_index140.clone());
                                            match source_ml_source2_s17439.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Result::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Done => {
                                                    if (({ let ml_reference_array298 = declarations.clone(); let ml_reference_projection298 = ml_reference_array298.borrow().len() as f64; ml_reference_projection298 }) > (0f64)) {
                                                        {
                                                            let held_id = match current_2.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17440, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17440
                                                                }
                                                            };
                                                            {
                                                                let current_3 = {
                                                                    let source_ml_source2_nominal_field17442 = match current_2.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17443, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value17443 = *source_ml_source2_data_property_value17443;
                                                                            source_ml_source2_data_property_value17443.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17444 = match current_2.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17445, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17445 = *source_ml_source2_data_property_value17445;
                                                                                source_ml_source2_data_property_value17445.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17446 = match current_2.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17447, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17447.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17448 = match current_2.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17449, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value17449
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17450 = match current_2.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17451, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value17451
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17452 = match current_2.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17453, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value17453.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field17454 = match current_2.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17455, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value17455
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field17456 = match current_2.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17457, _, _) => {
                                                                                                        source_ml_source2_data_property_value17457.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field17458 = match current_2.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17459, _) => {
                                                                                                            source_ml_source2_data_property_value17459.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17460 = match current_2.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17461) => {
                                                                                                                source_ml_source2_data_property_value17461.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override17462 = (held_id + 1f64);
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17442.clone()), Box::new(source_ml_source2_nominal_field17444.clone()), source_ml_source2_nominal_field17446.clone(), source_ml_source2_nominal_override17462, source_ml_source2_nominal_field17450, source_ml_source2_nominal_field17452.clone(), source_ml_source2_nominal_field17454, source_ml_source2_nominal_field17456.clone(), source_ml_source2_nominal_field17458.clone(), source_ml_source2_nominal_field17460.clone())
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
                                                                    let source = {
                                                                        let source_ml_source2_constructor_field17466 = match match current_3.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17464, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17464 = *source_ml_source2_data_property_value17464;
                                                                                source_ml_source2_data_property_value17464.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17465, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17465.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field17468 = self::JavaScriptCallSource::SourceSwitch(target, Box::new(self::JavaScriptCallSource::SourceBinding(held_id)), clauses.clone(), Box::new(match rest.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17467, _) => {
                                                                                    let source_ml_source2_data_property_value17467 = *source_ml_source2_data_property_value17467;
                                                                                    source_ml_source2_data_property_value17467.clone()
                                                                                }
                                                                            }));
                                                                            self::JavaScriptCallSource::SourceLexicalScope(declarations.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field17466.clone(), Box::new(source_ml_source2_constructor_field17468.clone()))
                                                                        }
                                                                    };
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(current_3.clone(), held_id, match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17469, _) => {
                                                                            let source_ml_source2_data_property_value17469 = *source_ml_source2_data_property_value17469;
                                                                            source_ml_source2_data_property_value17469.clone()
                                                                        }
                                                                    }, source.clone(), false)), Box::new(current_3.clone()))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceSwitch(target, Box::new(match value.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17470, _) => {
                                                                let source_ml_source2_data_property_value17470 = *source_ml_source2_data_property_value17470;
                                                                source_ml_source2_data_property_value17470.clone()
                                                            }
                                                        }), clauses.clone(), Box::new(match rest.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17471, _) => {
                                                                let source_ml_source2_data_property_value17471 = *source_ml_source2_data_property_value17471;
                                                                source_ml_source2_data_property_value17471.clone()
                                                            }
                                                        }))), Box::new(current_2.clone()))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_local_declaration(identity: f64, initializer: self::JavaScriptErrorExpression, mutable: bool, shared: bool, rest: self::JavaScriptErrorSourceExpression, depth: f64) -> self::JavaScriptErrorSourceExpression {
    {
        let lexical = { let ml_values299 = { let ml_reference_value300 = ((match match rest.clone() {
            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17472) => {
                let source_ml_source2_data_property_value17472 = *source_ml_source2_data_property_value17472;
                source_ml_source2_data_property_value17472.clone()
            }
        } {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17473) => {
                ((source_ml_source2_data_property_value17473.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value300 }; let ml_callback299 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17475, _, _, _, _) => {
                source_ml_source2_data_property_value17475
            }
        }) == identity) }) }; ml_values299.iter().cloned().enumerate().any(|(ml_index299, ml_value299)| ml_callback299(ml_value299)) };
        {
            let value = self::lower_java_script_error_expression(initializer.clone(), match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17476) => {
                    let source_ml_source2_data_property_value17476 = *source_ml_source2_data_property_value17476;
                    source_ml_source2_data_property_value17476.clone()
                }
            }, depth);
            {
                let next = match value.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17477) => {
                        let source_ml_source2_data_property_value17477 = *source_ml_source2_data_property_value17477;
                        source_ml_source2_data_property_value17477.clone()
                    }
                };
                if lexical {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceInitializeCell(identity, Box::new(match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17478, _) => {
                            let source_ml_source2_data_property_value17478 = *source_ml_source2_data_property_value17478;
                            source_ml_source2_data_property_value17478.clone()
                        }
                    }), Box::new(match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17479, _) => {
                            let source_ml_source2_data_property_value17479 = *source_ml_source2_data_property_value17479;
                            source_ml_source2_data_property_value17479.clone()
                        }
                    }))), Box::new(next.clone()))
                } else {
                    {
                        let stored = if shared {
                            {
                                let source_ml_source2_constructor_field17480 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                                {
                                    let source_ml_source2_constructor_field17483 = match match next.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17481, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value17481 = *source_ml_source2_data_property_value17481;
                                            source_ml_source2_data_property_value17481.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17482, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value17482.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field17487 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                            let source_ml_source2_constructor_field17484 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                            {
                                                let source_ml_source2_constructor_field17486 = match value.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17485, _) => {
                                                        let source_ml_source2_data_property_value17485 = *source_ml_source2_data_property_value17485;
                                                        source_ml_source2_data_property_value17485.clone()
                                                    }
                                                };
                                                self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field17484.clone()), Box::new(source_ml_source2_constructor_field17486.clone()), None::<(Option<bool>,)>)
                                            }
                                        }]));
                                        self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field17480.clone()), source_ml_source2_constructor_field17487.clone(), source_ml_source2_constructor_field17483.clone())
                                    }
                                }
                            }
                        } else {
                            match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17488, _) => {
                                    let source_ml_source2_data_property_value17488 = *source_ml_source2_data_property_value17488;
                                    source_ml_source2_data_property_value17488.clone()
                                }
                            }
                        };
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), identity, stored.clone(), match rest.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17489, _) => {
                                let source_ml_source2_data_property_value17489 = *source_ml_source2_data_property_value17489;
                                source_ml_source2_data_property_value17489.clone()
                            }
                        }, (mutable && !shared))), Box::new(next.clone()))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_object_binding(parameter: self::JavaScriptErrorParameter, first: self::JavaScriptErrorSourceContext, continuation: self::JavaScriptCallSource, mutable: bool, depth: f64, initialize_existing: bool) -> self::JavaScriptErrorSourceExpression {
    if (depth > (128f64)) {
        self::refuse_java_script_error_source(first.clone(), &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 98, 105, 110, 100, 105, 110, 103, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 115, 116, 105, 110, 103, 32, 98, 117, 100, 103, 101, 116])))
    } else {
        match parameter.clone() {
            self::JavaScriptErrorParameter::ErrorObjectParameter(parameter_identity, _, _, fields, parameter_rest, rest_target) => {
                {
                    let context = first.clone();
                    {
                        let source = continuation.clone();
                        {
                            let keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<(f64, self::JavaScriptCallSource)>>>::new()));
                            {
                                let source_ml_source1_temporary_values148 = fields.clone();
                                {
                                    let source_ml_source1_temporary_index148 = self::ml::Big::from_i128(0);
                                    {
                                        let context_2 = self::source_ml_source1_temporary_lower_java_script_source_object_binding_loop149(context.clone(), keys.clone(), source_ml_source1_temporary_values148.clone(), source_ml_source1_temporary_index148.clone());
                                        {
                                            let rest = ((parameter_rest.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
                                            {
                                                let source_3 = if {
                                                    let source_ml_source2_comparison_operand17507 = None::<()>;
                                                    !(match (rest.clone(), source_ml_source2_comparison_operand17507.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                } {
                                                    {
                                                        let input = self::JavaScriptCallSource::SourceBinding(parameter_identity);
                                                        {
                                                            let excluded = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values301 = { let ml_reference_value302 = keys.borrow().clone(); ml_reference_value302 }; let ml_callback301 = std::rc::Rc::new(move |key: std::rc::Rc<std::cell::RefCell<(f64, self::JavaScriptCallSource)>>| -> self::JavaScriptCallSource { ({ let ml_reference_value303 = key.borrow().clone(); ml_reference_value303 }).1 }); ml_values301.iter().cloned().enumerate().map(|(ml_index301, ml_value301)| ml_callback301(ml_value301)).collect::<Vec<_>>() }));
                                                            {
                                                                let value = {
                                                                    let source_ml_source2_constructor_field17510 = self::construct_java_script_error_constant(match match context_2.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17508, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value17508 = *source_ml_source2_data_property_value17508;
                                                                            source_ml_source2_data_property_value17508.clone()
                                                                        }
                                                                    } {
                                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, source_ml_source2_data_property_value17509, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value17509 = *source_ml_source2_data_property_value17509;
                                                                            source_ml_source2_data_property_value17509.clone()
                                                                        }
                                                                    });
                                                                    {
                                                                        let source_ml_source2_constructor_field17513 = match match context_2.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17511, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17511 = *source_ml_source2_data_property_value17511;
                                                                                source_ml_source2_data_property_value17511.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17512, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17512.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field17516 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                                let source_ml_source2_constructor_field17515 = Some(excluded.clone());
                                                                                self::JavaScriptCallSourceProperty::SourceObjectSpread(Box::new(input.clone()), Some((source_ml_source2_constructor_field17515.clone(),)))
                                                                            }]));
                                                                            self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field17510.clone()), source_ml_source2_constructor_field17516.clone(), source_ml_source2_constructor_field17513.clone())
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let source_2 = self::construct_java_script_source_pattern_binding(context_2.clone(), rest.clone().expect("a proven present value"), value.clone(), source.clone(), mutable, initialize_existing, ((rest_target.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone());
                                                                    source_2.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    source.clone()
                                                };
                                                {
                                                    let field_at = ({ let ml_reference_array304 = fields.clone(); let ml_reference_projection304 = ml_reference_array304.borrow().len() as f64; ml_reference_projection304 } - 1f64);
                                                    {
                                                        let source_ml_source2_s17517 = self::source_ml_source1_temporary_lower_java_script_source_object_binding_loop152(parameter.clone(), mutable, depth, initialize_existing, context_2.clone(), source_3.clone(), keys.clone(), field_at);
                                                        match source_ml_source2_s17517.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Result::MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Done(context_3, source_4) => {
                                                                let context_3 = *context_3;
                                                                let source_4 = *source_4;
                                                                {
                                                                    let identity = match context_3.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17518, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value17518
                                                                        }
                                                                    };
                                                                    {
                                                                        let context_4 = {
                                                                            let source_ml_source2_nominal_field17520 = match context_3.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17521, _, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17521 = *source_ml_source2_data_property_value17521;
                                                                                    source_ml_source2_data_property_value17521.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17522 = match context_3.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17523, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value17523 = *source_ml_source2_data_property_value17523;
                                                                                        source_ml_source2_data_property_value17523.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17524 = match context_3.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17525, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value17525.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17526 = match context_3.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17527, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value17527
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field17528 = match context_3.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17529, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value17529
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field17530 = match context_3.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17531, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value17531.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field17532 = match context_3.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17533, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value17533
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17534 = match context_3.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17535, _, _) => {
                                                                                                                source_ml_source2_data_property_value17535.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field17536 = match context_3.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17537, _) => {
                                                                                                                    source_ml_source2_data_property_value17537.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field17538 = match context_3.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17539) => {
                                                                                                                        source_ml_source2_data_property_value17539.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override17540 = (identity + 1f64);
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17520.clone()), Box::new(source_ml_source2_nominal_field17522.clone()), source_ml_source2_nominal_field17524.clone(), source_ml_source2_nominal_override17540, source_ml_source2_nominal_field17528, source_ml_source2_nominal_field17530.clone(), source_ml_source2_nominal_field17532, source_ml_source2_nominal_field17534.clone(), source_ml_source2_nominal_field17536.clone(), source_ml_source2_nominal_field17538.clone())
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
                                                                            let source_5 = self::construct_java_script_error_local_binding(context_4.clone(), identity, self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 99, 111, 101, 114, 99, 105, 98, 108, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSource::SourceBinding(parameter_identity)]))), source_4.clone(), false);
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_5.clone()), Box::new(context_4.clone()))
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
                self::refuse_java_script_error_source(first.clone(), &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 98, 106, 101, 99, 116, 32, 112, 97, 116, 116, 101, 114, 110])))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_parameters(parameters: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, first: self::JavaScriptErrorSourceContext, continuation: self::JavaScriptCallSource) -> self::JavaScriptErrorSourceExpression {
    {
        let context = first.clone();
        {
            let source = continuation.clone();
            {
                let at = ({ let ml_reference_array305 = parameters.clone(); let ml_reference_projection305 = ml_reference_array305.borrow().len() as f64; ml_reference_projection305 } - 1f64);
                {
                    let source_ml_source2_s17541 = self::source_ml_source1_temporary_lower_java_script_source_parameters_loop157(parameters.clone(), context.clone(), source.clone(), at);
                    match source_ml_source2_s17541.clone() {
                        self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Done(context_2, source_2) => {
                            let context_2 = *context_2;
                            let source_2 = *source_2;
                            {
                                let owned = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let source_ml_source1_temporary_values155 = parameters.clone();
                                    {
                                        let source_ml_source1_temporary_index155 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s17542 = self::source_ml_source1_temporary_lower_java_script_source_parameters_loop158(context_2.clone(), owned.clone(), source_ml_source1_temporary_values155.clone(), source_ml_source1_temporary_index155.clone());
                                            match source_ml_source2_s17542.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Done => {
                                                    {
                                                        let source_4 = if (({ let ml_reference_array306 = owned.clone(); let ml_reference_projection306 = ml_reference_array306.borrow().len() as f64; ml_reference_projection306 }) > (0f64)) {
                                                            {
                                                                let source_3 = {
                                                                    let source_ml_source2_constructor_field17543 = owned.clone();
                                                                    {
                                                                        let source_ml_source2_constructor_field17554 = Some(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values307 = { let ml_reference_value308 = owned.borrow().clone(); ml_reference_value308 }; let ml_callback307 = { let parameters = parameters.clone(); std::rc::Rc::new(move |identity: f64| -> bool { { let ml_values309 = { let ml_reference_value310 = parameters.borrow().clone(); ml_reference_value310 }; let ml_callback309 = { let identity = identity; std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> bool { ((match parameter.clone() {
                                                                            self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value17549, _, _) => {
                                                                                source_ml_source2_data_property_value17549
                                                                            }
                                                                            self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value17550, _) => {
                                                                                source_ml_source2_data_property_value17550
                                                                            }
                                                                            self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value17551, _, _) => {
                                                                                source_ml_source2_data_property_value17551
                                                                            }
                                                                            self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value17552, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17552
                                                                            }
                                                                            self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value17553, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17553
                                                                            }
                                                                        }) == identity) }) }; ml_values309.iter().cloned().enumerate().any(|(ml_index309, ml_value309)| ml_callback309(ml_value309)) } }) }; ml_values307.iter().cloned().enumerate().filter_map(|(ml_index307, ml_value307)| if ml_callback307(ml_value307.clone()) { Some(ml_value307) } else { None }).collect::<Vec<_>>() })));
                                                                        {
                                                                            let source_ml_source2_constructor_field17557 = match match context_2.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17555, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17555 = *source_ml_source2_data_property_value17555;
                                                                                    source_ml_source2_data_property_value17555.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17556, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17556.clone()
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSource::SourceLexicalScope(source_ml_source2_constructor_field17543.clone(), Some((source_ml_source2_constructor_field17554.clone(),)), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field17557.clone(), Box::new(source_2.clone()))
                                                                        }
                                                                    }
                                                                };
                                                                source_3.clone()
                                                            }
                                                        } else {
                                                            source_2.clone()
                                                        };
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_4.clone()), Box::new(context_2.clone()))
                                                    }
                                                }
                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Return(source_ml_source1_temporary_result158) => {
                                                    let source_ml_source1_temporary_result158 = *source_ml_source1_temporary_result158;
                                                    source_ml_source1_temporary_result158.clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Return(source_ml_source1_temporary_result160) => {
                            let source_ml_source1_temporary_result160 = *source_ml_source1_temporary_result160;
                            source_ml_source1_temporary_result160.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_error_module_plan(plan: self::JavaScriptErrorModulePlan, registry: self::JavaScriptErrorModuleRegistry) -> self::JavaScriptErrorModuleCompilation {
    self::compile_java_script_error_module_plan_with_bindings(plan.clone(), registry.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptErrorModuleBinding>::new())))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_error_module_plan_with_bindings(plan: self::JavaScriptErrorModulePlan, registry: self::JavaScriptErrorModuleRegistry, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorModuleBinding>>>) -> self::JavaScriptErrorModuleCompilation {
    self::compile_java_script_error_module_plan_with_capabilities(plan.clone(), registry.clone(), bindings.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceCapabilityOwner>::new())))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_error_module_plan_with_capabilities(plan: self::JavaScriptErrorModulePlan, registry: self::JavaScriptErrorModuleRegistry, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorModuleBinding>>>, capabilities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>) -> self::JavaScriptErrorModuleCompilation {
    {
        let empty = {
            let source_ml_source2_constructor_field17559 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallFunction>::new()));
            {
                let source_ml_source2_constructor_field17561 = match registry.clone() {
                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value17560, _) => {
                        source_ml_source2_data_property_value17560.clone()
                    }
                };
                {
                    let source_ml_source2_constructor_field17563 = match registry.clone() {
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, _, source_ml_source2_data_property_value17562) => {
                            let source_ml_source2_data_property_value17562 = *source_ml_source2_data_property_value17562;
                            source_ml_source2_data_property_value17562.clone()
                        }
                    };
                    self::JavaScriptCallProgram::CallProgram(source_ml_source2_constructor_field17559.clone(), source_ml_source2_constructor_field17561.clone(), Box::new(source_ml_source2_constructor_field17563.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)>, None::<(Option<self::JavaScriptArraySpeciesDirectory>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>)
                }
            }
        };
        {
            let validation_error = self::read_java_script_error_module_plan_error(plan.clone(), registry.clone(), bindings.clone(), capabilities.clone());
            if (validation_error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), validation_error.clone())
            } else {
                {
                    let functions = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallFunction>::new()));
                    {
                        let identities = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()));
                        {
                            let source_ml_source1_temporary_values165 = match plan.clone() {
                                self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value17564, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17564.clone()
                                }
                            };
                            {
                                let source_ml_source1_temporary_index165 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s17565 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop171(plan.clone(), registry.clone(), bindings.clone(), capabilities.clone(), empty.clone(), functions.clone(), identities.clone(), source_ml_source1_temporary_values165.clone(), source_ml_source1_temporary_index165.clone());
                                    match source_ml_source2_s17565.clone() {
                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Done => {
                                            {
                                                let compiled_templates = self::compile_java_script_error_module_templates(plan.clone(), registry.clone(), bindings.clone(), capabilities.clone());
                                                if ((({ let ml_reference_value311 = compiled_templates.borrow().clone(); ml_reference_value311 }).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), ({ let ml_reference_value312 = compiled_templates.borrow().clone(); ml_reference_value312 }).0)
                                                } else {
                                                    {
                                                        let source_ml_source1_temporary_values167 = match registry.clone() {
                                                            self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, source_ml_source2_data_property_value17566, _, _) => {
                                                                source_ml_source2_data_property_value17566.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_temporary_index167 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s17567 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop172(registry.clone(), empty.clone(), functions.clone(), identities.clone(), source_ml_source1_temporary_values167.clone(), source_ml_source1_temporary_index167.clone());
                                                                match source_ml_source2_s17567.clone() {
                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Done => {
                                                                        {
                                                                            let at = 0f64;
                                                                            {
                                                                                let source_ml_source2_s17568 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop173(capabilities.clone(), empty.clone(), functions.clone(), identities.clone(), at);
                                                                                match source_ml_source2_s17568.clone() {
                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Done => {
                                                                                        {
                                                                                            let functions_2 = self::order_java_script_call_code_functions(functions.clone());
                                                                                            self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new({
                                                                                                let source_ml_source1_temporary_constructor_field169 = match registry.clone() {
                                                                                                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value17569, _) => {
                                                                                                        source_ml_source2_data_property_value17569.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_temporary_constructor_field170 = match registry.clone() {
                                                                                                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, _, source_ml_source2_data_property_value17570) => {
                                                                                                            let source_ml_source2_data_property_value17570 = *source_ml_source2_data_property_value17570;
                                                                                                            source_ml_source2_data_property_value17570.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    if !(match (match plan.clone() {
                                                                                                        self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, source_ml_source2_data_property_value17571, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value17571.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>>,))).0).clone()
                                                                                                        }
                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                        {
                                                                                                            let source_ml_source2_constructor_field17575 = Some(({ let ml_reference_value313 = compiled_templates.borrow().clone(); ml_reference_value313 }).1);
                                                                                                            self::JavaScriptCallProgram::CallProgram(functions_2.clone(), source_ml_source1_temporary_constructor_field169.clone(), Box::new(source_ml_source1_temporary_constructor_field170.clone()), Some((source_ml_source2_constructor_field17575.clone(),)), None::<(Option<self::JavaScriptArraySpeciesDirectory>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>)
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::JavaScriptCallProgram::CallProgram(functions_2.clone(), source_ml_source1_temporary_constructor_field169.clone(), Box::new(source_ml_source1_temporary_constructor_field170.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)>, None::<(Option<self::JavaScriptArraySpeciesDirectory>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>)
                                                                                                    }
                                                                                                }
                                                                                            }), self::ml_string::Text::from_units(vec![]))
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Return(source_ml_source1_temporary_result173) => {
                                                                                        let source_ml_source1_temporary_result173 = *source_ml_source1_temporary_result173;
                                                                                        source_ml_source1_temporary_result173.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Return(source_ml_source1_temporary_result175) => {
                                                                        let source_ml_source1_temporary_result175 = *source_ml_source1_temporary_result175;
                                                                        source_ml_source1_temporary_result175.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(source_ml_source1_temporary_result177) => {
                                            let source_ml_source1_temporary_result177 = *source_ml_source1_temporary_result177;
                                            source_ml_source1_temporary_result177.clone()
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
