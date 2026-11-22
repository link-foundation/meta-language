// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptErrorFunctionOwner;
use super::JavaScriptErrorFunctionPlan;
use super::JavaScriptErrorModuleBinding;
use super::JavaScriptErrorModuleCompilation;
use super::JavaScriptErrorModulePlan;
use super::JavaScriptErrorModuleRegistry;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
use super::JavaScriptSourceLexicalBinding;
use super::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179;
use super::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result;
use super::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Result;
use super::compile_java_script_call_source_with_captures;
use super::compile_java_script_mutable_call_source_with_captures;
use super::construct_java_script_error_constant;
use super::lower_java_script_error_expression;
use super::lower_java_script_error_function_body;
use super::lower_java_script_source_parameters;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop181;
use super::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop183;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop171(mut plan: self::JavaScriptErrorModulePlan, mut registry: self::JavaScriptErrorModuleRegistry, mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorModuleBinding>>>, mut capabilities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>, mut empty: self::JavaScriptCallProgram, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut source_ml_source1_temporary_values165: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorFunctionPlan>>>, mut source_ml_source1_temporary_index165: self::ml::Big) -> self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result {
    loop {
        return if (source_ml_source1_temporary_index165 < ({ let ml_reference_array569 = source_ml_source1_temporary_values165.clone(); let ml_reference_projection569 = self::ml::Big::from_u128(ml_reference_array569.borrow().len() as u128); ml_reference_projection569 })) {
            {
                let definition = { let ml_reference_array570 = source_ml_source1_temporary_values165.clone(); let ml_reference_position570 = source_ml_source1_temporary_index165.clone(); let ml_reference_projection570 = self::ml_array::at(&*ml_reference_array570.borrow(), ml_reference_position570.to_index()); ml_reference_projection570 };
                {
                    let matches = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values571 = { let ml_reference_value572 = (match registry.clone() {
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value20736, _, _, _, _) => {
                            source_ml_source2_data_property_value20736.clone()
                        }
                    }).borrow().clone(); ml_reference_value572 }; let ml_callback571 = { let definition = definition.clone(); std::rc::Rc::new(move |owner: self::JavaScriptErrorFunctionOwner| -> bool { ((match owner.clone() {
                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value20746, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20746
                        }
                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value20747, _, _, _) => {
                            source_ml_source2_data_property_value20747
                        }
                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value20748, _, _) => {
                            source_ml_source2_data_property_value20748
                        }
                    }) == (match definition.clone() {
                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(source_ml_source2_data_property_value20749, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20749
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(source_ml_source2_data_property_value20750, _, _, _, _) => {
                            source_ml_source2_data_property_value20750
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(source_ml_source2_data_property_value20751, _, _, _) => {
                            source_ml_source2_data_property_value20751
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(source_ml_source2_data_property_value20752, _, _, _) => {
                            source_ml_source2_data_property_value20752
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(source_ml_source2_data_property_value20753, _, _, _) => {
                            source_ml_source2_data_property_value20753
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(source_ml_source2_data_property_value20754, _, _, _) => {
                            source_ml_source2_data_property_value20754
                        }
                    })) }) }; ml_values571.iter().cloned().enumerate().filter_map(|(ml_index571, ml_value571)| if ml_callback571(ml_value571.clone()) { Some(ml_value571) } else { None }).collect::<Vec<_>>() }));
                    if (({ let ml_reference_array573 = matches.clone(); let ml_reference_projection573 = ml_reference_array573.borrow().len() as f64; ml_reference_projection573 }) != (1f64)) {
                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114]))))
                    } else {
                        {
                            let owner = {
                                let source_ml_source2_array_read_index20756 = 0f64;
                                { let ml_reference_array574 = matches.clone(); let ml_reference_position574 = source_ml_source2_array_read_index20756; let ml_reference_projection574 = self::ml_array::optional_at(&*ml_reference_array574.borrow(), self::ml_array::number_index(ml_reference_position574)); ml_reference_projection574 }
                            };
                            if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                            } else {
                                if (self::ml_array::includes(&{ let ml_reference_value575 = identities.borrow().clone(); ml_reference_value575 }, &match owner.clone().expect("a proven present value") {
                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value20757, _, _, _, _) => {
                                        source_ml_source2_data_property_value20757.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value20758, _, _) => {
                                        source_ml_source2_data_property_value20758.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value20759, _) => {
                                        source_ml_source2_data_property_value20759.clone()
                                    }
                                }, 0f64, false, |a, b| a == b) || !(self::ml_array::includes(&{ let ml_reference_value576 = (match registry.clone() {
                                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value20761, _) => {
                                        source_ml_source2_data_property_value20761.clone()
                                    }
                                }).borrow().clone(); ml_reference_value576 }, &match owner.clone().expect("a proven present value") {
                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value20762, _, _, _) => {
                                        source_ml_source2_data_property_value20762.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value20763, _) => {
                                        source_ml_source2_data_property_value20763.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value20764) => {
                                        source_ml_source2_data_property_value20764.clone()
                                    }
                                }, 0f64, false, |a, b| a == b))) {
                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 115]))))
                                } else {
                                    {
                                        let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored178 = { let ml_method_owner577 = identities.clone(); let ml_method_argument577_0 = match owner.clone().expect("a proven present value") {
                                            self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value20765, _, _, _, _) => {
                                                source_ml_source2_data_property_value20765.clone()
                                            }
                                            self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value20766, _, _) => {
                                                source_ml_source2_data_property_value20766.clone()
                                            }
                                            self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value20767, _) => {
                                                source_ml_source2_data_property_value20767.clone()
                                            }
                                        }; let ml_inserted = vec![ml_method_argument577_0]; let mut ml_method_values577 = ml_method_owner577.borrow_mut(); let ml_length = ml_method_values577.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values577.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let operations = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values578 = { let ml_reference_value579 = (match registry.clone() {
                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, source_ml_source2_data_property_value20768, _, _, _) => {
                                                    source_ml_source2_data_property_value20768.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value579 }; let ml_callback578 = { let owner = owner.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorRealmOperations| -> bool { ((match row.clone() {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20773, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20773.clone()
                                                }
                                            }) == (match owner.clone().expect("a proven present value") {
                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value20774, _, _, _) => {
                                                    source_ml_source2_data_property_value20774.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value20775, _) => {
                                                    source_ml_source2_data_property_value20775.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value20776) => {
                                                    source_ml_source2_data_property_value20776.clone()
                                                }
                                            })) }) }; ml_values578.iter().cloned().enumerate().filter_map(|(ml_index578, ml_value578)| if ml_callback578(ml_value578.clone()) { Some(ml_value578) } else { None }).collect::<Vec<_>>() }));
                                            if (({ let ml_reference_array580 = operations.clone(); let ml_reference_projection580 = ml_reference_array580.borrow().len() as f64; ml_reference_projection580 }) != (1f64)) {
                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))))
                                            } else {
                                                {
                                                    let realm = {
                                                        let source_ml_source2_array_read_index20778 = 0f64;
                                                        { let ml_reference_array581 = operations.clone(); let ml_reference_position581 = source_ml_source2_array_read_index20778; let ml_reference_projection581 = self::ml_array::optional_at(&*ml_reference_array581.borrow(), self::ml_array::number_index(ml_reference_position581)); ml_reference_projection581 }
                                                    };
                                                    if match (realm.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109, 32, 114, 111, 119, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                                                    } else {
                                                        {
                                                            let context = {
                                                                let source_ml_source2_constructor_field20779 = registry.clone();
                                                                {
                                                                    let source_ml_source2_constructor_field20780 = realm.clone().expect("a proven present value");
                                                                    {
                                                                        let source_ml_source2_constructor_field20781 = capabilities.clone();
                                                                        {
                                                                            let source_ml_source2_constructor_field20783 = Some((match plan.clone() {
                                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, source_ml_source2_data_property_value20782, _) => {
                                                                                    ((source_ml_source2_data_property_value20782.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                                                            {
                                                                                let source_ml_source2_constructor_field20785 = Some((match plan.clone() {
                                                                                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, _, source_ml_source2_data_property_value20784) => {
                                                                                        ((source_ml_source2_data_property_value20784.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new()))));
                                                                                {
                                                                                    let source_ml_source2_constructor_field20787 = match plan.clone() {
                                                                                        self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value20786, _, _, _) => {
                                                                                            source_ml_source2_data_property_value20786
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_constructor_field20788 = 0f64;
                                                                                        {
                                                                                            let source_ml_source2_constructor_field20789 = self::ml_string::Text::from_units(vec![]);
                                                                                            {
                                                                                                let source_ml_source2_constructor_field20791 = Some(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values582 = { let ml_reference_value583 = bindings.borrow().clone(); ml_reference_value583 }; let ml_callback582 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value20790, _) => {
                                                                                                        source_ml_source2_data_property_value20790
                                                                                                    }
                                                                                                } }); ml_values582.iter().cloned().enumerate().map(|(ml_index582, ml_value582)| ml_callback582(ml_value582)).collect::<Vec<_>>() })));
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field20873 = ((match definition.clone() {
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, _, _) => {
                                                                                                            true
                                                                                                        }
                                                                                                        _ => {
                                                                                                            false
                                                                                                        }
                                                                                                    } || match definition.clone() {
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, _, _) => {
                                                                                                            true
                                                                                                        }
                                                                                                        _ => {
                                                                                                            false
                                                                                                        }
                                                                                                    }) || match definition.clone() {
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, _, _) => {
                                                                                                            true
                                                                                                        }
                                                                                                        _ => {
                                                                                                            false
                                                                                                        }
                                                                                                    });
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_constructor_field20779.clone()), Box::new(source_ml_source2_constructor_field20780.clone()), source_ml_source2_constructor_field20781.clone(), source_ml_source2_constructor_field20787, source_ml_source2_constructor_field20788, source_ml_source2_constructor_field20789.clone(), source_ml_source2_constructor_field20873, Some((source_ml_source2_constructor_field20791.clone(),)), Some((source_ml_source2_constructor_field20783.clone(),)), Some((source_ml_source2_constructor_field20785.clone(),)))
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
                                                                let source = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
                                                                {
                                                                    let source_ml_source2_s21234 = match definition.clone() {
                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, base_class, _, definition_message, fields) => {
                                                                            let definition_message = *definition_message;
                                                                            match owner.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, identity, _, prototype, base_constructor, base_realm) => {
                                                                                    let prototype = *prototype;
                                                                                    let base_constructor = *base_constructor;
                                                                                    if !(self::ml_array::includes(&{ let ml_reference_value584 = (match registry.clone() {
                                                                                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value21269, _) => {
                                                                                            source_ml_source2_data_property_value21269.clone()
                                                                                        }
                                                                                    }).borrow().clone(); ml_reference_value584 }, &base_realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 115, 117, 98, 99, 108, 97, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 97, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                                    } else {
                                                                                        {
                                                                                            let message = self::lower_java_script_error_expression(definition_message.clone(), context.clone(), 0f64);
                                                                                            {
                                                                                                let context_2 = match message.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21270) => {
                                                                                                        let source_ml_source2_data_property_value21270 = *source_ml_source2_data_property_value21270;
                                                                                                        source_ml_source2_data_property_value21270.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let message_identity = match context_2.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21271, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21271
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let context_3 = {
                                                                                                            let source_ml_source2_nominal_field21274 = match context_2.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value21275, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    let source_ml_source2_data_property_value21275 = *source_ml_source2_data_property_value21275;
                                                                                                                    source_ml_source2_data_property_value21275.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field21276 = match context_2.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value21277, _, _, _, _, _, _, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value21277 = *source_ml_source2_data_property_value21277;
                                                                                                                        source_ml_source2_data_property_value21277.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field21278 = match context_2.clone() {
                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value21279, _, _, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value21279.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field21280 = match context_2.clone() {
                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21281, _, _, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value21281
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field21282 = match context_2.clone() {
                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value21283, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value21283
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field21284 = match context_2.clone() {
                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21285, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value21285.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field21286 = match context_2.clone() {
                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value21287, _, _, _) => {
                                                                                                                                            source_ml_source2_data_property_value21287
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field21288 = match context_2.clone() {
                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value21289, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value21289.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field21290 = match context_2.clone() {
                                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value21291, _) => {
                                                                                                                                                    source_ml_source2_data_property_value21291.clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field21292 = match context_2.clone() {
                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21293) => {
                                                                                                                                                        source_ml_source2_data_property_value21293.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override21294 = (match context_2.clone() {
                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21272, _, _, _, _, _, _) => {
                                                                                                                                                            source_ml_source2_data_property_value21272
                                                                                                                                                        }
                                                                                                                                                    } + 1f64);
                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field21274.clone()), Box::new(source_ml_source2_nominal_field21276.clone()), source_ml_source2_nominal_field21278.clone(), source_ml_source2_nominal_override21294, source_ml_source2_nominal_field21282, source_ml_source2_nominal_field21284.clone(), source_ml_source2_nominal_field21286, source_ml_source2_nominal_field21288.clone(), source_ml_source2_nominal_field21290.clone(), source_ml_source2_nominal_field21292.clone())
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
                                                                                                            let raw_ = self::JavaScriptCallSource::SourceBinding(message_identity);
                                                                                                            {
                                                                                                                let absent = self::JavaScriptCallSource::SourceStrictEqual(Box::new(raw_.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)));
                                                                                                                {
                                                                                                                    let construct = self::JavaScriptCallSource::SourceConstructErrorValue(Box::new(self::construct_java_script_error_constant(base_constructor.clone())), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Object(identity.clone(), true))), Box::new(self::construct_java_script_error_constant(prototype.clone())), Box::new(self::JavaScriptCallSource::SourceIf(Box::new(absent.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))))), Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![raw_.clone()])))))), Box::new(self::JavaScriptCallSource::SourceIf(Box::new(absent.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Boolean(false))), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Boolean(true))))), base_realm.clone(), base_class.clone(), true);
                                                                                                                    {
                                                                                                                        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()));
                                                                                                                        {
                                                                                                                            let source_ml_source1_temporary_index166 = self::ml::Big::from_i128(0);
                                                                                                                            {
                                                                                                                                let context_4 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop181(Some(realm.clone().expect("a proven present value")), context_3.clone(), writes.clone(), fields.clone(), source_ml_source1_temporary_index166.clone());
                                                                                                                                {
                                                                                                                                    let source_2 = self::JavaScriptCallSource::SourceDerivedReceiver;
                                                                                                                                    {
                                                                                                                                        let at = ({ let ml_reference_array585 = writes.clone(); let ml_reference_projection585 = ml_reference_array585.borrow().len() as f64; ml_reference_projection585 } - 1f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s21295 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop183(context_4.clone(), source_2.clone(), writes.clone(), at);
                                                                                                                                            match source_ml_source2_s21295.clone() {
                                                                                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Done(context_5, source_3) => {
                                                                                                                                                    let context_5 = *context_5;
                                                                                                                                                    let source_3 = *source_3;
                                                                                                                                                    {
                                                                                                                                                        let super_identity = match context_5.clone() {
                                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21296, _, _, _, _, _, _) => {
                                                                                                                                                                source_ml_source2_data_property_value21296
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let context_6 = {
                                                                                                                                                                let source_ml_source2_nominal_field21298 = match context_5.clone() {
                                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value21299, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                        let source_ml_source2_data_property_value21299 = *source_ml_source2_data_property_value21299;
                                                                                                                                                                        source_ml_source2_data_property_value21299.clone()
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field21300 = match context_5.clone() {
                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value21301, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value21301 = *source_ml_source2_data_property_value21301;
                                                                                                                                                                            source_ml_source2_data_property_value21301.clone()
                                                                                                                                                                        }
                                                                                                                                                                    };
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field21302 = match context_5.clone() {
                                                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value21303, _, _, _, _, _, _, _) => {
                                                                                                                                                                                source_ml_source2_data_property_value21303.clone()
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field21304 = match context_5.clone() {
                                                                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21305, _, _, _, _, _, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value21305
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_field21306 = match context_5.clone() {
                                                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value21307, _, _, _, _, _) => {
                                                                                                                                                                                        source_ml_source2_data_property_value21307
                                                                                                                                                                                    }
                                                                                                                                                                                };
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_nominal_field21308 = match context_5.clone() {
                                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21309, _, _, _, _) => {
                                                                                                                                                                                            source_ml_source2_data_property_value21309.clone()
                                                                                                                                                                                        }
                                                                                                                                                                                    };
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source2_nominal_field21310 = match context_5.clone() {
                                                                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value21311, _, _, _) => {
                                                                                                                                                                                                source_ml_source2_data_property_value21311
                                                                                                                                                                                            }
                                                                                                                                                                                        };
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_nominal_field21312 = match context_5.clone() {
                                                                                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value21313, _, _) => {
                                                                                                                                                                                                    source_ml_source2_data_property_value21313.clone()
                                                                                                                                                                                                }
                                                                                                                                                                                            };
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source2_nominal_field21314 = match context_5.clone() {
                                                                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value21315, _) => {
                                                                                                                                                                                                        source_ml_source2_data_property_value21315.clone()
                                                                                                                                                                                                    }
                                                                                                                                                                                                };
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_nominal_field21316 = match context_5.clone() {
                                                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21317) => {
                                                                                                                                                                                                            source_ml_source2_data_property_value21317.clone()
                                                                                                                                                                                                        }
                                                                                                                                                                                                    };
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_nominal_override21318 = (super_identity + 1f64);
                                                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field21298.clone()), Box::new(source_ml_source2_nominal_field21300.clone()), source_ml_source2_nominal_field21302.clone(), source_ml_source2_nominal_override21318, source_ml_source2_nominal_field21306, source_ml_source2_nominal_field21308.clone(), source_ml_source2_nominal_field21310, source_ml_source2_nominal_field21312.clone(), source_ml_source2_nominal_field21314.clone(), source_ml_source2_nominal_field21316.clone())
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
                                                                                                                                                                let source_4 = self::JavaScriptCallSource::SourceLet(message_identity, Box::new(match message.clone() {
                                                                                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21319, _) => {
                                                                                                                                                                        let source_ml_source2_data_property_value21319 = *source_ml_source2_data_property_value21319;
                                                                                                                                                                        source_ml_source2_data_property_value21319.clone()
                                                                                                                                                                    }
                                                                                                                                                                }), Box::new(self::JavaScriptCallSource::SourceLet(super_identity, Box::new(construct.clone()), Box::new(source_3.clone()))));
                                                                                                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(Box::new(context_6.clone()), Box::new(source_4.clone()))
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
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, _, _) => {
                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 115, 117, 98, 99, 108, 97, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 97, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                                }
                                                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, _) => {
                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 115, 117, 98, 99, 108, 97, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 97, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                                }
                                                                            }
                                                                        }
                                                                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, _, _, _) => {
                                                                            if match owner.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } {
                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 111, 114, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 119, 110, 101, 114, 115, 104, 105, 112]))))
                                                                            } else {
                                                                                {
                                                                                    let body = self::lower_java_script_error_function_body(definition.clone(), context.clone());
                                                                                    {
                                                                                        let context_7 = match body.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21373) => {
                                                                                                let source_ml_source2_data_property_value21373 = *source_ml_source2_data_property_value21373;
                                                                                                source_ml_source2_data_property_value21373.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_5 = match body.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21374, _) => {
                                                                                                    let source_ml_source2_data_property_value21374 = *source_ml_source2_data_property_value21374;
                                                                                                    source_ml_source2_data_property_value21374.clone()
                                                                                                }
                                                                                            };
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(Box::new(context_7.clone()), Box::new(source_5.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, _, _) => {
                                                                            if match owner.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, _, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } {
                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 111, 114, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 119, 110, 101, 114, 115, 104, 105, 112]))))
                                                                            } else {
                                                                                {
                                                                                    let body_2 = self::lower_java_script_error_function_body(definition.clone(), context.clone());
                                                                                    {
                                                                                        let context_8 = match body_2.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21428) => {
                                                                                                let source_ml_source2_data_property_value21428 = *source_ml_source2_data_property_value21428;
                                                                                                source_ml_source2_data_property_value21428.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_6 = match body_2.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21429, _) => {
                                                                                                    let source_ml_source2_data_property_value21429 = *source_ml_source2_data_property_value21429;
                                                                                                    source_ml_source2_data_property_value21429.clone()
                                                                                                }
                                                                                            };
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(Box::new(context_8.clone()), Box::new(source_6.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, _, _) => {
                                                                            if match owner.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } {
                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 111, 114, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 119, 110, 101, 114, 115, 104, 105, 112]))))
                                                                            } else {
                                                                                {
                                                                                    let body_3 = self::lower_java_script_error_function_body(definition.clone(), context.clone());
                                                                                    {
                                                                                        let context_9 = match body_3.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21483) => {
                                                                                                let source_ml_source2_data_property_value21483 = *source_ml_source2_data_property_value21483;
                                                                                                source_ml_source2_data_property_value21483.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_7 = match body_3.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21484, _) => {
                                                                                                    let source_ml_source2_data_property_value21484 = *source_ml_source2_data_property_value21484;
                                                                                                    source_ml_source2_data_property_value21484.clone()
                                                                                                }
                                                                                            };
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(Box::new(context_9.clone()), Box::new(source_7.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, _, _) => {
                                                                            if match owner.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } {
                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 111, 114, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 119, 110, 101, 114, 115, 104, 105, 112]))))
                                                                            } else {
                                                                                {
                                                                                    let body_4 = self::lower_java_script_error_function_body(definition.clone(), context.clone());
                                                                                    {
                                                                                        let context_10 = match body_4.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21538) => {
                                                                                                let source_ml_source2_data_property_value21538 = *source_ml_source2_data_property_value21538;
                                                                                                source_ml_source2_data_property_value21538.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_8 = match body_4.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21539, _) => {
                                                                                                    let source_ml_source2_data_property_value21539 = *source_ml_source2_data_property_value21539;
                                                                                                    source_ml_source2_data_property_value21539.clone()
                                                                                                }
                                                                                            };
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(Box::new(context_10.clone()), Box::new(source_8.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, _, _) => {
                                                                            if match owner.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } {
                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 111, 114, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 119, 110, 101, 114, 115, 104, 105, 112]))))
                                                                            } else {
                                                                                {
                                                                                    let body_5 = self::lower_java_script_error_function_body(definition.clone(), context.clone());
                                                                                    {
                                                                                        let context_11 = match body_5.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21593) => {
                                                                                                let source_ml_source2_data_property_value21593 = *source_ml_source2_data_property_value21593;
                                                                                                source_ml_source2_data_property_value21593.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_9 = match body_5.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21594, _) => {
                                                                                                    let source_ml_source2_data_property_value21594 = *source_ml_source2_data_property_value21594;
                                                                                                    source_ml_source2_data_property_value21594.clone()
                                                                                                }
                                                                                            };
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(Box::new(context_11.clone()), Box::new(source_9.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    match source_ml_source2_s21234.clone() {
                                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(context_12, source_10) => {
                                                                            let context_12 = *context_12;
                                                                            let source_10 = *source_10;
                                                                            {
                                                                                let initialized = self::lower_java_script_source_parameters(match definition.clone() {
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21595, _, _) => {
                                                                                        source_ml_source2_data_property_value21595.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21596, _, _) => {
                                                                                        source_ml_source2_data_property_value21596.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21597, _) => {
                                                                                        source_ml_source2_data_property_value21597.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21598, _) => {
                                                                                        source_ml_source2_data_property_value21598.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21599, _) => {
                                                                                        source_ml_source2_data_property_value21599.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21600, _) => {
                                                                                        source_ml_source2_data_property_value21600.clone()
                                                                                    }
                                                                                }, context_12.clone(), source_10.clone());
                                                                                {
                                                                                    let context_13 = match initialized.clone() {
                                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21601) => {
                                                                                            let source_ml_source2_data_property_value21601 = *source_ml_source2_data_property_value21601;
                                                                                            source_ml_source2_data_property_value21601.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_11 = match initialized.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21602, _) => {
                                                                                                let source_ml_source2_data_property_value21602 = *source_ml_source2_data_property_value21602;
                                                                                                source_ml_source2_data_property_value21602.clone()
                                                                                            }
                                                                                        };
                                                                                        if ((match context_13.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21604, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value21604.clone()
                                                                                            }
                                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), match context_13.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21605, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value21605.clone()
                                                                                                }
                                                                                            })))
                                                                                        } else {
                                                                                            {
                                                                                                let code = if match context_13.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value21606, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value21606
                                                                                                    }
                                                                                                } {
                                                                                                    self::compile_java_script_mutable_call_source_with_captures(source_11.clone(), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values586 = { let ml_reference_value587 = (match definition.clone() {
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21607, _, _) => {
                                                                                                            source_ml_source2_data_property_value21607.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21608, _, _) => {
                                                                                                            source_ml_source2_data_property_value21608.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21609, _) => {
                                                                                                            source_ml_source2_data_property_value21609.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21610, _) => {
                                                                                                            source_ml_source2_data_property_value21610.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21611, _) => {
                                                                                                            source_ml_source2_data_property_value21611.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21612, _) => {
                                                                                                            source_ml_source2_data_property_value21612.clone()
                                                                                                        }
                                                                                                    }).borrow().clone(); ml_reference_value587 }; let ml_callback586 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> f64 { match parameter.clone() {
                                                                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value21613, _, _) => {
                                                                                                            source_ml_source2_data_property_value21613
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value21614, _) => {
                                                                                                            source_ml_source2_data_property_value21614
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value21615, _, _) => {
                                                                                                            source_ml_source2_data_property_value21615
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value21616, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21616
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value21617, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21617
                                                                                                        }
                                                                                                    } }); ml_values586.iter().cloned().enumerate().map(|(ml_index586, ml_value586)| ml_callback586(ml_value586)).collect::<Vec<_>>() })), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values588 = { let ml_reference_value589 = bindings.borrow().clone(); ml_reference_value589 }; let ml_callback588 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                        self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21618, _) => {
                                                                                                            source_ml_source2_data_property_value21618
                                                                                                        }
                                                                                                    } }); ml_values588.iter().cloned().enumerate().map(|(ml_index588, ml_value588)| ml_callback588(ml_value588)).collect::<Vec<_>>() })))
                                                                                                } else {
                                                                                                    self::compile_java_script_call_source_with_captures(source_11.clone(), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values590 = { let ml_reference_value591 = (match definition.clone() {
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21619, _, _) => {
                                                                                                            source_ml_source2_data_property_value21619.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21620, _, _) => {
                                                                                                            source_ml_source2_data_property_value21620.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21621, _) => {
                                                                                                            source_ml_source2_data_property_value21621.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21622, _) => {
                                                                                                            source_ml_source2_data_property_value21622.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21623, _) => {
                                                                                                            source_ml_source2_data_property_value21623.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21624, _) => {
                                                                                                            source_ml_source2_data_property_value21624.clone()
                                                                                                        }
                                                                                                    }).borrow().clone(); ml_reference_value591 }; let ml_callback590 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> f64 { match parameter.clone() {
                                                                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value21625, _, _) => {
                                                                                                            source_ml_source2_data_property_value21625
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value21626, _) => {
                                                                                                            source_ml_source2_data_property_value21626
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value21627, _, _) => {
                                                                                                            source_ml_source2_data_property_value21627
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value21628, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21628
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value21629, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21629
                                                                                                        }
                                                                                                    } }); ml_values590.iter().cloned().enumerate().map(|(ml_index590, ml_value590)| ml_callback590(ml_value590)).collect::<Vec<_>>() })), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values592 = { let ml_reference_value593 = bindings.borrow().clone(); ml_reference_value593 }; let ml_callback592 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                        self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21630, _) => {
                                                                                                            source_ml_source2_data_property_value21630
                                                                                                        }
                                                                                                    } }); ml_values592.iter().cloned().enumerate().map(|(ml_index592, ml_value592)| ml_callback592(ml_value592)).collect::<Vec<_>>() })))
                                                                                                };
                                                                                                if ((match code.clone() {
                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21632) => {
                                                                                                        source_ml_source2_data_property_value21632.clone()
                                                                                                    }
                                                                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), match code.clone() {
                                                                                                        self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21633) => {
                                                                                                            source_ml_source2_data_property_value21633.clone()
                                                                                                        }
                                                                                                    })))
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored180 = { let ml_method_owner594 = functions.clone(); let ml_method_argument594_0 = {
                                                                                                            let source_ml_source2_constructor_field21637 = match owner.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value21634, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value21634.clone()
                                                                                                                }
                                                                                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value21635, _, _) => {
                                                                                                                    source_ml_source2_data_property_value21635.clone()
                                                                                                                }
                                                                                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value21636, _) => {
                                                                                                                    source_ml_source2_data_property_value21636.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_constructor_field21641 = match owner.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value21638, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value21638.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value21639, _) => {
                                                                                                                        source_ml_source2_data_property_value21639.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value21640) => {
                                                                                                                        source_ml_source2_data_property_value21640.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_constructor_field21696 = if match definition.clone() {
                                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, _, _, _) => {
                                                                                                                            true
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            false
                                                                                                                        }
                                                                                                                    } {
                                                                                                                        self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114])
                                                                                                                    } else {
                                                                                                                        if match definition.clone() {
                                                                                                                            self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, _, _) => {
                                                                                                                                true
                                                                                                                            }
                                                                                                                            _ => {
                                                                                                                                false
                                                                                                                            }
                                                                                                                        } {
                                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115])
                                                                                                                        } else {
                                                                                                                            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116])
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_constructor_field21697 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                        {
                                                                                                                            let source_ml_source2_constructor_field21704 = { let ml_reference_array595 = match definition.clone() {
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21698, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value21698.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21699, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value21699.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21700, _) => {
                                                                                                                                    source_ml_source2_data_property_value21700.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21701, _) => {
                                                                                                                                    source_ml_source2_data_property_value21701.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21702, _) => {
                                                                                                                                    source_ml_source2_data_property_value21702.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21703, _) => {
                                                                                                                                    source_ml_source2_data_property_value21703.clone()
                                                                                                                                }
                                                                                                                            }; let ml_reference_projection595 = ml_reference_array595.borrow().len() as f64; ml_reference_projection595 };
                                                                                                                            {
                                                                                                                                let source_ml_source2_constructor_field21706 = match code.clone() {
                                                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, source_ml_source2_data_property_value21705, _) => {
                                                                                                                                        source_ml_source2_data_property_value21705
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_constructor_field21708 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values596 = { let ml_reference_value597 = bindings.borrow().clone(); ml_reference_value597 }; let ml_callback596 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> self::JavaScriptSemanticValue { match binding.clone() {
                                                                                                                                        self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(_, source_ml_source2_data_property_value21707) => {
                                                                                                                                            let source_ml_source2_data_property_value21707 = *source_ml_source2_data_property_value21707;
                                                                                                                                            source_ml_source2_data_property_value21707.clone()
                                                                                                                                        }
                                                                                                                                    } }); ml_values596.iter().cloned().enumerate().map(|(ml_index596, ml_value596)| ml_callback596(ml_value596)).collect::<Vec<_>>() }));
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_constructor_field21710 = match code.clone() {
                                                                                                                                            self::JavaScriptCallSourceCompilation::SourceCompiled(source_ml_source2_data_property_value21709, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value21709.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source2_constructor_field21637.clone(), source_ml_source2_constructor_field21641.clone(), source_ml_source2_constructor_field21696.clone(), Box::new(source_ml_source2_constructor_field21697.clone()), source_ml_source2_constructor_field21704, source_ml_source2_constructor_field21706, source_ml_source2_constructor_field21708.clone(), source_ml_source2_constructor_field21710.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }; let ml_inserted = vec![ml_method_argument594_0]; let mut ml_method_values594 = ml_method_owner594.borrow_mut(); let ml_length = ml_method_values594.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values594.extend(ml_inserted); ml_length as f64 };
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index165_2 = source_ml_source1_temporary_index165.add(&self::ml::Big::from_i128(1));
                                                                                                            {
                                                                                                                source_ml_source1_temporary_index165 = source_ml_source1_temporary_index165_2.clone();
                                                                                                                continue;
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
                                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(source_ml_source1_temporary_result180) => {
                                                                            let source_ml_source1_temporary_result180 = *source_ml_source1_temporary_result180;
                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(source_ml_source1_temporary_result180.clone()))
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
        } else {
            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Done
        };
    }
}
