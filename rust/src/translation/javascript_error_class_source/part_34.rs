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
                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20013, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value20013
                }
            };
            { let ml_cell516 = current.clone(); let ml_value516 = {
                let source_ml_source2_nominal_spread20014 = { let ml_cell_value = current.borrow().clone(); ml_cell_value };
                {
                    let source_ml_source2_nominal_field20015 = match source_ml_source2_nominal_spread20014.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20016, _, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value20016 = *source_ml_source2_data_property_value20016;
                            source_ml_source2_data_property_value20016.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field20017 = match source_ml_source2_nominal_spread20014.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20018, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value20018 = *source_ml_source2_data_property_value20018;
                                source_ml_source2_data_property_value20018.clone()
                            }
                        };
                        {
                            let source_ml_source2_nominal_field20019 = match source_ml_source2_nominal_spread20014.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20020, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value20020.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field20021 = match source_ml_source2_nominal_spread20014.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20022, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value20022
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field20023 = match source_ml_source2_nominal_spread20014.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20024, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value20024
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field20025 = match source_ml_source2_nominal_spread20014.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20026, _, _, _, _) => {
                                                source_ml_source2_data_property_value20026.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field20027 = match source_ml_source2_nominal_spread20014.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20028, _, _, _) => {
                                                    source_ml_source2_data_property_value20028
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field20029 = match source_ml_source2_nominal_spread20014.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20030, _, _) => {
                                                        source_ml_source2_data_property_value20030.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field20031 = match source_ml_source2_nominal_spread20014.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20032, _) => {
                                                            source_ml_source2_data_property_value20032.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field20033 = match source_ml_source2_nominal_spread20014.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20034) => {
                                                                source_ml_source2_data_property_value20034.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override20035 = (temporary + 1f64);
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20015.clone()), Box::new(source_ml_source2_nominal_field20017.clone()), source_ml_source2_nominal_field20019.clone(), source_ml_source2_nominal_override20035, source_ml_source2_nominal_field20023, source_ml_source2_nominal_field20025.clone(), source_ml_source2_nominal_field20027, source_ml_source2_nominal_field20029.clone(), source_ml_source2_nominal_field20031.clone(), source_ml_source2_nominal_field20033.clone())
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
                        let source_ml_source2_array_read_index20037 = { let ml_cell_value = at.borrow().clone(); ml_cell_value };
                        { let ml_reference_array517 = identities.clone(); let ml_reference_position517 = source_ml_source2_array_read_index20037; let ml_reference_projection517 = self::ml_array::optional_at(&*ml_reference_array517.borrow(), self::ml_array::number_index(ml_reference_position517)); ml_reference_projection517 }
                    }).clone().unwrap_or_else(|| (-1f64));
                    {
                        let source_ml_source1_temporary_constructor_field127 = match match { let ml_cell_value = current.borrow().clone(); ml_cell_value } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20038, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value20038 = *source_ml_source2_data_property_value20038;
                                source_ml_source2_data_property_value20038.clone()
                            }
                        } {
                            self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20039, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value20039.clone()
                            }
                        };
                        if { let ml_values518 = { let ml_reference_value519 = ((match { let ml_cell_value = current.borrow().clone(); ml_cell_value } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20040) => {
                                ((source_ml_source2_data_property_value20040.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                            }
                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value519 }; let ml_callback518 = { let identities = identities.clone(); let at = at.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { {
                            let source_ml_source2_comparison_operand20044 = match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value20041, _, _, _, _) => {
                                    source_ml_source2_data_property_value20041
                                }
                            };
                            {
                                let source_ml_source2_comparison_operand20045 = {
                                    let source_ml_source2_array_read_value20042 = identities.clone();
                                    {
                                        let source_ml_source2_array_read_index20043 = { let ml_cell_value = at.borrow().clone(); ml_cell_value };
                                        { let ml_reference_array520 = source_ml_source2_array_read_value20042.clone(); let ml_reference_position520 = source_ml_source2_array_read_index20043; let ml_reference_projection520 = self::ml_array::optional_at(&*ml_reference_array520.borrow(), self::ml_array::number_index(ml_reference_position520)); ml_reference_projection520 }
                                    }
                                };
                                match (source_ml_source2_comparison_operand20044, source_ml_source2_comparison_operand20045.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                            }
                        } }) }; ml_values518.iter().cloned().enumerate().any(|(ml_index518, ml_value518)| ml_callback518(ml_value518)) } {
                            {
                                let source_ml_source2_constructor_field20048 = Some(true);
                                self::JavaScriptCallSource::SourceCloneBindingCell(source_ml_source1_temporary_constructor_field126, source_ml_source1_temporary_constructor_field127.clone(), Some((source_ml_source2_constructor_field20048.clone(),)))
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
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20053) => {
                                        let source_ml_source2_data_property_value20053 = *source_ml_source2_data_property_value20053;
                                        source_ml_source2_data_property_value20053.clone()
                                    }
                                }, self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined), (depth + 1f64), false);
                                {
                                    let current_2 = match body.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20054) => {
                                            let source_ml_source2_data_property_value20054 = *source_ml_source2_data_property_value20054;
                                            source_ml_source2_data_property_value20054.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_temporary_lower_java_script_source_selection_continuation_ignored143 = { let ml_method_owner524 = clauses.clone(); let ml_method_argument524_0 = self::JavaScriptCallSourceSwitchClause::SourceSwitchCase(Box::new(match test.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20055, _) => {
                                                let source_ml_source2_data_property_value20055 = *source_ml_source2_data_property_value20055;
                                                source_ml_source2_data_property_value20055.clone()
                                            }
                                        }), Box::new(match body.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20056, _) => {
                                                let source_ml_source2_data_property_value20056 = *source_ml_source2_data_property_value20056;
                                                source_ml_source2_data_property_value20056.clone()
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
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20057) => {
                                        let source_ml_source2_data_property_value20057 = *source_ml_source2_data_property_value20057;
                                        source_ml_source2_data_property_value20057.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_lower_java_script_source_selection_continuation_ignored144 = { let ml_method_owner525 = clauses.clone(); let ml_method_argument525_0 = self::JavaScriptCallSourceSwitchClause::SourceSwitchDefault(Box::new(match body_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20058, _) => {
                                            let source_ml_source2_data_property_value20058 = *source_ml_source2_data_property_value20058;
                                            source_ml_source2_data_property_value20058.clone()
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
                        self::JavaScriptSourceSwitchClause::SourceCaseStatement(_, source_ml_source2_data_property_value20063) => {
                            source_ml_source2_data_property_value20063.clone()
                        }
                        self::JavaScriptSourceSwitchClause::SourceDefaultStatement(source_ml_source2_data_property_value20064) => {
                            source_ml_source2_data_property_value20064.clone()
                        }
                    }, current.clone(), 0f64);
                    {
                        let source_ml_source1_temporary_index141 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s20065 = self::source_ml_source1_temporary_lower_java_script_source_selection_continuation_loop146(declarations.clone(), source_ml_source1_temporary_values141.clone(), source_ml_source1_temporary_index141.clone());
                            match source_ml_source2_s20065.clone() {
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
                    self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, source_ml_source2_data_property_value20068, _, _) => {
                        ((source_ml_source2_data_property_value20068.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorExpression>,))).0).clone()
                    }
                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    {
                        let source_ml_source1_temporary_lower_java_script_source_object_binding_ignored150 = { let ml_method_owner533 = keys.clone(); let ml_method_argument533_0 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field20070 = self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(match field.clone() {
                                self::JavaScriptErrorParameterField::ErrorParameterField(_, source_ml_source2_data_property_value20069, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value20069.clone()
                                }
                            }));
                            {
                                let source_ml_source2_field20071 = (-1f64);
                                (source_ml_source2_field20071, source_ml_source2_field20070.clone())
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
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20072, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value20072
                            }
                        };
                        {
                            let context_2 = {
                                let source_ml_source2_nominal_field20074 = match context.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20075, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value20075 = *source_ml_source2_data_property_value20075;
                                        source_ml_source2_data_property_value20075.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field20076 = match context.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20077, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value20077 = *source_ml_source2_data_property_value20077;
                                            source_ml_source2_data_property_value20077.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field20078 = match context.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20079, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value20079.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field20080 = match context.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20081, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value20081
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field20082 = match context.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20083, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value20083
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field20084 = match context.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20085, _, _, _, _) => {
                                                            source_ml_source2_data_property_value20085.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field20086 = match context.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20087, _, _, _) => {
                                                                source_ml_source2_data_property_value20087
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field20088 = match context.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20089, _, _) => {
                                                                    source_ml_source2_data_property_value20089.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field20090 = match context.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20091, _) => {
                                                                        source_ml_source2_data_property_value20091.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field20092 = match context.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20093) => {
                                                                            source_ml_source2_data_property_value20093.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override20094 = (identity + 1f64);
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20074.clone()), Box::new(source_ml_source2_nominal_field20076.clone()), source_ml_source2_nominal_field20078.clone(), source_ml_source2_nominal_override20094, source_ml_source2_nominal_field20082, source_ml_source2_nominal_field20084.clone(), source_ml_source2_nominal_field20086, source_ml_source2_nominal_field20088.clone(), source_ml_source2_nominal_field20090.clone(), source_ml_source2_nominal_field20092.clone())
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
                                    let source_ml_source2_field20095 = self::JavaScriptCallSource::SourceBinding(identity);
                                    (identity, source_ml_source2_field20095.clone())
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
                    let source_ml_source2_nullish_field_receiver20098 = (match parameter.clone() {
                        self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                        self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                        self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                        self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, source_ml_source2_data_property_value20097, _, _) => {
                            Some(source_ml_source2_data_property_value20097.clone())
                        }
                        self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, _, _) => {
                            None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>>
                        }
                    }, field_at);
                    {
                        let source_ml_source2_computed_property_key20099 = (source_ml_source2_nullish_field_receiver20098.clone()).1;
                        if match (source_ml_source2_nullish_field_receiver20098.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present20100 = ((source_ml_source2_nullish_field_receiver20098.clone()).0).clone().expect("a proven present value");
                                { let ml_reference_array535 = source_ml_source2_nullish_field_present20100.clone(); let ml_reference_position535 = source_ml_source2_computed_property_key20099; let ml_reference_projection535 = self::ml_array::optional_at(&*ml_reference_array535.borrow(), self::ml_array::number_index(ml_reference_position535)); ml_reference_projection535 }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source2_nullish_field_receiver20098.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
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
                                    self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, source_ml_source2_data_property_value20105, _) => {
                                        ((source_ml_source2_data_property_value20105.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone()
                                    }
                                };
                                {
                                    let source_ml_source2_s20108 = if !(match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let initialized = self::lower_java_script_source_binding_pattern(pattern.clone().expect("a proven present value"), context.clone(), source.clone(), mutable, (depth + 1f64), initialize_existing);
                                            {
                                                let context_2 = match initialized.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20109) => {
                                                        let source_ml_source2_data_property_value20109 = *source_ml_source2_data_property_value20109;
                                                        source_ml_source2_data_property_value20109.clone()
                                                    }
                                                };
                                                {
                                                    let source_2 = match initialized.clone() {
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20110, _) => {
                                                            let source_ml_source2_data_property_value20110 = *source_ml_source2_data_property_value20110;
                                                            source_ml_source2_data_property_value20110.clone()
                                                        }
                                                    };
                                                    self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(Box::new(context_2.clone()), Box::new(source_2.clone()))
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(Box::new(context.clone()), Box::new(source.clone()))
                                    };
                                    match source_ml_source2_s20108.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin153Next(context_3, source_3) => {
                                            let context_3 = *context_3;
                                            let source_3 = *source_3;
                                            {
                                                let raw_ = self::construct_java_script_error_property_read(self::JavaScriptCallSource::SourceBinding(match parameter.clone() {
                                                    self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value20111, _, _) => {
                                                        source_ml_source2_data_property_value20111
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value20112, _) => {
                                                        source_ml_source2_data_property_value20112
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value20113, _, _) => {
                                                        source_ml_source2_data_property_value20113
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value20114, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value20114
                                                    }
                                                    self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value20115, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value20115
                                                    }
                                                }), ({ let ml_reference_value537 = (key.clone().expect("a proven present value")).borrow().clone(); ml_reference_value537 }).1, context_3.clone());
                                                {
                                                    let source_ml_source2_s20148 = if match field.clone().expect("a proven present value") {
                                                        self::JavaScriptErrorParameterField::ErrorParameterField(_, _, source_ml_source2_data_property_value20149, _, _, _, _) => {
                                                            source_ml_source2_data_property_value20149
                                                        }
                                                    } {
                                                        {
                                                            let value = self::lower_java_script_error_expression(match field.clone().expect("a proven present value") {
                                                                self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, source_ml_source2_data_property_value20150, _, _, _) => {
                                                                    let source_ml_source2_data_property_value20150 = *source_ml_source2_data_property_value20150;
                                                                    source_ml_source2_data_property_value20150.clone()
                                                                }
                                                            }, context_3.clone(), 0f64);
                                                            {
                                                                let identity = match match value.clone() {
                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20151) => {
                                                                        let source_ml_source2_data_property_value20151 = *source_ml_source2_data_property_value20151;
                                                                        source_ml_source2_data_property_value20151.clone()
                                                                    }
                                                                } {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20152, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value20152
                                                                    }
                                                                };
                                                                {
                                                                    let context_4 = {
                                                                        let source_ml_source2_nominal_spread20154 = match value.clone() {
                                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20153) => {
                                                                                let source_ml_source2_data_property_value20153 = *source_ml_source2_data_property_value20153;
                                                                                source_ml_source2_data_property_value20153.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field20155 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value20156, _, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value20156 = *source_ml_source2_data_property_value20156;
                                                                                    source_ml_source2_data_property_value20156.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field20157 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20158, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value20158 = *source_ml_source2_data_property_value20158;
                                                                                        source_ml_source2_data_property_value20158.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field20159 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value20160, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value20160.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20161 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value20162, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value20162
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20163 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value20164, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value20164
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20165 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value20166, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value20166.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field20167 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value20168, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value20168
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field20169 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value20170, _, _) => {
                                                                                                                source_ml_source2_data_property_value20170.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field20171 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20172, _) => {
                                                                                                                    source_ml_source2_data_property_value20172.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field20173 = match source_ml_source2_nominal_spread20154.clone() {
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20174) => {
                                                                                                                        source_ml_source2_data_property_value20174.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override20175 = (identity + 1f64);
                                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field20155.clone()), Box::new(source_ml_source2_nominal_field20157.clone()), source_ml_source2_nominal_field20159.clone(), source_ml_source2_nominal_override20175, source_ml_source2_nominal_field20163, source_ml_source2_nominal_field20165.clone(), source_ml_source2_nominal_field20167, source_ml_source2_nominal_field20169.clone(), source_ml_source2_nominal_field20171.clone(), source_ml_source2_nominal_field20173.clone())
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
                                                                                self::JavaScriptErrorParameterField::ErrorParameterField(source_ml_source2_data_property_value20176, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value20176
                                                                                }
                                                                            }, self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(saved.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)))), Box::new(match value.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20177, _) => {
                                                                                    let source_ml_source2_data_property_value20177 = *source_ml_source2_data_property_value20177;
                                                                                    source_ml_source2_data_property_value20177.clone()
                                                                                }
                                                                            }), Box::new(saved.clone())), source_3.clone(), mutable, initialize_existing, match field.clone().expect("a proven present value") {
                                                                                self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, _, source_ml_source2_data_property_value20178) => {
                                                                                    ((source_ml_source2_data_property_value20178.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone()
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
                                                                self::JavaScriptErrorParameterField::ErrorParameterField(source_ml_source2_data_property_value20179, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value20179
                                                                }
                                                            }, raw_.clone(), source_3.clone(), mutable, initialize_existing, match field.clone().expect("a proven present value") {
                                                                self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, _, source_ml_source2_data_property_value20180) => {
                                                                    ((source_ml_source2_data_property_value20180.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone()
                                                                }
                                                            });
                                                            self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154Next(Box::new(context_3.clone()), Box::new(source_5.clone()))
                                                        }
                                                    };
                                                    match source_ml_source2_s20148.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154::MlSource1TemporaryLowerJavaScriptSourceObjectBindingJoin154Next(context_5, source_6) => {
                                                            let context_5 = *context_5;
                                                            let source_6 = *source_6;
                                                            {
                                                                let key_expression = match field.clone().expect("a proven present value") {
                                                                    self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, source_ml_source2_data_property_value20181, _, _) => {
                                                                        ((source_ml_source2_data_property_value20181.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorExpression>,))).0).clone()
                                                                    }
                                                                };
                                                                if !(match (key_expression.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    {
                                                                        let value_2 = self::lower_java_script_error_expression(key_expression.clone().expect("a proven present value"), context_5.clone(), 0f64);
                                                                        {
                                                                            let context_6 = match value_2.clone() {
                                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value20182) => {
                                                                                    let source_ml_source2_data_property_value20182 = *source_ml_source2_data_property_value20182;
                                                                                    source_ml_source2_data_property_value20182.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_7 = self::construct_java_script_error_local_binding(context_6.clone(), ({ let ml_reference_value538 = (key.clone().expect("a proven present value")).borrow().clone(); ml_reference_value538 }).0, self::JavaScriptCallSource::SourcePropertyKey(Box::new(match value_2.clone() {
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value20183, _) => {
                                                                                        let source_ml_source2_data_property_value20183 = *source_ml_source2_data_property_value20183;
                                                                                        source_ml_source2_data_property_value20183.clone()
                                                                                    }
                                                                                }), match match context_6.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value20184, _, _, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value20184 = *source_ml_source2_data_property_value20184;
                                                                                        source_ml_source2_data_property_value20184.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value20185, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value20185.clone()
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
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20188) => {
                        ((source_ml_source2_data_property_value20188.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value542 }; let ml_callback541 = { let declaration = declaration.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { ((match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value20191, _, _, _, _) => {
                        source_ml_source2_data_property_value20191
                    }
                }) == (match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value20192, _) => {
                        source_ml_source2_data_property_value20192
                    }
                })) }) }; ml_values541.iter().cloned().enumerate().any(|(ml_index541, ml_value541)| ml_callback541(ml_value541)) } && !(self::ml_array::includes(&{ let ml_reference_value543 = owned.borrow().clone(); ml_reference_value543 }, &match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value20193, _) => {
                        source_ml_source2_data_property_value20193
                    }
                }, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                    {
                        let source_ml_source1_temporary_lower_java_script_source_parameters_ignored160 = { let ml_method_owner544 = owned.clone(); let ml_method_argument544_0 = match declaration.clone() {
                            self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value20194, _) => {
                                source_ml_source2_data_property_value20194
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
