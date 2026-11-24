// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArrayCreationFallback;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptSpeciesConstructionCall;
use super::JavaScriptSpeciesConstructionState;
use super::JavaScriptSpeciesConstructionTransition;
use super::accept_java_script_call_argument_rest;
use super::accept_java_script_call_array_concatenation;
use super::accept_java_script_call_array_construction;
use super::accept_java_script_call_array_creation;
use super::accept_java_script_call_array_slice;
use super::accept_java_script_call_array_transform;
use super::accept_java_script_call_species_construction;
use super::allocate_java_script_array;
use super::classify_java_script_array_species_constructor;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_array_transform_directory;
use super::read_java_script_call_machine_error;
use super::read_java_script_construction_state_error;
use super::read_java_script_known_array_constructor_fact;
use super::resume_java_script_argument_rest_allocation;
use super::resume_java_script_array_concatenation_allocation;
use super::resume_java_script_array_creation_allocation;
use super::resume_java_script_array_creation_construction;
use super::resume_java_script_array_slice_allocation;
use super::resume_java_script_array_transform_allocation;
use super::resume_java_script_call_pending_allocation;
use super::resume_java_script_construction_allocation;
use super::resume_java_script_species_construction_allocation;
use super::retain_java_script_shared_species_construction;
use super::step_java_script_call_machine;
use super::test_java_script_semantic_same_value;
use super::test_java_script_shared_species_child_call;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_shared_species_construction_state_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptSpeciesConstructionState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            {
                let operation = match state.clone() {
                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value24864,
                    ) => ((source_ml_source2_data_property_value24864
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                    .0)
                        .clone(),
                };
                {
                    let classification = self::classify_java_script_array_species_constructor(
                        heap.clone(),
                        directory.clone(),
                        shared_constructor_field1.clone(),
                    );
                    {
                        let fact = self::read_java_script_known_array_constructor_fact(
                            classification.clone(),
                        );
                        if ((((((((((((({
                            let source_ml_source2_comparison_operand24867 = match state.clone() {
                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value24866,
                                    _,
                                ) => ((source_ml_source2_data_property_value24866
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                .0)
                                    .clone(),
                            };
                            !(match (
                                source_ml_source2_comparison_operand24867.clone(),
                                self::ml_string::Text::from_units(vec![
                                    115, 104, 97, 114, 101, 100, 32, 67, 111, 110, 115, 116, 114,
                                    117, 99, 116, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99,
                                    111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 49,
                                ]),
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        } || match (operation.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) || match (fact.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) || !(match fact.clone().expect("a proven present value") {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                _,
                                source_ml_source2_data_property_value24869,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value24869,
                        })) || ((match fact.clone().expect("a proven present value") {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                _,
                                _,
                                source_ml_source2_data_property_value24873,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value24873.clone(),
                        }) != (match shared_constructor_field2.clone() {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                _,
                                _,
                                source_ml_source2_data_property_value24875,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value24875.clone(),
                        }))) || ((match fact.clone().expect("a proven present value") {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                _,
                                source_ml_source2_data_property_value24879,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value24879,
                        }) != (match shared_constructor_field2.clone() {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                _,
                                source_ml_source2_data_property_value24881,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value24881,
                        }))) || ((match fact.clone().expect("a proven present value") {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value24885,
                                _,
                            ) => source_ml_source2_data_property_value24885,
                        }) != (match shared_constructor_field2.clone() {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value24887,
                                _,
                            ) => source_ml_source2_data_property_value24887,
                        }))) || {
                            let source_ml_source2_comparison_operand24891 =
                                match fact.clone().expect("a proven present value") {
                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value24888,
                                    ) => ((source_ml_source2_data_property_value24888
                                        .clone()
                                        .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                    .0)
                                        .clone(),
                                };
                            {
                                let source_ml_source2_comparison_operand24892 = match shared_constructor_field2.clone() {
                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value24890) => {
                                        ((source_ml_source2_data_property_value24890.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                    }
                                };
                                !(match (
                                    source_ml_source2_comparison_operand24891.clone(),
                                    source_ml_source2_comparison_operand24892.clone(),
                                ) {
                                    (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                    (None, None) => true,
                                    _ => false,
                                })
                            }
                        }) || {
                            let source_ml_source2_s24894 = shared_constructor_field1.clone();
                            match source_ml_source2_s24894.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            }
                        }) || !(((match shared_constructor_field1.clone() {
                            self::JavaScriptSemanticValue::Object(
                                _,
                                source_ml_source2_data_property_value24899,
                            ) => Some((source_ml_source2_data_property_value24899,)),
                            _ => None::<(bool,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0)) || (((((match shared_constructor_field1.clone() {
                            self::JavaScriptSemanticValue::Object(
                                source_ml_source2_data_property_value24904,
                                _,
                            ) => Some((source_ml_source2_data_property_value24904.clone(),)),
                            _ => None::<(self::ml::Big,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0)
                            .clone())
                            != (match fact.clone().expect("a proven present value") {
                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                    source_ml_source2_data_property_value24905,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value24905.clone(),
                            })))
                            || !(self::ml_math::is_safe_integer(shared_constructor_field3)))
                            || (shared_constructor_field3 < (0f64)))
                            || {
                                let source_ml_source2_same_value_operand24910 =
                                    shared_constructor_field3;
                                {
                                    let source_ml_source2_same_value_operand24911 = (-0f64);
                                    (((source_ml_source2_same_value_operand24910
                                        == source_ml_source2_same_value_operand24911)
                                        && ((source_ml_source2_same_value_operand24910 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand24910)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand24911))))
                                        || ((source_ml_source2_same_value_operand24910
                                            != source_ml_source2_same_value_operand24910)
                                            && (source_ml_source2_same_value_operand24911
                                                != source_ml_source2_same_value_operand24911)))
                                }
                            })
                        {
                            self::ml_string::Text::from_units(vec![
                                115, 104, 97, 114, 101, 100, 32, 115, 112, 101, 99, 105, 101, 115,
                                32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32,
                                114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101,
                                32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99,
                                108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32,
                                97, 110, 100, 32, 110, 111, 110, 110, 101, 103, 97, 116, 105, 118,
                                101, 32, 108, 101, 110, 103, 116, 104,
                            ])
                        } else {
                            if (((((((!(self::test_java_script_semantic_same_value(match operation.clone().expect("a proven present value") {
                                self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value24912, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value24912 = *source_ml_source2_data_property_value24912;
                                    source_ml_source2_data_property_value24912.clone()
                                }
                            }, shared_constructor_field1.clone())) || !(self::test_java_script_semantic_same_value(match operation.clone().expect("a proven present value") {
                                self::JavaScriptConstructionState::ConstructionState(_, source_ml_source2_data_property_value24914, _, _, _, _, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value24914 = *source_ml_source2_data_property_value24914;
                                    source_ml_source2_data_property_value24914.clone()
                                }
                            }, shared_constructor_field1.clone()))) || (({ let ml_reference_array1300 = match operation.clone().expect("a proven present value") {
                                self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value24917, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value24917.clone()
                                }
                            }; let ml_reference_projection1300 = ml_reference_array1300.borrow().len() as f64; ml_reference_projection1300 }) != (1f64))) || !(self::test_java_script_semantic_same_value(({
                                let source_ml_source2_array_read_value24919 = match operation.clone().expect("a proven present value") {
                                    self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value24918, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value24918.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_array_read_index24920 = 0f64;
                                    { let ml_reference_array1301 = source_ml_source2_array_read_value24919.clone(); let ml_reference_position1301 = source_ml_source2_array_read_index24920; let ml_reference_projection1301 = self::ml_array::optional_at(&*ml_reference_array1301.borrow(), self::ml_array::number_index(ml_reference_position1301)); ml_reference_projection1301 }
                                }
                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Number(shared_constructor_field3)))) || (shared_constructor_field4 != (match operation.clone().expect("a proven present value") {
                                self::JavaScriptConstructionState::ConstructionState(_, _, _, source_ml_source2_data_property_value24925, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value24925.clone()
                                }
                            }))) || (shared_constructor_field5 != (match operation.clone().expect("a proven present value") {
                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, source_ml_source2_data_property_value24929, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value24929.clone()
                                }
                            }))) || !(self::test_java_script_semantic_same_value(shared_constructor_field6.clone(), match operation.clone().expect("a proven present value") {
                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, source_ml_source2_data_property_value24931, _, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value24931 = *source_ml_source2_data_property_value24931;
                                    source_ml_source2_data_property_value24931.clone()
                                }
                            }))) || !(self::test_java_script_semantic_same_value(shared_constructor_field7.clone(), match operation.clone().expect("a proven present value") {
                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, source_ml_source2_data_property_value24933, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value24933 = *source_ml_source2_data_property_value24933;
                                    source_ml_source2_data_property_value24933.clone()
                                }
                            }))) {
                                self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 101, 120, 97, 99, 116, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 116, 97, 114, 103, 101, 116, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 115, 104, 97, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
                            } else {
                                {
                                    let call = match state.clone() {
                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24934, _, _) => {
                                            ((source_ml_source2_data_property_value24934.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                        }
                                    };
                                    {
                                        let actual = match operation.clone().expect("a proven present value") {
                                            self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value24935, _) => {
                                                ((source_ml_source2_data_property_value24935.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                            }
                                        };
                                        if ((match (call.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) != (match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                            self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108])
                                        } else {
                                            if ((!(match (call.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) && !(self::test_java_script_shared_species_child_call(call.clone().expect("a proven present value"), actual.clone().expect("a proven present value")))) {
                                                self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 104, 105, 108, 100, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 110, 100, 115])
                                            } else {
                                                self::read_java_script_construction_state_error(program.clone(), heap.clone(), directory.clone(), operation.clone().expect("a proven present value"))
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
pub fn adapt_java_script_species_construction(
    state: self::JavaScriptSpeciesConstructionState,
    transition: self::JavaScriptConstructionTransition,
) -> self::JavaScriptSpeciesConstructionTransition {
    match transition.clone() {
        self::JavaScriptConstructionTransition::ConstructionContinue(heap, transition_state) => {
            let heap = *heap;
            let transition_state = *transition_state;
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(
                Box::new(heap.clone()),
                Box::new(self::retain_java_script_shared_species_construction(
                    state.clone(),
                    transition_state.clone(),
                )),
            )
        }
        self::JavaScriptConstructionTransition::ConstructionCall(
            heap_2,
            transition_state_2,
            callee,
            receiver,
            arguments,
            constructing,
            _,
        ) => {
            let heap_2 = *heap_2;
            let transition_state_2 = *transition_state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCall(
                Box::new(heap_2.clone()),
                Box::new(self::retain_java_script_shared_species_construction(
                    state.clone(),
                    transition_state_2.clone(),
                )),
                Box::new(callee.clone()),
                Box::new(receiver.clone()),
                arguments.clone(),
                constructing,
            )
        }
        self::JavaScriptConstructionTransition::ConstructionAllocate(
            heap_3,
            transition_state_3,
            prototype,
        ) => {
            let heap_3 = *heap_3;
            let transition_state_3 = *transition_state_3;
            let prototype = *prototype;
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionAllocate(
                Box::new(heap_3.clone()),
                Box::new(self::retain_java_script_shared_species_construction(
                    state.clone(),
                    transition_state_3.clone(),
                )),
                Box::new(prototype.clone()),
            )
        }
        self::JavaScriptConstructionTransition::ConstructionNeedAuthority(
            heap_4,
            transition_state_4,
            constructor,
        ) => {
            let heap_4 = *heap_4;
            let transition_state_4 = *transition_state_4;
            let constructor = *constructor;
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedAuthority(
                Box::new(heap_4.clone()),
                Box::new(self::retain_java_script_shared_species_construction(
                    state.clone(),
                    transition_state_4.clone(),
                )),
                Box::new(constructor.clone()),
            )
        }
        self::JavaScriptConstructionTransition::ConstructionNeedImplementation(
            heap_5,
            transition_state_5,
            constructor_2,
            new_target,
            arguments_2,
            operation,
            reason,
        ) => {
            let heap_5 = *heap_5;
            let transition_state_5 = *transition_state_5;
            let constructor_2 = *constructor_2;
            let new_target = *new_target;
            {
                let source_ml_source2_constructor_field24938 =
                    self::retain_java_script_shared_species_construction(
                        state.clone(),
                        transition_state_5.clone(),
                    );
                {
                    let source_ml_source2_constructor_field24940 = Some(new_target.clone());
                    {
                        let source_ml_source2_constructor_field24941 = Some(std::rc::Rc::new(
                            std::cell::RefCell::new(self::ml_array::slice(
                                &{
                                    let ml_reference_value1302 = arguments_2.borrow().clone();
                                    ml_reference_value1302
                                },
                                0f64,
                                f64::INFINITY,
                            )),
                        ));
                        {
                            let source_ml_source2_constructor_field24942 = Some(operation.clone());
                            {
                                let source_ml_source2_constructor_field24943 = Some(reason.clone());
                                self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedImplementation(Box::new(heap_5.clone()), Box::new(source_ml_source2_constructor_field24938.clone()), Box::new(constructor_2.clone()), Some((source_ml_source2_constructor_field24940.clone(),)), Some((source_ml_source2_constructor_field24941.clone(),)), Some((source_ml_source2_constructor_field24942.clone(),)), Some((source_ml_source2_constructor_field24943.clone(),)))
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptConstructionTransition::ConstructionCompleted(heap_6, value) => {
            let heap_6 = *heap_6;
            let value = *value;
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCompleted(
                Box::new(heap_6.clone()),
                Box::new(value.clone()),
            )
        }
        self::JavaScriptConstructionTransition::ConstructionThrow(heap_7, value_2) => {
            let heap_7 = *heap_7;
            let value_2 = *value_2;
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionThrow(
                Box::new(heap_7.clone()),
                Box::new(value_2.clone()),
            )
        }
        self::JavaScriptConstructionTransition::ConstructionError(
            source_ml_source1_temporary_constructor_field1114,
            source_ml_source1_temporary_constructor_field1115,
            source_ml_source1_temporary_constructor_field1116,
            realm,
        ) => {
            let source_ml_source1_temporary_constructor_field1114 =
                *source_ml_source1_temporary_constructor_field1114;
            if {
                let source_ml_source2_comparison_operand24944 =
                    ((realm.clone().unwrap_or_else(|| (None::<self::ml::Big>,))).0).clone();
                {
                    let source_ml_source2_comparison_operand24945 = None::<()>;
                    match (
                        source_ml_source2_comparison_operand24944.clone(),
                        source_ml_source2_comparison_operand24945.clone(),
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
                self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionError(
                    Box::new(source_ml_source1_temporary_constructor_field1114.clone()),
                    source_ml_source1_temporary_constructor_field1115.clone(),
                    source_ml_source1_temporary_constructor_field1116.clone(),
                    None::<(Option<self::ml::Big>,)>,
                )
            } else {
                {
                    let source_ml_source2_constructor_field24952 =
                        ((realm.clone().unwrap_or_else(|| (None::<self::ml::Big>,))).0).clone();
                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionError(
                        Box::new(source_ml_source1_temporary_constructor_field1114.clone()),
                        source_ml_source1_temporary_constructor_field1115.clone(),
                        source_ml_source1_temporary_constructor_field1116.clone(),
                        Some((source_ml_source2_constructor_field24952.clone(),)),
                    )
                }
            }
        }
        self::JavaScriptConstructionTransition::ConstructionInvalid(heap_8, reason_2) => {
            let heap_8 = *heap_8;
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(
                Box::new(heap_8.clone()),
                reason_2.clone(),
            )
        }
        self::JavaScriptConstructionTransition::ConstructionResourceLimit(
            heap_9,
            transition_state_6,
            reason_3,
        ) => {
            let heap_9 = *heap_9;
            let transition_state_6 = *transition_state_6;
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionResourceLimit(
                Box::new(heap_9.clone()),
                Box::new(self::retain_java_script_shared_species_construction(
                    state.clone(),
                    transition_state_6.clone(),
                )),
                reason_3.clone(),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_allocation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    identity: self::ml::Big,
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
                            let source_ml_source2_array_read_value24956 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index24957 = ({
                                    let ml_reference_array1303 = shared_constructor_field2.clone();
                                    let ml_reference_projection1303 =
                                        ml_reference_array1303.borrow().len() as f64;
                                    ml_reference_projection1303
                                } - 1f64);
                                {
                                    let ml_reference_array1304 =
                                        source_ml_source2_array_read_value24956.clone();
                                    let ml_reference_position1304 =
                                        source_ml_source2_array_read_index24957;
                                    let ml_reference_projection1304 = self::ml_array::optional_at(
                                        &*ml_reference_array1304.borrow(),
                                        self::ml_array::number_index(ml_reference_position1304),
                                    );
                                    ml_reference_projection1304
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
                                    79, 98, 106, 101, 99, 116, 32, 114, 101, 115, 117, 109, 112,
                                    116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99,
                                    116, 105, 118, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                    111, 110, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111,
                                    110,
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
                                        source_ml_source2_data_property_value24959,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value24959 =
                                            *source_ml_source2_data_property_value24959;
                                        source_ml_source2_data_property_value24959.clone()
                                    }
                                };
                                match pending.clone() {
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
                                            self::resume_java_script_argument_rest_allocation(
                                                shared_constructor_field1.clone(),
                                                match frame.clone().expect("a proven present value")
                                                {
                                                    self::JavaScriptCallFrame::CallFrame(
                                                        _,
                                                        _,
                                                        _,
                                                        source_ml_source2_data_property_value24962,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source2_data_property_value24962
                                                        .clone(),
                                                },
                                                state.clone(),
                                                identity.clone(),
                                            ),
                                        )
                                    }
                                    self::JavaScriptCallPending::WaitArrayConcatenation(
                                        destination_2,
                                        realm_2,
                                        state_2,
                                        pending_construction,
                                    ) => {
                                        let state_2 = *state_2;
                                        {
                                            let directory = self::read_java_script_call_array_transform_directory(program.clone(), machine.clone(), destination_2, realm_2.clone());
                                            if match (directory.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                            } else {
                                                {
                                                    let construction = ((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                                    if !(match (construction.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }) {
                                                        self::accept_java_script_call_species_construction(program.clone(), machine.clone(), self::resume_java_script_species_construction_allocation(program.clone(), shared_constructor_field1.clone(), directory.clone().expect("a proven present value"), construction.clone().expect("a proven present value"), identity.clone()))
                                                    } else {
                                                        self::accept_java_script_call_array_concatenation(program.clone(), machine.clone(), destination_2, realm_2.clone(), self::resume_java_script_array_concatenation_allocation(shared_constructor_field1.clone(), directory.clone().expect("a proven present value"), state_2.clone(), identity.clone()))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitArraySlice(
                                        destination_3,
                                        realm_3,
                                        state_3,
                                        pending_construction_2,
                                    ) => {
                                        let state_3 = *state_3;
                                        {
                                            let directory_2 = self::read_java_script_call_array_transform_directory(program.clone(), machine.clone(), destination_3, realm_3.clone());
                                            if match (directory_2.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                            } else {
                                                {
                                                    let construction_2 = ((pending_construction_2.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                                    if !(match (construction_2.clone(), None::<()>)
                                                    {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }) {
                                                        self::accept_java_script_call_species_construction(program.clone(), machine.clone(), self::resume_java_script_species_construction_allocation(program.clone(), shared_constructor_field1.clone(), directory_2.clone().expect("a proven present value"), construction_2.clone().expect("a proven present value"), identity.clone()))
                                                    } else {
                                                        self::accept_java_script_call_array_slice(program.clone(), machine.clone(), destination_3, realm_3.clone(), self::resume_java_script_array_slice_allocation(shared_constructor_field1.clone(), directory_2.clone().expect("a proven present value"), state_3.clone(), identity.clone()))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitArrayTransform(
                                        destination_4,
                                        realm_4,
                                        state_4,
                                        pending_construction_3,
                                    ) => {
                                        let state_4 = *state_4;
                                        {
                                            let directory_3 = self::read_java_script_call_array_transform_directory(program.clone(), machine.clone(), destination_4, realm_4.clone());
                                            if match (directory_3.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                            } else {
                                                {
                                                    let construction_3 = ((pending_construction_3.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                                    if !(match (construction_3.clone(), None::<()>)
                                                    {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }) {
                                                        self::accept_java_script_call_species_construction(program.clone(), machine.clone(), self::resume_java_script_species_construction_allocation(program.clone(), shared_constructor_field1.clone(), directory_3.clone().expect("a proven present value"), construction_3.clone().expect("a proven present value"), identity.clone()))
                                                    } else {
                                                        self::accept_java_script_call_array_transform(program.clone(), machine.clone(), destination_4, realm_4.clone(), self::resume_java_script_array_transform_allocation(shared_constructor_field1.clone(), directory_3.clone().expect("a proven present value"), state_4.clone(), identity.clone()))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitArrayCreation(
                                        destination_5,
                                        realm_5,
                                        state_5,
                                        pending_construction_4,
                                        fallback,
                                        _,
                                    ) => {
                                        let state_5 = *state_5;
                                        {
                                            let request = self::step_java_script_call_machine(
                                                program.clone(),
                                                machine.clone(),
                                            );
                                            match request.clone() {
                                                self::JavaScriptCallMachineTransition::MachineAllocateObject(_, _) => {
                                                    {
                                                        let construction_4 = ((pending_construction_4.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone();
                                                        if !(match (construction_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                            {
                                                                let directory_4 = match program.clone() {
                                                                    self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value24974, _) => {
                                                                        ((source_ml_source2_data_property_value24974.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                    }
                                                                };
                                                                if match (directory_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                } else {
                                                                    self::accept_java_script_call_array_construction(program.clone(), machine.clone(), self::resume_java_script_construction_allocation(program.clone(), shared_constructor_field1.clone(), directory_4.clone().expect("a proven present value"), construction_4.clone().expect("a proven present value"), identity.clone()))
                                                                }
                                                            }
                                                        } else {
                                                            if !(match (((fallback.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationFallback>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                {
                                                                    let made = self::allocate_java_script_array(shared_constructor_field1.clone(), identity.clone(), {
                                                                        let source_ml_source2_nullish_field_receiver24978 = (((fallback.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationFallback>,))).0).clone(),);
                                                                        if match (source_ml_source2_nullish_field_receiver24978.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source2_nullish_field_present24979 = ((source_ml_source2_nullish_field_receiver24978.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source2_nullish_field_present24979.clone() {
                                                                                    self::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(source_ml_source2_data_property_value24980, _) => {
                                                                                        let source_ml_source2_data_property_value24980 = *source_ml_source2_data_property_value24980;
                                                                                        source_ml_source2_data_property_value24980.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }, {
                                                                        let source_ml_source2_nullish_field_receiver24981 = (((fallback.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationFallback>,))).0).clone(),);
                                                                        if match (source_ml_source2_nullish_field_receiver24981.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source2_nullish_field_present24982 = ((source_ml_source2_nullish_field_receiver24981.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source2_nullish_field_present24982.clone() {
                                                                                    self::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(_, source_ml_source2_data_property_value24983) => {
                                                                                        source_ml_source2_data_property_value24983
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    });
                                                                    match made.clone() {
                                                                        self::JavaScriptArrayCompletion::ArrayNormal(heap, value) => {
                                                                            let heap = *heap;
                                                                            let value = *value;
                                                                            self::accept_java_script_call_array_creation(program.clone(), machine.clone(), destination_5, realm_5.clone(), self::resume_java_script_array_creation_construction(heap.clone(), state_5.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))))
                                                                        }
                                                                        self::JavaScriptArrayCompletion::ArrayPropertyComplete(heap_2, _, _) => {
                                                                            let heap_2 = *heap_2;
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_2.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116]))
                                                                        }
                                                                        self::JavaScriptArrayCompletion::ArrayNumberRequest(heap_3, _, _) => {
                                                                            let heap_3 = *heap_3;
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116]))
                                                                        }
                                                                        self::JavaScriptArrayCompletion::ArrayDefinitionContinue(heap_4, _) => {
                                                                            let heap_4 = *heap_4;
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_4.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116]))
                                                                        }
                                                                        self::JavaScriptArrayCompletion::ArrayInvalid(heap_5, reason) => {
                                                                            let heap_5 = *heap_5;
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_5.clone()), reason.clone())
                                                                        }
                                                                        self::JavaScriptArrayCompletion::ArrayError(heap_6, _, reason_2) => {
                                                                            let heap_6 = *heap_6;
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_6.clone()), reason_2.clone())
                                                                        }
                                                                        self::JavaScriptArrayCompletion::ArrayResourceLimit(_, reason_3, _) => {
                                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_3.clone())
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                self::accept_java_script_call_array_creation(program.clone(), machine.clone(), destination_5, realm_5.clone(), self::resume_java_script_array_creation_allocation(shared_constructor_field1.clone(), state_5.clone(), identity.clone()))
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    if match request.clone() {
                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(_, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    } {
                                                        request.clone()
                                                    } else {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 114, 101, 115, 112, 111, 110, 115, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 101, 115, 116]))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => self::resume_java_script_call_pending_allocation(
                                        program.clone(),
                                        machine.clone(),
                                        frame.clone().expect("a proven present value"),
                                        identity.clone(),
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
