// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptErrorModuleCompilation;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptPatternBindingDeclarations;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceLexicalBinding;
use super::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result;
use super::MlSource1TemporaryLowerJavaScriptSourceParametersLoop159Result;
use super::collect_java_script_pattern_binding_declarations;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_field_key;
use super::lower_java_script_error_expression;
use super::lower_java_script_source_binding_pattern;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_lower_java_script_source_parameters_loop159;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_parameters_loop158(mut context: self::JavaScriptErrorSourceContext, mut owned: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values155: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, mut source_ml_source1_temporary_index155: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result {
    loop {
        return if (source_ml_source1_temporary_index155 < ({ let ml_reference_array545 = source_ml_source1_temporary_values155.clone(); let ml_reference_projection545 = self::ml::Big::from_u128(ml_reference_array545.borrow().len() as u128); ml_reference_projection545 })) {
            {
                let parameter = { let ml_reference_array546 = source_ml_source1_temporary_values155.clone(); let ml_reference_position546 = source_ml_source1_temporary_index155.clone(); let ml_reference_projection546 = self::ml_array::at(&*ml_reference_array546.borrow(), ml_reference_position546.to_index()); ml_reference_projection546 };
                {
                    let outputs = self::JavaScriptPatternBindingDeclarations::BindingDeclarations(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(match parameter.clone() {
                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20197, _, _) => {
                            source_ml_source2_data_property_value20197
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20198, _) => {
                            source_ml_source2_data_property_value20198
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20199, _, _) => {
                            source_ml_source2_data_property_value20199
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value20200, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20200
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value20201, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20201
                        }
                    }, true)])), self::ml_string::Text::from_units(vec![]));
                    {
                        let outputs_6 = match parameter.clone() {
                            self::JavaScriptErrorParameter::ErrorRestParameter(_, parameter_pattern, _) => {
                                {
                                    let pattern = ((parameter_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                                    {
                                        let outputs_3 = if !(match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                            {
                                                let outputs_2 = self::collect_java_script_pattern_binding_declarations(pattern.clone().expect("a proven present value"), true, 0f64, std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                outputs_2.clone()
                                            }
                                        } else {
                                            outputs.clone()
                                        };
                                        outputs_3.clone()
                                    }
                                }
                            }
                            self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                                outputs.clone()
                            }
                            self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                                outputs.clone()
                            }
                            self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                                {
                                    let outputs_4 = self::collect_java_script_pattern_binding_declarations(parameter.clone(), true, 0f64, std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                    outputs_4.clone()
                                }
                            }
                            self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                                {
                                    let outputs_5 = self::collect_java_script_pattern_binding_declarations(parameter.clone(), true, 0f64, std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                    outputs_5.clone()
                                }
                            }
                        };
                        if ((match outputs_6.clone() {
                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value20222) => {
                                source_ml_source2_data_property_value20222.clone()
                            }
                        }) != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Return(Box::new(self::refuse_java_script_error_source(context.clone(), &(match outputs_6.clone() {
                                self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value20223) => {
                                    source_ml_source2_data_property_value20223.clone()
                                }
                            }))))
                        } else {
                            {
                                let source_ml_source1_temporary_values156 = match outputs_6.clone() {
                                    self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value20224, _) => {
                                        source_ml_source2_data_property_value20224.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index156 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s20225 = self::source_ml_source1_temporary_lower_java_script_source_parameters_loop159(context.clone(), owned.clone(), source_ml_source1_temporary_values156.clone(), source_ml_source1_temporary_index156.clone());
                                        match source_ml_source2_s20225.clone() {
                                            self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop159Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop159Done => {
                                                {
                                                    let source_ml_source1_temporary_index155_2 = source_ml_source1_temporary_index155.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index155 = source_ml_source1_temporary_index155_2.clone();
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
        } else {
            self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result {
    MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Done(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
    MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Return(Box<self::JavaScriptErrorSourceExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceParametersJoin161 {
    MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Next(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
    MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Return(Box<self::JavaScriptErrorSourceExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceParametersJoin163 {
    MlSource1TemporaryLowerJavaScriptSourceParametersJoin163Next(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceParametersJoin164 {
    MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceParametersJoin162 {
    MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Next(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
    MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Return(Box<self::JavaScriptErrorSourceExpression>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_parameters_loop157(mut parameters: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameter>>>, mut context: self::JavaScriptErrorSourceContext, mut source: self::JavaScriptCallSource, mut at: f64) -> self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result {
    loop {
        return if (at >= (0f64)) {
            {
                let parameter = { let ml_reference_array547 = parameters.clone(); let ml_reference_position547 = at; let ml_reference_projection547 = self::ml_array::optional_at(&*ml_reference_array547.borrow(), self::ml_array::number_index(ml_reference_position547)); ml_reference_projection547 };
                if match (parameter.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    {
                        let at_2 = (at - 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                } else {
                    if (((match parameter.clone().expect("a proven present value") {
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                            true
                        }
                        _ => {
                            false
                        }
                    } || match parameter.clone().expect("a proven present value") {
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                            true
                        }
                        _ => {
                            false
                        }
                    }) || match parameter.clone().expect("a proven present value") {
                        self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                            true
                        }
                        _ => {
                            false
                        }
                    }) && { let ml_values548 = { let ml_reference_value549 = ((match context.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20288) => {
                            ((source_ml_source2_data_property_value20288.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value549 }; let ml_callback548 = { let parameter = parameter.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                        self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value20295, _, _, _, _) => {
                            source_ml_source2_data_property_value20295
                        }
                    }) == (match parameter.clone().expect("a proven present value") {
                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20296, _, _) => {
                            source_ml_source2_data_property_value20296
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20297, _) => {
                            source_ml_source2_data_property_value20297
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20298, _, _) => {
                            source_ml_source2_data_property_value20298
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value20299, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20299
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value20300, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20300
                        }
                    })) }) }; ml_values548.iter().cloned().enumerate().any(|(ml_index548, ml_value548)| ml_callback548(ml_value548)) }) {
                        {
                            let value = self::JavaScriptCallSource::SourceArgument(at);
                            {
                                let source_ml_source2_s20402 = match parameter.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameter::ErrorRestParameter(_, source_ml_source2_f20404, _) => {
                                        {
                                            let prototype = match match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20441, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value20441 = *source_ml_source2_data_property_value20441;
                                                    source_ml_source2_data_property_value20441.clone()
                                                }
                                            } {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, source_ml_source2_data_property_value20442, _, _, _, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value20442.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            };
                                            if ((at != (({ let ml_reference_array550 = parameters.clone(); let ml_reference_projection550 = ml_reference_array550.borrow().len() as f64; ml_reference_projection550 } - 1f64))) || !(match (((source_ml_source2_f20404.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Return(Box::new(self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 115, 116, 32, 99, 101, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 101, 32, 102, 105, 110, 97, 108, 32, 115, 105, 109, 112, 108, 101, 32, 102, 111, 114, 109, 97, 108, 32, 97, 110, 100, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))))
                                            } else {
                                                if match (prototype.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Return(Box::new(self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 115, 116, 32, 99, 101, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 101, 32, 102, 105, 110, 97, 108, 32, 115, 105, 109, 112, 108, 101, 32, 102, 111, 114, 109, 97, 108, 32, 97, 110, 100, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))))
                                                } else {
                                                    {
                                                        let value_2 = self::JavaScriptCallSource::SourceArgumentRest(at, Box::new(self::construct_java_script_error_constant(prototype.clone().expect("a proven present value"))), match match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20443, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value20443 = *source_ml_source2_data_property_value20443;
                                                                source_ml_source2_data_property_value20443.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20444, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value20444.clone()
                                                            }
                                                        });
                                                        self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Next(Box::new(context.clone()), Box::new(value_2.clone()))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                                        self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Next(Box::new(context.clone()), Box::new(value.clone()))
                                    }
                                    self::JavaScriptErrorParameter::ErrorDefaultParameter(_, parameter_value, _) => {
                                        let parameter_value = *parameter_value;
                                        {
                                            let lowered = self::lower_java_script_error_expression(parameter_value.clone(), context.clone(), 0f64);
                                            {
                                                let context_2 = match lowered.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20464) => {
                                                        let source_ml_source2_data_property_value20464 = *source_ml_source2_data_property_value20464;
                                                        source_ml_source2_data_property_value20464.clone()
                                                    }
                                                };
                                                {
                                                    let value_3 = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(self::JavaScriptCallSource::SourceArgument(at)), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)))), Box::new(match lowered.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20465, _) => {
                                                            let source_ml_source2_data_property_value20465 = *source_ml_source2_data_property_value20465;
                                                            source_ml_source2_data_property_value20465.clone()
                                                        }
                                                    }), Box::new(self::JavaScriptCallSource::SourceArgument(at)));
                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Next(Box::new(context_2.clone()), Box::new(value_3.clone()))
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                                        self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Next(Box::new(context.clone()), Box::new(value.clone()))
                                    }
                                    self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                                        self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Next(Box::new(context.clone()), Box::new(value.clone()))
                                    }
                                };
                                match source_ml_source2_s20402.clone() {
                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Next(context_3, value_4) => {
                                        let context_3 = *context_3;
                                        let value_4 = *value_4;
                                        {
                                            let source_2 = self::JavaScriptCallSource::SourceInitializeCell(match parameter.clone().expect("a proven present value") {
                                                self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20504, _, _) => {
                                                    source_ml_source2_data_property_value20504
                                                }
                                                self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20505, _) => {
                                                    source_ml_source2_data_property_value20505
                                                }
                                                self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20506, _, _) => {
                                                    source_ml_source2_data_property_value20506
                                                }
                                                self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value20507, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20507
                                                }
                                                self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value20508, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20508
                                                }
                                            }, Box::new(value_4.clone()), Box::new(source.clone()));
                                            {
                                                let at_3 = (at - 1f64);
                                                {
                                                    (context, source, at) = (context_3.clone(), source_2.clone(), at_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Return(source_ml_source1_temporary_result161) => {
                                        let source_ml_source1_temporary_result161 = *source_ml_source1_temporary_result161;
                                        self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Return(Box::new(source_ml_source1_temporary_result161.clone()))
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source2_s20634 = if match {
                                let source_ml_source2_logical20698 = ((match parameter.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                } || match parameter.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                }) || match parameter.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                });
                                if source_ml_source2_logical20698 {
                                    match parameter.clone().expect("a proven present value") {
                                        self::JavaScriptErrorParameter::ErrorRestParameter(_, _, source_ml_source2_data_property_value20695) => {
                                            ((source_ml_source2_data_property_value20695.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                        }
                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(_, source_ml_source2_data_property_value20696) => {
                                            ((source_ml_source2_data_property_value20696.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                        }
                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, source_ml_source2_data_property_value20697) => {
                                            ((source_ml_source2_data_property_value20697.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                        }
                                        self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                                            None::<bool>
                                        }
                                        self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                                            None::<bool>
                                        }
                                    }
                                } else {
                                    Some(source_ml_source2_logical20698)
                                }
                            } { Some(ml_present) => ml_present, None => false } {
                                if (match parameter.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                } && !(match (match parameter.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameter::ErrorRestParameter(_, source_ml_source2_data_property_value20719, _) => {
                                        ((source_ml_source2_data_property_value20719.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone()
                                    }
                                    self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                                        None::<self::JavaScriptErrorParameter>
                                    }
                                    self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                                        None::<self::JavaScriptErrorParameter>
                                    }
                                    self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                                        None::<self::JavaScriptErrorParameter>
                                    }
                                    self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                                        None::<self::JavaScriptErrorParameter>
                                    }
                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Return(Box::new(self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 104, 97, 114, 101, 100, 32, 100, 101, 115, 116, 114, 117, 99, 116, 117, 114, 105, 110, 103, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 101, 108, 108, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 111, 119, 110, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110])))))
                                } else {
                                    {
                                        let temporary = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20720, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value20720
                                            }
                                        };
                                        {
                                            let context_4 = {
                                                let source_ml_source2_nominal_field20722 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20723, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value20723 = *source_ml_source2_data_property_value20723;
                                                        source_ml_source2_data_property_value20723.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field20724 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20725, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value20725 = *source_ml_source2_data_property_value20725;
                                                            source_ml_source2_data_property_value20725.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field20726 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20727, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value20727.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field20728 = match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20729, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value20729
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field20730 = match context.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20731, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value20731
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field20732 = match context.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20733, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value20733.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field20734 = match context.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20735, _, _, _) => {
                                                                                source_ml_source2_data_property_value20735
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field20736 = match context.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20737, _, _) => {
                                                                                    source_ml_source2_data_property_value20737.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field20738 = match context.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20739, _) => {
                                                                                        source_ml_source2_data_property_value20739.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field20740 = match context.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20741) => {
                                                                                            source_ml_source2_data_property_value20741.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override20742 = (temporary + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20722.clone()), Box::new(source_ml_source2_nominal_field20724.clone()), source_ml_source2_nominal_field20726.clone(), source_ml_source2_nominal_override20742, source_ml_source2_nominal_field20730, source_ml_source2_nominal_field20732.clone(), source_ml_source2_nominal_field20734, source_ml_source2_nominal_field20736.clone(), source_ml_source2_nominal_field20738.clone(), source_ml_source2_nominal_field20740.clone())
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
                                                let cell = {
                                                    let source_ml_source2_constructor_field20743 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                                                    {
                                                        let source_ml_source2_constructor_field20746 = match match context_4.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20744, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value20744 = *source_ml_source2_data_property_value20744;
                                                                source_ml_source2_data_property_value20744.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20745, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value20745.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field20754 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                let source_ml_source2_constructor_field20747 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                                                {
                                                                    let source_ml_source2_constructor_field20753 = self::JavaScriptCallSource::SourceBinding(match parameter.clone().expect("a proven present value") {
                                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20748, _, _) => {
                                                                            source_ml_source2_data_property_value20748
                                                                        }
                                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20749, _) => {
                                                                            source_ml_source2_data_property_value20749
                                                                        }
                                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20750, _, _) => {
                                                                            source_ml_source2_data_property_value20750
                                                                        }
                                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value20751, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value20751
                                                                        }
                                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value20752, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value20752
                                                                        }
                                                                    });
                                                                    self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field20747.clone()), Box::new(source_ml_source2_constructor_field20753.clone()), None::<(Option<bool>,)>)
                                                                }
                                                            }]));
                                                            self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field20743.clone()), source_ml_source2_constructor_field20754.clone(), source_ml_source2_constructor_field20746.clone())
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_3 = self::construct_java_script_error_local_binding(context_4.clone(), temporary, self::JavaScriptCallSource::SourceAssignBinding(match parameter.clone().expect("a proven present value") {
                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20755, _, _) => {
                                                            source_ml_source2_data_property_value20755
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20756, _) => {
                                                            source_ml_source2_data_property_value20756
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20757, _, _) => {
                                                            source_ml_source2_data_property_value20757
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value20758, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value20758
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value20759, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value20759
                                                        }
                                                    }, Box::new(cell.clone())), source.clone(), false);
                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Next(Box::new(context_4.clone()), Box::new(source_3.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Next(Box::new(context.clone()), Box::new(source.clone()))
                            };
                            match source_ml_source2_s20634.clone() {
                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Next(context_5, source_4) => {
                                    let context_5 = *context_5;
                                    let source_4 = *source_4;
                                    match parameter.clone().expect("a proven present value") {
                                        self::JavaScriptErrorParameter::ErrorRestParameter(identity, parameter_pattern, _) => {
                                            {
                                                let prototype_2 = match match context_5.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20770, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value20770 = *source_ml_source2_data_property_value20770;
                                                        source_ml_source2_data_property_value20770.clone()
                                                    }
                                                } {
                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, source_ml_source2_data_property_value20771, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value20771.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                };
                                                if (at != (({ let ml_reference_array551 = parameters.clone(); let ml_reference_projection551 = ml_reference_array551.borrow().len() as f64; ml_reference_projection551 } - 1f64))) {
                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Return(Box::new(self::refuse_java_script_error_source(context_5.clone(), &(self::ml_string::Text::from_units(vec![114, 101, 115, 116, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 102, 105, 110, 97, 108, 32, 102, 111, 114, 109, 97, 108, 32, 97, 110, 100, 32, 105, 116, 115, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))))
                                                } else {
                                                    if match (prototype_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Return(Box::new(self::refuse_java_script_error_source(context_5.clone(), &(self::ml_string::Text::from_units(vec![114, 101, 115, 116, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 101, 32, 97, 99, 116, 117, 97, 108, 32, 102, 105, 110, 97, 108, 32, 102, 111, 114, 109, 97, 108, 32, 97, 110, 100, 32, 105, 116, 115, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))))
                                                    } else {
                                                        {
                                                            let pattern = ((parameter_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                                                            {
                                                                let source_ml_source2_s20774 = if !(match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    {
                                                                        let initialized = self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), context_5.clone(), source_4.clone(), true, 0f64, false);
                                                                        {
                                                                            let context_6 = match initialized.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20775) => {
                                                                                    let source_ml_source2_data_property_value20775 = *source_ml_source2_data_property_value20775;
                                                                                    source_ml_source2_data_property_value20775.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_5 = match initialized.clone() {
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20776, _) => {
                                                                                        let source_ml_source2_data_property_value20776 = *source_ml_source2_data_property_value20776;
                                                                                        source_ml_source2_data_property_value20776.clone()
                                                                                    }
                                                                                };
                                                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163Next(Box::new(context_6.clone()), Box::new(source_5.clone()))
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163Next(Box::new(context_5.clone()), Box::new(source_4.clone()))
                                                                };
                                                                match source_ml_source2_s20774.clone() {
                                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163Next(context_7, source_6) => {
                                                                        let context_7 = *context_7;
                                                                        let source_6 = *source_6;
                                                                        {
                                                                            let source_7 = self::JavaScriptCallSource::SourceParameterRest(identity, at, Box::new(self::construct_java_script_error_constant(prototype_2.clone().expect("a proven present value"))), match match context_7.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20777, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value20777 = *source_ml_source2_data_property_value20777;
                                                                                    source_ml_source2_data_property_value20777.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20778, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value20778.clone()
                                                                                }
                                                                            }, Box::new(source_6.clone()));
                                                                            {
                                                                                let at_4 = (at - 1f64);
                                                                                {
                                                                                    (context, source, at) = (context_7.clone(), source_7.clone(), at_4);
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
                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                                            {
                                                let at_5 = (at - 1f64);
                                                {
                                                    (context, source, at) = (context_5.clone(), source_4.clone(), at_5);
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(identity_2, parameter_value_2, _) => {
                                            let parameter_value_2 = *parameter_value_2;
                                            {
                                                let value_5 = self::lower_java_script_error_expression(parameter_value_2.clone(), context_5.clone(), 0f64);
                                                {
                                                    let context_8 = match value_5.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20779) => {
                                                            let source_ml_source2_data_property_value20779 = *source_ml_source2_data_property_value20779;
                                                            source_ml_source2_data_property_value20779.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_8 = self::JavaScriptCallSource::SourceParameterDefault(identity_2, Box::new(match value_5.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20780, _) => {
                                                                let source_ml_source2_data_property_value20780 = *source_ml_source2_data_property_value20780;
                                                                source_ml_source2_data_property_value20780.clone()
                                                            }
                                                        }), Box::new(source_4.clone()));
                                                        {
                                                            let at_6 = (at - 1f64);
                                                            {
                                                                (context, source, at) = (context_8.clone(), source_8.clone(), at_6);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptErrorParameter::ErrorObjectParameter(identity_3, has_default, parameter_value_3, _, _, _) => {
                                            let parameter_value_3 = *parameter_value_3;
                                            {
                                                let initialized_2 = self::lower_java_script_source_binding_pattern(parameter.clone().expect("a proven present value"), context_5.clone(), source_4.clone(), true, 0f64, false);
                                                {
                                                    let context_9 = match initialized_2.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20781) => {
                                                            let source_ml_source2_data_property_value20781 = *source_ml_source2_data_property_value20781;
                                                            source_ml_source2_data_property_value20781.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_9 = match initialized_2.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20782, _) => {
                                                                let source_ml_source2_data_property_value20782 = *source_ml_source2_data_property_value20782;
                                                                source_ml_source2_data_property_value20782.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_s20785 = if has_default {
                                                                {
                                                                    let value_6 = self::lower_java_script_error_expression(parameter_value_3.clone(), context_9.clone(), 0f64);
                                                                    {
                                                                        let context_10 = match value_6.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20786) => {
                                                                                let source_ml_source2_data_property_value20786 = *source_ml_source2_data_property_value20786;
                                                                                source_ml_source2_data_property_value20786.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_10 = self::JavaScriptCallSource::SourceParameterDefault(identity_3, Box::new(match value_6.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20787, _) => {
                                                                                    let source_ml_source2_data_property_value20787 = *source_ml_source2_data_property_value20787;
                                                                                    source_ml_source2_data_property_value20787.clone()
                                                                                }
                                                                            }), Box::new(source_9.clone()));
                                                                            self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(Box::new(context_10.clone()), Box::new(source_10.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(Box::new(context_9.clone()), Box::new(source_9.clone()))
                                                            };
                                                            match source_ml_source2_s20785.clone() {
                                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(context_11, source_11) => {
                                                                    let context_11 = *context_11;
                                                                    let source_11 = *source_11;
                                                                    {
                                                                        let at_7 = (at - 1f64);
                                                                        {
                                                                            (context, source, at) = (context_11.clone(), source_11.clone(), at_7);
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
                                        self::JavaScriptErrorParameter::ErrorArrayParameter(identity_4, has_default_2, parameter_value_4, _, _, _) => {
                                            let parameter_value_4 = *parameter_value_4;
                                            {
                                                let initialized_3 = self::lower_java_script_source_binding_pattern(parameter.clone().expect("a proven present value"), context_5.clone(), source_4.clone(), true, 0f64, false);
                                                {
                                                    let context_12 = match initialized_3.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20788) => {
                                                            let source_ml_source2_data_property_value20788 = *source_ml_source2_data_property_value20788;
                                                            source_ml_source2_data_property_value20788.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_12 = match initialized_3.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20789, _) => {
                                                                let source_ml_source2_data_property_value20789 = *source_ml_source2_data_property_value20789;
                                                                source_ml_source2_data_property_value20789.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_s20792 = if has_default_2 {
                                                                {
                                                                    let value_7 = self::lower_java_script_error_expression(parameter_value_4.clone(), context_12.clone(), 0f64);
                                                                    {
                                                                        let context_13 = match value_7.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20793) => {
                                                                                let source_ml_source2_data_property_value20793 = *source_ml_source2_data_property_value20793;
                                                                                source_ml_source2_data_property_value20793.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_13 = self::JavaScriptCallSource::SourceParameterDefault(identity_4, Box::new(match value_7.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20794, _) => {
                                                                                    let source_ml_source2_data_property_value20794 = *source_ml_source2_data_property_value20794;
                                                                                    source_ml_source2_data_property_value20794.clone()
                                                                                }
                                                                            }), Box::new(source_12.clone()));
                                                                            self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(Box::new(context_13.clone()), Box::new(source_13.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(Box::new(context_12.clone()), Box::new(source_12.clone()))
                                                            };
                                                            match source_ml_source2_s20792.clone() {
                                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(context_14, source_14) => {
                                                                    let context_14 = *context_14;
                                                                    let source_14 = *source_14;
                                                                    {
                                                                        let at_8 = (at - 1f64);
                                                                        {
                                                                            (context, source, at) = (context_14.clone(), source_14.clone(), at_8);
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
                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Return(source_ml_source1_temporary_result164) => {
                                    let source_ml_source1_temporary_result164 = *source_ml_source1_temporary_result164;
                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Return(Box::new(source_ml_source1_temporary_result164.clone()))
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop157Done(Box::new(context.clone()), Box::new(source.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result {
    MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Done,
    MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Return(Box<self::JavaScriptErrorModuleCompilation>),
}
