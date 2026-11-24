// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSpeciesConstructionState;
use super::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Result;
use super::accept_java_script_call_argument_rest;
use super::accept_java_script_call_array_assignment;
use super::accept_java_script_call_array_callback;
use super::accept_java_script_call_array_concatenation;
use super::accept_java_script_call_array_iteration;
use super::accept_java_script_call_array_search;
use super::accept_java_script_call_array_slice;
use super::accept_java_script_call_array_spread;
use super::accept_java_script_call_array_transform;
use super::accept_java_script_call_collection_for_each;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_instance_of;
use super::accept_java_script_call_iterator_protocol;
use super::accept_java_script_call_key;
use super::accept_java_script_call_object_enumeration;
use super::accept_java_script_call_object_integrity;
use super::accept_java_script_call_object_spread;
use super::accept_java_script_call_property_definitions;
use super::accept_java_script_call_property_reflection;
use super::accept_java_script_call_species_construction;
use super::accept_java_script_collection_iterator_result_completion;
use super::create_java_script_call_frame;
use super::execute_java_script_call_instruction;
use super::ml;
use super::ml_string;
use super::project_java_script_call_iterator_operation;
use super::push_java_script_call;
use super::read_java_script_call_array_transform_directory;
use super::read_java_script_call_program_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_ordinary_value_error;
use super::resume_java_script_call_allocation;
use super::source_ml_source1_temporary_begin_java_script_call_machine_loop1514;
use super::step_java_script_argument_rest;
use super::step_java_script_array_assignment;
use super::step_java_script_array_callback;
use super::step_java_script_array_concatenation;
use super::step_java_script_array_iterator_next;
use super::step_java_script_array_search;
use super::step_java_script_array_slice;
use super::step_java_script_array_spread;
use super::step_java_script_array_transform;
use super::step_java_script_call_number;
use super::step_java_script_call_text;
use super::step_java_script_collection_for_each;
use super::step_java_script_collection_iterator_result;
use super::step_java_script_instance_of;
use super::step_java_script_iterator_protocol;
use super::step_java_script_object_enumeration;
use super::step_java_script_object_integrity;
use super::step_java_script_object_spread;
use super::step_java_script_property_definition;
use super::step_java_script_property_definitions;
use super::step_java_script_property_key_conversion;
use super::step_java_script_property_reflection;
use super::step_java_script_species_construction;
use super::test_java_script_collection_constructor_mode;
use super::test_java_script_ordinary_class_constructor_mode;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_call_pending_operation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            _,
            _,
            _,
            shared_constructor_field5,
            _,
            shared_constructor_field6,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field6 = *shared_constructor_field6;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(shared_constructor_field4, _) => {
                    let shared_constructor_field4 = *shared_constructor_field4;
                    {
                        let pending = shared_constructor_field6.clone();
                        match pending.clone() {
                            self::JavaScriptCallPending::CallIdle => {
                                self::execute_java_script_call_instruction(
                                    program.clone(),
                                    machine.clone(),
                                    frame.clone(),
                                )
                            }
                            self::JavaScriptCallPending::WaitArgumentRest(
                                destination,
                                realm,
                                state,
                            ) => {
                                let state = *state;
                                self::accept_java_script_call_argument_rest(
                                    machine.clone(),
                                    destination,
                                    realm.clone(),
                                    self::step_java_script_argument_rest(
                                        shared_constructor_field4.clone(),
                                        shared_constructor_field5.clone(),
                                        state.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitIteratorProtocol(
                                _,
                                _,
                                _,
                                _,
                                _,
                                state_2,
                            ) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_iterator_protocol(
                                    program.clone(),
                                    machine.clone(),
                                    self::project_java_script_call_iterator_operation(
                                        pending.clone(),
                                    ),
                                    self::step_java_script_iterator_protocol(
                                        shared_constructor_field4.clone(),
                                        state_2.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitArraySpread(
                                destination_2,
                                realm_2,
                                state_3,
                            ) => {
                                let state_3 = *state_3;
                                self::accept_java_script_call_array_spread(
                                    program.clone(),
                                    machine.clone(),
                                    destination_2,
                                    realm_2.clone(),
                                    self::step_java_script_array_spread(
                                        shared_constructor_field4.clone(),
                                        state_3.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitArrayIteratorNext(_, _, state_4) => {
                                let state_4 = *state_4;
                                self::accept_java_script_call_array_iteration(
                                    program.clone(),
                                    machine.clone(),
                                    self::step_java_script_array_iterator_next(
                                        shared_constructor_field4.clone(),
                                        state_4.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitCollectionIteratorResult(
                                destination_3,
                                realm_3,
                                state_5,
                            ) => {
                                let state_5 = *state_5;
                                self::accept_java_script_collection_iterator_result_completion(
                                    program.clone(),
                                    machine.clone(),
                                    frame.clone(),
                                    destination_3,
                                    realm_3.clone(),
                                    self::step_java_script_collection_iterator_result(
                                        shared_constructor_field4.clone(),
                                        state_5.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitArrayConcatenation(
                                destination_4,
                                realm_4,
                                state_6,
                                pending_construction,
                            ) => {
                                let state_6 = *state_6;
                                {
                                    let directory =
                                        self::read_java_script_call_array_transform_directory(
                                            program.clone(),
                                            machine.clone(),
                                            destination_4,
                                            realm_4.clone(),
                                        );
                                    if match (directory.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(shared_constructor_field4.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                65, 114, 114, 97, 121, 32, 99, 111, 110, 99, 97,
                                                116, 101, 110, 97, 116, 105, 111, 110, 32, 115,
                                                116, 101, 112, 32, 114, 101, 116, 97, 105, 110,
                                                115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112,
                                                101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111,
                                                114, 105, 116, 121,
                                            ]),
                                        )
                                    } else {
                                        {
                                            let construction = ((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                            if !(match (construction.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            }) {
                                                self::accept_java_script_call_species_construction(
                                                    program.clone(),
                                                    machine.clone(),
                                                    self::step_java_script_species_construction(
                                                        program.clone(),
                                                        shared_constructor_field4.clone(),
                                                        directory
                                                            .clone()
                                                            .expect("a proven present value"),
                                                        construction
                                                            .clone()
                                                            .expect("a proven present value"),
                                                    ),
                                                )
                                            } else {
                                                self::accept_java_script_call_array_concatenation(
                                                    program.clone(),
                                                    machine.clone(),
                                                    destination_4,
                                                    realm_4.clone(),
                                                    self::step_java_script_array_concatenation(
                                                        shared_constructor_field4.clone(),
                                                        directory
                                                            .clone()
                                                            .expect("a proven present value"),
                                                        state_6.clone(),
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                            self::JavaScriptCallPending::WaitArraySlice(
                                destination_5,
                                realm_5,
                                state_7,
                                pending_construction_2,
                            ) => {
                                let state_7 = *state_7;
                                {
                                    let directory_2 =
                                        self::read_java_script_call_array_transform_directory(
                                            program.clone(),
                                            machine.clone(),
                                            destination_5,
                                            realm_5.clone(),
                                        );
                                    if match (directory_2.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(shared_constructor_field4.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                65, 114, 114, 97, 121, 32, 115, 108, 105, 99, 101,
                                                32, 115, 116, 101, 112, 32, 114, 101, 116, 97, 105,
                                                110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                                                112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104,
                                                111, 114, 105, 116, 121,
                                            ]),
                                        )
                                    } else {
                                        {
                                            let construction_2 = ((pending_construction_2.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                            if !(match (construction_2.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            }) {
                                                self::accept_java_script_call_species_construction(
                                                    program.clone(),
                                                    machine.clone(),
                                                    self::step_java_script_species_construction(
                                                        program.clone(),
                                                        shared_constructor_field4.clone(),
                                                        directory_2
                                                            .clone()
                                                            .expect("a proven present value"),
                                                        construction_2
                                                            .clone()
                                                            .expect("a proven present value"),
                                                    ),
                                                )
                                            } else {
                                                self::accept_java_script_call_array_slice(
                                                    program.clone(),
                                                    machine.clone(),
                                                    destination_5,
                                                    realm_5.clone(),
                                                    self::step_java_script_array_slice(
                                                        shared_constructor_field4.clone(),
                                                        directory_2
                                                            .clone()
                                                            .expect("a proven present value"),
                                                        state_7.clone(),
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                            self::JavaScriptCallPending::WaitArrayTransform(
                                destination_6,
                                realm_6,
                                state_8,
                                pending_construction_3,
                            ) => {
                                let state_8 = *state_8;
                                {
                                    let directory_3 =
                                        self::read_java_script_call_array_transform_directory(
                                            program.clone(),
                                            machine.clone(),
                                            destination_6,
                                            realm_6.clone(),
                                        );
                                    if match (directory_3.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(shared_constructor_field4.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115,
                                                102, 111, 114, 109, 32, 115, 116, 101, 112, 32,
                                                114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                                115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111,
                                                117, 114, 99, 101, 32, 100, 105, 114, 101, 99, 116,
                                                111, 114, 121,
                                            ]),
                                        )
                                    } else {
                                        {
                                            let construction_3 = ((pending_construction_3.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                            if !(match (construction_3.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            }) {
                                                self::accept_java_script_call_species_construction(
                                                    program.clone(),
                                                    machine.clone(),
                                                    self::step_java_script_species_construction(
                                                        program.clone(),
                                                        shared_constructor_field4.clone(),
                                                        directory_3
                                                            .clone()
                                                            .expect("a proven present value"),
                                                        construction_3
                                                            .clone()
                                                            .expect("a proven present value"),
                                                    ),
                                                )
                                            } else {
                                                self::accept_java_script_call_array_transform(
                                                    program.clone(),
                                                    machine.clone(),
                                                    destination_6,
                                                    realm_6.clone(),
                                                    self::step_java_script_array_transform(
                                                        shared_constructor_field4.clone(),
                                                        directory_3
                                                            .clone()
                                                            .expect("a proven present value"),
                                                        state_8.clone(),
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                            self::JavaScriptCallPending::WaitArraySearch(
                                destination_7,
                                realm_7,
                                state_9,
                            ) => {
                                let state_9 = *state_9;
                                self::accept_java_script_call_array_search(
                                    program.clone(),
                                    machine.clone(),
                                    destination_7,
                                    realm_7.clone(),
                                    self::step_java_script_array_search(
                                        shared_constructor_field4.clone(),
                                        state_9.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitCollectionForEach(
                                destination_8,
                                realm_8,
                                state_10,
                            ) => {
                                let state_10 = *state_10;
                                self::accept_java_script_call_collection_for_each(
                                    program.clone(),
                                    machine.clone(),
                                    destination_8,
                                    realm_8.clone(),
                                    self::step_java_script_collection_for_each(
                                        shared_constructor_field4.clone(),
                                        state_10.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitArrayCallback(
                                destination_9,
                                realm_9,
                                state_11,
                            ) => {
                                let state_11 = *state_11;
                                self::accept_java_script_call_array_callback(
                                    program.clone(),
                                    machine.clone(),
                                    destination_9,
                                    realm_9.clone(),
                                    self::step_java_script_array_callback(
                                        shared_constructor_field4.clone(),
                                        state_11.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitObjectSpread(
                                destination_10,
                                realm_10,
                                state_12,
                            ) => {
                                let state_12 = *state_12;
                                match state_12.clone() {
                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(
                                        _,
                                        shared_constructor_field1,
                                        _,
                                        _,
                                        _,
                                        shared_constructor_field2,
                                        _,
                                        shared_constructor_field3,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let shared_constructor_field1 = *shared_constructor_field1;
                                        let shared_constructor_field3 = *shared_constructor_field3;
                                        if (shared_constructor_field2
                                            == (self::ml_string::Text::from_units(vec![
                                                103, 101, 116, 116, 101, 114,
                                            ])))
                                        {
                                            self::push_java_script_call(
                                                program.clone(),
                                                machine.clone(),
                                                shared_constructor_field3.clone(),
                                                shared_constructor_field1.clone(),
                                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                    self::JavaScriptSemanticValue,
                                                >::new(
                                                ))),
                                            )
                                        } else {
                                            self::accept_java_script_call_object_spread(
                                                program.clone(),
                                                machine.clone(),
                                                destination_10,
                                                realm_10.clone(),
                                                self::step_java_script_object_spread(
                                                    shared_constructor_field4.clone(),
                                                    state_12.clone(),
                                                ),
                                            )
                                        }
                                    }
                                }
                            }
                            self::JavaScriptCallPending::WaitObjectEnumeration(
                                destination_11,
                                realm_11,
                                state_13,
                            ) => {
                                let state_13 = *state_13;
                                self::accept_java_script_call_object_enumeration(
                                    program.clone(),
                                    machine.clone(),
                                    destination_11,
                                    realm_11.clone(),
                                    self::step_java_script_object_enumeration(
                                        shared_constructor_field4.clone(),
                                        state_13.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitPropertyDefinitions(
                                destination_12,
                                realm_12,
                                state_14,
                            ) => {
                                let state_14 = *state_14;
                                self::accept_java_script_call_property_definitions(
                                    program.clone(),
                                    machine.clone(),
                                    destination_12,
                                    realm_12.clone(),
                                    self::step_java_script_property_definitions(
                                        shared_constructor_field4.clone(),
                                        state_14.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitPropertyReflection(
                                destination_13,
                                realm_13,
                                state_15,
                            ) => {
                                let state_15 = *state_15;
                                self::accept_java_script_call_property_reflection(
                                    machine.clone(),
                                    destination_13,
                                    realm_13.clone(),
                                    self::step_java_script_property_reflection(
                                        shared_constructor_field4.clone(),
                                        state_15.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitInstanceOf(
                                destination_14,
                                realm_14,
                                state_16,
                            ) => {
                                let state_16 = *state_16;
                                self::accept_java_script_call_instance_of(
                                    program.clone(),
                                    machine.clone(),
                                    destination_14,
                                    realm_14.clone(),
                                    self::step_java_script_instance_of(
                                        shared_constructor_field4.clone(),
                                        state_16.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitObjectIntegrity(
                                destination_15,
                                realm_15,
                                state_17,
                            ) => {
                                let state_17 = *state_17;
                                self::accept_java_script_call_object_integrity(
                                    machine.clone(),
                                    destination_15,
                                    realm_15.clone(),
                                    self::step_java_script_object_integrity(
                                        shared_constructor_field4.clone(),
                                        state_17.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitObjectPrototype(
                                _,
                                realm_16,
                                input,
                            ) => {
                                let input = *input;
                                self::JavaScriptCallMachineTransition::MachineNeedObjectPrototype(
                                    Box::new(machine.clone()),
                                    Box::new(input.clone()),
                                    realm_16.clone(),
                                )
                            }
                            self::JavaScriptCallPending::WaitArrayAssignment(
                                destination_16,
                                realm_17,
                                state_18,
                            ) => {
                                let state_18 = *state_18;
                                self::accept_java_script_call_array_assignment(
                                    program.clone(),
                                    machine.clone(),
                                    destination_16,
                                    realm_17.clone(),
                                    self::step_java_script_array_assignment(
                                        shared_constructor_field4.clone(),
                                        state_18.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitKey(
                                destination_17,
                                realm_18,
                                state_19,
                            ) => {
                                let state_19 = *state_19;
                                self::accept_java_script_call_key(
                                    program.clone(),
                                    machine.clone(),
                                    destination_17,
                                    realm_18.clone(),
                                    self::step_java_script_property_key_conversion(
                                        shared_constructor_field4.clone(),
                                        state_19.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitDefinition(
                                destination_18,
                                realm_19,
                                state_20,
                            ) => {
                                let state_20 = *state_20;
                                self::accept_java_script_call_definition(
                                    program.clone(),
                                    machine.clone(),
                                    destination_18,
                                    realm_19.clone(),
                                    self::step_java_script_property_definition(
                                        shared_constructor_field4.clone(),
                                        state_20.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitNumber(
                                destination_19,
                                realm_20,
                                state_21,
                            ) => {
                                let state_21 = *state_21;
                                self::step_java_script_call_number(
                                    program.clone(),
                                    machine.clone(),
                                    destination_19,
                                    realm_20.clone(),
                                    state_21.clone(),
                                )
                            }
                            self::JavaScriptCallPending::WaitText(
                                destination_20,
                                realm_21,
                                state_22,
                            ) => {
                                let state_22 = *state_22;
                                self::step_java_script_call_text(
                                    program.clone(),
                                    machine.clone(),
                                    destination_20,
                                    realm_21.clone(),
                                    state_22.clone(),
                                )
                            }
                            _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field4.clone()),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108,
                                    108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                    111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101,
                                    116, 105, 111, 110,
                                ]),
                            ),
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_call_machine(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallMachineTransition {
    {
        let error = self::read_java_script_call_program_error(program.clone(), heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallMachineTransition::MachineInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            {
                let fn_ = self::read_java_script_executable_call_function(
                    program.clone(),
                    heap.clone(),
                    identity.clone(),
                );
                if match (fn_.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            116, 104, 101, 32, 101, 110, 116, 114, 121, 32, 99, 97, 108, 108, 97,
                            98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97,
                            99, 116, 117, 97, 108, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108,
                            101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110,
                        ]),
                    )
                } else {
                    if self::test_java_script_ordinary_class_constructor_mode(
                        &(match fn_.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(
                                _,
                                _,
                                source_ml_source2_data_property_value36457,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value36457.clone(),
                        }),
                    ) {
                        self::JavaScriptCallMachineTransition::MachineInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108,
                                97, 115, 115, 32, 101, 110, 116, 114, 121, 32, 114, 101, 113, 117,
                                105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                108, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116,
                                114, 117, 99, 116, 105, 111, 110, 32, 111, 112, 101, 114, 97, 116,
                                105, 111, 110,
                            ]),
                        )
                    } else {
                        if self::test_java_script_collection_constructor_mode(
                            &(match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value36458,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value36458.clone(),
                            }),
                        ) {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32,
                                    101, 110, 116, 114, 121, 32, 114, 101, 113, 117, 105, 114, 101,
                                    115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                                    111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117,
                                    99, 116, 105, 111, 110, 32, 111, 112, 101, 114, 97, 116, 105,
                                    111, 110,
                                ]),
                            )
                        } else {
                            if (({
                                let ml_reference_array2406 = arguments_.clone();
                                let ml_reference_projection2406 =
                                    ml_reference_array2406.borrow().len() as f64;
                                ml_reference_projection2406
                            }) > (1048576f64))
                            {
                                self::JavaScriptCallMachineTransition::MachineInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101,
                                        110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116,
                                        104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                        99, 97, 108, 108, 32, 115, 116, 111, 114, 97, 103, 101, 32,
                                        98, 117, 100, 103, 101, 116,
                                    ]),
                                )
                            } else {
                                {
                                    let receiver_error =
                                        self::read_java_script_ordinary_value_error(
                                            heap.clone(),
                                            receiver.clone(),
                                        );
                                    if (receiver_error
                                        != (self::ml_string::Text::from_units(vec![])))
                                    {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(heap.clone()),
                                            receiver_error.clone(),
                                        )
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index1513 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s36459 = self::source_ml_source1_temporary_begin_java_script_call_machine_loop1514(heap.clone(), arguments_.clone(), source_ml_source1_temporary_index1513.clone());
                                                match source_ml_source2_s36459.clone() {
                                                    self::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Done => {
                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_.clone())])))))
                                                    }
                                                    self::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop1514Return(source_ml_source1_temporary_result1514) => {
                                                        let source_ml_source1_temporary_result1514 = *source_ml_source1_temporary_result1514;
                                                        source_ml_source1_temporary_result1514.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_object_allocation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    identity: self::ml::Big,
) -> self::JavaScriptCallMachineTransition {
    self::resume_java_script_call_allocation(program.clone(), machine.clone(), identity.clone())
}
