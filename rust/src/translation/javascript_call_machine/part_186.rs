// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptArraySearchTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
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
use super::read_java_script_executable_call_function;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_array_search(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptArraySearchTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptArraySearchTransition::ArraySearchContinue(
                    source_ml_source2_data_property_value32958,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32958 =
                        *source_ml_source2_data_property_value32958;
                    source_ml_source2_data_property_value32958.clone()
                }
                self::JavaScriptArraySearchTransition::ArraySearchCall(
                    source_ml_source2_data_property_value32959,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32959 =
                        *source_ml_source2_data_property_value32959;
                    source_ml_source2_data_property_value32959.clone()
                }
                self::JavaScriptArraySearchTransition::ArraySearchCompleted(
                    source_ml_source2_data_property_value32960,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32960 =
                        *source_ml_source2_data_property_value32960;
                    source_ml_source2_data_property_value32960.clone()
                }
                self::JavaScriptArraySearchTransition::ArraySearchThrow(
                    source_ml_source2_data_property_value32961,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32961 =
                        *source_ml_source2_data_property_value32961;
                    source_ml_source2_data_property_value32961.clone()
                }
                self::JavaScriptArraySearchTransition::ArraySearchError(
                    source_ml_source2_data_property_value32962,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32962 =
                        *source_ml_source2_data_property_value32962;
                    source_ml_source2_data_property_value32962.clone()
                }
                self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                    source_ml_source2_data_property_value32963,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32963 =
                        *source_ml_source2_data_property_value32963;
                    source_ml_source2_data_property_value32963.clone()
                }
                self::JavaScriptArraySearchTransition::ArraySearchResourceLimit(
                    source_ml_source2_data_property_value32964,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32964 =
                        *source_ml_source2_data_property_value32964;
                    source_ml_source2_data_property_value32964.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value32965,
                ) => source_ml_source2_data_property_value32965.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                let frame = {
                    let source_ml_source2_array_read_value32968 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_array_read_index32969 = ({
                            let ml_reference_array2155 = shared_constructor_field1.clone();
                            let ml_reference_projection2155 =
                                ml_reference_array2155.borrow().len() as f64;
                            ml_reference_projection2155
                        } - 1f64);
                        {
                            let ml_reference_array2156 =
                                source_ml_source2_array_read_value32968.clone();
                            let ml_reference_position2156 = source_ml_source2_array_read_index32969;
                            let ml_reference_projection2156 = self::ml_array::optional_at(
                                &*ml_reference_array2156.borrow(),
                                self::ml_array::number_index(ml_reference_position2156),
                            );
                            ml_reference_projection2156
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
                            self::JavaScriptArraySearchTransition::ArraySearchContinue(
                                source_ml_source2_data_property_value32970,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32970 =
                                    *source_ml_source2_data_property_value32970;
                                source_ml_source2_data_property_value32970.clone()
                            }
                            self::JavaScriptArraySearchTransition::ArraySearchCall(
                                source_ml_source2_data_property_value32971,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32971 =
                                    *source_ml_source2_data_property_value32971;
                                source_ml_source2_data_property_value32971.clone()
                            }
                            self::JavaScriptArraySearchTransition::ArraySearchCompleted(
                                source_ml_source2_data_property_value32972,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32972 =
                                    *source_ml_source2_data_property_value32972;
                                source_ml_source2_data_property_value32972.clone()
                            }
                            self::JavaScriptArraySearchTransition::ArraySearchThrow(
                                source_ml_source2_data_property_value32973,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32973 =
                                    *source_ml_source2_data_property_value32973;
                                source_ml_source2_data_property_value32973.clone()
                            }
                            self::JavaScriptArraySearchTransition::ArraySearchError(
                                source_ml_source2_data_property_value32974,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32974 =
                                    *source_ml_source2_data_property_value32974;
                                source_ml_source2_data_property_value32974.clone()
                            }
                            self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                                source_ml_source2_data_property_value32975,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32975 =
                                    *source_ml_source2_data_property_value32975;
                                source_ml_source2_data_property_value32975.clone()
                            }
                            self::JavaScriptArraySearchTransition::ArraySearchResourceLimit(
                                source_ml_source2_data_property_value32976,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32976 =
                                    *source_ml_source2_data_property_value32976;
                                source_ml_source2_data_property_value32976.clone()
                            }
                        }),
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101,
                            113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                            97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105,
                            110, 115, 105, 99, 32, 102, 114, 97, 109, 101,
                        ]),
                    )
                } else {
                    match transition.clone() {
                        self::JavaScriptArraySearchTransition::ArraySearchContinue(heap, state) => {
                            let heap = *heap;
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArraySearch(
                                            destination,
                                            realm.clone(),
                                            Box::new(state.clone()),
                                        ),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptArraySearchTransition::ArraySearchCall(
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
                                        self::JavaScriptCallPending::WaitArraySearch(
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
                        self::JavaScriptArraySearchTransition::ArraySearchCompleted(
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
                        self::JavaScriptArraySearchTransition::ArraySearchThrow(_, value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(current.clone(), value_2.clone())
                        }
                        self::JavaScriptArraySearchTransition::ArraySearchError(
                            _,
                            error_class,
                            reason,
                        ) => self::request_java_script_call_error(
                            current.clone(),
                            realm.clone(),
                            &(error_class),
                            &(reason),
                        ),
                        self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                            heap_4,
                            reason_2,
                        ) => {
                            let heap_4 = *heap_4;
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(heap_4.clone()),
                                reason_2.clone(),
                            )
                        }
                        self::JavaScriptArraySearchTransition::ArraySearchResourceLimit(
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
                                        self::JavaScriptCallPending::WaitArraySearch(
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
pub fn read_java_script_call_array_transform_directory(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
) -> Option<self::JavaScriptArraySpeciesDirectory> {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value32985 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index32986 = ({
                            let ml_reference_array2157 = shared_constructor_field2.clone();
                            let ml_reference_projection2157 =
                                ml_reference_array2157.borrow().len() as f64;
                            ml_reference_projection2157
                        } - 1f64);
                        {
                            let ml_reference_array2158 =
                                source_ml_source2_array_read_value32985.clone();
                            let ml_reference_position2158 = source_ml_source2_array_read_index32986;
                            let ml_reference_projection2158 = self::ml_array::optional_at(
                                &*ml_reference_array2158.borrow(),
                                self::ml_array::number_index(ml_reference_position2158),
                            );
                            ml_reference_projection2158
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    None::<self::JavaScriptArraySpeciesDirectory>
                } else {
                    {
                        let fn_ = self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field1.clone(),
                            match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    source_ml_source2_data_property_value32988,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value32988.clone(),
                            },
                        );
                        {
                            let instruction = if ((match frame
                                .clone()
                                .expect("a proven present value")
                            {
                                self::JavaScriptCallFrame::CallFrame(
                                    _,
                                    source_ml_source2_data_property_value32990,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value32990,
                            }) > (0f64))
                            {
                                {
                                    let source_ml_source2_optional_chain_receiver32991 =
                                        (fn_.clone(),);
                                    match if match (source_ml_source2_optional_chain_receiver32991
                                        .clone())
                                    .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        Some({
                                            let source_ml_source1_temporary_optional_chain1408 =
                                                ((source_ml_source2_optional_chain_receiver32991
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            {
                                                let source_ml_source2_array_read_value32994 = match source_ml_source1_temporary_optional_chain1408.clone() {
                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value32992, _, _) => {
                                                        source_ml_source2_data_property_value32992.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_array_read_index32995 = (match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value32993, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value32993
                                                        }
                                                    } - 1f64);
                                                    {
                                                        let ml_reference_array2159 =
                                                            source_ml_source2_array_read_value32994
                                                                .clone();
                                                        let ml_reference_position2159 =
                                                            source_ml_source2_array_read_index32995;
                                                        let ml_reference_projection2159 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array2159.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position2159,
                                                                ),
                                                            );
                                                        ml_reference_projection2159
                                                    }
                                                }
                                            }
                                        })
                                    } else {
                                        None::<self::JavaScriptCallInstruction>
                                    } {
                                        Some(ml_nested) => match ml_nested {
                                            Some(ml_present) => Some(ml_present),
                                            None => None,
                                        },
                                        None => None,
                                    }
                                }
                            } else {
                                None::<self::JavaScriptCallInstruction>
                            };
                            if ((((match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value32997, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value32997.clone()
                                }
                            }) != realm)) || match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((match instruction.clone().expect("a proven present value") {
                                self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            } && match instruction.clone().expect("a proven present value") {
                                self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            }) && match instruction.clone().expect("a proven present value") {
                                self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            })) || ((((match instruction.clone().expect("a proven present value") {
                                self::JavaScriptCallInstruction::ArrayConcatenation(source_ml_source2_data_property_value33012, _, _, _) => {
                                    Some((source_ml_source2_data_property_value33012,))
                                }
                                self::JavaScriptCallInstruction::ArraySlice(source_ml_source2_data_property_value33013, _, _) => {
                                    Some((source_ml_source2_data_property_value33013,))
                                }
                                self::JavaScriptCallInstruction::ArrayTransform(source_ml_source2_data_property_value33014, _, _, _) => {
                                    Some((source_ml_source2_data_property_value33014,))
                                }
                                _ => {
                                    None::<(f64,)>
                                }
                            }).clone().expect("a proven present value")).0) != destination)) {
                                None::<self::JavaScriptArraySpeciesDirectory>
                            } else {
                                Some((((match instruction.clone().expect("a proven present value") {
                                    self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, source_ml_source2_data_property_value33015) => {
                                        let source_ml_source2_data_property_value33015 = *source_ml_source2_data_property_value33015;
                                        Some((source_ml_source2_data_property_value33015.clone(),))
                                    }
                                    self::JavaScriptCallInstruction::ArraySlice(_, _, source_ml_source2_data_property_value33016) => {
                                        let source_ml_source2_data_property_value33016 = *source_ml_source2_data_property_value33016;
                                        Some((source_ml_source2_data_property_value33016.clone(),))
                                    }
                                    self::JavaScriptCallInstruction::ArrayTransform(_, _, source_ml_source2_data_property_value33017, _) => {
                                        let source_ml_source2_data_property_value33017 = *source_ml_source2_data_property_value33017;
                                        Some((source_ml_source2_data_property_value33017.clone(),))
                                    }
                                    _ => {
                                        None::<(self::JavaScriptArraySpeciesDirectory,)>
                                    }
                                }).clone().expect("a proven present value")).0).clone())
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
pub fn accept_java_script_call_array_concatenation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptArrayConcatenationTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(
                    source_ml_source2_data_property_value33018,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33018 =
                        *source_ml_source2_data_property_value33018;
                    source_ml_source2_data_property_value33018.clone()
                }
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationCall(
                    source_ml_source2_data_property_value33019,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33019 =
                        *source_ml_source2_data_property_value33019;
                    source_ml_source2_data_property_value33019.clone()
                }
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationAllocate(
                    source_ml_source2_data_property_value33020,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33020 =
                        *source_ml_source2_data_property_value33020;
                    source_ml_source2_data_property_value33020.clone()
                }
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationConstruct(
                    source_ml_source2_data_property_value33021,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33021 =
                        *source_ml_source2_data_property_value33021;
                    source_ml_source2_data_property_value33021.clone()
                }
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationNeedAuthority(
                    source_ml_source2_data_property_value33022,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33022 =
                        *source_ml_source2_data_property_value33022;
                    source_ml_source2_data_property_value33022.clone()
                }
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationCompleted(
                    source_ml_source2_data_property_value33023,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33023 =
                        *source_ml_source2_data_property_value33023;
                    source_ml_source2_data_property_value33023.clone()
                }
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationThrow(
                    source_ml_source2_data_property_value33024,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33024 =
                        *source_ml_source2_data_property_value33024;
                    source_ml_source2_data_property_value33024.clone()
                }
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(
                    source_ml_source2_data_property_value33025,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33025 =
                        *source_ml_source2_data_property_value33025;
                    source_ml_source2_data_property_value33025.clone()
                }
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(
                    source_ml_source2_data_property_value33026,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33026 =
                        *source_ml_source2_data_property_value33026;
                    source_ml_source2_data_property_value33026.clone()
                }
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationResourceLimit(
                    source_ml_source2_data_property_value33027,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33027 =
                        *source_ml_source2_data_property_value33027;
                    source_ml_source2_data_property_value33027.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value33028,
                ) => source_ml_source2_data_property_value33028.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                let frame = {
                    let source_ml_source2_array_read_value33031 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_array_read_index33032 = ({
                            let ml_reference_array2160 = shared_constructor_field1.clone();
                            let ml_reference_projection2160 =
                                ml_reference_array2160.borrow().len() as f64;
                            ml_reference_projection2160
                        } - 1f64);
                        {
                            let ml_reference_array2161 =
                                source_ml_source2_array_read_value33031.clone();
                            let ml_reference_position2161 = source_ml_source2_array_read_index33032;
                            let ml_reference_projection2161 = self::ml_array::optional_at(
                                &*ml_reference_array2161.borrow(),
                                self::ml_array::number_index(ml_reference_position2161),
                            );
                            ml_reference_projection2161
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(source_ml_source2_data_property_value33033, _) => {
                                let source_ml_source2_data_property_value33033 = *source_ml_source2_data_property_value33033;
                                source_ml_source2_data_property_value33033.clone()
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationCall(source_ml_source2_data_property_value33034, _, _, _, _) => {
                                let source_ml_source2_data_property_value33034 = *source_ml_source2_data_property_value33034;
                                source_ml_source2_data_property_value33034.clone()
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationAllocate(source_ml_source2_data_property_value33035, _, _, _) => {
                                let source_ml_source2_data_property_value33035 = *source_ml_source2_data_property_value33035;
                                source_ml_source2_data_property_value33035.clone()
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationConstruct(source_ml_source2_data_property_value33036, _, _, _, _) => {
                                let source_ml_source2_data_property_value33036 = *source_ml_source2_data_property_value33036;
                                source_ml_source2_data_property_value33036.clone()
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationNeedAuthority(source_ml_source2_data_property_value33037, _, _) => {
                                let source_ml_source2_data_property_value33037 = *source_ml_source2_data_property_value33037;
                                source_ml_source2_data_property_value33037.clone()
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationCompleted(source_ml_source2_data_property_value33038, _) => {
                                let source_ml_source2_data_property_value33038 = *source_ml_source2_data_property_value33038;
                                source_ml_source2_data_property_value33038.clone()
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationThrow(source_ml_source2_data_property_value33039, _) => {
                                let source_ml_source2_data_property_value33039 = *source_ml_source2_data_property_value33039;
                                source_ml_source2_data_property_value33039.clone()
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(source_ml_source2_data_property_value33040, _, _) => {
                                let source_ml_source2_data_property_value33040 = *source_ml_source2_data_property_value33040;
                                source_ml_source2_data_property_value33040.clone()
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(source_ml_source2_data_property_value33041, _) => {
                                let source_ml_source2_data_property_value33041 = *source_ml_source2_data_property_value33041;
                                source_ml_source2_data_property_value33041.clone()
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationResourceLimit(source_ml_source2_data_property_value33042, _, _) => {
                                let source_ml_source2_data_property_value33042 = *source_ml_source2_data_property_value33042;
                                source_ml_source2_data_property_value33042.clone()
                            }
                        }), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(heap, state) => {
                                let heap = *heap;
                                let state = *state;
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayConcatenation(destination, realm.clone(), Box::new(state.clone()), None::<(Option<self::JavaScriptSpeciesConstructionState>,)>)))))
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationCall(heap_2, state_2, callee, receiver, arguments) => {
                                let heap_2 = *heap_2;
                                let state_2 = *state_2;
                                let callee = *callee;
                                let receiver = *receiver;
                                self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayConcatenation(destination, realm.clone(), Box::new(state_2.clone()), None::<(Option<self::JavaScriptSpeciesConstructionState>,)>))), callee.clone(), receiver.clone(), arguments.clone())
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationAllocate(heap_3, state_3, prototype, _) => {
                                let heap_3 = *heap_3;
                                let state_3 = *state_3;
                                let prototype = *prototype;
                                self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayConcatenation(destination, realm.clone(), Box::new(state_3.clone()), None::<(Option<self::JavaScriptSpeciesConstructionState>,)>)))), Box::new(prototype.clone()))
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationConstruct(heap_4, state_4, constructor, _, fact) => {
                                let heap_4 = *heap_4;
                                let state_4 = *state_4;
                                let constructor = *constructor;
                                let fact = *fact;
                                {
                                    let suspended = self::replace_java_script_call_frame(heap_4.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayConcatenation(destination, realm.clone(), Box::new(state_4.clone()), None::<(Option<self::JavaScriptSpeciesConstructionState>,)>)));
                                    {
                                        let directory = self::read_java_script_call_array_transform_directory(program.clone(), suspended.clone(), destination, realm.clone());
                                        if match (directory.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_4.clone()), self::ml_string::Text::from_units(vec![99, 117, 115, 116, 111, 109, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                        } else {
                                            self::accept_java_script_call_species_construction(program.clone(), suspended.clone(), self::begin_java_script_species_construction(program.clone(), heap_4.clone(), directory.clone().expect("a proven present value"), constructor.clone(), fact.clone(), 0f64))
                                        }
                                    }
                                }
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationNeedAuthority(heap_5, state_5, constructor_2) => {
                                let heap_5 = *heap_5;
                                let state_5 = *state_5;
                                let constructor_2 = *constructor_2;
                                self::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(Box::new(self::replace_java_script_call_frame(heap_5.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayConcatenation(destination, realm.clone(), Box::new(state_5.clone()), None::<(Option<self::JavaScriptSpeciesConstructionState>,)>)))), Box::new(constructor_2.clone()))
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationCompleted(heap_6, value) => {
                                let heap_6 = *heap_6;
                                let value = *value;
                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_6.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationThrow(_, value_2) => {
                                let value_2 = *value_2;
                                self::throw_java_script_call_value(current.clone(), value_2.clone())
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(_, error_class, reason) => {
                                self::request_java_script_call_error(current.clone(), realm.clone(), &(error_class), &(reason))
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(heap_7, reason_2) => {
                                let heap_7 = *heap_7;
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_7.clone()), reason_2.clone())
                            }
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationResourceLimit(heap_8, state_6, reason_3) => {
                                let heap_8 = *heap_8;
                                let state_6 = *state_6;
                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(heap_8.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayConcatenation(destination, realm.clone(), Box::new(state_6.clone()), None::<(Option<self::JavaScriptSpeciesConstructionState>,)>)))), reason_3.clone())
                            }
                        }
                }
            }
        }
    }
}
