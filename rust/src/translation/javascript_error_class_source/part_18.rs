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
use super::ml_source_exception;
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
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15522, _, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value15522 = *source_ml_source2_data_property_value15522;
                            source_ml_source2_data_property_value15522.clone()
                        }
                    } {
                        self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value15523, _, _, _, _) => {
                            source_ml_source2_data_property_value15523.clone()
                        }
                    }).borrow().clone(); ml_reference_value232 }; let ml_callback231 = { let expression_owner = expression_owner; std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionOwner| -> bool { ((match row.clone() {
                        self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value15527, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15527
                        }
                        self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value15528, _, _, _) => {
                            source_ml_source2_data_property_value15528
                        }
                        self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value15529, _, _) => {
                            source_ml_source2_data_property_value15529
                        }
                    }) == expression_owner) }) }; ml_values231.iter().cloned().enumerate().find_map(|(ml_index231, ml_value231)| if ml_callback231(ml_value231.clone()) { Some(ml_value231) } else { None }) };
                    if (match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (({
                        let source_ml_source2_nullish_field_receiver15537 = (owner.clone(),);
                        if match (source_ml_source2_nullish_field_receiver15537.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present15538 = ((source_ml_source2_nullish_field_receiver15537.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present15538.clone() {
                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, _, source_ml_source2_data_property_value15539, _, _, _) => {
                                        source_ml_source2_data_property_value15539.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, _, source_ml_source2_data_property_value15540, _) => {
                                        source_ml_source2_data_property_value15540.clone()
                                    }
                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, _, source_ml_source2_data_property_value15541) => {
                                        source_ml_source2_data_property_value15541.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 97, 108, 109, 39, 41])).0, message_present: true }).raise()
                        }
                    }) != (match match next.clone() {
                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15542, _, _, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value15542 = *source_ml_source2_data_property_value15542;
                            source_ml_source2_data_property_value15542.clone()
                        }
                    } {
                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value15543, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value15543.clone()
                        }
                    }))) {
                        self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
                    } else {
                        {
                            let identities = std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_concat_value15545 = (match next.clone() {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15544, _, _) => {
                                        ((source_ml_source2_data_property_value15544.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value233 = source_ml_source2_concat_value15545.borrow().clone(); ml_reference_value233 }), { let ml_reference_value234 = captures.borrow().clone(); ml_reference_value234 })
                            }));
                            if ((({ let ml_reference_array235 = identities.clone(); let ml_reference_projection235 = ml_reference_array235.borrow().len() as f64; ml_reference_projection235 }) > (4096f64)) || { let ml_values236 = { let ml_reference_value237 = identities.borrow().clone(); ml_reference_value237 }; let ml_callback236 = { let identities = identities.clone(); std::rc::Rc::new(move |identity: f64, index: f64| -> bool { ((self::ml_array::index_of(&{ let ml_reference_value238 = identities.borrow().clone(); ml_reference_value238 }, &identity, 0f64, false, |a, b| a == b)) != index) }) }; ml_values236.iter().cloned().enumerate().any(|(ml_index236, ml_value236)| ml_callback236(ml_value236, (ml_index236 as f64))) }) {
                                self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![97, 114, 114, 111, 119, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 119, 105, 116, 104, 105, 110, 32, 116, 104, 101, 105, 114, 32, 98, 117, 100, 103, 101, 116])))
                            } else {
                                if (({
                                    let source_ml_source2_comparison_operand15547 = ((receiver_mode.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                    {
                                        let source_ml_source2_comparison_operand15548 = None::<()>;
                                        !(match (source_ml_source2_comparison_operand15547.clone(), source_ml_source2_comparison_operand15548.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                    }
                                } && {
                                    let source_ml_source2_comparison_operand15549 = ((receiver_mode.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                    !(match (source_ml_source2_comparison_operand15549.clone(), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                }) && {
                                    let source_ml_source2_comparison_operand15551 = ((receiver_mode.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                    !(match (source_ml_source2_comparison_operand15551.clone(), self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                }) {
                                    self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![114, 117, 110, 116, 105, 109, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 99, 116, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 112, 111, 108, 105, 99, 121])))
                                } else {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceClosure({
                                        let source_ml_source2_nullish_field_receiver15553 = (owner.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver15553.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source2_nullish_field_present15554 = ((source_ml_source2_nullish_field_receiver15553.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source2_nullish_field_present15554.clone() {
                                                    self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(_, source_ml_source2_data_property_value15555, _, _, _, _) => {
                                                        source_ml_source2_data_property_value15555.clone()
                                                    }
                                                    self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(_, source_ml_source2_data_property_value15556, _, _) => {
                                                        source_ml_source2_data_property_value15556.clone()
                                                    }
                                                    self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(_, source_ml_source2_data_property_value15557, _) => {
                                                        source_ml_source2_data_property_value15557.clone()
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
                                        }
                                    }, ordinal, std::rc::Rc::new(std::cell::RefCell::new({ let ml_values239 = { let ml_reference_value240 = identities.borrow().clone(); ml_reference_value240 }; let ml_callback239 = std::rc::Rc::new(move |identity: f64| -> self::JavaScriptCallSource { self::JavaScriptCallSource::SourceBinding(identity) }); ml_values239.iter().cloned().enumerate().map(|(ml_index239, ml_value239)| ml_callback239(ml_value239)).collect::<Vec<_>>() })), Box::new(if {
                                        let source_ml_source2_comparison_operand15558 = ((receiver_mode.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                        match (source_ml_source2_comparison_operand15558.clone(), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
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
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15560) => {
                                let source_ml_source2_data_property_value15560 = *source_ml_source2_data_property_value15560;
                                source_ml_source2_data_property_value15560.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let current = match receiver.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15561) => {
                                    let source_ml_source2_data_property_value15561 = *source_ml_source2_data_property_value15561;
                                    source_ml_source2_data_property_value15561.clone()
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
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15562, _) => {
                                                    let source_ml_source2_data_property_value15562 = *source_ml_source2_data_property_value15562;
                                                    source_ml_source2_data_property_value15562.clone()
                                                }
                                            }), arguments_.clone())
                                        } else {
                                            if match next.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15563, _, _, _) => {
                                                    source_ml_source2_data_property_value15563
                                                }
                                            } {
                                                self::JavaScriptCallSource::SourceSnapshotCall(Box::new(match callee.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15564, _) => {
                                                        let source_ml_source2_data_property_value15564 = *source_ml_source2_data_property_value15564;
                                                        source_ml_source2_data_property_value15564.clone()
                                                    }
                                                }), Box::new(match receiver.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15565, _) => {
                                                        let source_ml_source2_data_property_value15565 = *source_ml_source2_data_property_value15565;
                                                        source_ml_source2_data_property_value15565.clone()
                                                    }
                                                }), arguments_.clone())
                                            } else {
                                                self::JavaScriptCallSource::SourceCall(Box::new(match callee.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15566, _) => {
                                                        let source_ml_source2_data_property_value15566 = *source_ml_source2_data_property_value15566;
                                                        source_ml_source2_data_property_value15566.clone()
                                                    }
                                                }), Box::new(match receiver.clone() {
                                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15567, _) => {
                                                        let source_ml_source2_data_property_value15567 = *source_ml_source2_data_property_value15567;
                                                        source_ml_source2_data_property_value15567.clone()
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
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value15568) => {
                                let source_ml_source2_data_property_value15568 = *source_ml_source2_data_property_value15568;
                                source_ml_source2_data_property_value15568.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSource>::new()));
                            {
                                let source_ml_source1_temporary_index81 = self::ml::Big::from_i128(0);
                                {
                                    let current_4 = self::source_ml_source1_temporary_lower_java_script_error_construction_expression_loop86(depth, current_3.clone(), arguments__2.clone(), arguments_2.clone(), source_ml_source1_temporary_index81.clone());
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceConstruct(Box::new(match callee_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value15569, _) => {
                                            let source_ml_source2_data_property_value15569 = *source_ml_source2_data_property_value15569;
                                            source_ml_source2_data_property_value15569.clone()
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
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15570, _, _, _) => {
                        source_ml_source2_data_property_value15570
                    }
                }) {
                    self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101])))
                } else {
                    {
                        let owner_2 = { let ml_values241 = { let ml_reference_value242 = (match match next.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15571, _, _, _, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value15571 = *source_ml_source2_data_property_value15571;
                                source_ml_source2_data_property_value15571.clone()
                            }
                        } {
                            self::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(source_ml_source2_data_property_value15572, _, _, _, _) => {
                                source_ml_source2_data_property_value15572.clone()
                            }
                        }).borrow().clone(); ml_reference_value242 }; let ml_callback241 = { let expression_identity = expression_identity; std::rc::Rc::new(move |row: self::JavaScriptErrorFunctionOwner| -> bool { ((match row.clone() {
                            self::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(source_ml_source2_data_property_value15576, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15576
                            }
                            self::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(source_ml_source2_data_property_value15577, _, _, _) => {
                                source_ml_source2_data_property_value15577
                            }
                            self::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(source_ml_source2_data_property_value15578, _, _) => {
                                source_ml_source2_data_property_value15578
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
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15669, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value15669
                                                                    }
                                                                };
                                                                {
                                                                    let current_9 = {
                                                                        let source_ml_source2_nominal_field15671 = match current_8.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value15672, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value15672 = *source_ml_source2_data_property_value15672;
                                                                                source_ml_source2_data_property_value15672.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field15673 = match current_8.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value15674, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value15674 = *source_ml_source2_data_property_value15674;
                                                                                    source_ml_source2_data_property_value15674.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field15675 = match current_8.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15676, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value15676.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field15677 = match current_8.clone() {
                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value15678, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value15678
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15679 = match current_8.clone() {
                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value15680, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value15680
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15681 = match current_8.clone() {
                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value15682, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value15682.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15683 = match current_8.clone() {
                                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15684, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value15684
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15685 = match current_8.clone() {
                                                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value15686, _, _) => {
                                                                                                            source_ml_source2_data_property_value15686.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15687 = match current_8.clone() {
                                                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value15688, _) => {
                                                                                                                source_ml_source2_data_property_value15688.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15689 = match current_8.clone() {
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15690) => {
                                                                                                                    source_ml_source2_data_property_value15690.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override15691 = (identity + 1f64);
                                                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field15671.clone()), Box::new(source_ml_source2_nominal_field15673.clone()), source_ml_source2_nominal_field15675.clone(), source_ml_source2_nominal_override15691, source_ml_source2_nominal_field15679, source_ml_source2_nominal_field15681.clone(), source_ml_source2_nominal_field15683, source_ml_source2_nominal_field15685.clone(), source_ml_source2_nominal_field15687.clone(), source_ml_source2_nominal_field15689.clone())
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
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value15692, _, _, _) => {
                        source_ml_source2_data_property_value15692
                    }
                }) {
                    self::refuse_java_script_error_source(next.clone(), &(self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101])))
                } else {
                    {
                        let owner_3 = { let ml_values249 = { let ml_reference_value250 = (match next.clone() {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value15693, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15693.clone()
                            }
                        }).borrow().clone(); ml_reference_value250 }; let ml_callback249 = { let source_3 = source_3.clone(); std::rc::Rc::new(move |row: self::JavaScriptSourceCapabilityOwner| -> bool { ((match row.clone() {
                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(source_ml_source2_data_property_value15695, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value15695.clone()
                            }
                        }) == source_3) }) }; ml_values249.iter().cloned().enumerate().find_map(|(ml_index249, ml_value249)| if ml_callback249(ml_value249.clone()) { Some(ml_value249) } else { None }) };
                        if ((match (owner_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !({
                            let source_ml_source2_nullish_field_receiver15696 = (owner_3.clone(),);
                            if match (source_ml_source2_nullish_field_receiver15696.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source2_nullish_field_present15697 = ((source_ml_source2_nullish_field_receiver15696.clone()).0).clone().expect("a proven present value");
                                    match source_ml_source2_nullish_field_present15697.clone() {
                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, source_ml_source2_data_property_value15698, _) => {
                                            source_ml_source2_data_property_value15698
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                            }
                        })) || {
                            let source_ml_source2_nullish_field_receiver15699 = (owner_3.clone(),);
                            if match (source_ml_source2_nullish_field_receiver15699.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source2_nullish_field_present15700 = ((source_ml_source2_nullish_field_receiver15699.clone()).0).clone().expect("a proven present value");
                                    match source_ml_source2_nullish_field_present15700.clone() {
                                        self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, _, _, _, _, _, source_ml_source2_data_property_value15701) => {
                                            source_ml_source2_data_property_value15701
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
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
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(Box::new(self::JavaScriptCallSource::SourceConstructCollection(Box::new((constant.clone())(self::JavaScriptSemanticValue::Object({
                                                let source_ml_source2_nullish_field_receiver15702 = (owner_3.clone(),);
                                                if match (source_ml_source2_nullish_field_receiver15702.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present15703 = ((source_ml_source2_nullish_field_receiver15702.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present15703.clone() {
                                                            self::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(_, source_ml_source2_data_property_value15704, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value15704.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 100, 101, 110, 116, 105, 116, 121, 39, 41])).0, message_present: true }).raise()
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
