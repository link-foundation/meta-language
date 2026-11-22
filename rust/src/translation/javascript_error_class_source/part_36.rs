// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptErrorFieldAssignment;
use super::JavaScriptErrorIntrinsicOwner;
use super::JavaScriptErrorModuleCompilation;
use super::JavaScriptErrorModuleRegistry;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
use super::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result;
use super::construct_java_script_error_constant;
use super::lower_java_script_error_expression;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop173(mut capabilities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>, mut empty: self::JavaScriptCallProgram, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut at: f64) -> self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result {
    loop {
        return if (at < ({ let ml_reference_array552 = capabilities.clone(); let ml_reference_projection552 = ml_reference_array552.borrow().len() as f64; ml_reference_projection552 })) {
            {
                let capability = { let ml_reference_array553 = capabilities.clone(); let ml_reference_position553 = at; let ml_reference_projection553 = self::ml_array::optional_at(&*ml_reference_array553.borrow(), self::ml_array::number_index(ml_reference_position553)); ml_reference_projection553 };
                if ((match (capability.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !({
                    let source_ml_source2_nullish_field_receiver20797 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver20797.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present20798 = ((source_ml_source2_nullish_field_receiver20797.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present20798.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value20799) => {
                                    source_ml_source2_data_property_value20799
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                    }
                })) || (({ let ml_values554 = { let ml_reference_value555 = capabilities.borrow().clone(); ml_reference_value555 }; let ml_callback554 = { let capability = capability.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20812, _, _, _, _, _) => {
                        source_ml_source2_data_property_value20812.clone()
                    }
                }) == ({
                    let source_ml_source2_nullish_field_receiver20813 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver20813.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present20814 = ((source_ml_source2_nullish_field_receiver20813.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present20814.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20815, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value20815.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                    }
                })) }) }; ml_values554.iter().cloned().enumerate().position(|(ml_index554, ml_value554)| ml_callback554(ml_value554)).map_or(-1f64, |ml_position| ml_position as f64) }) != at)) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                } else {
                    if self::ml_array::includes(&{ let ml_reference_value556 = identities.borrow().clone(); ml_reference_value556 }, &{
                        let source_ml_source2_nullish_field_receiver20816 = (capability.clone(),);
                        if match (source_ml_source2_nullish_field_receiver20816.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present20817 = ((source_ml_source2_nullish_field_receiver20816.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present20817.clone() {
                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20818, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value20818.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                        }
                    }, 0f64, false, |a, b| a == b) {
                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![97, 32, 104, 111, 115, 116, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 32, 69, 114, 114, 111, 114, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 111, 100, 121]))))
                    } else {
                        {
                            let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored174 = { let ml_method_owner557 = identities.clone(); let ml_method_argument557_0 = {
                                let source_ml_source2_nullish_field_receiver20819 = (capability.clone(),);
                                if match (source_ml_source2_nullish_field_receiver20819.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present20820 = ((source_ml_source2_nullish_field_receiver20819.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source2_nullish_field_present20820.clone() {
                                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20821, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value20821.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                }
                            }; let ml_inserted = vec![ml_method_argument557_0]; let mut ml_method_values557 = ml_method_owner557.borrow_mut(); let ml_length = ml_method_values557.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values557.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored175 = { let ml_method_owner558 = functions.clone(); let ml_method_argument558_0 = {
                                    let source_ml_source2_constructor_field20825 = {
                                        let source_ml_source2_nullish_field_receiver20822 = (capability.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver20822.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source2_nullish_field_present20823 = ((source_ml_source2_nullish_field_receiver20822.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source2_nullish_field_present20823.clone() {
                                                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20824, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value20824.clone()
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field20829 = {
                                            let source_ml_source2_nullish_field_receiver20826 = (capability.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver20826.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present20827 = ((source_ml_source2_nullish_field_receiver20826.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present20827.clone() {
                                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value20828, _, _, _, _) => {
                                                            source_ml_source2_data_property_value20828.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 97, 108, 109, 39, 41])).0, message_present: true }).raise()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field20831 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_constructor_field20832 = 0f64;
                                                {
                                                    let source_ml_source2_constructor_field20833 = 2f64;
                                                    {
                                                        let source_ml_source2_constructor_field20834 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                        {
                                                            let source_ml_source2_constructor_field20835 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::InvokeHostCapability(1f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_constructor_field20825.clone(), source_ml_source2_constructor_field20829.clone(), self::ml_string::Text::from_units(vec![113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 104, 111, 115, 116, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121]), Box::new(source_ml_source2_constructor_field20831.clone()), source_ml_source2_constructor_field20832, source_ml_source2_constructor_field20833, source_ml_source2_constructor_field20834.clone(), source_ml_source2_constructor_field20835.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }; let ml_inserted = vec![ml_method_argument558_0]; let mut ml_method_values558 = ml_method_owner558.borrow_mut(); let ml_length = ml_method_values558.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values558.extend(ml_inserted); ml_length as f64 };
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
            }
        } else {
            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result {
    MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Done,
    MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Return(Box<self::JavaScriptErrorModuleCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop172(mut registry: self::JavaScriptErrorModuleRegistry, mut empty: self::JavaScriptCallProgram, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut source_ml_source1_temporary_values167: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorIntrinsicOwner>>>, mut source_ml_source1_temporary_index167: self::ml::Big) -> self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result {
    loop {
        return if (source_ml_source1_temporary_index167 < ({ let ml_reference_array559 = source_ml_source1_temporary_values167.clone(); let ml_reference_projection559 = self::ml::Big::from_u128(ml_reference_array559.borrow().len() as u128); ml_reference_projection559 })) {
            {
                let intrinsic = { let ml_reference_array560 = source_ml_source1_temporary_values167.clone(); let ml_reference_position560 = source_ml_source1_temporary_index167.clone(); let ml_reference_projection560 = self::ml_array::at(&*ml_reference_array560.borrow(), ml_reference_position560.to_index()); ml_reference_projection560 };
                if (self::ml_array::includes(&{ let ml_reference_value561 = identities.borrow().clone(); ml_reference_value561 }, &match intrinsic.clone() {
                    self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(source_ml_source2_data_property_value20838, _) => {
                        source_ml_source2_data_property_value20838.clone()
                    }
                }, 0f64, false, |a, b| a == b) || !(self::ml_array::includes(&{ let ml_reference_value562 = (match registry.clone() {
                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value20840, _) => {
                        source_ml_source2_data_property_value20840.clone()
                    }
                }).borrow().clone(); ml_reference_value562 }, &match intrinsic.clone() {
                    self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(_, source_ml_source2_data_property_value20841) => {
                        source_ml_source2_data_property_value20841.clone()
                    }
                }, 0f64, false, |a, b| a == b))) {
                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![101, 120, 116, 101, 114, 110, 97, 108, 32, 69, 114, 114, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 47, 114, 101, 97, 108, 109, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))))
                } else {
                    {
                        let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored176 = { let ml_method_owner563 = identities.clone(); let ml_method_argument563_0 = match intrinsic.clone() {
                            self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(source_ml_source2_data_property_value20842, _) => {
                                source_ml_source2_data_property_value20842.clone()
                            }
                        }; let ml_inserted = vec![ml_method_argument563_0]; let mut ml_method_values563 = ml_method_owner563.borrow_mut(); let ml_length = ml_method_values563.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values563.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored177 = { let ml_method_owner564 = functions.clone(); let ml_method_argument564_0 = {
                                let source_ml_source2_constructor_field20844 = match intrinsic.clone() {
                                    self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(source_ml_source2_data_property_value20843, _) => {
                                        source_ml_source2_data_property_value20843.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_constructor_field20846 = match intrinsic.clone() {
                                        self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(_, source_ml_source2_data_property_value20845) => {
                                            source_ml_source2_data_property_value20845.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field20848 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_constructor_field20849 = 0f64;
                                            {
                                                let source_ml_source2_constructor_field20850 = 2f64;
                                                {
                                                    let source_ml_source2_constructor_field20851 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                    {
                                                        let source_ml_source2_constructor_field20852 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::InvokeErrorIntrinsic(1f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source2_constructor_field20844.clone(), source_ml_source2_constructor_field20846.clone(), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source2_constructor_field20848.clone()), source_ml_source2_constructor_field20849, source_ml_source2_constructor_field20850, source_ml_source2_constructor_field20851.clone(), source_ml_source2_constructor_field20852.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }; let ml_inserted = vec![ml_method_argument564_0]; let mut ml_method_values564 = ml_method_owner564.borrow_mut(); let ml_length = ml_method_values564.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values564.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index167_2 = source_ml_source1_temporary_index167.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index167 = source_ml_source1_temporary_index167_2.clone();
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Result {
    MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Done,
    MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop171Return(Box<self::JavaScriptErrorModuleCompilation>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179 {
    MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Next(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
    MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesJoin179Return(Box<self::JavaScriptErrorModuleCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop181(mut realm: Option<self::JavaScriptErrorRealmOperations>, mut context: self::JavaScriptErrorSourceContext, mut writes: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_values166: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorFieldAssignment>>>, mut source_ml_source1_temporary_index166: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index166 < ({ let ml_reference_array565 = source_ml_source1_temporary_values166.clone(); let ml_reference_projection565 = self::ml::Big::from_u128(ml_reference_array565.borrow().len() as u128); ml_reference_projection565 })) {
            {
                let field = { let ml_reference_array566 = source_ml_source1_temporary_values166.clone(); let ml_reference_position566 = source_ml_source1_temporary_index166.clone(); let ml_reference_projection566 = self::ml_array::at(&*ml_reference_array566.borrow(), ml_reference_position566.to_index()); ml_reference_projection566 };
                {
                    let value = self::lower_java_script_error_expression(match field.clone() {
                        self::JavaScriptErrorFieldAssignment::ErrorFieldAssignment(_, source_ml_source2_data_property_value20855) => {
                            let source_ml_source2_data_property_value20855 = *source_ml_source2_data_property_value20855;
                            source_ml_source2_data_property_value20855.clone()
                        }
                    }, context.clone(), 0f64);
                    {
                        let context_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20856) => {
                                let source_ml_source2_data_property_value20856 = *source_ml_source2_data_property_value20856;
                                source_ml_source2_data_property_value20856.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored182 = { let ml_method_owner567 = writes.clone(); let ml_method_argument567_0 = self::JavaScriptCallSource::SourceCall(Box::new(self::construct_java_script_error_constant({
                                let source_ml_source2_nullish_field_receiver20857 = (realm.clone(),);
                                if match (source_ml_source2_nullish_field_receiver20857.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present20858 = ((source_ml_source2_nullish_field_receiver20857.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source2_nullish_field_present20858.clone() {
                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20859) => {
                                                let source_ml_source2_data_property_value20859 = *source_ml_source2_data_property_value20859;
                                                source_ml_source2_data_property_value20859.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 97, 115, 115, 105, 103, 110, 39, 41])).0, message_present: true }).raise()
                                }
                            })), Box::new(self::JavaScriptCallSource::SourceDerivedReceiver), std::rc::Rc::new(std::cell::RefCell::new(vec![self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(match field.clone() {
                                self::JavaScriptErrorFieldAssignment::ErrorFieldAssignment(source_ml_source2_data_property_value20860, _) => {
                                    source_ml_source2_data_property_value20860.clone()
                                }
                            })), match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20861, _) => {
                                    let source_ml_source2_data_property_value20861 = *source_ml_source2_data_property_value20861;
                                    source_ml_source2_data_property_value20861.clone()
                                }
                            }]))); let ml_inserted = vec![ml_method_argument567_0]; let mut ml_method_values567 = ml_method_owner567.borrow_mut(); let ml_length = ml_method_values567.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values567.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index166_2 = source_ml_source1_temporary_index166.add(&self::ml::Big::from_i128(1));
                                {
                                    (context, source_ml_source1_temporary_index166) = (context_2.clone(), source_ml_source1_temporary_index166_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            context.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Result {
    MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Done(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop183(mut context: self::JavaScriptErrorSourceContext, mut source: self::JavaScriptCallSource, mut writes: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut at: f64) -> self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Result {
    loop {
        return if (at >= (0f64)) {
            {
                let identity = match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20862, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value20862
                    }
                };
                {
                    let context_2 = {
                        let source_ml_source2_nominal_field20864 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20865, _, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value20865 = *source_ml_source2_data_property_value20865;
                                source_ml_source2_data_property_value20865.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field20866 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20867, _, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value20867 = *source_ml_source2_data_property_value20867;
                                    source_ml_source2_data_property_value20867.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field20868 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20869, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value20869.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field20870 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20871, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value20871
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field20872 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20873, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value20873
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field20874 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20875, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20875.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field20876 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20877, _, _, _) => {
                                                        source_ml_source2_data_property_value20877
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field20878 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20879, _, _) => {
                                                            source_ml_source2_data_property_value20879.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field20880 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20881, _) => {
                                                                source_ml_source2_data_property_value20881.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field20882 = match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20883) => {
                                                                    source_ml_source2_data_property_value20883.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override20884 = (identity + 1f64);
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20864.clone()), Box::new(source_ml_source2_nominal_field20866.clone()), source_ml_source2_nominal_field20868.clone(), source_ml_source2_nominal_override20884, source_ml_source2_nominal_field20872, source_ml_source2_nominal_field20874.clone(), source_ml_source2_nominal_field20876, source_ml_source2_nominal_field20878.clone(), source_ml_source2_nominal_field20880.clone(), source_ml_source2_nominal_field20882.clone())
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
                        let source_2 = self::JavaScriptCallSource::SourceLet(identity, Box::new(({ let ml_reference_array568 = writes.clone(); let ml_reference_position568 = at; let ml_reference_projection568 = self::ml_array::optional_at(&*ml_reference_array568.borrow(), self::ml_array::number_index(ml_reference_position568)); ml_reference_projection568 }).clone().unwrap_or_else(|| self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined))), Box::new(source.clone()));
                        {
                            let at_2 = (at - 1f64);
                            {
                                (context, source, at) = (context_2.clone(), source_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop183Done(Box::new(context.clone()), Box::new(source.clone()))
        };
    }
}
