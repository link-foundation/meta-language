// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptErrorArrayBindingElement;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptPatternBindingDeclarations;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceBindingTarget;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceStatement;
use super::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221;
use super::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Result;
use super::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213;
use super::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215;
use super::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218;
use super::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result;
use super::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result;
use super::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Result;
use super::MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Result;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_field_key;
use super::construct_java_script_source_pattern_binding;
use super::lower_java_script_error_expression;
use super::lower_java_script_source_binding_pattern;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_append_java_script_pattern_binding_declarations_loop222;
use super::source_ml_source1_temporary_collect_java_script_pattern_binding_declarations_loop214;
use super::source_ml_source1_temporary_collect_java_script_pattern_binding_declarations_loop217;
use super::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop230;
use super::source_ml_source1_temporary_construct_java_script_lexical_cell_failure_loop225;
use super::source_ml_source1_temporary_lower_java_script_source_array_binding_loop224;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn collect_java_script_pattern_binding_declarations(pattern: self::JavaScriptErrorParameter, mutable: bool, depth: f64, shared: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptPatternBindingDeclarations {
    if (depth > (128f64)) {
        self::JavaScriptPatternBindingDeclarations::BindingDeclarations(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallBindingDeclaration>::new())), self::ml_string::Text::from_units(vec![112, 97, 116, 116, 101, 114, 110, 32, 111, 117, 116, 112, 117, 116, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 112, 97, 116, 116, 101, 114, 110]))
    } else {
        {
            let declarations = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallBindingDeclaration>::new()));
            {
                let source_ml_source2_s18131 = match pattern.clone() {
                    self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                        self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Return(Box::new(self::JavaScriptPatternBindingDeclarations::BindingDeclarations(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallBindingDeclaration>::new())), self::ml_string::Text::from_units(vec![112, 97, 116, 116, 101, 114, 110, 32, 111, 117, 116, 112, 117, 116, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 112, 97, 116, 116, 101, 114, 110]))))
                    }
                    self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                        self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Return(Box::new(self::JavaScriptPatternBindingDeclarations::BindingDeclarations(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallBindingDeclaration>::new())), self::ml_string::Text::from_units(vec![112, 97, 116, 116, 101, 114, 110, 32, 111, 117, 116, 112, 117, 116, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 112, 97, 116, 116, 101, 114, 110]))))
                    }
                    self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                        self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Return(Box::new(self::JavaScriptPatternBindingDeclarations::BindingDeclarations(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallBindingDeclaration>::new())), self::ml_string::Text::from_units(vec![112, 97, 116, 116, 101, 114, 110, 32, 111, 117, 116, 112, 117, 116, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 112, 97, 116, 116, 101, 114, 110]))))
                    }
                    self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, fields, pattern_rest, _) => {
                        {
                            let source_ml_source1_temporary_index211 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s18148 = self::source_ml_source1_temporary_collect_java_script_pattern_binding_declarations_loop214(mutable, depth, shared.clone(), declarations.clone(), fields.clone(), source_ml_source1_temporary_index211.clone());
                                match source_ml_source2_s18148.clone() {
                                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Done => {
                                        {
                                            let source_ml_source2_s18151 = {
                                                let rest = ((pattern_rest.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
                                                if {
                                                    let source_ml_source2_comparison_operand18153 = None::<()>;
                                                    !(match (rest.clone(), source_ml_source2_comparison_operand18153.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                } {
                                                    {
                                                        let source_ml_source1_temporary_collect_java_script_pattern_binding_declarations_ignored216 = { let ml_method_owner341 = declarations.clone(); let ml_method_argument341_0 = self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(rest.clone().expect("a proven present value"), (mutable && !(self::ml_array::includes(&{ let ml_reference_value342 = shared.borrow().clone(); ml_reference_value342 }, &rest.clone().expect("a proven present value"), 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))))); let ml_inserted = vec![ml_method_argument341_0]; let mut ml_method_values341 = ml_method_owner341.borrow_mut(); let ml_length = ml_method_values341.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values341.extend(ml_inserted); ml_length as f64 };
                                                        self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215Next
                                                    }
                                                } else {
                                                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215Next
                                                }
                                            };
                                            match source_ml_source2_s18151.clone() {
                                                self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215Next => {
                                                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Next
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Return(source_ml_source1_temporary_result216) => {
                                        let source_ml_source1_temporary_result216 = *source_ml_source1_temporary_result216;
                                        self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Return(Box::new(source_ml_source1_temporary_result216.clone()))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, elements, pattern_rest_2) => {
                        {
                            let source_ml_source1_temporary_index212 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s18154 = self::source_ml_source1_temporary_collect_java_script_pattern_binding_declarations_loop217(mutable, depth, shared.clone(), declarations.clone(), elements.clone(), source_ml_source1_temporary_index212.clone());
                                match source_ml_source2_s18154.clone() {
                                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Done => {
                                        {
                                            let source_ml_source2_s18159 = {
                                                let rest_2 = ((pattern_rest_2.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorArrayBindingElement>,))).0).clone();
                                                if !(match (rest_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                    {
                                                        let child = self::append_java_script_pattern_binding_declarations(declarations.clone(), match rest_2.clone().expect("a proven present value") {
                                                            self::JavaScriptErrorArrayBindingElement::ErrorArrayBindingElement(source_ml_source2_data_property_value18160, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18160
                                                            }
                                                        }, match rest_2.clone().expect("a proven present value") {
                                                            self::JavaScriptErrorArrayBindingElement::ErrorArrayBindingElement(_, _, _, source_ml_source2_data_property_value18161, _) => {
                                                                ((source_ml_source2_data_property_value18161.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone()
                                                            }
                                                        }, mutable, depth, shared.clone());
                                                        if ((match child.clone() {
                                                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value18163) => {
                                                                source_ml_source2_data_property_value18163.clone()
                                                            }
                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                            self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218Return(Box::new(child.clone()))
                                                        } else {
                                                            self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218Next
                                                        }
                                                    }
                                                } else {
                                                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218Next
                                                }
                                            };
                                            match source_ml_source2_s18159.clone() {
                                                self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218Next => {
                                                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Next
                                                }
                                                self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218Return(source_ml_source1_temporary_result218) => {
                                                    let source_ml_source1_temporary_result218 = *source_ml_source1_temporary_result218;
                                                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Return(Box::new(source_ml_source1_temporary_result218.clone()))
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Return(source_ml_source1_temporary_result218_2) => {
                                        let source_ml_source1_temporary_result218_2 = *source_ml_source1_temporary_result218_2;
                                        self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Return(Box::new(source_ml_source1_temporary_result218_2.clone()))
                                    }
                                }
                            }
                        }
                    }
                };
                match source_ml_source2_s18131.clone() {
                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Next => {
                        if (({ let ml_reference_array343 = declarations.clone(); let ml_reference_projection343 = ml_reference_array343.borrow().len() as f64; ml_reference_projection343 }) > (4096f64)) {
                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallBindingDeclaration>::new())), self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 111, 117, 116, 112, 117, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(declarations.clone(), self::ml_string::Text::from_units(vec![]))
                        }
                    }
                    self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Return(source_ml_source1_temporary_result213) => {
                        let source_ml_source1_temporary_result213 = *source_ml_source1_temporary_result213;
                        source_ml_source1_temporary_result213.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_pattern_binding_declarations(declarations: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, identity: f64, pattern: Option<self::JavaScriptErrorParameter>, mutable: bool, depth: f64, shared: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptPatternBindingDeclarations {
    {
        let source_ml_source1_temporary_append_java_script_pattern_binding_declarations_ignored220 = { let ml_method_owner344 = declarations.clone(); let ml_method_argument344_0 = self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(identity, (mutable && !(self::ml_array::includes(&{ let ml_reference_value345 = shared.borrow().clone(); ml_reference_value345 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))))); let ml_inserted = vec![ml_method_argument344_0]; let mut ml_method_values344 = ml_method_owner344.borrow_mut(); let ml_length = ml_method_values344.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values344.extend(ml_inserted); ml_length as f64 };
        {
            let source_ml_source2_s18168 = if !(match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                {
                    let child = self::collect_java_script_pattern_binding_declarations(pattern.clone().expect("a proven present value"), mutable, (depth + 1f64), shared.clone());
                    if ((match child.clone() {
                        self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value18170) => {
                            source_ml_source2_data_property_value18170.clone()
                        }
                    }) != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221Return(Box::new(child.clone()))
                    } else {
                        {
                            let source_ml_source1_temporary_values219 = match child.clone() {
                                self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value18171, _) => {
                                    source_ml_source2_data_property_value18171.clone()
                                }
                            };
                            {
                                let source_ml_source1_temporary_index219 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s18172 = self::source_ml_source1_temporary_append_java_script_pattern_binding_declarations_loop222(declarations.clone(), source_ml_source1_temporary_values219.clone(), source_ml_source1_temporary_index219.clone());
                                    match source_ml_source2_s18172.clone() {
                                        self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Result::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Done => {
                                            self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221Next
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221Next
            };
            match source_ml_source2_s18168.clone() {
                self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221Next => {
                    if (({ let ml_reference_array346 = declarations.clone(); let ml_reference_projection346 = ml_reference_array346.borrow().len() as f64; ml_reference_projection346 }) > (4096f64)) {
                        self::JavaScriptPatternBindingDeclarations::BindingDeclarations(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallBindingDeclaration>::new())), self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 111, 117, 116, 112, 117, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 117, 99, 116, 117, 114, 97, 108, 32, 98, 117, 100, 103, 101, 116]))
                    } else {
                        self::JavaScriptPatternBindingDeclarations::BindingDeclarations(declarations.clone(), self::ml_string::Text::from_units(vec![]))
                    }
                }
                self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221Return(source_ml_source1_temporary_result221) => {
                    let source_ml_source1_temporary_result221 = *source_ml_source1_temporary_result221;
                    source_ml_source1_temporary_result221.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_source_array_binding(pattern: self::JavaScriptErrorParameter, first: self::JavaScriptErrorSourceContext, continuation: self::JavaScriptCallSource, mutable: bool, depth: f64, initialize_existing: bool) -> self::JavaScriptErrorSourceExpression {
    if (depth > (128f64)) {
        self::refuse_java_script_error_source(first.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))
    } else {
        match pattern.clone() {
            self::JavaScriptErrorParameter::ErrorArrayParameter(pattern_identity, _, _, iterator_key, elements, pattern_rest) => {
                let iterator_key = *iterator_key;
                {
                    let prototype = match match first.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18189, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value18189 = *source_ml_source2_data_property_value18189;
                            source_ml_source2_data_property_value18189.clone()
                        }
                    } {
                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(_, _, source_ml_source2_data_property_value18190, _, _, _, _, _, _, _, _) => {
                            ((source_ml_source2_data_property_value18190.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    };
                    if match (prototype.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::refuse_java_script_error_source(first.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))
                    } else {
                        {
                            let key = self::lower_java_script_error_expression(iterator_key.clone(), first.clone(), 0f64);
                            {
                                let context = std::rc::Rc::new(std::cell::RefCell::new(match key.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18191) => {
                                        let source_ml_source2_data_property_value18191 = *source_ml_source2_data_property_value18191;
                                        source_ml_source2_data_property_value18191.clone()
                                    }
                                }));
                                {
                                    let source = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
                                    {
                                        let rest = ((pattern_rest.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorArrayBindingElement>,))).0).clone();
                                        {
                                            let source_5 = if !(match (rest.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                {
                                                    let nested_pattern = match rest.clone().expect("a proven present value") {
                                                        self::JavaScriptErrorArrayBindingElement::ErrorArrayBindingElement(_, _, _, source_ml_source2_data_property_value18192, _) => {
                                                            ((source_ml_source2_data_property_value18192.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone()
                                                        }
                                                    };
                                                    {
                                                        let source_3 = if !(match (nested_pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                            {
                                                                let nested = self::lower_java_script_source_binding_pattern(nested_pattern.clone().expect("a proven present value"), { let ml_cell_value = context.borrow().clone(); ml_cell_value }, source.clone(), mutable, (depth + 1f64), true);
                                                                { let ml_cell347 = context.clone(); let ml_value347 = match nested.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value18193) => {
                                                                        let source_ml_source2_data_property_value18193 = *source_ml_source2_data_property_value18193;
                                                                        source_ml_source2_data_property_value18193.clone()
                                                                    }
                                                                }; *ml_cell347.borrow_mut() = ml_value347; {
                                                                    let source_2 = match nested.clone() {
                                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18194, _) => {
                                                                            let source_ml_source2_data_property_value18194 = *source_ml_source2_data_property_value18194;
                                                                            source_ml_source2_data_property_value18194.clone()
                                                                        }
                                                                    };
                                                                    source_2.clone()
                                                                } }
                                                            }
                                                        } else {
                                                            source.clone()
                                                        };
                                                        {
                                                            let source_4 = self::construct_java_script_source_pattern_binding({ let ml_cell_value = context.borrow().clone(); ml_cell_value }, match rest.clone().expect("a proven present value") {
                                                                self::JavaScriptErrorArrayBindingElement::ErrorArrayBindingElement(source_ml_source2_data_property_value18195, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18195
                                                                }
                                                            }, self::JavaScriptCallSource::SourceIteratorRemainder(pattern_identity, Box::new(self::construct_java_script_error_constant(prototype.clone().expect("a proven present value"))), match match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18196, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value18196 = *source_ml_source2_data_property_value18196;
                                                                    source_ml_source2_data_property_value18196.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18197, _, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value18197.clone()
                                                                }
                                                            }), source_3.clone(), mutable, true, match rest.clone().expect("a proven present value") {
                                                                self::JavaScriptErrorArrayBindingElement::ErrorArrayBindingElement(_, _, _, _, source_ml_source2_data_property_value18198) => {
                                                                    ((source_ml_source2_data_property_value18198.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone()
                                                                }
                                                            });
                                                            source_4.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                source.clone()
                                            };
                                            {
                                                let at = ({ let ml_reference_array348 = elements.clone(); let ml_reference_projection348 = ml_reference_array348.borrow().len() as f64; ml_reference_projection348 } - 1f64);
                                                {
                                                    let source_6 = self::source_ml_source1_temporary_lower_java_script_source_array_binding_loop224(pattern.clone(), mutable, depth, context.clone(), source_5.clone(), at);
                                                    {
                                                        let initializer = self::JavaScriptCallSource::SourceIteratorBinding(pattern_identity, Box::new(self::JavaScriptCallSource::SourceBinding(pattern_identity)), Box::new(match key.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value18199, _) => {
                                                                let source_ml_source2_data_property_value18199 = *source_ml_source2_data_property_value18199;
                                                                source_ml_source2_data_property_value18199.clone()
                                                            }
                                                        }), Box::new(source_6.clone()), match match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18200, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value18200 = *source_ml_source2_data_property_value18200;
                                                                source_ml_source2_data_property_value18200.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18201, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value18201.clone()
                                                            }
                                                        });
                                                        if initialize_existing {
                                                            {
                                                                let identity = match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18202, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value18202
                                                                    }
                                                                };
                                                                { let ml_cell349 = context.clone(); let ml_value349 = {
                                                                    let source_ml_source2_nominal_spread18203 = { let ml_cell_value = context.borrow().clone(); ml_cell_value };
                                                                    {
                                                                        let source_ml_source2_nominal_field18204 = match source_ml_source2_nominal_spread18203.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18205, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value18205 = *source_ml_source2_data_property_value18205;
                                                                                source_ml_source2_data_property_value18205.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field18206 = match source_ml_source2_nominal_spread18203.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18207, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value18207 = *source_ml_source2_data_property_value18207;
                                                                                    source_ml_source2_data_property_value18207.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field18208 = match source_ml_source2_nominal_spread18203.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18209, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value18209.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field18210 = match source_ml_source2_nominal_spread18203.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18211, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value18211
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18212 = match source_ml_source2_nominal_spread18203.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18213, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value18213
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18214 = match source_ml_source2_nominal_spread18203.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18215, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value18215.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field18216 = match source_ml_source2_nominal_spread18203.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18217, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value18217
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field18218 = match source_ml_source2_nominal_spread18203.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18219, _, _) => {
                                                                                                            source_ml_source2_data_property_value18219.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field18220 = match source_ml_source2_nominal_spread18203.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18221, _) => {
                                                                                                                source_ml_source2_data_property_value18221.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field18222 = match source_ml_source2_nominal_spread18203.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18223) => {
                                                                                                                    source_ml_source2_data_property_value18223.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override18224 = (identity + 1f64);
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18204.clone()), Box::new(source_ml_source2_nominal_field18206.clone()), source_ml_source2_nominal_field18208.clone(), source_ml_source2_nominal_override18224, source_ml_source2_nominal_field18212, source_ml_source2_nominal_field18214.clone(), source_ml_source2_nominal_field18216, source_ml_source2_nominal_field18218.clone(), source_ml_source2_nominal_field18220.clone(), source_ml_source2_nominal_field18222.clone())
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
                                                                }; *ml_cell349.borrow_mut() = ml_value349; self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding({ let ml_cell_value = context.borrow().clone(); ml_cell_value }, identity, initializer.clone(), continuation.clone(), false)), Box::new({ let ml_cell_value = context.borrow().clone(); ml_cell_value })) }
                                                            }
                                                        } else {
                                                            {
                                                                let outputs = self::collect_java_script_pattern_binding_declarations(pattern.clone(), mutable, depth, (match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18225, _) => {
                                                                        ((source_ml_source2_data_property_value18225.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                    }
                                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                                                if ((match outputs.clone() {
                                                                    self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value18227) => {
                                                                        source_ml_source2_data_property_value18227.clone()
                                                                    }
                                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::refuse_java_script_error_source({ let ml_cell_value = context.borrow().clone(); ml_cell_value }, &(match outputs.clone() {
                                                                        self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value18228) => {
                                                                            source_ml_source2_data_property_value18228.clone()
                                                                        }
                                                                    }))
                                                                } else {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceBindingScope(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values350 = { let ml_reference_value351 = (match outputs.clone() {
                                                                        self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value18229, _) => {
                                                                            source_ml_source2_data_property_value18229.clone()
                                                                        }
                                                                    }).borrow().clone(); ml_reference_value351 }; let ml_callback350 = { let context = context.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallBindingDeclaration| -> bool { !({ let ml_values352 = { let ml_reference_value353 = ((match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18230) => {
                                                                            ((source_ml_source2_data_property_value18230.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                                                        }
                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value353 }; let ml_callback352 = { let row = row.clone(); std::rc::Rc::new(move |binding: self::JavaScriptSourceLexicalBinding| -> bool { ((match binding.clone() {
                                                                        self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18233, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18233
                                                                        }
                                                                    }) == (match row.clone() {
                                                                        self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value18234, _) => {
                                                                            source_ml_source2_data_property_value18234
                                                                        }
                                                                    })) }) }; ml_values352.iter().cloned().enumerate().any(|(ml_index352, ml_value352)| ml_callback352(ml_value352)) }) }) }; ml_values350.iter().cloned().enumerate().filter_map(|(ml_index350, ml_value350)| if ml_callback350(ml_value350.clone()) { Some(ml_value350) } else { None }).collect::<Vec<_>>() })), Box::new(initializer.clone()), Box::new(continuation.clone()))), Box::new({ let ml_cell_value = context.borrow().clone(); ml_cell_value }))
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
            _ => {
                self::refuse_java_script_error_source(first.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 98, 105, 110, 100, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_lexical_cell(context: self::JavaScriptErrorSourceContext) -> self::JavaScriptCallSource {
    {
        let source_ml_source2_constructor_field18235 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Null);
        {
            let source_ml_source2_constructor_field18238 = match match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18236, _, _, _, _, _, _, _, _) => {
                    let source_ml_source2_data_property_value18236 = *source_ml_source2_data_property_value18236;
                    source_ml_source2_data_property_value18236.clone()
                }
            } {
                self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value18237, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value18237.clone()
                }
            };
            {
                let source_ml_source2_constructor_field18243 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                    let source_ml_source2_constructor_field18239 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                    {
                        let source_ml_source2_constructor_field18240 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined);
                        self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field18239.clone()), Box::new(source_ml_source2_constructor_field18240.clone()), None::<(Option<bool>,)>)
                    }
                }, {
                    let source_ml_source2_constructor_field18241 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100])));
                    {
                        let source_ml_source2_constructor_field18242 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Boolean(false));
                        self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field18241.clone()), Box::new(source_ml_source2_constructor_field18242.clone()), None::<(Option<bool>,)>)
                    }
                }]));
                self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field18235.clone()), source_ml_source2_constructor_field18243.clone(), source_ml_source2_constructor_field18238.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_lexical_cell_failure(context: self::JavaScriptErrorSourceContext, identity: f64) -> self::JavaScriptCallSource {
    {
        let binding = { let ml_values354 = { let ml_reference_value355 = ((match context.clone() {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18244) => {
                ((source_ml_source2_data_property_value18244.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value355 }; let ml_callback354 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
            self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18246, _, _, _, _) => {
                source_ml_source2_data_property_value18246
            }
        }) == identity) }) }; ml_values354.iter().cloned().enumerate().find_map(|(ml_index354, ml_value354)| if ml_callback354(ml_value354.clone()) { Some(ml_value354) } else { None }) };
        {
            let name = if match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                self::ml_string::Text::from_units(vec![])
            } else {
                match binding.clone().expect("a proven present value") {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, source_ml_source2_data_property_value18247, _, _, _) => {
                        source_ml_source2_data_property_value18247.clone()
                    }
                }
            };
            {
                let message = ((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 97, 99, 99, 101, 115, 115, 32, 39])).concat(&name)).concat(&self::ml_string::Text::from_units(vec![39, 32, 98, 101, 102, 111, 114, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110]));
                {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source2_s18248 = self::source_ml_source1_temporary_construct_java_script_lexical_cell_failure_loop225(&(message), units.clone(), at);
                            match source_ml_source2_s18248.clone() {
                                self::MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Result::MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Done => {
                                    self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(units.clone())), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Boolean(true))])))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_lexical_cell_read(context: self::JavaScriptErrorSourceContext, identity: f64) -> self::JavaScriptCallSource {
    {
        let cell = self::JavaScriptCallSource::SourceBinding(identity);
        {
            let read = {
                let source_ml_source2_constructor_field18250 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])));
                self::JavaScriptCallSource::SourceGet(Box::new(cell.clone()), Box::new(source_ml_source2_constructor_field18250.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
            };
            if { let ml_values356 = { let ml_reference_value357 = ((match context.clone() {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18251) => {
                    ((source_ml_source2_data_property_value18251.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                }
            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value357 }; let ml_callback356 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18253, _, _, _, _) => {
                    source_ml_source2_data_property_value18253
                }
            }) == identity) }) }; ml_values356.iter().cloned().enumerate().any(|(ml_index356, ml_value356)| ml_callback356(ml_value356)) } {
                self::JavaScriptCallSource::SourceIf(Box::new({
                    let source_ml_source2_constructor_field18255 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100])));
                    self::JavaScriptCallSource::SourceGet(Box::new(cell.clone()), Box::new(source_ml_source2_constructor_field18255.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
                }), Box::new(read.clone()), Box::new(self::construct_java_script_lexical_cell_failure(context.clone(), identity)))
            } else {
                read.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_lexical_cell_write(context: self::JavaScriptErrorSourceContext, identity: f64, value: self::JavaScriptCallSource, initializing: bool) -> self::JavaScriptErrorSourceExpression {
    {
        let held_id = match context.clone() {
            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18256, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value18256
            }
        };
        {
            let written_id = (held_id + 1f64);
            {
                let flag_id = (written_id + 1f64);
                {
                    let next = {
                        let source_ml_source2_nominal_field18258 = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value18259, _, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value18259 = *source_ml_source2_data_property_value18259;
                                source_ml_source2_data_property_value18259.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field18260 = match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value18261, _, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value18261 = *source_ml_source2_data_property_value18261;
                                    source_ml_source2_data_property_value18261.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field18262 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value18263, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value18263.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field18264 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value18265, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value18265
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field18266 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value18267, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value18267
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field18268 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value18269, _, _, _, _) => {
                                                    source_ml_source2_data_property_value18269.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field18270 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value18271, _, _, _) => {
                                                        source_ml_source2_data_property_value18271
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field18272 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value18273, _, _) => {
                                                            source_ml_source2_data_property_value18273.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field18274 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value18275, _) => {
                                                                source_ml_source2_data_property_value18275.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field18276 = match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18277) => {
                                                                    source_ml_source2_data_property_value18277.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_override18278 = (flag_id + 1f64);
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field18258.clone()), Box::new(source_ml_source2_nominal_field18260.clone()), source_ml_source2_nominal_field18262.clone(), source_ml_source2_nominal_override18278, source_ml_source2_nominal_field18266, source_ml_source2_nominal_field18268.clone(), source_ml_source2_nominal_field18270, source_ml_source2_nominal_field18272.clone(), source_ml_source2_nominal_field18274.clone(), source_ml_source2_nominal_field18276.clone())
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
                        let held = self::JavaScriptCallSource::SourceBinding(held_id);
                        {
                            let cell = self::JavaScriptCallSource::SourceBinding(identity);
                            {
                                let binding = { let ml_values358 = { let ml_reference_value359 = ((match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18279) => {
                                        ((source_ml_source2_data_property_value18279.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value359 }; let ml_callback358 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value18281, _, _, _, _) => {
                                        source_ml_source2_data_property_value18281
                                    }
                                }) == identity) }) }; ml_values358.iter().cloned().enumerate().find_map(|(ml_index358, ml_value358)| if ml_callback358(ml_value358.clone()) { Some(ml_value358) } else { None }) };
                                {
                                    let write = self::JavaScriptCallSource::SourceSnapshotSet(Box::new(cell.clone()), Box::new(self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))), Box::new(held.clone()));
                                    {
                                        let body = if initializing {
                                            self::construct_java_script_error_local_binding(next.clone(), written_id, write.clone(), self::construct_java_script_error_local_binding(next.clone(), flag_id, self::JavaScriptCallSource::SourceSnapshotSet(Box::new(cell.clone()), Box::new(self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100])))), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Boolean(true)))), held.clone(), false), false)
                                        } else {
                                            self::JavaScriptCallSource::SourceIf(Box::new({
                                                let source_ml_source2_constructor_field18283 = self::construct_java_script_source_field_key(&(self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100])));
                                                self::JavaScriptCallSource::SourceGet(Box::new(cell.clone()), Box::new(source_ml_source2_constructor_field18283.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
                                            }), Box::new(if (!(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match binding.clone().expect("a proven present value") {
                                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, source_ml_source2_data_property_value18284, _, _) => {
                                                    source_ml_source2_data_property_value18284
                                                }
                                            }) {
                                                self::JavaScriptCallSource::SourceImmutableBindingWrite(Box::new(held.clone()))
                                            } else {
                                                self::construct_java_script_error_local_binding(next.clone(), written_id, write.clone(), held.clone(), false)
                                            }), Box::new(self::construct_java_script_lexical_cell_failure(next.clone(), identity)))
                                        };
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::construct_java_script_error_local_binding(next.clone(), held_id, value.clone(), body.clone(), false)), Box::new(next.clone()))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn collect_java_script_source_lexical_declarations(statements: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, context: self::JavaScriptErrorSourceContext, depth: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (depth > (128f64)) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        {
            let declarations = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let source_ml_source1_temporary_index227 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s18285 = self::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop230(context.clone(), depth, declarations.clone(), statements.clone(), source_ml_source1_temporary_index227.clone());
                    match source_ml_source2_s18285.clone() {
                        self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Result::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Done => {
                            declarations.clone()
                        }
                    }
                }
            }
        }
    }
}
