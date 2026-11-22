// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorClassification;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArrayCreationFallback;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptObjectFromEntriesErrorRequest;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::begin_java_script_construction;
use super::classify_java_script_array_species_constructor;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::push_java_script_construction_body;
use super::read_java_script_known_array_constructor_fact;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::resume_java_script_array_creation_construction;
use super::source_ml_source1_temporary_request_java_script_call_array_fallback_loop1437;
use super::test_java_script_array_length;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_call_array_fallback(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    state: self::JavaScriptArrayCreationState,
    fallback: self::JavaScriptArrayCreationFallback,
) -> self::JavaScriptCallMachineTransition {
    match fallback.clone() {
        self::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(
            shared_constructor_field3,
            shared_constructor_field4,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    shared_constructor_field1,
                    shared_constructor_field2,
                ) => {
                    let shared_constructor_field1 = *shared_constructor_field1;
                    {
                        let frame = {
                            let source_ml_source2_array_read_value33718 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index33719 = ({
                                    let ml_reference_array2190 = shared_constructor_field2.clone();
                                    let ml_reference_projection2190 =
                                        ml_reference_array2190.borrow().len() as f64;
                                    ml_reference_projection2190
                                } - 1f64);
                                {
                                    let ml_reference_array2191 =
                                        source_ml_source2_array_read_value33718.clone();
                                    let ml_reference_position2191 =
                                        source_ml_source2_array_read_index33719;
                                    let ml_reference_projection2191 = self::ml_array::optional_at(
                                        &*ml_reference_array2191.borrow(),
                                        self::ml_array::number_index(ml_reference_position2191),
                                    );
                                    ml_reference_projection2191
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
                                    65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 102, 97,
                                    108, 108, 98, 97, 99, 107, 32, 114, 101, 116, 97, 105, 110,
                                    115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                                    111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101,
                                ]),
                            )
                        } else {
                            if !(self::test_java_script_array_length(shared_constructor_field4)) {
                                self::request_java_script_call_error(
                                    self::replace_java_script_call_frame(
                                        shared_constructor_field1.clone(),
                                        shared_constructor_field2.clone(),
                                        self::replace_java_script_call_pending(
                                            frame.clone().expect("a proven present value"),
                                            self::JavaScriptCallPending::CallIdle,
                                        ),
                                    ),
                                    realm.clone(),
                                    &(self::ml_string::Text::from_units(vec![
                                        82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                                    ])),
                                    &(self::ml_string::Text::from_units(vec![
                                        73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121,
                                        32, 108, 101, 110, 103, 116, 104,
                                    ])),
                                )
                            } else {
                                {
                                    let next = self::replace_java_script_call_frame(
                                        shared_constructor_field1.clone(),
                                        shared_constructor_field2.clone(),
                                        self::replace_java_script_call_pending(
                                            frame.clone().expect("a proven present value"),
                                            {
                                                let source_ml_source2_constructor_field33729 =
                                                    Some(fallback.clone());
                                                self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(state.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, Some((source_ml_source2_constructor_field33729.clone(),)), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                            },
                                        ),
                                    );
                                    {
                                        let properties = 1f64;
                                        {
                                            let source_ml_source1_temporary_values1436 =
                                                match shared_constructor_field1.clone() {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                        source_ml_source2_data_property_value33731,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source2_data_property_value33731
                                                        .clone(),
                                                };
                                            {
                                                let source_ml_source1_temporary_index1436 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let properties_2 = self::source_ml_source1_temporary_request_java_script_call_array_fallback_loop1437(properties, source_ml_source1_temporary_values1436.clone(), source_ml_source1_temporary_index1436.clone());
                                                    if ((({
                                                        let ml_reference_array2192 = match shared_constructor_field1.clone() {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value33735, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value33735.clone()
                                                        }
                                                    };
                                                        let ml_reference_projection2192 =
                                                            ml_reference_array2192.borrow().len()
                                                                as f64;
                                                        ml_reference_projection2192
                                                    }) >= (4096f64))
                                                        || (properties_2 > (16384f64)))
                                                    {
                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(next.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 115, 104, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 111, 117, 110, 100, 115]))
                                                    } else {
                                                        self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(next.clone()), Box::new(shared_constructor_field3.clone()))
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
pub fn accept_java_script_call_array_creation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptArrayCreationTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field2) => {
            let frame = {
                let source_ml_source2_array_read_value33739 = shared_constructor_field2.clone();
                {
                    let source_ml_source2_array_read_index33740 = ({
                        let ml_reference_array2193 = shared_constructor_field2.clone();
                        let ml_reference_projection2193 =
                            ml_reference_array2193.borrow().len() as f64;
                        ml_reference_projection2193
                    } - 1f64);
                    {
                        let ml_reference_array2194 =
                            source_ml_source2_array_read_value33739.clone();
                        let ml_reference_position2194 = source_ml_source2_array_read_index33740;
                        let ml_reference_projection2194 = self::ml_array::optional_at(
                            &*ml_reference_array2194.borrow(),
                            self::ml_array::number_index(ml_reference_position2194),
                        );
                        ml_reference_projection2194
                    }
                }
            };
            {
                let current = self::JavaScriptCallMachine::CallMachine(
                    Box::new(match transition.clone() {
                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(
                            source_ml_source2_data_property_value33741,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33741 =
                                *source_ml_source2_data_property_value33741;
                            source_ml_source2_data_property_value33741.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(
                            source_ml_source2_data_property_value33742,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33742 =
                                *source_ml_source2_data_property_value33742;
                            source_ml_source2_data_property_value33742.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationCall(
                            source_ml_source2_data_property_value33743,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33743 =
                                *source_ml_source2_data_property_value33743;
                            source_ml_source2_data_property_value33743.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationAllocate(
                            source_ml_source2_data_property_value33744,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33744 =
                                *source_ml_source2_data_property_value33744;
                            source_ml_source2_data_property_value33744.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationPrimitiveGet(
                            source_ml_source2_data_property_value33745,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33745 =
                                *source_ml_source2_data_property_value33745;
                            source_ml_source2_data_property_value33745.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationNeedError(
                            source_ml_source2_data_property_value33746,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33746 =
                                *source_ml_source2_data_property_value33746;
                            source_ml_source2_data_property_value33746.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationCompleted(
                            source_ml_source2_data_property_value33747,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33747 =
                                *source_ml_source2_data_property_value33747;
                            source_ml_source2_data_property_value33747.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationThrow(
                            source_ml_source2_data_property_value33748,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33748 =
                                *source_ml_source2_data_property_value33748;
                            source_ml_source2_data_property_value33748.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationError(
                            source_ml_source2_data_property_value33749,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33749 =
                                *source_ml_source2_data_property_value33749;
                            source_ml_source2_data_property_value33749.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                            source_ml_source2_data_property_value33750,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33750 =
                                *source_ml_source2_data_property_value33750;
                            source_ml_source2_data_property_value33750.clone()
                        }
                        self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(
                            source_ml_source2_data_property_value33751,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33751 =
                                *source_ml_source2_data_property_value33751;
                            source_ml_source2_data_property_value33751.clone()
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
                                    self::JavaScriptArrayCreationTransition::ArrayCreationContinue(source_ml_source2_data_property_value33753, _) => {
                                        let source_ml_source2_data_property_value33753 = *source_ml_source2_data_property_value33753;
                                        source_ml_source2_data_property_value33753.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(source_ml_source2_data_property_value33754, _, _, _, _) => {
                                        let source_ml_source2_data_property_value33754 = *source_ml_source2_data_property_value33754;
                                        source_ml_source2_data_property_value33754.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationCall(source_ml_source2_data_property_value33755, _, _, _, _) => {
                                        let source_ml_source2_data_property_value33755 = *source_ml_source2_data_property_value33755;
                                        source_ml_source2_data_property_value33755.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationAllocate(source_ml_source2_data_property_value33756, _, _) => {
                                        let source_ml_source2_data_property_value33756 = *source_ml_source2_data_property_value33756;
                                        source_ml_source2_data_property_value33756.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationPrimitiveGet(source_ml_source2_data_property_value33757, _, _, _) => {
                                        let source_ml_source2_data_property_value33757 = *source_ml_source2_data_property_value33757;
                                        source_ml_source2_data_property_value33757.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationNeedError(source_ml_source2_data_property_value33758, _, _, _) => {
                                        let source_ml_source2_data_property_value33758 = *source_ml_source2_data_property_value33758;
                                        source_ml_source2_data_property_value33758.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationCompleted(source_ml_source2_data_property_value33759, _) => {
                                        let source_ml_source2_data_property_value33759 = *source_ml_source2_data_property_value33759;
                                        source_ml_source2_data_property_value33759.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationThrow(source_ml_source2_data_property_value33760, _) => {
                                        let source_ml_source2_data_property_value33760 = *source_ml_source2_data_property_value33760;
                                        source_ml_source2_data_property_value33760.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationError(source_ml_source2_data_property_value33761, _, _) => {
                                        let source_ml_source2_data_property_value33761 = *source_ml_source2_data_property_value33761;
                                        source_ml_source2_data_property_value33761.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(source_ml_source2_data_property_value33762, _) => {
                                        let source_ml_source2_data_property_value33762 = *source_ml_source2_data_property_value33762;
                                        source_ml_source2_data_property_value33762.clone()
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(source_ml_source2_data_property_value33763, _, _) => {
                                        let source_ml_source2_data_property_value33763 = *source_ml_source2_data_property_value33763;
                                        source_ml_source2_data_property_value33763.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            match transition.clone() {
                                    self::JavaScriptArrayCreationTransition::ArrayCreationContinue(transition_heap, transition_state) => {
                                        let transition_heap = *transition_heap;
                                        let transition_state = *transition_state;
                                        {
                                            let next = self::replace_java_script_call_frame(transition_heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(transition_state.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(next.clone()))
                                        }
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(transition_heap_2, transition_state_2, source_ml_source2_f33764, source_ml_source2_f33765, source_ml_source2_f33766) => {
                                        let transition_heap_2 = *transition_heap_2;
                                        let transition_state_2 = *transition_state_2;
                                        let source_ml_source2_f33764 = *source_ml_source2_f33764;
                                        let source_ml_source2_f33766 = *source_ml_source2_f33766;
                                        {
                                            let next_2 = self::replace_java_script_call_frame(transition_heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(transition_state_2.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            {
                                                let directory = match program.clone() {
                                                    self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value33802, _) => {
                                                        ((source_ml_source2_data_property_value33802.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                    }
                                                };
                                                if match (directory.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(transition_heap_2.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                } else {
                                                    {
                                                        let selected = self::classify_java_script_array_species_constructor(transition_heap_2.clone(), directory.clone().expect("a proven present value"), source_ml_source2_f33764.clone());
                                                        match selected.clone() {
                                                            self::JavaScriptArrayConstructorClassification::ArrayConstructorUnknown(constructor) => {
                                                                self::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(Box::new(next_2.clone()), Box::new((((constructor.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()).clone().unwrap_or_else(|| source_ml_source2_f33764.clone())))
                                                            }
                                                            _ => {
                                                                if {
                                                                    let source_ml_source2_comparison_operand33805 = {
                                                                        let source_ml_source2_optional_chain_receiver33803 = (self::read_java_script_known_array_constructor_fact(selected.clone()),);
                                                                        if match (source_ml_source2_optional_chain_receiver33803.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            Some({
                                                                                let source_ml_source1_temporary_optional_chain1438 = ((source_ml_source2_optional_chain_receiver33803.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_temporary_optional_chain1438.clone() {
                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value33804, _, _, _) => {
                                                                                        source_ml_source2_data_property_value33804
                                                                                    }
                                                                                }
                                                                            })
                                                                        } else {
                                                                            None::<bool>
                                                                        }
                                                                    };
                                                                    match (source_ml_source2_comparison_operand33805.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                } {
                                                                    self::accept_java_script_call_array_construction(program.clone(), next_2.clone(), self::begin_java_script_construction(program.clone(), transition_heap_2.clone(), directory.clone().expect("a proven present value"), source_ml_source2_f33764.clone(), source_ml_source2_f33764.clone(), source_ml_source2_f33765.clone()))
                                                                } else {
                                                                    self::request_java_script_call_array_fallback(program.clone(), next_2.clone(), destination, realm.clone(), transition_state_2.clone(), source_ml_source2_f33766.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationCall(transition_heap_3, transition_state_3, source_ml_source2_f33767, source_ml_source2_f33768, source_ml_source2_f33769) => {
                                        let transition_heap_3 = *transition_heap_3;
                                        let transition_state_3 = *transition_state_3;
                                        let source_ml_source2_f33767 = *source_ml_source2_f33767;
                                        let source_ml_source2_f33768 = *source_ml_source2_f33768;
                                        {
                                            let next_3 = self::replace_java_script_call_frame(transition_heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(transition_state_3.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::push_java_script_call(program.clone(), next_3.clone(), source_ml_source2_f33767.clone(), source_ml_source2_f33768.clone(), source_ml_source2_f33769.clone())
                                        }
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationAllocate(transition_heap_4, transition_state_4, source_ml_source2_f33770) => {
                                        let transition_heap_4 = *transition_heap_4;
                                        let transition_state_4 = *transition_state_4;
                                        let source_ml_source2_f33770 = *source_ml_source2_f33770;
                                        {
                                            let next_4 = self::replace_java_script_call_frame(transition_heap_4.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(transition_state_4.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(next_4.clone()), Box::new(source_ml_source2_f33770.clone()))
                                        }
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationPrimitiveGet(transition_heap_5, transition_state_5, source_ml_source2_f33771, source_ml_source2_f33772) => {
                                        let transition_heap_5 = *transition_heap_5;
                                        let transition_state_5 = *transition_state_5;
                                        let source_ml_source2_f33771 = *source_ml_source2_f33771;
                                        let source_ml_source2_f33772 = *source_ml_source2_f33772;
                                        {
                                            let next_5 = self::replace_java_script_call_frame(transition_heap_5.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(transition_state_5.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineNeedPrimitiveIteratorGet(Box::new(next_5.clone()), Box::new(source_ml_source2_f33771.clone()), Box::new(source_ml_source2_f33772.clone()))
                                        }
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationNeedError(transition_heap_6, transition_state_6, source_ml_source2_f33773, source_ml_source2_f33774) => {
                                        let transition_heap_6 = *transition_heap_6;
                                        let transition_state_6 = *transition_state_6;
                                        {
                                            let next_6 = self::replace_java_script_call_frame(transition_heap_6.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(transition_state_6.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(next_6.clone()), realm.clone(), source_ml_source2_f33773.clone(), source_ml_source2_f33774.clone())
                                        }
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationCompleted(heap, value) => {
                                        let heap = *heap;
                                        let value = *value;
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationThrow(heap_2, value_2) => {
                                        let heap_2 = *heap_2;
                                        let value_2 = *value_2;
                                        self::throw_java_script_call_value(self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::CallIdle)), value_2.clone())
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationError(heap_3, error_class, reason) => {
                                        let heap_3 = *heap_3;
                                        self::request_java_script_call_error(self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::CallIdle)), realm.clone(), &(error_class), &(reason))
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(heap_4, reason_2) => {
                                        let heap_4 = *heap_4;
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_4.clone()), reason_2.clone())
                                    }
                                    self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(transition_heap_7, transition_state_7, source_ml_source2_f33775) => {
                                        let transition_heap_7 = *transition_heap_7;
                                        let transition_state_7 = *transition_state_7;
                                        {
                                            let next_7 = self::replace_java_script_call_frame(transition_heap_7.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(transition_state_7.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(next_7.clone()), source_ml_source2_f33775.clone())
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
pub fn accept_java_script_call_array_construction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    transition: self::JavaScriptConstructionTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field2) => {
            let frame = {
                let source_ml_source2_array_read_value33874 = shared_constructor_field2.clone();
                {
                    let source_ml_source2_array_read_index33875 = ({
                        let ml_reference_array2195 = shared_constructor_field2.clone();
                        let ml_reference_projection2195 =
                            ml_reference_array2195.borrow().len() as f64;
                        ml_reference_projection2195
                    } - 1f64);
                    {
                        let ml_reference_array2196 =
                            source_ml_source2_array_read_value33874.clone();
                        let ml_reference_position2196 = source_ml_source2_array_read_index33875;
                        let ml_reference_projection2196 = self::ml_array::optional_at(
                            &*ml_reference_array2196.borrow(),
                            self::ml_array::number_index(ml_reference_position2196),
                        );
                        ml_reference_projection2196
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
                        self::JavaScriptConstructionTransition::ConstructionContinue(
                            source_ml_source2_data_property_value33876,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33876 =
                                *source_ml_source2_data_property_value33876;
                            source_ml_source2_data_property_value33876.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionCall(
                            source_ml_source2_data_property_value33877,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33877 =
                                *source_ml_source2_data_property_value33877;
                            source_ml_source2_data_property_value33877.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionAllocate(
                            source_ml_source2_data_property_value33878,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33878 =
                                *source_ml_source2_data_property_value33878;
                            source_ml_source2_data_property_value33878.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionNeedAuthority(
                            source_ml_source2_data_property_value33879,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33879 =
                                *source_ml_source2_data_property_value33879;
                            source_ml_source2_data_property_value33879.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionNeedImplementation(
                            source_ml_source2_data_property_value33880,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33880 =
                                *source_ml_source2_data_property_value33880;
                            source_ml_source2_data_property_value33880.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionCompleted(
                            source_ml_source2_data_property_value33881,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33881 =
                                *source_ml_source2_data_property_value33881;
                            source_ml_source2_data_property_value33881.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionThrow(
                            source_ml_source2_data_property_value33882,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33882 =
                                *source_ml_source2_data_property_value33882;
                            source_ml_source2_data_property_value33882.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionError(
                            source_ml_source2_data_property_value33883,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33883 =
                                *source_ml_source2_data_property_value33883;
                            source_ml_source2_data_property_value33883.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionInvalid(
                            source_ml_source2_data_property_value33884,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33884 =
                                *source_ml_source2_data_property_value33884;
                            source_ml_source2_data_property_value33884.clone()
                        }
                        self::JavaScriptConstructionTransition::ConstructionResourceLimit(
                            source_ml_source2_data_property_value33885,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33885 =
                                *source_ml_source2_data_property_value33885;
                            source_ml_source2_data_property_value33885.clone()
                        }
                    }),
                    self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105,
                        111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                        99, 116, 117, 97, 108, 32, 111, 117, 116, 101, 114, 32, 99, 111, 110, 116,
                        105, 110, 117, 97, 116, 105, 111, 110,
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
                            source_ml_source2_data_property_value33886,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value33886 =
                                *source_ml_source2_data_property_value33886;
                            source_ml_source2_data_property_value33886.clone()
                        }
                    };
                    match pending.clone() {
                            self::JavaScriptCallPending::WaitArrayCreation(destination, pending_realm, pending_state, _, _, _) => {
                                let pending_state = *pending_state;
                                {
                                    let current = self::JavaScriptCallMachine::CallMachine(Box::new(match transition.clone() {
                                        self::JavaScriptConstructionTransition::ConstructionContinue(source_ml_source2_data_property_value33890, _) => {
                                            let source_ml_source2_data_property_value33890 = *source_ml_source2_data_property_value33890;
                                            source_ml_source2_data_property_value33890.clone()
                                        }
                                        self::JavaScriptConstructionTransition::ConstructionCall(source_ml_source2_data_property_value33891, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value33891 = *source_ml_source2_data_property_value33891;
                                            source_ml_source2_data_property_value33891.clone()
                                        }
                                        self::JavaScriptConstructionTransition::ConstructionAllocate(source_ml_source2_data_property_value33892, _, _) => {
                                            let source_ml_source2_data_property_value33892 = *source_ml_source2_data_property_value33892;
                                            source_ml_source2_data_property_value33892.clone()
                                        }
                                        self::JavaScriptConstructionTransition::ConstructionNeedAuthority(source_ml_source2_data_property_value33893, _, _) => {
                                            let source_ml_source2_data_property_value33893 = *source_ml_source2_data_property_value33893;
                                            source_ml_source2_data_property_value33893.clone()
                                        }
                                        self::JavaScriptConstructionTransition::ConstructionNeedImplementation(source_ml_source2_data_property_value33894, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value33894 = *source_ml_source2_data_property_value33894;
                                            source_ml_source2_data_property_value33894.clone()
                                        }
                                        self::JavaScriptConstructionTransition::ConstructionCompleted(source_ml_source2_data_property_value33895, _) => {
                                            let source_ml_source2_data_property_value33895 = *source_ml_source2_data_property_value33895;
                                            source_ml_source2_data_property_value33895.clone()
                                        }
                                        self::JavaScriptConstructionTransition::ConstructionThrow(source_ml_source2_data_property_value33896, _) => {
                                            let source_ml_source2_data_property_value33896 = *source_ml_source2_data_property_value33896;
                                            source_ml_source2_data_property_value33896.clone()
                                        }
                                        self::JavaScriptConstructionTransition::ConstructionError(source_ml_source2_data_property_value33897, _, _, _) => {
                                            let source_ml_source2_data_property_value33897 = *source_ml_source2_data_property_value33897;
                                            source_ml_source2_data_property_value33897.clone()
                                        }
                                        self::JavaScriptConstructionTransition::ConstructionInvalid(source_ml_source2_data_property_value33898, _) => {
                                            let source_ml_source2_data_property_value33898 = *source_ml_source2_data_property_value33898;
                                            source_ml_source2_data_property_value33898.clone()
                                        }
                                        self::JavaScriptConstructionTransition::ConstructionResourceLimit(source_ml_source2_data_property_value33899, _, _) => {
                                            let source_ml_source2_data_property_value33899 = *source_ml_source2_data_property_value33899;
                                            source_ml_source2_data_property_value33899.clone()
                                        }
                                    }), shared_constructor_field2.clone());
                                    match current.clone() {
                                        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                                            match transition.clone() {
                                                self::JavaScriptConstructionTransition::ConstructionContinue(heap, state) => {
                                                    let heap = *heap;
                                                    let state = *state;
                                                    {
                                                        let next = self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), {
                                                            let source_ml_source2_constructor_field33918 = Some(state.clone());
                                                            self::JavaScriptCallPending::WaitArrayCreation(destination, pending_realm.clone(), Box::new(pending_state.clone()), Some((source_ml_source2_constructor_field33918.clone(),)), None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                        }));
                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(next.clone()))
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionCall(heap_2, state_2, source_ml_source2_f33901, source_ml_source2_f33902, source_ml_source2_f33903, source_ml_source2_f33904, source_ml_source2_f33905) => {
                                                    let heap_2 = *heap_2;
                                                    let state_2 = *state_2;
                                                    let source_ml_source2_f33901 = *source_ml_source2_f33901;
                                                    let source_ml_source2_f33902 = *source_ml_source2_f33902;
                                                    let source_ml_source2_f33905 = *source_ml_source2_f33905;
                                                    {
                                                        let next_2 = self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), {
                                                            let source_ml_source2_constructor_field33933 = Some(state_2.clone());
                                                            self::JavaScriptCallPending::WaitArrayCreation(destination, pending_realm.clone(), Box::new(pending_state.clone()), Some((source_ml_source2_constructor_field33933.clone(),)), None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                        }));
                                                        if source_ml_source2_f33904 {
                                                            self::push_java_script_construction_body(program.clone(), next_2.clone(), source_ml_source2_f33901.clone(), source_ml_source2_f33902.clone(), source_ml_source2_f33903.clone(), source_ml_source2_f33905.clone())
                                                        } else {
                                                            self::push_java_script_call(program.clone(), next_2.clone(), source_ml_source2_f33901.clone(), source_ml_source2_f33902.clone(), source_ml_source2_f33903.clone())
                                                        }
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionAllocate(heap_3, state_3, source_ml_source2_f33906) => {
                                                    let heap_3 = *heap_3;
                                                    let state_3 = *state_3;
                                                    let source_ml_source2_f33906 = *source_ml_source2_f33906;
                                                    {
                                                        let next_3 = self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), {
                                                            let source_ml_source2_constructor_field33946 = Some(state_3.clone());
                                                            self::JavaScriptCallPending::WaitArrayCreation(destination, pending_realm.clone(), Box::new(pending_state.clone()), Some((source_ml_source2_constructor_field33946.clone(),)), None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                        }));
                                                        self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(next_3.clone()), Box::new(source_ml_source2_f33906.clone()))
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionNeedAuthority(heap_4, state_4, source_ml_source2_f33907) => {
                                                    let heap_4 = *heap_4;
                                                    let state_4 = *state_4;
                                                    let source_ml_source2_f33907 = *source_ml_source2_f33907;
                                                    {
                                                        let next_4 = self::replace_java_script_call_frame(heap_4.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), {
                                                            let source_ml_source2_constructor_field33959 = Some(state_4.clone());
                                                            self::JavaScriptCallPending::WaitArrayCreation(destination, pending_realm.clone(), Box::new(pending_state.clone()), Some((source_ml_source2_constructor_field33959.clone(),)), None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                        }));
                                                        self::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(Box::new(next_4.clone()), Box::new(source_ml_source2_f33907.clone()))
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionNeedImplementation(heap_5, state_5, source_ml_source2_f33908, source_ml_source2_f33909, source_ml_source2_f33910, source_ml_source2_f33911, source_ml_source2_f33912) => {
                                                    let heap_5 = *heap_5;
                                                    let state_5 = *state_5;
                                                    let source_ml_source2_f33908 = *source_ml_source2_f33908;
                                                    let source_ml_source2_f33909 = *source_ml_source2_f33909;
                                                    {
                                                        let next_5 = self::replace_java_script_call_frame(heap_5.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), {
                                                            let source_ml_source2_constructor_field33972 = Some(state_5.clone());
                                                            self::JavaScriptCallPending::WaitArrayCreation(destination, pending_realm.clone(), Box::new(pending_state.clone()), Some((source_ml_source2_constructor_field33972.clone(),)), None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                        }));
                                                        self::JavaScriptCallMachineTransition::MachineConstruct(Box::new(next_5.clone()), Box::new(source_ml_source2_f33908.clone()), Box::new(source_ml_source2_f33909.clone()), source_ml_source2_f33910.clone(), source_ml_source2_f33911.clone(), source_ml_source2_f33912.clone())
                                                    }
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionCompleted(heap_6, value) => {
                                                    let heap_6 = *heap_6;
                                                    let value = *value;
                                                    self::accept_java_script_call_array_creation(program.clone(), current.clone(), destination, pending_realm.clone(), self::resume_java_script_array_creation_construction(heap_6.clone(), pending_state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))))
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionThrow(heap_7, value_2) => {
                                                    let heap_7 = *heap_7;
                                                    let value_2 = *value_2;
                                                    self::accept_java_script_call_array_creation(program.clone(), current.clone(), destination, pending_realm.clone(), self::resume_java_script_array_creation_construction(heap_7.clone(), pending_state.clone(), self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(value_2.clone()))))
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionError(heap_8, error_class, reason, realm) => {
                                                    let heap_8 = *heap_8;
                                                    self::request_java_script_call_error(self::replace_java_script_call_frame(heap_8.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::CallIdle)), (((realm.clone().unwrap_or_else(|| (None::<self::ml::Big>,))).0).clone()).clone().unwrap_or_else(|| pending_realm.clone()), &(error_class), &(reason))
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionInvalid(heap_9, reason_2) => {
                                                    let heap_9 = *heap_9;
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_9.clone()), reason_2.clone())
                                                }
                                                self::JavaScriptConstructionTransition::ConstructionResourceLimit(heap_10, state_6, source_ml_source2_f33913) => {
                                                    let heap_10 = *heap_10;
                                                    let state_6 = *state_6;
                                                    {
                                                        let next_6 = self::replace_java_script_call_frame(heap_10.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), {
                                                            let source_ml_source2_constructor_field33986 = Some(state_6.clone());
                                                            self::JavaScriptCallPending::WaitArrayCreation(destination, pending_realm.clone(), Box::new(pending_state.clone()), Some((source_ml_source2_constructor_field33986.clone(),)), None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)
                                                        }));
                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(next_6.clone()), source_ml_source2_f33913.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                                    self::JavaScriptConstructionTransition::ConstructionContinue(source_ml_source2_data_property_value33997, _) => {
                                        let source_ml_source2_data_property_value33997 = *source_ml_source2_data_property_value33997;
                                        source_ml_source2_data_property_value33997.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionCall(source_ml_source2_data_property_value33998, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value33998 = *source_ml_source2_data_property_value33998;
                                        source_ml_source2_data_property_value33998.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionAllocate(source_ml_source2_data_property_value33999, _, _) => {
                                        let source_ml_source2_data_property_value33999 = *source_ml_source2_data_property_value33999;
                                        source_ml_source2_data_property_value33999.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionNeedAuthority(source_ml_source2_data_property_value34000, _, _) => {
                                        let source_ml_source2_data_property_value34000 = *source_ml_source2_data_property_value34000;
                                        source_ml_source2_data_property_value34000.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionNeedImplementation(source_ml_source2_data_property_value34001, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value34001 = *source_ml_source2_data_property_value34001;
                                        source_ml_source2_data_property_value34001.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionCompleted(source_ml_source2_data_property_value34002, _) => {
                                        let source_ml_source2_data_property_value34002 = *source_ml_source2_data_property_value34002;
                                        source_ml_source2_data_property_value34002.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionThrow(source_ml_source2_data_property_value34003, _) => {
                                        let source_ml_source2_data_property_value34003 = *source_ml_source2_data_property_value34003;
                                        source_ml_source2_data_property_value34003.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionError(source_ml_source2_data_property_value34004, _, _, _) => {
                                        let source_ml_source2_data_property_value34004 = *source_ml_source2_data_property_value34004;
                                        source_ml_source2_data_property_value34004.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionInvalid(source_ml_source2_data_property_value34005, _) => {
                                        let source_ml_source2_data_property_value34005 = *source_ml_source2_data_property_value34005;
                                        source_ml_source2_data_property_value34005.clone()
                                    }
                                    self::JavaScriptConstructionTransition::ConstructionResourceLimit(source_ml_source2_data_property_value34006, _, _) => {
                                        let source_ml_source2_data_property_value34006 = *source_ml_source2_data_property_value34006;
                                        source_ml_source2_data_property_value34006.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 117, 116, 101, 114, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                            }
                        }
                }
            }
        }
    }
}
