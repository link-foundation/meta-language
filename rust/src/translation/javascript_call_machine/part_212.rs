// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallCompletionTransition;
use super::JavaScriptCallFrame;
use super::JavaScriptCallIteratorOperation;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptIteratorRegionsUpdate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Result;
use super::accept_java_script_call_array_creation;
use super::accept_java_script_call_array_spread;
use super::accept_java_script_call_object_from_entries;
use super::finish_java_script_call_iterator_protocol;
use super::finish_java_script_call_return;
use super::insert_java_script_iterator_region;
use super::ml_array;
use super::ml_string;
use super::provide_java_script_array_creation_primitive_prototype;
use super::provide_java_script_array_spread_primitive_prototype;
use super::provide_java_script_iterator_primitive_prototype;
use super::provide_java_script_object_from_entries_primitive_prototype;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::remove_java_script_iterator_region;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::source_ml_source1_temporary_read_java_script_call_iterator_close_completion_loop1533;
use super::step_java_script_call_machine;
use super::update_java_script_iterator_region;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_primitive_iterator_prototype(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    prototype: self::JavaScriptSemanticValue,
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
                            let source_ml_source2_array_read_value37507 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index37508 = ({
                                    let ml_reference_array2452 = shared_constructor_field2.clone();
                                    let ml_reference_projection2452 =
                                        ml_reference_array2452.borrow().len() as f64;
                                    ml_reference_projection2452
                                } - 1f64);
                                {
                                    let ml_reference_array2453 =
                                        source_ml_source2_array_read_value37507.clone();
                                    let ml_reference_position2453 =
                                        source_ml_source2_array_read_index37508;
                                    let ml_reference_projection2453 = self::ml_array::optional_at(
                                        &*ml_reference_array2453.borrow(),
                                        self::ml_array::number_index(ml_reference_position2453),
                                    );
                                    ml_reference_projection2453
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
                                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111,
                                    116, 111, 116, 121, 112, 101, 32, 114, 101, 115, 112, 111, 110,
                                    115, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                    115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112,
                                    101, 110, 100, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111,
                                    114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110,
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
                                        source_ml_source2_data_property_value37510,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value37510 =
                                            *source_ml_source2_data_property_value37510;
                                        source_ml_source2_data_property_value37510.clone()
                                    }
                                };
                                if (((match pending.clone() {
                                    self::JavaScriptCallPending::WaitArraySpread(_, _, _) => false,
                                    _ => true,
                                } && match pending.clone() {
                                    self::JavaScriptCallPending::WaitIteratorProtocol(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => false,
                                    _ => true,
                                }) && match pending.clone() {
                                    self::JavaScriptCallPending::WaitObjectFromEntries(
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => false,
                                    _ => true,
                                }) && match pending.clone() {
                                    self::JavaScriptCallPending::WaitArrayCreation(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => false,
                                    _ => true,
                                }) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112,
                                            114, 111, 116, 111, 116, 121, 112, 101, 32, 114, 101,
                                            115, 112, 111, 110, 115, 101, 32, 114, 101, 116, 97,
                                            105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                            97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101,
                                            100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97,
                                            99, 113, 117, 105, 115, 105, 116, 105, 111, 110,
                                        ]),
                                    )
                                } else {
                                    {
                                        let request = self::step_java_script_call_machine(
                                            program.clone(),
                                            machine.clone(),
                                        );
                                        match request.clone() {
                                            self::JavaScriptCallMachineTransition::MachineNeedPrimitiveIteratorGet(_, _, _) => {
                                                match pending.clone() {
                                                    self::JavaScriptCallPending::WaitIteratorProtocol(_, _, _, _, _, state) => {
                                                        let state = *state;
                                                        self::accept_java_script_call_iterator_protocol(program.clone(), machine.clone(), self::project_java_script_call_iterator_operation(pending.clone()), self::provide_java_script_iterator_primitive_prototype(shared_constructor_field1.clone(), state.clone(), prototype.clone()))
                                                    }
                                                    self::JavaScriptCallPending::WaitArraySpread(destination, realm, state_2) => {
                                                        let state_2 = *state_2;
                                                        self::accept_java_script_call_array_spread(program.clone(), machine.clone(), destination, realm.clone(), self::provide_java_script_array_spread_primitive_prototype(shared_constructor_field1.clone(), state_2.clone(), prototype.clone()))
                                                    }
                                                    self::JavaScriptCallPending::WaitArrayCreation(destination_2, realm_2, state_3, _, _, _) => {
                                                        let state_3 = *state_3;
                                                        self::accept_java_script_call_array_creation(program.clone(), machine.clone(), destination_2, realm_2.clone(), self::provide_java_script_array_creation_primitive_prototype(shared_constructor_field1.clone(), state_3.clone(), prototype.clone()))
                                                    }
                                                    self::JavaScriptCallPending::WaitObjectFromEntries(destination_3, realm_3, state_4, _) => {
                                                        let state_4 = *state_4;
                                                        self::accept_java_script_call_object_from_entries(program.clone(), machine.clone(), destination_3, realm_3.clone(), self::provide_java_script_object_from_entries_primitive_prototype(shared_constructor_field1.clone(), state_4.clone(), prototype.clone()))
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 114, 101, 115, 112, 111, 110, 115, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110]))
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 114, 101, 115, 112, 111, 110, 115, 101, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 32, 108, 105, 118, 101, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108]))
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
pub fn accept_java_script_call_completion(
    program: self::JavaScriptCallProgram,
    transition: self::JavaScriptCallCompletionTransition,
) -> self::JavaScriptCallMachineTransition {
    match transition.clone() {
        self::JavaScriptCallCompletionTransition::CallReturn(machine, value) => {
            let machine = *machine;
            let value = *value;
            self::finish_java_script_call_return(program.clone(), machine.clone(), value.clone())
        }
        _ => self::project_java_script_call_completion_transition(transition.clone()),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_java_script_call_completion_transition(
    transition: self::JavaScriptCallCompletionTransition,
) -> self::JavaScriptCallMachineTransition {
    match transition.clone() {
        self::JavaScriptCallCompletionTransition::MachineContinue(machine) => {
            let machine = *machine;
            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(machine.clone()))
        }
        self::JavaScriptCallCompletionTransition::MachineThrow(heap, value) => {
            let heap = *heap;
            let value = *value;
            self::JavaScriptCallMachineTransition::MachineThrow(
                Box::new(heap.clone()),
                Box::new(value.clone()),
            )
        }
        self::JavaScriptCallCompletionTransition::MachineInvalid(heap_2, reason) => {
            let heap_2 = *heap_2;
            self::JavaScriptCallMachineTransition::MachineInvalid(
                Box::new(heap_2.clone()),
                reason.clone(),
            )
        }
        self::JavaScriptCallCompletionTransition::MachineResourceLimit(machine_2, reason_2) => {
            let machine_2 = *machine_2;
            self::JavaScriptCallMachineTransition::MachineResourceLimit(
                Box::new(machine_2.clone()),
                reason_2.clone(),
            )
        }
        self::JavaScriptCallCompletionTransition::CallReturn(machine_3, _) => {
            let machine_3 = *machine_3;
            self::JavaScriptCallMachineTransition::MachineInvalid(
                Box::new(match machine_3.clone() {
                    self::JavaScriptCallMachine::CallMachine(
                        source_ml_source2_data_property_value37550,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value37550 =
                            *source_ml_source2_data_property_value37550;
                        source_ml_source2_data_property_value37550.clone()
                    }
                }),
                self::ml_string::Text::from_units(vec![
                    116, 104, 114, 111, 119, 32, 117, 110, 119, 105, 110, 100, 105, 110, 103, 32,
                    99, 97, 110, 110, 111, 116, 32, 98, 101, 99, 111, 109, 101, 32, 97, 32, 114,
                    101, 116, 117, 114, 110,
                ]),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_iterator_close_completion(
    frame: self::JavaScriptCallFrame,
) -> self::JavaScriptIteratorCompletion {
    {
        let at = ({
            let ml_reference_array2454 = match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value37551,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value37551.clone(),
            };
            let ml_reference_projection2454 = ml_reference_array2454.borrow().len() as f64;
            ml_reference_projection2454
        } - 1f64);
        {
            let source_ml_source2_s37552 = self::source_ml_source1_temporary_read_java_script_call_iterator_close_completion_loop1533(frame.clone(), at);
            match source_ml_source2_s37552.clone() {
                self::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Result::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Done => {
                    self::JavaScriptIteratorCompletion::IteratorCompletion(self::ml_string::Text::from_units(vec![110, 111, 114, 109, 97, 108]), Box::new(self::JavaScriptSemanticValue::Undefined), (-1f64))
                }
                self::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Result::MlSource1TemporaryReadJavaScriptCallIteratorCloseCompletionLoop1533Return(source_ml_source1_temporary_result1533) => {
                    let source_ml_source1_temporary_result1533 = *source_ml_source1_temporary_result1533;
                    source_ml_source1_temporary_result1533.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn write_java_script_call_iterator_result(
    frame: self::JavaScriptCallFrame,
    destination: f64,
    done: f64,
    value: self::JavaScriptSemanticValue,
    finished: bool,
) -> self::JavaScriptCallFrame {
    self::write_java_script_call_register(
        self::write_java_script_call_register(frame.clone(), destination, value.clone()),
        done,
        self::JavaScriptSemanticValue::Boolean(finished),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn update_java_script_call_iterator_protocol_regions(
    regions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>,
    slot: f64,
    operation: &self::ml_string::Text,
    transition: self::JavaScriptIteratorProtocolTransition,
) -> self::JavaScriptIteratorRegionsUpdate {
    match transition.clone() {
        self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(
            transition_heap,
            _,
        ) => {
            let transition_heap = *transition_heap;
            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                self::remove_java_script_iterator_region(
                    transition_heap.clone(),
                    regions.clone(),
                    slot,
                )
            } else {
                self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions.clone())
            }
        }
        self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(
            transition_heap_2,
            _,
            _,
            _,
            _,
        ) => {
            let transition_heap_2 = *transition_heap_2;
            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                self::remove_java_script_iterator_region(
                    transition_heap_2.clone(),
                    regions.clone(),
                    slot,
                )
            } else {
                self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions.clone())
            }
        }
        self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(
            transition_heap_3,
            _,
            _,
            _,
        ) => {
            let transition_heap_3 = *transition_heap_3;
            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                self::remove_java_script_iterator_region(
                    transition_heap_3.clone(),
                    regions.clone(),
                    slot,
                )
            } else {
                self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions.clone())
            }
        }
        self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(heap, record) => {
            let heap = *heap;
            let record = *record;
            self::insert_java_script_iterator_region(
                heap.clone(),
                regions.clone(),
                slot,
                record.clone(),
            )
        }
        self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(
            transition_heap_4,
            source_ml_source2_f37561,
            _,
            _,
        ) => {
            let transition_heap_4 = *transition_heap_4;
            let source_ml_source2_f37561 = *source_ml_source2_f37561;
            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                self::remove_java_script_iterator_region(
                    transition_heap_4.clone(),
                    regions.clone(),
                    slot,
                )
            } else {
                self::update_java_script_iterator_region(
                    transition_heap_4.clone(),
                    regions.clone(),
                    slot,
                    source_ml_source2_f37561.clone(),
                )
            }
        }
        self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(
            transition_heap_5,
            _,
            _,
        ) => {
            let transition_heap_5 = *transition_heap_5;
            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                self::remove_java_script_iterator_region(
                    transition_heap_5.clone(),
                    regions.clone(),
                    slot,
                )
            } else {
                self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions.clone())
            }
        }
        self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(
            transition_heap_6,
            source_ml_source2_f37566,
            _,
        ) => {
            let transition_heap_6 = *transition_heap_6;
            let source_ml_source2_f37566 = *source_ml_source2_f37566;
            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                self::remove_java_script_iterator_region(
                    transition_heap_6.clone(),
                    regions.clone(),
                    slot,
                )
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) {
                    self::update_java_script_iterator_region(
                        transition_heap_6.clone(),
                        regions.clone(),
                        slot,
                        match source_ml_source2_f37566.clone() {
                            self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value37602,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value37602 =
                                    *source_ml_source2_data_property_value37602;
                                source_ml_source2_data_property_value37602.clone()
                            }
                        },
                    )
                } else {
                    self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions.clone())
                }
            }
        }
        self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(
            transition_heap_7,
            source_ml_source2_f37568,
            _,
            _,
        ) => {
            let transition_heap_7 = *transition_heap_7;
            let source_ml_source2_f37568 = *source_ml_source2_f37568;
            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                self::remove_java_script_iterator_region(
                    transition_heap_7.clone(),
                    regions.clone(),
                    slot,
                )
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) {
                    self::update_java_script_iterator_region(
                        transition_heap_7.clone(),
                        regions.clone(),
                        slot,
                        match source_ml_source2_f37568.clone() {
                            self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value37608,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value37608 =
                                    *source_ml_source2_data_property_value37608;
                                source_ml_source2_data_property_value37608.clone()
                            }
                        },
                    )
                } else {
                    self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions.clone())
                }
            }
        }
        self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
            transition_heap_8,
            _,
        ) => {
            let transition_heap_8 = *transition_heap_8;
            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                self::remove_java_script_iterator_region(
                    transition_heap_8.clone(),
                    regions.clone(),
                    slot,
                )
            } else {
                self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_java_script_call_iterator_operation(
    pending: self::JavaScriptCallPending,
) -> Option<self::JavaScriptCallIteratorOperation> {
    match pending.clone() {
        self::JavaScriptCallPending::WaitIteratorProtocol(
            destination,
            done,
            slot,
            operation,
            realm,
            _,
        ) => Some(self::JavaScriptCallIteratorOperation::IteratorOperation(
            destination,
            done,
            slot,
            operation.clone(),
            realm.clone(),
        )),
        _ => None::<self::JavaScriptCallIteratorOperation>,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_iterator_protocol(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    pending: Option<self::JavaScriptCallIteratorOperation>,
    transition: self::JavaScriptIteratorProtocolTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
            let frame = {
                let source_ml_source2_array_read_value37617 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_array_read_index37618 = ({
                        let ml_reference_array2455 = shared_constructor_field1.clone();
                        let ml_reference_projection2455 =
                            ml_reference_array2455.borrow().len() as f64;
                        ml_reference_projection2455
                    } - 1f64);
                    {
                        let ml_reference_array2456 =
                            source_ml_source2_array_read_value37617.clone();
                        let ml_reference_position2456 = source_ml_source2_array_read_index37618;
                        let ml_reference_projection2456 = self::ml_array::optional_at(
                            &*ml_reference_array2456.borrow(),
                            self::ml_array::number_index(ml_reference_position2456),
                        );
                        ml_reference_projection2456
                    }
                }
            };
            if ((match (frame.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } || match (pending.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) || match pending.clone().expect("a proven present value") {
                self::JavaScriptCallIteratorOperation::IteratorOperation(_, _, _, _, _) => false,
            }) {
                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(source_ml_source2_data_property_value37624, _) => {
                            let source_ml_source2_data_property_value37624 = *source_ml_source2_data_property_value37624;
                            source_ml_source2_data_property_value37624.clone()
                        }
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(source_ml_source2_data_property_value37625, _, _, _, _) => {
                            let source_ml_source2_data_property_value37625 = *source_ml_source2_data_property_value37625;
                            source_ml_source2_data_property_value37625.clone()
                        }
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(source_ml_source2_data_property_value37626, _, _, _) => {
                            let source_ml_source2_data_property_value37626 = *source_ml_source2_data_property_value37626;
                            source_ml_source2_data_property_value37626.clone()
                        }
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(source_ml_source2_data_property_value37627, _) => {
                            let source_ml_source2_data_property_value37627 = *source_ml_source2_data_property_value37627;
                            source_ml_source2_data_property_value37627.clone()
                        }
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(source_ml_source2_data_property_value37628, _, _, _) => {
                            let source_ml_source2_data_property_value37628 = *source_ml_source2_data_property_value37628;
                            source_ml_source2_data_property_value37628.clone()
                        }
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(source_ml_source2_data_property_value37629, _, _) => {
                            let source_ml_source2_data_property_value37629 = *source_ml_source2_data_property_value37629;
                            source_ml_source2_data_property_value37629.clone()
                        }
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(source_ml_source2_data_property_value37630, _, _) => {
                            let source_ml_source2_data_property_value37630 = *source_ml_source2_data_property_value37630;
                            source_ml_source2_data_property_value37630.clone()
                        }
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(source_ml_source2_data_property_value37631, _, _, _) => {
                            let source_ml_source2_data_property_value37631 = *source_ml_source2_data_property_value37631;
                            source_ml_source2_data_property_value37631.clone()
                        }
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(source_ml_source2_data_property_value37632, _) => {
                            let source_ml_source2_data_property_value37632 = *source_ml_source2_data_property_value37632;
                            source_ml_source2_data_property_value37632.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
            } else {
                {
                    let current = self::JavaScriptCallMachine::CallMachine(Box::new(match transition.clone() {
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(source_ml_source2_data_property_value37633, _) => {
                                let source_ml_source2_data_property_value37633 = *source_ml_source2_data_property_value37633;
                                source_ml_source2_data_property_value37633.clone()
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(source_ml_source2_data_property_value37634, _, _, _, _) => {
                                let source_ml_source2_data_property_value37634 = *source_ml_source2_data_property_value37634;
                                source_ml_source2_data_property_value37634.clone()
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(source_ml_source2_data_property_value37635, _, _, _) => {
                                let source_ml_source2_data_property_value37635 = *source_ml_source2_data_property_value37635;
                                source_ml_source2_data_property_value37635.clone()
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(source_ml_source2_data_property_value37636, _) => {
                                let source_ml_source2_data_property_value37636 = *source_ml_source2_data_property_value37636;
                                source_ml_source2_data_property_value37636.clone()
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(source_ml_source2_data_property_value37637, _, _, _) => {
                                let source_ml_source2_data_property_value37637 = *source_ml_source2_data_property_value37637;
                                source_ml_source2_data_property_value37637.clone()
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(source_ml_source2_data_property_value37638, _, _) => {
                                let source_ml_source2_data_property_value37638 = *source_ml_source2_data_property_value37638;
                                source_ml_source2_data_property_value37638.clone()
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(source_ml_source2_data_property_value37639, _, _) => {
                                let source_ml_source2_data_property_value37639 = *source_ml_source2_data_property_value37639;
                                source_ml_source2_data_property_value37639.clone()
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(source_ml_source2_data_property_value37640, _, _, _) => {
                                let source_ml_source2_data_property_value37640 = *source_ml_source2_data_property_value37640;
                                source_ml_source2_data_property_value37640.clone()
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(source_ml_source2_data_property_value37641, _) => {
                                let source_ml_source2_data_property_value37641 = *source_ml_source2_data_property_value37641;
                                source_ml_source2_data_property_value37641.clone()
                            }
                        }), shared_constructor_field1.clone());
                    if ((match transition.clone() {
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(_, _) => {
                                true
                            }
                            _ => {
                                false
                            }
                        } || match transition.clone() {
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(_, _, _, _, _) => {
                                true
                            }
                            _ => {
                                false
                            }
                        }) || match transition.clone() {
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(_, _, _, _) => {
                                true
                            }
                            _ => {
                                false
                            }
                        }) {
                            {
                                let next = self::replace_java_script_call_frame((((match transition.clone() {
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(source_ml_source2_data_property_value37654, _) => {
                                        let source_ml_source2_data_property_value37654 = *source_ml_source2_data_property_value37654;
                                        Some((source_ml_source2_data_property_value37654.clone(),))
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(source_ml_source2_data_property_value37655, _, _, _, _) => {
                                        let source_ml_source2_data_property_value37655 = *source_ml_source2_data_property_value37655;
                                        Some((source_ml_source2_data_property_value37655.clone(),))
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(source_ml_source2_data_property_value37656, _, _, _) => {
                                        let source_ml_source2_data_property_value37656 = *source_ml_source2_data_property_value37656;
                                        Some((source_ml_source2_data_property_value37656.clone(),))
                                    }
                                    _ => {
                                        None::<(self::JavaScriptOrdinaryHeap,)>
                                    }
                                }).clone().expect("a proven present value")).0).clone(), match current.clone() {
                                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value37657) => {
                                        source_ml_source2_data_property_value37657.clone()
                                    }
                                }, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitIteratorProtocol(match pending.clone().expect("a proven present value") {
                                    self::JavaScriptCallIteratorOperation::IteratorOperation(source_ml_source2_data_property_value37658, _, _, _, _) => {
                                        source_ml_source2_data_property_value37658
                                    }
                                }, match pending.clone().expect("a proven present value") {
                                    self::JavaScriptCallIteratorOperation::IteratorOperation(_, source_ml_source2_data_property_value37659, _, _, _) => {
                                        source_ml_source2_data_property_value37659
                                    }
                                }, match pending.clone().expect("a proven present value") {
                                    self::JavaScriptCallIteratorOperation::IteratorOperation(_, _, source_ml_source2_data_property_value37660, _, _) => {
                                        source_ml_source2_data_property_value37660
                                    }
                                }, match pending.clone().expect("a proven present value") {
                                    self::JavaScriptCallIteratorOperation::IteratorOperation(_, _, _, source_ml_source2_data_property_value37661, _) => {
                                        source_ml_source2_data_property_value37661.clone()
                                    }
                                }, match pending.clone().expect("a proven present value") {
                                    self::JavaScriptCallIteratorOperation::IteratorOperation(_, _, _, _, source_ml_source2_data_property_value37662) => {
                                        source_ml_source2_data_property_value37662.clone()
                                    }
                                }, Box::new((((match transition.clone() {
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(_, source_ml_source2_data_property_value37663) => {
                                        let source_ml_source2_data_property_value37663 = *source_ml_source2_data_property_value37663;
                                        Some((source_ml_source2_data_property_value37663.clone(),))
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(_, source_ml_source2_data_property_value37664, _, _, _) => {
                                        let source_ml_source2_data_property_value37664 = *source_ml_source2_data_property_value37664;
                                        Some((source_ml_source2_data_property_value37664.clone(),))
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(_, source_ml_source2_data_property_value37665, _, _) => {
                                        let source_ml_source2_data_property_value37665 = *source_ml_source2_data_property_value37665;
                                        Some((source_ml_source2_data_property_value37665.clone(),))
                                    }
                                    _ => {
                                        None::<(self::JavaScriptIteratorProtocolState,)>
                                    }
                                }).clone().expect("a proven present value")).0).clone()))));
                                match transition.clone() {
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(_, _, callee, receiver, arguments) => {
                                        let callee = *callee;
                                        let receiver = *receiver;
                                        self::push_java_script_call(program.clone(), next.clone(), callee.clone(), receiver.clone(), arguments.clone())
                                    }
                                    _ => {
                                        match transition.clone() {
                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(_, _, receiver_2, key) => {
                                                let receiver_2 = *receiver_2;
                                                let key = *key;
                                                self::JavaScriptCallMachineTransition::MachineNeedPrimitiveIteratorGet(Box::new(next.clone()), Box::new(receiver_2.clone()), Box::new(key.clone()))
                                            }
                                            _ => {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(next.clone()))
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::finish_java_script_call_iterator_protocol(current.clone(), frame.clone().expect("a proven present value"), pending.clone().expect("a proven present value"), transition.clone())
                        }
                }
            }
        }
    }
}
