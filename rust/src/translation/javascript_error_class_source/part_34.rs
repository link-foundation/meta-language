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
pub fn source_ml_source1_temporary_lower_java_script_source_iteration_continuation_loop138(current: std::rc::Rc<std::cell::RefCell<self::JavaScriptErrorSourceContext>>, iteration: self::JavaScriptCallSource, identities: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, at: std::rc::Rc<std::cell::RefCell<f64>>) -> Option<self::JavaScriptCallSource> {
    if (({ let ml_cell_value = at.borrow().clone(); ml_cell_value }) >= (0f64)) {
        {
            let temporary = match { let ml_cell_value = current.borrow().clone(); ml_cell_value } {
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19892, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value19892
                }
            };
            { let ml_cell516 = current.clone(); let ml_value516 = {
                let source_ml_source2_nominal_spread19893 = { let ml_cell_value = current.borrow().clone(); ml_cell_value };
                {
                    let source_ml_source2_nominal_field19894 = match source_ml_source2_nominal_spread19893.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value19895, _, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value19895 = *source_ml_source2_data_property_value19895;
                            source_ml_source2_data_property_value19895.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field19896 = match source_ml_source2_nominal_spread19893.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value19897, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value19897 = *source_ml_source2_data_property_value19897;
                                source_ml_source2_data_property_value19897.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field19898 = match source_ml_source2_nominal_spread19893.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value19899, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value19899.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field19900 = match source_ml_source2_nominal_spread19893.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19901, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value19901
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field19902 = match source_ml_source2_nominal_spread19893.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value19903, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value19903
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field19904 = match source_ml_source2_nominal_spread19893.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value19905, _, _, _, _) => {
                                                source_ml_source2_data_property_value19905.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field19906 = match source_ml_source2_nominal_spread19893.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value19907, _, _, _) => {
                                                    source_ml_source2_data_property_value19907
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field19908 = match source_ml_source2_nominal_spread19893.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value19909, _, _) => {
                                                        source_ml_source2_data_property_value19909.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field19910 = match source_ml_source2_nominal_spread19893.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value19911, _) => {
                                                            source_ml_source2_data_property_value19911.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field19912 = match source_ml_source2_nominal_spread19893.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19913) => {
                                                                source_ml_source2_data_property_value19913.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override19914 = (temporary + 1f64);
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field19894.clone()), Box::new(source_ml_source2_nominal_field19896.clone()), source_ml_source2_nominal_field19898.clone(), source_ml_source2_nominal_override19914, source_ml_source2_nominal_field19902, source_ml_source2_nominal_field19904.clone(), source_ml_source2_nominal_field19906, source_ml_source2_nominal_field19908.clone(), source_ml_source2_nominal_field19910.clone(), source_ml_source2_nominal_field19912.clone())
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
            }; *ml_cell516.borrow_mut() = ml_value516; {
                let iteration_2 = self::construct_java_script_error_local_binding({ let ml_cell_value = current.borrow().clone(); ml_cell_value }, temporary, {
                    let source_ml_source1_temporary_constructor_field126 = ({
                        let source_ml_source2_array_read_index19916 = { let ml_cell_value = at.borrow().clone(); ml_cell_value };
                        { let ml_reference_array517 = identities.clone(); let ml_reference_position517 = source_ml_source2_array_read_index19916; let ml_reference_projection517 = self::ml_array::optional_at(&*ml_reference_array517.borrow(), self::ml_array::number_index(ml_reference_position517)); ml_reference_projection517 }
                    }).clone().unwrap_or_else(|| (-1f64));
                    {
                        let source_ml_source1_temporary_constructor_field127 = match match { let ml_cell_value = current.borrow().clone(); ml_cell_value } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value19917, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value19917 = *source_ml_source2_data_property_value19917;
                                source_ml_source2_data_property_value19917.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value19918, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value19918.clone()
                            }
                        };
                        if { let ml_values518 = { let ml_reference_value519 = ((match { let ml_cell_value = current.borrow().clone(); ml_cell_value } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19919) => {
                                ((source_ml_source2_data_property_value19919.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                            }
                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value519 }; let ml_callback518 = { let identities = identities.clone(); let at = at.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { {
                            let source_ml_source2_comparison_operand19923 = match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value19920, _, _, _, _) => {
                                    source_ml_source2_data_property_value19920
                                }
                            };
                            {
                                let source_ml_source2_comparison_operand19924 = {
                                    let source_ml_source2_array_read_value19921 = identities.clone();
                                    {
                                        let source_ml_source2_array_read_index19922 = { let ml_cell_value = at.borrow().clone(); ml_cell_value };
                                        { let ml_reference_array520 = source_ml_source2_array_read_value19921.clone(); let ml_reference_position520 = source_ml_source2_array_read_index19922; let ml_reference_projection520 = self::ml_array::optional_at(&*ml_reference_array520.borrow(), self::ml_array::number_index(ml_reference_position520)); ml_reference_projection520 }
                                    }
                                };
                                match (source_ml_source2_comparison_operand19923, source_ml_source2_comparison_operand19924.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                            }
                        } }) }; ml_values518.iter().cloned().enumerate().any(|(ml_index518, ml_value518)| ml_callback518(ml_value518)) } {
                            {
                                let source_ml_source2_constructor_field19927 = Some(true);
                                self::JavaScriptCallSource::SourceCloneBindingCell(source_ml_source1_temporary_constructor_field126, source_ml_source1_temporary_constructor_field127.clone(), Some((source_ml_source2_constructor_field19927.clone(),)))
                            }
                        } else {
                            self::JavaScriptCallSource::SourceCloneBindingCell(source_ml_source1_temporary_constructor_field126, source_ml_source1_temporary_constructor_field127.clone(), None::<(Option<bool>,)>)
                        }
                    }
                }, iteration.clone(), false);
                { let ml_cell521 = at.clone(); let ml_value521 = ({ let ml_cell_value = at.borrow().clone(); ml_cell_value } - 1f64); *ml_cell521.borrow_mut() = ml_value521; self::source_ml_source1_temporary_lower_java_script_source_iteration_continuation_loop138(current.clone(), iteration_2.clone(), identities.clone(), at.clone()) }
            } }
        }
    } else {
        Some(iteration.clone())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop142(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut clauses: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>, mut source_ml_source1_temporary_values139: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceSwitchClause>>>, mut source_ml_source1_temporary_index139: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index139 < ({ let ml_reference_array522 = source_ml_source1_temporary_values139.clone(); let ml_reference_projection522 = self::ml::Big::from_u128(ml_reference_array522.borrow().len() as u128); ml_reference_projection522 })) {
            {
                let clause = { let ml_reference_array523 = source_ml_source1_temporary_values139.clone(); let ml_reference_position523 = source_ml_source1_temporary_index139.clone(); let ml_reference_projection523 = self::ml_array::at(&*ml_reference_array523.borrow(), ml_reference_position523.to_index()); ml_reference_projection523 };
                match clause.clone() {
                    self::JavaScriptSourceSwitchClause::SourceCaseStatement(value, clause_body) => {
                        let value = *value;
                        {
                            let test = self::lower_java_script_error_expression(value.clone(), current.clone(), (depth + 1f64));
                            {
                                let body = self::lower_java_script_source_statements(clause_body.clone(), 0f64, match test.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19932) => {
                                        let source_ml_source2_data_property_value19932 = *source_ml_source2_data_property_value19932;
                                        source_ml_source2_data_property_value19932.clone()
                                    }
                                }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), false);
                                {
                                    let current_2 = match body.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19933) => {
                                            let source_ml_source2_data_property_value19933 = *source_ml_source2_data_property_value19933;
                                            source_ml_source2_data_property_value19933.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_temporary_lower_java_script_source_selection_continuation_ignored143 = { let ml_method_owner524 = clauses.clone(); let ml_method_argument524_0 = self::JavaScriptCallSourceSwitchClause::SourceSwitchCase(Box::new(match test.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19934, _) => {
                                                let source_ml_source2_data_property_value19934 = *source_ml_source2_data_property_value19934;
                                                source_ml_source2_data_property_value19934.clone()
                                            }
                                        }), Box::new(match body.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19935, _) => {
                                                let source_ml_source2_data_property_value19935 = *source_ml_source2_data_property_value19935;
                                                source_ml_source2_data_property_value19935.clone()
                                            }
                                        })); let ml_inserted = vec![ml_method_argument524_0]; let mut ml_method_values524 = ml_method_owner524.borrow_mut(); let ml_length = ml_method_values524.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values524.extend(ml_inserted); ml_length as f64 };
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
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19936) => {
                                        let source_ml_source2_data_property_value19936 = *source_ml_source2_data_property_value19936;
                                        source_ml_source2_data_property_value19936.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_lower_java_script_source_selection_continuation_ignored144 = { let ml_method_owner525 = clauses.clone(); let ml_method_argument525_0 = self::JavaScriptCallSourceSwitchClause::SourceSwitchDefault(Box::new(match body_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19937, _) => {
                                            let source_ml_source2_data_property_value19937 = *source_ml_source2_data_property_value19937;
                                            source_ml_source2_data_property_value19937.clone()
                                        }
                                    })); let ml_inserted = vec![ml_method_argument525_0]; let mut ml_method_values525 = ml_method_owner525.borrow_mut(); let ml_length = ml_method_values525.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values525.extend(ml_inserted); ml_length as f64 };
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
        return if (source_ml_source1_temporary_index141 < ({ let ml_reference_array526 = source_ml_source1_temporary_values141.clone(); let ml_reference_projection526 = self::ml::Big::from_u128(ml_reference_array526.borrow().len() as u128); ml_reference_projection526 })) {
            {
                let identity = { let ml_reference_array527 = source_ml_source1_temporary_values141.clone(); let ml_reference_position527 = source_ml_source1_temporary_index141.clone(); let ml_reference_projection527 = self::ml_array::at(&*ml_reference_array527.borrow(), ml_reference_position527.to_index()); ml_reference_projection527 };
                {
                    let source_ml_source1_temporary_lower_java_script_source_selection_continuation_ignored147 = { let ml_method_owner528 = declarations.clone(); let ml_method_argument528_0 = identity; let ml_inserted = vec![ml_method_argument528_0]; let mut ml_method_values528 = ml_method_owner528.borrow_mut(); let ml_length = ml_method_values528.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values528.extend(ml_inserted); ml_length as f64 };
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
        return if (source_ml_source1_temporary_index140 < ({ let ml_reference_array529 = source_ml_source1_temporary_values140.clone(); let ml_reference_projection529 = self::ml::Big::from_u128(ml_reference_array529.borrow().len() as u128); ml_reference_projection529 })) {
            {
                let clause = { let ml_reference_array530 = source_ml_source1_temporary_values140.clone(); let ml_reference_position530 = source_ml_source1_temporary_index140.clone(); let ml_reference_projection530 = self::ml_array::at(&*ml_reference_array530.borrow(), ml_reference_position530.to_index()); ml_reference_projection530 };
                {
                    let source_ml_source1_temporary_values141 = self::collect_java_script_source_lexical_declarations(match clause.clone() {
                        self::JavaScriptSourceSwitchClause::SourceCaseStatement(_, source_ml_source2_data_property_value19942) => {
                            source_ml_source2_data_property_value19942.clone()
                        }
                        self::JavaScriptSourceSwitchClause::SourceDefaultStatement(source_ml_source2_data_property_value19943) => {
                            source_ml_source2_data_property_value19943.clone()
                        }
                    }, current.clone(), 0f64);
                    {
                        let source_ml_source1_temporary_index141 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s19944 = self::source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop146(declarations.clone(), source_ml_source1_temporary_values141.clone(), source_ml_source1_temporary_index141.clone());
                            match source_ml_source2_s19944.clone() {
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
        return if (source_ml_source1_temporary_index148 < ({ let ml_reference_array531 = source_ml_source1_temporary_values148.clone(); let ml_reference_projection531 = self::ml::Big::from_u128(ml_reference_array531.borrow().len() as u128); ml_reference_projection531 })) {
            {
                let field = { let ml_reference_array532 = source_ml_source1_temporary_values148.clone(); let ml_reference_position532 = source_ml_source1_temporary_index148.clone(); let ml_reference_projection532 = self::ml_array::at(&*ml_reference_array532.borrow(), ml_reference_position532.to_index()); ml_reference_projection532 };
                if match (match field.clone() {
                    self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, source_ml_source2_data_property_value19947, _, _) => {
                        ((source_ml_source2_data_property_value19947.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorExpression>,))).0).clone()
                    }
                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    {
                        let source_ml_source1_temporary_lower_java_script_source_object_binding_ignored150 = { let ml_method_owner533 = keys.clone(); let ml_method_argument533_0 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field19949 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(match field.clone() {
                                self::JavaScriptErrorParameterField::ErrorParameterField(_, source_ml_source2_data_property_value19948, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value19948.clone()
                                }
                            }));
                            {
                                let source_ml_source2_field19950 = (-1f64);
                                (source_ml_source2_field19950, source_ml_source2_field19949.clone())
                            }
                        })); let ml_inserted = vec![ml_method_argument533_0]; let mut ml_method_values533 = ml_method_owner533.borrow_mut(); let ml_length = ml_method_values533.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values533.extend(ml_inserted); ml_length as f64 };
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
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19951, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value19951
                            }
                        };
                        {
                            let context_2 = {
                                let source_ml_source2_nominal_field19953 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value19954, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value19954 = *source_ml_source2_data_property_value19954;
                                        source_ml_source2_data_property_value19954.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field19955 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value19956, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value19956 = *source_ml_source2_data_property_value19956;
                                            source_ml_source2_data_property_value19956.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field19957 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value19958, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19958.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field19959 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19960, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value19960
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field19961 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value19962, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value19962
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field19963 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value19964, _, _, _, _) => {
                                                            source_ml_source2_data_property_value19964.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field19965 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value19966, _, _, _) => {
                                                                source_ml_source2_data_property_value19966
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field19967 = match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value19968, _, _) => {
                                                                    source_ml_source2_data_property_value19968.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field19969 = match context.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value19970, _) => {
                                                                        source_ml_source2_data_property_value19970.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field19971 = match context.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19972) => {
                                                                            source_ml_source2_data_property_value19972.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override19973 = (identity + 1f64);
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field19953.clone()), Box::new(source_ml_source2_nominal_field19955.clone()), source_ml_source2_nominal_field19957.clone(), source_ml_source2_nominal_override19973, source_ml_source2_nominal_field19961, source_ml_source2_nominal_field19963.clone(), source_ml_source2_nominal_field19965, source_ml_source2_nominal_field19967.clone(), source_ml_source2_nominal_field19969.clone(), source_ml_source2_nominal_field19971.clone())
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
                                let source_ml_source1_temporary_lower_java_script_source_object_binding_ignored151 = { let ml_method_owner534 = keys.clone(); let ml_method_argument534_0 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field19974 = self::JavaScriptCallSource::SourceBinding(identity);
                                    (identity, source_ml_source2_field19974.clone())
                                })); let ml_inserted = vec![ml_method_argument534_0]; let mut ml_method_values534 = ml_method_owner534.borrow_mut(); let ml_length = ml_method_values534.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values534.extend(ml_inserted); ml_length as f64 };
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
                    let source_ml_source2_nullish_field_receiver19977 = (match parameter.clone() {
                        self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, source_ml_source2_data_property_value19976, _, _) => {
                            Some(source_ml_source2_data_property_value19976.clone())
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                    }, field_at);
                    {
                        let source_ml_source2_computed_property_key19978 = (source_ml_source2_nullish_field_receiver19977.clone()).1;
                        if match (source_ml_source2_nullish_field_receiver19977.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present19979 = ((source_ml_source2_nullish_field_receiver19977.clone()).0).clone().expect("a proven present value");
                                { let ml_reference_array535 = source_ml_source2_nullish_field_present19979.clone(); let ml_reference_position535 = source_ml_source2_computed_property_key19978; let ml_reference_projection535 = self::ml_array::optional_at(&*ml_reference_array535.borrow(), self::ml_array::number_index(ml_reference_position535)); ml_reference_projection535 }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source2_nullish_field_receiver19977.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
                        }
                    }
                };
                {
                    let key = { let ml_reference_array536 = keys.clone(); let ml_reference_position536 = field_at; let ml_reference_projection536 = self::ml_array::optional_at(&*ml_reference_array536.borrow(), self::ml_array::number_index(ml_reference_position536)); ml_reference_projection536 };
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
                                    self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, source_ml_source2_data_property_value19984, _) => {
                                        ((source_ml_source2_data_property_value19984.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone()
                                    }
                                };
                                {
                                    let source_ml_source2_s19987 = if !(match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let initialized = self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), context.clone(), source.clone(), mutable, (depth + 1f64), initialize_existing);
                                            {
                                                let context_2 = match initialized.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19988) => {
                                                        let source_ml_source2_data_property_value19988 = *source_ml_source2_data_property_value19988;
                                                        source_ml_source2_data_property_value19988.clone()
                                                    }
                                                };
                                                {
                                                    let source_2 = match initialized.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19989, _) => {
                                                            let source_ml_source2_data_property_value19989 = *source_ml_source2_data_property_value19989;
                                                            source_ml_source2_data_property_value19989.clone()
                                                        }
                                                    };
                                                    self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(Box::new(context_2.clone()), Box::new(source_2.clone()))
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(Box::new(context.clone()), Box::new(source.clone()))
                                    };
                                    match source_ml_source2_s19987.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(context_3, source_3) => {
                                            let context_3 = *context_3;
                                            let source_3 = *source_3;
                                            {
                                                let raw_ = self::construct_java_script_error_property_read(self::JavaScriptCallSource::SourceBinding(match parameter.clone() {
                                                    self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value19990, _, _) => {
                                                        source_ml_source2_data_property_value19990
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value19991, _) => {
                                                        source_ml_source2_data_property_value19991
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value19992, _, _) => {
                                                        source_ml_source2_data_property_value19992
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value19993, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value19993
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value19994, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value19994
                                                    }
                                                }), ({ let ml_reference_value537 = (key.clone().expect("a proven present value")).borrow().clone(); ml_reference_value537 }).1, context_3.clone());
                                                {
                                                    let source_ml_source2_s20027 = if match field.clone().expect("a proven present value") {
                                                        self::JavaScriptErrorParameterField::ErrorParameterField(_, _, source_ml_source2_data_property_value20028, _, _, _, _) => {
                                                            source_ml_source2_data_property_value20028
                                                        }
                                                    } {
                                                        {
                                                            let value = self::lower_java_script_error_expression(match field.clone().expect("a proven present value") {
                                                                self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, source_ml_source2_data_property_value20029, _, _, _) => {
                                                                    let source_ml_source2_data_property_value20029 = *source_ml_source2_data_property_value20029;
                                                                    source_ml_source2_data_property_value20029.clone()
                                                                }
                                                            }, context_3.clone(), 0f64);
                                                            {
                                                                let identity = match match value.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20030) => {
                                                                        let source_ml_source2_data_property_value20030 = *source_ml_source2_data_property_value20030;
                                                                        source_ml_source2_data_property_value20030.clone()
                                                                    }
                                                                } {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20031, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value20031
                                                                    }
                                                                };
                                                                {
                                                                    let context_4 = {
                                                                        let source_ml_source2_nominal_spread20033 = match value.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20032) => {
                                                                                let source_ml_source2_data_property_value20032 = *source_ml_source2_data_property_value20032;
                                                                                source_ml_source2_data_property_value20032.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field20034 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20035, _, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value20035 = *source_ml_source2_data_property_value20035;
                                                                                    source_ml_source2_data_property_value20035.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field20036 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20037, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value20037 = *source_ml_source2_data_property_value20037;
                                                                                        source_ml_source2_data_property_value20037.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field20038 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20039, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value20039.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20040 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20041, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value20041
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20042 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20043, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value20043
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20044 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20045, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value20045.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20046 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20047, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value20047
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20048 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20049, _, _) => {
                                                                                                                source_ml_source2_data_property_value20049.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20050 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20051, _) => {
                                                                                                                    source_ml_source2_data_property_value20051.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field20052 = match source_ml_source2_nominal_spread20033.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20053) => {
                                                                                                                        source_ml_source2_data_property_value20053.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override20054 = (identity + 1f64);
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20034.clone()), Box::new(source_ml_source2_nominal_field20036.clone()), source_ml_source2_nominal_field20038.clone(), source_ml_source2_nominal_override20054, source_ml_source2_nominal_field20042, source_ml_source2_nominal_field20044.clone(), source_ml_source2_nominal_field20046, source_ml_source2_nominal_field20048.clone(), source_ml_source2_nominal_field20050.clone(), source_ml_source2_nominal_field20052.clone())
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
                                                                                self::JavaScriptErrorParameterField::ErrorParameterField(source_ml_source2_data_property_value20055, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value20055
                                                                                }
                                                                            }, self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(saved.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)))), Box::new(match value.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20056, _) => {
                                                                                    let source_ml_source2_data_property_value20056 = *source_ml_source2_data_property_value20056;
                                                                                    source_ml_source2_data_property_value20056.clone()
                                                                                }
                                                                            }), Box::new(saved.clone())), source_3.clone(), mutable, initialize_existing, match field.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, _, source_ml_source2_data_property_value20057) => {
                                                                                    ((source_ml_source2_data_property_value20057.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone()
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
                                                                self::JavaScriptErrorParameterField::ErrorParameterField(source_ml_source2_data_property_value20058, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value20058
                                                                }
                                                            }, raw_.clone(), source_3.clone(), mutable, initialize_existing, match field.clone().expect("a proven present value") {
                                                                self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, _, source_ml_source2_data_property_value20059) => {
                                                                    ((source_ml_source2_data_property_value20059.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone()
                                                                }
                                                            });
                                                            self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154Next(Box::new(context_3.clone()), Box::new(source_5.clone()))
                                                        }
                                                    };
                                                    match source_ml_source2_s20027.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154Next(context_5, source_6) => {
                                                            let context_5 = *context_5;
                                                            let source_6 = *source_6;
                                                            {
                                                                let key_expression = match field.clone().expect("a proven present value") {
                                                                    self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, source_ml_source2_data_property_value20060, _, _) => {
                                                                        ((source_ml_source2_data_property_value20060.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorExpression>,))).0).clone()
                                                                    }
                                                                };
                                                                if !(match (key_expression.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    {
                                                                        let value_2 = self::lower_java_script_error_expression(key_expression.clone().expect("a proven present value"), context_5.clone(), 0f64);
                                                                        {
                                                                            let context_6 = match value_2.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20061) => {
                                                                                    let source_ml_source2_data_property_value20061 = *source_ml_source2_data_property_value20061;
                                                                                    source_ml_source2_data_property_value20061.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_7 = self::construct_java_script_error_local_binding(context_6.clone(), ({ let ml_reference_value538 = (key.clone().expect("a proven present value")).borrow().clone(); ml_reference_value538 }).0, self::JavaScriptCallSource::SourcePropertyKey(Box::new(match value_2.clone() {
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20062, _) => {
                                                                                        let source_ml_source2_data_property_value20062 = *source_ml_source2_data_property_value20062;
                                                                                        source_ml_source2_data_property_value20062.clone()
                                                                                    }
                                                                                }), match match context_6.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20063, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value20063 = *source_ml_source2_data_property_value20063;
                                                                                        source_ml_source2_data_property_value20063.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20064, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value20064.clone()
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
        return if (source_ml_source1_temporary_index156 < ({ let ml_reference_array539 = source_ml_source1_temporary_values156.clone(); let ml_reference_projection539 = self::ml::Big::from_u128(ml_reference_array539.borrow().len() as u128); ml_reference_projection539 })) {
            {
                let declaration = { let ml_reference_array540 = source_ml_source1_temporary_values156.clone(); let ml_reference_position540 = source_ml_source1_temporary_index156.clone(); let ml_reference_projection540 = self::ml_array::at(&*ml_reference_array540.borrow(), ml_reference_position540.to_index()); ml_reference_projection540 };
                if ({ let ml_values541 = { let ml_reference_value542 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20067) => {
                        ((source_ml_source2_data_property_value20067.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value542 }; let ml_callback541 = { let declaration = declaration.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value20070, _, _, _, _) => {
                        source_ml_source2_data_property_value20070
                    }
                }) == (match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value20071, _) => {
                        source_ml_source2_data_property_value20071
                    }
                })) }) }; ml_values541.iter().cloned().enumerate().any(|(ml_index541, ml_value541)| ml_callback541(ml_value541)) } && !(self::ml_array::includes(&{ let ml_reference_value543 = owned.borrow().clone(); ml_reference_value543 }, &match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value20072, _) => {
                        source_ml_source2_data_property_value20072
                    }
                }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                    {
                        let source_ml_source1_temporary_lower_java_script_source_parameters_ignored160 = { let ml_method_owner544 = owned.clone(); let ml_method_argument544_0 = match declaration.clone() {
                            self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value20073, _) => {
                                source_ml_source2_data_property_value20073
                            }
                        }; let ml_inserted = vec![ml_method_argument544_0]; let mut ml_method_values544 = ml_method_owner544.borrow_mut(); let ml_length = ml_method_values544.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values544.extend(ml_inserted); ml_length as f64 };
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
