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
        return if (at < ({ let ml_reference_array595 = bindings.clone(); let ml_reference_projection595 = ml_reference_array595.borrow().len() as f64; ml_reference_projection595 })) {
            {
                let binding = { let ml_reference_array596 = bindings.clone(); let ml_reference_position596 = at; let ml_reference_projection596 = self::ml_array::optional_at(&*ml_reference_array596.borrow(), self::ml_array::number_index(ml_reference_position596)); ml_reference_projection596 };
                if (((((match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::ml_math::is_safe_integer(match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21712, _) => {
                        source_ml_source2_data_property_value21712
                    }
                }))) || ((match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21714, _) => {
                        source_ml_source2_data_property_value21714
                    }
                }) < (0f64))) || ((match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21717, _) => {
                        source_ml_source2_data_property_value21717
                    }
                }) >= (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value21718, _, _, _) => {
                        source_ml_source2_data_property_value21718
                    }
                }))) || (({ let ml_values597 = { let ml_reference_value598 = bindings.borrow().clone(); ml_reference_value598 }; let ml_callback597 = { let binding = binding.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorModuleBinding| -> bool { ((match row.clone() {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21725, _) => {
                        source_ml_source2_data_property_value21725
                    }
                }) == (match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21726, _) => {
                        source_ml_source2_data_property_value21726
                    }
                })) }) }; ml_values597.iter().cloned().enumerate().position(|(ml_index597, ml_value597)| ml_callback597(ml_value597)).map_or(-1f64, |ml_position| ml_position as f64) }) != at)) || ((self::read_java_script_semantic_value_error(match binding.clone().expect("a proven present value") {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(_, source_ml_source2_data_property_value21728) => {
                        let source_ml_source2_data_property_value21728 = *source_ml_source2_data_property_value21728;
                        source_ml_source2_data_property_value21728.clone()
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
        return if (at < ({ let ml_reference_array599 = capabilities.clone(); let ml_reference_projection599 = ml_reference_array599.borrow().len() as f64; ml_reference_projection599 })) {
            {
                let capability = { let ml_reference_array600 = capabilities.clone(); let ml_reference_position600 = at; let ml_reference_projection600 = self::ml_array::optional_at(&*ml_reference_array600.borrow(), self::ml_array::number_index(ml_reference_position600)); ml_reference_projection600 };
                if ((((((((match (capability.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value21732, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21732.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![])))) || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, source_ml_source2_data_property_value21734, _, _) => {
                        source_ml_source2_data_property_value21734.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![])))) || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, source_ml_source2_data_property_value21736, _, _, _) => {
                        source_ml_source2_data_property_value21736.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![])))) || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21738, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21738.clone()
                    }
                }) <= (self::ml::Big::from_i128(0)))) || ((match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21740, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21740.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::ml_array::includes(&{ let ml_reference_value601 = (match registry.clone() {
                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value21742, _) => {
                        source_ml_source2_data_property_value21742.clone()
                    }
                }).borrow().clone(); ml_reference_value601 }, &match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value21743, _, _, _, _) => {
                        source_ml_source2_data_property_value21743.clone()
                    }
                }, 0f64, false, |a, b| a == b))) || (match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value21744) => {
                        source_ml_source2_data_property_value21744
                    }
                } && !(match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value21745, _) => {
                        source_ml_source2_data_property_value21745
                    }
                }))) || (({ let ml_values602 = { let ml_reference_value603 = capabilities.borrow().clone(); ml_reference_value603 }; let ml_callback602 = { let capability = capability.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value21752, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21752.clone()
                    }
                }) == (match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value21753, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21753.clone()
                    }
                })) }) }; ml_values602.iter().cloned().enumerate().position(|(ml_index602, ml_value602)| ml_callback602(ml_value602)).map_or(-1f64, |ml_position| ml_position as f64) }) != at)) {
                    self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Return(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 115, 44, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 44, 32, 114, 101, 97, 108, 109, 115, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114, 115]))
                } else {
                    {
                        let previous = { let ml_values604 = { let ml_reference_value605 = capabilities.borrow().clone(); ml_reference_value605 }; let ml_callback604 = { let capability = capability.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21756, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21756.clone()
                            }
                        }) == (match capability.clone().expect("a proven present value") {
                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21757, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21757.clone()
                            }
                        })) }) }; ml_values604.iter().cloned().enumerate().position(|(ml_index604, ml_value604)| ml_callback604(ml_value604)).map_or(-1f64, |ml_position| ml_position as f64) };
                        if (previous != at) {
                            {
                                let owner = { let ml_reference_array606 = capabilities.clone(); let ml_reference_position606 = previous; let ml_reference_projection606 = self::ml_array::optional_at(&*ml_reference_array606.borrow(), self::ml_array::number_index(ml_reference_position606)); ml_reference_projection606 };
                                if (((((match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value21762, _, _, _, _) => {
                                        source_ml_source2_data_property_value21762.clone()
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value21763, _, _, _, _) => {
                                        source_ml_source2_data_property_value21763.clone()
                                    }
                                }))) || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, source_ml_source2_data_property_value21766, _, _) => {
                                        source_ml_source2_data_property_value21766.clone()
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, source_ml_source2_data_property_value21767, _, _) => {
                                        source_ml_source2_data_property_value21767.clone()
                                    }
                                }))) || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, source_ml_source2_data_property_value21770, _, _, _) => {
                                        source_ml_source2_data_property_value21770.clone()
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, source_ml_source2_data_property_value21771, _, _, _) => {
                                        source_ml_source2_data_property_value21771.clone()
                                    }
                                }))) || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value21774, _) => {
                                        source_ml_source2_data_property_value21774
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value21775, _) => {
                                        source_ml_source2_data_property_value21775
                                    }
                                }))) || ((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value21778) => {
                                        source_ml_source2_data_property_value21778
                                    }
                                }) != (match capability.clone().expect("a proven present value") {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value21779) => {
                                        source_ml_source2_data_property_value21779
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
            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, source_ml_source2_data_property_value21781, _, _, _, _, _) => {
                source_ml_source2_data_property_value21781.clone()
            }
        }).length())) {
            {
                let source_ml_source1_temporary_compile_java_script_error_module_templates_ignored200 = { let ml_method_owner607 = name.clone(); let ml_method_argument607_0 = {
                    let source_ml_source2_string_unit_operand21783 = match definition.clone() {
                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, source_ml_source2_data_property_value21782, _, _, _, _, _) => {
                            source_ml_source2_data_property_value21782.clone()
                        }
                    };
                    {
                        let source_ml_source2_string_unit_operand21784 = index;
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
                                    let source_ml_source2_string_unit_operand21790 = {
                                        let source_ml_source2_string_range_operand21785 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand21786 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand21787 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position21788 = source_ml_source2_string_range_operand21786;
                                                    {
                                                        let source_ml_source2_string_range_position21789 = source_ml_source2_string_range_operand21787;
                                                        source_ml_source2_string_range_operand21785.slice(source_ml_source2_string_range_position21788, source_ml_source2_string_range_position21789)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand21791 = 0f64;
                                        source_ml_source2_string_unit_operand21790.code_point_at(source_ml_source2_string_unit_operand21791)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand21783.clone(), source_ml_source2_string_unit_operand21784)
                    }
                }; let ml_inserted = vec![ml_method_argument607_0]; let mut ml_method_values607 = ml_method_owner607.borrow_mut(); let ml_length = ml_method_values607.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values607.extend(ml_inserted); ml_length as f64 };
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
        return if (source_ml_source1_temporary_index186 < ({ let ml_reference_array608 = source_ml_source1_temporary_values186.clone(); let ml_reference_projection608 = self::ml::Big::from_u128(ml_reference_array608.borrow().len() as u128); ml_reference_projection608 })) {
            {
                let definition = { let ml_reference_array609 = source_ml_source1_temporary_values186.clone(); let ml_reference_position609 = source_ml_source1_temporary_index186.clone(); let ml_reference_projection609 = self::ml_array::at(&*ml_reference_array609.borrow(), ml_reference_position609.to_index()); ml_reference_projection609 };
                {
                    let owner = { let ml_values610 = { let ml_reference_value611 = (match registry.clone() {
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value21794, _, _, _, _) => {
                            source_ml_source2_data_property_value21794.clone()
                        }
                    }).borrow().clone(); ml_reference_value611 }; let ml_callback610 = { let definition = definition.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionOwner| -> bool { ((match row.clone() {
                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value21799, _, _, _, _, _) => {
                            source_ml_source2_data_property_value21799
                        }
                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value21800, _, _, _) => {
                            source_ml_source2_data_property_value21800
                        }
                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value21801, _, _) => {
                            source_ml_source2_data_property_value21801
                        }
                    }) == (match definition.clone() {
                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(source_ml_source2_data_property_value21802, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value21802
                        }
                    })) }) }; ml_values610.iter().cloned().enumerate().find_map(|(ml_index610, ml_value610)| if ml_callback610(ml_value610.clone()) { Some(ml_value610) } else { None }) };
                    {
                        let source_owner = { let ml_values612 = { let ml_reference_value613 = (match plan.clone() {
                            self::JavaScriptErrorModulePlan::ErrorModulePlan(source_ml_source2_data_property_value21803, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21803.clone()
                            }
                        }).borrow().clone(); ml_reference_value613 }; let ml_callback612 = { let definition = definition.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionPlan| -> bool { ((match row.clone() {
                            self::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(source_ml_source2_data_property_value21811, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21811
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(source_ml_source2_data_property_value21812, _, _, _, _) => {
                                source_ml_source2_data_property_value21812
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(source_ml_source2_data_property_value21813, _, _, _) => {
                                source_ml_source2_data_property_value21813
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(source_ml_source2_data_property_value21814, _, _, _) => {
                                source_ml_source2_data_property_value21814
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(source_ml_source2_data_property_value21815, _, _, _) => {
                                source_ml_source2_data_property_value21815
                            }
                            self::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(source_ml_source2_data_property_value21816, _, _, _) => {
                                source_ml_source2_data_property_value21816
                            }
                        }) == (match definition.clone() {
                            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(source_ml_source2_data_property_value21817, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21817
                            }
                        })) }) }; ml_values612.iter().cloned().enumerate().find_map(|(ml_index612, ml_value612)| if ml_callback612(ml_value612.clone()) { Some(ml_value612) } else { None }) };
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
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21849, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21849
                                        }
                                    }))) || ((match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21851, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21851
                                        }
                                    }) < (0f64))) || ((match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21853, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21853
                                        }
                                    }) >= (4096f64))) || {
                                        let source_ml_source2_same_value_operand21855 = match definition.clone() {
                                            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21854, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value21854
                                            }
                                        };
                                        {
                                            let source_ml_source2_same_value_operand21856 = (-0f64);
                                            (((source_ml_source2_same_value_operand21855 == source_ml_source2_same_value_operand21856) && ((source_ml_source2_same_value_operand21855 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand21855)) == ((1f64 / source_ml_source2_same_value_operand21856))))) || ((source_ml_source2_same_value_operand21855 != source_ml_source2_same_value_operand21855) && (source_ml_source2_same_value_operand21856 != source_ml_source2_same_value_operand21856)))
                                        }
                                    }) || { let ml_values614 = { let ml_reference_value615 = templates.borrow().clone(); ml_reference_value615 }; let ml_callback614 = { let owner = owner.clone(); let definition = definition.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallTemplate| -> bool { (((match row.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value21861, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21861.clone()
                                        }
                                    }) == (match owner.clone().expect("a proven present value") {
                                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value21862, _, _, _, _) => {
                                            source_ml_source2_data_property_value21862.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value21863, _, _) => {
                                            source_ml_source2_data_property_value21863.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value21864, _) => {
                                            source_ml_source2_data_property_value21864.clone()
                                        }
                                    })) && ((match row.clone() {
                                        self::JavaScriptCallTemplate::CallTemplate(_, source_ml_source2_data_property_value21867, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21867
                                        }
                                    }) == (match definition.clone() {
                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21868, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21868
                                        }
                                    }))) }) }; ml_values614.iter().cloned().enumerate().any(|(ml_index614, ml_value614)| ml_callback614(ml_value614)) }) {
                                        self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((owner_error.clone(), templates.clone()))))
                                    } else {
                                        {
                                            let realm = { let ml_values616 = { let ml_reference_value617 = (match registry.clone() {
                                                self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, source_ml_source2_data_property_value21871, _, _, _) => {
                                                    source_ml_source2_data_property_value21871.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value617 }; let ml_callback616 = { let owner = owner.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorRealmOperations| -> bool { ((match row.clone() {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value21876, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value21876.clone()
                                                }
                                            }) == (match owner.clone().expect("a proven present value") {
                                                self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value21877, _, _, _) => {
                                                    source_ml_source2_data_property_value21877.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value21878, _) => {
                                                    source_ml_source2_data_property_value21878.clone()
                                                }
                                                self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value21879) => {
                                                    source_ml_source2_data_property_value21879.clone()
                                                }
                                            })) }) }; ml_values616.iter().cloned().enumerate().find_map(|(ml_index616, ml_value616)| if ml_callback616(ml_value616.clone()) { Some(ml_value616) } else { None }) };
                                            if match (realm.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 115, 111, 117, 114, 99, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]), templates.clone()))))
                                            } else {
                                                {
                                                    let prototype = match realm.clone().expect("a proven present value") {
                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, source_ml_source2_data_property_value21882, _, _, _, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value21882.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    };
                                                    if match (prototype.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 115, 111, 117, 114, 99, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]), templates.clone()))))
                                                    } else {
                                                        {
                                                            let first_default = { let ml_values618 = { let ml_reference_value619 = (match definition.clone() {
                                                                self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value21885, _, _, _) => {
                                                                    source_ml_source2_data_property_value21885.clone()
                                                                }
                                                            }).borrow().clone(); ml_reference_value619 }; let ml_callback618 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> bool { match parameter.clone() {
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
                                                            } }); ml_values618.iter().cloned().enumerate().position(|(ml_index618, ml_value618)| ml_callback618(ml_value618)).map_or(-1f64, |ml_position| ml_position as f64) };
                                                            {
                                                                let length = if (first_default < (0f64)) {
                                                                    { let ml_reference_array620 = match definition.clone() {
                                                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value21904, _, _, _) => {
                                                                            source_ml_source2_data_property_value21904.clone()
                                                                        }
                                                                    }; let ml_reference_projection620 = ml_reference_array620.borrow().len() as f64; ml_reference_projection620 }
                                                                } else {
                                                                    first_default
                                                                };
                                                                if ((match definition.clone() {
                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, source_ml_source2_data_property_value21906, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value21906
                                                                    }
                                                                }) != length) {
                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 98, 111, 117, 110, 100, 97, 114, 121]), templates.clone()))))
                                                                } else {
                                                                    {
                                                                        let identities = std::rc::Rc::new(std::cell::RefCell::new({
                                                                            let source_ml_source2_concat_value21911 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values621 = { let ml_reference_value622 = bindings.borrow().clone(); ml_reference_value622 }; let ml_callback621 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21909, _) => {
                                                                                    source_ml_source2_data_property_value21909
                                                                                }
                                                                            } }); ml_values621.iter().cloned().enumerate().map(|(ml_index621, ml_value621)| ml_callback621(ml_value621)).collect::<Vec<_>>() }));
                                                                            {
                                                                                let source_ml_source2_concat_value21912 = match definition.clone() {
                                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, _, source_ml_source2_data_property_value21910, _, _) => {
                                                                                        source_ml_source2_data_property_value21910.clone()
                                                                                    }
                                                                                };
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value623 = source_ml_source2_concat_value21911.borrow().clone(); ml_reference_value623 }), { let ml_reference_value624 = source_ml_source2_concat_value21912.borrow().clone(); ml_reference_value624 })
                                                                            }
                                                                        }));
                                                                        {
                                                                            let parameters = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values625 = { let ml_reference_value626 = (match definition.clone() {
                                                                                self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value21913, _, _, _) => {
                                                                                    source_ml_source2_data_property_value21913.clone()
                                                                                }
                                                                            }).borrow().clone(); ml_reference_value626 }; let ml_callback625 = std::rc::Rc::new(move |parameter: self::JavaScriptErrorParameter| -> f64 { match parameter.clone() {
                                                                                self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value21914, _, _) => {
                                                                                    source_ml_source2_data_property_value21914
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value21915, _) => {
                                                                                    source_ml_source2_data_property_value21915
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value21916, _, _) => {
                                                                                    source_ml_source2_data_property_value21916
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value21917, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value21917
                                                                                }
                                                                                self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value21918, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value21918
                                                                                }
                                                                            } }); ml_values625.iter().cloned().enumerate().map(|(ml_index625, ml_value625)| ml_callback625(ml_value625)).collect::<Vec<_>>() }));
                                                                            if ((({ let ml_reference_array627 = identities.clone(); let ml_reference_projection627 = ml_reference_array627.borrow().len() as f64; ml_reference_projection627 }) > (4096f64)) || { let ml_values628 = { let ml_reference_value629 = identities.borrow().clone(); ml_reference_value629 }; let ml_callback628 = { let identities = identities.clone(); let parameters = parameters.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { (((self::ml_array::index_of(&{ let ml_reference_value630 = identities.borrow().clone(); ml_reference_value630 }, &identity, 0f64, false, |a, b| a == b)) != index) || self::ml_array::includes(&{ let ml_reference_value631 = parameters.borrow().clone(); ml_reference_value631 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) }) }; ml_values628.iter().cloned().enumerate().any(|(ml_index628, ml_value628)| ml_callback628(ml_value628, (ml_index628 as f64))) }) {
                                                                                self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 97, 110, 100, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 105, 115, 106, 111, 105, 110, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 115]), templates.clone()))))
                                                                            } else {
                                                                                {
                                                                                    let context = {
                                                                                        let source_ml_source2_constructor_field21921 = registry.clone();
                                                                                        {
                                                                                            let source_ml_source2_constructor_field21922 = realm.clone().expect("a proven present value");
                                                                                            {
                                                                                                let source_ml_source2_constructor_field21923 = capabilities.clone();
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field21925 = Some((match plan.clone() {
                                                                                                        self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, source_ml_source2_data_property_value21924, _) => {
                                                                                                            ((source_ml_source2_data_property_value21924.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field21927 = Some((match plan.clone() {
                                                                                                            self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, _, _, _, source_ml_source2_data_property_value21926) => {
                                                                                                                ((source_ml_source2_data_property_value21926.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new()))));
                                                                                                        {
                                                                                                            let source_ml_source2_constructor_field21929 = Some(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values632 = { let ml_reference_value633 = bindings.borrow().clone(); ml_reference_value633 }; let ml_callback632 = std::rc::Rc::new(move |binding: self::JavaScriptErrorModuleBinding| -> f64 { match binding.clone() {
                                                                                                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21928, _) => {
                                                                                                                    source_ml_source2_data_property_value21928
                                                                                                                }
                                                                                                            } }); ml_values632.iter().cloned().enumerate().map(|(ml_index632, ml_value632)| ml_callback632(ml_value632)).collect::<Vec<_>>() })));
                                                                                                            {
                                                                                                                let source_ml_source2_constructor_field21931 = match plan.clone() {
                                                                                                                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value21930, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value21930
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_constructor_field21932 = 0f64;
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_constructor_field21921.clone()), Box::new(source_ml_source2_constructor_field21922.clone()), source_ml_source2_constructor_field21923.clone(), source_ml_source2_constructor_field21931, source_ml_source2_constructor_field21932, self::ml_string::Text::from_units(vec![]), true, Some((source_ml_source2_constructor_field21929.clone(),)), Some((source_ml_source2_constructor_field21925.clone(),)), Some((source_ml_source2_constructor_field21927.clone(),)))
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
                                                                                            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, _, _, source_ml_source2_data_property_value21935, _) => {
                                                                                                source_ml_source2_data_property_value21935.clone()
                                                                                            }
                                                                                        }, 0f64, context.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), 0f64, true);
                                                                                        if ((match match lowered.clone() {
                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21938) => {
                                                                                                let source_ml_source2_data_property_value21938 = *source_ml_source2_data_property_value21938;
                                                                                                source_ml_source2_data_property_value21938.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21939, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value21939.clone()
                                                                                            }
                                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                let source_ml_source2_field21943 = match match lowered.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21941) => {
                                                                                                        let source_ml_source2_data_property_value21941 = *source_ml_source2_data_property_value21941;
                                                                                                        source_ml_source2_data_property_value21941.clone()
                                                                                                    }
                                                                                                } {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21942, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value21942.clone()
                                                                                                    }
                                                                                                };
                                                                                                (source_ml_source2_field21943.clone(), templates.clone())
                                                                                            })))
                                                                                        } else {
                                                                                            {
                                                                                                let initialized = self::lower_java_script_source_parameters(match definition.clone() {
                                                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, source_ml_source2_data_property_value21944, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value21944.clone()
                                                                                                    }
                                                                                                }, match lowered.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21945) => {
                                                                                                        let source_ml_source2_data_property_value21945 = *source_ml_source2_data_property_value21945;
                                                                                                        source_ml_source2_data_property_value21945.clone()
                                                                                                    }
                                                                                                }, match lowered.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21946, _) => {
                                                                                                        let source_ml_source2_data_property_value21946 = *source_ml_source2_data_property_value21946;
                                                                                                        source_ml_source2_data_property_value21946.clone()
                                                                                                    }
                                                                                                });
                                                                                                if ((match match initialized.clone() {
                                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21949) => {
                                                                                                        let source_ml_source2_data_property_value21949 = *source_ml_source2_data_property_value21949;
                                                                                                        source_ml_source2_data_property_value21949.clone()
                                                                                                    }
                                                                                                } {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21950, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value21950.clone()
                                                                                                    }
                                                                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                        let source_ml_source2_field21954 = match match initialized.clone() {
                                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value21952) => {
                                                                                                                let source_ml_source2_data_property_value21952 = *source_ml_source2_data_property_value21952;
                                                                                                                source_ml_source2_data_property_value21952.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value21953, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value21953.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        (source_ml_source2_field21954.clone(), templates.clone())
                                                                                                    })))
                                                                                                } else {
                                                                                                    {
                                                                                                        let code = self::compile_java_script_mutable_call_source_with_captures(match initialized.clone() {
                                                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value21955, _) => {
                                                                                                                let source_ml_source2_data_property_value21955 = *source_ml_source2_data_property_value21955;
                                                                                                                source_ml_source2_data_property_value21955.clone()
                                                                                                            }
                                                                                                        }, parameters.clone(), identities.clone());
                                                                                                        if ((match code.clone() {
                                                                                                            self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21957) => {
                                                                                                                source_ml_source2_data_property_value21957.clone()
                                                                                                            }
                                                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                            self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                let source_ml_source2_field21960 = match code.clone() {
                                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, _, source_ml_source2_data_property_value21959) => {
                                                                                                                        source_ml_source2_data_property_value21959.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                (source_ml_source2_field21960.clone(), templates.clone())
                                                                                                            })))
                                                                                                        } else {
                                                                                                            {
                                                                                                                let receiver_mode = match definition.clone() {
                                                                                                                    self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, _, _, _, _, source_ml_source2_data_property_value21961) => {
                                                                                                                        ((source_ml_source2_data_property_value21961.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                if (({
                                                                                                                    let source_ml_source2_comparison_operand21963 = None::<()>;
                                                                                                                    !(match (receiver_mode.clone(), source_ml_source2_comparison_operand21963.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                } && ((receiver_mode.clone().expect("a proven present value")) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116])))) && ((receiver_mode.clone().expect("a proven present value")) != (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])))) {
                                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop197Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![114, 117, 110, 116, 105, 109, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 97, 114, 114, 111, 119, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 109, 111, 100, 101]), templates.clone()))))
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let name = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                                        {
                                                                                                                            let index = 0f64;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s21966 = self::source_ml_source1_temporary_compile_java_script_error_module_templates_loop198(definition.clone(), name.clone(), index);
                                                                                                                                match source_ml_source2_s21966.clone() {
                                                                                                                                    self::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Result::MlSource1TemporaryCompileJavaScriptErrorModuleTemplatesLoop198Done => {
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_temporary_compile_java_script_error_module_templates_ignored199 = { let ml_method_owner634 = templates.clone(); let ml_method_argument634_0 = {
                                                                                                                                                let source_ml_source1_temporary_constructor_field187 = match owner.clone().expect("a proven present value") {
                                                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value21967, _, _, _, _) => {
                                                                                                                                                        source_ml_source2_data_property_value21967.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value21968, _, _) => {
                                                                                                                                                        source_ml_source2_data_property_value21968.clone()
                                                                                                                                                    }
                                                                                                                                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value21969, _) => {
                                                                                                                                                        source_ml_source2_data_property_value21969.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source1_temporary_constructor_field188 = match definition.clone() {
                                                                                                                                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, source_ml_source2_data_property_value21970, _, _, _, _, _, _) => {
                                                                                                                                                            source_ml_source2_data_property_value21970
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_temporary_constructor_field189 = match owner.clone().expect("a proven present value") {
                                                                                                                                                            self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value21971, _, _, _) => {
                                                                                                                                                                source_ml_source2_data_property_value21971.clone()
                                                                                                                                                            }
                                                                                                                                                            self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value21972, _) => {
                                                                                                                                                                source_ml_source2_data_property_value21972.clone()
                                                                                                                                                            }
                                                                                                                                                            self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value21973) => {
                                                                                                                                                                source_ml_source2_data_property_value21973.clone()
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_temporary_constructor_field190 = { let ml_reference_array635 = parameters.clone(); let ml_reference_projection635 = ml_reference_array635.borrow().len() as f64; ml_reference_projection635 };
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_temporary_constructor_field191 = match code.clone() {
                                                                                                                                                                    self::JavaScriptCallSourceCompilation::SourceCompiled(_, source_ml_source2_data_property_value21974, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value21974
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_temporary_constructor_field192 = { let ml_reference_array636 = identities.clone(); let ml_reference_projection636 = ml_reference_array636.borrow().len() as f64; ml_reference_projection636 };
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_temporary_constructor_field193 = match code.clone() {
                                                                                                                                                                            self::JavaScriptCallSourceCompilation::SourceCompiled(source_ml_source2_data_property_value21975, _, _) => {
                                                                                                                                                                                source_ml_source2_data_property_value21975.clone()
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_temporary_constructor_field194 = prototype.clone().expect("a proven present value");
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_temporary_constructor_field195 = self::JavaScriptSemanticValue::Text(name.clone());
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source1_temporary_constructor_field196 = match definition.clone() {
                                                                                                                                                                                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, _, source_ml_source2_data_property_value21976, _, _, _, _) => {
                                                                                                                                                                                            source_ml_source2_data_property_value21976
                                                                                                                                                                                        }
                                                                                                                                                                                    };
                                                                                                                                                                                    if {
                                                                                                                                                                                        let source_ml_source2_comparison_operand21978 = None::<()>;
                                                                                                                                                                                        !(match (receiver_mode.clone(), source_ml_source2_comparison_operand21978.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                                                                                                                    } {
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_constructor_field21989 = Some(receiver_mode.clone().expect("a proven present value"));
                                                                                                                                                                                            self::JavaScriptCallTemplate::CallTemplate(source_ml_source1_temporary_constructor_field187.clone(), source_ml_source1_temporary_constructor_field188, source_ml_source1_temporary_constructor_field189.clone(), source_ml_source1_temporary_constructor_field190, source_ml_source1_temporary_constructor_field191, source_ml_source1_temporary_constructor_field192, source_ml_source1_temporary_constructor_field193.clone(), Box::new(source_ml_source1_temporary_constructor_field194.clone()), Box::new(source_ml_source1_temporary_constructor_field195.clone()), source_ml_source1_temporary_constructor_field196, Some((source_ml_source2_constructor_field21989.clone(),)), None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
                                                                                                                                            }; let ml_inserted = vec![ml_method_argument634_0]; let mut ml_method_values634 = ml_method_owner634.borrow_mut(); let ml_length = ml_method_values634.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values634.extend(ml_inserted); ml_length as f64 };
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
                let value = { let ml_reference_array637 = ordered.clone(); let ml_reference_position637 = index; let ml_reference_projection637 = self::ml_array::optional_at(&*ml_reference_array637.borrow(), self::ml_array::number_index(ml_reference_position637)); ml_reference_projection637 };
                if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value638 = templates.borrow().clone(); ml_reference_value638 }, 0f64, f64::INFINITY))))
                } else {
                    {
                        let source_ml_source1_temporary_order_java_script_error_call_templates_ignored206 = { let ml_method_owner639 = merged.clone(); let ml_method_argument639_0 = value.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument639_0]; let mut ml_method_values639 = ml_method_owner639.borrow_mut(); let ml_length = ml_method_values639.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values639.extend(ml_inserted); ml_length as f64 };
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
                let value = { let ml_reference_array640 = ordered.clone(); let ml_reference_position640 = index; let ml_reference_projection640 = self::ml_array::optional_at(&*ml_reference_array640.borrow(), self::ml_array::number_index(ml_reference_position640)); ml_reference_projection640 };
                if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value641 = templates.borrow().clone(); ml_reference_value641 }, 0f64, f64::INFINITY))))
                } else {
                    {
                        let source_ml_source1_temporary_order_java_script_error_call_templates_ignored207 = { let ml_method_owner642 = merged.clone(); let ml_method_argument642_0 = value.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument642_0]; let mut ml_method_values642 = ml_method_owner642.borrow_mut(); let ml_length = ml_method_values642.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values642.extend(ml_inserted); ml_length as f64 };
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
