// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallTemplate;
use super::JavaScriptErrorArrowTemplatePlan;
use super::JavaScriptErrorModuleBinding;
use super::JavaScriptErrorModulePlan;
use super::JavaScriptErrorModuleRegistry;
use super::JavaScriptSourceCapabilityOwner;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
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
                if (((((match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::ml_math::is_safe_integer({
                    let source_ml_source2_nullish_field_receiver21866 = (binding.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21866.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21867 = ((source_ml_source2_nullish_field_receiver21866.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21867.clone() {
                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21868, _) => {
                                    source_ml_source2_data_property_value21868
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }))) || (({
                    let source_ml_source2_nullish_field_receiver21872 = (binding.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21872.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21873 = ((source_ml_source2_nullish_field_receiver21872.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21873.clone() {
                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21874, _) => {
                                    source_ml_source2_data_property_value21874
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }) < (0f64))) || (({
                    let source_ml_source2_nullish_field_receiver21879 = (binding.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21879.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21880 = ((source_ml_source2_nullish_field_receiver21879.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21880.clone() {
                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21881, _) => {
                                    source_ml_source2_data_property_value21881
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }) >= (match plan.clone() {
                    self::JavaScriptErrorModulePlan::ErrorModulePlan(_, _, source_ml_source2_data_property_value21882, _, _, _) => {
                        source_ml_source2_data_property_value21882
                    }
                }))) || (({ let ml_values600 = { let ml_reference_value601 = bindings.borrow().clone(); ml_reference_value601 }; let ml_callback600 = { let binding = binding.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorModuleBinding| -> bool { ((match row.clone() {
                    self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21895, _) => {
                        source_ml_source2_data_property_value21895
                    }
                }) == ({
                    let source_ml_source2_nullish_field_receiver21896 = (binding.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21896.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21897 = ((source_ml_source2_nullish_field_receiver21896.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21897.clone() {
                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(source_ml_source2_data_property_value21898, _) => {
                                    source_ml_source2_data_property_value21898
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                    }
                })) }) }; ml_values600.iter().cloned().enumerate().position(|(ml_index600, ml_value600)| ml_callback600(ml_value600)).map_or(-1f64, |ml_position| ml_position as f64) }) != at)) || ((self::read_java_script_semantic_value_error({
                    let source_ml_source2_nullish_field_receiver21902 = (binding.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21902.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21903 = ((source_ml_source2_nullish_field_receiver21902.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21903.clone() {
                                self::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(_, source_ml_source2_data_property_value21904) => {
                                    let source_ml_source2_data_property_value21904 = *source_ml_source2_data_property_value21904;
                                    source_ml_source2_data_property_value21904.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41])).0, message_present: true }).raise()
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
                if ((((((((match (capability.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (({
                    let source_ml_source2_nullish_field_receiver21910 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21910.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21911 = ((source_ml_source2_nullish_field_receiver21910.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21911.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value21912, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value21912.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 111, 117, 114, 99, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }) == (self::ml_string::Text::from_units(vec![])))) || (({
                    let source_ml_source2_nullish_field_receiver21916 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21916.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21917 = ((source_ml_source2_nullish_field_receiver21916.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21917.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, source_ml_source2_data_property_value21918, _, _) => {
                                    source_ml_source2_data_property_value21918.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 119, 110, 101, 114, 39, 41])).0, message_present: true }).raise()
                    }
                }) == (self::ml_string::Text::from_units(vec![])))) || (({
                    let source_ml_source2_nullish_field_receiver21922 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21922.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21923 = ((source_ml_source2_nullish_field_receiver21922.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21923.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, source_ml_source2_data_property_value21924, _, _, _) => {
                                    source_ml_source2_data_property_value21924.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 112, 101, 114, 97, 116, 105, 111, 110, 39, 41])).0, message_present: true }).raise()
                    }
                }) == (self::ml_string::Text::from_units(vec![])))) || (({
                    let source_ml_source2_nullish_field_receiver21928 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21928.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21929 = ((source_ml_source2_nullish_field_receiver21928.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21929.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21930, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value21930.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }) <= (self::ml::Big::from_i128(0)))) || (({
                    let source_ml_source2_nullish_field_receiver21934 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21934.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21935 = ((source_ml_source2_nullish_field_receiver21934.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21935.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21936, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value21936.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::ml_array::includes(&{ let ml_reference_value604 = (match registry.clone() {
                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value21938, _) => {
                        source_ml_source2_data_property_value21938.clone()
                    }
                }).borrow().clone(); ml_reference_value604 }, &{
                    let source_ml_source2_nullish_field_receiver21939 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21939.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21940 = ((source_ml_source2_nullish_field_receiver21939.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21940.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value21941, _, _, _, _) => {
                                    source_ml_source2_data_property_value21941.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 97, 108, 109, 39, 41])).0, message_present: true }).raise()
                    }
                }, 0f64, false, |a, b| a == b))) || ({
                    let source_ml_source2_nullish_field_receiver21942 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21942.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21943 = ((source_ml_source2_nullish_field_receiver21942.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21943.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value21944) => {
                                    source_ml_source2_data_property_value21944
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                    }
                } && !({
                    let source_ml_source2_nullish_field_receiver21945 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21945.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21946 = ((source_ml_source2_nullish_field_receiver21945.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21946.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value21947, _) => {
                                    source_ml_source2_data_property_value21947
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }))) || (({ let ml_values605 = { let ml_reference_value606 = capabilities.borrow().clone(); ml_reference_value606 }; let ml_callback605 = { let capability = capability.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value21960, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value21960.clone()
                    }
                }) == ({
                    let source_ml_source2_nullish_field_receiver21961 = (capability.clone(),);
                    if match (source_ml_source2_nullish_field_receiver21961.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present21962 = ((source_ml_source2_nullish_field_receiver21961.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present21962.clone() {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value21963, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value21963.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 111, 117, 114, 99, 101, 39, 41])).0, message_present: true }).raise()
                    }
                })) }) }; ml_values605.iter().cloned().enumerate().position(|(ml_index605, ml_value605)| ml_callback605(ml_value605)).map_or(-1f64, |ml_position| ml_position as f64) }) != at)) {
                    self::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Result::MlSource1TemporaryReadJavaScriptErrorModulePlanErrorLoop184Return(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 115, 44, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 44, 32, 114, 101, 97, 108, 109, 115, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114, 115]))
                } else {
                    {
                        let previous = { let ml_values607 = { let ml_reference_value608 = capabilities.borrow().clone(); ml_reference_value608 }; let ml_callback607 = { let capability = capability.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21968, _, _, _, _, _) => {
                                source_ml_source2_data_property_value21968.clone()
                            }
                        }) == ({
                            let source_ml_source2_nullish_field_receiver21969 = (capability.clone(),);
                            if match (source_ml_source2_nullish_field_receiver21969.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source2_nullish_field_present21970 = ((source_ml_source2_nullish_field_receiver21969.clone()).0).clone().expect("a proven present value");
                                    match source_ml_source2_nullish_field_present21970.clone() {
                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value21971, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value21971.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                            }
                        })) }) }; ml_values607.iter().cloned().enumerate().position(|(ml_index607, ml_value607)| ml_callback607(ml_value607)).map_or(-1f64, |ml_position| ml_position as f64) };
                        if (previous != at) {
                            {
                                let owner = { let ml_reference_array609 = capabilities.clone(); let ml_reference_position609 = previous; let ml_reference_projection609 = self::ml_array::optional_at(&*ml_reference_array609.borrow(), self::ml_array::number_index(ml_reference_position609)); ml_reference_projection609 };
                                if (((((match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (({
                                    let source_ml_source2_nullish_field_receiver21980 = (owner.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver21980.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present21981 = ((source_ml_source2_nullish_field_receiver21980.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present21981.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value21982, _, _, _, _) => {
                                                    source_ml_source2_data_property_value21982.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 97, 108, 109, 39, 41])).0, message_present: true }).raise()
                                    }
                                }) != ({
                                    let source_ml_source2_nullish_field_receiver21983 = (capability.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver21983.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present21984 = ((source_ml_source2_nullish_field_receiver21983.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present21984.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value21985, _, _, _, _) => {
                                                    source_ml_source2_data_property_value21985.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 97, 108, 109, 39, 41])).0, message_present: true }).raise()
                                    }
                                }))) || (({
                                    let source_ml_source2_nullish_field_receiver21992 = (owner.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver21992.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present21993 = ((source_ml_source2_nullish_field_receiver21992.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present21993.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, source_ml_source2_data_property_value21994, _, _) => {
                                                    source_ml_source2_data_property_value21994.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 119, 110, 101, 114, 39, 41])).0, message_present: true }).raise()
                                    }
                                }) != ({
                                    let source_ml_source2_nullish_field_receiver21995 = (capability.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver21995.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present21996 = ((source_ml_source2_nullish_field_receiver21995.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present21996.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, source_ml_source2_data_property_value21997, _, _) => {
                                                    source_ml_source2_data_property_value21997.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 119, 110, 101, 114, 39, 41])).0, message_present: true }).raise()
                                    }
                                }))) || (({
                                    let source_ml_source2_nullish_field_receiver22004 = (owner.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver22004.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present22005 = ((source_ml_source2_nullish_field_receiver22004.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present22005.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, source_ml_source2_data_property_value22006, _, _, _) => {
                                                    source_ml_source2_data_property_value22006.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 112, 101, 114, 97, 116, 105, 111, 110, 39, 41])).0, message_present: true }).raise()
                                    }
                                }) != ({
                                    let source_ml_source2_nullish_field_receiver22007 = (capability.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver22007.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present22008 = ((source_ml_source2_nullish_field_receiver22007.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present22008.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, source_ml_source2_data_property_value22009, _, _, _) => {
                                                    source_ml_source2_data_property_value22009.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 112, 101, 114, 97, 116, 105, 111, 110, 39, 41])).0, message_present: true }).raise()
                                    }
                                }))) || (({
                                    let source_ml_source2_nullish_field_receiver22016 = (owner.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver22016.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present22017 = ((source_ml_source2_nullish_field_receiver22016.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present22017.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value22018, _) => {
                                                    source_ml_source2_data_property_value22018
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                    }
                                }) != ({
                                    let source_ml_source2_nullish_field_receiver22019 = (capability.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver22019.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present22020 = ((source_ml_source2_nullish_field_receiver22019.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present22020.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value22021, _) => {
                                                    source_ml_source2_data_property_value22021
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                    }
                                }))) || (({
                                    let source_ml_source2_nullish_field_receiver22028 = (owner.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver22028.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present22029 = ((source_ml_source2_nullish_field_receiver22028.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present22029.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value22030) => {
                                                    source_ml_source2_data_property_value22030
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                    }
                                }) != ({
                                    let source_ml_source2_nullish_field_receiver22031 = (capability.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver22031.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present22032 = ((source_ml_source2_nullish_field_receiver22031.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source2_nullish_field_present22032.clone() {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value22033) => {
                                                    source_ml_source2_data_property_value22033
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
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
            self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, source_ml_source2_data_property_value22035, _, _, _, _, _) => {
                source_ml_source2_data_property_value22035.clone()
            }
        }).length())) {
            {
                let source_ml_source1_temporary_compile_java_script_error_module_templates_ignored200 = { let ml_method_owner610 = name.clone(); let ml_method_argument610_0 = {
                    let source_ml_source2_string_unit_operand22037 = match definition.clone() {
                        self::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(_, _, source_ml_source2_data_property_value22036, _, _, _, _, _) => {
                            source_ml_source2_data_property_value22036.clone()
                        }
                    };
                    {
                        let source_ml_source2_string_unit_operand22038 = index;
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
                                    let source_ml_source2_string_unit_operand22044 = {
                                        let source_ml_source2_string_range_operand22039 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand22040 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand22041 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position22042 = source_ml_source2_string_range_operand22040;
                                                    {
                                                        let source_ml_source2_string_range_position22043 = source_ml_source2_string_range_operand22041;
                                                        source_ml_source2_string_range_operand22039.slice(source_ml_source2_string_range_position22042, source_ml_source2_string_range_position22043)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand22045 = 0f64;
                                        source_ml_source2_string_unit_operand22044.code_point_at(source_ml_source2_string_unit_operand22045)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand22037.clone(), source_ml_source2_string_unit_operand22038)
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
