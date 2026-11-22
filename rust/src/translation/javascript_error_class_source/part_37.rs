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
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value20889, _, _, _, _) => {
                            source_ml_source2_data_property_value20889.clone()
                        }
                    }).borrow().clone(); ml_reference_value572 }; let ml_callback571 = { let definition = definition.clone(); std::rc::Rc::new(move |owner: self::JavaScriptErrorFunctionOwner| -> bool { ((match owner.clone() {
                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value20899, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20899
                        }
                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value20900, _, _, _) => {
                            source_ml_source2_data_property_value20900
                        }
                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value20901, _, _) => {
                            source_ml_source2_data_property_value20901
                        }
                    }) == (match definition.clone() {
                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(source_ml_source2_data_property_value20902, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20902
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(source_ml_source2_data_property_value20903, _, _, _, _) => {
                            source_ml_source2_data_property_value20903
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(source_ml_source2_data_property_value20904, _, _, _) => {
                            source_ml_source2_data_property_value20904
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(source_ml_source2_data_property_value20905, _, _, _) => {
                            source_ml_source2_data_property_value20905
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(source_ml_source2_data_property_value20906, _, _, _) => {
                            source_ml_source2_data_property_value20906
                        }
                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(source_ml_source2_data_property_value20907, _, _, _) => {
                            source_ml_source2_data_property_value20907
                        }
                    })) }) }; ml_values571.iter().cloned().enumerate().filter_map(|(ml_index571, ml_value571)| if ml_callback571(ml_value571.clone()) { Some(ml_value571) } else { None }).collect::<Vec<_>>() }));
                    if (({ let ml_reference_array573 = matches.clone(); let ml_reference_projection573 = ml_reference_array573.borrow().len() as f64; ml_reference_projection573 }) != (1f64)) {
                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 101, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114]))))
                    } else {
                        {
                            let owner = {
                                let source_ml_source2_array_read_index20909 = 0f64;
                                { let ml_reference_array574 = matches.clone(); let ml_reference_position574 = source_ml_source2_array_read_index20909; let ml_reference_projection574 = self::ml_array::optional_at(&*ml_reference_array574.borrow(), self::ml_array::number_index(ml_reference_position574)); ml_reference_projection574 }
                            };
                            if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                            } else {
                                if (self::ml_array::includes(&{ let ml_reference_value575 = identities.borrow().clone(); ml_reference_value575 }, &match owner.clone().expect("a proven present value") {
                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value20910, _, _, _, _) => {
                                        source_ml_source2_data_property_value20910.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value20911, _, _) => {
                                        source_ml_source2_data_property_value20911.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value20912, _) => {
                                        source_ml_source2_data_property_value20912.clone()
                                    }
                                }, 0f64, false, |a, b| a == b) || !(self::ml_array::includes(&{ let ml_reference_value576 = (match registry.clone() {
                                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value20914, _) => {
                                        source_ml_source2_data_property_value20914.clone()
                                    }
                                }).borrow().clone(); ml_reference_value576 }, &match owner.clone().expect("a proven present value") {
                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value20915, _, _, _) => {
                                        source_ml_source2_data_property_value20915.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value20916, _) => {
                                        source_ml_source2_data_property_value20916.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value20917) => {
                                        source_ml_source2_data_property_value20917.clone()
                                    }
                                }, 0f64, false, |a, b| a == b))) {
                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 115]))))
                                } else {
                                    {
                                        let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored178 = { let ml_method_owner577 = identities.clone(); let ml_method_argument577_0 = match owner.clone().expect("a proven present value") {
                                            self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value20918, _, _, _, _) => {
                                                source_ml_source2_data_property_value20918.clone()
                                            }
                                            self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value20919, _, _) => {
                                                source_ml_source2_data_property_value20919.clone()
                                            }
                                            self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value20920, _) => {
                                                source_ml_source2_data_property_value20920.clone()
                                            }
                                        }; let ml_inserted = vec![ml_method_argument577_0]; let mut ml_method_values577 = ml_method_owner577.borrow_mut(); let ml_length = ml_method_values577.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values577.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let operations = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values578 = { let ml_reference_value579 = (match registry.clone() {
                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, source_ml_source2_data_property_value20921, _, _, _) => {
                                                    source_ml_source2_data_property_value20921.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value579 }; let ml_callback578 = { let owner = owner.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorRealmOperations| -> bool { ((match row.clone() {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20926, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20926.clone()
                                                }
                                            }) == (match owner.clone().expect("a proven present value") {
                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value20927, _, _, _) => {
                                                    source_ml_source2_data_property_value20927.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value20928, _) => {
                                                    source_ml_source2_data_property_value20928.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value20929) => {
                                                    source_ml_source2_data_property_value20929.clone()
                                                }
                                            })) }) }; ml_values578.iter().cloned().enumerate().filter_map(|(ml_index578, ml_value578)| if ml_callback578(ml_value578.clone()) { Some(ml_value578) } else { None }).collect::<Vec<_>>() }));
                                            if (({ let ml_reference_array580 = operations.clone(); let ml_reference_projection580 = ml_reference_array580.borrow().len() as f64; ml_reference_projection580 }) != (1f64)) {
                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))))
                                            } else {
                                                {
                                                    let realm = {
                                                        let source_ml_source2_array_read_index20931 = 0f64;
                                                        { let ml_reference_array581 = operations.clone(); let ml_reference_position581 = source_ml_source2_array_read_index20931; let ml_reference_projection581 = self::ml_array::optional_at(&*ml_reference_array581.borrow(), self::ml_array::number_index(ml_reference_position581)); ml_reference_projection581 }
                                                    };
                                                    if match (realm.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109, 32, 114, 111, 119, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))))
                                                    } else {
                                                        {
                                                            let context = {
                                                                let source_ml_source2_constructor_field20932 = registry.clone();
                                                                {
                                                                    let source_ml_source2_constructor_field20933 = realm.clone().expect("a proven present value");
                                                                    {
                                                                        let source_ml_source2_constructor_field20934 = capabilities.clone();
                                                                        {
                                                                            let source_ml_source2_constructor_field20936 = Some((match plan.clone() {
                                                                                self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, source_ml_source2_data_property_value20935, _) => {
                                                                                    ((source_ml_source2_data_property_value20935.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                                                            {
                                                                                let source_ml_source2_constructor_field20938 = Some((match plan.clone() {
                                                                                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, _, source_ml_source2_data_property_value20937) => {
                                                                                        ((source_ml_source2_data_property_value20937.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new()))));
                                                                                {
                                                                                    let source_ml_source2_constructor_field20940 = match plan.clone() {
                                                                                        self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value20939, _, _, _) => {
                                                                                            source_ml_source2_data_property_value20939
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_constructor_field20941 = 0f64;
                                                                                        {
                                                                                            let source_ml_source2_constructor_field20942 = self::ml_string::Text::from_units(vec![]);
                                                                                            {
                                                                                                let source_ml_source2_constructor_field20944 = Some(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values582 = { let ml_reference_value583 = bindings.borrow().clone(); ml_reference_value583 }; let ml_callback582 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value20943, _) => {
                                                                                                        source_ml_source2_data_property_value20943
                                                                                                    }
                                                                                                } }); ml_values582.iter().cloned().enumerate().map(|(ml_index582, ml_value582)| ml_callback582(ml_value582)).collect::<Vec<_>>() })));
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field21026 = ((match definition.clone() {
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
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_constructor_field20932.clone()), Box::new(source_ml_source2_constructor_field20933.clone()), source_ml_source2_constructor_field20934.clone(), source_ml_source2_constructor_field20940, source_ml_source2_constructor_field20941, source_ml_source2_constructor_field20942.clone(), source_ml_source2_constructor_field21026, Some((source_ml_source2_constructor_field20944.clone(),)), Some((source_ml_source2_constructor_field20936.clone(),)), Some((source_ml_source2_constructor_field20938.clone(),)))
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
                                                                    let source_ml_source2_s21387 = match definition.clone() {
                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, base_class, _, definition_message, fields) => {
                                                                            let definition_message = *definition_message;
                                                                            match owner.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, identity, _, prototype, base_constructor, base_realm) => {
                                                                                    let prototype = *prototype;
                                                                                    let base_constructor = *base_constructor;
                                                                                    if !(self::ml_array::includes(&{ let ml_reference_value584 = (match registry.clone() {
                                                                                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value21422, _) => {
                                                                                            source_ml_source2_data_property_value21422.clone()
                                                                                        }
                                                                                    }).borrow().clone(); ml_reference_value584 }, &base_realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 115, 117, 98, 99, 108, 97, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 97, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))))
                                                                                    } else {
                                                                                        {
                                                                                            let message = self::lower_java_script_error_expression(definition_message.clone(), context.clone(), 0f64);
                                                                                            {
                                                                                                let context_2 = match message.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21423) => {
                                                                                                        let source_ml_source2_data_property_value21423 = *source_ml_source2_data_property_value21423;
                                                                                                        source_ml_source2_data_property_value21423.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let message_identity = match context_2.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21424, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21424
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let context_3 = {
                                                                                                            let source_ml_source2_nominal_field21427 = match context_2.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value21428, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    let source_ml_source2_data_property_value21428 = *source_ml_source2_data_property_value21428;
                                                                                                                    source_ml_source2_data_property_value21428.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field21429 = match context_2.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value21430, _, _, _, _, _, _, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value21430 = *source_ml_source2_data_property_value21430;
                                                                                                                        source_ml_source2_data_property_value21430.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field21431 = match context_2.clone() {
                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value21432, _, _, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value21432.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field21433 = match context_2.clone() {
                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21434, _, _, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value21434
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field21435 = match context_2.clone() {
                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value21436, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value21436
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field21437 = match context_2.clone() {
                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21438, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value21438.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field21439 = match context_2.clone() {
                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value21440, _, _, _) => {
                                                                                                                                            source_ml_source2_data_property_value21440
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field21441 = match context_2.clone() {
                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value21442, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value21442.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field21443 = match context_2.clone() {
                                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value21444, _) => {
                                                                                                                                                    source_ml_source2_data_property_value21444.clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field21445 = match context_2.clone() {
                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21446) => {
                                                                                                                                                        source_ml_source2_data_property_value21446.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override21447 = (match context_2.clone() {
                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21425, _, _, _, _, _, _) => {
                                                                                                                                                            source_ml_source2_data_property_value21425
                                                                                                                                                        }
                                                                                                                                                    } + 1f64);
                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field21427.clone()), Box::new(source_ml_source2_nominal_field21429.clone()), source_ml_source2_nominal_field21431.clone(), source_ml_source2_nominal_override21447, source_ml_source2_nominal_field21435, source_ml_source2_nominal_field21437.clone(), source_ml_source2_nominal_field21439, source_ml_source2_nominal_field21441.clone(), source_ml_source2_nominal_field21443.clone(), source_ml_source2_nominal_field21445.clone())
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
                                                                                                                                            let source_ml_source2_s21448 = self::source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop183(context_4.clone(), source_2.clone(), writes.clone(), at);
                                                                                                                                            match source_ml_source2_s21448.clone() {
                                                                                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Done(context_5, source_3) => {
                                                                                                                                                    let context_5 = *context_5;
                                                                                                                                                    let source_3 = *source_3;
                                                                                                                                                    {
                                                                                                                                                        let super_identity = match context_5.clone() {
                                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21449, _, _, _, _, _, _) => {
                                                                                                                                                                source_ml_source2_data_property_value21449
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let context_6 = {
                                                                                                                                                                let source_ml_source2_nominal_field21451 = match context_5.clone() {
                                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value21452, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                        let source_ml_source2_data_property_value21452 = *source_ml_source2_data_property_value21452;
                                                                                                                                                                        source_ml_source2_data_property_value21452.clone()
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field21453 = match context_5.clone() {
                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value21454, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value21454 = *source_ml_source2_data_property_value21454;
                                                                                                                                                                            source_ml_source2_data_property_value21454.clone()
                                                                                                                                                                        }
                                                                                                                                                                    };
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field21455 = match context_5.clone() {
                                                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value21456, _, _, _, _, _, _, _) => {
                                                                                                                                                                                source_ml_source2_data_property_value21456.clone()
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field21457 = match context_5.clone() {
                                                                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value21458, _, _, _, _, _, _) => {
                                                                                                                                                                                    source_ml_source2_data_property_value21458
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_field21459 = match context_5.clone() {
                                                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value21460, _, _, _, _, _) => {
                                                                                                                                                                                        source_ml_source2_data_property_value21460
                                                                                                                                                                                    }
                                                                                                                                                                                };
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_nominal_field21461 = match context_5.clone() {
                                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21462, _, _, _, _) => {
                                                                                                                                                                                            source_ml_source2_data_property_value21462.clone()
                                                                                                                                                                                        }
                                                                                                                                                                                    };
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source2_nominal_field21463 = match context_5.clone() {
                                                                                                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value21464, _, _, _) => {
                                                                                                                                                                                                source_ml_source2_data_property_value21464
                                                                                                                                                                                            }
                                                                                                                                                                                        };
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_nominal_field21465 = match context_5.clone() {
                                                                                                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value21466, _, _) => {
                                                                                                                                                                                                    source_ml_source2_data_property_value21466.clone()
                                                                                                                                                                                                }
                                                                                                                                                                                            };
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source2_nominal_field21467 = match context_5.clone() {
                                                                                                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value21468, _) => {
                                                                                                                                                                                                        source_ml_source2_data_property_value21468.clone()
                                                                                                                                                                                                    }
                                                                                                                                                                                                };
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_nominal_field21469 = match context_5.clone() {
                                                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21470) => {
                                                                                                                                                                                                            source_ml_source2_data_property_value21470.clone()
                                                                                                                                                                                                        }
                                                                                                                                                                                                    };
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_nominal_override21471 = (super_identity + 1f64);
                                                                                                                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field21451.clone()), Box::new(source_ml_source2_nominal_field21453.clone()), source_ml_source2_nominal_field21455.clone(), source_ml_source2_nominal_override21471, source_ml_source2_nominal_field21459, source_ml_source2_nominal_field21461.clone(), source_ml_source2_nominal_field21463, source_ml_source2_nominal_field21465.clone(), source_ml_source2_nominal_field21467.clone(), source_ml_source2_nominal_field21469.clone())
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
                                                                                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21472, _) => {
                                                                                                                                                                        let source_ml_source2_data_property_value21472 = *source_ml_source2_data_property_value21472;
                                                                                                                                                                        source_ml_source2_data_property_value21472.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21526) => {
                                                                                                let source_ml_source2_data_property_value21526 = *source_ml_source2_data_property_value21526;
                                                                                                source_ml_source2_data_property_value21526.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_5 = match body.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21527, _) => {
                                                                                                    let source_ml_source2_data_property_value21527 = *source_ml_source2_data_property_value21527;
                                                                                                    source_ml_source2_data_property_value21527.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21581) => {
                                                                                                let source_ml_source2_data_property_value21581 = *source_ml_source2_data_property_value21581;
                                                                                                source_ml_source2_data_property_value21581.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_6 = match body_2.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21582, _) => {
                                                                                                    let source_ml_source2_data_property_value21582 = *source_ml_source2_data_property_value21582;
                                                                                                    source_ml_source2_data_property_value21582.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21636) => {
                                                                                                let source_ml_source2_data_property_value21636 = *source_ml_source2_data_property_value21636;
                                                                                                source_ml_source2_data_property_value21636.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_7 = match body_3.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21637, _) => {
                                                                                                    let source_ml_source2_data_property_value21637 = *source_ml_source2_data_property_value21637;
                                                                                                    source_ml_source2_data_property_value21637.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21691) => {
                                                                                                let source_ml_source2_data_property_value21691 = *source_ml_source2_data_property_value21691;
                                                                                                source_ml_source2_data_property_value21691.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_8 = match body_4.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21692, _) => {
                                                                                                    let source_ml_source2_data_property_value21692 = *source_ml_source2_data_property_value21692;
                                                                                                    source_ml_source2_data_property_value21692.clone()
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
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21746) => {
                                                                                                let source_ml_source2_data_property_value21746 = *source_ml_source2_data_property_value21746;
                                                                                                source_ml_source2_data_property_value21746.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_9 = match body_5.clone() {
                                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21747, _) => {
                                                                                                    let source_ml_source2_data_property_value21747 = *source_ml_source2_data_property_value21747;
                                                                                                    source_ml_source2_data_property_value21747.clone()
                                                                                                }
                                                                                            };
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(Box::new(context_11.clone()), Box::new(source_9.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    match source_ml_source2_s21387.clone() {
                                                                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(context_12, source_10) => {
                                                                            let context_12 = *context_12;
                                                                            let source_10 = *source_10;
                                                                            {
                                                                                let initialized = self::lower_java_script_source_parameters(match definition.clone() {
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21748, _, _) => {
                                                                                        source_ml_source2_data_property_value21748.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21749, _, _) => {
                                                                                        source_ml_source2_data_property_value21749.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21750, _) => {
                                                                                        source_ml_source2_data_property_value21750.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21751, _) => {
                                                                                        source_ml_source2_data_property_value21751.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21752, _) => {
                                                                                        source_ml_source2_data_property_value21752.clone()
                                                                                    }
                                                                                    self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21753, _) => {
                                                                                        source_ml_source2_data_property_value21753.clone()
                                                                                    }
                                                                                }, context_12.clone(), source_10.clone());
                                                                                {
                                                                                    let context_13 = match initialized.clone() {
                                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21754) => {
                                                                                            let source_ml_source2_data_property_value21754 = *source_ml_source2_data_property_value21754;
                                                                                            source_ml_source2_data_property_value21754.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_11 = match initialized.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21755, _) => {
                                                                                                let source_ml_source2_data_property_value21755 = *source_ml_source2_data_property_value21755;
                                                                                                source_ml_source2_data_property_value21755.clone()
                                                                                            }
                                                                                        };
                                                                                        if ((match context_13.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21757, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value21757.clone()
                                                                                            }
                                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), match context_13.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21758, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value21758.clone()
                                                                                                }
                                                                                            })))
                                                                                        } else {
                                                                                            {
                                                                                                let code = if match context_13.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value21759, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value21759
                                                                                                    }
                                                                                                } {
                                                                                                    self::compile_java_script_mutable_call_source_with_captures(source_11.clone(), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values586 = { let ml_reference_value587 = (match definition.clone() {
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21760, _, _) => {
                                                                                                            source_ml_source2_data_property_value21760.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21761, _, _) => {
                                                                                                            source_ml_source2_data_property_value21761.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21762, _) => {
                                                                                                            source_ml_source2_data_property_value21762.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21763, _) => {
                                                                                                            source_ml_source2_data_property_value21763.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21764, _) => {
                                                                                                            source_ml_source2_data_property_value21764.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21765, _) => {
                                                                                                            source_ml_source2_data_property_value21765.clone()
                                                                                                        }
                                                                                                    }).borrow().clone(); ml_reference_value587 }; let ml_callback586 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> f64 { match parameter.clone() {
                                                                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value21766, _, _) => {
                                                                                                            source_ml_source2_data_property_value21766
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value21767, _) => {
                                                                                                            source_ml_source2_data_property_value21767
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value21768, _, _) => {
                                                                                                            source_ml_source2_data_property_value21768
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value21769, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21769
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value21770, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21770
                                                                                                        }
                                                                                                    } }); ml_values586.iter().cloned().enumerate().map(|(ml_index586, ml_value586)| ml_callback586(ml_value586)).collect::<Vec<_>>() })), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values588 = { let ml_reference_value589 = bindings.borrow().clone(); ml_reference_value589 }; let ml_callback588 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                        self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21771, _) => {
                                                                                                            source_ml_source2_data_property_value21771
                                                                                                        }
                                                                                                    } }); ml_values588.iter().cloned().enumerate().map(|(ml_index588, ml_value588)| ml_callback588(ml_value588)).collect::<Vec<_>>() })))
                                                                                                } else {
                                                                                                    self::compile_java_script_call_source_with_captures(source_11.clone(), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values590 = { let ml_reference_value591 = (match definition.clone() {
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21772, _, _) => {
                                                                                                            source_ml_source2_data_property_value21772.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21773, _, _) => {
                                                                                                            source_ml_source2_data_property_value21773.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21774, _) => {
                                                                                                            source_ml_source2_data_property_value21774.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21775, _) => {
                                                                                                            source_ml_source2_data_property_value21775.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21776, _) => {
                                                                                                            source_ml_source2_data_property_value21776.clone()
                                                                                                        }
                                                                                                        self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21777, _) => {
                                                                                                            source_ml_source2_data_property_value21777.clone()
                                                                                                        }
                                                                                                    }).borrow().clone(); ml_reference_value591 }; let ml_callback590 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> f64 { match parameter.clone() {
                                                                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value21778, _, _) => {
                                                                                                            source_ml_source2_data_property_value21778
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value21779, _) => {
                                                                                                            source_ml_source2_data_property_value21779
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value21780, _, _) => {
                                                                                                            source_ml_source2_data_property_value21780
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value21781, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21781
                                                                                                        }
                                                                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value21782, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value21782
                                                                                                        }
                                                                                                    } }); ml_values590.iter().cloned().enumerate().map(|(ml_index590, ml_value590)| ml_callback590(ml_value590)).collect::<Vec<_>>() })), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values592 = { let ml_reference_value593 = bindings.borrow().clone(); ml_reference_value593 }; let ml_callback592 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                        self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21783, _) => {
                                                                                                            source_ml_source2_data_property_value21783
                                                                                                        }
                                                                                                    } }); ml_values592.iter().cloned().enumerate().map(|(ml_index592, ml_value592)| ml_callback592(ml_value592)).collect::<Vec<_>>() })))
                                                                                                };
                                                                                                if ((match code.clone() {
                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21785) => {
                                                                                                        source_ml_source2_data_property_value21785.clone()
                                                                                                    }
                                                                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), match code.clone() {
                                                                                                        self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21786) => {
                                                                                                            source_ml_source2_data_property_value21786.clone()
                                                                                                        }
                                                                                                    })))
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored180 = { let ml_method_owner594 = functions.clone(); let ml_method_argument594_0 = {
                                                                                                            let source_ml_source2_constructor_field21790 = match owner.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value21787, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value21787.clone()
                                                                                                                }
                                                                                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value21788, _, _) => {
                                                                                                                    source_ml_source2_data_property_value21788.clone()
                                                                                                                }
                                                                                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value21789, _) => {
                                                                                                                    source_ml_source2_data_property_value21789.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_constructor_field21794 = match owner.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value21791, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value21791.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value21792, _) => {
                                                                                                                        source_ml_source2_data_property_value21792.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value21793) => {
                                                                                                                        source_ml_source2_data_property_value21793.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_constructor_field21849 = if match definition.clone() {
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
                                                                                                                        let source_ml_source2_constructor_field21850 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                        {
                                                                                                                            let source_ml_source2_constructor_field21857 = { let ml_reference_array595 = match definition.clone() {
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, source_ml_source2_data_property_value21851, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value21851.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(_, _, source_ml_source2_data_property_value21852, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value21852.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(_, _, source_ml_source2_data_property_value21853, _) => {
                                                                                                                                    source_ml_source2_data_property_value21853.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(_, _, source_ml_source2_data_property_value21854, _) => {
                                                                                                                                    source_ml_source2_data_property_value21854.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(_, _, source_ml_source2_data_property_value21855, _) => {
                                                                                                                                    source_ml_source2_data_property_value21855.clone()
                                                                                                                                }
                                                                                                                                self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(_, _, source_ml_source2_data_property_value21856, _) => {
                                                                                                                                    source_ml_source2_data_property_value21856.clone()
                                                                                                                                }
                                                                                                                            }; let ml_reference_projection595 = ml_reference_array595.borrow().len() as f64; ml_reference_projection595 };
                                                                                                                            {
                                                                                                                                let source_ml_source2_constructor_field21859 = match code.clone() {
                                                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, source_ml_source2_data_property_value21858, _) => {
                                                                                                                                        source_ml_source2_data_property_value21858
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_constructor_field21861 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values596 = { let ml_reference_value597 = bindings.borrow().clone(); ml_reference_value597 }; let ml_callback596 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> self::JavaScriptSemanticValue { match binding.clone() {
                                                                                                                                        self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(_, source_ml_source2_data_property_value21860) => {
                                                                                                                                            let source_ml_source2_data_property_value21860 = *source_ml_source2_data_property_value21860;
                                                                                                                                            source_ml_source2_data_property_value21860.clone()
                                                                                                                                        }
                                                                                                                                    } }); ml_values596.iter().cloned().enumerate().map(|(ml_index596, ml_value596)| ml_callback596(ml_value596)).collect::<Vec<_>>() }));
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_constructor_field21863 = match code.clone() {
                                                                                                                                            self::JavaScriptCallSourceCompilation::SourceCompiled(source_ml_source2_data_property_value21862, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value21862.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source2_constructor_field21790.clone(), source_ml_source2_constructor_field21794.clone(), source_ml_source2_constructor_field21849.clone(), Box::new(source_ml_source2_constructor_field21850.clone()), source_ml_source2_constructor_field21857, source_ml_source2_constructor_field21859, source_ml_source2_constructor_field21861.clone(), source_ml_source2_constructor_field21863.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
