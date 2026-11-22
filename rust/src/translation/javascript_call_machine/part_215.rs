// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptInstanceOfTransition;
use super::JavaScriptPropertyDefinitionsTransition;
use super::JavaScriptPropertyReflectionTransition;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_object_primitive_prototype(
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
                let error = {
                    let source_ml_source2_logical38321 =
                        self::read_java_script_call_machine_error(program.clone(), machine.clone());
                    if !source_ml_source2_logical38321.0.is_empty() {
                        source_ml_source2_logical38321.clone()
                    } else {
                        self::read_java_script_ordinary_value_error(
                            shared_constructor_field1.clone(),
                            prototype.clone(),
                        )
                    }
                };
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let frame = {
                            let source_ml_source2_array_read_value38325 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index38326 = ({
                                    let ml_reference_array2477 = shared_constructor_field2.clone();
                                    let ml_reference_projection2477 =
                                        ml_reference_array2477.borrow().len() as f64;
                                    ml_reference_projection2477
                                } - 1f64);
                                {
                                    let ml_reference_array2478 =
                                        source_ml_source2_array_read_value38325.clone();
                                    let ml_reference_position2478 =
                                        source_ml_source2_array_read_index38326;
                                    let ml_reference_projection2478 = self::ml_array::optional_at(
                                        &*ml_reference_array2478.borrow(),
                                        self::ml_array::number_index(ml_reference_position2478),
                                    );
                                    ml_reference_projection2478
                                }
                            }
                        };
                        if (((match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || {
                            let source_ml_source2_s38328 =
                                match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value38332,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value38332 =
                                            *source_ml_source2_data_property_value38332;
                                        source_ml_source2_data_property_value38332.clone()
                                    }
                                };
                            match source_ml_source2_s38328.clone() {
                                self::JavaScriptCallPending::WaitObjectPrototype(_, _, _) => false,
                                _ => true,
                            }
                        }) || match prototype.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }) || ((match prototype.clone() {
                            self::JavaScriptSemanticValue::Object(
                                _,
                                source_ml_source2_data_property_value38335,
                            ) => Some((source_ml_source2_data_property_value38335,)),
                            _ => None::<(bool,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0)
                        {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111,
                                    116, 111, 116, 121, 112, 101, 32, 114, 101, 115, 112, 111, 110,
                                    115, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                                    116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110,
                                    100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 111,
                                    112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32,
                                    105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 79, 98, 106,
                                    101, 99, 116, 32, 114, 111, 111, 116,
                                ]),
                            )
                        } else {
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), ((match match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value38339, _, _, _, _) => {
                                    let source_ml_source2_data_property_value38339 = *source_ml_source2_data_property_value38339;
                                    source_ml_source2_data_property_value38339.clone()
                                }
                            } {
                                self::JavaScriptCallPending::WaitObjectPrototype(source_ml_source2_data_property_value38340, _, _) => {
                                    Some((source_ml_source2_data_property_value38340,))
                                }
                                _ => {
                                    None::<(f64,)>
                                }
                            }).clone().expect("a proven present value")).0, prototype.clone()))))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_instance_of(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptInstanceOfTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field2) => {
            let frame = {
                let source_ml_source2_array_read_value38343 = shared_constructor_field2.clone();
                {
                    let source_ml_source2_array_read_index38344 = ({
                        let ml_reference_array2479 = shared_constructor_field2.clone();
                        let ml_reference_projection2479 =
                            ml_reference_array2479.borrow().len() as f64;
                        ml_reference_projection2479
                    } - 1f64);
                    {
                        let ml_reference_array2480 =
                            source_ml_source2_array_read_value38343.clone();
                        let ml_reference_position2480 = source_ml_source2_array_read_index38344;
                        let ml_reference_projection2480 = self::ml_array::optional_at(
                            &*ml_reference_array2480.borrow(),
                            self::ml_array::number_index(ml_reference_position2480),
                        );
                        ml_reference_projection2480
                    }
                }
            };
            {
                let current = self::JavaScriptCallMachine::CallMachine(
                    Box::new(match transition.clone() {
                        self::JavaScriptInstanceOfTransition::InstanceOfContinue(
                            source_ml_source2_data_property_value38345,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38345 =
                                *source_ml_source2_data_property_value38345;
                            source_ml_source2_data_property_value38345.clone()
                        }
                        self::JavaScriptInstanceOfTransition::InstanceOfCall(
                            source_ml_source2_data_property_value38346,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38346 =
                                *source_ml_source2_data_property_value38346;
                            source_ml_source2_data_property_value38346.clone()
                        }
                        self::JavaScriptInstanceOfTransition::InstanceOfCompleted(
                            source_ml_source2_data_property_value38347,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38347 =
                                *source_ml_source2_data_property_value38347;
                            source_ml_source2_data_property_value38347.clone()
                        }
                        self::JavaScriptInstanceOfTransition::InstanceOfThrow(
                            source_ml_source2_data_property_value38348,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38348 =
                                *source_ml_source2_data_property_value38348;
                            source_ml_source2_data_property_value38348.clone()
                        }
                        self::JavaScriptInstanceOfTransition::InstanceOfError(
                            source_ml_source2_data_property_value38349,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38349 =
                                *source_ml_source2_data_property_value38349;
                            source_ml_source2_data_property_value38349.clone()
                        }
                        self::JavaScriptInstanceOfTransition::InstanceOfInvalid(
                            source_ml_source2_data_property_value38350,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38350 =
                                *source_ml_source2_data_property_value38350;
                            source_ml_source2_data_property_value38350.clone()
                        }
                        self::JavaScriptInstanceOfTransition::InstanceOfResourceLimit(
                            source_ml_source2_data_property_value38351,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38351 =
                                *source_ml_source2_data_property_value38351;
                            source_ml_source2_data_property_value38351.clone()
                        }
                    }),
                    shared_constructor_field2.clone(),
                );
                match current.clone() {
                    self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                        if match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                                    self::JavaScriptInstanceOfTransition::InstanceOfContinue(source_ml_source2_data_property_value38353, _) => {
                                        let source_ml_source2_data_property_value38353 = *source_ml_source2_data_property_value38353;
                                        source_ml_source2_data_property_value38353.clone()
                                    }
                                    self::JavaScriptInstanceOfTransition::InstanceOfCall(source_ml_source2_data_property_value38354, _, _, _, _) => {
                                        let source_ml_source2_data_property_value38354 = *source_ml_source2_data_property_value38354;
                                        source_ml_source2_data_property_value38354.clone()
                                    }
                                    self::JavaScriptInstanceOfTransition::InstanceOfCompleted(source_ml_source2_data_property_value38355, _) => {
                                        let source_ml_source2_data_property_value38355 = *source_ml_source2_data_property_value38355;
                                        source_ml_source2_data_property_value38355.clone()
                                    }
                                    self::JavaScriptInstanceOfTransition::InstanceOfThrow(source_ml_source2_data_property_value38356, _) => {
                                        let source_ml_source2_data_property_value38356 = *source_ml_source2_data_property_value38356;
                                        source_ml_source2_data_property_value38356.clone()
                                    }
                                    self::JavaScriptInstanceOfTransition::InstanceOfError(source_ml_source2_data_property_value38357, _, _) => {
                                        let source_ml_source2_data_property_value38357 = *source_ml_source2_data_property_value38357;
                                        source_ml_source2_data_property_value38357.clone()
                                    }
                                    self::JavaScriptInstanceOfTransition::InstanceOfInvalid(source_ml_source2_data_property_value38358, _) => {
                                        let source_ml_source2_data_property_value38358 = *source_ml_source2_data_property_value38358;
                                        source_ml_source2_data_property_value38358.clone()
                                    }
                                    self::JavaScriptInstanceOfTransition::InstanceOfResourceLimit(source_ml_source2_data_property_value38359, _, _) => {
                                        let source_ml_source2_data_property_value38359 = *source_ml_source2_data_property_value38359;
                                        source_ml_source2_data_property_value38359.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 117, 114, 114, 101, 110, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            match transition.clone() {
                                self::JavaScriptInstanceOfTransition::InstanceOfContinue(
                                    heap,
                                    state,
                                ) => {
                                    let heap = *heap;
                                    let state = *state;
                                    {
                                        let next = self::replace_java_script_call_frame(
                                            heap.clone(),
                                            shared_constructor_field1.clone(),
                                            self::replace_java_script_call_pending(
                                                frame.clone().expect("a proven present value"),
                                                self::JavaScriptCallPending::WaitInstanceOf(
                                                    destination,
                                                    realm.clone(),
                                                    Box::new(state.clone()),
                                                ),
                                            ),
                                        );
                                        self::JavaScriptCallMachineTransition::MachineContinue(
                                            Box::new(next.clone()),
                                        )
                                    }
                                }
                                self::JavaScriptInstanceOfTransition::InstanceOfCall(
                                    heap_2,
                                    state_2,
                                    source_ml_source2_f38360,
                                    source_ml_source2_f38361,
                                    source_ml_source2_f38362,
                                ) => {
                                    let heap_2 = *heap_2;
                                    let state_2 = *state_2;
                                    let source_ml_source2_f38360 = *source_ml_source2_f38360;
                                    let source_ml_source2_f38361 = *source_ml_source2_f38361;
                                    {
                                        let next_2 = self::replace_java_script_call_frame(
                                            heap_2.clone(),
                                            shared_constructor_field1.clone(),
                                            self::replace_java_script_call_pending(
                                                frame.clone().expect("a proven present value"),
                                                self::JavaScriptCallPending::WaitInstanceOf(
                                                    destination,
                                                    realm.clone(),
                                                    Box::new(state_2.clone()),
                                                ),
                                            ),
                                        );
                                        self::push_java_script_call(
                                            program.clone(),
                                            next_2.clone(),
                                            source_ml_source2_f38360.clone(),
                                            source_ml_source2_f38361.clone(),
                                            source_ml_source2_f38362.clone(),
                                        )
                                    }
                                }
                                self::JavaScriptInstanceOfTransition::InstanceOfCompleted(
                                    heap_3,
                                    value,
                                ) => {
                                    let heap_3 = *heap_3;
                                    let value = *value;
                                    self::JavaScriptCallMachineTransition::MachineContinue(
                                        Box::new(self::replace_java_script_call_frame(
                                            heap_3.clone(),
                                            shared_constructor_field1.clone(),
                                            self::write_java_script_call_register(
                                                frame.clone().expect("a proven present value"),
                                                destination,
                                                value.clone(),
                                            ),
                                        )),
                                    )
                                }
                                self::JavaScriptInstanceOfTransition::InstanceOfThrow(
                                    _,
                                    value_2,
                                ) => {
                                    let value_2 = *value_2;
                                    self::throw_java_script_call_value(
                                        current.clone(),
                                        value_2.clone(),
                                    )
                                }
                                self::JavaScriptInstanceOfTransition::InstanceOfError(
                                    _,
                                    error_class,
                                    reason,
                                ) => self::request_java_script_call_error(
                                    current.clone(),
                                    realm.clone(),
                                    &(error_class),
                                    &(reason),
                                ),
                                self::JavaScriptInstanceOfTransition::InstanceOfInvalid(
                                    heap_4,
                                    reason_2,
                                ) => {
                                    let heap_4 = *heap_4;
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(heap_4.clone()),
                                        reason_2.clone(),
                                    )
                                }
                                self::JavaScriptInstanceOfTransition::InstanceOfResourceLimit(
                                    heap_5,
                                    state_3,
                                    source_ml_source2_f38365,
                                ) => {
                                    let heap_5 = *heap_5;
                                    let state_3 = *state_3;
                                    {
                                        let next_3 = self::replace_java_script_call_frame(
                                            heap_5.clone(),
                                            shared_constructor_field1.clone(),
                                            self::replace_java_script_call_pending(
                                                frame.clone().expect("a proven present value"),
                                                self::JavaScriptCallPending::WaitInstanceOf(
                                                    destination,
                                                    realm.clone(),
                                                    Box::new(state_3.clone()),
                                                ),
                                            ),
                                        );
                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(
                                            Box::new(next_3.clone()),
                                            source_ml_source2_f38365.clone(),
                                        )
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
pub fn accept_java_script_call_property_reflection(
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptPropertyReflectionTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
            let frame = {
                let source_ml_source2_array_read_value38382 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_array_read_index38383 = ({
                        let ml_reference_array2481 = shared_constructor_field1.clone();
                        let ml_reference_projection2481 =
                            ml_reference_array2481.borrow().len() as f64;
                        ml_reference_projection2481
                    } - 1f64);
                    {
                        let ml_reference_array2482 =
                            source_ml_source2_array_read_value38382.clone();
                        let ml_reference_position2482 = source_ml_source2_array_read_index38383;
                        let ml_reference_projection2482 = self::ml_array::optional_at(
                            &*ml_reference_array2482.borrow(),
                            self::ml_array::number_index(ml_reference_position2482),
                        );
                        ml_reference_projection2482
                    }
                }
            };
            if match (frame.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(source_ml_source2_data_property_value38384, _) => {
                            let source_ml_source2_data_property_value38384 = *source_ml_source2_data_property_value38384;
                            source_ml_source2_data_property_value38384.clone()
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionAllocate(source_ml_source2_data_property_value38385, _, _) => {
                            let source_ml_source2_data_property_value38385 = *source_ml_source2_data_property_value38385;
                            source_ml_source2_data_property_value38385.clone()
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionCompleted(source_ml_source2_data_property_value38386, _) => {
                            let source_ml_source2_data_property_value38386 = *source_ml_source2_data_property_value38386;
                            source_ml_source2_data_property_value38386.clone()
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionError(source_ml_source2_data_property_value38387, _, _) => {
                            let source_ml_source2_data_property_value38387 = *source_ml_source2_data_property_value38387;
                            source_ml_source2_data_property_value38387.clone()
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(source_ml_source2_data_property_value38388, _) => {
                            let source_ml_source2_data_property_value38388 = *source_ml_source2_data_property_value38388;
                            source_ml_source2_data_property_value38388.clone()
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionResourceLimit(source_ml_source2_data_property_value38389, _, _) => {
                            let source_ml_source2_data_property_value38389 = *source_ml_source2_data_property_value38389;
                            source_ml_source2_data_property_value38389.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
            } else {
                match transition.clone() {
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(heap, state) => {
                            let heap = *heap;
                            let state = *state;
                            {
                                let next = self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitPropertyReflection(destination, realm.clone(), Box::new(state.clone()))));
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(next.clone()))
                            }
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionAllocate(heap_2, state_2, source_ml_source2_f38390) => {
                            let heap_2 = *heap_2;
                            let state_2 = *state_2;
                            let source_ml_source2_f38390 = *source_ml_source2_f38390;
                            {
                                let next_2 = self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitPropertyReflection(destination, realm.clone(), Box::new(state_2.clone()))));
                                self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(next_2.clone()), Box::new(source_ml_source2_f38390.clone()))
                            }
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionCompleted(heap_3, value) => {
                            let heap_3 = *heap_3;
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionError(heap_4, error_class, reason) => {
                            let heap_4 = *heap_4;
                            self::request_java_script_call_error(self::JavaScriptCallMachine::CallMachine(Box::new(heap_4.clone()), shared_constructor_field1.clone()), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(heap_5, reason_2) => {
                            let heap_5 = *heap_5;
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_5.clone()), reason_2.clone())
                        }
                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionResourceLimit(heap_6, state_3, source_ml_source2_f38391) => {
                            let heap_6 = *heap_6;
                            let state_3 = *state_3;
                            {
                                let next_3 = self::replace_java_script_call_frame(heap_6.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitPropertyReflection(destination, realm.clone(), Box::new(state_3.clone()))));
                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(next_3.clone()), source_ml_source2_f38391.clone())
                            }
                        }
                    }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_property_definitions(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptPropertyDefinitionsTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
            let frame = {
                let source_ml_source2_array_read_value38409 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_array_read_index38410 = ({
                        let ml_reference_array2483 = shared_constructor_field1.clone();
                        let ml_reference_projection2483 =
                            ml_reference_array2483.borrow().len() as f64;
                        ml_reference_projection2483
                    } - 1f64);
                    {
                        let ml_reference_array2484 =
                            source_ml_source2_array_read_value38409.clone();
                        let ml_reference_position2484 = source_ml_source2_array_read_index38410;
                        let ml_reference_projection2484 = self::ml_array::optional_at(
                            &*ml_reference_array2484.borrow(),
                            self::ml_array::number_index(ml_reference_position2484),
                        );
                        ml_reference_projection2484
                    }
                }
            };
            {
                let current = self::JavaScriptCallMachine::CallMachine(Box::new(match transition.clone() {
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(source_ml_source2_data_property_value38411, _) => {
                            let source_ml_source2_data_property_value38411 = *source_ml_source2_data_property_value38411;
                            source_ml_source2_data_property_value38411.clone()
                        }
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCall(source_ml_source2_data_property_value38412, _, _, _, _) => {
                            let source_ml_source2_data_property_value38412 = *source_ml_source2_data_property_value38412;
                            source_ml_source2_data_property_value38412.clone()
                        }
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsAllocate(source_ml_source2_data_property_value38413, _, _) => {
                            let source_ml_source2_data_property_value38413 = *source_ml_source2_data_property_value38413;
                            source_ml_source2_data_property_value38413.clone()
                        }
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCompleted(source_ml_source2_data_property_value38414, _) => {
                            let source_ml_source2_data_property_value38414 = *source_ml_source2_data_property_value38414;
                            source_ml_source2_data_property_value38414.clone()
                        }
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsThrow(source_ml_source2_data_property_value38415, _) => {
                            let source_ml_source2_data_property_value38415 = *source_ml_source2_data_property_value38415;
                            source_ml_source2_data_property_value38415.clone()
                        }
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsError(source_ml_source2_data_property_value38416, _, _) => {
                            let source_ml_source2_data_property_value38416 = *source_ml_source2_data_property_value38416;
                            source_ml_source2_data_property_value38416.clone()
                        }
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(source_ml_source2_data_property_value38417, _) => {
                            let source_ml_source2_data_property_value38417 = *source_ml_source2_data_property_value38417;
                            source_ml_source2_data_property_value38417.clone()
                        }
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsResourceLimit(source_ml_source2_data_property_value38418, _, _) => {
                            let source_ml_source2_data_property_value38418 = *source_ml_source2_data_property_value38418;
                            source_ml_source2_data_property_value38418.clone()
                        }
                    }), shared_constructor_field1.clone());
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(source_ml_source2_data_property_value38420, _) => {
                                let source_ml_source2_data_property_value38420 = *source_ml_source2_data_property_value38420;
                                source_ml_source2_data_property_value38420.clone()
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCall(source_ml_source2_data_property_value38421, _, _, _, _) => {
                                let source_ml_source2_data_property_value38421 = *source_ml_source2_data_property_value38421;
                                source_ml_source2_data_property_value38421.clone()
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsAllocate(source_ml_source2_data_property_value38422, _, _) => {
                                let source_ml_source2_data_property_value38422 = *source_ml_source2_data_property_value38422;
                                source_ml_source2_data_property_value38422.clone()
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCompleted(source_ml_source2_data_property_value38423, _) => {
                                let source_ml_source2_data_property_value38423 = *source_ml_source2_data_property_value38423;
                                source_ml_source2_data_property_value38423.clone()
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsThrow(source_ml_source2_data_property_value38424, _) => {
                                let source_ml_source2_data_property_value38424 = *source_ml_source2_data_property_value38424;
                                source_ml_source2_data_property_value38424.clone()
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsError(source_ml_source2_data_property_value38425, _, _) => {
                                let source_ml_source2_data_property_value38425 = *source_ml_source2_data_property_value38425;
                                source_ml_source2_data_property_value38425.clone()
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(source_ml_source2_data_property_value38426, _) => {
                                let source_ml_source2_data_property_value38426 = *source_ml_source2_data_property_value38426;
                                source_ml_source2_data_property_value38426.clone()
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsResourceLimit(source_ml_source2_data_property_value38427, _, _) => {
                                let source_ml_source2_data_property_value38427 = *source_ml_source2_data_property_value38427;
                                source_ml_source2_data_property_value38427.clone()
                            }
                        }), self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(heap, state) => {
                                let heap = *heap;
                                let state = *state;
                                {
                                    let next = self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitPropertyDefinitions(destination, realm.clone(), Box::new(state.clone()))));
                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(next.clone()))
                                }
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCall(heap_2, state_2, source_ml_source2_f38428, source_ml_source2_f38429, source_ml_source2_f38430) => {
                                let heap_2 = *heap_2;
                                let state_2 = *state_2;
                                let source_ml_source2_f38428 = *source_ml_source2_f38428;
                                let source_ml_source2_f38429 = *source_ml_source2_f38429;
                                {
                                    let next_2 = self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitPropertyDefinitions(destination, realm.clone(), Box::new(state_2.clone()))));
                                    self::push_java_script_call(program.clone(), next_2.clone(), source_ml_source2_f38428.clone(), source_ml_source2_f38429.clone(), source_ml_source2_f38430.clone())
                                }
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsAllocate(heap_3, state_3, source_ml_source2_f38431) => {
                                let heap_3 = *heap_3;
                                let state_3 = *state_3;
                                let source_ml_source2_f38431 = *source_ml_source2_f38431;
                                {
                                    let next_3 = self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitPropertyDefinitions(destination, realm.clone(), Box::new(state_3.clone()))));
                                    self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(next_3.clone()), Box::new(source_ml_source2_f38431.clone()))
                                }
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsCompleted(heap_4, value) => {
                                let heap_4 = *heap_4;
                                let value = *value;
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_4.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsThrow(_, value_2) => {
                                let value_2 = *value_2;
                                self::throw_java_script_call_value(current.clone(), value_2.clone())
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsError(_, error_class, reason) => {
                                self::request_java_script_call_error(current.clone(), realm.clone(), &(error_class), &(reason))
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(heap_5, reason_2) => {
                                let heap_5 = *heap_5;
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_5.clone()), reason_2.clone())
                            }
                            self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsResourceLimit(heap_6, state_4, source_ml_source2_f38434) => {
                                let heap_6 = *heap_6;
                                let state_4 = *state_4;
                                {
                                    let next_4 = self::replace_java_script_call_frame(heap_6.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitPropertyDefinitions(destination, realm.clone(), Box::new(state_4.clone()))));
                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(next_4.clone()), source_ml_source2_f38434.clone())
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
pub fn resume_java_script_call_symbol_description(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    description: self::JavaScriptSemanticValue,
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
                            let source_ml_source2_array_read_value38461 =
                                shared_constructor_field12.clone();
                            {
                                let source_ml_source2_array_read_index38462 = ({
                                    let ml_reference_array2485 = shared_constructor_field12.clone();
                                    let ml_reference_projection2485 =
                                        ml_reference_array2485.borrow().len() as f64;
                                    ml_reference_projection2485
                                } - 1f64);
                                {
                                    let ml_reference_array2486 =
                                        source_ml_source2_array_read_value38461.clone();
                                    let ml_reference_position2486 =
                                        source_ml_source2_array_read_index38462;
                                    let ml_reference_projection2486 = self::ml_array::optional_at(
                                        &*ml_reference_array2486.borrow(),
                                        self::ml_array::number_index(ml_reference_position2486),
                                    );
                                    ml_reference_projection2486
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
                                    83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105,
                                    112, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114,
                                    101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97,
                                    108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 115,
                                    111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99,
                                    116, 105, 111, 110,
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
                                        source_ml_source2_data_property_value38464,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value38464 =
                                            *source_ml_source2_data_property_value38464;
                                        source_ml_source2_data_property_value38464.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::WaitSymbolDescription(
                                        destination,
                                        _,
                                    ) => {
                                        if (((match description.clone() {
                                            self::JavaScriptSemanticValue::Undefined => false,
                                            _ => true,
                                        } && match description.clone() {
                                            self::JavaScriptSemanticValue::Text(_) => false,
                                            _ => true,
                                        }) && match description.clone() {
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                false
                                            }
                                            _ => true,
                                        }) || ((self::read_java_script_ordinary_value_error(
                                            shared_constructor_field11.clone(),
                                            description.clone(),
                                        )) != (self::ml_string::Text::from_units(
                                            vec![],
                                        )))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                                Box::new(shared_constructor_field11.clone()),
                                                self::ml_string::Text::from_units(vec![
                                                    83, 121, 109, 98, 111, 108, 32, 100, 101, 115,
                                                    99, 114, 105, 112, 116, 105, 111, 110, 115, 32,
                                                    114, 101, 116, 97, 105, 110, 32, 97, 99, 116,
                                                    117, 97, 108, 32, 111, 112, 116, 105, 111, 110,
                                                    97, 108, 32, 85, 84, 70, 49, 54, 32, 116, 101,
                                                    120, 116, 32, 109, 101, 116, 97, 100, 97, 116,
                                                    97,
                                                ]),
                                            )
                                        } else {
                                            self::JavaScriptCallMachineTransition::MachineContinue(
                                                Box::new(self::replace_java_script_call_frame(
                                                    shared_constructor_field11.clone(),
                                                    shared_constructor_field12.clone(),
                                                    self::write_java_script_call_register(
                                                        {
                                                            let source_ml_source2_nominal_spread38474 =
                                                                frame.clone().expect(
                                                                    "a proven present value",
                                                                );
                                                            match source_ml_source2_nominal_spread38474.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                        {
                                                            let source_ml_source2_nominal_field38475 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field38477 = shared_constructor_field2;
                                                                {
                                                                    let source_ml_source2_nominal_field38479 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field38481 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field38483 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field38485 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field38487 = shared_constructor_field7.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field38489 = shared_constructor_field8.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field38491 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field38493 = shared_constructor_field10.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override38495 = self::JavaScriptCallPending::CallIdle;
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field38475.clone(), source_ml_source2_nominal_field38477, source_ml_source2_nominal_field38479.clone(), source_ml_source2_nominal_field38481.clone(), source_ml_source2_nominal_field38483.clone(), Box::new(source_ml_source2_nominal_override38495.clone()), source_ml_source2_nominal_field38487.clone(), source_ml_source2_nominal_field38489.clone(), source_ml_source2_nominal_field38491.clone(), source_ml_source2_nominal_field38493.clone())
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
                                                        },
                                                        destination,
                                                        description.clone(),
                                                    ),
                                                )),
                                            )
                                        }
                                    }
                                    _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field11.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114,
                                            105, 112, 116, 105, 111, 110, 115, 32, 114, 101, 113,
                                            117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32,
                                            97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101,
                                            110, 100, 101, 100, 32, 115, 111, 117, 114, 99, 101,
                                            32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111,
                                            110,
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
