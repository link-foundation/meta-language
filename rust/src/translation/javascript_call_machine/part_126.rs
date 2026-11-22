// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayIterationTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptClosureAllocation;
use super::JavaScriptCollectionCompletion;
use super::JavaScriptCollectionIteration;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptStringIterationTransition;
use super::accept_java_script_call_construction;
use super::accept_java_script_call_function_bind;
use super::accept_java_script_call_object_enumeration;
use super::accept_java_script_call_object_from_entries;
use super::accept_java_script_call_primitive_wrapper;
use super::accept_java_script_call_property_definitions;
use super::accept_java_script_call_property_reflection;
use super::accept_java_script_collection_completion;
use super::accept_java_script_collection_iterator_result_completion;
use super::advance_java_script_call_frame;
use super::allocate_java_script_array;
use super::allocate_java_script_array_iterator;
use super::allocate_java_script_call_closure;
use super::allocate_java_script_collection;
use super::allocate_java_script_collection_iterator;
use super::allocate_java_script_string_iterator;
use super::allocate_java_script_weak_collection;
use super::find_java_script_heap_directory_object;
use super::find_java_script_heap_directory_property;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_executable_call_function;
use super::read_java_script_semantic_text_string;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::resume_java_script_collection_iterator_result_allocation;
use super::resume_java_script_construction_allocation;
use super::resume_java_script_function_bind_allocation;
use super::resume_java_script_object_enumeration_allocation;
use super::resume_java_script_object_from_entries_allocation;
use super::resume_java_script_primitive_wrapper;
use super::resume_java_script_property_definitions_allocation;
use super::resume_java_script_property_reflection_allocation;
use super::step_java_script_call_machine;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_pending_allocation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    identity: self::ml::Big,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            _,
            _,
            shared_constructor_field6,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field6 = *shared_constructor_field6;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    shared_constructor_field1,
                    shared_constructor_field2,
                ) => {
                    let shared_constructor_field1 = *shared_constructor_field1;
                    {
                        let pending = shared_constructor_field6.clone();
                        match pending.clone() {
                            self::JavaScriptCallPending::WaitCollectionIteratorResult(
                                destination,
                                realm,
                                state,
                            ) => {
                                let state = *state;
                                self::accept_java_script_collection_iterator_result_completion(
                                    program.clone(),
                                    machine.clone(),
                                    frame.clone(),
                                    destination,
                                    realm.clone(),
                                    self::resume_java_script_collection_iterator_result_allocation(
                                        shared_constructor_field1.clone(),
                                        state.clone(),
                                        identity.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitObjectEnumeration(
                                destination_2,
                                realm_2,
                                state_2,
                            ) => {
                                let state_2 = *state_2;
                                self::accept_java_script_call_object_enumeration(
                                    program.clone(),
                                    machine.clone(),
                                    destination_2,
                                    realm_2.clone(),
                                    self::resume_java_script_object_enumeration_allocation(
                                        shared_constructor_field1.clone(),
                                        state_2.clone(),
                                        identity.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitPropertyDefinitions(
                                destination_3,
                                realm_3,
                                state_3,
                            ) => {
                                let state_3 = *state_3;
                                self::accept_java_script_call_property_definitions(
                                    program.clone(),
                                    machine.clone(),
                                    destination_3,
                                    realm_3.clone(),
                                    self::resume_java_script_property_definitions_allocation(
                                        shared_constructor_field1.clone(),
                                        state_3.clone(),
                                        identity.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitPropertyReflection(
                                destination_4,
                                realm_4,
                                state_4,
                            ) => {
                                let state_4 = *state_4;
                                self::accept_java_script_call_property_reflection(
                                    machine.clone(),
                                    destination_4,
                                    realm_4.clone(),
                                    self::resume_java_script_property_reflection_allocation(
                                        shared_constructor_field1.clone(),
                                        state_4.clone(),
                                        identity.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitConstruction(
                                destination_5,
                                realm_5,
                                state_5,
                                _,
                            ) => {
                                let state_5 = *state_5;
                                {
                                    let directory = match program.clone() {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value25003,
                                            _,
                                        ) => ((source_ml_source2_data_property_value25003
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptArraySpeciesDirectory>,)
                                            }))
                                        .0)
                                            .clone(),
                                    };
                                    if match (directory.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(shared_constructor_field1.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 115, 116, 114, 117, 99, 116, 105,
                                                111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                                111, 110, 32, 114, 101, 113, 117, 105, 114, 101,
                                                115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111,
                                                110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97,
                                                117, 116, 104, 111, 114, 105, 116, 121,
                                            ]),
                                        )
                                    } else {
                                        {
                                            let request = self::step_java_script_call_machine(
                                                program.clone(),
                                                machine.clone(),
                                            );
                                            match request.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateObject(_, _) => {
                                                    self::accept_java_script_call_construction(program.clone(), machine.clone(), destination_5, realm_5.clone(), self::resume_java_script_construction_allocation(program.clone(), shared_constructor_field1.clone(), directory.clone().expect("a proven present value"), state_5.clone(), identity.clone()), None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>)
                                                }
                                                _ => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 101, 115, 116]))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::JavaScriptCallPending::WaitFunctionBind(
                                destination_6,
                                realm_6,
                                state_6,
                            ) => {
                                let state_6 = *state_6;
                                self::accept_java_script_call_function_bind(
                                    program.clone(),
                                    machine.clone(),
                                    destination_6,
                                    realm_6.clone(),
                                    self::resume_java_script_function_bind_allocation(
                                        shared_constructor_field1.clone(),
                                        state_6.clone(),
                                        identity.clone(),
                                    ),
                                )
                            }
                            self::JavaScriptCallPending::WaitObjectFromEntries(
                                destination_7,
                                realm_7,
                                state_7,
                                _,
                            ) => {
                                let state_7 = *state_7;
                                {
                                    let request_2 = self::step_java_script_call_machine(
                                        program.clone(),
                                        machine.clone(),
                                    );
                                    match request_2.clone() {
                                        self::JavaScriptCallMachineTransition::MachineAllocateObject(_, _) => {
                                            self::accept_java_script_call_object_from_entries(program.clone(), machine.clone(), destination_7, realm_7.clone(), self::resume_java_script_object_from_entries_allocation(shared_constructor_field1.clone(), state_7.clone(), identity.clone()))
                                        }
                                        _ => {
                                            if match request_2.clone() {
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(_, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            } {
                                                request_2.clone()
                                            } else {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 115, 112, 111, 110, 115, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 101, 115, 116]))
                                            }
                                        }
                                    }
                                }
                            }
                            self::JavaScriptCallPending::WaitPrimitiveWrapper(
                                destination_8,
                                state_8,
                            ) => {
                                let state_8 = *state_8;
                                self::accept_java_script_call_primitive_wrapper(
                                    machine.clone(),
                                    destination_8,
                                    self::resume_java_script_primitive_wrapper(
                                        shared_constructor_field1.clone(),
                                        state_8.clone(),
                                        identity.clone(),
                                    ),
                                )
                            }
                            _ => match pending.clone() {
                                self::JavaScriptCallPending::CallIdle => {
                                    let fn_ = self::read_java_script_executable_call_function(
                                        program.clone(),
                                        shared_constructor_field1.clone(),
                                        shared_constructor_field3.clone(),
                                    );
                                    if match (fn_.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(shared_constructor_field1.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                79, 98, 106, 101, 99, 116, 32, 114, 101, 115, 117,
                                                109, 112, 116, 105, 111, 110, 32, 114, 101, 113,
                                                117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97,
                                                99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99,
                                                101, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                                            ]),
                                        )
                                    } else {
                                        {
                                            let instruction = {
                                                let source_ml_source2_array_read_value25022 = match fn_.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value25020, _, _) => {
                                                                source_ml_source2_data_property_value25020.clone()
                                                            }
                                                        };
                                                {
                                                    let source_ml_source2_array_read_index25023 =
                                                        shared_constructor_field4;
                                                    {
                                                        let ml_reference_array1305 =
                                                            source_ml_source2_array_read_value25022
                                                                .clone();
                                                        let ml_reference_position1305 =
                                                            source_ml_source2_array_read_index25023;
                                                        let ml_reference_projection1305 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1305.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1305,
                                                                ),
                                                            );
                                                        ml_reference_projection1305
                                                    }
                                                }
                                            };
                                            if (match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((((match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    } && match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) && match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) && match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) && match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    })) {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                                    } else {
                                                        match instruction.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ClosureAllocate(destination_9, owner, ordinal, captures, receiver) => {
                                                                {
                                                                    let allocation = self::allocate_java_script_call_closure(program.clone(), shared_constructor_field1.clone(), identity.clone(), owner.clone(), ordinal, std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1306 = { let ml_reference_value1307 = captures.borrow().clone(); ml_reference_value1307 }; let ml_callback1306 = { let frame = frame.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { ({
                                                                        let source_ml_source2_array_read_value25045 = match frame.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value25044, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value25044.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_array_read_index25046 = index;
                                                                            { let ml_reference_array1308 = source_ml_source2_array_read_value25045.clone(); let ml_reference_position1308 = source_ml_source2_array_read_index25046; let ml_reference_projection1308 = self::ml_array::optional_at(&*ml_reference_array1308.borrow(), self::ml_array::number_index(ml_reference_position1308)); ml_reference_projection1308 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values1306.iter().cloned().enumerate().map(|(ml_index1306, ml_value1306)| ml_callback1306(ml_value1306)).collect::<Vec<_>>() })), ({
                                                                        let source_ml_source2_array_read_value25048 = shared_constructor_field5.clone();
                                                                        { let ml_reference_array1309 = source_ml_source2_array_read_value25048.clone(); let ml_reference_position1309 = receiver; let ml_reference_projection1309 = self::ml_array::optional_at(&*ml_reference_array1309.borrow(), self::ml_array::number_index(ml_reference_position1309)); ml_reference_projection1309 }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match frame.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25050) => {
                                                                            ((source_ml_source2_data_property_value25050.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    });
                                                                    match allocation.clone() {
                                                                        self::JavaScriptClosureAllocation::ClosureAllocationNormal(heap, value) => {
                                                                            let heap = *heap;
                                                                            let value = *value;
                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field2.clone(), self::write_java_script_call_register(self::advance_java_script_call_frame(frame.clone()), destination_9, value.clone()))))
                                                                        }
                                                                        self::JavaScriptClosureAllocation::ClosureAllocationInvalid(_, reason) => {
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason.clone())
                                                                        }
                                                                        self::JavaScriptClosureAllocation::ClosureAllocationResourceLimit(_, reason_2) => {
                                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_2.clone())
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                {
                                                                    let prototype = ({
                                                                        let source_ml_source2_array_read_value25056 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index25057 = self::read_java_script_call_allocation_prototype(instruction.clone().expect("a proven present value"));
                                                                            { let ml_reference_array1310 = source_ml_source2_array_read_value25056.clone(); let ml_reference_position1310 = source_ml_source2_array_read_index25057; let ml_reference_projection1310 = self::ml_array::optional_at(&*ml_reference_array1310.borrow(), self::ml_array::number_index(ml_reference_position1310)); ml_reference_projection1310 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    if (match prototype.clone() {
                                                                        self::JavaScriptSemanticValue::Null => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    } && match prototype.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 108, 108, 32, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                    } else {
                                                                        if (({ let ml_reference_array1311 = match shared_constructor_field1.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value25064, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value25064.clone()
                                                                            }
                                                                        }; let ml_reference_projection1311 = ml_reference_array1311.borrow().len() as f64; ml_reference_projection1311 }) >= (4096f64)) {
                                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                        } else {
                                                                            match instruction.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallInstruction::CollectionAllocate(destination_10, _, kind) => {
                                                                                    {
                                                                                        let allocation_2 = if self::ml_array::includes(&{ let ml_reference_value1312 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])]))).borrow().clone(); ml_reference_value1312 }, &kind.clone(), 0f64, false, |a, b| a == b) {
                                                                                            self::allocate_java_script_weak_collection(shared_constructor_field1.clone(), identity.clone(), prototype.clone(), &(kind))
                                                                                        } else {
                                                                                            self::allocate_java_script_collection(shared_constructor_field1.clone(), identity.clone(), prototype.clone(), &(kind))
                                                                                        };
                                                                                        match allocation_2.clone() {
                                                                                            self::JavaScriptCollectionCompletion::CollectionNormal(heap_2, value_2) => {
                                                                                                let heap_2 = *heap_2;
                                                                                                let value_2 = *value_2;
                                                                                                {
                                                                                                    let next = self::write_java_script_call_register(self::advance_java_script_call_frame(frame.clone()), destination_10, value_2.clone());
                                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field2.clone(), next.clone())))
                                                                                                }
                                                                                            }
                                                                                            _ => {
                                                                                                self::accept_java_script_collection_completion(program.clone(), machine.clone(), frame.clone(), destination_10, allocation_2.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::CollectionIteratorAllocate(destination_11, _, kind_2, iteration_kind, input) => {
                                                                                    if (kind_2 == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) {
                                                                                        {
                                                                                            let allocation_3 = self::allocate_java_script_string_iterator(shared_constructor_field1.clone(), identity.clone(), prototype.clone(), ({
                                                                                                let source_ml_source2_array_read_value25074 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_array_read_index25075 = (((input.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()).clone().unwrap_or_else(|| (-1f64));
                                                                                                    { let ml_reference_array1313 = source_ml_source2_array_read_value25074.clone(); let ml_reference_position1313 = source_ml_source2_array_read_index25075; let ml_reference_projection1313 = self::ml_array::optional_at(&*ml_reference_array1313.borrow(), self::ml_array::number_index(ml_reference_position1313)); ml_reference_projection1313 }
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                                                            match allocation_3.clone() {
                                                                                                self::JavaScriptStringIterationTransition::StringIteratorAllocated(heap_3, value_3) => {
                                                                                                    let heap_3 = *heap_3;
                                                                                                    let value_3 = *value_3;
                                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field2.clone(), self::write_java_script_call_register(self::advance_java_script_call_frame(frame.clone()), destination_11, value_3.clone()))))
                                                                                                }
                                                                                                self::JavaScriptStringIterationTransition::StringIteratorResourceLimit(_, reason_3) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_3.clone())
                                                                                                }
                                                                                                _ => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), match allocation_3.clone() {
                                                                                                        self::JavaScriptStringIterationTransition::StringIteratorInvalid(_, reason_4) => {
                                                                                                            reason_4.clone()
                                                                                                        }
                                                                                                        _ => {
                                                                                                            self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116])
                                                                                                        }
                                                                                                    })
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        if (kind_2 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) {
                                                                                            {
                                                                                                let allocation_4 = self::allocate_java_script_array_iterator(shared_constructor_field1.clone(), identity.clone(), prototype.clone(), ({
                                                                                                    let source_ml_source2_array_read_value25082 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_array_read_index25083 = 0f64;
                                                                                                        { let ml_reference_array1314 = source_ml_source2_array_read_value25082.clone(); let ml_reference_position1314 = source_ml_source2_array_read_index25083; let ml_reference_projection1314 = self::ml_array::optional_at(&*ml_reference_array1314.borrow(), self::ml_array::number_index(ml_reference_position1314)); ml_reference_projection1314 }
                                                                                                    }
                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), &(iteration_kind));
                                                                                                match allocation_4.clone() {
                                                                                                    self::JavaScriptArrayIterationTransition::ArrayIteratorAllocated(heap_4, value_4) => {
                                                                                                        let heap_4 = *heap_4;
                                                                                                        let value_4 = *value_4;
                                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_4.clone(), shared_constructor_field2.clone(), self::write_java_script_call_register(self::advance_java_script_call_frame(frame.clone()), destination_11, value_4.clone()))))
                                                                                                    }
                                                                                                    self::JavaScriptArrayIterationTransition::ArrayIteratorAllocationResourceLimit(_, reason_5) => {
                                                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_5.clone())
                                                                                                    }
                                                                                                    self::JavaScriptArrayIterationTransition::ArrayIteratorNeedBoxing(_, receiver_2) => {
                                                                                                        let receiver_2 = *receiver_2;
                                                                                                        self::JavaScriptCallMachineTransition::MachineNeedArrayIteratorBoxing(Box::new(machine.clone()), Box::new(receiver_2.clone()))
                                                                                                    }
                                                                                                    self::JavaScriptArrayIterationTransition::ArrayIteratorError(_, error_class, reason_6) => {
                                                                                                        self::request_java_script_call_error(machine.clone(), match fn_.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value25088, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value25088.clone()
                                                                                                            }
                                                                                                        }, &(error_class), &(reason_6))
                                                                                                    }
                                                                                                    _ => {
                                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), match allocation_4.clone() {
                                                                                                            self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(_, reason_7) => {
                                                                                                                reason_7.clone()
                                                                                                            }
                                                                                                            _ => {
                                                                                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116])
                                                                                                            }
                                                                                                        })
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let allocation_5 = self::allocate_java_script_collection_iterator(shared_constructor_field1.clone(), identity.clone(), prototype.clone(), ({
                                                                                                    let source_ml_source2_array_read_value25093 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_array_read_index25094 = 0f64;
                                                                                                        { let ml_reference_array1315 = source_ml_source2_array_read_value25093.clone(); let ml_reference_position1315 = source_ml_source2_array_read_index25094; let ml_reference_projection1315 = self::ml_array::optional_at(&*ml_reference_array1315.borrow(), self::ml_array::number_index(ml_reference_position1315)); ml_reference_projection1315 }
                                                                                                    }
                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), &(kind_2), &(iteration_kind));
                                                                                                match allocation_5.clone() {
                                                                                                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(_, reason_8) => {
                                                                                                        self::request_java_script_call_error(machine.clone(), match fn_.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value25097, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value25097.clone()
                                                                                                            }
                                                                                                        }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(reason_8))
                                                                                                    }
                                                                                                    self::JavaScriptCollectionIteration::CollectionIterationResourceLimit(_, reason_9) => {
                                                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_9.clone())
                                                                                                    }
                                                                                                    self::JavaScriptCollectionIteration::CollectionIteratorAllocated(heap_5, value_5) => {
                                                                                                        let heap_5 = *heap_5;
                                                                                                        let value_5 = *value_5;
                                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_5.clone(), shared_constructor_field2.clone(), self::write_java_script_call_register(self::advance_java_script_call_frame(frame.clone()), destination_11, value_5.clone()))))
                                                                                                    }
                                                                                                    _ => {
                                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), match allocation_5.clone() {
                                                                                                            self::JavaScriptCollectionIteration::CollectionIterationInvalid(_, reason_10) => {
                                                                                                                reason_10.clone()
                                                                                                            }
                                                                                                            _ => {
                                                                                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 99, 114, 101, 97, 116, 105, 111, 110])
                                                                                                            }
                                                                                                        })
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::OrdinaryAllocate(destination_12, _) => {
                                                                                    {
                                                                                        let allocation_6 = self::insert_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone(), prototype.clone());
                                                                                        if !((({ let ml_reference_value1316 = allocation_6.borrow().clone(); ml_reference_value1316 }).0).2) {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), (({ let ml_reference_value1317 = allocation_6.borrow().clone(); ml_reference_value1317 }).0).0)
                                                                                        } else {
                                                                                            {
                                                                                                let next_2 = self::write_java_script_call_register(self::advance_java_script_call_frame(frame.clone()), destination_12, self::JavaScriptSemanticValue::Object(identity.clone(), false));
                                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame((({ let ml_reference_value1318 = allocation_6.borrow().clone(); ml_reference_value1318 }).0).1, shared_constructor_field2.clone(), next_2.clone())))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::ArrayAllocate(destination_13, _, length) => {
                                                                                    {
                                                                                        let allocation_7 = self::allocate_java_script_array(shared_constructor_field1.clone(), identity.clone(), prototype.clone(), length);
                                                                                        match allocation_7.clone() {
                                                                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(_, reason_11, _) => {
                                                                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_11.clone())
                                                                                            }
                                                                                            _ => {
                                                                                                match allocation_7.clone() {
                                                                                                    self::JavaScriptArrayCompletion::ArrayNormal(heap_6, value_6) => {
                                                                                                        let heap_6 = *heap_6;
                                                                                                        let value_6 = *value_6;
                                                                                                        {
                                                                                                            let next_3 = self::write_java_script_call_register(self::advance_java_script_call_frame(frame.clone()), destination_13, value_6.clone());
                                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_6.clone(), shared_constructor_field2.clone(), next_3.clone())))
                                                                                                        }
                                                                                                    }
                                                                                                    _ => {
                                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), if match allocation_7.clone() {
                                                                                                            self::JavaScriptArrayCompletion::ArrayError(_, _, _) => {
                                                                                                                true
                                                                                                            }
                                                                                                            _ => {
                                                                                                                false
                                                                                                            }
                                                                                                        } {
                                                                                                            self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 109, 117, 115, 116, 32, 104, 97, 118, 101, 32, 98, 101, 101, 110, 32, 118, 97, 108, 105, 100, 97, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110])
                                                                                                        } else {
                                                                                                            match allocation_7.clone() {
                                                                                                                self::JavaScriptArrayCompletion::ArrayInvalid(_, reason_12) => {
                                                                                                                    reason_12.clone()
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 114, 109, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110])
                                                                                                                }
                                                                                                            }
                                                                                                        })
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                _ => {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
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
                                _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                    Box::new(shared_constructor_field1.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        79, 98, 106, 101, 99, 116, 32, 114, 101, 115, 117, 109,
                                        112, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114,
                                        101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                        32, 97, 99, 116, 105, 118, 101, 32, 97, 108, 108, 111, 99,
                                        97, 116, 105, 111, 110, 32, 105, 110, 115, 116, 114, 117,
                                        99, 116, 105, 111, 110,
                                    ]),
                                ),
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_allocation_prototype(
    instruction: self::JavaScriptCallInstruction,
) -> f64 {
    match instruction.clone() {
        self::JavaScriptCallInstruction::CollectionAllocate(_, prototype, _) => prototype,
        self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, prototype_2, _, _, _) => {
            prototype_2
        }
        self::JavaScriptCallInstruction::OrdinaryAllocate(_, prototype_3) => prototype_3,
        self::JavaScriptCallInstruction::ArrayAllocate(_, prototype_4, _) => prototype_4,
        _ => (-1f64),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_class_constructor_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_class_receiver_result(
    mode: &self::ml_string::Text,
    result: &self::ml_string::Text,
) -> bool {
    (self::test_java_script_ordinary_class_constructor_mode(mode)
        && ((*result) != (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_class_call_reason(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let object = {
                let source_ml_source2_array_read_value25126 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_array_read_index25127 =
                        self::find_java_script_heap_directory_object(
                            shared_constructor_field1.clone(),
                            identity.clone(),
                        );
                    {
                        let ml_reference_array1319 =
                            source_ml_source2_array_read_value25126.clone();
                        let ml_reference_position1319 = source_ml_source2_array_read_index25127;
                        let ml_reference_projection1319 = self::ml_array::optional_at(
                            &*ml_reference_array1319.borrow(),
                            self::ml_array::number_index(ml_reference_position1319),
                        );
                        ml_reference_projection1319
                    }
                }
            };
            if match (object.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::ml_string::Text::from_units(vec![])
            } else {
                match object.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        properties,
                    ) => {
                        let field = {
                            let source_ml_source2_array_read_index25137 =
                                self::find_java_script_heap_directory_property(
                                    properties.clone(),
                                    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                        std::cell::RefCell::new(vec![
                                            110f64, 97f64, 109f64, 101f64,
                                        ]),
                                    )),
                                );
                            {
                                let ml_reference_array1320 = properties.clone();
                                let ml_reference_position1320 =
                                    source_ml_source2_array_read_index25137;
                                let ml_reference_projection1320 = self::ml_array::optional_at(
                                    &*ml_reference_array1320.borrow(),
                                    self::ml_array::number_index(ml_reference_position1320),
                                );
                                ml_reference_projection1320
                            }
                        };
                        if (((match (field.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || !((({
                            let ml_reference_value1321 =
                                (match field.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        _,
                                        source_ml_source2_data_property_value25138,
                                    ) => source_ml_source2_data_property_value25138.clone(),
                                })
                                .borrow()
                                .clone();
                            ml_reference_value1321
                        })
                        .0)
                            .9))
                            || (({
                                let ml_reference_value1322 =
                                    (match field.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                            _,
                                            source_ml_source2_data_property_value25139,
                                        ) => source_ml_source2_data_property_value25139.clone(),
                                    })
                                    .borrow()
                                    .clone();
                                ml_reference_value1322
                            })
                            .0)
                                .5)
                            || (({
                                let ml_reference_value1323 =
                                    (match field.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                            _,
                                            source_ml_source2_data_property_value25140,
                                        ) => source_ml_source2_data_property_value25140.clone(),
                                    })
                                    .borrow()
                                    .clone();
                                ml_reference_value1323
                            })
                            .0)
                                .7)
                        {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            {
                                let value = (({
                                    let ml_reference_value1324 =
                                        (match field.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                _,
                                                source_ml_source2_data_property_value25141,
                                            ) => source_ml_source2_data_property_value25141.clone(),
                                        })
                                        .borrow()
                                        .clone();
                                    ml_reference_value1324
                                })
                                .0)
                                    .8;
                                if (match value.clone() {
                                    self::JavaScriptSemanticValue::Text(_) => false,
                                    _ => true,
                                } && match value.clone() {
                                    self::JavaScriptSemanticValue::SourceText(_, _) => false,
                                    _ => true,
                                }) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    ((self::ml_string::Text::from_units(vec![
                                        67, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114,
                                        117, 99, 116, 111, 114, 32,
                                    ]))
                                    .concat(
                                        &self::read_java_script_semantic_text_string(value.clone()),
                                    ))
                                    .concat(
                                        &self::ml_string::Text::from_units(vec![
                                            32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 105,
                                            110, 118, 111, 107, 101, 100, 32, 119, 105, 116, 104,
                                            111, 117, 116, 32, 39, 110, 101, 119, 39,
                                        ]),
                                    )
                                }
                            }
                        }
                    }
                    _ => self::ml_string::Text::from_units(vec![]),
                }
            }
        }
    }
}
