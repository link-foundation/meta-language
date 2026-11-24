// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceSwitchClause;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorParameterField;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceBindingTarget;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceSwitchClause;
use super::MlSource1TemporaryLowerJavaScriptSourceIterationContinuationLoop138Result;
use super::collect_java_script_source_lexical_declarations;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_error_property_read;
use super::construct_java_script_source_pattern_binding;
use super::lower_java_script_error_expression;
use super::lower_java_script_source_binding_pattern;
use super::lower_java_script_source_statements;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_iteration_continuation_loop138(mut current: self::JavaScriptErrorSourceContext, mut iteration: self::JavaScriptCallSource, mut identities: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut lexical_bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>, mut at: f64) -> self::MlSource1TemporaryLowerJavaScriptSourceIterationContinuationLoop138Result {
    loop {
        return if (at >= (0f64)) {
            {
                let temporary = match current.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19895, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value19895
                    }
                };
                {
                    let identity = ({ let ml_reference_array516 = identities.clone(); let ml_reference_position516 = at; let ml_reference_projection516 = self::ml_array::optional_at(&*ml_reference_array516.borrow(), self::ml_array::number_index(ml_reference_position516)); ml_reference_projection516 }).clone().unwrap_or_else(|| (-1f64));
                    {
                        let current_2 = {
                            let source_ml_source2_nominal_field19899 = match current.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value19900, _, _, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value19900 = *source_ml_source2_data_property_value19900;
                                    source_ml_source2_data_property_value19900.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field19901 = match current.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value19902, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value19902 = *source_ml_source2_data_property_value19902;
                                        source_ml_source2_data_property_value19902.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field19903 = match current.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value19904, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19904.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field19905 = match current.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19906, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19906
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field19907 = match current.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value19908, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value19908
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field19909 = match current.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value19910, _, _, _, _) => {
                                                        source_ml_source2_data_property_value19910.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field19911 = match current.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value19912, _, _, _) => {
                                                            source_ml_source2_data_property_value19912
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field19913 = match current.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value19914, _, _) => {
                                                                source_ml_source2_data_property_value19914.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field19915 = match current.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value19916, _) => {
                                                                    source_ml_source2_data_property_value19916.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field19917 = match current.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19918) => {
                                                                        source_ml_source2_data_property_value19918.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override19919 = (temporary + 1f64);
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field19899.clone()), Box::new(source_ml_source2_nominal_field19901.clone()), source_ml_source2_nominal_field19903.clone(), source_ml_source2_nominal_override19919, source_ml_source2_nominal_field19907, source_ml_source2_nominal_field19909.clone(), source_ml_source2_nominal_field19911, source_ml_source2_nominal_field19913.clone(), source_ml_source2_nominal_field19915.clone(), source_ml_source2_nominal_field19917.clone())
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
                            let iteration_2 = self::construct_java_script_error_local_binding(current_2.clone(), temporary, {
                                let source_ml_source1_temporary_constructor_field126 = identity;
                                {
                                    let source_ml_source1_temporary_constructor_field127 = match match current_2.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value19920, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value19920 = *source_ml_source2_data_property_value19920;
                                            source_ml_source2_data_property_value19920.clone()
                                        }
                                    } {
                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value19921, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19921.clone()
                                        }
                                    };
                                    if { let ml_values517 = { let ml_reference_value518 = lexical_bindings.borrow().clone(); ml_reference_value518 }; let ml_callback517 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                                        self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value19923, _, _, _, _) => {
                                            source_ml_source2_data_property_value19923
                                        }
                                    }) == identity) }) }; ml_values517.iter().cloned().enumerate().any(|(ml_index517, ml_value517)| ml_callback517(ml_value517)) } {
                                        {
                                            let source_ml_source2_constructor_field19926 = Some(true);
                                            self::JavaScriptCallSource::SourceCloneBindingCell(source_ml_source1_temporary_constructor_field126, source_ml_source1_temporary_constructor_field127.clone(), Some((source_ml_source2_constructor_field19926.clone(),)))
                                        }
                                    } else {
                                        self::JavaScriptCallSource::SourceCloneBindingCell(source_ml_source1_temporary_constructor_field126, source_ml_source1_temporary_constructor_field127.clone(), None::<(Option<bool>,)>)
                                    }
                                }
                            }, iteration.clone(), false);
                            {
                                let at_2 = (at - 1f64);
                                {
                                    (current, iteration, at) = (current_2.clone(), iteration_2.clone(), at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptSourceIterationContinuationLoop138Result::MlSource1TemporaryLowerJavaScriptSourceIterationContinuationLoop138Done(Box::new(current.clone()), Some(iteration.clone()))
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop142(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut clauses: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>, mut source_ml_source1_temporary_values139: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceSwitchClause>>>, mut source_ml_source1_temporary_index139: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index139 < ({ let ml_reference_array519 = source_ml_source1_temporary_values139.clone(); let ml_reference_projection519 = self::ml::Big::from_u128(ml_reference_array519.borrow().len() as u128); ml_reference_projection519 })) {
            {
                let clause = { let ml_reference_array520 = source_ml_source1_temporary_values139.clone(); let ml_reference_position520 = source_ml_source1_temporary_index139.clone(); let ml_reference_projection520 = self::ml_array::at(&*ml_reference_array520.borrow(), ml_reference_position520.to_index()); ml_reference_projection520 };
                match clause.clone() {
                    self::JavaScriptSourceSwitchClause::SourceCaseStatement(value, clause_body) => {
                        let value = *value;
                        {
                            let test = self::lower_java_script_error_expression(value.clone(), current.clone(), (depth + 1f64));
                            {
                                let body = self::lower_java_script_source_statements(clause_body.clone(), 0f64, match test.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19931) => {
                                        let source_ml_source2_data_property_value19931 = *source_ml_source2_data_property_value19931;
                                        source_ml_source2_data_property_value19931.clone()
                                    }
                                }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), false);
                                {
                                    let current_2 = match body.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19932) => {
                                            let source_ml_source2_data_property_value19932 = *source_ml_source2_data_property_value19932;
                                            source_ml_source2_data_property_value19932.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_temporary_lower_java_script_source_selection_continuation_ignored143 = { let ml_method_owner521 = clauses.clone(); let ml_method_argument521_0 = self::JavaScriptCallSourceSwitchClause::SourceSwitchCase(Box::new(match test.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19933, _) => {
                                                let source_ml_source2_data_property_value19933 = *source_ml_source2_data_property_value19933;
                                                source_ml_source2_data_property_value19933.clone()
                                            }
                                        }), Box::new(match body.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19934, _) => {
                                                let source_ml_source2_data_property_value19934 = *source_ml_source2_data_property_value19934;
                                                source_ml_source2_data_property_value19934.clone()
                                            }
                                        })); let ml_inserted = vec![ml_method_argument521_0]; let mut ml_method_values521 = ml_method_owner521.borrow_mut(); let ml_length = ml_method_values521.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values521.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let source_ml_source1_temporary_index139_2 = source_ml_source1_temporary_index139.add(&self::ml::Big::from_i128(1));
                                            {
                                                (current, source_ml_source1_temporary_index139) = (current_2.clone(), source_ml_source1_temporary_index139_2.clone());
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSourceSwitchClause::SourceDefaultStatement(clause_body_2) => {
                        {
                            let body_2 = self::lower_java_script_source_statements(clause_body_2.clone(), 0f64, current.clone(), self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), false);
                            {
                                let current_3 = match body_2.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19935) => {
                                        let source_ml_source2_data_property_value19935 = *source_ml_source2_data_property_value19935;
                                        source_ml_source2_data_property_value19935.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_lower_java_script_source_selection_continuation_ignored144 = { let ml_method_owner522 = clauses.clone(); let ml_method_argument522_0 = self::JavaScriptCallSourceSwitchClause::SourceSwitchDefault(Box::new(match body_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19936, _) => {
                                            let source_ml_source2_data_property_value19936 = *source_ml_source2_data_property_value19936;
                                            source_ml_source2_data_property_value19936.clone()
                                        }
                                    })); let ml_inserted = vec![ml_method_argument522_0]; let mut ml_method_values522 = ml_method_owner522.borrow_mut(); let ml_length = ml_method_values522.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values522.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let source_ml_source1_temporary_index139_3 = source_ml_source1_temporary_index139.add(&self::ml::Big::from_i128(1));
                                        {
                                            (current, source_ml_source1_temporary_index139) = (current_3.clone(), source_ml_source1_temporary_index139_3.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Result {
    MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop146Result {
    MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop146Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop146(mut declarations: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values141: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index141: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop146Result {
    loop {
        return if (source_ml_source1_temporary_index141 < ({ let ml_reference_array523 = source_ml_source1_temporary_values141.clone(); let ml_reference_projection523 = self::ml::Big::from_u128(ml_reference_array523.borrow().len() as u128); ml_reference_projection523 })) {
            {
                let identity = { let ml_reference_array524 = source_ml_source1_temporary_values141.clone(); let ml_reference_position524 = source_ml_source1_temporary_index141.clone(); let ml_reference_projection524 = self::ml_array::at(&*ml_reference_array524.borrow(), ml_reference_position524.to_index()); ml_reference_projection524 };
                {
                    let source_ml_source1_temporary_lower_java_script_source_selection_continuation_ignored147 = { let ml_method_owner525 = declarations.clone(); let ml_method_argument525_0 = identity; let ml_inserted = vec![ml_method_argument525_0]; let mut ml_method_values525 = ml_method_owner525.borrow_mut(); let ml_length = ml_method_values525.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values525.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_source1_temporary_index141_2 = source_ml_source1_temporary_index141.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index141 = source_ml_source1_temporary_index141_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop146Result::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop146Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop145(mut current: self::JavaScriptErrorSourceContext, mut declarations: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values140: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceSwitchClause>>>, mut source_ml_source1_temporary_index140: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Result {
    loop {
        return if (source_ml_source1_temporary_index140 < ({ let ml_reference_array526 = source_ml_source1_temporary_values140.clone(); let ml_reference_projection526 = self::ml::Big::from_u128(ml_reference_array526.borrow().len() as u128); ml_reference_projection526 })) {
            {
                let clause = { let ml_reference_array527 = source_ml_source1_temporary_values140.clone(); let ml_reference_position527 = source_ml_source1_temporary_index140.clone(); let ml_reference_projection527 = self::ml_array::at(&*ml_reference_array527.borrow(), ml_reference_position527.to_index()); ml_reference_projection527 };
                {
                    let source_ml_source1_temporary_values141 = self::collect_java_script_source_lexical_declarations(match clause.clone() {
                        self::JavaScriptSourceSwitchClause::SourceCaseStatement(_, source_ml_source2_data_property_value19941) => {
                            source_ml_source2_data_property_value19941.clone()
                        }
                        self::JavaScriptSourceSwitchClause::SourceDefaultStatement(source_ml_source2_data_property_value19942) => {
                            source_ml_source2_data_property_value19942.clone()
                        }
                    }, current.clone(), 0f64);
                    {
                        let source_ml_source1_temporary_index141 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s19943 = self::source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop146(declarations.clone(), source_ml_source1_temporary_values141.clone(), source_ml_source1_temporary_index141.clone());
                            match source_ml_source2_s19943.clone() {
                                self::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop146Result::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop146Done => {
                                    {
                                        let source_ml_source1_temporary_index140_2 = source_ml_source1_temporary_index140.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index140 = source_ml_source1_temporary_index140_2.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Result::MlSource1TemporaryLowerJavaScriptSourceSelectionContinuationLoop145Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_object_binding_loop149(mut context: self::JavaScriptErrorSourceContext, mut keys: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64, self::JavaScriptCallSource)>>>>>, mut source_ml_source1_temporary_values148: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>, mut source_ml_source1_temporary_index148: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index148 < ({ let ml_reference_array528 = source_ml_source1_temporary_values148.clone(); let ml_reference_projection528 = self::ml::Big::from_u128(ml_reference_array528.borrow().len() as u128); ml_reference_projection528 })) {
            {
                let field = { let ml_reference_array529 = source_ml_source1_temporary_values148.clone(); let ml_reference_position529 = source_ml_source1_temporary_index148.clone(); let ml_reference_projection529 = self::ml_array::at(&*ml_reference_array529.borrow(), ml_reference_position529.to_index()); ml_reference_projection529 };
                if match (match field.clone() {
                    self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, source_ml_source2_data_property_value19946, _, _) => {
                        ((source_ml_source2_data_property_value19946.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorExpression>,))).0).clone()
                    }
                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    {
                        let source_ml_source1_temporary_lower_java_script_source_object_binding_ignored150 = { let ml_method_owner530 = keys.clone(); let ml_method_argument530_0 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field19948 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(match field.clone() {
                                self::JavaScriptErrorParameterField::ErrorParameterField(_, source_ml_source2_data_property_value19947, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value19947.clone()
                                }
                            }));
                            {
                                let source_ml_source2_field19949 = (-1f64);
                                (source_ml_source2_field19949, source_ml_source2_field19948.clone())
                            }
                        })); let ml_inserted = vec![ml_method_argument530_0]; let mut ml_method_values530 = ml_method_owner530.borrow_mut(); let ml_length = ml_method_values530.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values530.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_temporary_index148_2 = source_ml_source1_temporary_index148.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index148 = source_ml_source1_temporary_index148_2.clone();
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let identity = match context.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19950, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value19950
                            }
                        };
                        {
                            let context_2 = {
                                let source_ml_source2_nominal_field19952 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value19953, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value19953 = *source_ml_source2_data_property_value19953;
                                        source_ml_source2_data_property_value19953.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field19954 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value19955, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value19955 = *source_ml_source2_data_property_value19955;
                                            source_ml_source2_data_property_value19955.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field19956 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value19957, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19957.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field19958 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19959, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value19959
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field19960 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value19961, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value19961
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field19962 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value19963, _, _, _, _) => {
                                                            source_ml_source2_data_property_value19963.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field19964 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value19965, _, _, _) => {
                                                                source_ml_source2_data_property_value19965
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field19966 = match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value19967, _, _) => {
                                                                    source_ml_source2_data_property_value19967.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field19968 = match context.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value19969, _) => {
                                                                        source_ml_source2_data_property_value19969.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field19970 = match context.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19971) => {
                                                                            source_ml_source2_data_property_value19971.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override19972 = (identity + 1f64);
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field19952.clone()), Box::new(source_ml_source2_nominal_field19954.clone()), source_ml_source2_nominal_field19956.clone(), source_ml_source2_nominal_override19972, source_ml_source2_nominal_field19960, source_ml_source2_nominal_field19962.clone(), source_ml_source2_nominal_field19964, source_ml_source2_nominal_field19966.clone(), source_ml_source2_nominal_field19968.clone(), source_ml_source2_nominal_field19970.clone())
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
                                let source_ml_source1_temporary_lower_java_script_source_object_binding_ignored151 = { let ml_method_owner531 = keys.clone(); let ml_method_argument531_0 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field19973 = self::JavaScriptCallSource::SourceBinding(identity);
                                    (identity, source_ml_source2_field19973.clone())
                                })); let ml_inserted = vec![ml_method_argument531_0]; let mut ml_method_values531 = ml_method_owner531.borrow_mut(); let ml_length = ml_method_values531.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values531.extend(ml_inserted); ml_length as f64 };
                                {
                                    let source_ml_source1_temporary_index148_3 = source_ml_source1_temporary_index148.add(&self::ml::Big::from_i128(1));
                                    {
                                        (context, source_ml_source1_temporary_index148) = (context_2.clone(), source_ml_source1_temporary_index148_3.clone());
                                        continue;
                                    }
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
pub enum MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Result {
    MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Done(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154 {
    MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154Next(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153 {
    MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_object_binding_loop152(mut parameter: self::JavaScriptErrorParameter, mut mutable: bool, mut depth: f64, mut initialize_existing: bool, mut context: self::JavaScriptErrorSourceContext, mut source: self::JavaScriptCallSource, mut keys: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64, self::JavaScriptCallSource)>>>>>, mut field_at: f64) -> self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Result {
    loop {
        return if (field_at >= (0f64)) {
            {
                let field = {
                    let source_ml_source2_nullish_field_receiver19976 = (match parameter.clone() {
                        self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, source_ml_source2_data_property_value19975, _, _) => {
                            Some(source_ml_source2_data_property_value19975.clone())
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                    }, field_at);
                    {
                        let source_ml_source2_computed_property_key19977 = (source_ml_source2_nullish_field_receiver19976.clone()).1;
                        if match (source_ml_source2_nullish_field_receiver19976.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present19978 = ((source_ml_source2_nullish_field_receiver19976.clone()).0).clone().expect("a proven present value");
                                { let ml_reference_array532 = source_ml_source2_nullish_field_present19978.clone(); let ml_reference_position532 = source_ml_source2_computed_property_key19977; let ml_reference_projection532 = self::ml_array::optional_at(&*ml_reference_array532.borrow(), self::ml_array::number_index(ml_reference_position532)); ml_reference_projection532 }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source2_nullish_field_receiver19976.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
                        }
                    }
                };
                {
                    let key = { let ml_reference_array533 = keys.clone(); let ml_reference_position533 = field_at; let ml_reference_projection533 = self::ml_array::optional_at(&*ml_reference_array533.borrow(), self::ml_array::number_index(ml_reference_position533)); ml_reference_projection533 };
                    if match (field.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let field_at_2 = (field_at - 1f64);
                            {
                                field_at = field_at_2;
                                continue;
                            }
                        }
                    } else {
                        if match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            {
                                let field_at_3 = (field_at - 1f64);
                                {
                                    field_at = field_at_3;
                                    continue;
                                }
                            }
                        } else {
                            {
                                let pattern = match field.clone().expect("a proven present value") {
                                    self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, source_ml_source2_data_property_value19983, _) => {
                                        ((source_ml_source2_data_property_value19983.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone()
                                    }
                                };
                                {
                                    let source_ml_source2_s19986 = if !(match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let initialized = self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), context.clone(), source.clone(), mutable, (depth + 1f64), initialize_existing);
                                            {
                                                let context_2 = match initialized.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19987) => {
                                                        let source_ml_source2_data_property_value19987 = *source_ml_source2_data_property_value19987;
                                                        source_ml_source2_data_property_value19987.clone()
                                                    }
                                                };
                                                {
                                                    let source_2 = match initialized.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19988, _) => {
                                                            let source_ml_source2_data_property_value19988 = *source_ml_source2_data_property_value19988;
                                                            source_ml_source2_data_property_value19988.clone()
                                                        }
                                                    };
                                                    self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(Box::new(context_2.clone()), Box::new(source_2.clone()))
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(Box::new(context.clone()), Box::new(source.clone()))
                                    };
                                    match source_ml_source2_s19986.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(context_3, source_3) => {
                                            let context_3 = *context_3;
                                            let source_3 = *source_3;
                                            {
                                                let raw_ = self::construct_java_script_error_property_read(self::JavaScriptCallSource::SourceBinding(match parameter.clone() {
                                                    self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value19989, _, _) => {
                                                        source_ml_source2_data_property_value19989
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value19990, _) => {
                                                        source_ml_source2_data_property_value19990
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value19991, _, _) => {
                                                        source_ml_source2_data_property_value19991
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value19992, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value19992
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value19993, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value19993
                                                    }
                                                }), ({ let ml_reference_value534 = (key.clone().expect("a proven present value")).borrow().clone(); ml_reference_value534 }).1, context_3.clone());
                                                {
                                                    let source_ml_source2_s20026 = if match field.clone().expect("a proven present value") {
                                                        self::JavaScriptErrorParameterField::ErrorParameterField(_, _, source_ml_source2_data_property_value20027, _, _, _, _) => {
                                                            source_ml_source2_data_property_value20027
                                                        }
                                                    } {
                                                        {
                                                            let value = self::lower_java_script_error_expression(match field.clone().expect("a proven present value") {
                                                                self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, source_ml_source2_data_property_value20028, _, _, _) => {
                                                                    let source_ml_source2_data_property_value20028 = *source_ml_source2_data_property_value20028;
                                                                    source_ml_source2_data_property_value20028.clone()
                                                                }
                                                            }, context_3.clone(), 0f64);
                                                            {
                                                                let identity = match match value.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20029) => {
                                                                        let source_ml_source2_data_property_value20029 = *source_ml_source2_data_property_value20029;
                                                                        source_ml_source2_data_property_value20029.clone()
                                                                    }
                                                                } {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20030, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value20030
                                                                    }
                                                                };
                                                                {
                                                                    let context_4 = {
                                                                        let source_ml_source2_nominal_spread20032 = match value.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20031) => {
                                                                                let source_ml_source2_data_property_value20031 = *source_ml_source2_data_property_value20031;
                                                                                source_ml_source2_data_property_value20031.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field20033 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20034, _, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value20034 = *source_ml_source2_data_property_value20034;
                                                                                    source_ml_source2_data_property_value20034.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field20035 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20036, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value20036 = *source_ml_source2_data_property_value20036;
                                                                                        source_ml_source2_data_property_value20036.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field20037 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20038, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value20038.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20039 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20040, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value20040
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20041 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20042, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value20042
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20043 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20044, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value20044.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20045 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20046, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value20046
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20047 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20048, _, _) => {
                                                                                                                source_ml_source2_data_property_value20048.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20049 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20050, _) => {
                                                                                                                    source_ml_source2_data_property_value20050.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field20051 = match source_ml_source2_nominal_spread20032.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20052) => {
                                                                                                                        source_ml_source2_data_property_value20052.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override20053 = (identity + 1f64);
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20033.clone()), Box::new(source_ml_source2_nominal_field20035.clone()), source_ml_source2_nominal_field20037.clone(), source_ml_source2_nominal_override20053, source_ml_source2_nominal_field20041, source_ml_source2_nominal_field20043.clone(), source_ml_source2_nominal_field20045, source_ml_source2_nominal_field20047.clone(), source_ml_source2_nominal_field20049.clone(), source_ml_source2_nominal_field20051.clone())
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
                                                                    };
                                                                    {
                                                                        let saved = self::JavaScriptCallSource::SourceBinding(identity);
                                                                        {
                                                                            let source_4 = self::construct_java_script_error_local_binding(context_4.clone(), identity, raw_.clone(), self::construct_java_script_source_pattern_binding(context_4.clone(), match field.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorParameterField::ErrorParameterField(source_ml_source2_data_property_value20054, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value20054
                                                                                }
                                                                            }, self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(saved.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)))), Box::new(match value.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20055, _) => {
                                                                                    let source_ml_source2_data_property_value20055 = *source_ml_source2_data_property_value20055;
                                                                                    source_ml_source2_data_property_value20055.clone()
                                                                                }
                                                                            }), Box::new(saved.clone())), source_3.clone(), mutable, initialize_existing, match field.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, _, source_ml_source2_data_property_value20056) => {
                                                                                    ((source_ml_source2_data_property_value20056.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone()
                                                                                }
                                                                            }), false);
                                                                            self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154Next(Box::new(context_4.clone()), Box::new(source_4.clone()))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            let source_5 = self::construct_java_script_source_pattern_binding(context_3.clone(), match field.clone().expect("a proven present value") {
                                                                self::JavaScriptErrorParameterField::ErrorParameterField(source_ml_source2_data_property_value20057, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value20057
                                                                }
                                                            }, raw_.clone(), source_3.clone(), mutable, initialize_existing, match field.clone().expect("a proven present value") {
                                                                self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, _, source_ml_source2_data_property_value20058) => {
                                                                    ((source_ml_source2_data_property_value20058.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone()
                                                                }
                                                            });
                                                            self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154Next(Box::new(context_3.clone()), Box::new(source_5.clone()))
                                                        }
                                                    };
                                                    match source_ml_source2_s20026.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154Next(context_5, source_6) => {
                                                            let context_5 = *context_5;
                                                            let source_6 = *source_6;
                                                            {
                                                                let key_expression = match field.clone().expect("a proven present value") {
                                                                    self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, source_ml_source2_data_property_value20059, _, _) => {
                                                                        ((source_ml_source2_data_property_value20059.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorExpression>,))).0).clone()
                                                                    }
                                                                };
                                                                if !(match (key_expression.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    {
                                                                        let value_2 = self::lower_java_script_error_expression(key_expression.clone().expect("a proven present value"), context_5.clone(), 0f64);
                                                                        {
                                                                            let context_6 = match value_2.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20060) => {
                                                                                    let source_ml_source2_data_property_value20060 = *source_ml_source2_data_property_value20060;
                                                                                    source_ml_source2_data_property_value20060.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_7 = self::construct_java_script_error_local_binding(context_6.clone(), ({ let ml_reference_value535 = (key.clone().expect("a proven present value")).borrow().clone(); ml_reference_value535 }).0, self::JavaScriptCallSource::SourcePropertyKey(Box::new(match value_2.clone() {
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20061, _) => {
                                                                                        let source_ml_source2_data_property_value20061 = *source_ml_source2_data_property_value20061;
                                                                                        source_ml_source2_data_property_value20061.clone()
                                                                                    }
                                                                                }), match match context_6.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20062, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value20062 = *source_ml_source2_data_property_value20062;
                                                                                        source_ml_source2_data_property_value20062.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20063, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value20063.clone()
                                                                                    }
                                                                                }), source_6.clone(), false);
                                                                                {
                                                                                    let field_at_4 = (field_at - 1f64);
                                                                                    {
                                                                                        (context, source, field_at) = (context_6.clone(), source_7.clone(), field_at_4);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let field_at_5 = (field_at - 1f64);
                                                                        {
                                                                            (context, source, field_at) = (context_5.clone(), source_6.clone(), field_at_5);
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
        } else {
            self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Result::MlSource1TemporaryLowerJavaScriptSourceObjectBindingLoop152Done(Box::new(context.clone()), Box::new(source.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Result {
    MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Done,
    MlSource1TemporaryLowerJavaScriptSourceParametersLoop158Return(Box<self::JavaScriptErrorSourceExpression>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceParametersLoop159Result {
    MlSource1TemporaryLowerJavaScriptSourceParametersLoop159Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_parameters_loop159(mut context: self::JavaScriptErrorSourceContext, mut owned: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values156: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_index156: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop159Result {
    loop {
        return if (source_ml_source1_temporary_index156 < ({ let ml_reference_array536 = source_ml_source1_temporary_values156.clone(); let ml_reference_projection536 = self::ml::Big::from_u128(ml_reference_array536.borrow().len() as u128); ml_reference_projection536 })) {
            {
                let declaration = { let ml_reference_array537 = source_ml_source1_temporary_values156.clone(); let ml_reference_position537 = source_ml_source1_temporary_index156.clone(); let ml_reference_projection537 = self::ml_array::at(&*ml_reference_array537.borrow(), ml_reference_position537.to_index()); ml_reference_projection537 };
                if ({ let ml_values538 = { let ml_reference_value539 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20066) => {
                        ((source_ml_source2_data_property_value20066.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value539 }; let ml_callback538 = { let declaration = declaration.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value20069, _, _, _, _) => {
                        source_ml_source2_data_property_value20069
                    }
                }) == (match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value20070, _) => {
                        source_ml_source2_data_property_value20070
                    }
                })) }) }; ml_values538.iter().cloned().enumerate().any(|(ml_index538, ml_value538)| ml_callback538(ml_value538)) } && !(self::ml_array::includes(&{ let ml_reference_value540 = owned.borrow().clone(); ml_reference_value540 }, &match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value20071, _) => {
                        source_ml_source2_data_property_value20071
                    }
                }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                    {
                        let source_ml_source1_temporary_lower_java_script_source_parameters_ignored160 = { let ml_method_owner541 = owned.clone(); let ml_method_argument541_0 = match declaration.clone() {
                            self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value20072, _) => {
                                source_ml_source2_data_property_value20072
                            }
                        }; let ml_inserted = vec![ml_method_argument541_0]; let mut ml_method_values541 = ml_method_owner541.borrow_mut(); let ml_length = ml_method_values541.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values541.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_temporary_index156_2 = source_ml_source1_temporary_index156.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index156 = source_ml_source1_temporary_index156_2.clone();
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index156_3 = source_ml_source1_temporary_index156.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index156 = source_ml_source1_temporary_index156_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptSourceParametersLoop159Result::MlSource1TemporaryLowerJavaScriptSourceParametersLoop159Done
        };
    }
}
