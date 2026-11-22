// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceControlScope;
use super::JavaScriptCallSourceControlTransfer;
use super::JavaScriptCallSourceResult;
use super::append_java_script_call_source_instruction;
use super::ml_array;
use super::ml_source_exception;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_control_jump(source: self::JavaScriptCallSource, builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    {
        let target_error = String::from("a control jump requires its actual source target");
        match source.clone() {
            self::JavaScriptCallSource::SourceConstant(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceNewTarget => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArgument(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceDerivedReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceErrorIntrinsic => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceConstruct(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceThrow(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceReturn(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceControlJump(target, continuing) => {
                {
                    let scope = { let ml_values141 = { let ml_reference_value142 = ((match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12912, _) => {
                            ((source_ml_source2_data_property_value12912.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlScope>::new())))).borrow().clone(); ml_reference_value142 }; let ml_callback141 = { let target = target; std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlScope| -> bool { ((match row.clone() {
                        self::JavaScriptCallSourceControlScope::SourceControlScope(source_ml_source2_data_property_value12914, _, _) => {
                            source_ml_source2_data_property_value12914
                        }
                    }) == target) }) }; ml_values141.iter().cloned().enumerate().find_map(|(ml_index141, ml_value141)| if ml_callback141(ml_value141.clone()) { Some(ml_value141) } else { None }) };
                    if ((match (scope.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (continuing && (({
                        let source_ml_source2_nullish_field_receiver12918 = (scope.clone(),);
                        if match (source_ml_source2_nullish_field_receiver12918.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present12919 = ((source_ml_source2_nullish_field_receiver12918.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present12919.clone() {
                                    self::JavaScriptCallSourceControlScope::SourceControlScope(_, source_ml_source2_data_property_value12920, _) => {
                                        source_ml_source2_data_property_value12920.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'kind')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    }) != "loop"))) || (({
                        let source_ml_source2_nullish_field_receiver12925 = (scope.clone(),);
                        if match (source_ml_source2_nullish_field_receiver12925.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present12926 = ((source_ml_source2_nullish_field_receiver12925.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present12926.clone() {
                                    self::JavaScriptCallSourceControlScope::SourceControlScope(_, _, source_ml_source2_data_property_value12927) => {
                                        source_ml_source2_data_property_value12927
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'handlers')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    }) > (match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12928, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12928
                        }
                    }))) {
                        self::refuse_java_script_call_source(builder.clone(), "break and continue retain an active actual target and handler depth")
                    } else {
                        {
                            let current = builder.clone();
                            {
                                let position = { let ml_reference_array143 = match current.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12929, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12929.clone()
                                    }
                                }; let ml_reference_projection143 = ml_reference_array143.borrow().len() as f64; ml_reference_projection143 };
                                {
                                    let current_2 = self::append_java_script_call_source_instruction(current.clone(), if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12934, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value12934
                                        }
                                    }) > ({
                                        let source_ml_source2_nullish_field_receiver12935 = (scope.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver12935.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source2_nullish_field_present12936 = ((source_ml_source2_nullish_field_receiver12935.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source2_nullish_field_present12936.clone() {
                                                    self::JavaScriptCallSourceControlScope::SourceControlScope(_, _, source_ml_source2_data_property_value12937) => {
                                                        source_ml_source2_data_property_value12937
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'handlers')")).encode_utf16().collect(), message_present: true }).raise()
                                        }
                                    })) {
                                        self::JavaScriptCallInstruction::CompleteJump(0f64, {
                                            let source_ml_source2_nullish_field_receiver12938 = (scope.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver12938.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present12939 = ((source_ml_source2_nullish_field_receiver12938.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present12939.clone() {
                                                        self::JavaScriptCallSourceControlScope::SourceControlScope(_, _, source_ml_source2_data_property_value12940) => {
                                                            source_ml_source2_data_property_value12940
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'handlers')")).encode_utf16().collect(), message_present: true }).raise()
                                            }
                                        })
                                    } else {
                                        self::JavaScriptCallInstruction::Jump(0f64)
                                    });
                                    {
                                        let transfers = (match current_2.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12941) => {
                                                ((source_ml_source2_data_property_value12941.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlTransfer>::new())));
                                        if (({ let ml_reference_array144 = transfers.clone(); let ml_reference_projection144 = ml_reference_array144.borrow().len() as f64; ml_reference_projection144 }) >= (65536f64)) {
                                            self::refuse_java_script_call_source(current_2.clone(), "control transfers exceed their explicit transfer budget")
                                        } else {
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                                                let source_ml_source2_nominal_field12945 = match current_2.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12946, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12946.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12947 = match current_2.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12948, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12948
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12949 = match current_2.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12950, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value12950
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12951 = match current_2.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12952, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12952
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12953 = match current_2.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12954, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value12954
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field12955 = match current_2.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12956, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value12956.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field12957 = match current_2.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12958, _, _, _) => {
                                                                                source_ml_source2_data_property_value12958.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field12959 = match current_2.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12960, _, _) => {
                                                                                    source_ml_source2_data_property_value12960.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field12961 = match current_2.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12962, _) => {
                                                                                        source_ml_source2_data_property_value12962.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field12963 = match current_2.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12964) => {
                                                                                            source_ml_source2_data_property_value12964.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_override12965 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source2_concat_value12943 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(position, target, continuing)]));
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceControlTransfer>::new(), { let ml_reference_value145 = transfers.borrow().clone(); ml_reference_value145 }), { let ml_reference_value146 = source_ml_source2_concat_value12943.borrow().clone(); ml_reference_value146 })
                                                                                        }));
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12945.clone(), source_ml_source2_nominal_field12947, source_ml_source2_nominal_field12949, source_ml_source2_nominal_field12951, source_ml_source2_nominal_field12953, source_ml_source2_nominal_field12955.clone(), source_ml_source2_nominal_field12957.clone(), source_ml_source2_nominal_field12959.clone(), source_ml_source2_nominal_field12961.clone(), Some((Some(source_ml_source2_nominal_override12965.clone()),)))
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
                                            }), 0f64)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceFinally(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceTry(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceBinding(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceLet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceRecur(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceAllocate(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceObject(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceDelete(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceHas(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
            self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(target_error))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_call_source_control_scope(builder: self::JavaScriptCallSourceBuilder, target: f64, kind: &str) -> self::JavaScriptCallSourceBuilder {
    {
        let scopes = (match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12966, _) => {
                ((source_ml_source2_data_property_value12966.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlScope>::new())));
        if (((!(self::ml_array::includes(&{ let ml_reference_value147 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("loop"), String::from("switch"), String::from("block")]))).borrow().clone(); ml_reference_value147 }, &kind.to_owned(), 0f64, false, |a, b| a == b)) || !(self::test_java_script_call_code_coordinate(target, 65536f64))) || (({ let ml_reference_array148 = scopes.clone(); let ml_reference_projection148 = ml_reference_array148.borrow().len() as f64; ml_reference_projection148 }) >= (128f64))) || { let ml_values149 = { let ml_reference_value150 = scopes.borrow().clone(); ml_reference_value150 }; let ml_callback149 = { let target = target; std::rc::Rc::new(move |scope: self::JavaScriptCallSourceControlScope| -> bool { ((match scope.clone() {
            self::JavaScriptCallSourceControlScope::SourceControlScope(source_ml_source2_data_property_value12968, _, _) => {
                source_ml_source2_data_property_value12968
            }
        }) == target) }) }; ml_values149.iter().cloned().enumerate().any(|(ml_index149, ml_value149)| ml_callback149(ml_value149)) }) {
            {
                let source_ml_source2_nominal_field12970 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12971, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value12971.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field12972 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12973, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12973
                        }
                    };
                    {
                        let source_ml_source2_nominal_field12974 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12975, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12975
                            }
                        };
                        {
                            let source_ml_source2_nominal_field12976 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12977, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value12977
                                }
                            };
                            {
                                let source_ml_source2_nominal_field12978 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12979, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12979
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field12980 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12981, _, _, _, _) => {
                                            source_ml_source2_data_property_value12981.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field12982 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12983, _, _, _) => {
                                                source_ml_source2_data_property_value12983.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12984 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12985, _, _) => {
                                                    source_ml_source2_data_property_value12985.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12986 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12987, _) => {
                                                        source_ml_source2_data_property_value12987.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12988 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12989) => {
                                                            source_ml_source2_data_property_value12989.clone()
                                                        }
                                                    };
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12970.clone(), source_ml_source2_nominal_field12972, source_ml_source2_nominal_field12974, source_ml_source2_nominal_field12976, source_ml_source2_nominal_field12978, String::from("control scopes require unique actual source targets within their lexical budget"), source_ml_source2_nominal_field12982.clone(), source_ml_source2_nominal_field12984.clone(), source_ml_source2_nominal_field12986.clone(), source_ml_source2_nominal_field12988.clone())
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
            {
                let source_ml_source2_nominal_field12995 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12996, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value12996.clone()
                    }
                };
                {
                    let source_ml_source2_nominal_field12997 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12998, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12998
                        }
                    };
                    {
                        let source_ml_source2_nominal_field12999 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value13000, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value13000
                            }
                        };
                        {
                            let source_ml_source2_nominal_field13001 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value13002, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value13002
                                }
                            };
                            {
                                let source_ml_source2_nominal_field13003 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value13004, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value13004
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field13005 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value13006, _, _, _, _) => {
                                            source_ml_source2_data_property_value13006.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field13007 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value13008, _, _, _) => {
                                                source_ml_source2_data_property_value13008.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field13009 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value13010, _, _) => {
                                                    source_ml_source2_data_property_value13010.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field13011 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13012, _) => {
                                                        source_ml_source2_data_property_value13012.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field13013 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13014) => {
                                                            source_ml_source2_data_property_value13014.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_override13015 = std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_concat_value12993 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallSourceControlScope::SourceControlScope(target, kind.to_owned(), match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12991, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value12991
                                                                }
                                                            })]));
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallSourceControlScope>::new(), { let ml_reference_value151 = scopes.borrow().clone(); ml_reference_value151 }), { let ml_reference_value152 = source_ml_source2_concat_value12993.borrow().clone(); ml_reference_value152 })
                                                        }));
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12995.clone(), source_ml_source2_nominal_field12997, source_ml_source2_nominal_field12999, source_ml_source2_nominal_field13001, source_ml_source2_nominal_field13003, source_ml_source2_nominal_field13005.clone(), source_ml_source2_nominal_field13007.clone(), source_ml_source2_nominal_field13009.clone(), Some((Some(source_ml_source2_nominal_override13015.clone()),)), source_ml_source2_nominal_field13013.clone())
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
