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
use super::ml_source_exception;
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
            self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, _, source_ml_source2_data_property_value17579) => {
                ((source_ml_source2_data_property_value17579.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())));
        if ((({ let ml_reference_array314 = lexical.clone(); let ml_reference_projection314 = ml_reference_array314.borrow().len() as f64; ml_reference_projection314 }) > (4096f64)) || { let ml_values315 = { let ml_reference_value316 = lexical.borrow().clone(); ml_reference_value316 }; let ml_callback315 = { let lexical = lexical.clone(); let plan = plan.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding, index: f64| -> bool { {
            let prior = if (index > (0f64)) {
                {
                    let source_ml_source2_array_read_value17580 = lexical.clone();
                    {
                        let source_ml_source2_array_read_index17581 = (index - 1f64);
                        { let ml_reference_array317 = source_ml_source2_array_read_value17580.clone(); let ml_reference_position317 = source_ml_source2_array_read_index17581; let ml_reference_projection317 = self::ml_array::optional_at(&*ml_reference_array317.borrow(), self::ml_array::number_index(ml_reference_position317)); ml_reference_projection317 }
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
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17587, _, _, _, _) => {
                    source_ml_source2_data_property_value17587
                }
            }))) || {
                let source_ml_source2_same_value_operand17589 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17588, _, _, _, _) => {
                        source_ml_source2_data_property_value17588
                    }
                };
                {
                    let source_ml_source2_same_value_operand17590 = (-0f64);
                    (((source_ml_source2_same_value_operand17589 == source_ml_source2_same_value_operand17590) && ((source_ml_source2_same_value_operand17589 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17589)) == ((1f64 / source_ml_source2_same_value_operand17590))))) || ((source_ml_source2_same_value_operand17589 != source_ml_source2_same_value_operand17589) && (source_ml_source2_same_value_operand17590 != source_ml_source2_same_value_operand17590)))
                }
            }) || ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17592, _, _, _, _) => {
                    source_ml_source2_data_property_value17592
                }
            }) < (0f64))) || ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17595, _, _, _, _) => {
                    source_ml_source2_data_property_value17595
                }
            }) >= (match plan.clone() {
                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17596, _, _, _) => {
                    source_ml_source2_data_property_value17596
                }
            }))) || ((index > (0f64)) && ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17599, _, _, _, _) => {
                    source_ml_source2_data_property_value17599
                }
            }) <= (if match (prior.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                (-1f64)
            } else {
                match prior.clone().expect("a proven present value") {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17600, _, _, _, _) => {
                        source_ml_source2_data_property_value17600
                    }
                }
            })))) || (({
                let source_ml_source2_type_query_operand17604 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value17603, _, _, _) => {
                        source_ml_source2_data_property_value17603.clone()
                    }
                };
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
            }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value17606, _, _, _) => {
                    source_ml_source2_data_property_value17606.clone()
                }
            }).length()) == (0f64))) || (((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value17608, _, _, _) => {
                    source_ml_source2_data_property_value17608.clone()
                }
            }).length()) > (4096f64))) || (({
                let source_ml_source2_type_query_operand17612 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value17611, _, _) => {
                        source_ml_source2_data_property_value17611
                    }
                };
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || ({
                let source_ml_source2_comparison_operand17614 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17613, _) => {
                        ((source_ml_source2_data_property_value17613.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17615 = None::<()>;
                    !(match (source_ml_source2_comparison_operand17614.clone(), source_ml_source2_comparison_operand17615.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                }
            } && ((match match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17617, _) => {
                    ((source_ml_source2_data_property_value17617.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                }
            } { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))))) || ({
                let source_ml_source2_comparison_operand17619 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17618, _) => {
                        ((source_ml_source2_data_property_value17618.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17620 = true;
                    match (source_ml_source2_comparison_operand17619.clone(), source_ml_source2_comparison_operand17620) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }
            } && match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value17621, _, _) => {
                    source_ml_source2_data_property_value17621
                }
            })) || ({
                let source_ml_source2_comparison_operand17623 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17622) => {
                        ((source_ml_source2_data_property_value17622.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17624 = None::<()>;
                    !(match (source_ml_source2_comparison_operand17623.clone(), source_ml_source2_comparison_operand17624.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                }
            } && ((((!(match match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17625) => {
                    ((source_ml_source2_data_property_value17625.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                }
            } { Some(ml_present) => self::ml_math::is_safe_integer(ml_present), None => false }) || {
                let source_ml_source2_same_value_operand17627 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17626) => {
                        ((source_ml_source2_data_property_value17626.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_same_value_operand17628 = (-0f64);
                    if match (source_ml_source2_same_value_operand17627.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                        match (None::<()>, source_ml_source2_same_value_operand17628) { (Some(_ml_left), _ml_right) => false, _ => false }
                    } else {
                        ((((source_ml_source2_same_value_operand17627.clone().expect("a proven present value")) == source_ml_source2_same_value_operand17628) && (((source_ml_source2_same_value_operand17627.clone().expect("a proven present value")) != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17627.clone().expect("a proven present value"))) == ((1f64 / source_ml_source2_same_value_operand17628))))) || (((source_ml_source2_same_value_operand17627.clone().expect("a proven present value")) != (source_ml_source2_same_value_operand17627.clone().expect("a proven present value"))) && (source_ml_source2_same_value_operand17628 != source_ml_source2_same_value_operand17628)))
                    }
                }
            }) || {
                let source_ml_source2_comparison_operand17630 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17629) => {
                        ((source_ml_source2_data_property_value17629.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17631 = 0f64;
                    if match (source_ml_source2_comparison_operand17630.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                        ((f64::NAN) < source_ml_source2_comparison_operand17631)
                    } else {
                        ((source_ml_source2_comparison_operand17630.clone().expect("a proven present value")) < source_ml_source2_comparison_operand17631)
                    }
                }
            }) || {
                let source_ml_source2_comparison_operand17634 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, _, source_ml_source2_data_property_value17632) => {
                        ((source_ml_source2_data_property_value17632.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17635 = match plan.clone() {
                        self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17633, _, _, _) => {
                            source_ml_source2_data_property_value17633
                        }
                    };
                    if match (source_ml_source2_comparison_operand17634.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                        ((f64::NAN) >= source_ml_source2_comparison_operand17635)
                    } else {
                        ((source_ml_source2_comparison_operand17634.clone().expect("a proven present value")) >= source_ml_source2_comparison_operand17635)
                    }
                }
            }) || {
                let source_ml_source2_comparison_operand17637 = match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value17636, _) => {
                        ((source_ml_source2_data_property_value17636.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                {
                    let source_ml_source2_comparison_operand17638 = true;
                    !(match (source_ml_source2_comparison_operand17637.clone(), source_ml_source2_comparison_operand17638) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            })))
        } }) }; ml_values315.iter().cloned().enumerate().any(|(ml_index315, ml_value315)| ml_callback315(ml_value315, (ml_index315 as f64))) }) {
            self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 110, 97, 109, 101, 115, 32, 97, 110, 100, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 102, 108, 97, 103, 115])
        } else {
            {
                let shared = (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, source_ml_source2_data_property_value17639, _) => {
                        ((source_ml_source2_data_property_value17639.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                if ((({ let ml_reference_array318 = shared.clone(); let ml_reference_projection318 = ml_reference_array318.borrow().len() as f64; ml_reference_projection318 }) > (4096f64)) || { let ml_values319 = { let ml_reference_value320 = shared.borrow().clone(); ml_reference_value320 }; let ml_callback319 = { let plan = plan.clone(); let shared = shared.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { ((((!(self::ml_math::is_safe_integer(identity)) || {
                    let source_ml_source2_same_value_operand17640 = identity;
                    {
                        let source_ml_source2_same_value_operand17641 = (-0f64);
                        (((source_ml_source2_same_value_operand17640 == source_ml_source2_same_value_operand17641) && ((source_ml_source2_same_value_operand17640 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand17640)) == ((1f64 / source_ml_source2_same_value_operand17641))))) || ((source_ml_source2_same_value_operand17640 != source_ml_source2_same_value_operand17640) && (source_ml_source2_same_value_operand17641 != source_ml_source2_same_value_operand17641)))
                    }
                }) || (identity < (0f64))) || (identity >= (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17643, _, _, _) => {
                        source_ml_source2_data_property_value17643
                    }
                }))) || ((index > (0f64)) && (identity <= (({
                    let source_ml_source2_array_read_value17646 = shared.clone();
                    {
                        let source_ml_source2_array_read_index17647 = (index - 1f64);
                        { let ml_reference_array321 = source_ml_source2_array_read_value17646.clone(); let ml_reference_position321 = source_ml_source2_array_read_index17647; let ml_reference_projection321 = self::ml_array::optional_at(&*ml_reference_array321.borrow(), self::ml_array::number_index(ml_reference_position321)); ml_reference_projection321 }
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
                                let source_ml_source2_s17648 = self::source_ml_source1_temporary_read_java_script_error_module_plan_error_loop184(registry.clone(), capabilities.clone(), at);
                                match source_ml_source2_s17648.clone() {
                                    self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Done => {
                                        if (({ let ml_reference_array323 = bindings.clone(); let ml_reference_projection323 = ml_reference_array323.borrow().len() as f64; ml_reference_projection323 }) > (4096f64)) {
                                            self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 109, 111, 100, 117, 108, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101, 32, 98, 117, 100, 103, 101, 116])
                                        } else {
                                            {
                                                let at_2 = 0f64;
                                                {
                                                    let source_ml_source2_s17649 = self::source_ml_source1_temporary_read_java_script_error_module_plan_error_loop185(plan.clone(), bindings.clone(), at_2);
                                                    match source_ml_source2_s17649.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Done => {
                                                            if (((((({ let ml_reference_array324 = match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value17651, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17651.clone()
                                                                }
                                                            }; let ml_reference_projection324 = ml_reference_array324.borrow().len() as f64; ml_reference_projection324 }) > (4096f64)) || (({ let ml_reference_array325 = match registry.clone() {
                                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value17654, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17654.clone()
                                                                }
                                                            }; let ml_reference_projection325 = ml_reference_array325.borrow().len() as f64; ml_reference_projection325 }) != ({ let ml_reference_array326 = match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value17655, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17655.clone()
                                                                }
                                                            }; let ml_reference_projection326 = ml_reference_array326.borrow().len() as f64; ml_reference_projection326 }))) || (({ let ml_reference_array327 = match registry.clone() {
                                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, source_ml_source2_data_property_value17657, _, _) => {
                                                                    source_ml_source2_data_property_value17657.clone()
                                                                }
                                                            }; let ml_reference_projection327 = ml_reference_array327.borrow().len() as f64; ml_reference_projection327 }) > (4096f64))) || ((match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17659, _, _, _) => {
                                                                    source_ml_source2_data_property_value17659
                                                                }
                                                            }) < (0f64))) || !(self::ml_math::is_safe_integer(match plan.clone() {
                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value17660, _, _, _) => {
                                                                    source_ml_source2_data_property_value17660
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
            self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, source_ml_source2_data_property_value17662, _, _) => {
                ((source_ml_source2_data_property_value17662.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptErrorArrowTemplatePlan>::new()))); let ml_reference_projection328 = ml_reference_array328.borrow().len() as f64; ml_reference_projection328 }) > (4096f64)) {
            std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 97, 114, 114, 111, 119, 32, 116, 101, 109, 112, 108, 97, 116, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]), templates.clone())))
        } else {
            {
                let source_ml_source1_temporary_values186 = (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, source_ml_source2_data_property_value17665, _, _) => {
                        ((source_ml_source2_data_property_value17665.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptErrorArrowTemplatePlan>::new())));
                {
                    let source_ml_source1_temporary_index186 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s17666 = self::source_ml_source1_temporary_compile_java_script_error_module_templates_loop197(plan.clone(), registry.clone(), bindings.clone(), capabilities.clone(), templates.clone(), source_ml_source1_temporary_values186.clone(), source_ml_source1_temporary_index186.clone());
                        match source_ml_source2_s17666.clone() {
                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Done => {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field17667 = self::order_java_script_error_call_templates(templates.clone());
                                    (self::ml_string::Text::from_units(vec![]), source_ml_source2_field17667.clone())
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
                        let source_ml_source2_nominal_field17692 = match body.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17693, _) => {
                                let source_ml_source2_data_property_value17693 = *source_ml_source2_data_property_value17693;
                                source_ml_source2_data_property_value17693.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field17694 = match body.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17695) => {
                                    let source_ml_source2_data_property_value17695 = *source_ml_source2_data_property_value17695;
                                    source_ml_source2_data_property_value17695.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_override17696 = self::JavaScriptCallSource::SourceThrow(Box::new(match body.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17690, _) => {
                                        let source_ml_source2_data_property_value17690 = *source_ml_source2_data_property_value17690;
                                        source_ml_source2_data_property_value17690.clone()
                                    }
                                }));
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_ml_source2_nominal_override17696.clone()), Box::new(source_ml_source2_nominal_field17694.clone()))
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
                let source_ml_source2_s17697 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop201(templates.clone(), ordered.clone(), width);
                match source_ml_source2_s17697.clone() {
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
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17770) => {
                        ((source_ml_source2_data_property_value17770.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value331 }; let ml_callback330 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17772, _, _, _, _) => {
                        source_ml_source2_data_property_value17772
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
                    let source_ml_source2_nominal_field17774 = match context.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17775, _, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value17775 = *source_ml_source2_data_property_value17775;
                            source_ml_source2_data_property_value17775.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field17776 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17777, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value17777 = *source_ml_source2_data_property_value17777;
                                source_ml_source2_data_property_value17777.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field17778 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17779, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17779.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field17780 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17781, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value17781
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field17782 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17783, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value17783
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field17784 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17785, _, _, _, _) => {
                                                source_ml_source2_data_property_value17785.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field17786 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17787, _, _, _) => {
                                                    source_ml_source2_data_property_value17787
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17788 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17789, _, _) => {
                                                        source_ml_source2_data_property_value17789.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17790 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17791, _) => {
                                                            source_ml_source2_data_property_value17791.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17792 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17793) => {
                                                                source_ml_source2_data_property_value17793.clone()
                                                            }
                                                        };
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17774.clone()), Box::new(source_ml_source2_nominal_field17776.clone()), source_ml_source2_nominal_field17778.clone(), source_ml_source2_nominal_field17780, source_ml_source2_nominal_field17782, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 100, 47, 109, 111, 100, 105, 102, 121, 47, 119, 114, 105, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 44, 32, 109, 111, 100, 117, 108, 101, 45, 99, 101, 108, 108, 32, 111, 114, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]), source_ml_source2_nominal_field17786, source_ml_source2_nominal_field17788.clone(), source_ml_source2_nominal_field17790.clone(), source_ml_source2_nominal_field17792.clone())
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
                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17795) => {
                        let source_ml_source2_data_property_value17795 = *source_ml_source2_data_property_value17795;
                        source_ml_source2_data_property_value17795.clone()
                    }
                }, (depth + 1f64));
                {
                    let receiver_id = match match property.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17796) => {
                            let source_ml_source2_data_property_value17796 = *source_ml_source2_data_property_value17796;
                            source_ml_source2_data_property_value17796.clone()
                        }
                    } {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17797, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value17797
                        }
                    };
                    {
                        let key_id = (receiver_id + 1f64);
                        {
                            let converted_id = (key_id + 1f64);
                            {
                                let current = {
                                    let source_ml_source2_nominal_spread17799 = match property.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value17798) => {
                                            let source_ml_source2_data_property_value17798 = *source_ml_source2_data_property_value17798;
                                            source_ml_source2_data_property_value17798.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field17800 = match source_ml_source2_nominal_spread17799.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value17801, _, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value17801 = *source_ml_source2_data_property_value17801;
                                                source_ml_source2_data_property_value17801.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field17802 = match source_ml_source2_nominal_spread17799.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17803, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value17803 = *source_ml_source2_data_property_value17803;
                                                    source_ml_source2_data_property_value17803.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17804 = match source_ml_source2_nominal_spread17799.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value17805, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value17805.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17806 = match source_ml_source2_nominal_spread17799.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value17807, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value17807
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17808 = match source_ml_source2_nominal_spread17799.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value17809, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value17809
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17810 = match source_ml_source2_nominal_spread17799.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value17811, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value17811.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17812 = match source_ml_source2_nominal_spread17799.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value17813, _, _, _) => {
                                                                        source_ml_source2_data_property_value17813
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field17814 = match source_ml_source2_nominal_spread17799.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value17815, _, _) => {
                                                                            source_ml_source2_data_property_value17815.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field17816 = match source_ml_source2_nominal_spread17799.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value17817, _) => {
                                                                                source_ml_source2_data_property_value17817.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field17818 = match source_ml_source2_nominal_spread17799.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17819) => {
                                                                                    source_ml_source2_data_property_value17819.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override17820 = (converted_id + 1f64);
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field17800.clone()), Box::new(source_ml_source2_nominal_field17802.clone()), source_ml_source2_nominal_field17804.clone(), source_ml_source2_nominal_override17820, source_ml_source2_nominal_field17808, source_ml_source2_nominal_field17810.clone(), source_ml_source2_nominal_field17812, source_ml_source2_nominal_field17814.clone(), source_ml_source2_nominal_field17816.clone(), source_ml_source2_nominal_field17818.clone())
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
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17821, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17821 = *source_ml_source2_data_property_value17821;
                                                        source_ml_source2_data_property_value17821.clone()
                                                    }
                                                } {
                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17822, _) => {
                                                        let source_ml_source2_data_property_value17822 = *source_ml_source2_data_property_value17822;
                                                        source_ml_source2_data_property_value17822.clone()
                                                    }
                                                })), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![raw_key.clone()])));
                                                {
                                                    let conversion = self::JavaScriptCallSource::SourcePropertyKey(Box::new(raw_key.clone()), match match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17823, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value17823 = *source_ml_source2_data_property_value17823;
                                                            source_ml_source2_data_property_value17823.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value17824, _, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value17824.clone()
                                                        }
                                                    });
                                                    {
                                                        let checked = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Null)))), Box::new(failure.clone()), Box::new(self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(receiver.clone()), Box::new((constant.clone())(self::JavaScriptSemanticValue::Undefined)))), Box::new(failure.clone()), Box::new(conversion.clone()))));
                                                        {
                                                            let captures = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(receiver_id, Box::new(match object.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17825, _) => {
                                                                    let source_ml_source2_data_property_value17825 = *source_ml_source2_data_property_value17825;
                                                                    source_ml_source2_data_property_value17825.clone()
                                                                }
                                                            })), self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(key_id, Box::new(match property.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value17826, _) => {
                                                                    let source_ml_source2_data_property_value17826 = *source_ml_source2_data_property_value17826;
                                                                    source_ml_source2_data_property_value17826.clone()
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
        self::JavaScriptSourceReference::SourceReference(source_ml_source2_data_property_value17828, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value17828.clone()
        }
    }) == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108]))) {
        {
            let binding = { let ml_values332 = { let ml_reference_value333 = ((match match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17829) => {
                    let source_ml_source2_data_property_value17829 = *source_ml_source2_data_property_value17829;
                    source_ml_source2_data_property_value17829.clone()
                }
            } {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17830) => {
                    ((source_ml_source2_data_property_value17830.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                }
            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value333 }; let ml_callback332 = { let reference = reference.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value17833, _, _, _, _) => {
                    source_ml_source2_data_property_value17833
                }
            }) == (match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, source_ml_source2_data_property_value17834, _, _, _, _, _) => {
                    source_ml_source2_data_property_value17834
                }
            })) }) }; ml_values332.iter().cloned().enumerate().find_map(|(ml_index332, ml_value332)| if ml_callback332(ml_value332.clone()) { Some(ml_value332) } else { None }) };
            self::JavaScriptCallSource::SourceIf(Box::new({
                let source_ml_source2_constructor_field17836 = match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17835, _, _, _) => {
                        let source_ml_source2_data_property_value17835 = *source_ml_source2_data_property_value17835;
                        source_ml_source2_data_property_value17835.clone()
                    }
                };
                {
                    let source_ml_source2_constructor_field17837 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100])));
                    self::JavaScriptCallSource::SourceGet(Box::new(source_ml_source2_constructor_field17836.clone()), Box::new(source_ml_source2_constructor_field17837.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
                }
            }), Box::new(if (!(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                let source_ml_source2_nullish_field_receiver17838 = (binding.clone(),);
                if match (source_ml_source2_nullish_field_receiver17838.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                    {
                        let source_ml_source2_nullish_field_present17839 = ((source_ml_source2_nullish_field_receiver17838.clone()).0).clone().expect("a proven present value");
                        match source_ml_source2_nullish_field_present17839.clone() {
                            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value17840, _, _) => {
                                source_ml_source2_data_property_value17840
                            }
                        }
                    }
                } else {
                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 109, 109, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                }
            }) {
                self::JavaScriptCallSource::SourceImmutableBindingWrite(Box::new(value.clone()))
            } else {
                self::JavaScriptCallSource::SourceSnapshotSet(Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17841, _, _, _) => {
                        let source_ml_source2_data_property_value17841 = *source_ml_source2_data_property_value17841;
                        source_ml_source2_data_property_value17841.clone()
                    }
                }), Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, source_ml_source2_data_property_value17842, _, _) => {
                        let source_ml_source2_data_property_value17842 = *source_ml_source2_data_property_value17842;
                        source_ml_source2_data_property_value17842.clone()
                    }
                }), Box::new(value.clone()))
            }), Box::new(self::construct_java_script_lexical_cell_failure(match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17843) => {
                    let source_ml_source2_data_property_value17843 = *source_ml_source2_data_property_value17843;
                    source_ml_source2_data_property_value17843.clone()
                }
            }, match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(_, source_ml_source2_data_property_value17844, _, _, _, _, _) => {
                    source_ml_source2_data_property_value17844
                }
            })))
        }
    } else {
        if ((match reference.clone() {
            self::JavaScriptSourceReference::SourceReference(source_ml_source2_data_property_value17846, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value17846.clone()
            }
        }) == (self::ml_string::Text::from_units(vec![105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 98, 105, 110, 100, 105, 110, 103]))) {
            self::JavaScriptCallSource::SourceImmutableBindingWrite(Box::new(value.clone()))
        } else {
            if ((match reference.clone() {
                self::JavaScriptSourceReference::SourceReference(source_ml_source2_data_property_value17848, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value17848.clone()
                }
            }) == (self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103]))) {
                self::JavaScriptCallSource::SourceAssignBinding(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, source_ml_source2_data_property_value17849, _, _, _, _, _) => {
                        source_ml_source2_data_property_value17849
                    }
                }, Box::new(value.clone()))
            } else {
                self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17850, _, _, _) => {
                        let source_ml_source2_data_property_value17850 = *source_ml_source2_data_property_value17850;
                        source_ml_source2_data_property_value17850.clone()
                    }
                }])))), Box::new(self::JavaScriptCallSource::SourceSnapshotSet(Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17851, _, _, _) => {
                        let source_ml_source2_data_property_value17851 = *source_ml_source2_data_property_value17851;
                        source_ml_source2_data_property_value17851.clone()
                    }
                }), Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, source_ml_source2_data_property_value17852, _, _) => {
                        let source_ml_source2_data_property_value17852 = *source_ml_source2_data_property_value17852;
                        source_ml_source2_data_property_value17852.clone()
                    }
                }), Box::new(value.clone()))), Box::new(self::JavaScriptCallSource::SourceCall(Box::new(self::construct_java_script_error_constant(match match match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, _, source_ml_source2_data_property_value17853) => {
                        let source_ml_source2_data_property_value17853 = *source_ml_source2_data_property_value17853;
                        source_ml_source2_data_property_value17853.clone()
                    }
                } {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value17854, _, _, _, _, _, _, _, _) => {
                        let source_ml_source2_data_property_value17854 = *source_ml_source2_data_property_value17854;
                        source_ml_source2_data_property_value17854.clone()
                    }
                } {
                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value17855) => {
                        let source_ml_source2_data_property_value17855 = *source_ml_source2_data_property_value17855;
                        source_ml_source2_data_property_value17855.clone()
                    }
                })), Box::new(match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, source_ml_source2_data_property_value17856, _, _, _) => {
                        let source_ml_source2_data_property_value17856 = *source_ml_source2_data_property_value17856;
                        source_ml_source2_data_property_value17856.clone()
                    }
                }), std::rc::Rc::new(std::cell::RefCell::new(vec![match reference.clone() {
                    self::JavaScriptSourceReference::SourceReference(_, _, _, _, source_ml_source2_data_property_value17857, _, _) => {
                        let source_ml_source2_data_property_value17857 = *source_ml_source2_data_property_value17857;
                        source_ml_source2_data_property_value17857.clone()
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
                self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, source_ml_source2_data_property_value17858, _) => {
                    source_ml_source2_data_property_value17858.clone()
                }
            }; let ml_reference_projection334 = ml_reference_array334.borrow().len() as f64; ml_reference_projection334 } - 1f64);
            {
                let source_2 = self::source_ml_source1_temporary_construct_java_script_source_reference_captures_loop210(reference.clone(), context.clone(), source.clone(), at);
                source_2.clone()
            }
        }
    }
}
