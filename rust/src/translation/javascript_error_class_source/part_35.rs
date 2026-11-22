// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptErrorModuleCompilation;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptPatternBindingDeclarations;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
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
                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20076, _, _) => {
                            source_ml_source2_data_property_value20076
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20077, _) => {
                            source_ml_source2_data_property_value20077
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20078, _, _) => {
                            source_ml_source2_data_property_value20078
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value20079, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20079
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value20080, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20080
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
                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value20101) => {
                                source_ml_source2_data_property_value20101.clone()
                            }
                        }) != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Return(Box::new(self::refuse_java_script_error_source(context.clone(), &(match outputs_6.clone() {
                                self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value20102) => {
                                    source_ml_source2_data_property_value20102.clone()
                                }
                            }))))
                        } else {
                            {
                                let source_ml_source1_temporary_values156 = match outputs_6.clone() {
                                    self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value20103, _) => {
                                        source_ml_source2_data_property_value20103.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index156 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s20104 = self::source_ml_source1_temporary_lower_java_script_source_parameters_loop159(context.clone(), owned.clone(), source_ml_source1_temporary_values156.clone(), source_ml_source1_temporary_index156.clone());
                                        match source_ml_source2_s20104.clone() {
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
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20167) => {
                            ((source_ml_source2_data_property_value20167.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value549 }; let ml_callback548 = { let parameter = parameter.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                        self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value20172, _, _, _, _) => {
                            source_ml_source2_data_property_value20172
                        }
                    }) == (((match parameter.clone().expect("a proven present value") {
                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20173, _, _) => {
                            Some((source_ml_source2_data_property_value20173,))
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20174, _) => {
                            Some((source_ml_source2_data_property_value20174,))
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20175, _, _) => {
                            Some((source_ml_source2_data_property_value20175,))
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                            None::<(f64,)>
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                            None::<(f64,)>
                        }
                    }).clone().expect("a proven present value")).0)) }) }; ml_values548.iter().cloned().enumerate().any(|(ml_index548, ml_value548)| ml_callback548(ml_value548)) }) {
                        {
                            let value = self::JavaScriptCallSource::SourceArgument(at);
                            {
                                let source_ml_source2_s20277 = match parameter.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameter::ErrorRestParameter(_, source_ml_source2_f20279, _) => {
                                        {
                                            let prototype = match match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20316, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value20316 = *source_ml_source2_data_property_value20316;
                                                    source_ml_source2_data_property_value20316.clone()
                                                }
                                            } {
                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, source_ml_source2_data_property_value20317, _, _, _, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value20317.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            };
                                            if ((at != (({ let ml_reference_array550 = parameters.clone(); let ml_reference_projection550 = ml_reference_array550.borrow().len() as f64; ml_reference_projection550 } - 1f64))) || !(match (((source_ml_source2_f20279.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Return(Box::new(self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 115, 116, 32, 99, 101, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 101, 32, 102, 105, 110, 97, 108, 32, 115, 105, 109, 112, 108, 101, 32, 102, 111, 114, 109, 97, 108, 32, 97, 110, 100, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))))
                                            } else {
                                                if match (prototype.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Return(Box::new(self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 115, 116, 32, 99, 101, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 101, 32, 102, 105, 110, 97, 108, 32, 115, 105, 109, 112, 108, 101, 32, 102, 111, 114, 109, 97, 108, 32, 97, 110, 100, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))))
                                                } else {
                                                    {
                                                        let value_2 = self::JavaScriptCallSource::SourceArgumentRest(at, Box::new(self::construct_java_script_error_constant(prototype.clone().expect("a proven present value"))), match match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20318, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value20318 = *source_ml_source2_data_property_value20318;
                                                                source_ml_source2_data_property_value20318.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20319, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value20319.clone()
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
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20339) => {
                                                        let source_ml_source2_data_property_value20339 = *source_ml_source2_data_property_value20339;
                                                        source_ml_source2_data_property_value20339.clone()
                                                    }
                                                };
                                                {
                                                    let value_3 = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(self::JavaScriptCallSource::SourceArgument(at)), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)))), Box::new(match lowered.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20340, _) => {
                                                            let source_ml_source2_data_property_value20340 = *source_ml_source2_data_property_value20340;
                                                            source_ml_source2_data_property_value20340.clone()
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
                                match source_ml_source2_s20277.clone() {
                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161::MlSource1TemporaryLowerJavaScriptSourceParametersJoin161Next(context_3, value_4) => {
                                        let context_3 = *context_3;
                                        let value_4 = *value_4;
                                        {
                                            let source_2 = self::JavaScriptCallSource::SourceInitializeCell(((match parameter.clone().expect("a proven present value") {
                                                self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20379, _, _) => {
                                                    Some((source_ml_source2_data_property_value20379,))
                                                }
                                                self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20380, _) => {
                                                    Some((source_ml_source2_data_property_value20380,))
                                                }
                                                self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20381, _, _) => {
                                                    Some((source_ml_source2_data_property_value20381,))
                                                }
                                                self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                                                    None::<(f64,)>
                                                }
                                                self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                                                    None::<(f64,)>
                                                }
                                            }).clone().expect("a proven present value")).0, Box::new(value_4.clone()), Box::new(source.clone()));
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
                            let source_ml_source2_s20503 = if match {
                                let source_ml_source2_logical20567 = ((match parameter.clone().expect("a proven present value") {
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
                                if source_ml_source2_logical20567 {
                                    (((match parameter.clone().expect("a proven present value") {
                                        self::JavaScriptErrorParameter::ErrorRestParameter(_, _, source_ml_source2_data_property_value20564) => {
                                            Some((((source_ml_source2_data_property_value20564.clone().unwrap_or_else(|| (None::<bool>,))).0).clone(),))
                                        }
                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(_, source_ml_source2_data_property_value20565) => {
                                            Some((((source_ml_source2_data_property_value20565.clone().unwrap_or_else(|| (None::<bool>,))).0).clone(),))
                                        }
                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, source_ml_source2_data_property_value20566) => {
                                            Some((((source_ml_source2_data_property_value20566.clone().unwrap_or_else(|| (None::<bool>,))).0).clone(),))
                                        }
                                        self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                                            None::<(Option<bool>,)>
                                        }
                                        self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                                            None::<(Option<bool>,)>
                                        }
                                    }).clone().expect("a proven present value")).0).clone()
                                } else {
                                    Some(source_ml_source2_logical20567)
                                }
                            } { Some(ml_present) => ml_present, None => false } {
                                if (match parameter.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                } && !(match ((((match parameter.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameter::ErrorRestParameter(_, source_ml_source2_data_property_value20588, _) => {
                                        Some((((source_ml_source2_data_property_value20588.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone(),))
                                    }
                                    self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                                        None::<(Option<self::JavaScriptErrorParameter>,)>
                                    }
                                    self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                                        None::<(Option<self::JavaScriptErrorParameter>,)>
                                    }
                                    self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                                        None::<(Option<self::JavaScriptErrorParameter>,)>
                                    }
                                    self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                                        None::<(Option<self::JavaScriptErrorParameter>,)>
                                    }
                                }).clone().expect("a proven present value")).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Return(Box::new(self::refuse_java_script_error_source(context.clone(), &(self::ml_string::Text::from_units(vec![115, 104, 97, 114, 101, 100, 32, 100, 101, 115, 116, 114, 117, 99, 116, 117, 114, 105, 110, 103, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 101, 108, 108, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 111, 119, 110, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110])))))
                                } else {
                                    {
                                        let temporary = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20589, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value20589
                                            }
                                        };
                                        {
                                            let context_4 = {
                                                let source_ml_source2_nominal_field20591 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20592, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value20592 = *source_ml_source2_data_property_value20592;
                                                        source_ml_source2_data_property_value20592.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field20593 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20594, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value20594 = *source_ml_source2_data_property_value20594;
                                                            source_ml_source2_data_property_value20594.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field20595 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20596, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value20596.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field20597 = match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20598, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value20598
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field20599 = match context.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20600, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value20600
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field20601 = match context.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20602, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value20602.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field20603 = match context.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20604, _, _, _) => {
                                                                                source_ml_source2_data_property_value20604
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field20605 = match context.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20606, _, _) => {
                                                                                    source_ml_source2_data_property_value20606.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field20607 = match context.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20608, _) => {
                                                                                        source_ml_source2_data_property_value20608.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field20609 = match context.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20610) => {
                                                                                            source_ml_source2_data_property_value20610.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override20611 = (temporary + 1f64);
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20591.clone()), Box::new(source_ml_source2_nominal_field20593.clone()), source_ml_source2_nominal_field20595.clone(), source_ml_source2_nominal_override20611, source_ml_source2_nominal_field20599, source_ml_source2_nominal_field20601.clone(), source_ml_source2_nominal_field20603, source_ml_source2_nominal_field20605.clone(), source_ml_source2_nominal_field20607.clone(), source_ml_source2_nominal_field20609.clone())
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
                                                    let source_ml_source2_constructor_field20612 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
                                                    {
                                                        let source_ml_source2_constructor_field20615 = match match context_4.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20613, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value20613 = *source_ml_source2_data_property_value20613;
                                                                source_ml_source2_data_property_value20613.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20614, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value20614.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_constructor_field20621 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                let source_ml_source2_constructor_field20616 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                                                                {
                                                                    let source_ml_source2_constructor_field20620 = self::JavaScriptCallSource::SourceBinding(((match parameter.clone().expect("a proven present value") {
                                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20617, _, _) => {
                                                                            Some((source_ml_source2_data_property_value20617,))
                                                                        }
                                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20618, _) => {
                                                                            Some((source_ml_source2_data_property_value20618,))
                                                                        }
                                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20619, _, _) => {
                                                                            Some((source_ml_source2_data_property_value20619,))
                                                                        }
                                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                                                                            None::<(f64,)>
                                                                        }
                                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                                                                            None::<(f64,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0);
                                                                    self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field20616.clone()), Box::new(source_ml_source2_constructor_field20620.clone()), None::<(Option<bool>,)>)
                                                                }
                                                            }]));
                                                            self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field20612.clone()), source_ml_source2_constructor_field20621.clone(), source_ml_source2_constructor_field20615.clone())
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_3 = self::construct_java_script_error_local_binding(context_4.clone(), temporary, self::JavaScriptCallSource::SourceAssignBinding(((match parameter.clone().expect("a proven present value") {
                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20622, _, _) => {
                                                            Some((source_ml_source2_data_property_value20622,))
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20623, _) => {
                                                            Some((source_ml_source2_data_property_value20623,))
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20624, _, _) => {
                                                            Some((source_ml_source2_data_property_value20624,))
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                                                            None::<(f64,)>
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                                                            None::<(f64,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0, Box::new(cell.clone())), source.clone(), false);
                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Next(Box::new(context_4.clone()), Box::new(source_3.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Next(Box::new(context.clone()), Box::new(source.clone()))
                            };
                            match source_ml_source2_s20503.clone() {
                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162::MlSource1TemporaryLowerJavaScriptSourceParametersJoin162Next(context_5, source_4) => {
                                    let context_5 = *context_5;
                                    let source_4 = *source_4;
                                    match parameter.clone().expect("a proven present value") {
                                        self::JavaScriptErrorParameter::ErrorRestParameter(identity, parameter_pattern, _) => {
                                            {
                                                let prototype_2 = match match context_5.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20635, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value20635 = *source_ml_source2_data_property_value20635;
                                                        source_ml_source2_data_property_value20635.clone()
                                                    }
                                                } {
                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, source_ml_source2_data_property_value20636, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value20636.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
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
                                                                let source_ml_source2_s20639 = if !(match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    {
                                                                        let initialized = self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), context_5.clone(), source_4.clone(), true, 0f64, false);
                                                                        {
                                                                            let context_6 = match initialized.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20640) => {
                                                                                    let source_ml_source2_data_property_value20640 = *source_ml_source2_data_property_value20640;
                                                                                    source_ml_source2_data_property_value20640.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_5 = match initialized.clone() {
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20641, _) => {
                                                                                        let source_ml_source2_data_property_value20641 = *source_ml_source2_data_property_value20641;
                                                                                        source_ml_source2_data_property_value20641.clone()
                                                                                    }
                                                                                };
                                                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163Next(Box::new(context_6.clone()), Box::new(source_5.clone()))
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163Next(Box::new(context_5.clone()), Box::new(source_4.clone()))
                                                                };
                                                                match source_ml_source2_s20639.clone() {
                                                                    self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163::MlSource1TemporaryLowerJavaScriptSourceParametersJoin163Next(context_7, source_6) => {
                                                                        let context_7 = *context_7;
                                                                        let source_6 = *source_6;
                                                                        {
                                                                            let source_7 = self::JavaScriptCallSource::SourceParameterRest(identity, at, Box::new(self::construct_java_script_error_constant(prototype_2.clone().expect("a proven present value"))), match match context_7.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20642, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value20642 = *source_ml_source2_data_property_value20642;
                                                                                    source_ml_source2_data_property_value20642.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20643, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value20643.clone()
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
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20644) => {
                                                            let source_ml_source2_data_property_value20644 = *source_ml_source2_data_property_value20644;
                                                            source_ml_source2_data_property_value20644.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_8 = self::JavaScriptCallSource::SourceParameterDefault(identity_2, Box::new(match value_5.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20645, _) => {
                                                                let source_ml_source2_data_property_value20645 = *source_ml_source2_data_property_value20645;
                                                                source_ml_source2_data_property_value20645.clone()
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
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20646) => {
                                                            let source_ml_source2_data_property_value20646 = *source_ml_source2_data_property_value20646;
                                                            source_ml_source2_data_property_value20646.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_9 = match initialized_2.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20647, _) => {
                                                                let source_ml_source2_data_property_value20647 = *source_ml_source2_data_property_value20647;
                                                                source_ml_source2_data_property_value20647.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_s20650 = if has_default {
                                                                {
                                                                    let value_6 = self::lower_java_script_error_expression(parameter_value_3.clone(), context_9.clone(), 0f64);
                                                                    {
                                                                        let context_10 = match value_6.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20651) => {
                                                                                let source_ml_source2_data_property_value20651 = *source_ml_source2_data_property_value20651;
                                                                                source_ml_source2_data_property_value20651.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_10 = self::JavaScriptCallSource::SourceParameterDefault(identity_3, Box::new(match value_6.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20652, _) => {
                                                                                    let source_ml_source2_data_property_value20652 = *source_ml_source2_data_property_value20652;
                                                                                    source_ml_source2_data_property_value20652.clone()
                                                                                }
                                                                            }), Box::new(source_9.clone()));
                                                                            self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(Box::new(context_10.clone()), Box::new(source_10.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(Box::new(context_9.clone()), Box::new(source_9.clone()))
                                                            };
                                                            match source_ml_source2_s20650.clone() {
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
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20653) => {
                                                            let source_ml_source2_data_property_value20653 = *source_ml_source2_data_property_value20653;
                                                            source_ml_source2_data_property_value20653.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_12 = match initialized_3.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20654, _) => {
                                                                let source_ml_source2_data_property_value20654 = *source_ml_source2_data_property_value20654;
                                                                source_ml_source2_data_property_value20654.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_s20657 = if has_default_2 {
                                                                {
                                                                    let value_7 = self::lower_java_script_error_expression(parameter_value_4.clone(), context_12.clone(), 0f64);
                                                                    {
                                                                        let context_13 = match value_7.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20658) => {
                                                                                let source_ml_source2_data_property_value20658 = *source_ml_source2_data_property_value20658;
                                                                                source_ml_source2_data_property_value20658.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_13 = self::JavaScriptCallSource::SourceParameterDefault(identity_4, Box::new(match value_7.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20659, _) => {
                                                                                    let source_ml_source2_data_property_value20659 = *source_ml_source2_data_property_value20659;
                                                                                    source_ml_source2_data_property_value20659.clone()
                                                                                }
                                                                            }), Box::new(source_12.clone()));
                                                                            self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(Box::new(context_13.clone()), Box::new(source_13.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164::MlSource1TemporaryLowerJavaScriptSourceParametersJoin164Next(Box::new(context_12.clone()), Box::new(source_12.clone()))
                                                            };
                                                            match source_ml_source2_s20657.clone() {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_loop173(mut capabilities: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceCapabilityOwner>>>, mut empty: self::JavaScriptCallProgram, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut at: f64) -> self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result {
    loop {
        return if (at < ({ let ml_reference_array552 = capabilities.clone(); let ml_reference_projection552 = ml_reference_array552.borrow().len() as f64; ml_reference_projection552 })) {
            {
                let capability = { let ml_reference_array553 = capabilities.clone(); let ml_reference_position553 = at; let ml_reference_projection553 = self::ml_array::optional_at(&*ml_reference_array553.borrow(), self::ml_array::number_index(ml_reference_position553)); ml_reference_projection553 };
                if ((match (capability.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value20662) => {
                        source_ml_source2_data_property_value20662
                    }
                })) || (({ let ml_values554 = { let ml_reference_value555 = capabilities.borrow().clone(); ml_reference_value555 }; let ml_callback554 = { let capability = capability.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20669, _, _, _, _, _) => {
                        source_ml_source2_data_property_value20669.clone()
                    }
                }) == (match capability.clone().expect("a proven present value") {
                    self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20670, _, _, _, _, _) => {
                        source_ml_source2_data_property_value20670.clone()
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
                    if self::ml_array::includes(&{ let ml_reference_value556 = identities.borrow().clone(); ml_reference_value556 }, &match capability.clone().expect("a proven present value") {
                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20671, _, _, _, _, _) => {
                            source_ml_source2_data_property_value20671.clone()
                        }
                    }, 0f64, false, |a, b| a == b) {
                        self::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Result::MlSource1TemporaryCompileJavaScriptErrorModulePlanWithCapabilitiesLoop173Return(Box::new(self::JavaScriptErrorModuleCompilation::ErrorModuleCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![97, 32, 104, 111, 115, 116, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 32, 69, 114, 114, 111, 114, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 111, 100, 121]))))
                    } else {
                        {
                            let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored174 = { let ml_method_owner557 = identities.clone(); let ml_method_argument557_0 = match capability.clone().expect("a proven present value") {
                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20672, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value20672.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument557_0]; let mut ml_method_values557 = ml_method_owner557.borrow_mut(); let ml_length = ml_method_values557.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values557.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_compile_java_script_error_module_plan_with_capabilities_ignored175 = { let ml_method_owner558 = functions.clone(); let ml_method_argument558_0 = {
                                    let source_ml_source2_constructor_field20674 = match capability.clone().expect("a proven present value") {
                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value20673, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value20673.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field20676 = match capability.clone().expect("a proven present value") {
                                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, source_ml_source2_data_property_value20675, _, _, _, _) => {
                                                source_ml_source2_data_property_value20675.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field20678 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_constructor_field20679 = 0f64;
                                                {
                                                    let source_ml_source2_constructor_field20680 = 2f64;
                                                    {
                                                        let source_ml_source2_constructor_field20681 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                        {
                                                            let source_ml_source2_constructor_field20682 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::InvokeHostCapability(1f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_constructor_field20674.clone(), source_ml_source2_constructor_field20676.clone(), self::ml_string::Text::from_units(vec![113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 104, 111, 115, 116, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121]), Box::new(source_ml_source2_constructor_field20678.clone()), source_ml_source2_constructor_field20679, source_ml_source2_constructor_field20680, source_ml_source2_constructor_field20681.clone(), source_ml_source2_constructor_field20682.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
