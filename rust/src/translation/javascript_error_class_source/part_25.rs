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
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17314) => {
                    let source_ml_source2_data_property_value17314 = *source_ml_source2_data_property_value17314;
                    source_ml_source2_data_property_value17314.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceFinallyStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17315) => {
                    let source_ml_source2_data_property_value17315 = *source_ml_source2_data_property_value17315;
                    source_ml_source2_data_property_value17315.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceTryStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17316) => {
                    let source_ml_source2_data_property_value17316 = *source_ml_source2_data_property_value17316;
                    source_ml_source2_data_property_value17316.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceArrayBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17317) => {
                    let source_ml_source2_data_property_value17317 = *source_ml_source2_data_property_value17317;
                    source_ml_source2_data_property_value17317.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceObjectBindingStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17318) => {
                    let source_ml_source2_data_property_value17318 = *source_ml_source2_data_property_value17318;
                    source_ml_source2_data_property_value17318.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceVariableStatement(_, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17319) => {
                    let source_ml_source2_data_property_value17319 = *source_ml_source2_data_property_value17319;
                    source_ml_source2_data_property_value17319.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceConstStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17320) => {
                    let source_ml_source2_data_property_value17320 = *source_ml_source2_data_property_value17320;
                    source_ml_source2_data_property_value17320.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceMutableStatement(_, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17321) => {
                    let source_ml_source2_data_property_value17321 = *source_ml_source2_data_property_value17321;
                    source_ml_source2_data_property_value17321.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceExpressionStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17322) => {
                    let source_ml_source2_data_property_value17322 = *source_ml_source2_data_property_value17322;
                    source_ml_source2_data_property_value17322.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17323) => {
                    let source_ml_source2_data_property_value17323 = *source_ml_source2_data_property_value17323;
                    source_ml_source2_data_property_value17323.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17324) => {
                    let source_ml_source2_data_property_value17324 = *source_ml_source2_data_property_value17324;
                    source_ml_source2_data_property_value17324.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceIfStatement(statement_condition, statement_yes, statement_no) => {
            let statement_condition = *statement_condition;
            {
                let condition = self::lower_java_script_error_expression(statement_condition.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17325) => {
                        let source_ml_source2_data_property_value17325 = *source_ml_source2_data_property_value17325;
                        source_ml_source2_data_property_value17325.clone()
                    }
                }, (depth + 1f64));
                {
                    let yes = self::lower_java_script_source_statements(statement_yes.clone(), 0f64, match condition.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17326) => {
                            let source_ml_source2_data_property_value17326 = *source_ml_source2_data_property_value17326;
                            source_ml_source2_data_property_value17326.clone()
                        }
                    }, match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17327, _) => {
                            let source_ml_source2_data_property_value17327 = *source_ml_source2_data_property_value17327;
                            source_ml_source2_data_property_value17327.clone()
                        }
                    }, (depth + 1f64), true);
                    {
                        let no = self::lower_java_script_source_statements(statement_no.clone(), 0f64, match yes.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17328) => {
                                let source_ml_source2_data_property_value17328 = *source_ml_source2_data_property_value17328;
                                source_ml_source2_data_property_value17328.clone()
                            }
                        }, match rest.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17329, _) => {
                                let source_ml_source2_data_property_value17329 = *source_ml_source2_data_property_value17329;
                                source_ml_source2_data_property_value17329.clone()
                            }
                        }, (depth + 1f64), true);
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceIf(Box::new(match condition.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17330, _) => {
                                let source_ml_source2_data_property_value17330 = *source_ml_source2_data_property_value17330;
                                source_ml_source2_data_property_value17330.clone()
                            }
                        }), Box::new(match yes.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17331, _) => {
                                let source_ml_source2_data_property_value17331 = *source_ml_source2_data_property_value17331;
                                source_ml_source2_data_property_value17331.clone()
                            }
                        }), Box::new(match no.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17332, _) => {
                                let source_ml_source2_data_property_value17332 = *source_ml_source2_data_property_value17332;
                                source_ml_source2_data_property_value17332.clone()
                            }
                        }))), Box::new(match no.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17333) => {
                                let source_ml_source2_data_property_value17333 = *source_ml_source2_data_property_value17333;
                                source_ml_source2_data_property_value17333.clone()
                            }
                        }))
                    }
                }
            }
        }
        self::JavaScriptSourceStatement::SourceBlockStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17334) => {
                    let source_ml_source2_data_property_value17334 = *source_ml_source2_data_property_value17334;
                    source_ml_source2_data_property_value17334.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(_) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17335) => {
                    let source_ml_source2_data_property_value17335 = *source_ml_source2_data_property_value17335;
                    source_ml_source2_data_property_value17335.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceWhileStatement(_, _, _, _, _, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17336) => {
                    let source_ml_source2_data_property_value17336 = *source_ml_source2_data_property_value17336;
                    source_ml_source2_data_property_value17336.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlBlockStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17337) => {
                    let source_ml_source2_data_property_value17337 = *source_ml_source2_data_property_value17337;
                    source_ml_source2_data_property_value17337.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
            self::refuse_java_script_error_source(match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17338) => {
                    let source_ml_source2_data_property_value17338 = *source_ml_source2_data_property_value17338;
                    source_ml_source2_data_property_value17338.clone()
                }
            }, &(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 101, 109, 101, 110, 116, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptSourceStatement::SourceSwitchStatement(target, statement_value, statement_clauses) => {
            let statement_value = *statement_value;
            {
                let value = self::lower_java_script_error_expression(statement_value.clone(), match rest.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17339) => {
                        let source_ml_source2_data_property_value17339 = *source_ml_source2_data_property_value17339;
                        source_ml_source2_data_property_value17339.clone()
                    }
                }, (depth + 1f64));
                {
                    let current = match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17340) => {
                            let source_ml_source2_data_property_value17340 = *source_ml_source2_data_property_value17340;
                            source_ml_source2_data_property_value17340.clone()
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
                                            let source_ml_source2_s17341 = self::source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop145(current_2.clone(), declarations.clone(), statement_clauses.clone(), source_ml_source1_temporary_index140.clone());
                                            match source_ml_source2_s17341.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Result::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Done => {
                                                    if (({ let ml_reference_array298 = declarations.clone(); let ml_reference_projection298 = ml_reference_array298.borrow().len() as f64; ml_reference_projection298 }) > (0f64)) {
                                                        {
                                                            let held_id = match current_2.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17342, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17342
                                                                }
                                                            };
                                                            {
                                                                let current_3 = {
                                                                    let source_ml_source2_nominal_field17344 = match current_2.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17345, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value17345 = *source_ml_source2_data_property_value17345;
                                                                            source_ml_source2_data_property_value17345.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17346 = match current_2.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17347, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17347 = *source_ml_source2_data_property_value17347;
                                                                                source_ml_source2_data_property_value17347.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17348 = match current_2.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17349, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17349.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17350 = match current_2.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17351, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value17351
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17352 = match current_2.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17353, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value17353
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17354 = match current_2.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17355, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value17355.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field17356 = match current_2.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17357, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value17357
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field17358 = match current_2.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17359, _, _) => {
                                                                                                        source_ml_source2_data_property_value17359.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field17360 = match current_2.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17361, _) => {
                                                                                                            source_ml_source2_data_property_value17361.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17362 = match current_2.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17363) => {
                                                                                                                source_ml_source2_data_property_value17363.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override17364 = (held_id + 1f64);
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17344.clone()), Box::new(source_ml_source2_nominal_field17346.clone()), source_ml_source2_nominal_field17348.clone(), source_ml_source2_nominal_override17364, source_ml_source2_nominal_field17352, source_ml_source2_nominal_field17354.clone(), source_ml_source2_nominal_field17356, source_ml_source2_nominal_field17358.clone(), source_ml_source2_nominal_field17360.clone(), source_ml_source2_nominal_field17362.clone())
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
                                                                        let source_ml_source2_constructor_field17368 = match match current_3.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17366, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17366 = *source_ml_source2_data_property_value17366;
                                                                                source_ml_source2_data_property_value17366.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17367, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17367.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field17370 = self::JavaScriptCallSource::SourceSwitch(target, Box::new(self::JavaScriptCallSource::SourceBinding(held_id)), clauses.clone(), Box::new(match rest.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17369, _) => {
                                                                                    let source_ml_source2_data_property_value17369 = *source_ml_source2_data_property_value17369;
                                                                                    source_ml_source2_data_property_value17369.clone()
                                                                                }
                                                                            }));
                                                                            self::JavaScriptCallSource::SourceLexicalScope(declarations.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field17368.clone(), Box::new(source_ml_source2_constructor_field17370.clone()))
                                                                        }
                                                                    };
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(current_3.clone(), held_id, match value.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17371, _) => {
                                                                            let source_ml_source2_data_property_value17371 = *source_ml_source2_data_property_value17371;
                                                                            source_ml_source2_data_property_value17371.clone()
                                                                        }
                                                                    }, source.clone(), false)), Box::new(current_3.clone()))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceSwitch(target, Box::new(match value.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17372, _) => {
                                                                let source_ml_source2_data_property_value17372 = *source_ml_source2_data_property_value17372;
                                                                source_ml_source2_data_property_value17372.clone()
                                                            }
                                                        }), clauses.clone(), Box::new(match rest.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17373, _) => {
                                                                let source_ml_source2_data_property_value17373 = *source_ml_source2_data_property_value17373;
                                                                source_ml_source2_data_property_value17373.clone()
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
            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17374) => {
                let source_ml_source2_data_property_value17374 = *source_ml_source2_data_property_value17374;
                source_ml_source2_data_property_value17374.clone()
            }
        } {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17375) => {
                ((source_ml_source2_data_property_value17375.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value300 }; let ml_callback299 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17377, _, _, _, _) => {
                source_ml_source2_data_property_value17377
            }
        }) == identity) }) }; ml_values299.iter().cloned().enumerate().any(|(ml_index299, ml_value299)| ml_callback299(ml_value299)) };
        {
            let value = self::lower_java_script_error_expression(initializer.clone(), match rest.clone() {
                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17378) => {
                    let source_ml_source2_data_property_value17378 = *source_ml_source2_data_property_value17378;
                    source_ml_source2_data_property_value17378.clone()
                }
            }, depth);
            {
                let next = match value.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17379) => {
                        let source_ml_source2_data_property_value17379 = *source_ml_source2_data_property_value17379;
                        source_ml_source2_data_property_value17379.clone()
                    }
                };
                if lexical {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceInitializeCell(identity, Box::new(match value.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17380, _) => {
                            let source_ml_source2_data_property_value17380 = *source_ml_source2_data_property_value17380;
                            source_ml_source2_data_property_value17380.clone()
                        }
                    }), Box::new(match rest.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17381, _) => {
                            let source_ml_source2_data_property_value17381 = *source_ml_source2_data_property_value17381;
                            source_ml_source2_data_property_value17381.clone()
                        }
                    }))), Box::new(next.clone()))
                } else {
                    {
                        let stored = if shared {
                            {
                                let source_ml_source2_constructor_field17382 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                                {
                                    let source_ml_source2_constructor_field17385 = match match next.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17383, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value17383 = *source_ml_source2_data_property_value17383;
                                            source_ml_source2_data_property_value17383.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17384, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value17384.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field17389 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                            let source_ml_source2_constructor_field17386 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                            {
                                                let source_ml_source2_constructor_field17388 = match value.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17387, _) => {
                                                        let source_ml_source2_data_property_value17387 = *source_ml_source2_data_property_value17387;
                                                        source_ml_source2_data_property_value17387.clone()
                                                    }
                                                };
                                                self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field17386.clone()), Box::new(source_ml_source2_constructor_field17388.clone()), None::<(Option<bool>,)>)
                                            }
                                        }]));
                                        self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field17382.clone()), source_ml_source2_constructor_field17389.clone(), source_ml_source2_constructor_field17385.clone())
                                    }
                                }
                            }
                        } else {
                            match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17390, _) => {
                                    let source_ml_source2_data_property_value17390 = *source_ml_source2_data_property_value17390;
                                    source_ml_source2_data_property_value17390.clone()
                                }
                            }
                        };
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), identity, stored.clone(), match rest.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17391, _) => {
                                let source_ml_source2_data_property_value17391 = *source_ml_source2_data_property_value17391;
                                source_ml_source2_data_property_value17391.clone()
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
                                                    let source_ml_source2_comparison_operand17409 = None::<()>;
                                                    !(match (rest.clone(), source_ml_source2_comparison_operand17409.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                } {
                                                    {
                                                        let input = self::JavaScriptCallSource::SourceBinding(parameter_identity);
                                                        {
                                                            let excluded = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values301 = { let ml_reference_value302 = keys.borrow().clone(); ml_reference_value302 }; let ml_callback301 = std::rc::Rc::new(move |key: std::rc::Rc<std::cell::RefCell<(f64, self::JavaScriptCallSource)>>| -> self::JavaScriptCallSource { ({ let ml_reference_value303 = key.borrow().clone(); ml_reference_value303 }).1 }); ml_values301.iter().cloned().enumerate().map(|(ml_index301, ml_value301)| ml_callback301(ml_value301)).collect::<Vec<_>>() }));
                                                            {
                                                                let value = {
                                                                    let source_ml_source2_constructor_field17412 = self::construct_java_script_error_constant(match match context_2.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17410, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value17410 = *source_ml_source2_data_property_value17410;
                                                                            source_ml_source2_data_property_value17410.clone()
                                                                        }
                                                                    } {
                                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, source_ml_source2_data_property_value17411, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value17411 = *source_ml_source2_data_property_value17411;
                                                                            source_ml_source2_data_property_value17411.clone()
                                                                        }
                                                                    });
                                                                    {
                                                                        let source_ml_source2_constructor_field17415 = match match context_2.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17413, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value17413 = *source_ml_source2_data_property_value17413;
                                                                                source_ml_source2_data_property_value17413.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17414, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17414.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_constructor_field17418 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                                let source_ml_source2_constructor_field17417 = Some(excluded.clone());
                                                                                self::JavaScriptCallSourceProperty::SourceObjectSpread(Box::new(input.clone()), Some((source_ml_source2_constructor_field17417.clone(),)))
                                                                            }]));
                                                                            self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field17412.clone()), source_ml_source2_constructor_field17418.clone(), source_ml_source2_constructor_field17415.clone())
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
                                                        let source_ml_source2_s17419 = self::source_ml_source1_temporary_lower_java_script_source_object_binding_loop152(parameter.clone(), mutable, depth, initialize_existing, context_2.clone(), source_3.clone(), keys.clone(), field_at);
                                                        match source_ml_source2_s17419.clone() {
                                                            self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Result::MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Done(context_3, source_4) => {
                                                                let context_3 = *context_3;
                                                                let source_4 = *source_4;
                                                                {
                                                                    let identity = match context_3.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17420, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value17420
                                                                        }
                                                                    };
                                                                    {
                                                                        let context_4 = {
                                                                            let source_ml_source2_nominal_field17422 = match context_3.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17423, _, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17423 = *source_ml_source2_data_property_value17423;
                                                                                    source_ml_source2_data_property_value17423.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field17424 = match context_3.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17425, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value17425 = *source_ml_source2_data_property_value17425;
                                                                                        source_ml_source2_data_property_value17425.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field17426 = match context_3.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17427, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value17427.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17428 = match context_3.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17429, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value17429
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field17430 = match context_3.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17431, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value17431
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field17432 = match context_3.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17433, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value17433.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field17434 = match context_3.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17435, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value17435
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17436 = match context_3.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17437, _, _) => {
                                                                                                                source_ml_source2_data_property_value17437.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field17438 = match context_3.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17439, _) => {
                                                                                                                    source_ml_source2_data_property_value17439.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field17440 = match context_3.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17441) => {
                                                                                                                        source_ml_source2_data_property_value17441.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override17442 = (identity + 1f64);
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17422.clone()), Box::new(source_ml_source2_nominal_field17424.clone()), source_ml_source2_nominal_field17426.clone(), source_ml_source2_nominal_override17442, source_ml_source2_nominal_field17430, source_ml_source2_nominal_field17432.clone(), source_ml_source2_nominal_field17434, source_ml_source2_nominal_field17436.clone(), source_ml_source2_nominal_field17438.clone(), source_ml_source2_nominal_field17440.clone())
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
                    let source_ml_source2_s17443 = self::source_ml_source1_temporary_lower_java_script_source_parameters_loop157(parameters.clone(), context.clone(), source.clone(), at);
                    match source_ml_source2_s17443.clone() {
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
                                            let source_ml_source2_s17444 = self::source_ml_source1_temporary_lower_java_script_source_parameters_loop158(context_2.clone(), owned.clone(), source_ml_source1_temporary_values155.clone(), source_ml_source1_temporary_index155.clone());
                                            match source_ml_source2_s17444.clone() {
                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Done => {
                                                    {
                                                        let source_4 = if (({ let ml_reference_array306 = owned.clone(); let ml_reference_projection306 = ml_reference_array306.borrow().len() as f64; ml_reference_projection306 }) > (0f64)) {
                                                            {
                                                                let source_3 = {
                                                                    let source_ml_source2_constructor_field17445 = owned.clone();
                                                                    {
                                                                        let source_ml_source2_constructor_field17456 = Some(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values307 = { let ml_reference_value308 = owned.borrow().clone(); ml_reference_value308 }; let ml_callback307 = { let parameters = parameters.clone(); std::rc::Rc::new(move |identity: f64| -> bool { { let ml_values309 = { let ml_reference_value310 = parameters.borrow().clone(); ml_reference_value310 }; let ml_callback309 = { let identity = identity; std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> bool { ((match parameter.clone() {
                                                                            self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value17451, _, _) => {
                                                                                source_ml_source2_data_property_value17451
                                                                            }
                                                                            self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value17452, _) => {
                                                                                source_ml_source2_data_property_value17452
                                                                            }
                                                                            self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value17453, _, _) => {
                                                                                source_ml_source2_data_property_value17453
                                                                            }
                                                                            self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value17454, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17454
                                                                            }
                                                                            self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value17455, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value17455
                                                                            }
                                                                        }) == identity) }) }; ml_values309.iter().cloned().enumerate().any(|(ml_index309, ml_value309)| ml_callback309(ml_value309)) } }) }; ml_values307.iter().cloned().enumerate().filter_map(|(ml_index307, ml_value307)| if ml_callback307(ml_value307.clone()) { Some(ml_value307) } else { None }).collect::<Vec<_>>() })));
                                                                        {
                                                                            let source_ml_source2_constructor_field17459 = match match context_2.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17457, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value17457 = *source_ml_source2_data_property_value17457;
                                                                                    source_ml_source2_data_property_value17457.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17458, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value17458.clone()
                                                                                }
                                                                            };
                                                                            self::JavaScriptCallSource::SourceLexicalScope(source_ml_source2_constructor_field17445.clone(), Some((source_ml_source2_constructor_field17456.clone(),)), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>>,)>, source_ml_source2_constructor_field17459.clone(), Box::new(source_2.clone()))
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
            let source_ml_source2_constructor_field17461 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallFunction>::new()));
            {
                let source_ml_source2_constructor_field17463 = match registry.clone() {
                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value17462, _) => {
                        source_ml_source2_data_property_value17462.clone()
                    }
                };
                {
                    let source_ml_source2_constructor_field17465 = match registry.clone() {
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, _, source_ml_source2_data_property_value17464) => {
                            let source_ml_source2_data_property_value17464 = *source_ml_source2_data_property_value17464;
                            source_ml_source2_data_property_value17464.clone()
                        }
                    };
                    self::JavaScriptCallProgram::CallProgram(source_ml_source2_constructor_field17461.clone(), source_ml_source2_constructor_field17463.clone(), Box::new(source_ml_source2_constructor_field17465.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)>, None::<(Option<self::JavaScriptArraySpeciesDirectory>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>)
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
                                self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value17466, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17466.clone()
                                }
                            };
                            {
                                let source_ml_source1_temporary_index165 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s17467 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop171(plan.clone(), registry.clone(), bindings.clone(), capabilities.clone(), empty.clone(), functions.clone(), identities.clone(), source_ml_source1_temporary_values165.clone(), source_ml_source1_temporary_index165.clone());
                                    match source_ml_source2_s17467.clone() {
                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Done => {
                                            {
                                                let compiled_templates = self::compile_java_script_error_module_templates(plan.clone(), registry.clone(), bindings.clone(), capabilities.clone());
                                                if ((({ let ml_reference_value311 = compiled_templates.borrow().clone(); ml_reference_value311 }).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), ({ let ml_reference_value312 = compiled_templates.borrow().clone(); ml_reference_value312 }).0)
                                                } else {
                                                    {
                                                        let source_ml_source1_temporary_values167 = match registry.clone() {
                                                            self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, source_ml_source2_data_property_value17468, _, _) => {
                                                                source_ml_source2_data_property_value17468.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_temporary_index167 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s17469 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop172(registry.clone(), empty.clone(), functions.clone(), identities.clone(), source_ml_source1_temporary_values167.clone(), source_ml_source1_temporary_index167.clone());
                                                                match source_ml_source2_s17469.clone() {
                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Done => {
                                                                        {
                                                                            let at = 0f64;
                                                                            {
                                                                                let source_ml_source2_s17470 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop173(capabilities.clone(), empty.clone(), functions.clone(), identities.clone(), at);
                                                                                match source_ml_source2_s17470.clone() {
                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Done => {
                                                                                        {
                                                                                            let functions_2 = self::order_java_script_call_code_functions(functions.clone());
                                                                                            self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new({
                                                                                                let source_ml_source1_temporary_constructor_field169 = match registry.clone() {
                                                                                                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value17471, _) => {
                                                                                                        source_ml_source2_data_property_value17471.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_temporary_constructor_field170 = match registry.clone() {
                                                                                                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, _, source_ml_source2_data_property_value17472) => {
                                                                                                            let source_ml_source2_data_property_value17472 = *source_ml_source2_data_property_value17472;
                                                                                                            source_ml_source2_data_property_value17472.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    if !(match (match plan.clone() {
                                                                                                        self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, source_ml_source2_data_property_value17473, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value17473.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>>,))).0).clone()
                                                                                                        }
                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                        {
                                                                                                            let source_ml_source2_constructor_field17477 = Some(({ let ml_reference_value313 = compiled_templates.borrow().clone(); ml_reference_value313 }).1);
                                                                                                            self::JavaScriptCallProgram::CallProgram(functions_2.clone(), source_ml_source1_temporary_constructor_field169.clone(), Box::new(source_ml_source1_temporary_constructor_field170.clone()), Some((source_ml_source2_constructor_field17477.clone(),)), None::<(Option<self::JavaScriptArraySpeciesDirectory>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>)
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
