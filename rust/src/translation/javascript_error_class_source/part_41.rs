// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptPatternBindingDeclarations;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceStatement;
use super::JavaScriptSourceSwitchClause;
use super::collect_java_script_pattern_binding_declarations;
use super::collect_java_script_source_variable_declarations;
use super::ml;
use super::ml_array;

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
        return if (source_ml_source1_temporary_index240 < ({ let ml_reference_array686 = source_ml_source1_temporary_values240.clone(); let ml_reference_projection686 = self::ml::Big::from_u128(ml_reference_array686.borrow().len() as u128); ml_reference_projection686 })) {
            {
                let identity = { let ml_reference_array687 = source_ml_source1_temporary_values240.clone(); let ml_reference_position687 = source_ml_source1_temporary_index240.clone(); let ml_reference_projection687 = self::ml_array::at(&*ml_reference_array687.borrow(), ml_reference_position687.to_index()); ml_reference_projection687 };
                if ({ let ml_values688 = { let ml_reference_value689 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22476) => {
                        ((source_ml_source2_data_property_value22476.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value689 }; let ml_callback688 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptSourceLexicalBinding| -> bool { (((match row.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value22478, _, _, _, _) => {
                        source_ml_source2_data_property_value22478
                    }
                }) == identity) && {
                    let source_ml_source2_comparison_operand22480 = match row.clone() {
                        self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(_, _, _, source_ml_source2_data_property_value22479, _) => {
                            ((source_ml_source2_data_property_value22479.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand22481 = true;
                        match (source_ml_source2_comparison_operand22480.clone(), source_ml_source2_comparison_operand22481) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    }
                }) }) }; ml_values688.iter().cloned().enumerate().any(|(ml_index688, ml_value688)| ml_callback688(ml_value688)) } && !(self::ml_array::includes(&{ let ml_reference_value690 = identities.borrow().clone(); ml_reference_value690 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                    {
                        let source_ml_source1_temporary_collect_java_script_source_variable_declarations_ignored244 = { let ml_method_owner691 = identities.clone(); let ml_method_argument691_0 = identity; let ml_inserted = vec![ml_method_argument691_0]; let mut ml_method_values691 = ml_method_owner691.borrow_mut(); let ml_length = ml_method_values691.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values691.extend(ml_inserted); ml_length as f64 };
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
        return if (source_ml_source1_temporary_index239 < ({ let ml_reference_array692 = source_ml_source1_temporary_values239.clone(); let ml_reference_projection692 = self::ml::Big::from_u128(ml_reference_array692.borrow().len() as u128); ml_reference_projection692 })) {
            {
                let clause = { let ml_reference_array693 = source_ml_source1_temporary_values239.clone(); let ml_reference_position693 = source_ml_source1_temporary_index239.clone(); let ml_reference_projection693 = self::ml_array::at(&*ml_reference_array693.borrow(), ml_reference_position693.to_index()); ml_reference_projection693 };
                {
                    let children_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value22487 = match clause.clone() {
                            self::JavaScriptSourceSwitchClause::SourceCaseStatement(_, source_ml_source2_data_property_value22484) => {
                                source_ml_source2_data_property_value22484.clone()
                            }
                            self::JavaScriptSourceSwitchClause::SourceDefaultStatement(source_ml_source2_data_property_value22485) => {
                                source_ml_source2_data_property_value22485.clone()
                            }
                        };
                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSourceStatement>::new(), { let ml_reference_value694 = children.borrow().clone(); ml_reference_value694 }), { let ml_reference_value695 = source_ml_source2_concat_value22487.borrow().clone(); ml_reference_value695 })
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
        return if (source_ml_source1_temporary_index238 < ({ let ml_reference_array696 = source_ml_source1_temporary_values238.clone(); let ml_reference_projection696 = self::ml::Big::from_u128(ml_reference_array696.borrow().len() as u128); ml_reference_projection696 })) {
            {
                let statement = { let ml_reference_array697 = source_ml_source1_temporary_values238.clone(); let ml_reference_position697 = source_ml_source1_temporary_index238.clone(); let ml_reference_projection697 = self::ml_array::at(&*ml_reference_array697.borrow(), ml_reference_position697.to_index()); ml_reference_projection697 };
                {
                    let candidates = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let children = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceStatement>::new()));
                        {
                            let source_ml_source2_s22541 = match statement.clone() {
                                self::JavaScriptSourceStatement::SourceForOfStatement(_, binding, _, variable, _, _, _, body, statement_pattern) => {
                                    {
                                        let pattern = ((statement_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                                        {
                                            let candidates_3 = if {
                                                let source_ml_source2_comparison_operand22575 = ((variable.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                match (source_ml_source2_comparison_operand22575.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                            } {
                                                {
                                                    let candidates_2 = if match (pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![binding]))
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({ let ml_values698 = { let ml_reference_value699 = (match self::collect_java_script_pattern_binding_declarations(pattern.clone().expect("a proven present value"), true, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))) {
                                                            self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22577, _) => {
                                                                source_ml_source2_data_property_value22577.clone()
                                                            }
                                                        }).borrow().clone(); ml_reference_value699 }; let ml_callback698 = std::rc::Rc::new(move |row: self::JavaScriptCallBindingDeclaration| -> f64 { match row.clone() {
                                                            self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22578, _) => {
                                                                source_ml_source2_data_property_value22578
                                                            }
                                                        } }); ml_values698.iter().cloned().enumerate().map(|(ml_index698, ml_value698)| ml_callback698(ml_value698)).collect::<Vec<_>>() }))
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
                                        let children_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSourceStatement>::new(), { let ml_reference_value700 = body_2.borrow().clone(); ml_reference_value700 }), { let ml_reference_value701 = finalizer.borrow().clone(); ml_reference_value701 })));
                                        self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_2.clone())
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceTryStatement(_, body_3, handler, _) => {
                                    {
                                        let children_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSourceStatement>::new(), { let ml_reference_value702 = body_3.borrow().clone(); ml_reference_value702 }), { let ml_reference_value703 = handler.borrow().clone(); ml_reference_value703 })));
                                        self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates.clone(), children_3.clone())
                                    }
                                }
                                self::JavaScriptSourceStatement::SourceArrayBindingStatement(pattern_2, _, _, variable_2) => {
                                    let pattern_2 = *pattern_2;
                                    {
                                        let candidates_5 = if {
                                            let source_ml_source2_comparison_operand22583 = ((variable_2.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                            match (source_ml_source2_comparison_operand22583.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        } {
                                            {
                                                let candidates_4 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values704 = { let ml_reference_value705 = (match self::collect_java_script_pattern_binding_declarations(pattern_2.clone(), true, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))) {
                                                    self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22585, _) => {
                                                        source_ml_source2_data_property_value22585.clone()
                                                    }
                                                }).borrow().clone(); ml_reference_value705 }; let ml_callback704 = std::rc::Rc::new(move |row: self::JavaScriptCallBindingDeclaration| -> f64 { match row.clone() {
                                                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22586, _) => {
                                                        source_ml_source2_data_property_value22586
                                                    }
                                                } }); ml_values704.iter().cloned().enumerate().map(|(ml_index704, ml_value704)| ml_callback704(ml_value704)).collect::<Vec<_>>() }));
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
                                            let source_ml_source2_comparison_operand22587 = ((variable_3.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                            match (source_ml_source2_comparison_operand22587.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        } {
                                            {
                                                let candidates_6 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values706 = { let ml_reference_value707 = (match self::collect_java_script_pattern_binding_declarations(pattern_3.clone(), true, (depth + 1f64), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))) {
                                                    self::JavaScriptPatternBindingDeclarations::BindingDeclarations(source_ml_source2_data_property_value22589, _) => {
                                                        source_ml_source2_data_property_value22589.clone()
                                                    }
                                                }).borrow().clone(); ml_reference_value707 }; let ml_callback706 = std::rc::Rc::new(move |row: self::JavaScriptCallBindingDeclaration| -> f64 { match row.clone() {
                                                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22590, _) => {
                                                        source_ml_source2_data_property_value22590
                                                    }
                                                } }); ml_values706.iter().cloned().enumerate().map(|(ml_index706, ml_value706)| ml_callback706(ml_value706)).collect::<Vec<_>>() }));
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
                                        let children_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSourceStatement>::new(), { let ml_reference_value708 = yes.borrow().clone(); ml_reference_value708 }), { let ml_reference_value709 = no.borrow().clone(); ml_reference_value709 })));
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
                            match source_ml_source2_s22541.clone() {
                                self::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242::MlSource1TemporaryCollectJavaScriptSourceVariableDeclarationsJoin242Next(candidates_9, children_10) => {
                                    {
                                        let candidates_10 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value22594 = self::collect_java_script_source_variable_declarations(children_10.clone(), context.clone(), (depth + 1f64));
                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value710 = candidates_9.borrow().clone(); ml_reference_value710 }), { let ml_reference_value711 = source_ml_source2_concat_value22594.borrow().clone(); ml_reference_value711 })
                                        }));
                                        {
                                            let source_ml_source1_temporary_index240 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s22595 = self::source_ml_source1_temporary_collect_java_script_source_variable_declarations_loop243(context.clone(), identities.clone(), candidates_10.clone(), source_ml_source1_temporary_index240.clone());
                                                match source_ml_source2_s22595.clone() {
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
