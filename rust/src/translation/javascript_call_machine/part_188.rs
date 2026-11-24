// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArrayTransformState;
use super::JavaScriptArrayTransformTransition;
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSpeciesConstructionState;
use super::JavaScriptSpeciesConstructionTransition;
use super::accept_java_script_call_array_concatenation;
use super::accept_java_script_call_array_slice;
use super::accept_java_script_call_species_construction;
use super::begin_java_script_species_construction;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_array_transform_directory;
use super::read_java_script_call_machine_error;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::resume_java_script_array_concatenation_construction;
use super::resume_java_script_array_slice_construction;
use super::resume_java_script_array_transform_construction;
use super::step_java_script_species_construction;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_array_transform(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptArrayTransformTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptArrayTransformTransition::ArrayTransformContinue(
                    source_ml_source2_data_property_value33127,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33127 =
                        *source_ml_source2_data_property_value33127;
                    source_ml_source2_data_property_value33127.clone()
                }
                self::JavaScriptArrayTransformTransition::ArrayTransformCall(
                    source_ml_source2_data_property_value33128,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33128 =
                        *source_ml_source2_data_property_value33128;
                    source_ml_source2_data_property_value33128.clone()
                }
                self::JavaScriptArrayTransformTransition::ArrayTransformAllocate(
                    source_ml_source2_data_property_value33129,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33129 =
                        *source_ml_source2_data_property_value33129;
                    source_ml_source2_data_property_value33129.clone()
                }
                self::JavaScriptArrayTransformTransition::ArrayTransformConstruct(
                    source_ml_source2_data_property_value33130,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33130 =
                        *source_ml_source2_data_property_value33130;
                    source_ml_source2_data_property_value33130.clone()
                }
                self::JavaScriptArrayTransformTransition::ArrayTransformNeedAuthority(
                    source_ml_source2_data_property_value33131,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33131 =
                        *source_ml_source2_data_property_value33131;
                    source_ml_source2_data_property_value33131.clone()
                }
                self::JavaScriptArrayTransformTransition::ArrayTransformCompleted(
                    source_ml_source2_data_property_value33132,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33132 =
                        *source_ml_source2_data_property_value33132;
                    source_ml_source2_data_property_value33132.clone()
                }
                self::JavaScriptArrayTransformTransition::ArrayTransformThrow(
                    source_ml_source2_data_property_value33133,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33133 =
                        *source_ml_source2_data_property_value33133;
                    source_ml_source2_data_property_value33133.clone()
                }
                self::JavaScriptArrayTransformTransition::ArrayTransformError(
                    source_ml_source2_data_property_value33134,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33134 =
                        *source_ml_source2_data_property_value33134;
                    source_ml_source2_data_property_value33134.clone()
                }
                self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                    source_ml_source2_data_property_value33135,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33135 =
                        *source_ml_source2_data_property_value33135;
                    source_ml_source2_data_property_value33135.clone()
                }
                self::JavaScriptArrayTransformTransition::ArrayTransformResourceLimit(
                    source_ml_source2_data_property_value33136,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33136 =
                        *source_ml_source2_data_property_value33136;
                    source_ml_source2_data_property_value33136.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value33137,
                ) => source_ml_source2_data_property_value33137.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field3) => {
                let frame = {
                    let source_ml_source2_array_read_value33140 = shared_constructor_field3.clone();
                    {
                        let source_ml_source2_array_read_index33141 = ({
                            let ml_reference_array2164 = shared_constructor_field3.clone();
                            let ml_reference_projection2164 =
                                ml_reference_array2164.borrow().len() as f64;
                            ml_reference_projection2164
                        } - 1f64);
                        {
                            let ml_reference_array2165 =
                                source_ml_source2_array_read_value33140.clone();
                            let ml_reference_position2165 = source_ml_source2_array_read_index33141;
                            let ml_reference_projection2165 = self::ml_array::optional_at(
                                &*ml_reference_array2165.borrow(),
                                self::ml_array::number_index(ml_reference_position2165),
                            );
                            ml_reference_projection2165
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                            self::JavaScriptArrayTransformTransition::ArrayTransformContinue(source_ml_source2_data_property_value33142, _) => {
                                let source_ml_source2_data_property_value33142 = *source_ml_source2_data_property_value33142;
                                source_ml_source2_data_property_value33142.clone()
                            }
                            self::JavaScriptArrayTransformTransition::ArrayTransformCall(source_ml_source2_data_property_value33143, _, _, _, _) => {
                                let source_ml_source2_data_property_value33143 = *source_ml_source2_data_property_value33143;
                                source_ml_source2_data_property_value33143.clone()
                            }
                            self::JavaScriptArrayTransformTransition::ArrayTransformAllocate(source_ml_source2_data_property_value33144, _, _, _) => {
                                let source_ml_source2_data_property_value33144 = *source_ml_source2_data_property_value33144;
                                source_ml_source2_data_property_value33144.clone()
                            }
                            self::JavaScriptArrayTransformTransition::ArrayTransformConstruct(source_ml_source2_data_property_value33145, _, _, _, _) => {
                                let source_ml_source2_data_property_value33145 = *source_ml_source2_data_property_value33145;
                                source_ml_source2_data_property_value33145.clone()
                            }
                            self::JavaScriptArrayTransformTransition::ArrayTransformNeedAuthority(source_ml_source2_data_property_value33146, _, _) => {
                                let source_ml_source2_data_property_value33146 = *source_ml_source2_data_property_value33146;
                                source_ml_source2_data_property_value33146.clone()
                            }
                            self::JavaScriptArrayTransformTransition::ArrayTransformCompleted(source_ml_source2_data_property_value33147, _) => {
                                let source_ml_source2_data_property_value33147 = *source_ml_source2_data_property_value33147;
                                source_ml_source2_data_property_value33147.clone()
                            }
                            self::JavaScriptArrayTransformTransition::ArrayTransformThrow(source_ml_source2_data_property_value33148, _) => {
                                let source_ml_source2_data_property_value33148 = *source_ml_source2_data_property_value33148;
                                source_ml_source2_data_property_value33148.clone()
                            }
                            self::JavaScriptArrayTransformTransition::ArrayTransformError(source_ml_source2_data_property_value33149, _, _) => {
                                let source_ml_source2_data_property_value33149 = *source_ml_source2_data_property_value33149;
                                source_ml_source2_data_property_value33149.clone()
                            }
                            self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(source_ml_source2_data_property_value33150, _) => {
                                let source_ml_source2_data_property_value33150 = *source_ml_source2_data_property_value33150;
                                source_ml_source2_data_property_value33150.clone()
                            }
                            self::JavaScriptArrayTransformTransition::ArrayTransformResourceLimit(source_ml_source2_data_property_value33151, _, _) => {
                                let source_ml_source2_data_property_value33151 = *source_ml_source2_data_property_value33151;
                                source_ml_source2_data_property_value33151.clone()
                            }
                        }), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptArrayTransformTransition::ArrayTransformContinue(
                            heap,
                            state,
                        ) => {
                            let heap = *heap;
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap.clone(),
                                    shared_constructor_field3.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArrayTransform(
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
                        self::JavaScriptArrayTransformTransition::ArrayTransformCall(
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
                                    shared_constructor_field3.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArrayTransform(
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
                        self::JavaScriptArrayTransformTransition::ArrayTransformAllocate(
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
                                    shared_constructor_field3.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArrayTransform(
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
                        self::JavaScriptArrayTransformTransition::ArrayTransformConstruct(
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
                            match state_4.clone() {
                                self::JavaScriptArrayTransformState::ArrayTransformState(
                                    shared_constructor_field1,
                                    _,
                                    _,
                                    _,
                                    _,
                                    shared_constructor_field2,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let suspended = self::replace_java_script_call_frame(
                                        heap_4.clone(),
                                        shared_constructor_field3.clone(),
                                        self::replace_java_script_call_pending(
                                            frame.clone().expect("a proven present value"),
                                            self::JavaScriptCallPending::WaitArrayTransform(
                                                destination,
                                                realm.clone(),
                                                Box::new(state_4.clone()),
                                                None::<(
                                                    Option<
                                                        self::JavaScriptSpeciesConstructionState,
                                                    >,
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
                                                    99, 117, 115, 116, 111, 109, 32, 115, 112, 101,
                                                    99, 105, 101, 115, 32, 99, 111, 110, 115, 116,
                                                    114, 117, 99, 116, 105, 111, 110, 32, 114, 101,
                                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                                    97, 99, 116, 117, 97, 108, 32, 116, 114, 97,
                                                    110, 115, 102, 111, 114, 109, 32, 100, 105,
                                                    114, 101, 99, 116, 111, 114, 121,
                                                ]),
                                            )
                                        } else {
                                            self::accept_java_script_call_species_construction(
                                                program.clone(),
                                                suspended.clone(),
                                                self::begin_java_script_species_construction(
                                                    program.clone(),
                                                    heap_4.clone(),
                                                    directory
                                                        .clone()
                                                        .expect("a proven present value"),
                                                    constructor.clone(),
                                                    fact.clone(),
                                                    if (shared_constructor_field1
                                                        == (self::ml_string::Text::from_units(
                                                            vec![
                                                                65, 114, 114, 97, 121, 46, 109, 97,
                                                                112,
                                                            ],
                                                        )))
                                                    {
                                                        shared_constructor_field2
                                                    } else {
                                                        0f64
                                                    },
                                                ),
                                            )
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptArrayTransformTransition::ArrayTransformNeedAuthority(
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
                                    shared_constructor_field3.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArrayTransform(
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
                        self::JavaScriptArrayTransformTransition::ArrayTransformCompleted(
                            heap_6,
                            value,
                        ) => {
                            let heap_6 = *heap_6;
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap_6.clone(),
                                    shared_constructor_field3.clone(),
                                    self::write_java_script_call_register(
                                        frame.clone().expect("a proven present value"),
                                        destination,
                                        value.clone(),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptArrayTransformTransition::ArrayTransformThrow(
                            _,
                            value_2,
                        ) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(current.clone(), value_2.clone())
                        }
                        self::JavaScriptArrayTransformTransition::ArrayTransformError(
                            _,
                            error_class,
                            reason,
                        ) => self::request_java_script_call_error(
                            current.clone(),
                            realm.clone(),
                            &(error_class),
                            &(reason),
                        ),
                        self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                            heap_7,
                            reason_2,
                        ) => {
                            let heap_7 = *heap_7;
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(heap_7.clone()),
                                reason_2.clone(),
                            )
                        }
                        self::JavaScriptArrayTransformTransition::ArrayTransformResourceLimit(
                            heap_8,
                            state_6,
                            reason_3,
                        ) => {
                            let heap_8 = *heap_8;
                            let state_6 = *state_6;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(
                                Box::new(self::replace_java_script_call_frame(
                                    heap_8.clone(),
                                    shared_constructor_field3.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArrayTransform(
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_array_construction(
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
                            let source_ml_source2_array_read_value33187 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index33188 = ({
                                    let ml_reference_array2166 = shared_constructor_field2.clone();
                                    let ml_reference_projection2166 =
                                        ml_reference_array2166.borrow().len() as f64;
                                    ml_reference_projection2166
                                } - 1f64);
                                {
                                    let ml_reference_array2167 =
                                        source_ml_source2_array_read_value33187.clone();
                                    let ml_reference_position2167 =
                                        source_ml_source2_array_read_index33188;
                                    let ml_reference_projection2167 = self::ml_array::optional_at(
                                        &*ml_reference_array2167.borrow(),
                                        self::ml_array::number_index(ml_reference_position2167),
                                    );
                                    ml_reference_projection2167
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
                                    115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116,
                                    114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115,
                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99,
                                    116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102,
                                    114, 97, 109, 101,
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
                                        source_ml_source2_data_property_value33190,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value33190 =
                                            *source_ml_source2_data_property_value33190;
                                        source_ml_source2_data_property_value33190.clone()
                                    }
                                };
                                if ((match pending.clone() {
                                    self::JavaScriptCallPending::WaitArrayTransform(_, _, _, _) => {
                                        false
                                    }
                                    _ => true,
                                } && match pending.clone() {
                                    self::JavaScriptCallPending::WaitArraySlice(_, _, _, _) => {
                                        false
                                    }
                                    _ => true,
                                }) && match pending.clone() {
                                    self::JavaScriptCallPending::WaitArrayConcatenation(
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
                                            115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110,
                                            115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114,
                                            101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114,
                                            101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                                            99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105,
                                            110, 103, 32, 116, 114, 97, 110, 115, 102, 111, 114,
                                            109,
                                        ]),
                                    )
                                } else {
                                    {
                                        let directory = self::read_java_script_call_array_transform_directory(program.clone(), machine.clone(), ((match pending.clone() {
                                            self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_data_property_value33204, _, _, _) => {
                                                Some((source_ml_source2_data_property_value33204,))
                                            }
                                            self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_data_property_value33205, _, _, _) => {
                                                Some((source_ml_source2_data_property_value33205,))
                                            }
                                            self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_data_property_value33206, _, _, _) => {
                                                Some((source_ml_source2_data_property_value33206,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0, (((match pending.clone() {
                                            self::JavaScriptCallPending::WaitArrayConcatenation(_, source_ml_source2_data_property_value33207, _, _) => {
                                                Some((source_ml_source2_data_property_value33207.clone(),))
                                            }
                                            self::JavaScriptCallPending::WaitArraySlice(_, source_ml_source2_data_property_value33208, _, _) => {
                                                Some((source_ml_source2_data_property_value33208.clone(),))
                                            }
                                            self::JavaScriptCallPending::WaitArrayTransform(_, source_ml_source2_data_property_value33209, _, _) => {
                                                Some((source_ml_source2_data_property_value33209.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml::Big,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone());
                                        if match (directory.clone(), None::<()>) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        } {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                                Box::new(shared_constructor_field1.clone()),
                                                self::ml_string::Text::from_units(vec![
                                                    115, 112, 101, 99, 105, 101, 115, 32, 99, 111,
                                                    110, 115, 116, 114, 117, 99, 116, 105, 111,
                                                    110, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                                    32, 115, 111, 117, 114, 99, 101, 32, 100, 105,
                                                    114, 101, 99, 116, 111, 114, 121,
                                                ]),
                                            )
                                        } else {
                                            {
                                                let construction = (((match pending.clone() {
                                                    self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, source_ml_source2_data_property_value33211) => {
                                                        Some((((source_ml_source2_data_property_value33211.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone(),))
                                                    }
                                                    self::JavaScriptCallPending::WaitArraySlice(_, _, _, source_ml_source2_data_property_value33212) => {
                                                        Some((((source_ml_source2_data_property_value33212.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone(),))
                                                    }
                                                    self::JavaScriptCallPending::WaitArrayTransform(_, _, _, source_ml_source2_data_property_value33213) => {
                                                        Some((((source_ml_source2_data_property_value33213.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone(),))
                                                    }
                                                    _ => {
                                                        None::<(Option<self::JavaScriptSpeciesConstructionState>,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone();
                                                if (match (construction.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } || {
                                                    let source_ml_source2_s33215 =
                                                        self::step_java_script_species_construction(
                                                            program.clone(),
                                                            shared_constructor_field1.clone(),
                                                            directory
                                                                .clone()
                                                                .expect("a proven present value"),
                                                            construction
                                                                .clone()
                                                                .expect("a proven present value"),
                                                        );
                                                    match source_ml_source2_s33215.clone() {
                                                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedImplementation(_, _, _, _, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                }) {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![101, 120, 116, 101, 114, 110, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 101, 115, 116]))
                                                } else {
                                                    self::resume_java_script_call_array_species_operation(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), directory.clone().expect("a proven present value"), completion.clone(), false)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_array_species_operation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    directory: self::JavaScriptArraySpeciesDirectory,
    completion: self::JavaScriptPrimitiveConversionCompletion,
    clear_construction: bool,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let pending = match frame.clone() {
                    self::JavaScriptCallFrame::CallFrame(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value33225,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value33225 =
                            *source_ml_source2_data_property_value33225;
                        source_ml_source2_data_property_value33225.clone()
                    }
                };
                match pending.clone() {
                    self::JavaScriptCallPending::WaitArrayConcatenation(
                        destination,
                        realm,
                        state,
                        _,
                    ) => {
                        let state = *state;
                        {
                            let next = if clear_construction {
                                self::replace_java_script_call_frame(
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone(),
                                        self::JavaScriptCallPending::WaitArrayConcatenation(
                                            destination,
                                            realm.clone(),
                                            Box::new(state.clone()),
                                            None::<(
                                                Option<self::JavaScriptSpeciesConstructionState>,
                                            )>,
                                        ),
                                    ),
                                )
                            } else {
                                machine.clone()
                            };
                            self::accept_java_script_call_array_concatenation(
                                program.clone(),
                                next.clone(),
                                destination,
                                realm.clone(),
                                self::resume_java_script_array_concatenation_construction(
                                    shared_constructor_field1.clone(),
                                    directory.clone(),
                                    state.clone(),
                                    completion.clone(),
                                ),
                            )
                        }
                    }
                    self::JavaScriptCallPending::WaitArraySlice(
                        destination_2,
                        realm_2,
                        state_2,
                        _,
                    ) => {
                        let state_2 = *state_2;
                        {
                            let next_2 = if clear_construction {
                                self::replace_java_script_call_frame(
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone(),
                                        self::JavaScriptCallPending::WaitArraySlice(
                                            destination_2,
                                            realm_2.clone(),
                                            Box::new(state_2.clone()),
                                            None::<(
                                                Option<self::JavaScriptSpeciesConstructionState>,
                                            )>,
                                        ),
                                    ),
                                )
                            } else {
                                machine.clone()
                            };
                            self::accept_java_script_call_array_slice(
                                program.clone(),
                                next_2.clone(),
                                destination_2,
                                realm_2.clone(),
                                self::resume_java_script_array_slice_construction(
                                    shared_constructor_field1.clone(),
                                    directory.clone(),
                                    state_2.clone(),
                                    completion.clone(),
                                ),
                            )
                        }
                    }
                    self::JavaScriptCallPending::WaitArrayTransform(
                        destination_3,
                        realm_3,
                        state_3,
                        _,
                    ) => {
                        let state_3 = *state_3;
                        {
                            let next_3 = if clear_construction {
                                self::replace_java_script_call_frame(
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone(),
                                        self::JavaScriptCallPending::WaitArrayTransform(
                                            destination_3,
                                            realm_3.clone(),
                                            Box::new(state_3.clone()),
                                            None::<(
                                                Option<self::JavaScriptSpeciesConstructionState>,
                                            )>,
                                        ),
                                    ),
                                )
                            } else {
                                machine.clone()
                            };
                            self::accept_java_script_call_array_transform(
                                program.clone(),
                                next_3.clone(),
                                destination_3,
                                realm_3.clone(),
                                self::resume_java_script_array_transform_construction(
                                    shared_constructor_field1.clone(),
                                    directory.clone(),
                                    state_3.clone(),
                                    completion.clone(),
                                ),
                            )
                        }
                    }
                    _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        self::ml_string::Text::from_units(vec![
                            115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117,
                            99, 116, 105, 111, 110, 32, 114, 101, 115, 117, 109, 112, 116, 105,
                            111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                            97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32,
                            116, 114, 97, 110, 115, 102, 111, 114, 109,
                        ]),
                    ),
                }
            }
        }
    }
}
