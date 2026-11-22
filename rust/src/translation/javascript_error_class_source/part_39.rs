// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallTemplate;
use super::JavaScriptErrorArrowTemplatePlan;
use super::JavaScriptErrorFunctionOwner;
use super::JavaScriptErrorFunctionPlan;
use super::JavaScriptErrorModuleBinding;
use super::JavaScriptErrorModulePlan;
use super::JavaScriptErrorModuleRegistry;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorParameterField;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptPatternBindingDeclarations;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceReference;
use super::JavaScriptSourceReferenceCapture;
use super::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result;
use super::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Result;
use super::append_java_script_pattern_binding_declarations;
use super::compile_java_script_mutable_call_source_with_captures;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::lower_java_script_source_parameters;
use super::lower_java_script_source_statements;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::source_ml_source1_temporary_compile_java_script_error_module_templates_loop198;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_templates_loop197(mut plan: self::JavaScriptErrorModulePlan, mut registry: self::JavaScriptErrorModuleRegistry, mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorModuleBinding>>>, mut capabilities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>, mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut source_ml_source1_temporary_values186: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>, mut source_ml_source1_temporary_index186: self::ml::Big) -> self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result {
    loop {
        return if (source_ml_source1_temporary_index186 < ({ let ml_reference_array611 = source_ml_source1_temporary_values186.clone(); let ml_reference_projection611 = self::ml::Big::from_u128(ml_reference_array611.borrow().len() as u128); ml_reference_projection611 })) {
            {
                let definition = { let ml_reference_array612 = source_ml_source1_temporary_values186.clone(); let ml_reference_position612 = source_ml_source1_temporary_index186.clone(); let ml_reference_projection612 = self::ml_array::at(&*ml_reference_array612.borrow(), ml_reference_position612.to_index()); ml_reference_projection612 };
                {
                    let owner = { let ml_values613 = { let ml_reference_value614 = (match registry.clone() {
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value22048, _, _, _, _) => {
                            source_ml_source2_data_property_value22048.clone()
                        }
                    }).borrow().clone(); ml_reference_value614 }; let ml_callback613 = { let definition = definition.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionOwner| -> bool { ((match row.clone() {
                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value22053, _, _, _, _, _) => {
                            source_ml_source2_data_property_value22053
                        }
                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value22054, _, _, _) => {
                            source_ml_source2_data_property_value22054
                        }
                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value22055, _, _) => {
                            source_ml_source2_data_property_value22055
                        }
                    }) == (match definition.clone() {
                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(source_ml_source2_data_property_value22056, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value22056
                        }
                    })) }) }; ml_values613.iter().cloned().enumerate().find_map(|(ml_index613, ml_value613)| if ml_callback613(ml_value613.clone()) { Some(ml_value613) } else { None }) };
                    {
                        let source_owner = { let ml_values615 = { let ml_reference_value616 = (match plan.clone() {
                            self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value22057, _, _, _, _, _) => {
                                source_ml_source2_data_property_value22057.clone()
                            }
                        }).borrow().clone(); ml_reference_value616 }; let ml_callback615 = { let definition = definition.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionPlan| -> bool { ((match row.clone() {
                            self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(source_ml_source2_data_property_value22065, _, _, _, _, _) => {
                                source_ml_source2_data_property_value22065
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(source_ml_source2_data_property_value22066, _, _, _, _) => {
                                source_ml_source2_data_property_value22066
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(source_ml_source2_data_property_value22067, _, _, _) => {
                                source_ml_source2_data_property_value22067
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(source_ml_source2_data_property_value22068, _, _, _) => {
                                source_ml_source2_data_property_value22068
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(source_ml_source2_data_property_value22069, _, _, _) => {
                                source_ml_source2_data_property_value22069
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(source_ml_source2_data_property_value22070, _, _, _) => {
                                source_ml_source2_data_property_value22070
                            }
                        }) == (match definition.clone() {
                            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(source_ml_source2_data_property_value22071, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value22071
                            }
                        })) }) }; ml_values615.iter().cloned().enumerate().find_map(|(ml_index615, ml_value615)| if ml_callback615(ml_value615.clone()) { Some(ml_value615) } else { None }) };
                        {
                            let owner_error = self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 105, 116, 101, 115, 59, 32, 100, 101, 114, 105, 118, 101, 100, 45, 116, 104, 105, 115, 32, 99, 101, 108, 108, 115, 32, 97, 114, 101, 32, 115, 101, 112, 97, 114, 97, 116, 101]);
                            if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((owner_error.clone(), templates.clone()))))
                            } else {
                                if match (source_owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((owner_error.clone(), templates.clone()))))
                                } else {
                                    if (((((match source_owner.clone().expect("a proven present value") {
                                        self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(_, _, _, _, _, _) => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    } || !(self::ml_math::is_safe_integer(match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value22103, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value22103
                                        }
                                    }))) || ((match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value22105, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value22105
                                        }
                                    }) < (0f64))) || ((match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value22107, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value22107
                                        }
                                    }) >= (4096f64))) || {
                                        let source_ml_source2_same_value_operand22109 = match definition.clone() {
                                            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value22108, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value22108
                                            }
                                        };
                                        {
                                            let source_ml_source2_same_value_operand22110 = (-0f64);
                                            (((source_ml_source2_same_value_operand22109 == source_ml_source2_same_value_operand22110) && ((source_ml_source2_same_value_operand22109 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand22109)) == ((1f64 / source_ml_source2_same_value_operand22110))))) || ((source_ml_source2_same_value_operand22109 != source_ml_source2_same_value_operand22109) && (source_ml_source2_same_value_operand22110 != source_ml_source2_same_value_operand22110)))
                                        }
                                    }) || { let ml_values617 = { let ml_reference_value618 = templates.borrow().clone(); ml_reference_value618 }; let ml_callback617 = { let owner = owner.clone(); let definition = definition.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallTemplate| -> bool { (((match row.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value22115, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value22115.clone()
                                        }
                                    }) == (match owner.clone().expect("a proven present value") {
                                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value22116, _, _, _, _) => {
                                            source_ml_source2_data_property_value22116.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value22117, _, _) => {
                                            source_ml_source2_data_property_value22117.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value22118, _) => {
                                            source_ml_source2_data_property_value22118.clone()
                                        }
                                    })) && ((match row.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(_, source_ml_source2_data_property_value22121, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value22121
                                        }
                                    }) == (match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value22122, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value22122
                                        }
                                    }))) }) }; ml_values617.iter().cloned().enumerate().any(|(ml_index617, ml_value617)| ml_callback617(ml_value617)) }) {
                                        self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((owner_error.clone(), templates.clone()))))
                                    } else {
                                        {
                                            let realm = { let ml_values619 = { let ml_reference_value620 = (match registry.clone() {
                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, source_ml_source2_data_property_value22125, _, _, _) => {
                                                    source_ml_source2_data_property_value22125.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value620 }; let ml_callback619 = { let owner = owner.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorRealmOperations| -> bool { ((match row.clone() {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value22130, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value22130.clone()
                                                }
                                            }) == (match owner.clone().expect("a proven present value") {
                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value22131, _, _, _) => {
                                                    source_ml_source2_data_property_value22131.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value22132, _) => {
                                                    source_ml_source2_data_property_value22132.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value22133) => {
                                                    source_ml_source2_data_property_value22133.clone()
                                                }
                                            })) }) }; ml_values619.iter().cloned().enumerate().find_map(|(ml_index619, ml_value619)| if ml_callback619(ml_value619.clone()) { Some(ml_value619) } else { None }) };
                                            if match (realm.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 115, 111, 117, 114, 99, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]), templates.clone()))))
                                            } else {
                                                {
                                                    let prototype = match realm.clone().expect("a proven present value") {
                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, source_ml_source2_data_property_value22136, _, _, _, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value22136.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    };
                                                    if match (prototype.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 115, 111, 117, 114, 99, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]), templates.clone()))))
                                                    } else {
                                                        {
                                                            let first_default = { let ml_values621 = { let ml_reference_value622 = (match definition.clone() {
                                                                self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value22139, _, _, _) => {
                                                                    source_ml_source2_data_property_value22139.clone()
                                                                }
                                                            }).borrow().clone(); ml_reference_value622 }; let ml_callback621 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> bool { match parameter.clone() {
                                                                self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                                                                    false
                                                                }
                                                                self::JavaScriptErrorParameter::ErrorObjectParameter(_, has_default, _, _, _, _) => {
                                                                    has_default
                                                                }
                                                                self::JavaScriptErrorParameter::ErrorArrayParameter(_, has_default_2, _, _, _, _) => {
                                                                    has_default_2
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            } }); ml_values621.iter().cloned().enumerate().position(|(ml_index621, ml_value621)| ml_callback621(ml_value621)).map_or(-1f64, |ml_position| ml_position as f64) };
                                                            {
                                                                let length = if (first_default < (0f64)) {
                                                                    { let ml_reference_array623 = match definition.clone() {
                                                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value22158, _, _, _) => {
                                                                            source_ml_source2_data_property_value22158.clone()
                                                                        }
                                                                    }; let ml_reference_projection623 = ml_reference_array623.borrow().len() as f64; ml_reference_projection623 }
                                                                } else {
                                                                    first_default
                                                                };
                                                                if ((match definition.clone() {
                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, source_ml_source2_data_property_value22160, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value22160
                                                                    }
                                                                }) != length) {
                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 98, 111, 117, 110, 100, 97, 114, 121]), templates.clone()))))
                                                                } else {
                                                                    {
                                                                        let identities = std::rc::Rc::new(std::cell::RefCell::new({
                                                                            let source_ml_source2_concat_value22165 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values624 = { let ml_reference_value625 = bindings.borrow().clone(); ml_reference_value625 }; let ml_callback624 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value22163, _) => {
                                                                                    source_ml_source2_data_property_value22163
                                                                                }
                                                                            } }); ml_values624.iter().cloned().enumerate().map(|(ml_index624, ml_value624)| ml_callback624(ml_value624)).collect::<Vec<_>>() }));
                                                                            {
                                                                                let source_ml_source2_concat_value22166 = match definition.clone() {
                                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, _, source_ml_source2_data_property_value22164, _, _) => {
                                                                                        source_ml_source2_data_property_value22164.clone()
                                                                                    }
                                                                                };
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value626 = source_ml_source2_concat_value22165.borrow().clone(); ml_reference_value626 }), { let ml_reference_value627 = source_ml_source2_concat_value22166.borrow().clone(); ml_reference_value627 })
                                                                            }
                                                                        }));
                                                                        {
                                                                            let parameters = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values628 = { let ml_reference_value629 = (match definition.clone() {
                                                                                self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value22167, _, _, _) => {
                                                                                    source_ml_source2_data_property_value22167.clone()
                                                                                }
                                                                            }).borrow().clone(); ml_reference_value629 }; let ml_callback628 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> f64 { match parameter.clone() {
                                                                                self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value22168, _, _) => {
                                                                                    source_ml_source2_data_property_value22168
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value22169, _) => {
                                                                                    source_ml_source2_data_property_value22169
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value22170, _, _) => {
                                                                                    source_ml_source2_data_property_value22170
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value22171, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value22171
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value22172, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value22172
                                                                                }
                                                                            } }); ml_values628.iter().cloned().enumerate().map(|(ml_index628, ml_value628)| ml_callback628(ml_value628)).collect::<Vec<_>>() }));
                                                                            if ((({ let ml_reference_array630 = identities.clone(); let ml_reference_projection630 = ml_reference_array630.borrow().len() as f64; ml_reference_projection630 }) > (4096f64)) || { let ml_values631 = { let ml_reference_value632 = identities.borrow().clone(); ml_reference_value632 }; let ml_callback631 = { let identities = identities.clone(); let parameters = parameters.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { (((self::ml_array::index_of(&{ let ml_reference_value633 = identities.borrow().clone(); ml_reference_value633 }, &identity, 0f64, false, |a, b| a == b)) != index) || self::ml_array::includes(&{ let ml_reference_value634 = parameters.borrow().clone(); ml_reference_value634 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values631.iter().cloned().enumerate().any(|(ml_index631, ml_value631)| ml_callback631(ml_value631, (ml_index631 as f64))) }) {
                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 97, 110, 100, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 105, 115, 106, 111, 105, 110, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 115]), templates.clone()))))
                                                                            } else {
                                                                                {
                                                                                    let context = {
                                                                                        let source_ml_source2_constructor_field22175 = registry.clone();
                                                                                        {
                                                                                            let source_ml_source2_constructor_field22176 = realm.clone().expect("a proven present value");
                                                                                            {
                                                                                                let source_ml_source2_constructor_field22177 = capabilities.clone();
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field22179 = Some((match plan.clone() {
                                                                                                        self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, source_ml_source2_data_property_value22178, _) => {
                                                                                                            ((source_ml_source2_data_property_value22178.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field22181 = Some((match plan.clone() {
                                                                                                            self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, _, source_ml_source2_data_property_value22180) => {
                                                                                                                ((source_ml_source2_data_property_value22180.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new()))));
                                                                                                        {
                                                                                                            let source_ml_source2_constructor_field22183 = Some(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values635 = { let ml_reference_value636 = bindings.borrow().clone(); ml_reference_value636 }; let ml_callback635 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value22182, _) => {
                                                                                                                    source_ml_source2_data_property_value22182
                                                                                                                }
                                                                                                            } }); ml_values635.iter().cloned().enumerate().map(|(ml_index635, ml_value635)| ml_callback635(ml_value635)).collect::<Vec<_>>() })));
                                                                                                            {
                                                                                                                let source_ml_source2_constructor_field22185 = match plan.clone() {
                                                                                                                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value22184, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value22184
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_constructor_field22186 = 0f64;
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_constructor_field22175.clone()), Box::new(source_ml_source2_constructor_field22176.clone()), source_ml_source2_constructor_field22177.clone(), source_ml_source2_constructor_field22185, source_ml_source2_constructor_field22186, self::ml_string::Text::from_units(vec![]), true, Some((source_ml_source2_constructor_field22183.clone(),)), Some((source_ml_source2_constructor_field22179.clone(),)), Some((source_ml_source2_constructor_field22181.clone(),)))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let lowered = self::lower_java_script_source_statements(match definition.clone() {
                                                                                            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, _, _, source_ml_source2_data_property_value22189, _) => {
                                                                                                source_ml_source2_data_property_value22189.clone()
                                                                                            }
                                                                                        }, 0f64, context.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), 0f64, true);
                                                                                        if ((match match lowered.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value22192) => {
                                                                                                let source_ml_source2_data_property_value22192 = *source_ml_source2_data_property_value22192;
                                                                                                source_ml_source2_data_property_value22192.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value22193, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value22193.clone()
                                                                                            }
                                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                let source_ml_source2_field22197 = match match lowered.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value22195) => {
                                                                                                        let source_ml_source2_data_property_value22195 = *source_ml_source2_data_property_value22195;
                                                                                                        source_ml_source2_data_property_value22195.clone()
                                                                                                    }
                                                                                                } {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value22196, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value22196.clone()
                                                                                                    }
                                                                                                };
                                                                                                (source_ml_source2_field22197.clone(), templates.clone())
                                                                                            })))
                                                                                        } else {
                                                                                            {
                                                                                                let initialized = self::lower_java_script_source_parameters(match definition.clone() {
                                                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value22198, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value22198.clone()
                                                                                                    }
                                                                                                }, match lowered.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value22199) => {
                                                                                                        let source_ml_source2_data_property_value22199 = *source_ml_source2_data_property_value22199;
                                                                                                        source_ml_source2_data_property_value22199.clone()
                                                                                                    }
                                                                                                }, match lowered.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value22200, _) => {
                                                                                                        let source_ml_source2_data_property_value22200 = *source_ml_source2_data_property_value22200;
                                                                                                        source_ml_source2_data_property_value22200.clone()
                                                                                                    }
                                                                                                });
                                                                                                if ((match match initialized.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value22203) => {
                                                                                                        let source_ml_source2_data_property_value22203 = *source_ml_source2_data_property_value22203;
                                                                                                        source_ml_source2_data_property_value22203.clone()
                                                                                                    }
                                                                                                } {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value22204, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value22204.clone()
                                                                                                    }
                                                                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                        let source_ml_source2_field22208 = match match initialized.clone() {
                                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value22206) => {
                                                                                                                let source_ml_source2_data_property_value22206 = *source_ml_source2_data_property_value22206;
                                                                                                                source_ml_source2_data_property_value22206.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value22207, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value22207.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        (source_ml_source2_field22208.clone(), templates.clone())
                                                                                                    })))
                                                                                                } else {
                                                                                                    {
                                                                                                        let code = self::compile_java_script_mutable_call_source_with_captures(match initialized.clone() {
                                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value22209, _) => {
                                                                                                                let source_ml_source2_data_property_value22209 = *source_ml_source2_data_property_value22209;
                                                                                                                source_ml_source2_data_property_value22209.clone()
                                                                                                            }
                                                                                                        }, parameters.clone(), identities.clone());
                                                                                                        if ((match code.clone() {
                                                                                                            self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value22211) => {
                                                                                                                source_ml_source2_data_property_value22211.clone()
                                                                                                            }
                                                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                let source_ml_source2_field22214 = match code.clone() {
                                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value22213) => {
                                                                                                                        source_ml_source2_data_property_value22213.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                (source_ml_source2_field22214.clone(), templates.clone())
                                                                                                            })))
                                                                                                        } else {
                                                                                                            {
                                                                                                                let receiver_mode = match definition.clone() {
                                                                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, _, _, _, source_ml_source2_data_property_value22215) => {
                                                                                                                        ((source_ml_source2_data_property_value22215.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                if (({
                                                                                                                    let source_ml_source2_comparison_operand22217 = None::<()>;
                                                                                                                    !(match (receiver_mode.clone(), source_ml_source2_comparison_operand22217.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                } && !(match (receiver_mode.clone(), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })) && !(match (receiver_mode.clone(), self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })) {
                                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![114, 117, 110, 116, 105, 109, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 97, 114, 114, 111, 119, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 109, 111, 100, 101]), templates.clone()))))
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let name = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                                        {
                                                                                                                            let index = 0f64;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s22224 = self::source_ml_source1_temporary_compile_java_script_error_module_templates_loop198(definition.clone(), name.clone(), index);
                                                                                                                                match source_ml_source2_s22224.clone() {
                                                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Done => {
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_temporary_compile_java_script_error_module_templates_ignored199 = { let ml_method_owner637 = templates.clone(); let ml_method_argument637_0 = {
                                                                                                                                                let source_ml_source1_temporary_constructor_field187 = match owner.clone().expect("a proven present value") {
                                                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value22225, _, _, _, _) => {
                                                                                                                                                        source_ml_source2_data_property_value22225.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value22226, _, _) => {
                                                                                                                                                        source_ml_source2_data_property_value22226.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value22227, _) => {
                                                                                                                                                        source_ml_source2_data_property_value22227.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_temporary_constructor_field188 = match definition.clone() {
                                                                                                                                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value22228, _, _, _, _, _, _) => {
                                                                                                                                                            source_ml_source2_data_property_value22228
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_temporary_constructor_field189 = match owner.clone().expect("a proven present value") {
                                                                                                                                                            self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value22229, _, _, _) => {
                                                                                                                                                                source_ml_source2_data_property_value22229.clone()
                                                                                                                                                            }
                                                                                                                                                            self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value22230, _) => {
                                                                                                                                                                source_ml_source2_data_property_value22230.clone()
                                                                                                                                                            }
                                                                                                                                                            self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value22231) => {
                                                                                                                                                                source_ml_source2_data_property_value22231.clone()
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_temporary_constructor_field190 = { let ml_reference_array638 = parameters.clone(); let ml_reference_projection638 = ml_reference_array638.borrow().len() as f64; ml_reference_projection638 };
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_temporary_constructor_field191 = match code.clone() {
                                                                                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, source_ml_source2_data_property_value22232, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value22232
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_temporary_constructor_field192 = { let ml_reference_array639 = identities.clone(); let ml_reference_projection639 = ml_reference_array639.borrow().len() as f64; ml_reference_projection639 };
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_temporary_constructor_field193 = match code.clone() {
                                                                                                                                                                            self::JavaScriptCallSourceCompilation::SourceCompiled(source_ml_source2_data_property_value22233, _, _) => {
                                                                                                                                                                                source_ml_source2_data_property_value22233.clone()
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_temporary_constructor_field194 = prototype.clone().expect("a proven present value");
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_temporary_constructor_field195 = self::JavaScriptSemanticValue::Text(name.clone());
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_temporary_constructor_field196 = match definition.clone() {
                                                                                                                                                                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, source_ml_source2_data_property_value22234, _, _, _, _) => {
                                                                                                                                                                                            source_ml_source2_data_property_value22234
                                                                                                                                                                                        }
                                                                                                                                                                                    };
                                                                                                                                                                                    if {
                                                                                                                                                                                        let source_ml_source2_comparison_operand22236 = None::<()>;
                                                                                                                                                                                        !(match (receiver_mode.clone(), source_ml_source2_comparison_operand22236.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                                                                                    } {
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_constructor_field22247 = Some(receiver_mode.clone().expect("a proven present value"));
                                                                                                                                                                                            self::JavaScriptCallTemplate::CallTemplate(source_ml_source1_temporary_constructor_field187.clone(), source_ml_source1_temporary_constructor_field188, source_ml_source1_temporary_constructor_field189.clone(), source_ml_source1_temporary_constructor_field190, source_ml_source1_temporary_constructor_field191, source_ml_source1_temporary_constructor_field192, source_ml_source1_temporary_constructor_field193.clone(), Box::new(source_ml_source1_temporary_constructor_field194.clone()), Box::new(source_ml_source1_temporary_constructor_field195.clone()), source_ml_source1_temporary_constructor_field196, Some((source_ml_source2_constructor_field22247.clone(),)), None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                                                                                                                        }
                                                                                                                                                                                    } else {
                                                                                                                                                                                        self::JavaScriptCallTemplate::CallTemplate(source_ml_source1_temporary_constructor_field187.clone(), source_ml_source1_temporary_constructor_field188, source_ml_source1_temporary_constructor_field189.clone(), source_ml_source1_temporary_constructor_field190, source_ml_source1_temporary_constructor_field191, source_ml_source1_temporary_constructor_field192, source_ml_source1_temporary_constructor_field193.clone(), Box::new(source_ml_source1_temporary_constructor_field194.clone()), Box::new(source_ml_source1_temporary_constructor_field195.clone()), source_ml_source1_temporary_constructor_field196, None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
                                                                                                                                            }; let ml_inserted = vec![ml_method_argument637_0]; let mut ml_method_values637 = ml_method_owner637.borrow_mut(); let ml_length = ml_method_values637.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values637.extend(ml_inserted); ml_length as f64 };
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_temporary_index186_2 = source_ml_source1_temporary_index186.add(&self::ml::Big::from_i128(1));
                                                                                                                                                {
                                                                                                                                                    source_ml_source1_temporary_index186 = source_ml_source1_temporary_index186_2.clone();
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
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result {
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>),
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result {
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Done,
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result {
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Done,
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_error_call_templates_loop205(mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut end: f64, mut index: f64) -> self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result {
    loop {
        return if (index < end) {
            {
                let value = { let ml_reference_array640 = ordered.clone(); let ml_reference_position640 = index; let ml_reference_projection640 = self::ml_array::optional_at(&*ml_reference_array640.borrow(), self::ml_array::number_index(ml_reference_position640)); ml_reference_projection640 };
                if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value641 = templates.borrow().clone(); ml_reference_value641 }, 0f64, f64::INFINITY))))
                } else {
                    {
                        let source_ml_source1_temporary_order_java_script_error_call_templates_ignored206 = { let ml_method_owner642 = merged.clone(); let ml_method_argument642_0 = value.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument642_0]; let mut ml_method_values642 = ml_method_owner642.borrow_mut(); let ml_length = ml_method_values642.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values642.extend(ml_inserted); ml_length as f64 };
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result {
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Done,
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_error_call_templates_loop204(mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut middle: f64, mut index: f64) -> self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result {
    loop {
        return if (index < middle) {
            {
                let value = { let ml_reference_array643 = ordered.clone(); let ml_reference_position643 = index; let ml_reference_projection643 = self::ml_array::optional_at(&*ml_reference_array643.borrow(), self::ml_array::number_index(ml_reference_position643)); ml_reference_projection643 };
                if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value644 = templates.borrow().clone(); ml_reference_value644 }, 0f64, f64::INFINITY))))
                } else {
                    {
                        let source_ml_source1_temporary_order_java_script_error_call_templates_ignored207 = { let ml_method_owner645 = merged.clone(); let ml_method_argument645_0 = value.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument645_0]; let mut ml_method_values645 = ml_method_owner645.borrow_mut(); let ml_length = ml_method_values645.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values645.extend(ml_inserted); ml_length as f64 };
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result {
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Done(f64, f64),
    MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_error_call_templates_loop203(mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut middle: f64, mut end: f64, mut left: f64, mut right: f64) -> self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result {
    loop {
        return if ((left < middle) && (right < end)) {
            {
                let first = { let ml_reference_array646 = ordered.clone(); let ml_reference_position646 = left; let ml_reference_projection646 = self::ml_array::optional_at(&*ml_reference_array646.borrow(), self::ml_array::number_index(ml_reference_position646)); ml_reference_projection646 };
                {
                    let second = { let ml_reference_array647 = ordered.clone(); let ml_reference_position647 = right; let ml_reference_projection647 = self::ml_array::optional_at(&*ml_reference_array647.borrow(), self::ml_array::number_index(ml_reference_position647)); ml_reference_projection647 };
                    if match (first.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value648 = templates.borrow().clone(); ml_reference_value648 }, 0f64, f64::INFINITY))))
                    } else {
                        if match (second.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value649 = templates.borrow().clone(); ml_reference_value649 }, 0f64, f64::INFINITY))))
                        } else {
                            if (((match first.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value22268, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22268.clone()
                                }
                            }) < (match second.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value22269, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22269.clone()
                                }
                            })) || (((match first.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value22272, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22272.clone()
                                }
                            }) == (match second.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value22273, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22273.clone()
                                }
                            })) && ((match first.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(_, source_ml_source2_data_property_value22276, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22276
                                }
                            }) <= (match second.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(_, source_ml_source2_data_property_value22277, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22277
                                }
                            })))) {
                                {
                                    let source_ml_source1_temporary_order_java_script_error_call_templates_ignored208 = { let ml_method_owner650 = merged.clone(); let ml_method_argument650_0 = first.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument650_0]; let mut ml_method_values650 = ml_method_owner650.borrow_mut(); let ml_length = ml_method_values650.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values650.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let left_2 = (left + 1f64);
                                        {
                                            left = left_2;
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_temporary_order_java_script_error_call_templates_ignored209 = { let ml_method_owner651 = merged.clone(); let ml_method_argument651_0 = second.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument651_0]; let mut ml_method_values651 = ml_method_owner651.borrow_mut(); let ml_length = ml_method_values651.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values651.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let right_2 = (right + 1f64);
                                        {
                                            right = right_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Done(left, right)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_error_call_templates_loop202(mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut width: f64, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut start: f64) -> self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result {
    loop {
        return if (start < ({ let ml_reference_array652 = ordered.clone(); let ml_reference_projection652 = ml_reference_array652.borrow().len() as f64; ml_reference_projection652 })) {
            {
                let middle = self::ml_math::min((start + width), { let ml_reference_array653 = ordered.clone(); let ml_reference_projection653 = ml_reference_array653.borrow().len() as f64; ml_reference_projection653 });
                {
                    let end = self::ml_math::min((start + (2f64 * width)), { let ml_reference_array654 = ordered.clone(); let ml_reference_projection654 = ml_reference_array654.borrow().len() as f64; ml_reference_projection654 });
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let source_ml_source2_s22278 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop203(templates.clone(), ordered.clone(), merged.clone(), middle, end, left, right);
                                match source_ml_source2_s22278.clone() {
                                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Done(left_2, right_2) => {
                                        {
                                            let source_ml_source2_s22279 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop204(templates.clone(), ordered.clone(), merged.clone(), middle, left_2);
                                            match source_ml_source2_s22279.clone() {
                                                self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Done => {
                                                    {
                                                        let source_ml_source2_s22280 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop205(templates.clone(), ordered.clone(), merged.clone(), end, right_2);
                                                        match source_ml_source2_s22280.clone() {
                                                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Done => {
                                                                {
                                                                    let start_2 = (start + (2f64 * width));
                                                                    {
                                                                        start = start_2;
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Return(source_ml_source1_temporary_result205) => {
                                                                self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Return(source_ml_source1_temporary_result205.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Return(source_ml_source1_temporary_result206) => {
                                                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Return(source_ml_source1_temporary_result206.clone())
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Return(source_ml_source1_temporary_result207) => {
                                        self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Return(source_ml_source1_temporary_result207.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_error_call_templates_loop201(mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut width: f64) -> self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result {
    loop {
        return if (width < ({ let ml_reference_array655 = ordered.clone(); let ml_reference_projection655 = ml_reference_array655.borrow().len() as f64; ml_reference_projection655 })) {
            {
                let merged = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallTemplate>::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source2_s22281 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop202(templates.clone(), ordered.clone(), width, merged.clone(), start);
                        match source_ml_source2_s22281.clone() {
                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Done => {
                                {
                                    let width_2 = (width * 2f64);
                                    {
                                        (ordered, width) = (merged.clone(), width_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Return(source_ml_source1_temporary_result202) => {
                                self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Return(source_ml_source1_temporary_result202.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Done(ordered.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_source_reference_captures_loop210(mut reference: self::JavaScriptSourceReference, mut context: self::JavaScriptErrorSourceContext, mut source: self::JavaScriptCallSource, mut at: f64) -> self::JavaScriptCallSource {
    loop {
        return if (at >= (0f64)) {
            {
                let capture = {
                    let source_ml_source2_array_read_value22283 = match reference.clone() {
                        self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, source_ml_source2_data_property_value22282, _) => {
                            source_ml_source2_data_property_value22282.clone()
                        }
                    };
                    { let ml_reference_array656 = source_ml_source2_array_read_value22283.clone(); let ml_reference_position656 = at; let ml_reference_projection656 = self::ml_array::optional_at(&*ml_reference_array656.borrow(), self::ml_array::number_index(ml_reference_position656)); ml_reference_projection656 }
                };
                if !(match (capture.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                    {
                        let source_2 = self::construct_java_script_error_local_binding(context.clone(), match capture.clone().expect("a proven present value") {
                            self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(source_ml_source2_data_property_value22285, _) => {
                                source_ml_source2_data_property_value22285
                            }
                        }, match capture.clone().expect("a proven present value") {
                            self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(_, source_ml_source2_data_property_value22286) => {
                                let source_ml_source2_data_property_value22286 = *source_ml_source2_data_property_value22286;
                                source_ml_source2_data_property_value22286.clone()
                            }
                        }, source.clone(), false);
                        {
                            let at_2 = (at - 1f64);
                            {
                                (source, at) = (source_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = (at - 1f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            source.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213 {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Next,
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Return(Box<self::JavaScriptPatternBindingDeclarations>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215 {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Done,
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Return(Box<self::JavaScriptPatternBindingDeclarations>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_pattern_binding_declarations_loop214(mut mutable: bool, mut depth: f64, mut shared: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_values211: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>, mut source_ml_source1_temporary_index211: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result {
    loop {
        return if (source_ml_source1_temporary_index211 < ({ let ml_reference_array657 = source_ml_source1_temporary_values211.clone(); let ml_reference_projection657 = self::ml::Big::from_u128(ml_reference_array657.borrow().len() as u128); ml_reference_projection657 })) {
            {
                let field = { let ml_reference_array658 = source_ml_source1_temporary_values211.clone(); let ml_reference_position658 = source_ml_source1_temporary_index211.clone(); let ml_reference_projection658 = self::ml_array::at(&*ml_reference_array658.borrow(), ml_reference_position658.to_index()); ml_reference_projection658 };
                {
                    let child = self::append_java_script_pattern_binding_declarations(declarations.clone(), match field.clone() {
                        self::JavaScriptErrorParameterField::ErrorParameterField(source_ml_source2_data_property_value22289, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value22289
                        }
                    }, match field.clone() {
                        self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, source_ml_source2_data_property_value22290, _) => {
                            ((source_ml_source2_data_property_value22290.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone()
                        }
                    }, mutable, depth, shared.clone());
                    if ((match child.clone() {
                        self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value22292) => {
                            source_ml_source2_data_property_value22292.clone()
                        }
                    }) != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Return(Box::new(child.clone()))
                    } else {
                        {
                            let source_ml_source1_temporary_index211_2 = source_ml_source1_temporary_index211.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index211 = source_ml_source1_temporary_index211_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218 {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218Next,
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218Return(Box<self::JavaScriptPatternBindingDeclarations>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Done,
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Return(Box<self::JavaScriptPatternBindingDeclarations>),
}
