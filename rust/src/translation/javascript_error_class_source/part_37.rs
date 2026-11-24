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
        return if (source_ml_source1_temporary_index165 < ({ let ml_reference_array566 = source_ml_source1_temporary_values165.clone(); let ml_reference_projection566 = self::ml::Big::from_u128(ml_reference_array566.borrow().len() as u128); ml_reference_projection566 })) {
            {
                let definition = { let ml_reference_array567 = source_ml_source1_temporary_values165.clone(); let ml_reference_position567 = source_ml_source1_temporary_index165.clone(); let ml_reference_projection567 = self::ml_array::at(&*ml_reference_array567.borrow(), ml_reference_position567.to_index()); ml_reference_projection567 };
                {
                    let matches = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values568 = { let ml_reference_value569 = (match registry.clone() {
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value20735, _, _, _, _) => {
                            source_ml_source2_data_property_value20735.clone()
                        }
                    }).borrow().clone(); ml_reference_value569 }; let ml_callback568 = { let definition = definition.clone(); std::rc::Rc::new(move |owner: self::JavaScriptErrorFunctionOwner| -> bool { ((match owner.clone() {
                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value20745, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20745
                        }
                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value20746, _, _, _) => {
                            source_ml_source2_data_property_value20746
                        }
                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value20747, _, _) => {
                            source_ml_source2_data_property_value20747
                        }
                    }) == (match definition.clone() {
                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(source_ml_source2_data_property_value20748, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20748
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(source_ml_source2_data_property_value20749, _, _, _, _) => {
                            source_ml_source2_data_property_value20749
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(source_ml_source2_data_property_value20750, _, _, _) => {
                            source_ml_source2_data_property_value20750
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(source_ml_source2_data_property_value20751, _, _, _) => {
                            source_ml_source2_data_property_value20751
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(source_ml_source2_data_property_value20752, _, _, _) => {
                            source_ml_source2_data_property_value20752
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(source_ml_source2_data_property_value20753, _, _, _) => {
                            source_ml_source2_data_property_value20753
                        }
                    })) }) }; ml_values568.iter().cloned().enumerate().filter_map(|(ml_index568, ml_value568)| if ml_callback568(ml_value568.clone()) { Some(ml_value568) } else { None }).collect::<Vec<_>>() }));
                    if (({ let ml_reference_array570 = matches.clone(); let ml_reference_projection570 = ml_reference_array570.borrow().len() as f64; ml_reference_projection570 }) != (1f64)) {
                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114]))))
                    } else {
                        {
                            let owner = {
                                let source_ml_source2_array_read_index20755 = 0f64;
                                { let ml_reference_array571 = matches.clone(); let ml_reference_position571 = source_ml_source2_array_read_index20755; let ml_reference_projection571 = self::ml_array::optional_at(&*ml_reference_array571.borrow(), self::ml_array::number_index(ml_reference_position571)); ml_reference_projection571 }
                            };
                            if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                            } else {
                                if (self::ml_array::includes(&{ let ml_reference_value572 = identities.borrow().clone(); ml_reference_value572 }, &match owner.clone().expect("a proven present value") {
                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value20756, _, _, _, _) => {
                                        source_ml_source2_data_property_value20756.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value20757, _, _) => {
                                        source_ml_source2_data_property_value20757.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value20758, _) => {
                                        source_ml_source2_data_property_value20758.clone()
                                    }
                                }, 0f64, false, |a, b| a == b) || !(self::ml_array::includes(&{ let ml_reference_value573 = (match registry.clone() {
                                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value20760, _) => {
                                        source_ml_source2_data_property_value20760.clone()
                                    }
                                }).borrow().clone(); ml_reference_value573 }, &match owner.clone().expect("a proven present value") {
                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value20761, _, _, _) => {
                                        source_ml_source2_data_property_value20761.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value20762, _) => {
                                        source_ml_source2_data_property_value20762.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value20763) => {
                                        source_ml_source2_data_property_value20763.clone()
                                    }
                                }, 0f64, false, |a, b| a == b))) {
                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 115]))))
                                } else {
                                    {
                                        let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored178 = { let ml_method_owner574 = identities.clone(); let ml_method_argument574_0 = match owner.clone().expect("a proven present value") {
                                            self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value20764, _, _, _, _) => {
                                                source_ml_source2_data_property_value20764.clone()
                                            }
                                            self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value20765, _, _) => {
                                                source_ml_source2_data_property_value20765.clone()
                                            }
                                            self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value20766, _) => {
                                                source_ml_source2_data_property_value20766.clone()
                                            }
                                        }; let ml_inserted = vec![ml_method_argument574_0]; let mut ml_method_values574 = ml_method_owner574.borrow_mut(); let ml_length = ml_method_values574.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values574.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let operations = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values575 = { let ml_reference_value576 = (match registry.clone() {
                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, source_ml_source2_data_property_value20767, _, _, _) => {
                                                    source_ml_source2_data_property_value20767.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value576 }; let ml_callback575 = { let owner = owner.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorRealmOperations| -> bool { ((match row.clone() {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20772, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20772.clone()
                                                }
                                            }) == (match owner.clone().expect("a proven present value") {
                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value20773, _, _, _) => {
                                                    source_ml_source2_data_property_value20773.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value20774, _) => {
                                                    source_ml_source2_data_property_value20774.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value20775) => {
                                                    source_ml_source2_data_property_value20775.clone()
                                                }
                                            })) }) }; ml_values575.iter().cloned().enumerate().filter_map(|(ml_index575, ml_value575)| if ml_callback575(ml_value575.clone()) { Some(ml_value575) } else { None }).collect::<Vec<_>>() }));
                                            if (({ let ml_reference_array577 = operations.clone(); let ml_reference_projection577 = ml_reference_array577.borrow().len() as f64; ml_reference_projection577 }) != (1f64)) {
                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))))
                                            } else {
                                                {
                                                    let realm = {
                                                        let source_ml_source2_array_read_index20777 = 0f64;
                                                        { let ml_reference_array578 = operations.clone(); let ml_reference_position578 = source_ml_source2_array_read_index20777; let ml_reference_projection578 = self::ml_array::optional_at(&*ml_reference_array578.borrow(), self::ml_array::number_index(ml_reference_position578)); ml_reference_projection578 }
                                                    };
                                                    if match (realm.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109, 32, 114, 111, 119, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                                                    } else {
                                                        {
                                                            let context = {
                                                                let source_ml_source2_constructor_field20778 = registry.clone();
                                                                {
                                                                    let source_ml_source2_constructor_field20779 = realm.clone().expect("a proven present value");
                                                                    {
                                                                        let source_ml_source2_constructor_field20780 = capabilities.clone();
                                                                        {
                                                                            let source_ml_source2_constructor_field20782 = Some((match plan.clone() {
                                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, source_ml_source2_data_property_value20781, _) => {
                                                                                    ((source_ml_source2_data_property_value20781.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                                                            {
                                                                                let source_ml_source2_constructor_field20784 = Some((match plan.clone() {
                                                                                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, _, source_ml_source2_data_property_value20783) => {
                                                                                        ((source_ml_source2_data_property_value20783.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new()))));
                                                                                {
                                                                                    let source_ml_source2_constructor_field20786 = match plan.clone() {
                                                                                        self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value20785, _, _, _) => {
                                                                                            source_ml_source2_data_property_value20785
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_constructor_field20787 = 0f64;
                                                                                        {
                                                                                            let source_ml_source2_constructor_field20788 = self::ml_string::Text::from_units(vec![]);
                                                                                            {
                                                                                                let source_ml_source2_constructor_field20790 = Some(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values579 = { let ml_reference_value580 = bindings.borrow().clone(); ml_reference_value580 }; let ml_callback579 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value20789, _) => {
                                                                                                        source_ml_source2_data_property_value20789
                                                                                                    }
                                                                                                } }); ml_values579.iter().cloned().enumerate().map(|(ml_index579, ml_value579)| ml_callback579(ml_value579)).collect::<Vec<_>>() })));
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field20872 = ((match definition.clone() {
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
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_constructor_field20778.clone()), Box::new(source_ml_source2_constructor_field20779.clone()), source_ml_source2_constructor_field20780.clone(), source_ml_source2_constructor_field20786, source_ml_source2_constructor_field20787, source_ml_source2_constructor_field20788.clone(), source_ml_source2_constructor_field20872, Some((source_ml_source2_constructor_field20790.clone(),)), Some((source_ml_source2_constructor_field20782.clone(),)), Some((source_ml_source2_constructor_field20784.clone(),)))
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
                                                                    let source_ml_source2_s21233 = match definition.clone() {
                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, base_class, _, definition_message, fields) => {
                                                                            let definition_message = *definition_message;
                                                                            match owner.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, identity, _, prototype, base_constructor, base_realm) => {
                                                                                    let prototype = *prototype;
                                                                                    let base_constructor = *base_constructor;
                                                                                    if !(self::ml_array::includes(&{ let ml_reference_value581 = (match registry.clone() {
                                                                                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value21268, _) => {
                                                                                            source_ml_source2_data_property_value21268.clone()
                                                                                        }
                                                                                    }).borrow().clone(); ml_reference_value581 }, &base_realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 115, 117, 98, 99, 108, 97, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 97, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                                    } else {
                                                                                        {
                                                                                            let message = self::lower_java_script_error_expression(definition_message.clone(), context.clone(), 0f64);
                                                                                            {
                                                                                                let context_2 = match message.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21269) => {
                                                                                                        let source_ml_source2_data_property_value21269 = *source_ml_source2_data_property_value21269;
                                                                                                        source_ml_source2_data_property_value21269.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let message_identity = match context_2.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21270, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21270
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let context_3 = {
                                                                                                            let source_ml_source2_nominal_field21273 = match context_2.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value21274, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    let source_ml_source2_data_property_value21274 = *source_ml_source2_data_property_value21274;
                                                                                                                    source_ml_source2_data_property_value21274.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field21275 = match context_2.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value21276, _, _, _, _, _, _, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value21276 = *source_ml_source2_data_property_value21276;
                                                                                                                        source_ml_source2_data_property_value21276.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field21277 = match context_2.clone() {
                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value21278, _, _, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value21278.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field21279 = match context_2.clone() {
                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21280, _, _, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value21280
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field21281 = match context_2.clone() {
                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value21282, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value21282
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field21283 = match context_2.clone() {
                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21284, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value21284.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field21285 = match context_2.clone() {
                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value21286, _, _, _) => {
                                                                                                                                            source_ml_source2_data_property_value21286
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field21287 = match context_2.clone() {
                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value21288, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value21288.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field21289 = match context_2.clone() {
                                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value21290, _) => {
                                                                                                                                                    source_ml_source2_data_property_value21290.clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field21291 = match context_2.clone() {
                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21292) => {
                                                                                                                                                        source_ml_source2_data_property_value21292.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override21293 = (match context_2.clone() {
                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21271, _, _, _, _, _, _) => {
                                                                                                                                                            source_ml_source2_data_property_value21271
                                                                                                                                                        }
                                                                                                                                                    } + 1f64);
                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field21273.clone()), Box::new(source_ml_source2_nominal_field21275.clone()), source_ml_source2_nominal_field21277.clone(), source_ml_source2_nominal_override21293, source_ml_source2_nominal_field21281, source_ml_source2_nominal_field21283.clone(), source_ml_source2_nominal_field21285, source_ml_source2_nominal_field21287.clone(), source_ml_source2_nominal_field21289.clone(), source_ml_source2_nominal_field21291.clone())
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
                                                                                                                                        let at = ({ let ml_reference_array582 = writes.clone(); let ml_reference_projection582 = ml_reference_array582.borrow().len() as f64; ml_reference_projection582 } - 1f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s21294 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop183(context_4.clone(), source_2.clone(), writes.clone(), at);
                                                                                                                                            match source_ml_source2_s21294.clone() {
                                                                                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Done(context_5, source_3) => {
                                                                                                                                                    let context_5 = *context_5;
                                                                                                                                                    let source_3 = *source_3;
                                                                                                                                                    {
                                                                                                                                                        let super_identity = match context_5.clone() {
                                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21295, _, _, _, _, _, _) => {
                                                                                                                                                                source_ml_source2_data_property_value21295
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let context_6 = {
                                                                                                                                                                let source_ml_source2_nominal_field21297 = match context_5.clone() {
                                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value21298, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                        let source_ml_source2_data_property_value21298 = *source_ml_source2_data_property_value21298;
                                                                                                                                                                        source_ml_source2_data_property_value21298.clone()
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field21299 = match context_5.clone() {
                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value21300, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value21300 = *source_ml_source2_data_property_value21300;
                                                                                                                                                                            source_ml_source2_data_property_value21300.clone()
                                                                                                                                                                        }
                                                                                                                                                                    };
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field21301 = match context_5.clone() {
                                                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value21302, _, _, _, _, _, _, _) => {
                                                                                                                                                                                source_ml_source2_data_property_value21302.clone()
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field21303 = match context_5.clone() {
                                                                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21304, _, _, _, _, _, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value21304
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_field21305 = match context_5.clone() {
                                                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value21306, _, _, _, _, _) => {
                                                                                                                                                                                        source_ml_source2_data_property_value21306
                                                                                                                                                                                    }
                                                                                                                                                                                };
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_nominal_field21307 = match context_5.clone() {
                                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21308, _, _, _, _) => {
                                                                                                                                                                                            source_ml_source2_data_property_value21308.clone()
                                                                                                                                                                                        }
                                                                                                                                                                                    };
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source2_nominal_field21309 = match context_5.clone() {
                                                                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value21310, _, _, _) => {
                                                                                                                                                                                                source_ml_source2_data_property_value21310
                                                                                                                                                                                            }
                                                                                                                                                                                        };
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_nominal_field21311 = match context_5.clone() {
                                                                                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value21312, _, _) => {
                                                                                                                                                                                                    source_ml_source2_data_property_value21312.clone()
                                                                                                                                                                                                }
                                                                                                                                                                                            };
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source2_nominal_field21313 = match context_5.clone() {
                                                                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value21314, _) => {
                                                                                                                                                                                                        source_ml_source2_data_property_value21314.clone()
                                                                                                                                                                                                    }
                                                                                                                                                                                                };
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_nominal_field21315 = match context_5.clone() {
                                                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21316) => {
                                                                                                                                                                                                            source_ml_source2_data_property_value21316.clone()
                                                                                                                                                                                                        }
                                                                                                                                                                                                    };
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_nominal_override21317 = (super_identity + 1f64);
                                                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field21297.clone()), Box::new(source_ml_source2_nominal_field21299.clone()), source_ml_source2_nominal_field21301.clone(), source_ml_source2_nominal_override21317, source_ml_source2_nominal_field21305, source_ml_source2_nominal_field21307.clone(), source_ml_source2_nominal_field21309, source_ml_source2_nominal_field21311.clone(), source_ml_source2_nominal_field21313.clone(), source_ml_source2_nominal_field21315.clone())
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
                                                                                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21318, _) => {
                                                                                                                                                                        let source_ml_source2_data_property_value21318 = *source_ml_source2_data_property_value21318;
                                                                                                                                                                        source_ml_source2_data_property_value21318.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21372) => {
                                                                                                let source_ml_source2_data_property_value21372 = *source_ml_source2_data_property_value21372;
                                                                                                source_ml_source2_data_property_value21372.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_5 = match body.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21373, _) => {
                                                                                                    let source_ml_source2_data_property_value21373 = *source_ml_source2_data_property_value21373;
                                                                                                    source_ml_source2_data_property_value21373.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21427) => {
                                                                                                let source_ml_source2_data_property_value21427 = *source_ml_source2_data_property_value21427;
                                                                                                source_ml_source2_data_property_value21427.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_6 = match body_2.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21428, _) => {
                                                                                                    let source_ml_source2_data_property_value21428 = *source_ml_source2_data_property_value21428;
                                                                                                    source_ml_source2_data_property_value21428.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21482) => {
                                                                                                let source_ml_source2_data_property_value21482 = *source_ml_source2_data_property_value21482;
                                                                                                source_ml_source2_data_property_value21482.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_7 = match body_3.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21483, _) => {
                                                                                                    let source_ml_source2_data_property_value21483 = *source_ml_source2_data_property_value21483;
                                                                                                    source_ml_source2_data_property_value21483.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21537) => {
                                                                                                let source_ml_source2_data_property_value21537 = *source_ml_source2_data_property_value21537;
                                                                                                source_ml_source2_data_property_value21537.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_8 = match body_4.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21538, _) => {
                                                                                                    let source_ml_source2_data_property_value21538 = *source_ml_source2_data_property_value21538;
                                                                                                    source_ml_source2_data_property_value21538.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21592) => {
                                                                                                let source_ml_source2_data_property_value21592 = *source_ml_source2_data_property_value21592;
                                                                                                source_ml_source2_data_property_value21592.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_9 = match body_5.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21593, _) => {
                                                                                                    let source_ml_source2_data_property_value21593 = *source_ml_source2_data_property_value21593;
                                                                                                    source_ml_source2_data_property_value21593.clone()
                                                                                                }
                                                                                            };
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(Box::new(context_11.clone()), Box::new(source_9.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    match source_ml_source2_s21233.clone() {
                                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(context_12, source_10) => {
                                                                            let context_12 = *context_12;
                                                                            let source_10 = *source_10;
                                                                            {
                                                                                let initialized = self::lower_java_script_source_parameters(match definition.clone() {
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21594, _, _) => {
                                                                                        source_ml_source2_data_property_value21594.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21595, _, _) => {
                                                                                        source_ml_source2_data_property_value21595.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21596, _) => {
                                                                                        source_ml_source2_data_property_value21596.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21597, _) => {
                                                                                        source_ml_source2_data_property_value21597.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21598, _) => {
                                                                                        source_ml_source2_data_property_value21598.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21599, _) => {
                                                                                        source_ml_source2_data_property_value21599.clone()
                                                                                    }
                                                                                }, context_12.clone(), source_10.clone());
                                                                                {
                                                                                    let context_13 = match initialized.clone() {
                                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21600) => {
                                                                                            let source_ml_source2_data_property_value21600 = *source_ml_source2_data_property_value21600;
                                                                                            source_ml_source2_data_property_value21600.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_11 = match initialized.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21601, _) => {
                                                                                                let source_ml_source2_data_property_value21601 = *source_ml_source2_data_property_value21601;
                                                                                                source_ml_source2_data_property_value21601.clone()
                                                                                            }
                                                                                        };
                                                                                        if ((match context_13.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21603, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value21603.clone()
                                                                                            }
                                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), match context_13.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21604, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value21604.clone()
                                                                                                }
                                                                                            })))
                                                                                        } else {
                                                                                            {
                                                                                                let code = if match context_13.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value21605, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value21605
                                                                                                    }
                                                                                                } {
                                                                                                    self::compile_java_script_mutable_call_source_with_captures(source_11.clone(), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values583 = { let ml_reference_value584 = (match definition.clone() {
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21606, _, _) => {
                                                                                                            source_ml_source2_data_property_value21606.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21607, _, _) => {
                                                                                                            source_ml_source2_data_property_value21607.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21608, _) => {
                                                                                                            source_ml_source2_data_property_value21608.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21609, _) => {
                                                                                                            source_ml_source2_data_property_value21609.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21610, _) => {
                                                                                                            source_ml_source2_data_property_value21610.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21611, _) => {
                                                                                                            source_ml_source2_data_property_value21611.clone()
                                                                                                        }
                                                                                                    }).borrow().clone(); ml_reference_value584 }; let ml_callback583 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> f64 { match parameter.clone() {
                                                                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value21612, _, _) => {
                                                                                                            source_ml_source2_data_property_value21612
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value21613, _) => {
                                                                                                            source_ml_source2_data_property_value21613
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value21614, _, _) => {
                                                                                                            source_ml_source2_data_property_value21614
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value21615, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21615
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value21616, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21616
                                                                                                        }
                                                                                                    } }); ml_values583.iter().cloned().enumerate().map(|(ml_index583, ml_value583)| ml_callback583(ml_value583)).collect::<Vec<_>>() })), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values585 = { let ml_reference_value586 = bindings.borrow().clone(); ml_reference_value586 }; let ml_callback585 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                        self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21617, _) => {
                                                                                                            source_ml_source2_data_property_value21617
                                                                                                        }
                                                                                                    } }); ml_values585.iter().cloned().enumerate().map(|(ml_index585, ml_value585)| ml_callback585(ml_value585)).collect::<Vec<_>>() })))
                                                                                                } else {
                                                                                                    self::compile_java_script_call_source_with_captures(source_11.clone(), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values587 = { let ml_reference_value588 = (match definition.clone() {
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21618, _, _) => {
                                                                                                            source_ml_source2_data_property_value21618.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21619, _, _) => {
                                                                                                            source_ml_source2_data_property_value21619.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21620, _) => {
                                                                                                            source_ml_source2_data_property_value21620.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21621, _) => {
                                                                                                            source_ml_source2_data_property_value21621.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21622, _) => {
                                                                                                            source_ml_source2_data_property_value21622.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21623, _) => {
                                                                                                            source_ml_source2_data_property_value21623.clone()
                                                                                                        }
                                                                                                    }).borrow().clone(); ml_reference_value588 }; let ml_callback587 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> f64 { match parameter.clone() {
                                                                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value21624, _, _) => {
                                                                                                            source_ml_source2_data_property_value21624
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value21625, _) => {
                                                                                                            source_ml_source2_data_property_value21625
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value21626, _, _) => {
                                                                                                            source_ml_source2_data_property_value21626
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value21627, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21627
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value21628, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21628
                                                                                                        }
                                                                                                    } }); ml_values587.iter().cloned().enumerate().map(|(ml_index587, ml_value587)| ml_callback587(ml_value587)).collect::<Vec<_>>() })), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values589 = { let ml_reference_value590 = bindings.borrow().clone(); ml_reference_value590 }; let ml_callback589 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                        self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21629, _) => {
                                                                                                            source_ml_source2_data_property_value21629
                                                                                                        }
                                                                                                    } }); ml_values589.iter().cloned().enumerate().map(|(ml_index589, ml_value589)| ml_callback589(ml_value589)).collect::<Vec<_>>() })))
                                                                                                };
                                                                                                if ((match code.clone() {
                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21631) => {
                                                                                                        source_ml_source2_data_property_value21631.clone()
                                                                                                    }
                                                                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), match code.clone() {
                                                                                                        self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21632) => {
                                                                                                            source_ml_source2_data_property_value21632.clone()
                                                                                                        }
                                                                                                    })))
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored180 = { let ml_method_owner591 = functions.clone(); let ml_method_argument591_0 = {
                                                                                                            let source_ml_source2_constructor_field21636 = match owner.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value21633, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value21633.clone()
                                                                                                                }
                                                                                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value21634, _, _) => {
                                                                                                                    source_ml_source2_data_property_value21634.clone()
                                                                                                                }
                                                                                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value21635, _) => {
                                                                                                                    source_ml_source2_data_property_value21635.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_constructor_field21640 = match owner.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value21637, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value21637.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value21638, _) => {
                                                                                                                        source_ml_source2_data_property_value21638.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value21639) => {
                                                                                                                        source_ml_source2_data_property_value21639.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_constructor_field21695 = if match definition.clone() {
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
                                                                                                                        let source_ml_source2_constructor_field21696 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                        {
                                                                                                                            let source_ml_source2_constructor_field21703 = { let ml_reference_array592 = match definition.clone() {
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21697, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value21697.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21698, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value21698.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21699, _) => {
                                                                                                                                    source_ml_source2_data_property_value21699.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21700, _) => {
                                                                                                                                    source_ml_source2_data_property_value21700.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21701, _) => {
                                                                                                                                    source_ml_source2_data_property_value21701.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21702, _) => {
                                                                                                                                    source_ml_source2_data_property_value21702.clone()
                                                                                                                                }
                                                                                                                            }; let ml_reference_projection592 = ml_reference_array592.borrow().len() as f64; ml_reference_projection592 };
                                                                                                                            {
                                                                                                                                let source_ml_source2_constructor_field21705 = match code.clone() {
                                                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, source_ml_source2_data_property_value21704, _) => {
                                                                                                                                        source_ml_source2_data_property_value21704
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_constructor_field21707 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values593 = { let ml_reference_value594 = bindings.borrow().clone(); ml_reference_value594 }; let ml_callback593 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> self::JavaScriptSemanticValue { match binding.clone() {
                                                                                                                                        self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(_, source_ml_source2_data_property_value21706) => {
                                                                                                                                            let source_ml_source2_data_property_value21706 = *source_ml_source2_data_property_value21706;
                                                                                                                                            source_ml_source2_data_property_value21706.clone()
                                                                                                                                        }
                                                                                                                                    } }); ml_values593.iter().cloned().enumerate().map(|(ml_index593, ml_value593)| ml_callback593(ml_value593)).collect::<Vec<_>>() }));
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_constructor_field21709 = match code.clone() {
                                                                                                                                            self::JavaScriptCallSourceCompilation::SourceCompiled(source_ml_source2_data_property_value21708, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value21708.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source2_constructor_field21636.clone(), source_ml_source2_constructor_field21640.clone(), source_ml_source2_constructor_field21695.clone(), Box::new(source_ml_source2_constructor_field21696.clone()), source_ml_source2_constructor_field21703, source_ml_source2_constructor_field21705, source_ml_source2_constructor_field21707.clone(), source_ml_source2_constructor_field21709.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }; let ml_inserted = vec![ml_method_argument591_0]; let mut ml_method_values591 = ml_method_owner591.borrow_mut(); let ml_length = ml_method_values591.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values591.extend(ml_inserted); ml_length as f64 };
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
