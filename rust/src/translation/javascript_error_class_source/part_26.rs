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
            self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, _, source_ml_source2_data_property_value17481) => {
                ((source_ml_source2_data_property_value17481.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())));
        if ((({ let ml_reference_array314 = lexical.clone(); let ml_reference_projection314 = ml_reference_array314.borrow().len() as f64; ml_reference_projection314 }) > (4096f64)) || { let ml_values315 = { let ml_reference_value316 = lexical.borrow().clone(); ml_reference_value316 }; let ml_callback315 = { let lexical = lexical.clone(); let plan = plan.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding, index: f64| -> bool { {
            let prior = if (index > (0f64)) {
                {
                    let source_ml_source2_array_read_value17482 = lexical.clone();
                    {
                        let source_ml_source2_array_read_index17483 = (index - 1f64);
                        { let ml_reference_array317 = source_ml_source2_array_read_value17482.clone(); let ml_reference_position317 = source_ml_source2_array_read_index17483; let ml_reference_projection317 = self::ml_array::optional_at(&*ml_reference_array317.borrow(), self::ml_array::number_index(ml_reference_position317)); ml_reference_projection317 }
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
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17489, _, _, _, _) => {
                    source_ml_source2_data_property_value17489
                }
            }))) || {
                let source_ml_source2_same_value_operand17491 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17490, _, _, _, _) => {
                        source_ml_source2_data_property_value17490
                    }
                };
                {
                    let source_ml_source2_same_value_operand17492 = (-0f64);
                    (((source_ml_source2_same_value_operand17491 == source_ml_source2_same_value_operand17492) && ((source_ml_source2_same_value_operand17491 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17491)) == ((1f64 / source_ml_source2_same_value_operand17492))))) || ((source_ml_source2_same_value_operand17491 != source_ml_source2_same_value_operand17491) && (source_ml_source2_same_value_operand17492 != source_ml_source2_same_value_operand17492)))
                }
            }) || ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17494, _, _, _, _) => {
                    source_ml_source2_data_property_value17494
                }
            }) < (0f64))) || ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17497, _, _, _, _) => {
                    source_ml_source2_data_property_value17497
                }
            }) >= (match plan.clone() {
                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17498, _, _, _) => {
                    source_ml_source2_data_property_value17498
                }
            }))) || ((index > (0f64)) && ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17501, _, _, _, _) => {
                    source_ml_source2_data_property_value17501
                }
            }) <= (if match (prior.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                (-1f64)
            } else {
                match prior.clone().expect("a proven present value") {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17502, _, _, _, _) => {
                        source_ml_source2_data_property_value17502
                    }
                }
            })))) || (({
                let source_ml_source2_type_query_operand17506 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value17505, _, _, _) => {
                        source_ml_source2_data_property_value17505.clone()
                    }
                };
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value17508, _, _, _) => {
                    source_ml_source2_data_property_value17508.clone()
                }
            }).length()) == (0f64))) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value17510, _, _, _) => {
                    source_ml_source2_data_property_value17510.clone()
                }
            }).length()) > (4096f64))) || (({
                let source_ml_source2_type_query_operand17514 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value17513, _, _) => {
                        source_ml_source2_data_property_value17513
                    }
                };
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || ({
                let source_ml_source2_comparison_operand17516 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17515, _) => {
                        ((source_ml_source2_data_property_value17515.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17517 = None::<()>;
                    !(match (source_ml_source2_comparison_operand17516.clone(), source_ml_source2_comparison_operand17517.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                }
            } && (({
                let source_ml_source2_type_query_operand17521 = (match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17520, _) => {
                        ((source_ml_source2_data_property_value17520.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                }).clone().expect("a proven present value");
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))))) || ({
                let source_ml_source2_comparison_operand17523 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17522, _) => {
                        ((source_ml_source2_data_property_value17522.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17524 = true;
                    match (source_ml_source2_comparison_operand17523.clone(), source_ml_source2_comparison_operand17524) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }
            } && match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value17525, _, _) => {
                    source_ml_source2_data_property_value17525
                }
            })) || ({
                let source_ml_source2_comparison_operand17527 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17526) => {
                        ((source_ml_source2_data_property_value17526.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17528 = None::<()>;
                    !(match (source_ml_source2_comparison_operand17527.clone(), source_ml_source2_comparison_operand17528.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                }
            } && ((((!(self::ml_math::is_safe_integer((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17529) => {
                    ((source_ml_source2_data_property_value17529.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                }
            }).clone().expect("a proven present value"))) || {
                let source_ml_source2_same_value_operand17531 = (match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17530) => {
                        ((source_ml_source2_data_property_value17530.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                    }
                }).clone().expect("a proven present value");
                {
                    let source_ml_source2_same_value_operand17532 = (-0f64);
                    (((source_ml_source2_same_value_operand17531 == source_ml_source2_same_value_operand17532) && ((source_ml_source2_same_value_operand17531 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17531)) == ((1f64 / source_ml_source2_same_value_operand17532))))) || ((source_ml_source2_same_value_operand17531 != source_ml_source2_same_value_operand17531) && (source_ml_source2_same_value_operand17532 != source_ml_source2_same_value_operand17532)))
                }
            }) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17534) => {
                    ((source_ml_source2_data_property_value17534.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                }
            }).clone().expect("a proven present value")) < (0f64))) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17537) => {
                    ((source_ml_source2_data_property_value17537.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                }
            }).clone().expect("a proven present value")) >= (match plan.clone() {
                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17538, _, _, _) => {
                    source_ml_source2_data_property_value17538
                }
            }))) || {
                let source_ml_source2_comparison_operand17540 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17539, _) => {
                        ((source_ml_source2_data_property_value17539.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17541 = true;
                    !(match (source_ml_source2_comparison_operand17540.clone(), source_ml_source2_comparison_operand17541) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            })))
        } }) }; ml_values315.iter().cloned().enumerate().any(|(ml_index315, ml_value315)| ml_callback315(ml_value315, (ml_index315 as f64))) }) {
            self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 110, 97, 109, 101, 115, 32, 97, 110, 100, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 102, 108, 97, 103, 115])
        } else {
            {
                let shared = (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, source_ml_source2_data_property_value17542, _) => {
                        ((source_ml_source2_data_property_value17542.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                if ((({ let ml_reference_array318 = shared.clone(); let ml_reference_projection318 = ml_reference_array318.borrow().len() as f64; ml_reference_projection318 }) > (4096f64)) || { let ml_values319 = { let ml_reference_value320 = shared.borrow().clone(); ml_reference_value320 }; let ml_callback319 = { let plan = plan.clone(); let shared = shared.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { ((((!(self::ml_math::is_safe_integer(identity)) || {
                    let source_ml_source2_same_value_operand17543 = identity;
                    {
                        let source_ml_source2_same_value_operand17544 = (-0f64);
                        (((source_ml_source2_same_value_operand17543 == source_ml_source2_same_value_operand17544) && ((source_ml_source2_same_value_operand17543 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17543)) == ((1f64 / source_ml_source2_same_value_operand17544))))) || ((source_ml_source2_same_value_operand17543 != source_ml_source2_same_value_operand17543) && (source_ml_source2_same_value_operand17544 != source_ml_source2_same_value_operand17544)))
                    }
                }) || (identity < (0f64))) || (identity >= (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17546, _, _, _) => {
                        source_ml_source2_data_property_value17546
                    }
                }))) || ((index > (0f64)) && (identity <= (({
                    let source_ml_source2_array_read_value17549 = shared.clone();
                    {
                        let source_ml_source2_array_read_index17550 = (index - 1f64);
                        { let ml_reference_array321 = source_ml_source2_array_read_value17549.clone(); let ml_reference_position321 = source_ml_source2_array_read_index17550; let ml_reference_projection321 = self::ml_array::optional_at(&*ml_reference_array321.borrow(), self::ml_array::number_index(ml_reference_position321)); ml_reference_projection321 }
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
                                let source_ml_source2_s17551 = self::source_ml_source1_temporary_read_java_script_error_module_plan_error_loop184(registry.clone(), capabilities.clone(), at);
                                match source_ml_source2_s17551.clone() {
                                    self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Done => {
                                        if (({ let ml_reference_array323 = bindings.clone(); let ml_reference_projection323 = ml_reference_array323.borrow().len() as f64; ml_reference_projection323 }) > (4096f64)) {
                                            self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 109, 111, 100, 117, 108, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101, 32, 98, 117, 100, 103, 101, 116])
                                        } else {
                                            {
                                                let at_2 = 0f64;
                                                {
                                                    let source_ml_source2_s17552 = self::source_ml_source1_temporary_read_java_script_error_module_plan_error_loop185(plan.clone(), bindings.clone(), at_2);
                                                    match source_ml_source2_s17552.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Done => {
                                                            if (((((({ let ml_reference_array324 = match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value17554, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17554.clone()
                                                                }
                                                            }; let ml_reference_projection324 = ml_reference_array324.borrow().len() as f64; ml_reference_projection324 }) > (4096f64)) || (({ let ml_reference_array325 = match registry.clone() {
                                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value17557, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17557.clone()
                                                                }
                                                            }; let ml_reference_projection325 = ml_reference_array325.borrow().len() as f64; ml_reference_projection325 }) != ({ let ml_reference_array326 = match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value17558, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17558.clone()
                                                                }
                                                            }; let ml_reference_projection326 = ml_reference_array326.borrow().len() as f64; ml_reference_projection326 }))) || (({ let ml_reference_array327 = match registry.clone() {
                                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, source_ml_source2_data_property_value17560, _, _) => {
                                                                    source_ml_source2_data_property_value17560.clone()
                                                                }
                                                            }; let ml_reference_projection327 = ml_reference_array327.borrow().len() as f64; ml_reference_projection327 }) > (4096f64))) || ((match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17562, _, _, _) => {
                                                                    source_ml_source2_data_property_value17562
                                                                }
                                                            }) < (0f64))) || !(self::ml_math::is_safe_integer(match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17563, _, _, _) => {
                                                                    source_ml_source2_data_property_value17563
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
            self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, source_ml_source2_data_property_value17565, _, _) => {
                ((source_ml_source2_data_property_value17565.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptErrorArrowTemplatePlan>::new()))); let ml_reference_projection328 = ml_reference_array328.borrow().len() as f64; ml_reference_projection328 }) > (4096f64)) {
            std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 97, 114, 114, 111, 119, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]), templates.clone())))
        } else {
            {
                let source_ml_source1_temporary_values186 = (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, source_ml_source2_data_property_value17568, _, _) => {
                        ((source_ml_source2_data_property_value17568.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptErrorArrowTemplatePlan>::new())));
                {
                    let source_ml_source1_temporary_index186 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s17569 = self::source_ml_source1_temporary_compile_java_script_error_module_templates_loop197(plan.clone(), registry.clone(), bindings.clone(), capabilities.clone(), templates.clone(), source_ml_source1_temporary_values186.clone(), source_ml_source1_temporary_index186.clone());
                        match source_ml_source2_s17569.clone() {
                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Done => {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field17570 = self::order_java_script_error_call_templates(templates.clone());
                                    (self::ml_string::Text::from_units(vec![]), source_ml_source2_field17570.clone())
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
                        let source_ml_source2_nominal_field17595 = match body.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17596, _) => {
                                let source_ml_source2_data_property_value17596 = *source_ml_source2_data_property_value17596;
                                source_ml_source2_data_property_value17596.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field17597 = match body.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17598) => {
                                    let source_ml_source2_data_property_value17598 = *source_ml_source2_data_property_value17598;
                                    source_ml_source2_data_property_value17598.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_override17599 = self::JavaScriptCallSource::SourceThrow(Box::new(match body.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17593, _) => {
                                        let source_ml_source2_data_property_value17593 = *source_ml_source2_data_property_value17593;
                                        source_ml_source2_data_property_value17593.clone()
                                    }
                                }));
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override17599.clone()), Box::new(source_ml_source2_nominal_field17597.clone()))
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
                let source_ml_source2_s17600 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop201(templates.clone(), ordered.clone(), width);
                match source_ml_source2_s17600.clone() {
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
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17673) => {
                        ((source_ml_source2_data_property_value17673.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value331 }; let ml_callback330 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17675, _, _, _, _) => {
                        source_ml_source2_data_property_value17675
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
                    let source_ml_source2_nominal_field17677 = match context.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17678, _, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value17678 = *source_ml_source2_data_property_value17678;
                            source_ml_source2_data_property_value17678.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field17679 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17680, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value17680 = *source_ml_source2_data_property_value17680;
                                source_ml_source2_data_property_value17680.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field17681 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17682, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17682.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field17683 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17684, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value17684
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field17685 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17686, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value17686
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field17687 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17688, _, _, _, _) => {
                                                source_ml_source2_data_property_value17688.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field17689 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17690, _, _, _) => {
                                                    source_ml_source2_data_property_value17690
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17691 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17692, _, _) => {
                                                        source_ml_source2_data_property_value17692.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17693 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17694, _) => {
                                                            source_ml_source2_data_property_value17694.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17695 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17696) => {
                                                                source_ml_source2_data_property_value17696.clone()
                                                            }
                                                        };
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17677.clone()), Box::new(source_ml_source2_nominal_field17679.clone()), source_ml_source2_nominal_field17681.clone(), source_ml_source2_nominal_field17683, source_ml_source2_nominal_field17685, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 100, 47, 109, 111, 100, 105, 102, 121, 47, 119, 114, 105, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 44, 32, 109, 111, 100, 117, 108, 101, 45, 99, 101, 108, 108, 32, 111, 114, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]), source_ml_source2_nominal_field17689, source_ml_source2_nominal_field17691.clone(), source_ml_source2_nominal_field17693.clone(), source_ml_source2_nominal_field17695.clone())
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
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17698) => {
                        let source_ml_source2_data_property_value17698 = *source_ml_source2_data_property_value17698;
                        source_ml_source2_data_property_value17698.clone()
                    }
                }, (depth + 1f64));
                {
                    let receiver_id = match match property.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17699) => {
                            let source_ml_source2_data_property_value17699 = *source_ml_source2_data_property_value17699;
                            source_ml_source2_data_property_value17699.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17700, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17700
                        }
                    };
                    {
                        let key_id = (receiver_id + 1f64);
                        {
                            let converted_id = (key_id + 1f64);
                            {
                                let current = {
                                    let source_ml_source2_nominal_spread17702 = match property.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17701) => {
                                            let source_ml_source2_data_property_value17701 = *source_ml_source2_data_property_value17701;
                                            source_ml_source2_data_property_value17701.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field17703 = match source_ml_source2_nominal_spread17702.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17704, _, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value17704 = *source_ml_source2_data_property_value17704;
                                                source_ml_source2_data_property_value17704.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field17705 = match source_ml_source2_nominal_spread17702.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17706, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value17706 = *source_ml_source2_data_property_value17706;
                                                    source_ml_source2_data_property_value17706.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17707 = match source_ml_source2_nominal_spread17702.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17708, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value17708.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17709 = match source_ml_source2_nominal_spread17702.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17710, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value17710
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17711 = match source_ml_source2_nominal_spread17702.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17712, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17712
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17713 = match source_ml_source2_nominal_spread17702.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17714, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17714.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17715 = match source_ml_source2_nominal_spread17702.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17716, _, _, _) => {
                                                                        source_ml_source2_data_property_value17716
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17717 = match source_ml_source2_nominal_spread17702.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17718, _, _) => {
                                                                            source_ml_source2_data_property_value17718.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17719 = match source_ml_source2_nominal_spread17702.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17720, _) => {
                                                                                source_ml_source2_data_property_value17720.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17721 = match source_ml_source2_nominal_spread17702.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17722) => {
                                                                                    source_ml_source2_data_property_value17722.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override17723 = (converted_id + 1f64);
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17703.clone()), Box::new(source_ml_source2_nominal_field17705.clone()), source_ml_source2_nominal_field17707.clone(), source_ml_source2_nominal_override17723, source_ml_source2_nominal_field17711, source_ml_source2_nominal_field17713.clone(), source_ml_source2_nominal_field17715, source_ml_source2_nominal_field17717.clone(), source_ml_source2_nominal_field17719.clone(), source_ml_source2_nominal_field17721.clone())
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
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17724, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17724 = *source_ml_source2_data_property_value17724;
                                                        source_ml_source2_data_property_value17724.clone()
                                                    }
                                                } {
                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17725, _) => {
                                                        let source_ml_source2_data_property_value17725 = *source_ml_source2_data_property_value17725;
                                                        source_ml_source2_data_property_value17725.clone()
                                                    }
                                                })), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![raw_key.clone()])));
                                                {
                                                    let conversion = self::JavaScriptCallSource::SourcePropertyKey(Box::new(raw_key.clone()), match match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17726, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value17726 = *source_ml_source2_data_property_value17726;
                                                            source_ml_source2_data_property_value17726.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17727, _, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value17727.clone()
                                                        }
                                                    });
                                                    {
                                                        let checked = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Null)))), Box::new(failure.clone()), Box::new(self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)))), Box::new(failure.clone()), Box::new(conversion.clone()))));
                                                        {
                                                            let captures = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(receiver_id, Box::new(match object.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17728, _) => {
                                                                    let source_ml_source2_data_property_value17728 = *source_ml_source2_data_property_value17728;
                                                                    source_ml_source2_data_property_value17728.clone()
                                                                }
                                                            })), self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(key_id, Box::new(match property.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17729, _) => {
                                                                    let source_ml_source2_data_property_value17729 = *source_ml_source2_data_property_value17729;
                                                                    source_ml_source2_data_property_value17729.clone()
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
        self::JavaScriptSourceReference::SourceReference(source_ml_source2_data_property_value17731, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value17731.clone()
        }
    }) == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108]))) {
        {
            let binding = { let ml_values332 = { let ml_reference_value333 = ((match match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17732) => {
                    let source_ml_source2_data_property_value17732 = *source_ml_source2_data_property_value17732;
                    source_ml_source2_data_property_value17732.clone()
                }
            } {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17733) => {
                    ((source_ml_source2_data_property_value17733.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                }
            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value333 }; let ml_callback332 = { let reference = reference.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17736, _, _, _, _) => {
                    source_ml_source2_data_property_value17736
                }
            }) == (match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, source_ml_source2_data_property_value17737, _, _, _, _, _) => {
                    source_ml_source2_data_property_value17737
                }
            })) }) }; ml_values332.iter().cloned().enumerate().find_map(|(ml_index332, ml_value332)| if ml_callback332(ml_value332.clone()) { Some(ml_value332) } else { None }) };
            self::JavaScriptCallSource::SourceIf(Box::new({
                let source_ml_source2_constructor_field17739 = match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17738, _, _, _) => {
                        let source_ml_source2_data_property_value17738 = *source_ml_source2_data_property_value17738;
                        source_ml_source2_data_property_value17738.clone()
                    }
                };
                {
                    let source_ml_source2_constructor_field17740 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100])));
                    self::JavaScriptCallSource::SourceGet(Box::new(source_ml_source2_constructor_field17739.clone()), Box::new(source_ml_source2_constructor_field17740.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
                }
            }), Box::new(if (!(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match binding.clone().expect("a proven present value") {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value17741, _, _) => {
                    source_ml_source2_data_property_value17741
                }
            }) {
                self::JavaScriptCallSource::SourceImmutableBindingWrite(Box::new(value.clone()))
            } else {
                self::JavaScriptCallSource::SourceSnapshotSet(Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17742, _, _, _) => {
                        let source_ml_source2_data_property_value17742 = *source_ml_source2_data_property_value17742;
                        source_ml_source2_data_property_value17742.clone()
                    }
                }), Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, source_ml_source2_data_property_value17743, _, _) => {
                        let source_ml_source2_data_property_value17743 = *source_ml_source2_data_property_value17743;
                        source_ml_source2_data_property_value17743.clone()
                    }
                }), Box::new(value.clone()))
            }), Box::new(self::construct_java_script_lexical_cell_failure(match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17744) => {
                    let source_ml_source2_data_property_value17744 = *source_ml_source2_data_property_value17744;
                    source_ml_source2_data_property_value17744.clone()
                }
            }, match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, source_ml_source2_data_property_value17745, _, _, _, _, _) => {
                    source_ml_source2_data_property_value17745
                }
            })))
        }
    } else {
        if ((match reference.clone() {
            self::JavaScriptSourceReference::SourceReference(source_ml_source2_data_property_value17747, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value17747.clone()
            }
        }) == (self::ml_string::Text::from_units(vec![105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 98, 105, 110, 100, 105, 110, 103]))) {
            self::JavaScriptCallSource::SourceImmutableBindingWrite(Box::new(value.clone()))
        } else {
            if ((match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(source_ml_source2_data_property_value17749, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value17749.clone()
                }
            }) == (self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103]))) {
                self::JavaScriptCallSource::SourceAssignBinding(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, source_ml_source2_data_property_value17750, _, _, _, _, _) => {
                        source_ml_source2_data_property_value17750
                    }
                }, Box::new(value.clone()))
            } else {
                self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17751, _, _, _) => {
                        let source_ml_source2_data_property_value17751 = *source_ml_source2_data_property_value17751;
                        source_ml_source2_data_property_value17751.clone()
                    }
                }])))), Box::new(self::JavaScriptCallSource::SourceSnapshotSet(Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17752, _, _, _) => {
                        let source_ml_source2_data_property_value17752 = *source_ml_source2_data_property_value17752;
                        source_ml_source2_data_property_value17752.clone()
                    }
                }), Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, source_ml_source2_data_property_value17753, _, _) => {
                        let source_ml_source2_data_property_value17753 = *source_ml_source2_data_property_value17753;
                        source_ml_source2_data_property_value17753.clone()
                    }
                }), Box::new(value.clone()))), Box::new(self::JavaScriptCallSource::SourceCall(Box::new(self::construct_java_script_error_constant(match match match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17754) => {
                        let source_ml_source2_data_property_value17754 = *source_ml_source2_data_property_value17754;
                        source_ml_source2_data_property_value17754.clone()
                    }
                } {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17755, _, _, _, _, _, _, _, _) => {
                        let source_ml_source2_data_property_value17755 = *source_ml_source2_data_property_value17755;
                        source_ml_source2_data_property_value17755.clone()
                    }
                } {
                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17756) => {
                        let source_ml_source2_data_property_value17756 = *source_ml_source2_data_property_value17756;
                        source_ml_source2_data_property_value17756.clone()
                    }
                })), Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17757, _, _, _) => {
                        let source_ml_source2_data_property_value17757 = *source_ml_source2_data_property_value17757;
                        source_ml_source2_data_property_value17757.clone()
                    }
                }), std::rc::Rc::new(std::cell::RefCell::new(vec![match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, source_ml_source2_data_property_value17758, _, _) => {
                        let source_ml_source2_data_property_value17758 = *source_ml_source2_data_property_value17758;
                        source_ml_source2_data_property_value17758.clone()
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
                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, source_ml_source2_data_property_value17759, _) => {
                    source_ml_source2_data_property_value17759.clone()
                }
            }; let ml_reference_projection334 = ml_reference_array334.borrow().len() as f64; ml_reference_projection334 } - 1f64);
            {
                let source_2 = self::source_ml_source1_temporary_construct_java_script_source_reference_captures_loop210(reference.clone(), context.clone(), source.clone(), at);
                source_2.clone()
            }
        }
    }
}
