// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorClassification;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionTransition;
use super::JavaScriptObjectFromEntriesErrorRequest;
use super::JavaScriptObjectFromEntriesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::classify_java_script_array_species_constructor;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::push_java_script_construction_body;
use super::read_java_script_collection_constructor_kind;
use super::read_java_script_executable_call_function;
use super::read_java_script_known_array_constructor_fact;
use super::read_java_script_ordinary_class_call_reason;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::test_java_script_collection_constructor_mode;
use super::test_java_script_error_constructor_mode;
use super::test_java_script_ordinary_class_constructor_mode;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_object_from_entries(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptObjectFromEntriesTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field2) => {
            let frame = {
                let source_ml_source2_array_read_value38638 = shared_constructor_field2.clone();
                {
                    let source_ml_source2_array_read_index38639 = ({
                        let ml_reference_array2500 = shared_constructor_field2.clone();
                        let ml_reference_projection2500 =
                            ml_reference_array2500.borrow().len() as f64;
                        ml_reference_projection2500
                    } - 1f64);
                    {
                        let ml_reference_array2501 =
                            source_ml_source2_array_read_value38638.clone();
                        let ml_reference_position2501 = source_ml_source2_array_read_index38639;
                        let ml_reference_projection2501 = self::ml_array::optional_at(
                            &*ml_reference_array2501.borrow(),
                            self::ml_array::number_index(ml_reference_position2501),
                        );
                        ml_reference_projection2501
                    }
                }
            };
            {
                let current = self::JavaScriptCallMachine::CallMachine(Box::new(match transition.clone() {
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(source_ml_source2_data_property_value38640, _) => {
                            let source_ml_source2_data_property_value38640 = *source_ml_source2_data_property_value38640;
                            source_ml_source2_data_property_value38640.clone()
                        }
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCall(source_ml_source2_data_property_value38641, _, _, _, _) => {
                            let source_ml_source2_data_property_value38641 = *source_ml_source2_data_property_value38641;
                            source_ml_source2_data_property_value38641.clone()
                        }
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesAllocate(source_ml_source2_data_property_value38642, _, _) => {
                            let source_ml_source2_data_property_value38642 = *source_ml_source2_data_property_value38642;
                            source_ml_source2_data_property_value38642.clone()
                        }
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesPrimitiveGet(source_ml_source2_data_property_value38643, _, _, _) => {
                            let source_ml_source2_data_property_value38643 = *source_ml_source2_data_property_value38643;
                            source_ml_source2_data_property_value38643.clone()
                        }
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesNeedError(source_ml_source2_data_property_value38644, _, _, _) => {
                            let source_ml_source2_data_property_value38644 = *source_ml_source2_data_property_value38644;
                            source_ml_source2_data_property_value38644.clone()
                        }
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCompleted(source_ml_source2_data_property_value38645, _) => {
                            let source_ml_source2_data_property_value38645 = *source_ml_source2_data_property_value38645;
                            source_ml_source2_data_property_value38645.clone()
                        }
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesThrow(source_ml_source2_data_property_value38646, _) => {
                            let source_ml_source2_data_property_value38646 = *source_ml_source2_data_property_value38646;
                            source_ml_source2_data_property_value38646.clone()
                        }
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesError(source_ml_source2_data_property_value38647, _, _) => {
                            let source_ml_source2_data_property_value38647 = *source_ml_source2_data_property_value38647;
                            source_ml_source2_data_property_value38647.clone()
                        }
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(source_ml_source2_data_property_value38648, _) => {
                            let source_ml_source2_data_property_value38648 = *source_ml_source2_data_property_value38648;
                            source_ml_source2_data_property_value38648.clone()
                        }
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesResourceLimit(source_ml_source2_data_property_value38649, _, _) => {
                            let source_ml_source2_data_property_value38649 = *source_ml_source2_data_property_value38649;
                            source_ml_source2_data_property_value38649.clone()
                        }
                    }), shared_constructor_field2.clone());
                match current.clone() {
                    self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                        if match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(source_ml_source2_data_property_value38651, _) => {
                                        let source_ml_source2_data_property_value38651 = *source_ml_source2_data_property_value38651;
                                        source_ml_source2_data_property_value38651.clone()
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCall(source_ml_source2_data_property_value38652, _, _, _, _) => {
                                        let source_ml_source2_data_property_value38652 = *source_ml_source2_data_property_value38652;
                                        source_ml_source2_data_property_value38652.clone()
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesAllocate(source_ml_source2_data_property_value38653, _, _) => {
                                        let source_ml_source2_data_property_value38653 = *source_ml_source2_data_property_value38653;
                                        source_ml_source2_data_property_value38653.clone()
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesPrimitiveGet(source_ml_source2_data_property_value38654, _, _, _) => {
                                        let source_ml_source2_data_property_value38654 = *source_ml_source2_data_property_value38654;
                                        source_ml_source2_data_property_value38654.clone()
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesNeedError(source_ml_source2_data_property_value38655, _, _, _) => {
                                        let source_ml_source2_data_property_value38655 = *source_ml_source2_data_property_value38655;
                                        source_ml_source2_data_property_value38655.clone()
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCompleted(source_ml_source2_data_property_value38656, _) => {
                                        let source_ml_source2_data_property_value38656 = *source_ml_source2_data_property_value38656;
                                        source_ml_source2_data_property_value38656.clone()
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesThrow(source_ml_source2_data_property_value38657, _) => {
                                        let source_ml_source2_data_property_value38657 = *source_ml_source2_data_property_value38657;
                                        source_ml_source2_data_property_value38657.clone()
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesError(source_ml_source2_data_property_value38658, _, _) => {
                                        let source_ml_source2_data_property_value38658 = *source_ml_source2_data_property_value38658;
                                        source_ml_source2_data_property_value38658.clone()
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(source_ml_source2_data_property_value38659, _) => {
                                        let source_ml_source2_data_property_value38659 = *source_ml_source2_data_property_value38659;
                                        source_ml_source2_data_property_value38659.clone()
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesResourceLimit(source_ml_source2_data_property_value38660, _, _) => {
                                        let source_ml_source2_data_property_value38660 = *source_ml_source2_data_property_value38660;
                                        source_ml_source2_data_property_value38660.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            match transition.clone() {
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(heap, state) => {
                                        let heap = *heap;
                                        let state = *state;
                                        {
                                            let next = self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination, realm.clone(), Box::new(state.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(next.clone()))
                                        }
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCall(heap_2, state_2, source_ml_source2_f38661, source_ml_source2_f38662, source_ml_source2_f38663) => {
                                        let heap_2 = *heap_2;
                                        let state_2 = *state_2;
                                        let source_ml_source2_f38661 = *source_ml_source2_f38661;
                                        let source_ml_source2_f38662 = *source_ml_source2_f38662;
                                        {
                                            let next_2 = self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination, realm.clone(), Box::new(state_2.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::push_java_script_call(program.clone(), next_2.clone(), source_ml_source2_f38661.clone(), source_ml_source2_f38662.clone(), source_ml_source2_f38663.clone())
                                        }
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesAllocate(heap_3, state_3, source_ml_source2_f38664) => {
                                        let heap_3 = *heap_3;
                                        let state_3 = *state_3;
                                        let source_ml_source2_f38664 = *source_ml_source2_f38664;
                                        {
                                            let next_3 = self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination, realm.clone(), Box::new(state_3.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(next_3.clone()), Box::new(source_ml_source2_f38664.clone()))
                                        }
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesPrimitiveGet(heap_4, state_4, source_ml_source2_f38665, source_ml_source2_f38666) => {
                                        let heap_4 = *heap_4;
                                        let state_4 = *state_4;
                                        let source_ml_source2_f38665 = *source_ml_source2_f38665;
                                        let source_ml_source2_f38666 = *source_ml_source2_f38666;
                                        {
                                            let next_4 = self::replace_java_script_call_frame(heap_4.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination, realm.clone(), Box::new(state_4.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineNeedPrimitiveIteratorGet(Box::new(next_4.clone()), Box::new(source_ml_source2_f38665.clone()), Box::new(source_ml_source2_f38666.clone()))
                                        }
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesNeedError(heap_5, state_5, source_ml_source2_f38667, source_ml_source2_f38668) => {
                                        let heap_5 = *heap_5;
                                        let state_5 = *state_5;
                                        {
                                            let next_5 = self::replace_java_script_call_frame(heap_5.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination, realm.clone(), Box::new(state_5.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(next_5.clone()), realm.clone(), source_ml_source2_f38667.clone(), source_ml_source2_f38668.clone())
                                        }
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCompleted(heap_6, value) => {
                                        let heap_6 = *heap_6;
                                        let value = *value;
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_6.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesThrow(heap_7, value_2) => {
                                        let heap_7 = *heap_7;
                                        let value_2 = *value_2;
                                        self::throw_java_script_call_value(self::replace_java_script_call_frame(heap_7.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::CallIdle)), value_2.clone())
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesError(heap_8, error_class, reason) => {
                                        let heap_8 = *heap_8;
                                        self::request_java_script_call_error(self::replace_java_script_call_frame(heap_8.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::CallIdle)), realm.clone(), &(error_class), &(reason))
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(heap_9, reason_2) => {
                                        let heap_9 = *heap_9;
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_9.clone()), reason_2.clone())
                                    }
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesResourceLimit(heap_10, state_6, source_ml_source2_f38669) => {
                                        let heap_10 = *heap_10;
                                        let state_6 = *state_6;
                                        {
                                            let next_6 = self::replace_java_script_call_frame(heap_10.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination, realm.clone(), Box::new(state_6.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(next_6.clone()), source_ml_source2_f38669.clone())
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
pub fn read_java_script_call_constructor_failure(
    heap: self::JavaScriptOrdinaryHeap,
    fn_: self::JavaScriptCallFunction,
) -> Option<self::JavaScriptObjectFromEntriesErrorRequest> {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let reason = self::ml_string::Text::from_units(vec![]);
            {
                let reason_4 = if self::test_java_script_error_constructor_mode(
                    &(shared_constructor_field3),
                ) {
                    self::ml_string::Text::from_units(vec![
                        97, 32, 100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114, 32,
                        99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 113,
                        117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                        32, 110, 101, 119, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
                    ])
                } else {
                    if self::test_java_script_ordinary_class_constructor_mode(
                        &(shared_constructor_field3),
                    ) {
                        {
                            let reason_2 = self::read_java_script_ordinary_class_call_reason(
                                heap.clone(),
                                shared_constructor_field1.clone(),
                            );
                            reason_2.clone()
                        }
                    } else {
                        if self::test_java_script_collection_constructor_mode(
                            &(shared_constructor_field3),
                        ) {
                            {
                                let reason_3 = ((self::ml_string::Text::from_units(vec![
                                    67, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32,
                                ]))
                                .concat(
                                    &self::read_java_script_collection_constructor_kind(
                                        fn_.clone(),
                                    ),
                                ))
                                .concat(
                                    &self::ml_string::Text::from_units(vec![
                                        32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 39, 110,
                                        101, 119, 39,
                                    ]),
                                );
                                reason_3.clone()
                            }
                        } else {
                            reason.clone()
                        }
                    }
                };
                if (reason_4 == (self::ml_string::Text::from_units(vec![]))) {
                    None::<self::JavaScriptObjectFromEntriesErrorRequest>
                } else {
                    Some(self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(shared_constructor_field2.clone(), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), reason_4.clone()))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_constructor_check(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    input: self::JavaScriptSemanticValue,
    required: bool,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value38756 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index38757 = ({
                            let ml_reference_array2502 = shared_constructor_field2.clone();
                            let ml_reference_projection2502 =
                                ml_reference_array2502.borrow().len() as f64;
                            ml_reference_projection2502
                        } - 1f64);
                        {
                            let ml_reference_array2503 =
                                source_ml_source2_array_read_value38756.clone();
                            let ml_reference_position2503 = source_ml_source2_array_read_index38757;
                            let ml_reference_projection2503 = self::ml_array::optional_at(
                                &*ml_reference_array2503.borrow(),
                                self::ml_array::number_index(ml_reference_position2503),
                            );
                            ml_reference_projection2503
                        }
                    }
                };
                {
                    let directory = match program.clone() {
                        self::JavaScriptCallProgram::CallProgram(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value38758,
                            _,
                        ) => ((source_ml_source2_data_property_value38758
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,)))
                        .0)
                            .clone(),
                    };
                    if (match (frame.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || match (directory.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptCallMachineTransition::MachineInvalid(
                            Box::new(shared_constructor_field1.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 104,
                                101, 99, 107, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97,
                                110, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101,
                                32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 97, 117, 116,
                                104, 111, 114, 105, 116, 121,
                            ]),
                        )
                    } else {
                        {
                            let checked = self::classify_java_script_array_species_constructor(
                                shared_constructor_field1.clone(),
                                directory.clone().expect("a proven present value"),
                                input.clone(),
                            );
                            match checked.clone() {
                                self::JavaScriptArrayConstructorClassification::ArrayConstructorUnknown(constructor) => {
                                    self::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitConstructorCheck(destination, Box::new(input.clone()), required)))), Box::new((((constructor.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()).clone().unwrap_or_else(|| input.clone())))
                                }
                                _ => {
                                    {
                                        let constructable = {
                                            let source_ml_source2_comparison_operand38765 = {
                                                let source_ml_source2_optional_chain_receiver38763 = (self::read_java_script_known_array_constructor_fact(checked.clone()),);
                                                if match (source_ml_source2_optional_chain_receiver38763.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    Some({
                                                        let source_ml_source1_temporary_optional_chain1534 = ((source_ml_source2_optional_chain_receiver38763.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source1_temporary_optional_chain1534.clone() {
                                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value38764, _, _, _) => {
                                                                source_ml_source2_data_property_value38764
                                                            }
                                                        }
                                                    })
                                                } else {
                                                    None::<bool>
                                                }
                                            };
                                            match (source_ml_source2_comparison_operand38765.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        };
                                        if (!constructable && required) {
                                            {
                                                let owner = self::read_java_script_executable_call_function(program.clone(), shared_constructor_field1.clone(), match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value38768, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value38768.clone()
                                                    }
                                                });
                                                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 104, 101, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                                                } else {
                                                    self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value38770, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value38770.clone()
                                                        }
                                                    }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 118, 97, 108, 117, 101, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])))
                                                }
                                            }
                                        } else {
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, if required {
                                                input.clone()
                                            } else {
                                                self::JavaScriptSemanticValue::Boolean(constructable)
                                            }))))
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
pub fn accept_java_script_call_construction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptConstructionTransition,
    held_arguments: Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field2) => {
            let frame = {
                let source_ml_source2_array_read_value38775 = shared_constructor_field2.clone();
                {
                    let source_ml_source2_array_read_index38776 = ({
                        let ml_reference_array2504 = shared_constructor_field2.clone();
                        let ml_reference_projection2504 =
                            ml_reference_array2504.borrow().len() as f64;
                        ml_reference_projection2504
                    } - 1f64);
                    {
                        let ml_reference_array2505 =
                            source_ml_source2_array_read_value38775.clone();
                        let ml_reference_position2505 = source_ml_source2_array_read_index38776;
                        let ml_reference_projection2505 = self::ml_array::optional_at(
                            &*ml_reference_array2505.borrow(),
                            self::ml_array::number_index(ml_reference_position2505),
                        );
                        ml_reference_projection2505
                    }
                }
            };
            {
                let current = self::JavaScriptCallMachine::CallMachine(
                    Box::new(match transition.clone() {
                        self::JavaScriptConstructionTransition::ConstructionContinue(
                            source_ml_source2_data_property_value38777,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38777 =
                                *source_ml_source2_data_property_value38777;
                            source_ml_source2_data_property_value38777.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionCall(
                            source_ml_source2_data_property_value38778,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38778 =
                                *source_ml_source2_data_property_value38778;
                            source_ml_source2_data_property_value38778.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionAllocate(
                            source_ml_source2_data_property_value38779,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38779 =
                                *source_ml_source2_data_property_value38779;
                            source_ml_source2_data_property_value38779.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionNeedAuthority(
                            source_ml_source2_data_property_value38780,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38780 =
                                *source_ml_source2_data_property_value38780;
                            source_ml_source2_data_property_value38780.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionNeedImplementation(
                            source_ml_source2_data_property_value38781,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38781 =
                                *source_ml_source2_data_property_value38781;
                            source_ml_source2_data_property_value38781.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionCompleted(
                            source_ml_source2_data_property_value38782,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38782 =
                                *source_ml_source2_data_property_value38782;
                            source_ml_source2_data_property_value38782.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionThrow(
                            source_ml_source2_data_property_value38783,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38783 =
                                *source_ml_source2_data_property_value38783;
                            source_ml_source2_data_property_value38783.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionError(
                            source_ml_source2_data_property_value38784,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38784 =
                                *source_ml_source2_data_property_value38784;
                            source_ml_source2_data_property_value38784.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionInvalid(
                            source_ml_source2_data_property_value38785,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38785 =
                                *source_ml_source2_data_property_value38785;
                            source_ml_source2_data_property_value38785.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionResourceLimit(
                            source_ml_source2_data_property_value38786,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38786 =
                                *source_ml_source2_data_property_value38786;
                            source_ml_source2_data_property_value38786.clone()
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
                                    self::JavaScriptConstructionTransition::ConstructionContinue(source_ml_source2_data_property_value38788, _) => {
                                        let source_ml_source2_data_property_value38788 = *source_ml_source2_data_property_value38788;
                                        source_ml_source2_data_property_value38788.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionCall(source_ml_source2_data_property_value38789, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value38789 = *source_ml_source2_data_property_value38789;
                                        source_ml_source2_data_property_value38789.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionAllocate(source_ml_source2_data_property_value38790, _, _) => {
                                        let source_ml_source2_data_property_value38790 = *source_ml_source2_data_property_value38790;
                                        source_ml_source2_data_property_value38790.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionNeedAuthority(source_ml_source2_data_property_value38791, _, _) => {
                                        let source_ml_source2_data_property_value38791 = *source_ml_source2_data_property_value38791;
                                        source_ml_source2_data_property_value38791.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionNeedImplementation(source_ml_source2_data_property_value38792, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value38792 = *source_ml_source2_data_property_value38792;
                                        source_ml_source2_data_property_value38792.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionCompleted(source_ml_source2_data_property_value38793, _) => {
                                        let source_ml_source2_data_property_value38793 = *source_ml_source2_data_property_value38793;
                                        source_ml_source2_data_property_value38793.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionThrow(source_ml_source2_data_property_value38794, _) => {
                                        let source_ml_source2_data_property_value38794 = *source_ml_source2_data_property_value38794;
                                        source_ml_source2_data_property_value38794.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionError(source_ml_source2_data_property_value38795, _, _, _) => {
                                        let source_ml_source2_data_property_value38795 = *source_ml_source2_data_property_value38795;
                                        source_ml_source2_data_property_value38795.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionInvalid(source_ml_source2_data_property_value38796, _) => {
                                        let source_ml_source2_data_property_value38796 = *source_ml_source2_data_property_value38796;
                                        source_ml_source2_data_property_value38796.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionResourceLimit(source_ml_source2_data_property_value38797, _, _) => {
                                        let source_ml_source2_data_property_value38797 = *source_ml_source2_data_property_value38797;
                                        source_ml_source2_data_property_value38797.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            {
                                let pending = match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value38798,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value38798 =
                                            *source_ml_source2_data_property_value38798;
                                        source_ml_source2_data_property_value38798.clone()
                                    }
                                };
                                {
                                    let held = held_arguments.clone();
                                    {
                                        let held_3 = if (match (held.clone(), None::<()>) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        } && match pending.clone() {
                                            self::JavaScriptCallPending::WaitConstruction(
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => true,
                                            _ => false,
                                        }) {
                                            {
                                                let held_2 = (((match pending.clone() {
                                                        self::JavaScriptCallPending::WaitConstruction(_, _, _, source_ml_source2_data_property_value38803) => {
                                                            Some((((source_ml_source2_data_property_value38803.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,))).0).clone(),))
                                                        }
                                                        _ => {
                                                            None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone();
                                                held_2.clone()
                                            }
                                        } else {
                                            held.clone()
                                        };
                                        match transition.clone() {
                                                self::JavaScriptConstructionTransition::ConstructionContinue(heap, state) => {
                                                    let heap = *heap;
                                                    let state = *state;
                                                    {
                                                        let suspension = if match (held_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>)
                                                        } else {
                                                            {
                                                                let source_ml_source2_constructor_field38823 = Some(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value2506 = (held_3.clone().expect("a proven present value")).borrow().clone(); ml_reference_value2506 }, 0f64, f64::INFINITY))));
                                                                self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state.clone()), Some((source_ml_source2_constructor_field38823.clone(),)))
                                                            }
                                                        };
                                                        {
                                                            let next = self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), suspension.clone()));
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(next.clone()))
                                                        }
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionCall(heap_2, state_2, source_ml_source2_f38804, source_ml_source2_f38805, source_ml_source2_f38806, source_ml_source2_f38807, source_ml_source2_f38808) => {
                                                    let heap_2 = *heap_2;
                                                    let state_2 = *state_2;
                                                    let source_ml_source2_f38804 = *source_ml_source2_f38804;
                                                    let source_ml_source2_f38805 = *source_ml_source2_f38805;
                                                    let source_ml_source2_f38808 = *source_ml_source2_f38808;
                                                    {
                                                        let suspension_2 = if match (held_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_2.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>)
                                                        } else {
                                                            {
                                                                let source_ml_source2_constructor_field38841 = Some(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value2507 = (held_3.clone().expect("a proven present value")).borrow().clone(); ml_reference_value2507 }, 0f64, f64::INFINITY))));
                                                                self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_2.clone()), Some((source_ml_source2_constructor_field38841.clone(),)))
                                                            }
                                                        };
                                                        {
                                                            let next_2 = self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), suspension_2.clone()));
                                                            if source_ml_source2_f38807 {
                                                                self::push_java_script_construction_body(program.clone(), next_2.clone(), source_ml_source2_f38804.clone(), source_ml_source2_f38805.clone(), source_ml_source2_f38806.clone(), source_ml_source2_f38808.clone())
                                                            } else {
                                                                self::push_java_script_call(program.clone(), next_2.clone(), source_ml_source2_f38804.clone(), source_ml_source2_f38805.clone(), source_ml_source2_f38806.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionAllocate(heap_3, state_3, source_ml_source2_f38809) => {
                                                    let heap_3 = *heap_3;
                                                    let state_3 = *state_3;
                                                    let source_ml_source2_f38809 = *source_ml_source2_f38809;
                                                    {
                                                        let suspension_3 = if match (held_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_3.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>)
                                                        } else {
                                                            {
                                                                let source_ml_source2_constructor_field38857 = Some(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value2508 = (held_3.clone().expect("a proven present value")).borrow().clone(); ml_reference_value2508 }, 0f64, f64::INFINITY))));
                                                                self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_3.clone()), Some((source_ml_source2_constructor_field38857.clone(),)))
                                                            }
                                                        };
                                                        {
                                                            let next_3 = self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), suspension_3.clone()));
                                                            self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(next_3.clone()), Box::new(source_ml_source2_f38809.clone()))
                                                        }
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionNeedAuthority(heap_4, state_4, source_ml_source2_f38810) => {
                                                    let heap_4 = *heap_4;
                                                    let state_4 = *state_4;
                                                    let source_ml_source2_f38810 = *source_ml_source2_f38810;
                                                    {
                                                        let suspension_4 = if match (held_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_4.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>)
                                                        } else {
                                                            {
                                                                let source_ml_source2_constructor_field38873 = Some(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value2509 = (held_3.clone().expect("a proven present value")).borrow().clone(); ml_reference_value2509 }, 0f64, f64::INFINITY))));
                                                                self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_4.clone()), Some((source_ml_source2_constructor_field38873.clone(),)))
                                                            }
                                                        };
                                                        {
                                                            let next_4 = self::replace_java_script_call_frame(heap_4.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), suspension_4.clone()));
                                                            self::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(Box::new(next_4.clone()), Box::new(source_ml_source2_f38810.clone()))
                                                        }
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionNeedImplementation(heap_5, state_5, source_ml_source2_f38811, source_ml_source2_f38812, source_ml_source2_f38813, source_ml_source2_f38814, source_ml_source2_f38815) => {
                                                    let heap_5 = *heap_5;
                                                    let state_5 = *state_5;
                                                    let source_ml_source2_f38811 = *source_ml_source2_f38811;
                                                    let source_ml_source2_f38812 = *source_ml_source2_f38812;
                                                    {
                                                        let suspension_5 = if match (held_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_5.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>)
                                                        } else {
                                                            {
                                                                let source_ml_source2_constructor_field38889 = Some(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value2510 = (held_3.clone().expect("a proven present value")).borrow().clone(); ml_reference_value2510 }, 0f64, f64::INFINITY))));
                                                                self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_5.clone()), Some((source_ml_source2_constructor_field38889.clone(),)))
                                                            }
                                                        };
                                                        {
                                                            let next_5 = self::replace_java_script_call_frame(heap_5.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), suspension_5.clone()));
                                                            self::JavaScriptCallMachineTransition::MachineConstruct(Box::new(next_5.clone()), Box::new(source_ml_source2_f38811.clone()), Box::new(source_ml_source2_f38812.clone()), source_ml_source2_f38813.clone(), source_ml_source2_f38814.clone(), source_ml_source2_f38815.clone())
                                                        }
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionCompleted(heap_6, value) => {
                                                    let heap_6 = *heap_6;
                                                    let value = *value;
                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_6.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionThrow(heap_7, value_2) => {
                                                    let heap_7 = *heap_7;
                                                    let value_2 = *value_2;
                                                    self::throw_java_script_call_value(self::replace_java_script_call_frame(heap_7.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::CallIdle)), value_2.clone())
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionError(heap_8, error_class, reason, transition_realm) => {
                                                    let heap_8 = *heap_8;
                                                    self::request_java_script_call_error(self::replace_java_script_call_frame(heap_8.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::CallIdle)), (((transition_realm.clone().unwrap_or_else(|| (None::<self::ml::Big>,))).0).clone()).clone().unwrap_or_else(|| realm.clone()), &(error_class), &(reason))
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionInvalid(heap_9, reason_2) => {
                                                    let heap_9 = *heap_9;
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_9.clone()), reason_2.clone())
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionResourceLimit(heap_10, state_6, source_ml_source2_f38816) => {
                                                    let heap_10 = *heap_10;
                                                    let state_6 = *state_6;
                                                    {
                                                        let suspension_6 = if match (held_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_6.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>)
                                                        } else {
                                                            {
                                                                let source_ml_source2_constructor_field38908 = Some(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value2511 = (held_3.clone().expect("a proven present value")).borrow().clone(); ml_reference_value2511 }, 0f64, f64::INFINITY))));
                                                                self::JavaScriptCallPending::WaitConstruction(destination, realm.clone(), Box::new(state_6.clone()), Some((source_ml_source2_constructor_field38908.clone(),)))
                                                            }
                                                        };
                                                        {
                                                            let next_6 = self::replace_java_script_call_frame(heap_10.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), suspension_6.clone()));
                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(next_6.clone()), source_ml_source2_f38816.clone())
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
