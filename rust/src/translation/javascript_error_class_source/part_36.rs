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
        return if (source_ml_source1_temporary_index167 < ({ let ml_reference_array556 = source_ml_source1_temporary_values167.clone(); let ml_reference_projection556 = self::ml::Big::from_u128(ml_reference_array556.borrow().len() as u128); ml_reference_projection556 })) {
            {
                let intrinsic = { let ml_reference_array557 = source_ml_source1_temporary_values167.clone(); let ml_reference_position557 = source_ml_source1_temporary_index167.clone(); let ml_reference_projection557 = self::ml_array::at(&*ml_reference_array557.borrow(), ml_reference_position557.to_index()); ml_reference_projection557 };
                if (self::ml_array::includes(&{ let ml_reference_value558 = identities.borrow().clone(); ml_reference_value558 }, &match intrinsic.clone() {
                    self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(source_ml_source2_data_property_value20684, _) => {
                        source_ml_source2_data_property_value20684.clone()
                    }
                }, 0f64, false, |a, b| a == b) || !(self::ml_array::includes(&{ let ml_reference_value559 = (match registry.clone() {
                    self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(_, _, _, source_ml_source2_data_property_value20686, _) => {
                        source_ml_source2_data_property_value20686.clone()
                    }
                }).borrow().clone(); ml_reference_value559 }, &match intrinsic.clone() {
                    self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(_, source_ml_source2_data_property_value20687) => {
                        source_ml_source2_data_property_value20687.clone()
                    }
                }, 0f64, false, |a, b| a == b))) {
                    self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop172Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![101, 120, 116, 101, 114, 110, 97, 108, 32, 69, 114, 114, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 47, 114, 101, 97, 108, 109, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))))
                } else {
                    {
                        let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored176 = { let ml_method_owner560 = identities.clone(); let ml_method_argument560_0 = match intrinsic.clone() {
                            self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(source_ml_source2_data_property_value20688, _) => {
                                source_ml_source2_data_property_value20688.clone()
                            }
                        }; let ml_inserted = vec![ml_method_argument560_0]; let mut ml_method_values560 = ml_method_owner560.borrow_mut(); let ml_length = ml_method_values560.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values560.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored177 = { let ml_method_owner561 = functions.clone(); let ml_method_argument561_0 = {
                                let source_ml_source2_constructor_field20690 = match intrinsic.clone() {
                                    self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(source_ml_source2_data_property_value20689, _) => {
                                        source_ml_source2_data_property_value20689.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_constructor_field20692 = match intrinsic.clone() {
                                        self::JavaScriptErrorIntrinsicOwner::ErrorIntrinsicOwner(_, source_ml_source2_data_property_value20691) => {
                                            source_ml_source2_data_property_value20691.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field20694 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_constructor_field20695 = 0f64;
                                            {
                                                let source_ml_source2_constructor_field20696 = 2f64;
                                                {
                                                    let source_ml_source2_constructor_field20697 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                    {
                                                        let source_ml_source2_constructor_field20698 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::InvokeErrorIntrinsic(1f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source2_constructor_field20690.clone(), source_ml_source2_constructor_field20692.clone(), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source2_constructor_field20694.clone()), source_ml_source2_constructor_field20695, source_ml_source2_constructor_field20696, source_ml_source2_constructor_field20697.clone(), source_ml_source2_constructor_field20698.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }; let ml_inserted = vec![ml_method_argument561_0]; let mut ml_method_values561 = ml_method_owner561.borrow_mut(); let ml_length = ml_method_values561.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values561.extend(ml_inserted); ml_length as f64 };
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
        return if (source_ml_source1_temporary_index166 < ({ let ml_reference_array562 = source_ml_source1_temporary_values166.clone(); let ml_reference_projection562 = self::ml::Big::from_u128(ml_reference_array562.borrow().len() as u128); ml_reference_projection562 })) {
            {
                let field = { let ml_reference_array563 = source_ml_source1_temporary_values166.clone(); let ml_reference_position563 = source_ml_source1_temporary_index166.clone(); let ml_reference_projection563 = self::ml_array::at(&*ml_reference_array563.borrow(), ml_reference_position563.to_index()); ml_reference_projection563 };
                {
                    let value = self::lower_java_script_error_expression(match field.clone() {
                        self::JavaScriptErrorFieldAssignment::ErrorFieldAssignment(_, source_ml_source2_data_property_value20701) => {
                            let source_ml_source2_data_property_value20701 = *source_ml_source2_data_property_value20701;
                            source_ml_source2_data_property_value20701.clone()
                        }
                    }, context.clone(), 0f64);
                    {
                        let context_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20702) => {
                                let source_ml_source2_data_property_value20702 = *source_ml_source2_data_property_value20702;
                                source_ml_source2_data_property_value20702.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored182 = { let ml_method_owner564 = writes.clone(); let ml_method_argument564_0 = self::JavaScriptCallSource::SourceCall(Box::new(self::construct_java_script_error_constant({
                                let source_ml_source2_nullish_field_receiver20703 = (realm.clone(),);
                                if match (source_ml_source2_nullish_field_receiver20703.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present20704 = ((source_ml_source2_nullish_field_receiver20703.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source2_nullish_field_present20704.clone() {
                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20705) => {
                                                let source_ml_source2_data_property_value20705 = *source_ml_source2_data_property_value20705;
                                                source_ml_source2_data_property_value20705.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 97, 115, 115, 105, 103, 110, 39, 41])).0, message_present: true }).raise()
                                }
                            })), Box::new(self::JavaScriptCallSource::SourceDerivedReceiver), std::rc::Rc::new(std::cell::RefCell::new(vec![self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(match field.clone() {
                                self::JavaScriptErrorFieldAssignment::ErrorFieldAssignment(source_ml_source2_data_property_value20706, _) => {
                                    source_ml_source2_data_property_value20706.clone()
                                }
                            })), match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20707, _) => {
                                    let source_ml_source2_data_property_value20707 = *source_ml_source2_data_property_value20707;
                                    source_ml_source2_data_property_value20707.clone()
                                }
                            }]))); let ml_inserted = vec![ml_method_argument564_0]; let mut ml_method_values564 = ml_method_owner564.borrow_mut(); let ml_length = ml_method_values564.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values564.extend(ml_inserted); ml_length as f64 };
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
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20708, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value20708
                    }
                };
                {
                    let context_2 = {
                        let source_ml_source2_nominal_field20710 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20711, _, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value20711 = *source_ml_source2_data_property_value20711;
                                source_ml_source2_data_property_value20711.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field20712 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20713, _, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value20713 = *source_ml_source2_data_property_value20713;
                                    source_ml_source2_data_property_value20713.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field20714 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20715, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value20715.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field20716 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20717, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value20717
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field20718 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20719, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value20719
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field20720 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20721, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20721.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field20722 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20723, _, _, _) => {
                                                        source_ml_source2_data_property_value20723
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field20724 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20725, _, _) => {
                                                            source_ml_source2_data_property_value20725.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field20726 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20727, _) => {
                                                                source_ml_source2_data_property_value20727.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field20728 = match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20729) => {
                                                                    source_ml_source2_data_property_value20729.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override20730 = (identity + 1f64);
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20710.clone()), Box::new(source_ml_source2_nominal_field20712.clone()), source_ml_source2_nominal_field20714.clone(), source_ml_source2_nominal_override20730, source_ml_source2_nominal_field20718, source_ml_source2_nominal_field20720.clone(), source_ml_source2_nominal_field20722, source_ml_source2_nominal_field20724.clone(), source_ml_source2_nominal_field20726.clone(), source_ml_source2_nominal_field20728.clone())
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
                        let source_2 = self::JavaScriptCallSource::SourceLet(identity, Box::new(({ let ml_reference_array565 = writes.clone(); let ml_reference_position565 = at; let ml_reference_projection565 = self::ml_array::optional_at(&*ml_reference_array565.borrow(), self::ml_array::number_index(ml_reference_position565)); ml_reference_projection565 }).clone().unwrap_or_else(|| self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined))), Box::new(source.clone()));
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
