// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
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
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
use super::JavaScriptSourceLexicalBinding;
use super::compile_java_script_mutable_call_source_with_captures;
use super::construct_java_script_error_constant;
use super::lower_java_script_source_parameters;
use super::lower_java_script_source_statements;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_semantic_value_error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Result {
    MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Done,
    MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_module_plan_error_loop185(mut plan: self::JavaScriptErrorModulePlan, mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorModuleBinding>>>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Result {
    loop {
        return if (at < ({ let ml_reference_array598 = bindings.clone(); let ml_reference_projection598 = ml_reference_array598.borrow().len() as f64; ml_reference_projection598 })) {
            {
                let binding = { let ml_reference_array599 = bindings.clone(); let ml_reference_position599 = at; let ml_reference_projection599 = self::ml_array::optional_at(&*ml_reference_array599.borrow(), self::ml_array::number_index(ml_reference_position599)); ml_reference_projection599 };
                if (((((match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::ml_math::is_safe_integer(match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21713, _) => {
                        source_ml_source2_data_property_value21713
                    }
                }))) || ((match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21715, _) => {
                        source_ml_source2_data_property_value21715
                    }
                }) < (0f64))) || ((match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21718, _) => {
                        source_ml_source2_data_property_value21718
                    }
                }) >= (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value21719, _, _, _) => {
                        source_ml_source2_data_property_value21719
                    }
                }))) || (({ let ml_values600 = { let ml_reference_value601 = bindings.borrow().clone(); ml_reference_value601 }; let ml_callback600 = { let binding = binding.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorModuleBinding| -> bool { ((match row.clone() {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21726, _) => {
                        source_ml_source2_data_property_value21726
                    }
                }) == (match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21727, _) => {
                        source_ml_source2_data_property_value21727
                    }
                })) }) }; ml_values600.iter().cloned().enumerate().position(|(ml_index600, ml_value600)| ml_callback600(ml_value600)).map_or(-1f64, |ml_position| ml_position as f64) }) != at)) || ((self::read_java_script_semantic_value_error(match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(_, source_ml_source2_data_property_value21729) => {
                        let source_ml_source2_data_property_value21729 = *source_ml_source2_data_property_value21729;
                        source_ml_source2_data_property_value21729.clone()
                    }
                })) != (self::ml_string::Text::from_units(vec![])))) {
                    self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Return(self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop185Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result {
    MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Done,
    MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_module_plan_error_loop184(mut registry: self::JavaScriptErrorModuleRegistry, mut capabilities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result {
    loop {
        return if (at < ({ let ml_reference_array602 = capabilities.clone(); let ml_reference_projection602 = ml_reference_array602.borrow().len() as f64; ml_reference_projection602 })) {
            {
                let capability = { let ml_reference_array603 = capabilities.clone(); let ml_reference_position603 = at; let ml_reference_projection603 = self::ml_array::optional_at(&*ml_reference_array603.borrow(), self::ml_array::number_index(ml_reference_position603)); ml_reference_projection603 };
                if ((((((((match (capability.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value21733, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21733.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![])))) || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, source_ml_source2_data_property_value21735, _, _) => {
                        source_ml_source2_data_property_value21735.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![])))) || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, source_ml_source2_data_property_value21737, _, _, _) => {
                        source_ml_source2_data_property_value21737.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![])))) || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21739, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21739.clone()
                    }
                }) <= (self::ml::Big::from_i128(0)))) || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21741, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21741.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::ml_array::includes(&{ let ml_reference_value604 = (match registry.clone() {
                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value21743, _) => {
                        source_ml_source2_data_property_value21743.clone()
                    }
                }).borrow().clone(); ml_reference_value604 }, &match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value21744, _, _, _, _) => {
                        source_ml_source2_data_property_value21744.clone()
                    }
                }, 0f64, false, |a, b| a == b))) || (match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value21745) => {
                        source_ml_source2_data_property_value21745
                    }
                } && !(match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value21746, _) => {
                        source_ml_source2_data_property_value21746
                    }
                }))) || (({ let ml_values605 = { let ml_reference_value606 = capabilities.borrow().clone(); ml_reference_value606 }; let ml_callback605 = { let capability = capability.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value21753, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21753.clone()
                    }
                }) == (match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value21754, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21754.clone()
                    }
                })) }) }; ml_values605.iter().cloned().enumerate().position(|(ml_index605, ml_value605)| ml_callback605(ml_value605)).map_or(-1f64, |ml_position| ml_position as f64) }) != at)) {
                    self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Return(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 115, 44, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 44, 32, 114, 101, 97, 108, 109, 115, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114, 115]))
                } else {
                    {
                        let previous = { let ml_values607 = { let ml_reference_value608 = capabilities.borrow().clone(); ml_reference_value608 }; let ml_callback607 = { let capability = capability.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21757, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21757.clone()
                            }
                        }) == (match capability.clone().expect("a proven present value") {
                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21758, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21758.clone()
                            }
                        })) }) }; ml_values607.iter().cloned().enumerate().position(|(ml_index607, ml_value607)| ml_callback607(ml_value607)).map_or(-1f64, |ml_position| ml_position as f64) };
                        if (previous != at) {
                            {
                                let owner = { let ml_reference_array609 = capabilities.clone(); let ml_reference_position609 = previous; let ml_reference_projection609 = self::ml_array::optional_at(&*ml_reference_array609.borrow(), self::ml_array::number_index(ml_reference_position609)); ml_reference_projection609 };
                                if (((((match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value21763, _, _, _, _) => {
                                        source_ml_source2_data_property_value21763.clone()
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value21764, _, _, _, _) => {
                                        source_ml_source2_data_property_value21764.clone()
                                    }
                                }))) || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, source_ml_source2_data_property_value21767, _, _) => {
                                        source_ml_source2_data_property_value21767.clone()
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, source_ml_source2_data_property_value21768, _, _) => {
                                        source_ml_source2_data_property_value21768.clone()
                                    }
                                }))) || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, source_ml_source2_data_property_value21771, _, _, _) => {
                                        source_ml_source2_data_property_value21771.clone()
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, source_ml_source2_data_property_value21772, _, _, _) => {
                                        source_ml_source2_data_property_value21772.clone()
                                    }
                                }))) || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value21775, _) => {
                                        source_ml_source2_data_property_value21775
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value21776, _) => {
                                        source_ml_source2_data_property_value21776
                                    }
                                }))) || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value21779) => {
                                        source_ml_source2_data_property_value21779
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value21780) => {
                                        source_ml_source2_data_property_value21780
                                    }
                                }))) {
                                    self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Return(self::ml_string::Text::from_units(vec![111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 97, 99, 113, 117, 105, 114, 101, 32, 99, 111, 110, 102, 108, 105, 99, 116, 105, 110, 103, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 114, 32, 110, 97, 109, 101, 115, 112, 97, 99, 101, 32, 114, 111, 108, 101, 115]))
                                } else {
                                    {
                                        let at_2 = (at + 1f64);
                                        {
                                            at = at_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let at_3 = (at + 1f64);
                                {
                                    at = at_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result {
    MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Done,
    MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>)>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Result {
    MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_templates_loop198(mut definition: self::JavaScriptErrorArrowTemplatePlan, mut name: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Result {
    loop {
        return if (index < ((match definition.clone() {
            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, source_ml_source2_data_property_value21782, _, _, _, _, _) => {
                source_ml_source2_data_property_value21782.clone()
            }
        }).length())) {
            {
                let source_ml_source1_temporary_compile_java_script_error_module_templates_ignored200 = { let ml_method_owner610 = name.clone(); let ml_method_argument610_0 = {
                    let source_ml_source2_string_unit_operand21784 = match definition.clone() {
                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, source_ml_source2_data_property_value21783, _, _, _, _, _) => {
                            source_ml_source2_data_property_value21783.clone()
                        }
                    };
                    {
                        let source_ml_source2_string_unit_operand21785 = index;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand21791 = {
                                        let source_ml_source2_string_range_operand21786 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand21787 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand21788 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position21789 = source_ml_source2_string_range_operand21787;
                                                    {
                                                        let source_ml_source2_string_range_position21790 = source_ml_source2_string_range_operand21788;
                                                        source_ml_source2_string_range_operand21786.slice(source_ml_source2_string_range_position21789, source_ml_source2_string_range_position21790)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand21792 = 0f64;
                                        source_ml_source2_string_unit_operand21791.code_point_at(source_ml_source2_string_unit_operand21792)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand21784.clone(), source_ml_source2_string_unit_operand21785)
                    }
                }; let ml_inserted = vec![ml_method_argument610_0]; let mut ml_method_values610 = ml_method_owner610.borrow_mut(); let ml_length = ml_method_values610.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values610.extend(ml_inserted); ml_length as f64 };
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_templates_loop197(mut plan: self::JavaScriptErrorModulePlan, mut registry: self::JavaScriptErrorModuleRegistry, mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorModuleBinding>>>, mut capabilities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>, mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut source_ml_source1_temporary_values186: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrowTemplatePlan>>>, mut source_ml_source1_temporary_index186: self::ml::Big) -> self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result {
    loop {
        return if (source_ml_source1_temporary_index186 < ({ let ml_reference_array611 = source_ml_source1_temporary_values186.clone(); let ml_reference_projection611 = self::ml::Big::from_u128(ml_reference_array611.borrow().len() as u128); ml_reference_projection611 })) {
            {
                let definition = { let ml_reference_array612 = source_ml_source1_temporary_values186.clone(); let ml_reference_position612 = source_ml_source1_temporary_index186.clone(); let ml_reference_projection612 = self::ml_array::at(&*ml_reference_array612.borrow(), ml_reference_position612.to_index()); ml_reference_projection612 };
                {
                    let owner = { let ml_values613 = { let ml_reference_value614 = (match registry.clone() {
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value21795, _, _, _, _) => {
                            source_ml_source2_data_property_value21795.clone()
                        }
                    }).borrow().clone(); ml_reference_value614 }; let ml_callback613 = { let definition = definition.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionOwner| -> bool { ((match row.clone() {
                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value21800, _, _, _, _, _) => {
                            source_ml_source2_data_property_value21800
                        }
                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value21801, _, _, _) => {
                            source_ml_source2_data_property_value21801
                        }
                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value21802, _, _) => {
                            source_ml_source2_data_property_value21802
                        }
                    }) == (match definition.clone() {
                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(source_ml_source2_data_property_value21803, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value21803
                        }
                    })) }) }; ml_values613.iter().cloned().enumerate().find_map(|(ml_index613, ml_value613)| if ml_callback613(ml_value613.clone()) { Some(ml_value613) } else { None }) };
                    {
                        let source_owner = { let ml_values615 = { let ml_reference_value616 = (match plan.clone() {
                            self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value21804, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21804.clone()
                            }
                        }).borrow().clone(); ml_reference_value616 }; let ml_callback615 = { let definition = definition.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionPlan| -> bool { ((match row.clone() {
                            self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(source_ml_source2_data_property_value21812, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21812
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(source_ml_source2_data_property_value21813, _, _, _, _) => {
                                source_ml_source2_data_property_value21813
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(source_ml_source2_data_property_value21814, _, _, _) => {
                                source_ml_source2_data_property_value21814
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(source_ml_source2_data_property_value21815, _, _, _) => {
                                source_ml_source2_data_property_value21815
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(source_ml_source2_data_property_value21816, _, _, _) => {
                                source_ml_source2_data_property_value21816
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(source_ml_source2_data_property_value21817, _, _, _) => {
                                source_ml_source2_data_property_value21817
                            }
                        }) == (match definition.clone() {
                            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(source_ml_source2_data_property_value21818, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21818
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
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21850, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21850
                                        }
                                    }))) || ((match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21852, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21852
                                        }
                                    }) < (0f64))) || ((match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21854, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21854
                                        }
                                    }) >= (4096f64))) || {
                                        let source_ml_source2_same_value_operand21856 = match definition.clone() {
                                            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21855, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value21855
                                            }
                                        };
                                        {
                                            let source_ml_source2_same_value_operand21857 = (-0f64);
                                            (((source_ml_source2_same_value_operand21856 == source_ml_source2_same_value_operand21857) && ((source_ml_source2_same_value_operand21856 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand21856)) == ((1f64 / source_ml_source2_same_value_operand21857))))) || ((source_ml_source2_same_value_operand21856 != source_ml_source2_same_value_operand21856) && (source_ml_source2_same_value_operand21857 != source_ml_source2_same_value_operand21857)))
                                        }
                                    }) || { let ml_values617 = { let ml_reference_value618 = templates.borrow().clone(); ml_reference_value618 }; let ml_callback617 = { let owner = owner.clone(); let definition = definition.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallTemplate| -> bool { (((match row.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value21862, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21862.clone()
                                        }
                                    }) == (match owner.clone().expect("a proven present value") {
                                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value21863, _, _, _, _) => {
                                            source_ml_source2_data_property_value21863.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value21864, _, _) => {
                                            source_ml_source2_data_property_value21864.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value21865, _) => {
                                            source_ml_source2_data_property_value21865.clone()
                                        }
                                    })) && ((match row.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(_, source_ml_source2_data_property_value21868, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21868
                                        }
                                    }) == (match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21869, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21869
                                        }
                                    }))) }) }; ml_values617.iter().cloned().enumerate().any(|(ml_index617, ml_value617)| ml_callback617(ml_value617)) }) {
                                        self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((owner_error.clone(), templates.clone()))))
                                    } else {
                                        {
                                            let realm = { let ml_values619 = { let ml_reference_value620 = (match registry.clone() {
                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, source_ml_source2_data_property_value21872, _, _, _) => {
                                                    source_ml_source2_data_property_value21872.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value620 }; let ml_callback619 = { let owner = owner.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorRealmOperations| -> bool { ((match row.clone() {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value21877, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value21877.clone()
                                                }
                                            }) == (match owner.clone().expect("a proven present value") {
                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value21878, _, _, _) => {
                                                    source_ml_source2_data_property_value21878.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value21879, _) => {
                                                    source_ml_source2_data_property_value21879.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value21880) => {
                                                    source_ml_source2_data_property_value21880.clone()
                                                }
                                            })) }) }; ml_values619.iter().cloned().enumerate().find_map(|(ml_index619, ml_value619)| if ml_callback619(ml_value619.clone()) { Some(ml_value619) } else { None }) };
                                            if match (realm.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 115, 111, 117, 114, 99, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]), templates.clone()))))
                                            } else {
                                                {
                                                    let prototype = match realm.clone().expect("a proven present value") {
                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, source_ml_source2_data_property_value21883, _, _, _, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value21883.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    };
                                                    if match (prototype.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 115, 111, 117, 114, 99, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]), templates.clone()))))
                                                    } else {
                                                        {
                                                            let first_default = { let ml_values621 = { let ml_reference_value622 = (match definition.clone() {
                                                                self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value21886, _, _, _) => {
                                                                    source_ml_source2_data_property_value21886.clone()
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
                                                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value21905, _, _, _) => {
                                                                            source_ml_source2_data_property_value21905.clone()
                                                                        }
                                                                    }; let ml_reference_projection623 = ml_reference_array623.borrow().len() as f64; ml_reference_projection623 }
                                                                } else {
                                                                    first_default
                                                                };
                                                                if ((match definition.clone() {
                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, source_ml_source2_data_property_value21907, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value21907
                                                                    }
                                                                }) != length) {
                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 98, 111, 117, 110, 100, 97, 114, 121]), templates.clone()))))
                                                                } else {
                                                                    {
                                                                        let identities = std::rc::Rc::new(std::cell::RefCell::new({
                                                                            let source_ml_source2_concat_value21912 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values624 = { let ml_reference_value625 = bindings.borrow().clone(); ml_reference_value625 }; let ml_callback624 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21910, _) => {
                                                                                    source_ml_source2_data_property_value21910
                                                                                }
                                                                            } }); ml_values624.iter().cloned().enumerate().map(|(ml_index624, ml_value624)| ml_callback624(ml_value624)).collect::<Vec<_>>() }));
                                                                            {
                                                                                let source_ml_source2_concat_value21913 = match definition.clone() {
                                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, _, source_ml_source2_data_property_value21911, _, _) => {
                                                                                        source_ml_source2_data_property_value21911.clone()
                                                                                    }
                                                                                };
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value626 = source_ml_source2_concat_value21912.borrow().clone(); ml_reference_value626 }), { let ml_reference_value627 = source_ml_source2_concat_value21913.borrow().clone(); ml_reference_value627 })
                                                                            }
                                                                        }));
                                                                        {
                                                                            let parameters = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values628 = { let ml_reference_value629 = (match definition.clone() {
                                                                                self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value21914, _, _, _) => {
                                                                                    source_ml_source2_data_property_value21914.clone()
                                                                                }
                                                                            }).borrow().clone(); ml_reference_value629 }; let ml_callback628 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> f64 { match parameter.clone() {
                                                                                self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value21915, _, _) => {
                                                                                    source_ml_source2_data_property_value21915
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value21916, _) => {
                                                                                    source_ml_source2_data_property_value21916
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value21917, _, _) => {
                                                                                    source_ml_source2_data_property_value21917
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value21918, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value21918
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value21919, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value21919
                                                                                }
                                                                            } }); ml_values628.iter().cloned().enumerate().map(|(ml_index628, ml_value628)| ml_callback628(ml_value628)).collect::<Vec<_>>() }));
                                                                            if ((({ let ml_reference_array630 = identities.clone(); let ml_reference_projection630 = ml_reference_array630.borrow().len() as f64; ml_reference_projection630 }) > (4096f64)) || { let ml_values631 = { let ml_reference_value632 = identities.borrow().clone(); ml_reference_value632 }; let ml_callback631 = { let identities = identities.clone(); let parameters = parameters.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { (((self::ml_array::index_of(&{ let ml_reference_value633 = identities.borrow().clone(); ml_reference_value633 }, &identity, 0f64, false, |a, b| a == b)) != index) || self::ml_array::includes(&{ let ml_reference_value634 = parameters.borrow().clone(); ml_reference_value634 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values631.iter().cloned().enumerate().any(|(ml_index631, ml_value631)| ml_callback631(ml_value631, (ml_index631 as f64))) }) {
                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 97, 110, 100, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 105, 115, 106, 111, 105, 110, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 115]), templates.clone()))))
                                                                            } else {
                                                                                {
                                                                                    let context = {
                                                                                        let source_ml_source2_constructor_field21922 = registry.clone();
                                                                                        {
                                                                                            let source_ml_source2_constructor_field21923 = realm.clone().expect("a proven present value");
                                                                                            {
                                                                                                let source_ml_source2_constructor_field21924 = capabilities.clone();
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field21926 = Some((match plan.clone() {
                                                                                                        self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, source_ml_source2_data_property_value21925, _) => {
                                                                                                            ((source_ml_source2_data_property_value21925.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field21928 = Some((match plan.clone() {
                                                                                                            self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, _, source_ml_source2_data_property_value21927) => {
                                                                                                                ((source_ml_source2_data_property_value21927.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new()))));
                                                                                                        {
                                                                                                            let source_ml_source2_constructor_field21930 = Some(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values635 = { let ml_reference_value636 = bindings.borrow().clone(); ml_reference_value636 }; let ml_callback635 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21929, _) => {
                                                                                                                    source_ml_source2_data_property_value21929
                                                                                                                }
                                                                                                            } }); ml_values635.iter().cloned().enumerate().map(|(ml_index635, ml_value635)| ml_callback635(ml_value635)).collect::<Vec<_>>() })));
                                                                                                            {
                                                                                                                let source_ml_source2_constructor_field21932 = match plan.clone() {
                                                                                                                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value21931, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value21931
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_constructor_field21933 = 0f64;
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_constructor_field21922.clone()), Box::new(source_ml_source2_constructor_field21923.clone()), source_ml_source2_constructor_field21924.clone(), source_ml_source2_constructor_field21932, source_ml_source2_constructor_field21933, self::ml_string::Text::from_units(vec![]), true, Some((source_ml_source2_constructor_field21930.clone(),)), Some((source_ml_source2_constructor_field21926.clone(),)), Some((source_ml_source2_constructor_field21928.clone(),)))
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
                                                                                            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, _, _, source_ml_source2_data_property_value21936, _) => {
                                                                                                source_ml_source2_data_property_value21936.clone()
                                                                                            }
                                                                                        }, 0f64, context.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), 0f64, true);
                                                                                        if ((match match lowered.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21939) => {
                                                                                                let source_ml_source2_data_property_value21939 = *source_ml_source2_data_property_value21939;
                                                                                                source_ml_source2_data_property_value21939.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21940, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value21940.clone()
                                                                                            }
                                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                let source_ml_source2_field21944 = match match lowered.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21942) => {
                                                                                                        let source_ml_source2_data_property_value21942 = *source_ml_source2_data_property_value21942;
                                                                                                        source_ml_source2_data_property_value21942.clone()
                                                                                                    }
                                                                                                } {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21943, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value21943.clone()
                                                                                                    }
                                                                                                };
                                                                                                (source_ml_source2_field21944.clone(), templates.clone())
                                                                                            })))
                                                                                        } else {
                                                                                            {
                                                                                                let initialized = self::lower_java_script_source_parameters(match definition.clone() {
                                                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value21945, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value21945.clone()
                                                                                                    }
                                                                                                }, match lowered.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21946) => {
                                                                                                        let source_ml_source2_data_property_value21946 = *source_ml_source2_data_property_value21946;
                                                                                                        source_ml_source2_data_property_value21946.clone()
                                                                                                    }
                                                                                                }, match lowered.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21947, _) => {
                                                                                                        let source_ml_source2_data_property_value21947 = *source_ml_source2_data_property_value21947;
                                                                                                        source_ml_source2_data_property_value21947.clone()
                                                                                                    }
                                                                                                });
                                                                                                if ((match match initialized.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21950) => {
                                                                                                        let source_ml_source2_data_property_value21950 = *source_ml_source2_data_property_value21950;
                                                                                                        source_ml_source2_data_property_value21950.clone()
                                                                                                    }
                                                                                                } {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21951, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value21951.clone()
                                                                                                    }
                                                                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                        let source_ml_source2_field21955 = match match initialized.clone() {
                                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21953) => {
                                                                                                                let source_ml_source2_data_property_value21953 = *source_ml_source2_data_property_value21953;
                                                                                                                source_ml_source2_data_property_value21953.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21954, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value21954.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        (source_ml_source2_field21955.clone(), templates.clone())
                                                                                                    })))
                                                                                                } else {
                                                                                                    {
                                                                                                        let code = self::compile_java_script_mutable_call_source_with_captures(match initialized.clone() {
                                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21956, _) => {
                                                                                                                let source_ml_source2_data_property_value21956 = *source_ml_source2_data_property_value21956;
                                                                                                                source_ml_source2_data_property_value21956.clone()
                                                                                                            }
                                                                                                        }, parameters.clone(), identities.clone());
                                                                                                        if ((match code.clone() {
                                                                                                            self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21958) => {
                                                                                                                source_ml_source2_data_property_value21958.clone()
                                                                                                            }
                                                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                let source_ml_source2_field21961 = match code.clone() {
                                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21960) => {
                                                                                                                        source_ml_source2_data_property_value21960.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                (source_ml_source2_field21961.clone(), templates.clone())
                                                                                                            })))
                                                                                                        } else {
                                                                                                            {
                                                                                                                let receiver_mode = match definition.clone() {
                                                                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, _, _, _, source_ml_source2_data_property_value21962) => {
                                                                                                                        ((source_ml_source2_data_property_value21962.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                if (({
                                                                                                                    let source_ml_source2_comparison_operand21964 = None::<()>;
                                                                                                                    !(match (receiver_mode.clone(), source_ml_source2_comparison_operand21964.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                } && ((receiver_mode.clone().expect("a proven present value")) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116])))) && ((receiver_mode.clone().expect("a proven present value")) != (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])))) {
                                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![114, 117, 110, 116, 105, 109, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 97, 114, 114, 111, 119, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 109, 111, 100, 101]), templates.clone()))))
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let name = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                                        {
                                                                                                                            let index = 0f64;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s21967 = self::source_ml_source1_temporary_compile_java_script_error_module_templates_loop198(definition.clone(), name.clone(), index);
                                                                                                                                match source_ml_source2_s21967.clone() {
                                                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Done => {
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_temporary_compile_java_script_error_module_templates_ignored199 = { let ml_method_owner637 = templates.clone(); let ml_method_argument637_0 = {
                                                                                                                                                let source_ml_source1_temporary_constructor_field187 = match owner.clone().expect("a proven present value") {
                                                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value21968, _, _, _, _) => {
                                                                                                                                                        source_ml_source2_data_property_value21968.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value21969, _, _) => {
                                                                                                                                                        source_ml_source2_data_property_value21969.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value21970, _) => {
                                                                                                                                                        source_ml_source2_data_property_value21970.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_temporary_constructor_field188 = match definition.clone() {
                                                                                                                                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21971, _, _, _, _, _, _) => {
                                                                                                                                                            source_ml_source2_data_property_value21971
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_temporary_constructor_field189 = match owner.clone().expect("a proven present value") {
                                                                                                                                                            self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value21972, _, _, _) => {
                                                                                                                                                                source_ml_source2_data_property_value21972.clone()
                                                                                                                                                            }
                                                                                                                                                            self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value21973, _) => {
                                                                                                                                                                source_ml_source2_data_property_value21973.clone()
                                                                                                                                                            }
                                                                                                                                                            self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value21974) => {
                                                                                                                                                                source_ml_source2_data_property_value21974.clone()
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_temporary_constructor_field190 = { let ml_reference_array638 = parameters.clone(); let ml_reference_projection638 = ml_reference_array638.borrow().len() as f64; ml_reference_projection638 };
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_temporary_constructor_field191 = match code.clone() {
                                                                                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, source_ml_source2_data_property_value21975, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value21975
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_temporary_constructor_field192 = { let ml_reference_array639 = identities.clone(); let ml_reference_projection639 = ml_reference_array639.borrow().len() as f64; ml_reference_projection639 };
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_temporary_constructor_field193 = match code.clone() {
                                                                                                                                                                            self::JavaScriptCallSourceCompilation::SourceCompiled(source_ml_source2_data_property_value21976, _, _) => {
                                                                                                                                                                                source_ml_source2_data_property_value21976.clone()
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_temporary_constructor_field194 = prototype.clone().expect("a proven present value");
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_temporary_constructor_field195 = self::JavaScriptSemanticValue::Text(name.clone());
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_temporary_constructor_field196 = match definition.clone() {
                                                                                                                                                                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, source_ml_source2_data_property_value21977, _, _, _, _) => {
                                                                                                                                                                                            source_ml_source2_data_property_value21977
                                                                                                                                                                                        }
                                                                                                                                                                                    };
                                                                                                                                                                                    if {
                                                                                                                                                                                        let source_ml_source2_comparison_operand21979 = None::<()>;
                                                                                                                                                                                        !(match (receiver_mode.clone(), source_ml_source2_comparison_operand21979.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                                                                                    } {
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_constructor_field21990 = Some(receiver_mode.clone().expect("a proven present value"));
                                                                                                                                                                                            self::JavaScriptCallTemplate::CallTemplate(source_ml_source1_temporary_constructor_field187.clone(), source_ml_source1_temporary_constructor_field188, source_ml_source1_temporary_constructor_field189.clone(), source_ml_source1_temporary_constructor_field190, source_ml_source1_temporary_constructor_field191, source_ml_source1_temporary_constructor_field192, source_ml_source1_temporary_constructor_field193.clone(), Box::new(source_ml_source1_temporary_constructor_field194.clone()), Box::new(source_ml_source1_temporary_constructor_field195.clone()), source_ml_source1_temporary_constructor_field196, Some((source_ml_source2_constructor_field21990.clone(),)), None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
