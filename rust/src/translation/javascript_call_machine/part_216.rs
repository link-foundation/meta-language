// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionForEachTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveWrapperTransition;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::JavaScriptSymbolRegistryTransition;
use super::allocate_java_script_registered_symbol;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::register_java_script_ordinary_symbol;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::retain_java_script_symbol_registration;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_collection_for_each(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptCollectionForEachTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptCollectionForEachTransition::CollectionForEachContinue(
                    source_ml_source2_data_property_value38497,
                    _,
                ) => {
                    let source_ml_source2_data_property_value38497 =
                        *source_ml_source2_data_property_value38497;
                    source_ml_source2_data_property_value38497.clone()
                }
                self::JavaScriptCollectionForEachTransition::CollectionForEachCall(
                    source_ml_source2_data_property_value38498,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value38498 =
                        *source_ml_source2_data_property_value38498;
                    source_ml_source2_data_property_value38498.clone()
                }
                self::JavaScriptCollectionForEachTransition::CollectionForEachCompleted(
                    source_ml_source2_data_property_value38499,
                    _,
                ) => {
                    let source_ml_source2_data_property_value38499 =
                        *source_ml_source2_data_property_value38499;
                    source_ml_source2_data_property_value38499.clone()
                }
                self::JavaScriptCollectionForEachTransition::CollectionForEachThrow(
                    source_ml_source2_data_property_value38500,
                    _,
                ) => {
                    let source_ml_source2_data_property_value38500 =
                        *source_ml_source2_data_property_value38500;
                    source_ml_source2_data_property_value38500.clone()
                }
                self::JavaScriptCollectionForEachTransition::CollectionForEachError(
                    source_ml_source2_data_property_value38501,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value38501 =
                        *source_ml_source2_data_property_value38501;
                    source_ml_source2_data_property_value38501.clone()
                }
                self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(
                    source_ml_source2_data_property_value38502,
                    _,
                ) => {
                    let source_ml_source2_data_property_value38502 =
                        *source_ml_source2_data_property_value38502;
                    source_ml_source2_data_property_value38502.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value38503,
                ) => source_ml_source2_data_property_value38503.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                let frame = {
                    let source_ml_source2_array_read_value38506 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_array_read_index38507 = ({
                            let ml_reference_array2487 = shared_constructor_field1.clone();
                            let ml_reference_projection2487 =
                                ml_reference_array2487.borrow().len() as f64;
                            ml_reference_projection2487
                        } - 1f64);
                        {
                            let ml_reference_array2488 =
                                source_ml_source2_array_read_value38506.clone();
                            let ml_reference_position2488 = source_ml_source2_array_read_index38507;
                            let ml_reference_projection2488 = self::ml_array::optional_at(
                                &*ml_reference_array2488.borrow(),
                                self::ml_array::number_index(ml_reference_position2488),
                            );
                            ml_reference_projection2488
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                            self::JavaScriptCollectionForEachTransition::CollectionForEachContinue(source_ml_source2_data_property_value38508, _) => {
                                let source_ml_source2_data_property_value38508 = *source_ml_source2_data_property_value38508;
                                source_ml_source2_data_property_value38508.clone()
                            }
                            self::JavaScriptCollectionForEachTransition::CollectionForEachCall(source_ml_source2_data_property_value38509, _, _, _, _) => {
                                let source_ml_source2_data_property_value38509 = *source_ml_source2_data_property_value38509;
                                source_ml_source2_data_property_value38509.clone()
                            }
                            self::JavaScriptCollectionForEachTransition::CollectionForEachCompleted(source_ml_source2_data_property_value38510, _) => {
                                let source_ml_source2_data_property_value38510 = *source_ml_source2_data_property_value38510;
                                source_ml_source2_data_property_value38510.clone()
                            }
                            self::JavaScriptCollectionForEachTransition::CollectionForEachThrow(source_ml_source2_data_property_value38511, _) => {
                                let source_ml_source2_data_property_value38511 = *source_ml_source2_data_property_value38511;
                                source_ml_source2_data_property_value38511.clone()
                            }
                            self::JavaScriptCollectionForEachTransition::CollectionForEachError(source_ml_source2_data_property_value38512, _, _) => {
                                let source_ml_source2_data_property_value38512 = *source_ml_source2_data_property_value38512;
                                source_ml_source2_data_property_value38512.clone()
                            }
                            self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(source_ml_source2_data_property_value38513, _) => {
                                let source_ml_source2_data_property_value38513 = *source_ml_source2_data_property_value38513;
                                source_ml_source2_data_property_value38513.clone()
                            }
                        }), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptCollectionForEachTransition::CollectionForEachContinue(
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
                                        self::JavaScriptCallPending::WaitCollectionForEach(
                                            destination,
                                            realm.clone(),
                                            Box::new(state.clone()),
                                        ),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptCollectionForEachTransition::CollectionForEachCall(
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
                                        self::JavaScriptCallPending::WaitCollectionForEach(
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
                        self::JavaScriptCollectionForEachTransition::CollectionForEachCompleted(
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
                        self::JavaScriptCollectionForEachTransition::CollectionForEachThrow(
                            _,
                            value_2,
                        ) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(current.clone(), value_2.clone())
                        }
                        self::JavaScriptCollectionForEachTransition::CollectionForEachError(
                            _,
                            error_class,
                            reason,
                        ) => self::request_java_script_call_error(
                            current.clone(),
                            realm.clone(),
                            &(error_class),
                            &(reason),
                        ),
                        self::JavaScriptCollectionForEachTransition::CollectionForEachInvalid(
                            heap_4,
                            reason_2,
                        ) => {
                            let heap_4 = *heap_4;
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(heap_4.clone()),
                                reason_2.clone(),
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
pub fn resume_java_script_call_symbol_allocation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    identity: self::ml::Big,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field11 = *shared_constructor_field11;
            {
                let error =
                    self::read_java_script_call_machine_error(program.clone(), machine.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field11.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let frame = {
                            let source_ml_source2_array_read_value38522 =
                                shared_constructor_field12.clone();
                            {
                                let source_ml_source2_array_read_index38523 = ({
                                    let ml_reference_array2489 = shared_constructor_field12.clone();
                                    let ml_reference_projection2489 =
                                        ml_reference_array2489.borrow().len() as f64;
                                    ml_reference_projection2489
                                } - 1f64);
                                {
                                    let ml_reference_array2490 =
                                        source_ml_source2_array_read_value38522.clone();
                                    let ml_reference_position2490 =
                                        source_ml_source2_array_read_index38523;
                                    let ml_reference_projection2490 = self::ml_array::optional_at(
                                        &*ml_reference_array2490.borrow(),
                                        self::ml_array::number_index(ml_reference_position2490),
                                    );
                                    ml_reference_projection2490
                                }
                            }
                        };
                        if match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field11.clone()),
                                self::ml_string::Text::from_units(vec![
                                    83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99, 97, 116,
                                    105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117,
                                    115, 112, 101, 110, 100, 101, 100, 32, 115, 111, 117, 114, 99,
                                    101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
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
                                        source_ml_source2_data_property_value38525,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value38525 =
                                            *source_ml_source2_data_property_value38525;
                                        source_ml_source2_data_property_value38525.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::WaitSymbolAllocation(
                                        destination,
                                        _,
                                        pending_registry_key,
                                    ) => {
                                        if ((((identity <= (self::ml::Big::from_i128(0)))
                                            || (identity
                                                > (self::ml::Big::from_i128(
                                                    18446744073709551615,
                                                ))))
                                            || self::ml_array::includes(
                                                &{
                                                    let ml_reference_value2491 = (match shared_constructor_field11.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value38530, _, _, _, _) => {
                                                source_ml_source2_data_property_value38530.clone()
                                            }
                                        }).borrow().clone();
                                                    ml_reference_value2491
                                                },
                                                &identity.clone(),
                                                0f64,
                                                false,
                                                |a, b| a == b,
                                            ))
                                            || {
                                                let ml_values2492 = {
                                                    let ml_reference_value2493 = (match shared_constructor_field11.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value38532, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38532.clone()
                                            }
                                        }).borrow().clone();
                                                    ml_reference_value2493
                                                };
                                                let ml_callback2492 = {
                                                    let identity = identity.clone();
                                                    std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> bool { ((match object.clone() {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value38546, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38546.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value38547, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38547.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value38548, _, _, _, _) => {
                                                source_ml_source2_data_property_value38548.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value38549, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38549.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value38550, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38550.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value38551, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38551.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value38552, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38552.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value38553, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38553.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value38554, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38554.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value38555, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38555.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value38556, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38556.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value38557, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38557.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value38558, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value38558.clone()
                                            }
                                        }) == identity) })
                                                };
                                                ml_values2492.iter().cloned().enumerate().any(
                                                    |(ml_index2492, ml_value2492)| {
                                                        ml_callback2492(ml_value2492)
                                                    },
                                                )
                                            })
                                        {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                                Box::new(shared_constructor_field11.clone()),
                                                self::ml_string::Text::from_units(vec![
                                                    83, 121, 109, 98, 111, 108, 32, 97, 108, 108,
                                                    111, 99, 97, 116, 105, 111, 110, 32, 114, 101,
                                                    113, 117, 105, 114, 101, 115, 32, 97, 110, 32,
                                                    117, 110, 117, 115, 101, 100, 32, 97, 99, 116,
                                                    117, 97, 108, 32, 112, 111, 115, 105, 116, 105,
                                                    118, 101, 32, 117, 110, 115, 105, 103, 110,
                                                    101, 100, 54, 52, 32, 103, 108, 111, 98, 97,
                                                    108, 32, 105, 100, 101, 110, 116, 105, 116,
                                                    121,
                                                ]),
                                            )
                                        } else {
                                            if (({
                                                let ml_reference_array2494 = match shared_constructor_field11.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value38563, _, _, _, _) => {
                                                    source_ml_source2_data_property_value38563.clone()
                                                }
                                            };
                                                let ml_reference_projection2494 =
                                                    ml_reference_array2494.borrow().len() as f64;
                                                ml_reference_projection2494
                                            }) >= (16384f64))
                                            {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 54, 51, 56, 52, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 112, 114, 111, 102, 105, 108, 101]))
                                            } else {
                                                {
                                                    let registry_key = ((pending_registry_key
                                                        .clone()
                                                        .unwrap_or_else(|| {
                                                            (None::<self::JavaScriptSemanticValue>,)
                                                        }))
                                                    .0)
                                                        .clone();
                                                    if !(match (registry_key.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }) {
                                                        {
                                                            let registered = self::allocate_java_script_registered_symbol(shared_constructor_field11.clone(), identity.clone(), registry_key.clone().expect("a proven present value"));
                                                            match registered.clone() {
                                                                self::JavaScriptSymbolRegistryTransition::SymbolRegistryValue(heap, value) => {
                                                                    let heap = *heap;
                                                                    let value = *value;
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register({
                                                                        let source_ml_source2_nominal_spread38572 = frame.clone().expect("a proven present value");
                                                                        match source_ml_source2_nominal_spread38572.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                                let shared_constructor_field6 = *shared_constructor_field6;
                                                                                {
                                                                                    let source_ml_source2_nominal_field38573 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field38575 = shared_constructor_field2;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field38577 = shared_constructor_field3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field38579 = shared_constructor_field4.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field38581 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field38583 = shared_constructor_field6.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field38585 = shared_constructor_field7.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field38587 = shared_constructor_field8.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field38589 = shared_constructor_field9.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field38591 = shared_constructor_field10.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override38593 = self::JavaScriptCallPending::CallIdle;
                                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field38573.clone(), source_ml_source2_nominal_field38575, source_ml_source2_nominal_field38577.clone(), source_ml_source2_nominal_field38579.clone(), source_ml_source2_nominal_field38581.clone(), Box::new(source_ml_source2_nominal_override38593.clone()), source_ml_source2_nominal_field38585.clone(), source_ml_source2_nominal_field38587.clone(), source_ml_source2_nominal_field38589.clone(), source_ml_source2_nominal_field38591.clone())
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
                                                                    }, destination, value.clone()))))
                                                                }
                                                                self::JavaScriptSymbolRegistryTransition::SymbolRegistryError(_, _, reason) => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), reason.clone())
                                                                }
                                                                self::JavaScriptSymbolRegistryTransition::SymbolRegistryInvalid(_, reason_2) => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), reason_2.clone())
                                                                }
                                                                self::JavaScriptSymbolRegistryTransition::SymbolRegistryResourceLimit(_, reason_3) => {
                                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_3.clone())
                                                                }
                                                                self::JavaScriptSymbolRegistryTransition::SymbolRegistryAllocate(_, _) => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            let allocation = self::register_java_script_ordinary_symbol(shared_constructor_field11.clone(), identity.clone());
                                                            if !((({
                                                                let ml_reference_value2495 =
                                                                    allocation.borrow().clone();
                                                                ml_reference_value2495
                                                            })
                                                            .0)
                                                                .2)
                                                            {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), (({ let ml_reference_value2496 = allocation.borrow().clone(); ml_reference_value2496 }).0).0)
                                                            } else {
                                                                {
                                                                    let heap_2 = self::retain_java_script_symbol_registration((({ let ml_reference_value2497 = allocation.borrow().clone(); ml_reference_value2497 }).0).1, self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(identity.clone(), false, None::<(Option<self::JavaScriptSemanticValue>,)>));
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register({
                                                                        let source_ml_source2_nominal_spread38602 = frame.clone().expect("a proven present value");
                                                                        match source_ml_source2_nominal_spread38602.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2, shared_constructor_field7_2, shared_constructor_field8_2, shared_constructor_field9_2, shared_constructor_field10_2) => {
                                                                                let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field38603 = shared_constructor_field1_2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field38605 = shared_constructor_field2_2;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field38607 = shared_constructor_field3_2.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field38609 = shared_constructor_field4_2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field38611 = shared_constructor_field5_2.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field38613 = shared_constructor_field6_2.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field38615 = shared_constructor_field7_2.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field38617 = shared_constructor_field8_2.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field38619 = shared_constructor_field9_2.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field38621 = shared_constructor_field10_2.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override38623 = self::JavaScriptCallPending::CallIdle;
                                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field38603.clone(), source_ml_source2_nominal_field38605, source_ml_source2_nominal_field38607.clone(), source_ml_source2_nominal_field38609.clone(), source_ml_source2_nominal_field38611.clone(), Box::new(source_ml_source2_nominal_override38623.clone()), source_ml_source2_nominal_field38615.clone(), source_ml_source2_nominal_field38617.clone(), source_ml_source2_nominal_field38619.clone(), source_ml_source2_nominal_field38621.clone())
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
                                                                    }, destination, self::JavaScriptSemanticValue::Symbol(identity.clone())))))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field11.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99,
                                            97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105,
                                            114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                            97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101,
                                            100, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110,
                                            115, 116, 114, 117, 99, 116, 105, 111, 110,
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_primitive_wrapper(
    machine: self::JavaScriptCallMachine,
    destination: f64,
    transition: self::JavaScriptPrimitiveWrapperTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
            let frame = {
                let source_ml_source2_array_read_value38627 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_array_read_index38628 = ({
                        let ml_reference_array2498 = shared_constructor_field1.clone();
                        let ml_reference_projection2498 =
                            ml_reference_array2498.borrow().len() as f64;
                        ml_reference_projection2498
                    } - 1f64);
                    {
                        let ml_reference_array2499 =
                            source_ml_source2_array_read_value38627.clone();
                        let ml_reference_position2499 = source_ml_source2_array_read_index38628;
                        let ml_reference_projection2499 = self::ml_array::optional_at(
                            &*ml_reference_array2499.borrow(),
                            self::ml_array::number_index(ml_reference_position2499),
                        );
                        ml_reference_projection2499
                    }
                }
            };
            if match (frame.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                        self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperAllocate(source_ml_source2_data_property_value38629, _, _) => {
                            let source_ml_source2_data_property_value38629 = *source_ml_source2_data_property_value38629;
                            source_ml_source2_data_property_value38629.clone()
                        }
                        self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperCompleted(source_ml_source2_data_property_value38630, _) => {
                            let source_ml_source2_data_property_value38630 = *source_ml_source2_data_property_value38630;
                            source_ml_source2_data_property_value38630.clone()
                        }
                        self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(source_ml_source2_data_property_value38631, _) => {
                            let source_ml_source2_data_property_value38631 = *source_ml_source2_data_property_value38631;
                            source_ml_source2_data_property_value38631.clone()
                        }
                        self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperResourceLimit(source_ml_source2_data_property_value38632, _, _) => {
                            let source_ml_source2_data_property_value38632 = *source_ml_source2_data_property_value38632;
                            source_ml_source2_data_property_value38632.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101]))
            } else {
                match transition.clone() {
                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperAllocate(
                        heap,
                        state,
                        prototype,
                    ) => {
                        let heap = *heap;
                        let state = *state;
                        let prototype = *prototype;
                        self::JavaScriptCallMachineTransition::MachineAllocateObject(
                            Box::new(self::replace_java_script_call_frame(
                                heap.clone(),
                                shared_constructor_field1.clone(),
                                self::replace_java_script_call_pending(
                                    frame.clone().expect("a proven present value"),
                                    self::JavaScriptCallPending::WaitPrimitiveWrapper(
                                        destination,
                                        Box::new(state.clone()),
                                    ),
                                ),
                            )),
                            Box::new(prototype.clone()),
                        )
                    }
                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperCompleted(
                        heap_2,
                        value,
                    ) => {
                        let heap_2 = *heap_2;
                        let value = *value;
                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                            self::replace_java_script_call_frame(
                                heap_2.clone(),
                                shared_constructor_field1.clone(),
                                self::write_java_script_call_register(
                                    frame.clone().expect("a proven present value"),
                                    destination,
                                    value.clone(),
                                ),
                            ),
                        ))
                    }
                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(
                        heap_3,
                        reason,
                    ) => {
                        let heap_3 = *heap_3;
                        self::JavaScriptCallMachineTransition::MachineInvalid(
                            Box::new(heap_3.clone()),
                            reason.clone(),
                        )
                    }
                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperResourceLimit(
                        heap_4,
                        state_2,
                        reason_2,
                    ) => {
                        let heap_4 = *heap_4;
                        let state_2 = *state_2;
                        self::JavaScriptCallMachineTransition::MachineResourceLimit(
                            Box::new(self::replace_java_script_call_frame(
                                heap_4.clone(),
                                shared_constructor_field1.clone(),
                                self::replace_java_script_call_pending(
                                    frame.clone().expect("a proven present value"),
                                    self::JavaScriptCallPending::WaitPrimitiveWrapper(
                                        destination,
                                        Box::new(state_2.clone()),
                                    ),
                                ),
                            )),
                            reason_2.clone(),
                        )
                    }
                }
            }
        }
    }
}
