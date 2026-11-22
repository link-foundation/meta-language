// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallSource;
use super::JavaScriptErrorArrayBindingPart;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptPatternBindingDeclarations;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceBindingTarget;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceStatement;
use super::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result;
use super::append_java_script_pattern_binding_declarations;
use super::collect_java_script_pattern_binding_declarations;
use super::collect_java_script_source_lexical_declarations;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_pattern_binding;
use super::lower_java_script_error_expression;
use super::lower_java_script_source_binding_pattern;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_pattern_binding_declarations_loop217(mut mutable: bool, mut depth: f64, mut shared: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_values212: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>, mut source_ml_source1_temporary_index212: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result {
    loop {
        return if (source_ml_source1_temporary_index212 < ({ let ml_reference_array659 = source_ml_source1_temporary_values212.clone(); let ml_reference_projection659 = self::ml::Big::from_u128(ml_reference_array659.borrow().len() as u128); ml_reference_projection659 })) {
            {
                let field = { let ml_reference_array660 = source_ml_source1_temporary_values212.clone(); let ml_reference_position660 = source_ml_source1_temporary_index212.clone(); let ml_reference_projection660 = self::ml_array::at(&*ml_reference_array660.borrow(), ml_reference_position660.to_index()); ml_reference_projection660 };
                match field.clone() {
                    self::JavaScriptErrorArrayBindingPart::ErrorArrayBindingElement(identity, _, _, pattern, _) => {
                        {
                            let child = self::append_java_script_pattern_binding_declarations(declarations.clone(), identity, ((pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone(), mutable, depth, shared.clone());
                            if ((match child.clone() {
                                self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value22299) => {
                                    source_ml_source2_data_property_value22299.clone()
                                }
                            }) != (self::ml_string::Text::from_units(vec![]))) {
                                self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Return(Box::new(child.clone()))
                            } else {
                                {
                                    let source_ml_source1_temporary_index212_2 = source_ml_source1_temporary_index212.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index212 = source_ml_source1_temporary_index212_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptErrorArrayBindingPart::ErrorArrayElision => {
                        {
                            let source_ml_source1_temporary_index212_3 = source_ml_source1_temporary_index212.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index212 = source_ml_source1_temporary_index212_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221 {
    MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221Next,
    MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221Return(Box<self::JavaScriptPatternBindingDeclarations>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Result {
    MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_append_java_script_pattern_binding_declarations_loop222(mut declarations: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_values219: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_index219: self::ml::Big) -> self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Result {
    loop {
        return if (source_ml_source1_temporary_index219 < ({ let ml_reference_array661 = source_ml_source1_temporary_values219.clone(); let ml_reference_projection661 = self::ml::Big::from_u128(ml_reference_array661.borrow().len() as u128); ml_reference_projection661 })) {
            {
                let declaration = { let ml_reference_array662 = source_ml_source1_temporary_values219.clone(); let ml_reference_position662 = source_ml_source1_temporary_index219.clone(); let ml_reference_projection662 = self::ml_array::at(&*ml_reference_array662.borrow(), ml_reference_position662.to_index()); ml_reference_projection662 };
                {
                    let source_ml_source1_temporary_append_java_script_pattern_binding_declarations_ignored223 = { let ml_method_owner663 = declarations.clone(); let ml_method_argument663_0 = declaration.clone(); let ml_inserted = vec![ml_method_argument663_0]; let mut ml_method_values663 = ml_method_owner663.borrow_mut(); let ml_length = ml_method_values663.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values663.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_source1_temporary_index219_2 = source_ml_source1_temporary_index219.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index219 = source_ml_source1_temporary_index219_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Result::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_array_binding_loop224(pattern: self::JavaScriptErrorParameter, mutable: bool, depth: f64, context: std::rc::Rc<std::cell::RefCell<self::JavaScriptErrorSourceContext>>, source: self::JavaScriptCallSource, at: f64) -> self::JavaScriptCallSource {
    if (at >= (0f64)) {
        {
            let element = {
                let source_ml_source2_nullish_field_receiver22303 = (match pattern.clone() {
                    self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>>
                    }
                    self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>>
                    }
                    self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>>
                    }
                    self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>>
                    }
                    self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, source_ml_source2_data_property_value22302, _) => {
                        Some(source_ml_source2_data_property_value22302.clone())
                    }
                }, at);
                {
                    let source_ml_source2_computed_property_key22304 = (source_ml_source2_nullish_field_receiver22303.clone()).1;
                    if match (source_ml_source2_nullish_field_receiver22303.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present22305 = ((source_ml_source2_nullish_field_receiver22303.clone()).0).clone().expect("a proven present value");
                            { let ml_reference_array664 = source_ml_source2_nullish_field_present22305.clone(); let ml_reference_position664 = source_ml_source2_computed_property_key22304; let ml_reference_projection664 = self::ml_array::optional_at(&*ml_reference_array664.borrow(), self::ml_array::number_index(ml_reference_position664)); ml_reference_projection664 }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source2_nullish_field_receiver22303.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
                    }
                }
            };
            if match (element.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                {
                    let at_2 = (at - 1f64);
                    self::source_ml_source1_temporary_lower_java_script_source_array_binding_loop224(pattern.clone(), mutable, depth, context.clone(), source.clone(), at_2)
                }
            } else {
                match element.clone().expect("a proven present value") {
                    self::JavaScriptErrorArrayBindingPart::ErrorArrayBindingElement(element_identity, has_default, element_value, element_pattern, target) => {
                        let element_value = *element_value;
                        {
                            let nested_pattern = ((element_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                            {
                                let source_3 = if !(match (nested_pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let nested = self::lower_java_script_source_binding_pattern(nested_pattern.clone().expect("a proven present value"), { let ml_cell_value = context.borrow().clone(); ml_cell_value }, source.clone(), mutable, (depth + 1f64), true);
                                        { let ml_cell665 = context.clone(); let ml_value665 = match nested.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value22308) => {
                                                let source_ml_source2_data_property_value22308 = *source_ml_source2_data_property_value22308;
                                                source_ml_source2_data_property_value22308.clone()
                                            }
                                        }; *ml_cell665.borrow_mut() = ml_value665; {
                                            let source_2 = match nested.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value22309, _) => {
                                                    let source_ml_source2_data_property_value22309 = *source_ml_source2_data_property_value22309;
                                                    source_ml_source2_data_property_value22309.clone()
                                                }
                                            };
                                            source_2.clone()
                                        } }
                                    }
                                } else {
                                    source.clone()
                                };
                                {
                                    let identity = match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value22310, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value22310
                                        }
                                    };
                                    { let ml_cell666 = context.clone(); let ml_value666 = {
                                        let source_ml_source2_nominal_spread22311 = { let ml_cell_value = context.borrow().clone(); ml_cell_value };
                                        {
                                            let source_ml_source2_nominal_field22312 = match source_ml_source2_nominal_spread22311.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value22313, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value22313 = *source_ml_source2_data_property_value22313;
                                                    source_ml_source2_data_property_value22313.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field22314 = match source_ml_source2_nominal_spread22311.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value22315, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value22315 = *source_ml_source2_data_property_value22315;
                                                        source_ml_source2_data_property_value22315.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field22316 = match source_ml_source2_nominal_spread22311.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value22317, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22317.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field22318 = match source_ml_source2_nominal_spread22311.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value22319, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value22319
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field22320 = match source_ml_source2_nominal_spread22311.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value22321, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value22321
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field22322 = match source_ml_source2_nominal_spread22311.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value22323, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value22323.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field22324 = match source_ml_source2_nominal_spread22311.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value22325, _, _, _) => {
                                                                            source_ml_source2_data_property_value22325
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field22326 = match source_ml_source2_nominal_spread22311.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value22327, _, _) => {
                                                                                source_ml_source2_data_property_value22327.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field22328 = match source_ml_source2_nominal_spread22311.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value22329, _) => {
                                                                                    source_ml_source2_data_property_value22329.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field22330 = match source_ml_source2_nominal_spread22311.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22331) => {
                                                                                        source_ml_source2_data_property_value22331.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override22332 = (identity + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field22312.clone()), Box::new(source_ml_source2_nominal_field22314.clone()), source_ml_source2_nominal_field22316.clone(), source_ml_source2_nominal_override22332, source_ml_source2_nominal_field22320, source_ml_source2_nominal_field22322.clone(), source_ml_source2_nominal_field22324, source_ml_source2_nominal_field22326.clone(), source_ml_source2_nominal_field22328.clone(), source_ml_source2_nominal_field22330.clone())
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
                                    }; *ml_cell666.borrow_mut() = ml_value666; {
                                        let saved = self::JavaScriptCallSource::SourceBinding(identity);
                                        {
                                            let value = saved.clone();
                                            {
                                                let value_3 = if has_default {
                                                    {
                                                        let lowered = self::lower_java_script_error_expression(element_value.clone(), { let ml_cell_value = context.borrow().clone(); ml_cell_value }, 0f64);
                                                        { let ml_cell667 = context.clone(); let ml_value667 = match lowered.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value22333) => {
                                                                let source_ml_source2_data_property_value22333 = *source_ml_source2_data_property_value22333;
                                                                source_ml_source2_data_property_value22333.clone()
                                                            }
                                                        }; *ml_cell667.borrow_mut() = ml_value667; {
                                                            let value_2 = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(saved.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)))), Box::new(match lowered.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value22334, _) => {
                                                                    let source_ml_source2_data_property_value22334 = *source_ml_source2_data_property_value22334;
                                                                    source_ml_source2_data_property_value22334.clone()
                                                                }
                                                            }), Box::new(saved.clone()));
                                                            value_2.clone()
                                                        } }
                                                    }
                                                } else {
                                                    value.clone()
                                                };
                                                {
                                                    let source_4 = self::construct_java_script_error_local_binding({ let ml_cell_value = context.borrow().clone(); ml_cell_value }, identity, self::JavaScriptCallSource::SourceIteratorStep(match pattern.clone() {
                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value22335, _, _) => {
                                                            source_ml_source2_data_property_value22335
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value22336, _) => {
                                                            source_ml_source2_data_property_value22336
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value22337, _, _) => {
                                                            source_ml_source2_data_property_value22337
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value22338, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22338
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value22339, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22339
                                                        }
                                                    }, match match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value22340, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value22340 = *source_ml_source2_data_property_value22340;
                                                            source_ml_source2_data_property_value22340.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value22341, _, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22341.clone()
                                                        }
                                                    }, true), self::construct_java_script_source_pattern_binding({ let ml_cell_value = context.borrow().clone(); ml_cell_value }, element_identity, value_3.clone(), source_3.clone(), mutable, true, ((target.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone()), false);
                                                    {
                                                        let at_3 = (at - 1f64);
                                                        self::source_ml_source1_temporary_lower_java_script_source_array_binding_loop224(pattern.clone(), mutable, depth, context.clone(), source_4.clone(), at_3)
                                                    }
                                                }
                                            }
                                        }
                                    } }
                                }
                            }
                        }
                    }
                    self::JavaScriptErrorArrayBindingPart::ErrorArrayElision => {
                        {
                            let identity_2 = match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value22342, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22342
                                }
                            };
                            { let ml_cell668 = context.clone(); let ml_value668 = {
                                let source_ml_source2_nominal_spread22343 = { let ml_cell_value = context.borrow().clone(); ml_cell_value };
                                {
                                    let source_ml_source2_nominal_field22344 = match source_ml_source2_nominal_spread22343.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value22345, _, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value22345 = *source_ml_source2_data_property_value22345;
                                            source_ml_source2_data_property_value22345.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field22346 = match source_ml_source2_nominal_spread22343.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value22347, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value22347 = *source_ml_source2_data_property_value22347;
                                                source_ml_source2_data_property_value22347.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field22348 = match source_ml_source2_nominal_spread22343.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value22349, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value22349.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field22350 = match source_ml_source2_nominal_spread22343.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value22351, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value22351
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field22352 = match source_ml_source2_nominal_spread22343.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value22353, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22353
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field22354 = match source_ml_source2_nominal_spread22343.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value22355, _, _, _, _) => {
                                                                source_ml_source2_data_property_value22355.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field22356 = match source_ml_source2_nominal_spread22343.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value22357, _, _, _) => {
                                                                    source_ml_source2_data_property_value22357
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field22358 = match source_ml_source2_nominal_spread22343.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value22359, _, _) => {
                                                                        source_ml_source2_data_property_value22359.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field22360 = match source_ml_source2_nominal_spread22343.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value22361, _) => {
                                                                            source_ml_source2_data_property_value22361.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field22362 = match source_ml_source2_nominal_spread22343.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22363) => {
                                                                                source_ml_source2_data_property_value22363.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override22364 = (identity_2 + 1f64);
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field22344.clone()), Box::new(source_ml_source2_nominal_field22346.clone()), source_ml_source2_nominal_field22348.clone(), source_ml_source2_nominal_override22364, source_ml_source2_nominal_field22352, source_ml_source2_nominal_field22354.clone(), source_ml_source2_nominal_field22356, source_ml_source2_nominal_field22358.clone(), source_ml_source2_nominal_field22360.clone(), source_ml_source2_nominal_field22362.clone())
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
                            }; *ml_cell668.borrow_mut() = ml_value668; {
                                let source_5 = self::construct_java_script_error_local_binding({ let ml_cell_value = context.borrow().clone(); ml_cell_value }, identity_2, self::JavaScriptCallSource::SourceIteratorStep(match pattern.clone() {
                                    self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value22365, _, _) => {
                                        source_ml_source2_data_property_value22365
                                    }
                                    self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value22366, _) => {
                                        source_ml_source2_data_property_value22366
                                    }
                                    self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value22367, _, _) => {
                                        source_ml_source2_data_property_value22367
                                    }
                                    self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value22368, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value22368
                                    }
                                    self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value22369, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value22369
                                    }
                                }, match match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value22370, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value22370 = *source_ml_source2_data_property_value22370;
                                        source_ml_source2_data_property_value22370.clone()
                                    }
                                } {
                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value22371, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value22371.clone()
                                    }
                                }, false), source.clone(), false);
                                {
                                    let at_4 = (at - 1f64);
                                    self::source_ml_source1_temporary_lower_java_script_source_array_binding_loop224(pattern.clone(), mutable, depth, context.clone(), source_5.clone(), at_4)
                                }
                            } }
                        }
                    }
                }
            }
        }
    } else {
        source.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Result {
    MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_lexical_cell_failure_loop225(message: &self::ml_string::Text, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Result {
    loop {
        return if (at < (message.length())) {
            {
                let source_ml_source1_temporary_construct_java_script_lexical_cell_failure_ignored226 = { let ml_method_owner669 = units.clone(); let ml_method_argument669_0 = {
                    let source_ml_source2_string_unit_operand22372 = message.clone();
                    {
                        let source_ml_source2_string_unit_operand22373 = at;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index < (0f64)) || (index >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand22379 = {
                                        let source_ml_source2_string_range_operand22374 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand22375 = index;
                                            {
                                                let source_ml_source2_string_range_operand22376 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position22377 = source_ml_source2_string_range_operand22375;
                                                    {
                                                        let source_ml_source2_string_range_position22378 = source_ml_source2_string_range_operand22376;
                                                        source_ml_source2_string_range_operand22374.slice(source_ml_source2_string_range_position22377, source_ml_source2_string_range_position22378)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand22380 = 0f64;
                                        source_ml_source2_string_unit_operand22379.code_point_at(source_ml_source2_string_unit_operand22380)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand22372.clone(), source_ml_source2_string_unit_operand22373)
                    }
                }; let ml_inserted = vec![ml_method_argument669_0]; let mut ml_method_values669 = ml_method_owner669.borrow_mut(); let ml_length = ml_method_values669.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values669.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Result::MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Result {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Result {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop231(mut declarations: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values228: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index228: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Result {
    loop {
        return if (source_ml_source1_temporary_index228 < ({ let ml_reference_array670 = source_ml_source1_temporary_values228.clone(); let ml_reference_projection670 = self::ml::Big::from_u128(ml_reference_array670.borrow().len() as u128); ml_reference_projection670 })) {
            {
                let identity = { let ml_reference_array671 = source_ml_source1_temporary_values228.clone(); let ml_reference_position671 = source_ml_source1_temporary_index228.clone(); let ml_reference_projection671 = self::ml_array::at(&*ml_reference_array671.borrow(), ml_reference_position671.to_index()); ml_reference_projection671 };
                {
                    let source_ml_source1_temporary_collect_java_script_source_lexical_declarations_ignored232 = { let ml_method_owner672 = declarations.clone(); let ml_method_argument672_0 = identity; let ml_inserted = vec![ml_method_argument672_0]; let mut ml_method_values672 = ml_method_owner672.borrow_mut(); let ml_length = ml_method_values672.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values672.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_source1_temporary_index228_2 = source_ml_source1_temporary_index228.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index228 = source_ml_source1_temporary_index228_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Result::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233 {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235 {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Result {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop236(mut context: self::JavaScriptErrorSourceContext, mut declarations: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values229: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_index229: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Result {
    loop {
        return if (source_ml_source1_temporary_index229 < ({ let ml_reference_array673 = source_ml_source1_temporary_values229.clone(); let ml_reference_projection673 = self::ml::Big::from_u128(ml_reference_array673.borrow().len() as u128); ml_reference_projection673 })) {
            {
                let row = { let ml_reference_array674 = source_ml_source1_temporary_values229.clone(); let ml_reference_position674 = source_ml_source1_temporary_index229.clone(); let ml_reference_projection674 = self::ml_array::at(&*ml_reference_array674.borrow(), ml_reference_position674.to_index()); ml_reference_projection674 };
                if { let ml_values675 = { let ml_reference_value676 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22385) => {
                        ((source_ml_source2_data_property_value22385.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value676 }; let ml_callback675 = { let row = row.clone(); std::rc::Rc::new(move |binding: self::JavaScriptSourceLexicalBinding| -> bool { ((match binding.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value22388, _, _, _, _) => {
                        source_ml_source2_data_property_value22388
                    }
                }) == (match row.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22389, _) => {
                        source_ml_source2_data_property_value22389
                    }
                })) }) }; ml_values675.iter().cloned().enumerate().any(|(ml_index675, ml_value675)| ml_callback675(ml_value675)) } {
                    {
                        let source_ml_source1_temporary_collect_java_script_source_lexical_declarations_ignored237 = { let ml_method_owner677 = declarations.clone(); let ml_method_argument677_0 = match row.clone() {
                            self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22390, _) => {
                                source_ml_source2_data_property_value22390
                            }
                        }; let ml_inserted = vec![ml_method_argument677_0]; let mut ml_method_values677 = ml_method_owner677.borrow_mut(); let ml_length = ml_method_values677.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values677.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_temporary_index229_2 = source_ml_source1_temporary_index229.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index229 = source_ml_source1_temporary_index229_2.clone();
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index229_3 = source_ml_source1_temporary_index229.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index229 = source_ml_source1_temporary_index229_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Result::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop230(mut context: self::JavaScriptErrorSourceContext, mut depth: f64, mut declarations: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values227: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, mut source_ml_source1_temporary_index227: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Result {
    loop {
        return if (source_ml_source1_temporary_index227 < ({ let ml_reference_array678 = source_ml_source1_temporary_values227.clone(); let ml_reference_projection678 = self::ml::Big::from_u128(ml_reference_array678.borrow().len() as u128); ml_reference_projection678 })) {
            {
                let statement = { let ml_reference_array679 = source_ml_source1_temporary_values227.clone(); let ml_reference_position679 = source_ml_source1_temporary_index227.clone(); let ml_reference_projection679 = self::ml_array::at(&*ml_reference_array679.borrow(), ml_reference_position679.to_index()); ml_reference_projection679 };
                match statement.clone() {
                    self::JavaScriptSourceStatement::SourceForOfStatement(_, _, _, _, _, _, _, _, _) => {
                        {
                            let source_ml_source1_temporary_index227_2 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_2.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceFinallyStatement(_, _) => {
                        {
                            let source_ml_source1_temporary_index227_3 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_3.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceTryStatement(_, _, _, _) => {
                        {
                            let source_ml_source1_temporary_index227_4 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_4.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceArrayBindingStatement(pattern, _, mutable, variable) => {
                        let pattern = *pattern;
                        {
                            let source_ml_source2_s22441 = if {
                                let source_ml_source2_comparison_operand22442 = ((variable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                !(match (source_ml_source2_comparison_operand22442.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                            } {
                                {
                                    let outputs = self::collect_java_script_pattern_binding_declarations(pattern.clone(), mutable, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                    {
                                        let source_ml_source1_temporary_values229 = match outputs.clone() {
                                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22444, _) => {
                                                source_ml_source2_data_property_value22444.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index229 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s22445 = self::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop236(context.clone(), declarations.clone(), source_ml_source1_temporary_values229.clone(), source_ml_source1_temporary_index229.clone());
                                                match source_ml_source2_s22445.clone() {
                                                    self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Result::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Done => {
                                                        self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235Next
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235Next
                            };
                            match source_ml_source2_s22441.clone() {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235Next => {
                                    {
                                        let source_ml_source1_temporary_index227_5 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_5.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceObjectBindingStatement(pattern_2, _, mutable_2, variable_2) => {
                        let pattern_2 = *pattern_2;
                        {
                            let source_ml_source2_s22450 = if {
                                let source_ml_source2_comparison_operand22451 = ((variable_2.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                !(match (source_ml_source2_comparison_operand22451.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                            } {
                                {
                                    let outputs_2 = self::collect_java_script_pattern_binding_declarations(pattern_2.clone(), mutable_2, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                    {
                                        let source_ml_source1_temporary_values229_2 = match outputs_2.clone() {
                                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22453, _) => {
                                                source_ml_source2_data_property_value22453.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index229_2 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s22454 = self::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop236(context.clone(), declarations.clone(), source_ml_source1_temporary_values229_2.clone(), source_ml_source1_temporary_index229_2.clone());
                                                match source_ml_source2_s22454.clone() {
                                                    self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Result::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Done => {
                                                        self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235Next
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235Next
                            };
                            match source_ml_source2_s22450.clone() {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235Next => {
                                    {
                                        let source_ml_source1_temporary_index227_6 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_6.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceVariableStatement(_, _, _, _) => {
                        {
                            let source_ml_source1_temporary_index227_7 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_7.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceConstStatement(identity, _) => {
                        {
                            let source_ml_source2_s22459 = if { let ml_values680 = { let ml_reference_value681 = ((match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22460) => {
                                    ((source_ml_source2_data_property_value22460.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                }
                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value681 }; let ml_callback680 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { (((match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value22462, _, _, _, _) => {
                                    source_ml_source2_data_property_value22462
                                }
                            }) == identity) && !(match match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value22463, _) => {
                                    ((source_ml_source2_data_property_value22463.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                }
                            } { Some(ml_present) => ml_present, None => false })) }) }; ml_values680.iter().cloned().enumerate().any(|(ml_index680, ml_value680)| ml_callback680(ml_value680)) } {
                                {
                                    let source_ml_source1_temporary_collect_java_script_source_lexical_declarations_ignored234 = { let ml_method_owner682 = declarations.clone(); let ml_method_argument682_0 = identity; let ml_inserted = vec![ml_method_argument682_0]; let mut ml_method_values682 = ml_method_owner682.borrow_mut(); let ml_length = ml_method_values682.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values682.extend(ml_inserted); ml_length as f64 };
                                    self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next
                                }
                            } else {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next
                            };
                            match source_ml_source2_s22459.clone() {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next => {
                                    {
                                        let source_ml_source1_temporary_index227_8 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_8.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceMutableStatement(identity_2, _, _) => {
                        {
                            let source_ml_source2_s22468 = if { let ml_values683 = { let ml_reference_value684 = ((match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22469) => {
                                    ((source_ml_source2_data_property_value22469.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                }
                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value684 }; let ml_callback683 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { (((match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value22471, _, _, _, _) => {
                                    source_ml_source2_data_property_value22471
                                }
                            }) == identity_2) && !(match match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value22472, _) => {
                                    ((source_ml_source2_data_property_value22472.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                }
                            } { Some(ml_present) => ml_present, None => false })) }) }; ml_values683.iter().cloned().enumerate().any(|(ml_index683, ml_value683)| ml_callback683(ml_value683)) } {
                                {
                                    let source_ml_source1_temporary_collect_java_script_source_lexical_declarations_ignored234_2 = { let ml_method_owner685 = declarations.clone(); let ml_method_argument685_0 = identity_2; let ml_inserted = vec![ml_method_argument685_0]; let mut ml_method_values685 = ml_method_owner685.borrow_mut(); let ml_length = ml_method_values685.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values685.extend(ml_inserted); ml_length as f64 };
                                    self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next
                                }
                            } else {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next
                            };
                            match source_ml_source2_s22468.clone() {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next => {
                                    {
                                        let source_ml_source1_temporary_index227_9 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_9.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceExpressionStatement(_) => {
                        {
                            let source_ml_source1_temporary_index227_10 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_10.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
                        {
                            let source_ml_source1_temporary_index227_11 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_11.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
                        {
                            let source_ml_source1_temporary_index227_12 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_12.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceIfStatement(_, _, _) => {
                        {
                            let source_ml_source1_temporary_index227_13 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_13.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceBlockStatement(_) => {
                        {
                            let source_ml_source1_temporary_index227_14 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_14.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(statements) => {
                        {
                            let source_ml_source1_temporary_values228 = self::collect_java_script_source_lexical_declarations(statements.clone(), context.clone(), (depth + 1f64));
                            {
                                let source_ml_source1_temporary_index228 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s22473 = self::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop231(declarations.clone(), source_ml_source1_temporary_values228.clone(), source_ml_source1_temporary_index228.clone());
                                    match source_ml_source2_s22473.clone() {
                                        self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Result::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Done => {
                                            {
                                                let source_ml_source1_temporary_index227_15 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_15.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceWhileStatement(_, _, _, _, _, _) => {
                        {
                            let source_ml_source1_temporary_index227_16 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_16.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceControlBlockStatement(_, _) => {
                        {
                            let source_ml_source1_temporary_index227_17 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_17.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
                        {
                            let source_ml_source1_temporary_index227_18 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_18.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptSourceStatement::SourceSwitchStatement(_, _, _) => {
                        {
                            let source_ml_source1_temporary_index227_19 = source_ml_source1_temporary_index227.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index227 = source_ml_source1_temporary_index227_19.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Result::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Done
        };
    }
}
