// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySliceTransition;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptSpeciesConstructionState;
use super::accept_java_script_call_species_construction;
use super::begin_java_script_species_construction;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_array_transform_directory;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_array_slice(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptArraySliceTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptArraySliceTransition::ArraySliceContinue(
                    source_ml_source2_data_property_value33072,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33072 =
                        *source_ml_source2_data_property_value33072;
                    source_ml_source2_data_property_value33072.clone()
                }
                self::JavaScriptArraySliceTransition::ArraySliceCall(
                    source_ml_source2_data_property_value33073,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33073 =
                        *source_ml_source2_data_property_value33073;
                    source_ml_source2_data_property_value33073.clone()
                }
                self::JavaScriptArraySliceTransition::ArraySliceAllocate(
                    source_ml_source2_data_property_value33074,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33074 =
                        *source_ml_source2_data_property_value33074;
                    source_ml_source2_data_property_value33074.clone()
                }
                self::JavaScriptArraySliceTransition::ArraySliceConstruct(
                    source_ml_source2_data_property_value33075,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33075 =
                        *source_ml_source2_data_property_value33075;
                    source_ml_source2_data_property_value33075.clone()
                }
                self::JavaScriptArraySliceTransition::ArraySliceNeedAuthority(
                    source_ml_source2_data_property_value33076,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33076 =
                        *source_ml_source2_data_property_value33076;
                    source_ml_source2_data_property_value33076.clone()
                }
                self::JavaScriptArraySliceTransition::ArraySliceCompleted(
                    source_ml_source2_data_property_value33077,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33077 =
                        *source_ml_source2_data_property_value33077;
                    source_ml_source2_data_property_value33077.clone()
                }
                self::JavaScriptArraySliceTransition::ArraySliceThrow(
                    source_ml_source2_data_property_value33078,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33078 =
                        *source_ml_source2_data_property_value33078;
                    source_ml_source2_data_property_value33078.clone()
                }
                self::JavaScriptArraySliceTransition::ArraySliceError(
                    source_ml_source2_data_property_value33079,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33079 =
                        *source_ml_source2_data_property_value33079;
                    source_ml_source2_data_property_value33079.clone()
                }
                self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                    source_ml_source2_data_property_value33080,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33080 =
                        *source_ml_source2_data_property_value33080;
                    source_ml_source2_data_property_value33080.clone()
                }
                self::JavaScriptArraySliceTransition::ArraySliceResourceLimit(
                    source_ml_source2_data_property_value33081,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33081 =
                        *source_ml_source2_data_property_value33081;
                    source_ml_source2_data_property_value33081.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value33082,
                ) => source_ml_source2_data_property_value33082.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                let frame = {
                    let source_ml_source2_array_read_value33085 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_array_read_index33086 = ({
                            let ml_reference_array2162 = shared_constructor_field1.clone();
                            let ml_reference_projection2162 =
                                ml_reference_array2162.borrow().len() as f64;
                            ml_reference_projection2162
                        } - 1f64);
                        {
                            let ml_reference_array2163 =
                                source_ml_source2_array_read_value33085.clone();
                            let ml_reference_position2163 = source_ml_source2_array_read_index33086;
                            let ml_reference_projection2163 = self::ml_array::optional_at(
                                &*ml_reference_array2163.borrow(),
                                self::ml_array::number_index(ml_reference_position2163),
                            );
                            ml_reference_projection2163
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
                            self::JavaScriptArraySliceTransition::ArraySliceContinue(
                                source_ml_source2_data_property_value33087,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33087 =
                                    *source_ml_source2_data_property_value33087;
                                source_ml_source2_data_property_value33087.clone()
                            }
                            self::JavaScriptArraySliceTransition::ArraySliceCall(
                                source_ml_source2_data_property_value33088,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33088 =
                                    *source_ml_source2_data_property_value33088;
                                source_ml_source2_data_property_value33088.clone()
                            }
                            self::JavaScriptArraySliceTransition::ArraySliceAllocate(
                                source_ml_source2_data_property_value33089,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33089 =
                                    *source_ml_source2_data_property_value33089;
                                source_ml_source2_data_property_value33089.clone()
                            }
                            self::JavaScriptArraySliceTransition::ArraySliceConstruct(
                                source_ml_source2_data_property_value33090,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33090 =
                                    *source_ml_source2_data_property_value33090;
                                source_ml_source2_data_property_value33090.clone()
                            }
                            self::JavaScriptArraySliceTransition::ArraySliceNeedAuthority(
                                source_ml_source2_data_property_value33091,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33091 =
                                    *source_ml_source2_data_property_value33091;
                                source_ml_source2_data_property_value33091.clone()
                            }
                            self::JavaScriptArraySliceTransition::ArraySliceCompleted(
                                source_ml_source2_data_property_value33092,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33092 =
                                    *source_ml_source2_data_property_value33092;
                                source_ml_source2_data_property_value33092.clone()
                            }
                            self::JavaScriptArraySliceTransition::ArraySliceThrow(
                                source_ml_source2_data_property_value33093,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33093 =
                                    *source_ml_source2_data_property_value33093;
                                source_ml_source2_data_property_value33093.clone()
                            }
                            self::JavaScriptArraySliceTransition::ArraySliceError(
                                source_ml_source2_data_property_value33094,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33094 =
                                    *source_ml_source2_data_property_value33094;
                                source_ml_source2_data_property_value33094.clone()
                            }
                            self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                source_ml_source2_data_property_value33095,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33095 =
                                    *source_ml_source2_data_property_value33095;
                                source_ml_source2_data_property_value33095.clone()
                            }
                            self::JavaScriptArraySliceTransition::ArraySliceResourceLimit(
                                source_ml_source2_data_property_value33096,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33096 =
                                    *source_ml_source2_data_property_value33096;
                                source_ml_source2_data_property_value33096.clone()
                            }
                        }),
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109,
                            97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                            105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105,
                            118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 114,
                            97, 109, 101,
                        ]),
                    )
                } else {
                    match transition.clone() {
                        self::JavaScriptArraySliceTransition::ArraySliceContinue(heap, state) => {
                            let heap = *heap;
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArraySlice(
                                            destination,
                                            realm.clone(),
                                            Box::new(state.clone()),
                                            None::<(
                                                Option<self::JavaScriptSpeciesConstructionState>,
                                            )>,
                                        ),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptArraySliceTransition::ArraySliceCall(
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
                                        self::JavaScriptCallPending::WaitArraySlice(
                                            destination,
                                            realm.clone(),
                                            Box::new(state_2.clone()),
                                            None::<(
                                                Option<self::JavaScriptSpeciesConstructionState>,
                                            )>,
                                        ),
                                    ),
                                ),
                                callee.clone(),
                                receiver.clone(),
                                arguments.clone(),
                            )
                        }
                        self::JavaScriptArraySliceTransition::ArraySliceAllocate(
                            heap_3,
                            state_3,
                            prototype,
                            _,
                        ) => {
                            let heap_3 = *heap_3;
                            let state_3 = *state_3;
                            let prototype = *prototype;
                            self::JavaScriptCallMachineTransition::MachineAllocateObject(
                                Box::new(self::replace_java_script_call_frame(
                                    heap_3.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArraySlice(
                                            destination,
                                            realm.clone(),
                                            Box::new(state_3.clone()),
                                            None::<(
                                                Option<self::JavaScriptSpeciesConstructionState>,
                                            )>,
                                        ),
                                    ),
                                )),
                                Box::new(prototype.clone()),
                            )
                        }
                        self::JavaScriptArraySliceTransition::ArraySliceConstruct(
                            heap_4,
                            state_4,
                            constructor,
                            _,
                            fact,
                        ) => {
                            let heap_4 = *heap_4;
                            let state_4 = *state_4;
                            let constructor = *constructor;
                            let fact = *fact;
                            {
                                let suspended = self::replace_java_script_call_frame(
                                    heap_4.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArraySlice(
                                            destination,
                                            realm.clone(),
                                            Box::new(state_4.clone()),
                                            None::<(
                                                Option<self::JavaScriptSpeciesConstructionState>,
                                            )>,
                                        ),
                                    ),
                                );
                                {
                                    let directory =
                                        self::read_java_script_call_array_transform_directory(
                                            program.clone(),
                                            suspended.clone(),
                                            destination,
                                            realm.clone(),
                                        );
                                    if match (directory.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(heap_4.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                99, 117, 115, 116, 111, 109, 32, 115, 112, 101, 99,
                                                105, 101, 115, 32, 99, 111, 110, 115, 116, 114,
                                                117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97,
                                                105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                                117, 97, 108, 32, 116, 114, 97, 110, 115, 102, 111,
                                                114, 109, 32, 100, 105, 114, 101, 99, 116, 111,
                                                114, 121,
                                            ]),
                                        )
                                    } else {
                                        self::accept_java_script_call_species_construction(program.clone(), suspended.clone(), self::begin_java_script_species_construction(program.clone(), heap_4.clone(), directory.clone().expect("a proven present value"), constructor.clone(), fact.clone(), match state_4.clone() {
                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value33117, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value33117
                                                }
                                            }))
                                    }
                                }
                            }
                        }
                        self::JavaScriptArraySliceTransition::ArraySliceNeedAuthority(
                            heap_5,
                            state_5,
                            constructor_2,
                        ) => {
                            let heap_5 = *heap_5;
                            let state_5 = *state_5;
                            let constructor_2 = *constructor_2;
                            self::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(
                                Box::new(self::replace_java_script_call_frame(
                                    heap_5.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArraySlice(
                                            destination,
                                            realm.clone(),
                                            Box::new(state_5.clone()),
                                            None::<(
                                                Option<self::JavaScriptSpeciesConstructionState>,
                                            )>,
                                        ),
                                    ),
                                )),
                                Box::new(constructor_2.clone()),
                            )
                        }
                        self::JavaScriptArraySliceTransition::ArraySliceCompleted(
                            heap_6,
                            value,
                        ) => {
                            let heap_6 = *heap_6;
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap_6.clone(),
                                    shared_constructor_field1.clone(),
                                    self::write_java_script_call_register(
                                        frame.clone().expect("a proven present value"),
                                        destination,
                                        value.clone(),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptArraySliceTransition::ArraySliceThrow(_, value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(current.clone(), value_2.clone())
                        }
                        self::JavaScriptArraySliceTransition::ArraySliceError(
                            _,
                            error_class,
                            reason,
                        ) => self::request_java_script_call_error(
                            current.clone(),
                            realm.clone(),
                            &(error_class),
                            &(reason),
                        ),
                        self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                            heap_7,
                            reason_2,
                        ) => {
                            let heap_7 = *heap_7;
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(heap_7.clone()),
                                reason_2.clone(),
                            )
                        }
                        self::JavaScriptArraySliceTransition::ArraySliceResourceLimit(
                            heap_8,
                            state_6,
                            reason_3,
                        ) => {
                            let heap_8 = *heap_8;
                            let state_6 = *state_6;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(
                                Box::new(self::replace_java_script_call_frame(
                                    heap_8.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArraySlice(
                                            destination,
                                            realm.clone(),
                                            Box::new(state_6.clone()),
                                            None::<(
                                                Option<self::JavaScriptSpeciesConstructionState>,
                                            )>,
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
