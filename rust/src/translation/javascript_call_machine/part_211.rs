// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionIteratorResultCompletion;
use super::JavaScriptObjectSpreadTransition;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::accept_java_script_call_array_spread;
use super::advance_java_script_call_frame;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::resume_java_script_array_spread;
use super::step_java_script_call_machine;
use super::test_java_script_host_capability_mode;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_host_capability(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    realm: self::ml::Big,
    identity: self::ml::Big,
    value: self::JavaScriptSemanticValue,
    abrupt: bool,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error =
                    self::read_java_script_call_machine_error(program.clone(), machine.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let frame = {
                            let source_ml_source2_array_read_value37432 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index37433 = ({
                                    let ml_reference_array2446 = shared_constructor_field2.clone();
                                    let ml_reference_projection2446 =
                                        ml_reference_array2446.borrow().len() as f64;
                                    ml_reference_projection2446
                                } - 1f64);
                                {
                                    let ml_reference_array2447 =
                                        source_ml_source2_array_read_value37432.clone();
                                    let ml_reference_position2447 =
                                        source_ml_source2_array_read_index37433;
                                    let ml_reference_projection2447 = self::ml_array::optional_at(
                                        &*ml_reference_array2447.borrow(),
                                        self::ml_array::number_index(ml_reference_position2447),
                                    );
                                    ml_reference_projection2447
                                }
                            }
                        };
                        if ((match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || {
                            let source_ml_source2_s37435 =
                                match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value37438,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value37438 =
                                            *source_ml_source2_data_property_value37438;
                                        source_ml_source2_data_property_value37438.clone()
                                    }
                                };
                            match source_ml_source2_s37435.clone() {
                                self::JavaScriptCallPending::WaitHostCapability(_, _) => false,
                                _ => true,
                            }
                        }) || ((match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(
                                source_ml_source2_data_property_value37440,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value37440.clone(),
                        }) != identity))
                        {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 114,
                                    101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115,
                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101,
                                    110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 97, 98, 108,
                                    101, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                                ]),
                            )
                        } else {
                            {
                                let fn_ = self::read_java_script_executable_call_function(
                                    program.clone(),
                                    shared_constructor_field1.clone(),
                                    identity.clone(),
                                );
                                if ((match (fn_.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } || ((match fn_.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(
                                        _,
                                        source_ml_source2_data_property_value37444,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value37444.clone(),
                                }) != realm))
                                    || !(self::test_java_script_host_capability_mode(
                                        &(match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(
                                                _,
                                                _,
                                                source_ml_source2_data_property_value37445,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value37445.clone(),
                                        }),
                                    )))
                                {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            97, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121,
                                            32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116,
                                            97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                            117, 97, 108, 32, 113, 117, 97, 108, 105, 102, 105,
                                            101, 100, 32, 114, 101, 97, 108, 109, 32, 97, 117, 116,
                                            104, 111, 114, 105, 116, 121,
                                        ]),
                                    )
                                } else {
                                    {
                                        let invalid = self::read_java_script_ordinary_value_error(
                                            shared_constructor_field1.clone(),
                                            value.clone(),
                                        );
                                        if (invalid != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                                Box::new(shared_constructor_field1.clone()),
                                                invalid.clone(),
                                            )
                                        } else {
                                            if abrupt {
                                                self::throw_java_script_call_value(
                                                    machine.clone(),
                                                    value.clone(),
                                                )
                                            } else {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), ((match match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value37451, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value37451 = *source_ml_source2_data_property_value37451;
                                                        source_ml_source2_data_property_value37451.clone()
                                                    }
                                                } {
                                                    self::JavaScriptCallPending::WaitHostCapability(source_ml_source2_data_property_value37452, _) => {
                                                        Some((source_ml_source2_data_property_value37452,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0, value.clone()))))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_collection_iterator_result_completion(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    destination: f64,
    realm: self::ml::Big,
    completion: self::JavaScriptCollectionIteratorResultCompletion,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
            let current = self::JavaScriptCallMachine::CallMachine(Box::new(match completion.clone() {
                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(source_ml_source2_data_property_value37453, _) => {
                        let source_ml_source2_data_property_value37453 = *source_ml_source2_data_property_value37453;
                        source_ml_source2_data_property_value37453.clone()
                    }
                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultAllocate(source_ml_source2_data_property_value37454, _, _, _) => {
                        let source_ml_source2_data_property_value37454 = *source_ml_source2_data_property_value37454;
                        source_ml_source2_data_property_value37454.clone()
                    }
                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultNormal(source_ml_source2_data_property_value37455, _) => {
                        let source_ml_source2_data_property_value37455 = *source_ml_source2_data_property_value37455;
                        source_ml_source2_data_property_value37455.clone()
                    }
                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(source_ml_source2_data_property_value37456, _) => {
                        let source_ml_source2_data_property_value37456 = *source_ml_source2_data_property_value37456;
                        source_ml_source2_data_property_value37456.clone()
                    }
                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultTypeError(source_ml_source2_data_property_value37457, _) => {
                        let source_ml_source2_data_property_value37457 = *source_ml_source2_data_property_value37457;
                        source_ml_source2_data_property_value37457.clone()
                    }
                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultResourceLimit(source_ml_source2_data_property_value37458, _, _) => {
                        let source_ml_source2_data_property_value37458 = *source_ml_source2_data_property_value37458;
                        source_ml_source2_data_property_value37458.clone()
                    }
                }), shared_constructor_field1.clone());
            match completion.clone() {
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(
                    heap,
                    state,
                ) => {
                    let heap = *heap;
                    let state = *state;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                        self::replace_java_script_call_frame(
                            heap.clone(),
                            shared_constructor_field1.clone(),
                            self::replace_java_script_call_pending(
                                frame.clone(),
                                self::JavaScriptCallPending::WaitCollectionIteratorResult(
                                    destination,
                                    realm.clone(),
                                    Box::new(state.clone()),
                                ),
                            ),
                        ),
                    ))
                }
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultAllocate(
                    heap_2,
                    state_2,
                    prototype,
                    _,
                ) => {
                    let heap_2 = *heap_2;
                    let state_2 = *state_2;
                    let prototype = *prototype;
                    self::JavaScriptCallMachineTransition::MachineAllocateObject(
                        Box::new(self::replace_java_script_call_frame(
                            heap_2.clone(),
                            shared_constructor_field1.clone(),
                            self::replace_java_script_call_pending(
                                frame.clone(),
                                self::JavaScriptCallPending::WaitCollectionIteratorResult(
                                    destination,
                                    realm.clone(),
                                    Box::new(state_2.clone()),
                                ),
                            ),
                        )),
                        Box::new(prototype.clone()),
                    )
                }
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultNormal(
                    heap_3,
                    value,
                ) => {
                    let heap_3 = *heap_3;
                    let value = *value;
                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                        self::replace_java_script_call_frame(
                            heap_3.clone(),
                            shared_constructor_field1.clone(),
                            self::write_java_script_call_register(
                                self::advance_java_script_call_frame(
                                    self::replace_java_script_call_pending(
                                        frame.clone(),
                                        self::JavaScriptCallPending::CallIdle,
                                    ),
                                ),
                                destination,
                                value.clone(),
                            ),
                        ),
                    ))
                }
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                    heap_4,
                    reason,
                ) => {
                    let heap_4 = *heap_4;
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(heap_4.clone()),
                        reason.clone(),
                    )
                }
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultTypeError(
                    _,
                    reason_2,
                ) => self::request_java_script_call_error(
                    current.clone(),
                    realm.clone(),
                    &(self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ])),
                    &(reason_2),
                ),
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultResourceLimit(
                    heap_5,
                    state_3,
                    reason_3,
                ) => {
                    let heap_5 = *heap_5;
                    let state_3 = *state_3;
                    self::JavaScriptCallMachineTransition::MachineResourceLimit(
                        Box::new(self::replace_java_script_call_frame(
                            heap_5.clone(),
                            shared_constructor_field1.clone(),
                            self::replace_java_script_call_pending(
                                frame.clone(),
                                self::JavaScriptCallPending::WaitCollectionIteratorResult(
                                    destination,
                                    realm.clone(),
                                    Box::new(state_3.clone()),
                                ),
                            ),
                        )),
                        reason_3.clone(),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_object_spread(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptObjectSpreadTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                    source_ml_source2_data_property_value37466,
                    _,
                ) => {
                    let source_ml_source2_data_property_value37466 =
                        *source_ml_source2_data_property_value37466;
                    source_ml_source2_data_property_value37466.clone()
                }
                self::JavaScriptObjectSpreadTransition::ObjectSpreadCall(
                    source_ml_source2_data_property_value37467,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value37467 =
                        *source_ml_source2_data_property_value37467;
                    source_ml_source2_data_property_value37467.clone()
                }
                self::JavaScriptObjectSpreadTransition::ObjectSpreadCompleted(
                    source_ml_source2_data_property_value37468,
                    _,
                ) => {
                    let source_ml_source2_data_property_value37468 =
                        *source_ml_source2_data_property_value37468;
                    source_ml_source2_data_property_value37468.clone()
                }
                self::JavaScriptObjectSpreadTransition::ObjectSpreadThrow(
                    source_ml_source2_data_property_value37469,
                    _,
                ) => {
                    let source_ml_source2_data_property_value37469 =
                        *source_ml_source2_data_property_value37469;
                    source_ml_source2_data_property_value37469.clone()
                }
                self::JavaScriptObjectSpreadTransition::ObjectSpreadError(
                    source_ml_source2_data_property_value37470,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value37470 =
                        *source_ml_source2_data_property_value37470;
                    source_ml_source2_data_property_value37470.clone()
                }
                self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                    source_ml_source2_data_property_value37471,
                    _,
                ) => {
                    let source_ml_source2_data_property_value37471 =
                        *source_ml_source2_data_property_value37471;
                    source_ml_source2_data_property_value37471.clone()
                }
                self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(
                    source_ml_source2_data_property_value37472,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value37472 =
                        *source_ml_source2_data_property_value37472;
                    source_ml_source2_data_property_value37472.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value37473,
                ) => source_ml_source2_data_property_value37473.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                let frame = {
                    let source_ml_source2_array_read_value37476 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_array_read_index37477 = ({
                            let ml_reference_array2448 = shared_constructor_field1.clone();
                            let ml_reference_projection2448 =
                                ml_reference_array2448.borrow().len() as f64;
                            ml_reference_projection2448
                        } - 1f64);
                        {
                            let ml_reference_array2449 =
                                source_ml_source2_array_read_value37476.clone();
                            let ml_reference_position2449 = source_ml_source2_array_read_index37477;
                            let ml_reference_projection2449 = self::ml_array::optional_at(
                                &*ml_reference_array2449.borrow(),
                                self::ml_array::number_index(ml_reference_position2449),
                            );
                            ml_reference_projection2449
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(match transition.clone() {
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                                source_ml_source2_data_property_value37478,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value37478 =
                                    *source_ml_source2_data_property_value37478;
                                source_ml_source2_data_property_value37478.clone()
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadCall(
                                source_ml_source2_data_property_value37479,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value37479 =
                                    *source_ml_source2_data_property_value37479;
                                source_ml_source2_data_property_value37479.clone()
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadCompleted(
                                source_ml_source2_data_property_value37480,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value37480 =
                                    *source_ml_source2_data_property_value37480;
                                source_ml_source2_data_property_value37480.clone()
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadThrow(
                                source_ml_source2_data_property_value37481,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value37481 =
                                    *source_ml_source2_data_property_value37481;
                                source_ml_source2_data_property_value37481.clone()
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadError(
                                source_ml_source2_data_property_value37482,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value37482 =
                                    *source_ml_source2_data_property_value37482;
                                source_ml_source2_data_property_value37482.clone()
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                                source_ml_source2_data_property_value37483,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value37483 =
                                    *source_ml_source2_data_property_value37483;
                                source_ml_source2_data_property_value37483.clone()
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(
                                source_ml_source2_data_property_value37484,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value37484 =
                                    *source_ml_source2_data_property_value37484;
                                source_ml_source2_data_property_value37484.clone()
                            }
                        }),
                        self::ml_string::Text::from_units(vec![
                            111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114,
                            101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                            117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117, 114,
                            99, 101, 32, 102, 114, 97, 109, 101,
                        ]),
                    )
                } else {
                    match transition.clone() {
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                            heap,
                            state,
                        ) => {
                            let heap = *heap;
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitObjectSpread(
                                            destination,
                                            realm.clone(),
                                            Box::new(state.clone()),
                                        ),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadCall(
                            heap_2,
                            state_2,
                            callee,
                            receiver,
                            arguments,
                        ) => {
                            let heap_2 = *heap_2;
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(
                                program.clone(),
                                self::replace_java_script_call_frame(
                                    heap_2.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitObjectSpread(
                                            destination,
                                            realm.clone(),
                                            Box::new(state_2.clone()),
                                        ),
                                    ),
                                ),
                                callee.clone(),
                                receiver.clone(),
                                arguments.clone(),
                            )
                        }
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadCompleted(
                            heap_3,
                            value,
                        ) => {
                            let heap_3 = *heap_3;
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap_3.clone(),
                                    shared_constructor_field1.clone(),
                                    self::write_java_script_call_register(
                                        frame.clone().expect("a proven present value"),
                                        destination,
                                        value.clone(),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadThrow(_, value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(current.clone(), value_2.clone())
                        }
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadError(
                            _,
                            reason,
                            error_class,
                        ) => self::request_java_script_call_error(
                            current.clone(),
                            realm.clone(),
                            &((((error_class
                                .clone()
                                .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                            .0)
                                .clone())
                            .clone()
                            .unwrap_or_else(|| {
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ])
                            })),
                            &(reason),
                        ),
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                            heap_4,
                            reason_2,
                        ) => {
                            let heap_4 = *heap_4;
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(heap_4.clone()),
                                reason_2.clone(),
                            )
                        }
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(
                            heap_5,
                            state_3,
                            reason_3,
                        ) => {
                            let heap_5 = *heap_5;
                            let state_3 = *state_3;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(
                                Box::new(self::replace_java_script_call_frame(
                                    heap_5.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitObjectSpread(
                                            destination,
                                            realm.clone(),
                                            Box::new(state_3.clone()),
                                        ),
                                    ),
                                )),
                                reason_3.clone(),
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_primitive_iterator_get(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error =
                    self::read_java_script_call_machine_error(program.clone(), machine.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let frame = {
                            let source_ml_source2_array_read_value37494 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index37495 = ({
                                    let ml_reference_array2450 = shared_constructor_field2.clone();
                                    let ml_reference_projection2450 =
                                        ml_reference_array2450.borrow().len() as f64;
                                    ml_reference_projection2450
                                } - 1f64);
                                {
                                    let ml_reference_array2451 =
                                        source_ml_source2_array_read_value37494.clone();
                                    let ml_reference_position2451 =
                                        source_ml_source2_array_read_index37495;
                                    let ml_reference_projection2451 = self::ml_array::optional_at(
                                        &*ml_reference_array2451.borrow(),
                                        self::ml_array::number_index(ml_reference_position2451),
                                    );
                                    ml_reference_projection2451
                                }
                            }
                        };
                        if match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 116, 101,
                                    114, 97, 116, 111, 114, 32, 114, 101, 115, 112, 111, 110, 115,
                                    101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                    32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101,
                                    110, 100, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111,
                                    112, 101, 114, 97, 116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            {
                                let pending = match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value37497,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value37497 =
                                            *source_ml_source2_data_property_value37497;
                                        source_ml_source2_data_property_value37497.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::WaitArraySpread(
                                        destination,
                                        realm,
                                        state,
                                    ) => {
                                        let state = *state;
                                        {
                                            let request = self::step_java_script_call_machine(
                                                program.clone(),
                                                machine.clone(),
                                            );
                                            match request.clone() {
                                                self::JavaScriptCallMachineTransition::MachineNeedPrimitiveIteratorGet(_, _, _) => {
                                                    self::accept_java_script_call_array_spread(program.clone(), machine.clone(), destination, realm.clone(), self::resume_java_script_array_spread(shared_constructor_field1.clone(), state.clone(), completion.clone(), true))
                                                }
                                                _ => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 112, 111, 110, 115, 101, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 32, 108, 105, 118, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108]))
                                                }
                                            }
                                        }
                                    }
                                    _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105,
                                            116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115,
                                            112, 111, 110, 115, 101, 32, 114, 101, 116, 97, 105,
                                            110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                            108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100,
                                            32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101,
                                            114, 97, 116, 105, 111, 110,
                                        ]),
                                    ),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
