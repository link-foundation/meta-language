// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptPatternBindingDeclarations;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceStatement;
use super::JavaScriptSourceSwitchClause;
use super::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233;
use super::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235;
use super::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Result;
use super::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Result;
use super::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Result;
use super::collect_java_script_pattern_binding_declarations;
use super::collect_java_script_source_lexical_declarations;
use super::collect_java_script_source_variable_declarations;
use super::ml;
use super::ml_array;
use super::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop231;
use super::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop236;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop230(mut context: self::JavaScriptErrorSourceContext, mut depth: f64, mut declarations: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values227: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, mut source_ml_source1_temporary_index227: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Result {
    loop {
        return if (source_ml_source1_temporary_index227 < ({ let ml_reference_array675 = source_ml_source1_temporary_values227.clone(); let ml_reference_projection675 = self::ml::Big::from_u128(ml_reference_array675.borrow().len() as u128); ml_reference_projection675 })) {
            {
                let statement = { let ml_reference_array676 = source_ml_source1_temporary_values227.clone(); let ml_reference_position676 = source_ml_source1_temporary_index227.clone(); let ml_reference_projection676 = self::ml_array::at(&*ml_reference_array676.borrow(), ml_reference_position676.to_index()); ml_reference_projection676 };
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
                            let source_ml_source2_s22183 = if {
                                let source_ml_source2_comparison_operand22184 = ((variable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                !(match (source_ml_source2_comparison_operand22184.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                            } {
                                {
                                    let outputs = self::collect_java_script_pattern_binding_declarations(pattern.clone(), mutable, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                    {
                                        let source_ml_source1_temporary_values229 = match outputs.clone() {
                                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22186, _) => {
                                                source_ml_source2_data_property_value22186.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index229 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s22187 = self::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop236(context.clone(), declarations.clone(), source_ml_source1_temporary_values229.clone(), source_ml_source1_temporary_index229.clone());
                                                match source_ml_source2_s22187.clone() {
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
                            match source_ml_source2_s22183.clone() {
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
                            let source_ml_source2_s22192 = if {
                                let source_ml_source2_comparison_operand22193 = ((variable_2.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                !(match (source_ml_source2_comparison_operand22193.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                            } {
                                {
                                    let outputs_2 = self::collect_java_script_pattern_binding_declarations(pattern_2.clone(), mutable_2, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                    {
                                        let source_ml_source1_temporary_values229_2 = match outputs_2.clone() {
                                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22195, _) => {
                                                source_ml_source2_data_property_value22195.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index229_2 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s22196 = self::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop236(context.clone(), declarations.clone(), source_ml_source1_temporary_values229_2.clone(), source_ml_source1_temporary_index229_2.clone());
                                                match source_ml_source2_s22196.clone() {
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
                            match source_ml_source2_s22192.clone() {
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
                            let source_ml_source2_s22201 = if { let ml_values677 = { let ml_reference_value678 = ((match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22202) => {
                                    ((source_ml_source2_data_property_value22202.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                }
                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value678 }; let ml_callback677 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { (((match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value22204, _, _, _, _) => {
                                    source_ml_source2_data_property_value22204
                                }
                            }) == identity) && !(match match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value22205, _) => {
                                    ((source_ml_source2_data_property_value22205.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                }
                            } { Some(ml_present) => ml_present, None => false })) }) }; ml_values677.iter().cloned().enumerate().any(|(ml_index677, ml_value677)| ml_callback677(ml_value677)) } {
                                {
                                    let source_ml_source1_temporary_collect_java_script_source_lexical_declarations_ignored234 = { let ml_method_owner679 = declarations.clone(); let ml_method_argument679_0 = identity; let ml_inserted = vec![ml_method_argument679_0]; let mut ml_method_values679 = ml_method_owner679.borrow_mut(); let ml_length = ml_method_values679.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values679.extend(ml_inserted); ml_length as f64 };
                                    self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next
                                }
                            } else {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next
                            };
                            match source_ml_source2_s22201.clone() {
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
                            let source_ml_source2_s22210 = if { let ml_values680 = { let ml_reference_value681 = ((match context.clone() {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22211) => {
                                    ((source_ml_source2_data_property_value22211.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                                }
                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value681 }; let ml_callback680 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { (((match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value22213, _, _, _, _) => {
                                    source_ml_source2_data_property_value22213
                                }
                            }) == identity_2) && !(match match row.clone() {
                                self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value22214, _) => {
                                    ((source_ml_source2_data_property_value22214.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                }
                            } { Some(ml_present) => ml_present, None => false })) }) }; ml_values680.iter().cloned().enumerate().any(|(ml_index680, ml_value680)| ml_callback680(ml_value680)) } {
                                {
                                    let source_ml_source1_temporary_collect_java_script_source_lexical_declarations_ignored234_2 = { let ml_method_owner682 = declarations.clone(); let ml_method_argument682_0 = identity_2; let ml_inserted = vec![ml_method_argument682_0]; let mut ml_method_values682 = ml_method_owner682.borrow_mut(); let ml_length = ml_method_values682.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values682.extend(ml_inserted); ml_length as f64 };
                                    self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next
                                }
                            } else {
                                self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next
                            };
                            match source_ml_source2_s22210.clone() {
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
                                    let source_ml_source2_s22215 = self::source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop231(declarations.clone(), source_ml_source1_temporary_values228.clone(), source_ml_source1_temporary_index228.clone());
                                    match source_ml_source2_s22215.clone() {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop241Result {
    MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop241Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop243Result {
    MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop243Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop243(mut context: self::JavaScriptErrorSourceContext, mut identities: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values240: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index240: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop243Result {
    loop {
        return if (source_ml_source1_temporary_index240 < ({ let ml_reference_array683 = source_ml_source1_temporary_values240.clone(); let ml_reference_projection683 = self::ml::Big::from_u128(ml_reference_array683.borrow().len() as u128); ml_reference_projection683 })) {
            {
                let identity = { let ml_reference_array684 = source_ml_source1_temporary_values240.clone(); let ml_reference_position684 = source_ml_source1_temporary_index240.clone(); let ml_reference_projection684 = self::ml_array::at(&*ml_reference_array684.borrow(), ml_reference_position684.to_index()); ml_reference_projection684 };
                if ({ let ml_values685 = { let ml_reference_value686 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22218) => {
                        ((source_ml_source2_data_property_value22218.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value686 }; let ml_callback685 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { (((match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value22220, _, _, _, _) => {
                        source_ml_source2_data_property_value22220
                    }
                }) == identity) && {
                    let source_ml_source2_comparison_operand22222 = match row.clone() {
                        self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value22221, _) => {
                            ((source_ml_source2_data_property_value22221.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand22223 = true;
                        match (source_ml_source2_comparison_operand22222.clone(), source_ml_source2_comparison_operand22223) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    }
                }) }) }; ml_values685.iter().cloned().enumerate().any(|(ml_index685, ml_value685)| ml_callback685(ml_value685)) } && !(self::ml_array::includes(&{ let ml_reference_value687 = identities.borrow().clone(); ml_reference_value687 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                    {
                        let source_ml_source1_temporary_collect_java_script_source_variable_declarations_ignored244 = { let ml_method_owner688 = identities.clone(); let ml_method_argument688_0 = identity; let ml_inserted = vec![ml_method_argument688_0]; let mut ml_method_values688 = ml_method_owner688.borrow_mut(); let ml_length = ml_method_values688.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values688.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_temporary_index240_2 = source_ml_source1_temporary_index240.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index240 = source_ml_source1_temporary_index240_2.clone();
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index240_3 = source_ml_source1_temporary_index240.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index240 = source_ml_source1_temporary_index240_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop243Result::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop243Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242 {
    MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(std::rc::Rc<std::cell::RefCell<Vec<f64>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop245(mut children: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, mut source_ml_source1_temporary_values239: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceSwitchClause>>>, mut source_ml_source1_temporary_index239: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>> {
    loop {
        return if (source_ml_source1_temporary_index239 < ({ let ml_reference_array689 = source_ml_source1_temporary_values239.clone(); let ml_reference_projection689 = self::ml::Big::from_u128(ml_reference_array689.borrow().len() as u128); ml_reference_projection689 })) {
            {
                let clause = { let ml_reference_array690 = source_ml_source1_temporary_values239.clone(); let ml_reference_position690 = source_ml_source1_temporary_index239.clone(); let ml_reference_projection690 = self::ml_array::at(&*ml_reference_array690.borrow(), ml_reference_position690.to_index()); ml_reference_projection690 };
                {
                    let children_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value22229 = match clause.clone() {
                            self::JavaScriptSourceSwitchClause::SourceCaseStatement(_, source_ml_source2_data_property_value22226) => {
                                source_ml_source2_data_property_value22226.clone()
                            }
                            self::JavaScriptSourceSwitchClause::SourceDefaultStatement(source_ml_source2_data_property_value22227) => {
                                source_ml_source2_data_property_value22227.clone()
                            }
                        };
                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSourceStatement>::new(), { let ml_reference_value691 = children.borrow().clone(); ml_reference_value691 }), { let ml_reference_value692 = source_ml_source2_concat_value22229.borrow().clone(); ml_reference_value692 })
                    }));
                    {
                        let source_ml_source1_temporary_index239_2 = source_ml_source1_temporary_index239.add(&self::ml::Big::from_i128(1));
                        {
                            (children, source_ml_source1_temporary_index239) = (children_2.clone(), source_ml_source1_temporary_index239_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            children.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop241(mut context: self::JavaScriptErrorSourceContext, mut depth: f64, mut identities: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values238: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceStatement>>>, mut source_ml_source1_temporary_index238: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop241Result {
    loop {
        return if (source_ml_source1_temporary_index238 < ({ let ml_reference_array693 = source_ml_source1_temporary_values238.clone(); let ml_reference_projection693 = self::ml::Big::from_u128(ml_reference_array693.borrow().len() as u128); ml_reference_projection693 })) {
            {
                let statement = { let ml_reference_array694 = source_ml_source1_temporary_values238.clone(); let ml_reference_position694 = source_ml_source1_temporary_index238.clone(); let ml_reference_projection694 = self::ml_array::at(&*ml_reference_array694.borrow(), ml_reference_position694.to_index()); ml_reference_projection694 };
                {
                    let candidates = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let children = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceStatement>::new()));
                        {
                            let source_ml_source2_s22283 = match statement.clone() {
                                self::JavaScriptSourceStatement::SourceForOfStatement(_, binding, _, variable, _, _, _, body, statement_pattern) => {
                                    {
                                        let pattern = ((statement_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                                        {
                                            let candidates_3 = if {
                                                let source_ml_source2_comparison_operand22317 = ((variable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                match (source_ml_source2_comparison_operand22317.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                            } {
                                                {
                                                    let candidates_2 = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![binding]))
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({ let ml_values695 = { let ml_reference_value696 = (match self::collect_java_script_pattern_binding_declarations(pattern.clone().expect("a proven present value"), true, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))) {
                                                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22319, _) => {
                                                                source_ml_source2_data_property_value22319.clone()
                                                            }
                                                        }).borrow().clone(); ml_reference_value696 }; let ml_callback695 = std::rc::Rc::new(move |row: self::JavaScriptCallBindingDeclaration| -> f64 { match row.clone() {
                                                            self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22320, _) => {
                                                                source_ml_source2_data_property_value22320
                                                            }
                                                        } }); ml_values695.iter().cloned().enumerate().map(|(ml_index695, ml_value695)| ml_callback695(ml_value695)).collect::<Vec<_>>() }))
                                                    };
                                                    candidates_2.clone()
                                                }
                                            } else {
                                                candidates.clone()
                                            };
                                            self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates_3.clone(), body.clone())
                                        }
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceFinallyStatement(body_2, finalizer) => {
                                    {
                                        let children_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSourceStatement>::new(), { let ml_reference_value697 = body_2.borrow().clone(); ml_reference_value697 }), { let ml_reference_value698 = finalizer.borrow().clone(); ml_reference_value698 })));
                                        self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_2.clone())
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceTryStatement(_, body_3, handler, _) => {
                                    {
                                        let children_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSourceStatement>::new(), { let ml_reference_value699 = body_3.borrow().clone(); ml_reference_value699 }), { let ml_reference_value700 = handler.borrow().clone(); ml_reference_value700 })));
                                        self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_3.clone())
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceArrayBindingStatement(pattern_2, _, _, variable_2) => {
                                    let pattern_2 = *pattern_2;
                                    {
                                        let candidates_5 = if {
                                            let source_ml_source2_comparison_operand22325 = ((variable_2.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                            match (source_ml_source2_comparison_operand22325.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        } {
                                            {
                                                let candidates_4 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values701 = { let ml_reference_value702 = (match self::collect_java_script_pattern_binding_declarations(pattern_2.clone(), true, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))) {
                                                    self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22327, _) => {
                                                        source_ml_source2_data_property_value22327.clone()
                                                    }
                                                }).borrow().clone(); ml_reference_value702 }; let ml_callback701 = std::rc::Rc::new(move |row: self::JavaScriptCallBindingDeclaration| -> f64 { match row.clone() {
                                                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22328, _) => {
                                                        source_ml_source2_data_property_value22328
                                                    }
                                                } }); ml_values701.iter().cloned().enumerate().map(|(ml_index701, ml_value701)| ml_callback701(ml_value701)).collect::<Vec<_>>() }));
                                                candidates_4.clone()
                                            }
                                        } else {
                                            candidates.clone()
                                        };
                                        self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates_5.clone(), children.clone())
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceObjectBindingStatement(pattern_3, _, _, variable_3) => {
                                    let pattern_3 = *pattern_3;
                                    {
                                        let candidates_7 = if {
                                            let source_ml_source2_comparison_operand22329 = ((variable_3.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                            match (source_ml_source2_comparison_operand22329.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        } {
                                            {
                                                let candidates_6 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values703 = { let ml_reference_value704 = (match self::collect_java_script_pattern_binding_declarations(pattern_3.clone(), true, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))) {
                                                    self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22331, _) => {
                                                        source_ml_source2_data_property_value22331.clone()
                                                    }
                                                }).borrow().clone(); ml_reference_value704 }; let ml_callback703 = std::rc::Rc::new(move |row: self::JavaScriptCallBindingDeclaration| -> f64 { match row.clone() {
                                                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22332, _) => {
                                                        source_ml_source2_data_property_value22332
                                                    }
                                                } }); ml_values703.iter().cloned().enumerate().map(|(ml_index703, ml_value703)| ml_callback703(ml_value703)).collect::<Vec<_>>() }));
                                                candidates_6.clone()
                                            }
                                        } else {
                                            candidates.clone()
                                        };
                                        self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates_7.clone(), children.clone())
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceVariableStatement(identity, _, _, _) => {
                                    {
                                        let candidates_8 = std::rc::Rc::new(std::cell::RefCell::new(vec![identity]));
                                        self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates_8.clone(), children.clone())
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceConstStatement(_, _) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children.clone())
                                }
                                self::JavaScriptSourceStatement::SourceMutableStatement(_, _, _) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children.clone())
                                }
                                self::JavaScriptSourceStatement::SourceExpressionStatement(_) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children.clone())
                                }
                                self::JavaScriptSourceStatement::SourceReturnStatement(_) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children.clone())
                                }
                                self::JavaScriptSourceStatement::SourceThrowStatement(_) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children.clone())
                                }
                                self::JavaScriptSourceStatement::SourceIfStatement(_, yes, no) => {
                                    {
                                        let children_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSourceStatement>::new(), { let ml_reference_value705 = yes.borrow().clone(); ml_reference_value705 }), { let ml_reference_value706 = no.borrow().clone(); ml_reference_value706 })));
                                        self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_4.clone())
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceBlockStatement(children_5) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_5.clone())
                                }
                                self::JavaScriptSourceStatement::SourceDeclarationGroupStatement(children_6) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_6.clone())
                                }
                                self::JavaScriptSourceStatement::SourceWhileStatement(_, children_7, _, _, _, _) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_7.clone())
                                }
                                self::JavaScriptSourceStatement::SourceControlBlockStatement(_, children_8) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_8.clone())
                                }
                                self::JavaScriptSourceStatement::SourceControlJumpStatement(_, _) => {
                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children.clone())
                                }
                                self::JavaScriptSourceStatement::SourceSwitchStatement(_, _, clauses) => {
                                    {
                                        let source_ml_source1_temporary_index239 = self::ml::Big::from_i128(0);
                                        {
                                            let children_9 = self::source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop245(children.clone(), clauses.clone(), source_ml_source1_temporary_index239.clone());
                                            self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_9.clone())
                                        }
                                    }
                                }
                            };
                            match source_ml_source2_s22283.clone() {
                                self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates_9, children_10) => {
                                    {
                                        let candidates_10 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value22336 = self::collect_java_script_source_variable_declarations(children_10.clone(), context.clone(), (depth + 1f64));
                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value707 = candidates_9.borrow().clone(); ml_reference_value707 }), { let ml_reference_value708 = source_ml_source2_concat_value22336.borrow().clone(); ml_reference_value708 })
                                        }));
                                        {
                                            let source_ml_source1_temporary_index240 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s22337 = self::source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop243(context.clone(), identities.clone(), candidates_10.clone(), source_ml_source1_temporary_index240.clone());
                                                match source_ml_source2_s22337.clone() {
                                                    self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop243Result::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop243Done => {
                                                        {
                                                            let source_ml_source1_temporary_index238_2 = source_ml_source1_temporary_index238.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                source_ml_source1_temporary_index238 = source_ml_source1_temporary_index238_2.clone();
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
        } else {
            self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop241Result::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsLoop241Done
        };
    }
}
