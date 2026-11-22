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
use super::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result;
use super::construct_java_script_error_constant;
use super::lower_java_script_error_expression;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop172(mut registry: self::JavaScriptErrorModuleRegistry, mut empty: self::JavaScriptCallProgram, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut source_ml_source1_temporary_values167: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorIntrinsicOwner>>>, mut source_ml_source1_temporary_index167: self::ml::Big) -> self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result {
    loop {
        return if (source_ml_source1_temporary_index167 < ({ let ml_reference_array559 = source_ml_source1_temporary_values167.clone(); let ml_reference_projection559 = self::ml::Big::from_u128(ml_reference_array559.borrow().len() as u128); ml_reference_projection559 })) {
            {
                let intrinsic = { let ml_reference_array560 = source_ml_source1_temporary_values167.clone(); let ml_reference_position560 = source_ml_source1_temporary_index167.clone(); let ml_reference_projection560 = self::ml_array::at(&*ml_reference_array560.borrow(), ml_reference_position560.to_index()); ml_reference_projection560 };
                if (self::ml_array::includes(&{ let ml_reference_value561 = identities.borrow().clone(); ml_reference_value561 }, &match intrinsic.clone() {
                    self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(source_ml_source2_data_property_value20685, _) => {
                        source_ml_source2_data_property_value20685.clone()
                    }
                }, 0f64, false, |a, b| a == b) || !(self::ml_array::includes(&{ let ml_reference_value562 = (match registry.clone() {
                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value20687, _) => {
                        source_ml_source2_data_property_value20687.clone()
                    }
                }).borrow().clone(); ml_reference_value562 }, &match intrinsic.clone() {
                    self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(_, source_ml_source2_data_property_value20688) => {
                        source_ml_source2_data_property_value20688.clone()
                    }
                }, 0f64, false, |a, b| a == b))) {
                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![101, 120, 116, 101, 114, 110, 97, 108, 32, 69, 114, 114, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 47, 114, 101, 97, 108, 109, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))))
                } else {
                    {
                        let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored176 = { let ml_method_owner563 = identities.clone(); let ml_method_argument563_0 = match intrinsic.clone() {
                            self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(source_ml_source2_data_property_value20689, _) => {
                                source_ml_source2_data_property_value20689.clone()
                            }
                        }; let ml_inserted = vec![ml_method_argument563_0]; let mut ml_method_values563 = ml_method_owner563.borrow_mut(); let ml_length = ml_method_values563.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values563.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored177 = { let ml_method_owner564 = functions.clone(); let ml_method_argument564_0 = {
                                let source_ml_source2_constructor_field20691 = match intrinsic.clone() {
                                    self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(source_ml_source2_data_property_value20690, _) => {
                                        source_ml_source2_data_property_value20690.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_constructor_field20693 = match intrinsic.clone() {
                                        self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(_, source_ml_source2_data_property_value20692) => {
                                            source_ml_source2_data_property_value20692.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field20695 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_constructor_field20696 = 0f64;
                                            {
                                                let source_ml_source2_constructor_field20697 = 2f64;
                                                {
                                                    let source_ml_source2_constructor_field20698 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                    {
                                                        let source_ml_source2_constructor_field20699 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::InvokeErrorIntrinsic(1f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source2_constructor_field20691.clone(), source_ml_source2_constructor_field20693.clone(), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source2_constructor_field20695.clone()), source_ml_source2_constructor_field20696, source_ml_source2_constructor_field20697, source_ml_source2_constructor_field20698.clone(), source_ml_source2_constructor_field20699.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
                        self::JavaScriptErrorFieldAssignment::ErrorFieldAssignment(_, source_ml_source2_data_property_value20702) => {
                            let source_ml_source2_data_property_value20702 = *source_ml_source2_data_property_value20702;
                            source_ml_source2_data_property_value20702.clone()
                        }
                    }, context.clone(), 0f64);
                    {
                        let context_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20703) => {
                                let source_ml_source2_data_property_value20703 = *source_ml_source2_data_property_value20703;
                                source_ml_source2_data_property_value20703.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored182 = { let ml_method_owner567 = writes.clone(); let ml_method_argument567_0 = self::JavaScriptCallSource::SourceCall(Box::new(self::construct_java_script_error_constant({
                                let source_ml_source2_nullish_field_receiver20704 = (realm.clone(),);
                                if match (source_ml_source2_nullish_field_receiver20704.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present20705 = ((source_ml_source2_nullish_field_receiver20704.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source2_nullish_field_present20705.clone() {
                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20706) => {
                                                let source_ml_source2_data_property_value20706 = *source_ml_source2_data_property_value20706;
                                                source_ml_source2_data_property_value20706.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 97, 115, 115, 105, 103, 110, 39, 41])).0, message_present: true }).raise()
                                }
                            })), Box::new(self::JavaScriptCallSource::SourceDerivedReceiver), std::rc::Rc::new(std::cell::RefCell::new(vec![self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(match field.clone() {
                                self::JavaScriptErrorFieldAssignment::ErrorFieldAssignment(source_ml_source2_data_property_value20707, _) => {
                                    source_ml_source2_data_property_value20707.clone()
                                }
                            })), match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20708, _) => {
                                    let source_ml_source2_data_property_value20708 = *source_ml_source2_data_property_value20708;
                                    source_ml_source2_data_property_value20708.clone()
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
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20709, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value20709
                    }
                };
                {
                    let context_2 = {
                        let source_ml_source2_nominal_field20711 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20712, _, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value20712 = *source_ml_source2_data_property_value20712;
                                source_ml_source2_data_property_value20712.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field20713 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20714, _, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value20714 = *source_ml_source2_data_property_value20714;
                                    source_ml_source2_data_property_value20714.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field20715 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20716, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value20716.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field20717 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20718, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value20718
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field20719 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20720, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value20720
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field20721 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20722, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20722.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field20723 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20724, _, _, _) => {
                                                        source_ml_source2_data_property_value20724
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field20725 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20726, _, _) => {
                                                            source_ml_source2_data_property_value20726.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field20727 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20728, _) => {
                                                                source_ml_source2_data_property_value20728.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field20729 = match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20730) => {
                                                                    source_ml_source2_data_property_value20730.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override20731 = (identity + 1f64);
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20711.clone()), Box::new(source_ml_source2_nominal_field20713.clone()), source_ml_source2_nominal_field20715.clone(), source_ml_source2_nominal_override20731, source_ml_source2_nominal_field20719, source_ml_source2_nominal_field20721.clone(), source_ml_source2_nominal_field20723, source_ml_source2_nominal_field20725.clone(), source_ml_source2_nominal_field20727.clone(), source_ml_source2_nominal_field20729.clone())
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
