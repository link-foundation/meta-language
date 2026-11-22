// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallSource;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorFunctionOwner;
use super::JavaScriptErrorModuleRegistry;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceCapabilityOwner;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::lower_java_script_error_expression;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::refuse_java_script_error_source;
use super::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop86;
use super::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop88;
use super::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop90;
use super::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop92;
use super::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop93;
use super::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop95;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_error_construction_expression(expression: self::JavaScriptErrorExpression, next: self::JavaScriptErrorSourceContext, depth: f64) -> self::JavaScriptErrorSourceExpression {
    {
        let constant = std::rc::Rc::new(self::construct_java_script_error_constant);
        match expression.clone() {
            self::JavaScriptErrorExpression::ErrorClosure(expression_owner, ordinal, captures, receiver_mode) => {
                {
                    let owner = { let ml_values231 = { let ml_reference_value232 = (match match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15450, _, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value15450 = *source_ml_source2_data_property_value15450;
                            source_ml_source2_data_property_value15450.clone()
                        }
                    } {
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value15451, _, _, _, _) => {
                            source_ml_source2_data_property_value15451.clone()
                        }
                    }).borrow().clone(); ml_reference_value232 }; let ml_callback231 = { let expression_owner = expression_owner; std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionOwner| -> bool { ((match row.clone() {
                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value15455, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15455
                        }
                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value15456, _, _, _) => {
                            source_ml_source2_data_property_value15456
                        }
                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value15457, _, _) => {
                            source_ml_source2_data_property_value15457
                        }
                    }) == expression_owner) }) }; ml_values231.iter().cloned().enumerate().find_map(|(ml_index231, ml_value231)| if ml_callback231(ml_value231.clone()) { Some(ml_value231) } else { None }) };
                    if (match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match owner.clone().expect("a proven present value") {
                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value15463, _, _, _) => {
                            source_ml_source2_data_property_value15463.clone()
                        }
                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value15464, _) => {
                            source_ml_source2_data_property_value15464.clone()
                        }
                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value15465) => {
                            source_ml_source2_data_property_value15465.clone()
                        }
                    }) != (match match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15466, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value15466 = *source_ml_source2_data_property_value15466;
                            source_ml_source2_data_property_value15466.clone()
                        }
                    } {
                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value15467, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15467.clone()
                        }
                    }))) {
                        self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
                    } else {
                        {
                            let identities = std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_concat_value15469 = (match next.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15468, _, _) => {
                                        ((source_ml_source2_data_property_value15468.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value233 = source_ml_source2_concat_value15469.borrow().clone(); ml_reference_value233 }), { let ml_reference_value234 = captures.borrow().clone(); ml_reference_value234 })
                            }));
                            if ((({ let ml_reference_array235 = identities.clone(); let ml_reference_projection235 = ml_reference_array235.borrow().len() as f64; ml_reference_projection235 }) > (4096f64)) || { let ml_values236 = { let ml_reference_value237 = identities.borrow().clone(); ml_reference_value237 }; let ml_callback236 = { let identities = identities.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { ((self::ml_array::index_of(&{ let ml_reference_value238 = identities.borrow().clone(); ml_reference_value238 }, &identity, 0f64, false, |a, b| a == b)) != index) }) }; ml_values236.iter().cloned().enumerate().any(|(ml_index236, ml_value236)| ml_callback236(ml_value236, (ml_index236 as f64))) }) {
                                self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 119, 105, 116, 104, 105, 110, 32, 116, 104, 101, 105, 114, 32, 98, 117, 100, 103, 101, 116])))
                            } else {
                                if (({
                                    let source_ml_source2_comparison_operand15471 = ((receiver_mode.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                    {
                                        let source_ml_source2_comparison_operand15472 = None::<()>;
                                        !(match (source_ml_source2_comparison_operand15471.clone(), source_ml_source2_comparison_operand15472.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                    }
                                } && {
                                    let source_ml_source2_comparison_operand15473 = ((receiver_mode.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                    !(match (source_ml_source2_comparison_operand15473.clone(), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                }) && {
                                    let source_ml_source2_comparison_operand15475 = ((receiver_mode.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                    !(match (source_ml_source2_comparison_operand15475.clone(), self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                }) {
                                    self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![114, 117, 110, 116, 105, 109, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 99, 116, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 112, 111, 108, 105, 99, 121])))
                                } else {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceClosure(match owner.clone().expect("a proven present value") {
                                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value15477, _, _, _, _) => {
                                            source_ml_source2_data_property_value15477.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value15478, _, _) => {
                                            source_ml_source2_data_property_value15478.clone()
                                        }
                                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value15479, _) => {
                                            source_ml_source2_data_property_value15479.clone()
                                        }
                                    }, ordinal, std::rc::Rc::new(std::cell::RefCell::new({ let ml_values239 = { let ml_reference_value240 = identities.borrow().clone(); ml_reference_value240 }; let ml_callback239 = std::rc::Rc::new(move |identity: f64| -> self::JavaScriptCallSource { self::JavaScriptCallSource::SourceBinding(identity) }); ml_values239.iter().cloned().enumerate().map(|(ml_index239, ml_value239)| ml_callback239(ml_value239)).collect::<Vec<_>>() })), Box::new(if {
                                        let source_ml_source2_comparison_operand15480 = ((receiver_mode.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                        match (source_ml_source2_comparison_operand15480.clone(), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                    } {
                                        (constant.clone())(self::JavaScriptSemanticValue::Undefined)
                                    } else {
                                        self::JavaScriptCallSource::SourceReceiver
                                    }))), Box::new(next.clone()))
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptErrorExpression::ErrorCall(expression_callee, expression_receiver, arguments, construct) => {
                let expression_callee = *expression_callee;
                let expression_receiver = *expression_receiver;
                {
                    let callee = self::lower_java_script_error_expression(expression_callee.clone(), next.clone(), (depth + 1f64));
                    {
                        let receiver = self::lower_java_script_error_expression(expression_receiver.clone(), match callee.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15482) => {
                                let source_ml_source2_data_property_value15482 = *source_ml_source2_data_property_value15482;
                                source_ml_source2_data_property_value15482.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let current = match receiver.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15483) => {
                                    let source_ml_source2_data_property_value15483 = *source_ml_source2_data_property_value15483;
                                    source_ml_source2_data_property_value15483.clone()
                                }
                            };
                            {
                                let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()));
                                {
                                    let source_ml_source1_temporary_index85 = self::ml::Big::from_i128(0);
                                    {
                                        let current_2 = self::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop95(depth, current.clone(), arguments_.clone(), arguments.clone(), source_ml_source1_temporary_index85.clone());
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(if construct {
                                            self::JavaScriptCallSource::SourceConstructErrorClass(Box::new(match callee.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15484, _) => {
                                                    let source_ml_source2_data_property_value15484 = *source_ml_source2_data_property_value15484;
                                                    source_ml_source2_data_property_value15484.clone()
                                                }
                                            }), arguments_.clone())
                                        } else {
                                            if match next.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15485, _, _, _) => {
                                                    source_ml_source2_data_property_value15485
                                                }
                                            } {
                                                self::JavaScriptCallSource::SourceSnapshotCall(Box::new(match callee.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15486, _) => {
                                                        let source_ml_source2_data_property_value15486 = *source_ml_source2_data_property_value15486;
                                                        source_ml_source2_data_property_value15486.clone()
                                                    }
                                                }), Box::new(match receiver.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15487, _) => {
                                                        let source_ml_source2_data_property_value15487 = *source_ml_source2_data_property_value15487;
                                                        source_ml_source2_data_property_value15487.clone()
                                                    }
                                                }), arguments_.clone())
                                            } else {
                                                self::JavaScriptCallSource::SourceCall(Box::new(match callee.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15488, _) => {
                                                        let source_ml_source2_data_property_value15488 = *source_ml_source2_data_property_value15488;
                                                        source_ml_source2_data_property_value15488.clone()
                                                    }
                                                }), Box::new(match receiver.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15489, _) => {
                                                        let source_ml_source2_data_property_value15489 = *source_ml_source2_data_property_value15489;
                                                        source_ml_source2_data_property_value15489.clone()
                                                    }
                                                }), arguments_.clone())
                                            }
                                        }), Box::new(current_2.clone()))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptErrorExpression::ErrorGenericConstruction(expression_callee_2, arguments_2) => {
                let expression_callee_2 = *expression_callee_2;
                {
                    let callee_2 = self::lower_java_script_error_expression(expression_callee_2.clone(), next.clone(), (depth + 1f64));
                    {
                        let current_3 = match callee_2.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15490) => {
                                let source_ml_source2_data_property_value15490 = *source_ml_source2_data_property_value15490;
                                source_ml_source2_data_property_value15490.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()));
                            {
                                let source_ml_source1_temporary_index81 = self::ml::Big::from_i128(0);
                                {
                                    let current_4 = self::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop86(depth, current_3.clone(), arguments__2.clone(), arguments_2.clone(), source_ml_source1_temporary_index81.clone());
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceConstruct(Box::new(match callee_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15491, _) => {
                                            let source_ml_source2_data_property_value15491 = *source_ml_source2_data_property_value15491;
                                            source_ml_source2_data_property_value15491.clone()
                                        }
                                    }), arguments__2.clone())), Box::new(current_4.clone()))
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptErrorExpression::ErrorOrdinaryConstruction(expression_identity, arguments_3) => {
                if !(match next.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15492, _, _, _) => {
                        source_ml_source2_data_property_value15492
                    }
                }) {
                    self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101])))
                } else {
                    {
                        let owner_2 = { let ml_values241 = { let ml_reference_value242 = (match match next.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15493, _, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value15493 = *source_ml_source2_data_property_value15493;
                                source_ml_source2_data_property_value15493.clone()
                            }
                        } {
                            self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value15494, _, _, _, _) => {
                                source_ml_source2_data_property_value15494.clone()
                            }
                        }).borrow().clone(); ml_reference_value242 }; let ml_callback241 = { let expression_identity = expression_identity; std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionOwner| -> bool { ((match row.clone() {
                            self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value15498, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15498
                            }
                            self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value15499, _, _, _) => {
                                source_ml_source2_data_property_value15499
                            }
                            self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value15500, _, _) => {
                                source_ml_source2_data_property_value15500
                            }
                        }) == expression_identity) }) }; ml_values241.iter().cloned().enumerate().find_map(|(ml_index241, ml_value241)| if ml_callback241(ml_value241.clone()) { Some(ml_value241) } else { None }) };
                        if match (owner_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 99, 108, 97, 115, 115, 32, 97, 110, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 111, 119, 110, 101, 114])))
                        } else {
                            match owner_2.clone().expect("a proven present value") {
                                self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, owner_identity, _, prototype) => {
                                    let prototype = *prototype;
                                    if { let ml_values243 = { let ml_reference_value244 = arguments_3.borrow().clone(); ml_reference_value244 }; let ml_callback243 = std::rc::Rc::new(move |argument: self::JavaScriptErrorExpression| -> bool { match argument.clone() {
                                        self::JavaScriptErrorExpression::ErrorArgumentSpread(_, _) => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    } }); ml_values243.iter().cloned().enumerate().any(|(ml_index243, ml_value243)| ml_callback243(ml_value243)) } {
                                        {
                                            let current_5 = next.clone();
                                            {
                                                let arguments__3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()));
                                                {
                                                    let source_ml_source1_temporary_index82 = self::ml::Big::from_i128(0);
                                                    {
                                                        let current_6 = self::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop88(depth, current_5.clone(), arguments__3.clone(), arguments_3.clone(), source_ml_source1_temporary_index82.clone());
                                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceVariadicCall(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object(owner_identity.clone(), true))), Box::new(self::JavaScriptCallSource::SourceAllocate(Box::new((constant.clone())(prototype.clone())))), arguments__3.clone(), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), true)), Box::new(current_6.clone()))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let current_7 = next.clone();
                                            {
                                                let captures_2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<(f64, self::JavaScriptCallSource)>>>::new()));
                                                {
                                                    let source_ml_source1_temporary_values83 = arguments_3.clone();
                                                    {
                                                        let source_ml_source1_temporary_index83 = self::ml::Big::from_i128(0);
                                                        {
                                                            let current_8 = self::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop90(depth, current_7.clone(), captures_2.clone(), source_ml_source1_temporary_values83.clone(), source_ml_source1_temporary_index83.clone());
                                                            {
                                                                let identity = match current_8.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15591, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value15591
                                                                    }
                                                                };
                                                                {
                                                                    let current_9 = {
                                                                        let source_ml_source2_nominal_field15593 = match current_8.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15594, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value15594 = *source_ml_source2_data_property_value15594;
                                                                                source_ml_source2_data_property_value15594.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field15595 = match current_8.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15596, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value15596 = *source_ml_source2_data_property_value15596;
                                                                                    source_ml_source2_data_property_value15596.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field15597 = match current_8.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15598, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value15598.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field15599 = match current_8.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15600, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value15600
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15601 = match current_8.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15602, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value15602
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15603 = match current_8.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15604, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value15604.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15605 = match current_8.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15606, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value15606
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15607 = match current_8.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15608, _, _) => {
                                                                                                            source_ml_source2_data_property_value15608.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15609 = match current_8.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15610, _) => {
                                                                                                                source_ml_source2_data_property_value15610.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15611 = match current_8.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15612) => {
                                                                                                                    source_ml_source2_data_property_value15612.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override15613 = (identity + 1f64);
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15593.clone()), Box::new(source_ml_source2_nominal_field15595.clone()), source_ml_source2_nominal_field15597.clone(), source_ml_source2_nominal_override15613, source_ml_source2_nominal_field15601, source_ml_source2_nominal_field15603.clone(), source_ml_source2_nominal_field15605, source_ml_source2_nominal_field15607.clone(), source_ml_source2_nominal_field15609.clone(), source_ml_source2_nominal_field15611.clone())
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
                                                                        let receiver_2 = self::JavaScriptCallSource::SourceBinding(identity);
                                                                        {
                                                                            let source = self::construct_java_script_error_local_binding(current_9.clone(), identity, self::JavaScriptCallSource::SourceAllocate(Box::new((constant.clone())(prototype.clone()))), self::JavaScriptCallSource::SourceClassConstructorCall(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object(owner_identity.clone(), true))), Box::new(receiver_2.clone()), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values245 = { let ml_reference_value246 = captures_2.borrow().clone(); ml_reference_value246 }; let ml_callback245 = std::rc::Rc::new(move |row: std::rc::Rc<std::cell::RefCell<(f64, self::JavaScriptCallSource)>>| -> self::JavaScriptCallSource { self::JavaScriptCallSource::SourceBinding(({ let ml_reference_value247 = row.borrow().clone(); ml_reference_value247 }).0) }); ml_values245.iter().cloned().enumerate().map(|(ml_index245, ml_value245)| ml_callback245(ml_value245)).collect::<Vec<_>>() }))), false);
                                                                            {
                                                                                let at = ({ let ml_reference_array248 = captures_2.clone(); let ml_reference_projection248 = ml_reference_array248.borrow().len() as f64; ml_reference_projection248 } - 1f64);
                                                                                {
                                                                                    let source_2 = self::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop92(current_9.clone(), captures_2.clone(), source.clone(), at);
                                                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(source_2.clone()), Box::new(current_9.clone()))
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
                                    self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 99, 108, 97, 115, 115, 32, 97, 110, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 111, 119, 110, 101, 114])))
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptErrorExpression::ErrorCollectionConstruction(source_3, arguments_4) => {
                if !(match next.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15614, _, _, _) => {
                        source_ml_source2_data_property_value15614
                    }
                }) {
                    self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101])))
                } else {
                    {
                        let owner_3 = { let ml_values249 = { let ml_reference_value250 = (match next.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15615, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15615.clone()
                            }
                        }).borrow().clone(); ml_reference_value250 }; let ml_callback249 = { let source_3 = source_3.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value15617, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15617.clone()
                            }
                        }) == source_3) }) }; ml_values249.iter().cloned().enumerate().find_map(|(ml_index249, ml_value249)| if ml_callback249(ml_value249.clone()) { Some(ml_value249) } else { None }) };
                        if ((match (owner_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match owner_3.clone().expect("a proven present value") {
                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value15618, _) => {
                                source_ml_source2_data_property_value15618
                            }
                        })) || match owner_3.clone().expect("a proven present value") {
                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value15619) => {
                                source_ml_source2_data_property_value15619
                            }
                        }) {
                            self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 111, 114, 116, 97, 98, 108, 101, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101])))
                        } else {
                            {
                                let current_10 = next.clone();
                                {
                                    let arguments__4 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()));
                                    {
                                        let source_ml_source1_temporary_index84 = self::ml::Big::from_i128(0);
                                        {
                                            let current_11 = self::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop93(depth, current_10.clone(), arguments__4.clone(), arguments_4.clone(), source_ml_source1_temporary_index84.clone());
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceConstructCollection(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object(match owner_3.clone().expect("a proven present value") {
                                                self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value15620, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value15620.clone()
                                                }
                                            }, true))), arguments__4.clone())), Box::new(current_11.clone()))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])))
            }
        }
    }
}
