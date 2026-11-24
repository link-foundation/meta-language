// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallSource;
use super::JavaScriptCallTemplate;
use super::JavaScriptErrorArrowTemplatePlan;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorFunctionPlan;
use super::JavaScriptErrorModuleBinding;
use super::JavaScriptErrorModulePlan;
use super::JavaScriptErrorModuleRegistry;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceReference;
use super::JavaScriptSourceReferenceCapture;
use super::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result;
use super::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result;
use super::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result;
use super::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Result;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_property_read;
use super::construct_java_script_lexical_cell_failure;
use super::construct_java_script_lexical_cell_read;
use super::construct_java_script_source_field_key;
use super::lower_java_script_error_expression;
use super::lower_java_script_source_statements;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_compile_java_script_error_module_templates_loop197;
use super::source_ml_source1_temporary_construct_java_script_source_reference_captures_loop210;
use super::source_ml_source1_temporary_order_java_script_error_call_templates_loop201;
use super::source_ml_source1_temporary_read_java_script_error_module_plan_error_loop184;
use super::source_ml_source1_temporary_read_java_script_error_module_plan_error_loop185;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_module_plan_error(plan: self::JavaScriptErrorModulePlan, registry: self::JavaScriptErrorModuleRegistry, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorModuleBinding>>>, capabilities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>) -> self::ml_string::Text {
    {
        let lexical = (match plan.clone() {
            self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, _, source_ml_source2_data_property_value17484) => {
                ((source_ml_source2_data_property_value17484.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())));
        if ((({ let ml_reference_array314 = lexical.clone(); let ml_reference_projection314 = ml_reference_array314.borrow().len() as f64; ml_reference_projection314 }) > (4096f64)) || { let ml_values315 = { let ml_reference_value316 = lexical.borrow().clone(); ml_reference_value316 }; let ml_callback315 = { let lexical = lexical.clone(); let plan = plan.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding, index: f64| -> bool { {
            let prior = if (index > (0f64)) {
                {
                    let source_ml_source2_array_read_value17485 = lexical.clone();
                    {
                        let source_ml_source2_array_read_index17486 = (index - 1f64);
                        { let ml_reference_array317 = source_ml_source2_array_read_value17485.clone(); let ml_reference_position317 = source_ml_source2_array_read_index17486; let ml_reference_projection317 = self::ml_array::optional_at(&*ml_reference_array317.borrow(), self::ml_array::number_index(ml_reference_position317)); ml_reference_projection317 }
                    }
                }
            } else {
                None::<self::JavaScriptSourceLexicalBinding>
            };
            ((((((((((((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, _) => {
                    false
                }
            } || !(self::ml_math::is_safe_integer(match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17492, _, _, _, _) => {
                    source_ml_source2_data_property_value17492
                }
            }))) || {
                let source_ml_source2_same_value_operand17494 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17493, _, _, _, _) => {
                        source_ml_source2_data_property_value17493
                    }
                };
                {
                    let source_ml_source2_same_value_operand17495 = (-0f64);
                    (((source_ml_source2_same_value_operand17494 == source_ml_source2_same_value_operand17495) && ((source_ml_source2_same_value_operand17494 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17494)) == ((1f64 / source_ml_source2_same_value_operand17495))))) || ((source_ml_source2_same_value_operand17494 != source_ml_source2_same_value_operand17494) && (source_ml_source2_same_value_operand17495 != source_ml_source2_same_value_operand17495)))
                }
            }) || ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17497, _, _, _, _) => {
                    source_ml_source2_data_property_value17497
                }
            }) < (0f64))) || ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17500, _, _, _, _) => {
                    source_ml_source2_data_property_value17500
                }
            }) >= (match plan.clone() {
                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17501, _, _, _) => {
                    source_ml_source2_data_property_value17501
                }
            }))) || ((index > (0f64)) && ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17504, _, _, _, _) => {
                    source_ml_source2_data_property_value17504
                }
            }) <= (if match (prior.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                (-1f64)
            } else {
                match prior.clone().expect("a proven present value") {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17505, _, _, _, _) => {
                        source_ml_source2_data_property_value17505
                    }
                }
            })))) || (({
                let source_ml_source2_type_query_operand17509 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value17508, _, _, _) => {
                        source_ml_source2_data_property_value17508.clone()
                    }
                };
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value17511, _, _, _) => {
                    source_ml_source2_data_property_value17511.clone()
                }
            }).length()) == (0f64))) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value17513, _, _, _) => {
                    source_ml_source2_data_property_value17513.clone()
                }
            }).length()) > (4096f64))) || (({
                let source_ml_source2_type_query_operand17517 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value17516, _, _) => {
                        source_ml_source2_data_property_value17516
                    }
                };
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || ({
                let source_ml_source2_comparison_operand17519 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17518, _) => {
                        ((source_ml_source2_data_property_value17518.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17520 = None::<()>;
                    !(match (source_ml_source2_comparison_operand17519.clone(), source_ml_source2_comparison_operand17520.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                }
            } && (({
                let source_ml_source2_type_query_operand17524 = (match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17523, _) => {
                        ((source_ml_source2_data_property_value17523.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                }).clone().expect("a proven present value");
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))))) || ({
                let source_ml_source2_comparison_operand17526 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17525, _) => {
                        ((source_ml_source2_data_property_value17525.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17527 = true;
                    match (source_ml_source2_comparison_operand17526.clone(), source_ml_source2_comparison_operand17527) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }
            } && match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value17528, _, _) => {
                    source_ml_source2_data_property_value17528
                }
            })) || ({
                let source_ml_source2_comparison_operand17530 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17529) => {
                        ((source_ml_source2_data_property_value17529.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17531 = None::<()>;
                    !(match (source_ml_source2_comparison_operand17530.clone(), source_ml_source2_comparison_operand17531.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                }
            } && ((((!(self::ml_math::is_safe_integer((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17532) => {
                    ((source_ml_source2_data_property_value17532.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                }
            }).clone().expect("a proven present value"))) || {
                let source_ml_source2_same_value_operand17534 = (match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17533) => {
                        ((source_ml_source2_data_property_value17533.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                    }
                }).clone().expect("a proven present value");
                {
                    let source_ml_source2_same_value_operand17535 = (-0f64);
                    (((source_ml_source2_same_value_operand17534 == source_ml_source2_same_value_operand17535) && ((source_ml_source2_same_value_operand17534 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17534)) == ((1f64 / source_ml_source2_same_value_operand17535))))) || ((source_ml_source2_same_value_operand17534 != source_ml_source2_same_value_operand17534) && (source_ml_source2_same_value_operand17535 != source_ml_source2_same_value_operand17535)))
                }
            }) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17537) => {
                    ((source_ml_source2_data_property_value17537.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                }
            }).clone().expect("a proven present value")) < (0f64))) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17540) => {
                    ((source_ml_source2_data_property_value17540.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                }
            }).clone().expect("a proven present value")) >= (match plan.clone() {
                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17541, _, _, _) => {
                    source_ml_source2_data_property_value17541
                }
            }))) || {
                let source_ml_source2_comparison_operand17543 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17542, _) => {
                        ((source_ml_source2_data_property_value17542.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17544 = true;
                    !(match (source_ml_source2_comparison_operand17543.clone(), source_ml_source2_comparison_operand17544) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            })))
        } }) }; ml_values315.iter().cloned().enumerate().any(|(ml_index315, ml_value315)| ml_callback315(ml_value315, (ml_index315 as f64))) }) {
            self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 110, 97, 109, 101, 115, 32, 97, 110, 100, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 102, 108, 97, 103, 115])
        } else {
            {
                let shared = (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, source_ml_source2_data_property_value17545, _) => {
                        ((source_ml_source2_data_property_value17545.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                if ((({ let ml_reference_array318 = shared.clone(); let ml_reference_projection318 = ml_reference_array318.borrow().len() as f64; ml_reference_projection318 }) > (4096f64)) || { let ml_values319 = { let ml_reference_value320 = shared.borrow().clone(); ml_reference_value320 }; let ml_callback319 = { let plan = plan.clone(); let shared = shared.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { ((((!(self::ml_math::is_safe_integer(identity)) || {
                    let source_ml_source2_same_value_operand17546 = identity;
                    {
                        let source_ml_source2_same_value_operand17547 = (-0f64);
                        (((source_ml_source2_same_value_operand17546 == source_ml_source2_same_value_operand17547) && ((source_ml_source2_same_value_operand17546 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17546)) == ((1f64 / source_ml_source2_same_value_operand17547))))) || ((source_ml_source2_same_value_operand17546 != source_ml_source2_same_value_operand17546) && (source_ml_source2_same_value_operand17547 != source_ml_source2_same_value_operand17547)))
                    }
                }) || (identity < (0f64))) || (identity >= (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17549, _, _, _) => {
                        source_ml_source2_data_property_value17549
                    }
                }))) || ((index > (0f64)) && (identity <= (({
                    let source_ml_source2_array_read_value17552 = shared.clone();
                    {
                        let source_ml_source2_array_read_index17553 = (index - 1f64);
                        { let ml_reference_array321 = source_ml_source2_array_read_value17552.clone(); let ml_reference_position321 = source_ml_source2_array_read_index17553; let ml_reference_projection321 = self::ml_array::optional_at(&*ml_reference_array321.borrow(), self::ml_array::number_index(ml_reference_position321)); ml_reference_projection321 }
                    }
                }).clone().unwrap_or_else(|| (-1f64)))))) }) }; ml_values319.iter().cloned().enumerate().any(|(ml_index319, ml_value319)| ml_callback319(ml_value319, (ml_index319 as f64))) }) {
                    self::ml_string::Text::from_units(vec![115, 104, 97, 114, 101, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                } else {
                    if (({ let ml_reference_array322 = capabilities.clone(); let ml_reference_projection322 = ml_reference_array322.borrow().len() as f64; ml_reference_projection322 }) > (4096f64)) {
                        self::ml_string::Text::from_units(vec![113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116])
                    } else {
                        {
                            let at = 0f64;
                            {
                                let source_ml_source2_s17554 = self::source_ml_source1_temporary_read_java_script_error_module_plan_error_loop184(registry.clone(), capabilities.clone(), at);
                                match source_ml_source2_s17554.clone() {
                                    self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Done => {
                                        if (({ let ml_reference_array323 = bindings.clone(); let ml_reference_projection323 = ml_reference_array323.borrow().len() as f64; ml_reference_projection323 }) > (4096f64)) {
                                            self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 109, 111, 100, 117, 108, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101, 32, 98, 117, 100, 103, 101, 116])
                                        } else {
                                            {
                                                let at_2 = 0f64;
                                                {
                                                    let source_ml_source2_s17555 = self::source_ml_source1_temporary_read_java_script_error_module_plan_error_loop185(plan.clone(), bindings.clone(), at_2);
                                                    match source_ml_source2_s17555.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Done => {
                                                            if (((((({ let ml_reference_array324 = match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value17557, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17557.clone()
                                                                }
                                                            }; let ml_reference_projection324 = ml_reference_array324.borrow().len() as f64; ml_reference_projection324 }) > (4096f64)) || (({ let ml_reference_array325 = match registry.clone() {
                                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value17560, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17560.clone()
                                                                }
                                                            }; let ml_reference_projection325 = ml_reference_array325.borrow().len() as f64; ml_reference_projection325 }) != ({ let ml_reference_array326 = match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value17561, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17561.clone()
                                                                }
                                                            }; let ml_reference_projection326 = ml_reference_array326.borrow().len() as f64; ml_reference_projection326 }))) || (({ let ml_reference_array327 = match registry.clone() {
                                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, source_ml_source2_data_property_value17563, _, _) => {
                                                                    source_ml_source2_data_property_value17563.clone()
                                                                }
                                                            }; let ml_reference_projection327 = ml_reference_array327.borrow().len() as f64; ml_reference_projection327 }) > (4096f64))) || ((match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17565, _, _, _) => {
                                                                    source_ml_source2_data_property_value17565
                                                                }
                                                            }) < (0f64))) || !(self::ml_math::is_safe_integer(match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17566, _, _, _) => {
                                                                    source_ml_source2_data_property_value17566
                                                                }
                                                            }))) {
                                                                self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 109, 111, 100, 117, 108, 101, 32, 112, 108, 97, 110, 115, 32, 110, 101, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 103, 105, 115, 116, 114, 105, 101, 115])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![])
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Return(source_ml_source1_temporary_result185) => {
                                                            source_ml_source1_temporary_result185.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Return(source_ml_source1_temporary_result185_2) => {
                                        source_ml_source1_temporary_result185_2.clone()
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
pub fn compile_java_script_error_module_templates(plan: self::JavaScriptErrorModulePlan, registry: self::JavaScriptErrorModuleRegistry, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorModuleBinding>>>, capabilities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>)>> {
    {
        let templates = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallTemplate>::new()));
        if (({ let ml_reference_array328 = (match plan.clone() {
            self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, source_ml_source2_data_property_value17568, _, _) => {
                ((source_ml_source2_data_property_value17568.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptErrorArrowTemplatePlan>::new()))); let ml_reference_projection328 = ml_reference_array328.borrow().len() as f64; ml_reference_projection328 }) > (4096f64)) {
            std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 97, 114, 114, 111, 119, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]), templates.clone())))
        } else {
            {
                let source_ml_source1_temporary_values186 = (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, source_ml_source2_data_property_value17571, _, _) => {
                        ((source_ml_source2_data_property_value17571.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptErrorArrowTemplatePlan>::new())));
                {
                    let source_ml_source1_temporary_index186 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s17572 = self::source_ml_source1_temporary_compile_java_script_error_module_templates_loop197(plan.clone(), registry.clone(), bindings.clone(), capabilities.clone(), templates.clone(), source_ml_source1_temporary_values186.clone(), source_ml_source1_temporary_index186.clone());
                        match source_ml_source2_s17572.clone() {
                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Done => {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field17573 = self::order_java_script_error_call_templates(templates.clone());
                                    (self::ml_string::Text::from_units(vec![]), source_ml_source2_field17573.clone())
                                }))
                            }
                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(source_ml_source1_temporary_result197) => {
                                source_ml_source1_temporary_result197.clone()
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
pub fn lower_java_script_error_function_body(definition: self::JavaScriptErrorFunctionPlan, context: self::JavaScriptErrorSourceContext) -> self::JavaScriptErrorSourceExpression {
    match definition.clone() {
        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, _, _, _) => {
            self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 111, 117, 114, 99, 101, 32, 98, 111, 100, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 108, 97, 110])))
        }
        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, _, definition_body, abrupt) => {
            let definition_body = *definition_body;
            {
                let body = self::lower_java_script_error_expression(definition_body.clone(), context.clone(), 0f64);
                if abrupt {
                    {
                        let source_ml_source2_nominal_field17598 = match body.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17599, _) => {
                                let source_ml_source2_data_property_value17599 = *source_ml_source2_data_property_value17599;
                                source_ml_source2_data_property_value17599.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field17600 = match body.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17601) => {
                                    let source_ml_source2_data_property_value17601 = *source_ml_source2_data_property_value17601;
                                    source_ml_source2_data_property_value17601.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_override17602 = self::JavaScriptCallSource::SourceThrow(Box::new(match body.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17596, _) => {
                                        let source_ml_source2_data_property_value17596 = *source_ml_source2_data_property_value17596;
                                        source_ml_source2_data_property_value17596.clone()
                                    }
                                }));
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override17602.clone()), Box::new(source_ml_source2_nominal_field17600.clone()))
                            }
                        }
                    }
                } else {
                    body.clone()
                }
            }
        }
        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, _, statements) => {
            self::lower_java_script_source_statements(statements.clone(), 0f64, context.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), 0f64, true)
        }
        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, _, statements_2) => {
            self::lower_java_script_source_statements(statements_2.clone(), 0f64, context.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), 0f64, true)
        }
        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, _, statements_3) => {
            self::lower_java_script_source_statements(statements_3.clone(), 0f64, context.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), 0f64, true)
        }
        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, _, statements_4) => {
            self::lower_java_script_source_statements(statements_4.clone(), 0f64, context.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), 0f64, true)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn order_java_script_error_call_templates(templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>> {
    {
        let ordered = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value329 = templates.borrow().clone(); ml_reference_value329 }, 0f64, f64::INFINITY)));
        {
            let width = 1f64;
            {
                let source_ml_source2_s17603 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop201(templates.clone(), ordered.clone(), width);
                match source_ml_source2_s17603.clone() {
                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Done(ordered_2) => {
                        ordered_2.clone()
                    }
                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Return(source_ml_source1_temporary_result201) => {
                        source_ml_source1_temporary_result201.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_reference(target: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptSourceReference {
    match target.clone() {
        self::JavaScriptErrorExpression::ErrorBinding(identity, immutable) => {
            {
                let kind = if match ((immutable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                    self::ml_string::Text::from_units(vec![105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 98, 105, 110, 100, 105, 110, 103])
                } else {
                    self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103])
                };
                self::JavaScriptSourceReference::SourceReference(kind.clone(), identity, Box::new(self::JavaScriptCallSource::SourceBinding(identity)), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceReferenceCapture>::new())), Box::new(context.clone()))
            }
        }
        self::JavaScriptErrorExpression::ErrorGet(object, key) => {
            let object = *object;
            let key = *key;
            self::lower_java_script_source_property_reference(object.clone(), key.clone(), context.clone(), depth)
        }
        self::JavaScriptErrorExpression::ErrorComputedGet(object_2, key_2) => {
            let object_2 = *object_2;
            let key_2 = *key_2;
            self::lower_java_script_source_property_reference(object_2.clone(), key_2.clone(), context.clone(), depth)
        }
        self::JavaScriptErrorExpression::ErrorModuleCell(identity_2) => {
            {
                let kind_2 = if { let ml_values330 = { let ml_reference_value331 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17676) => {
                        ((source_ml_source2_data_property_value17676.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value331 }; let ml_callback330 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17678, _, _, _, _) => {
                        source_ml_source2_data_property_value17678
                    }
                }) == identity_2) }) }; ml_values330.iter().cloned().enumerate().any(|(ml_index330, ml_value330)| ml_callback330(ml_value330)) } {
                    self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108])
                } else {
                    self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121])
                };
                self::JavaScriptSourceReference::SourceReference(kind_2.clone(), identity_2, Box::new(self::construct_java_script_lexical_cell_read(context.clone(), identity_2)), Box::new(self::JavaScriptCallSource::SourceBinding(identity_2)), Box::new(self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceReferenceCapture>::new())), Box::new(context.clone()))
            }
        }
        _ => {
            self::refuse_java_script_source_reference(context.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_source_reference(context: self::JavaScriptErrorSourceContext) -> self::JavaScriptSourceReference {
    {
        let read = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
        {
            let receiver = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
            {
                let key = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
                self::JavaScriptSourceReference::SourceReference(self::ml_string::Text::from_units(vec![]), 0f64, Box::new(read.clone()), Box::new(receiver.clone()), Box::new(key.clone()), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceReferenceCapture>::new())), Box::new({
                    let source_ml_source2_nominal_field17680 = match context.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17681, _, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value17681 = *source_ml_source2_data_property_value17681;
                            source_ml_source2_data_property_value17681.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field17682 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17683, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value17683 = *source_ml_source2_data_property_value17683;
                                source_ml_source2_data_property_value17683.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field17684 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17685, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17685.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field17686 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17687, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value17687
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field17688 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17689, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value17689
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field17690 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17691, _, _, _, _) => {
                                                source_ml_source2_data_property_value17691.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field17692 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17693, _, _, _) => {
                                                    source_ml_source2_data_property_value17693
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17694 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17695, _, _) => {
                                                        source_ml_source2_data_property_value17695.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17696 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17697, _) => {
                                                            source_ml_source2_data_property_value17697.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17698 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17699) => {
                                                                source_ml_source2_data_property_value17699.clone()
                                                            }
                                                        };
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17680.clone()), Box::new(source_ml_source2_nominal_field17682.clone()), source_ml_source2_nominal_field17684.clone(), source_ml_source2_nominal_field17686, source_ml_source2_nominal_field17688, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 100, 47, 109, 111, 100, 105, 102, 121, 47, 119, 114, 105, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 44, 32, 109, 111, 100, 117, 108, 101, 45, 99, 101, 108, 108, 32, 111, 114, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]), source_ml_source2_nominal_field17692, source_ml_source2_nominal_field17694.clone(), source_ml_source2_nominal_field17696.clone(), source_ml_source2_nominal_field17698.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_property_reference(object_expression: self::JavaScriptErrorExpression, key_expression: self::JavaScriptErrorExpression, context: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptSourceReference {
    {
        let constant = std::rc::Rc::new(self::construct_java_script_error_constant);
        {
            let object = self::lower_java_script_error_expression(object_expression.clone(), context.clone(), (depth + 1f64));
            {
                let property = self::lower_java_script_error_expression(key_expression.clone(), match object.clone() {
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17701) => {
                        let source_ml_source2_data_property_value17701 = *source_ml_source2_data_property_value17701;
                        source_ml_source2_data_property_value17701.clone()
                    }
                }, (depth + 1f64));
                {
                    let receiver_id = match match property.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17702) => {
                            let source_ml_source2_data_property_value17702 = *source_ml_source2_data_property_value17702;
                            source_ml_source2_data_property_value17702.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17703, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17703
                        }
                    };
                    {
                        let key_id = (receiver_id + 1f64);
                        {
                            let converted_id = (key_id + 1f64);
                            {
                                let current = {
                                    let source_ml_source2_nominal_spread17705 = match property.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17704) => {
                                            let source_ml_source2_data_property_value17704 = *source_ml_source2_data_property_value17704;
                                            source_ml_source2_data_property_value17704.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field17706 = match source_ml_source2_nominal_spread17705.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17707, _, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value17707 = *source_ml_source2_data_property_value17707;
                                                source_ml_source2_data_property_value17707.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field17708 = match source_ml_source2_nominal_spread17705.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17709, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value17709 = *source_ml_source2_data_property_value17709;
                                                    source_ml_source2_data_property_value17709.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17710 = match source_ml_source2_nominal_spread17705.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17711, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value17711.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17712 = match source_ml_source2_nominal_spread17705.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17713, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value17713
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17714 = match source_ml_source2_nominal_spread17705.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17715, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17715
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17716 = match source_ml_source2_nominal_spread17705.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17717, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17717.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17718 = match source_ml_source2_nominal_spread17705.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17719, _, _, _) => {
                                                                        source_ml_source2_data_property_value17719
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17720 = match source_ml_source2_nominal_spread17705.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17721, _, _) => {
                                                                            source_ml_source2_data_property_value17721.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17722 = match source_ml_source2_nominal_spread17705.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17723, _) => {
                                                                                source_ml_source2_data_property_value17723.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17724 = match source_ml_source2_nominal_spread17705.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17725) => {
                                                                                    source_ml_source2_data_property_value17725.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override17726 = (converted_id + 1f64);
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17706.clone()), Box::new(source_ml_source2_nominal_field17708.clone()), source_ml_source2_nominal_field17710.clone(), source_ml_source2_nominal_override17726, source_ml_source2_nominal_field17714, source_ml_source2_nominal_field17716.clone(), source_ml_source2_nominal_field17718, source_ml_source2_nominal_field17720.clone(), source_ml_source2_nominal_field17722.clone(), source_ml_source2_nominal_field17724.clone())
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
                                    let receiver = self::JavaScriptCallSource::SourceBinding(receiver_id);
                                    {
                                        let raw_key = self::JavaScriptCallSource::SourceBinding(key_id);
                                        {
                                            let key = self::JavaScriptCallSource::SourceBinding(converted_id);
                                            {
                                                let failure = self::JavaScriptCallSource::SourceCall(Box::new((constant.clone())(match match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17727, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17727 = *source_ml_source2_data_property_value17727;
                                                        source_ml_source2_data_property_value17727.clone()
                                                    }
                                                } {
                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17728, _) => {
                                                        let source_ml_source2_data_property_value17728 = *source_ml_source2_data_property_value17728;
                                                        source_ml_source2_data_property_value17728.clone()
                                                    }
                                                })), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![raw_key.clone()])));
                                                {
                                                    let conversion = self::JavaScriptCallSource::SourcePropertyKey(Box::new(raw_key.clone()), match match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17729, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value17729 = *source_ml_source2_data_property_value17729;
                                                            source_ml_source2_data_property_value17729.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17730, _, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value17730.clone()
                                                        }
                                                    });
                                                    {
                                                        let checked = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Null)))), Box::new(failure.clone()), Box::new(self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)))), Box::new(failure.clone()), Box::new(conversion.clone()))));
                                                        {
                                                            let captures = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(receiver_id, Box::new(match object.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17731, _) => {
                                                                    let source_ml_source2_data_property_value17731 = *source_ml_source2_data_property_value17731;
                                                                    source_ml_source2_data_property_value17731.clone()
                                                                }
                                                            })), self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(key_id, Box::new(match property.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17732, _) => {
                                                                    let source_ml_source2_data_property_value17732 = *source_ml_source2_data_property_value17732;
                                                                    source_ml_source2_data_property_value17732.clone()
                                                                }
                                                            })), self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(converted_id, Box::new(checked.clone()))]));
                                                            self::JavaScriptSourceReference::SourceReference(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121]), 0f64, Box::new(self::construct_java_script_error_property_read(receiver.clone(), key.clone(), current.clone())), Box::new(receiver.clone()), Box::new(key.clone()), captures.clone(), Box::new(current.clone()))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_source_reference_write(reference: self::JavaScriptSourceReference, value: self::JavaScriptCallSource) -> self::JavaScriptCallSource {
    if ((match reference.clone() {
        self::JavaScriptSourceReference::SourceReference(source_ml_source2_data_property_value17734, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value17734.clone()
        }
    }) == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108]))) {
        {
            let binding = { let ml_values332 = { let ml_reference_value333 = ((match match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17735) => {
                    let source_ml_source2_data_property_value17735 = *source_ml_source2_data_property_value17735;
                    source_ml_source2_data_property_value17735.clone()
                }
            } {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17736) => {
                    ((source_ml_source2_data_property_value17736.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                }
            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value333 }; let ml_callback332 = { let reference = reference.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17739, _, _, _, _) => {
                    source_ml_source2_data_property_value17739
                }
            }) == (match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, source_ml_source2_data_property_value17740, _, _, _, _, _) => {
                    source_ml_source2_data_property_value17740
                }
            })) }) }; ml_values332.iter().cloned().enumerate().find_map(|(ml_index332, ml_value332)| if ml_callback332(ml_value332.clone()) { Some(ml_value332) } else { None }) };
            self::JavaScriptCallSource::SourceIf(Box::new({
                let source_ml_source2_constructor_field17742 = match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17741, _, _, _) => {
                        let source_ml_source2_data_property_value17741 = *source_ml_source2_data_property_value17741;
                        source_ml_source2_data_property_value17741.clone()
                    }
                };
                {
                    let source_ml_source2_constructor_field17743 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100])));
                    self::JavaScriptCallSource::SourceGet(Box::new(source_ml_source2_constructor_field17742.clone()), Box::new(source_ml_source2_constructor_field17743.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
                }
            }), Box::new(if (!(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match binding.clone().expect("a proven present value") {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value17744, _, _) => {
                    source_ml_source2_data_property_value17744
                }
            }) {
                self::JavaScriptCallSource::SourceImmutableBindingWrite(Box::new(value.clone()))
            } else {
                self::JavaScriptCallSource::SourceSnapshotSet(Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17745, _, _, _) => {
                        let source_ml_source2_data_property_value17745 = *source_ml_source2_data_property_value17745;
                        source_ml_source2_data_property_value17745.clone()
                    }
                }), Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, source_ml_source2_data_property_value17746, _, _) => {
                        let source_ml_source2_data_property_value17746 = *source_ml_source2_data_property_value17746;
                        source_ml_source2_data_property_value17746.clone()
                    }
                }), Box::new(value.clone()))
            }), Box::new(self::construct_java_script_lexical_cell_failure(match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17747) => {
                    let source_ml_source2_data_property_value17747 = *source_ml_source2_data_property_value17747;
                    source_ml_source2_data_property_value17747.clone()
                }
            }, match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, source_ml_source2_data_property_value17748, _, _, _, _, _) => {
                    source_ml_source2_data_property_value17748
                }
            })))
        }
    } else {
        if ((match reference.clone() {
            self::JavaScriptSourceReference::SourceReference(source_ml_source2_data_property_value17750, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value17750.clone()
            }
        }) == (self::ml_string::Text::from_units(vec![105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 98, 105, 110, 100, 105, 110, 103]))) {
            self::JavaScriptCallSource::SourceImmutableBindingWrite(Box::new(value.clone()))
        } else {
            if ((match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(source_ml_source2_data_property_value17752, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value17752.clone()
                }
            }) == (self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103]))) {
                self::JavaScriptCallSource::SourceAssignBinding(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, source_ml_source2_data_property_value17753, _, _, _, _, _) => {
                        source_ml_source2_data_property_value17753
                    }
                }, Box::new(value.clone()))
            } else {
                self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17754, _, _, _) => {
                        let source_ml_source2_data_property_value17754 = *source_ml_source2_data_property_value17754;
                        source_ml_source2_data_property_value17754.clone()
                    }
                }])))), Box::new(self::JavaScriptCallSource::SourceSnapshotSet(Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17755, _, _, _) => {
                        let source_ml_source2_data_property_value17755 = *source_ml_source2_data_property_value17755;
                        source_ml_source2_data_property_value17755.clone()
                    }
                }), Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, source_ml_source2_data_property_value17756, _, _) => {
                        let source_ml_source2_data_property_value17756 = *source_ml_source2_data_property_value17756;
                        source_ml_source2_data_property_value17756.clone()
                    }
                }), Box::new(value.clone()))), Box::new(self::JavaScriptCallSource::SourceCall(Box::new(self::construct_java_script_error_constant(match match match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17757) => {
                        let source_ml_source2_data_property_value17757 = *source_ml_source2_data_property_value17757;
                        source_ml_source2_data_property_value17757.clone()
                    }
                } {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17758, _, _, _, _, _, _, _, _) => {
                        let source_ml_source2_data_property_value17758 = *source_ml_source2_data_property_value17758;
                        source_ml_source2_data_property_value17758.clone()
                    }
                } {
                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17759) => {
                        let source_ml_source2_data_property_value17759 = *source_ml_source2_data_property_value17759;
                        source_ml_source2_data_property_value17759.clone()
                    }
                })), Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17760, _, _, _) => {
                        let source_ml_source2_data_property_value17760 = *source_ml_source2_data_property_value17760;
                        source_ml_source2_data_property_value17760.clone()
                    }
                }), std::rc::Rc::new(std::cell::RefCell::new(vec![match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, source_ml_source2_data_property_value17761, _, _) => {
                        let source_ml_source2_data_property_value17761 = *source_ml_source2_data_property_value17761;
                        source_ml_source2_data_property_value17761.clone()
                    }
                }, value.clone()])))))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_source_reference_captures(reference: self::JavaScriptSourceReference, context: self::JavaScriptErrorSourceContext, body: self::JavaScriptCallSource) -> self::JavaScriptCallSource {
    {
        let source = body.clone();
        {
            let at = ({ let ml_reference_array334 = match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, source_ml_source2_data_property_value17762, _) => {
                    source_ml_source2_data_property_value17762.clone()
                }
            }; let ml_reference_projection334 = ml_reference_array334.borrow().len() as f64; ml_reference_projection334 } - 1f64);
            {
                let source_2 = self::source_ml_source1_temporary_construct_java_script_source_reference_captures_loop210(reference.clone(), context.clone(), source.clone(), at);
                source_2.clone()
            }
        }
    }
}
