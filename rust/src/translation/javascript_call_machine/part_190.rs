// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionState;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptSpeciesConstructionState;
use super::JavaScriptSpeciesConstructionTransition;
use super::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434;
use super::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyLoop1435Result;
use super::classify_java_script_array_species_constructor;
use super::create_java_script_call_frame;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::push_java_script_construction_body;
use super::read_java_script_call_array_transform_directory;
use super::read_java_script_executable_call_function;
use super::read_java_script_known_array_constructor_fact;
use super::read_java_script_ordinary_class_receiver_error;
use super::read_java_script_pending_construction;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::replace_java_script_pending_species_construction;
use super::request_java_script_call_error;
use super::resume_java_script_call_array_species_operation;
use super::source_ml_source1_temporary_push_java_script_species_constructor_body_loop1435;
use super::test_java_script_ordinary_class_constructor_mode;
use super::throw_java_script_call_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_species_construction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    transition: self::JavaScriptSpeciesConstructionTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(Box::new(match transition.clone() {
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(source_ml_source2_data_property_value33416, _) => {
                let source_ml_source2_data_property_value33416 = *source_ml_source2_data_property_value33416;
                source_ml_source2_data_property_value33416.clone()
            }
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCall(source_ml_source2_data_property_value33417, _, _, _, _, _) => {
                let source_ml_source2_data_property_value33417 = *source_ml_source2_data_property_value33417;
                source_ml_source2_data_property_value33417.clone()
            }
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionAllocate(source_ml_source2_data_property_value33418, _, _) => {
                let source_ml_source2_data_property_value33418 = *source_ml_source2_data_property_value33418;
                source_ml_source2_data_property_value33418.clone()
            }
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedAuthority(source_ml_source2_data_property_value33419, _, _) => {
                let source_ml_source2_data_property_value33419 = *source_ml_source2_data_property_value33419;
                source_ml_source2_data_property_value33419.clone()
            }
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedImplementation(source_ml_source2_data_property_value33420, _, _, _, _, _, _) => {
                let source_ml_source2_data_property_value33420 = *source_ml_source2_data_property_value33420;
                source_ml_source2_data_property_value33420.clone()
            }
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCompleted(source_ml_source2_data_property_value33421, _) => {
                let source_ml_source2_data_property_value33421 = *source_ml_source2_data_property_value33421;
                source_ml_source2_data_property_value33421.clone()
            }
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionThrow(source_ml_source2_data_property_value33422, _) => {
                let source_ml_source2_data_property_value33422 = *source_ml_source2_data_property_value33422;
                source_ml_source2_data_property_value33422.clone()
            }
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionError(source_ml_source2_data_property_value33423, _, _, _) => {
                let source_ml_source2_data_property_value33423 = *source_ml_source2_data_property_value33423;
                source_ml_source2_data_property_value33423.clone()
            }
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(source_ml_source2_data_property_value33424, _) => {
                let source_ml_source2_data_property_value33424 = *source_ml_source2_data_property_value33424;
                source_ml_source2_data_property_value33424.clone()
            }
            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionResourceLimit(source_ml_source2_data_property_value33425, _, _) => {
                let source_ml_source2_data_property_value33425 = *source_ml_source2_data_property_value33425;
                source_ml_source2_data_property_value33425.clone()
            }
        }), match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value33426) => {
                source_ml_source2_data_property_value33426.clone()
            }
        });
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field6) => {
                let frame = {
                    let source_ml_source2_array_read_value33429 = shared_constructor_field6.clone();
                    {
                        let source_ml_source2_array_read_index33430 = ({
                            let ml_reference_array2175 = shared_constructor_field6.clone();
                            let ml_reference_projection2175 =
                                ml_reference_array2175.borrow().len() as f64;
                            ml_reference_projection2175
                        } - 1f64);
                        {
                            let ml_reference_array2176 =
                                source_ml_source2_array_read_value33429.clone();
                            let ml_reference_position2176 = source_ml_source2_array_read_index33430;
                            let ml_reference_projection2176 = self::ml_array::optional_at(
                                &*ml_reference_array2176.borrow(),
                                self::ml_array::number_index(ml_reference_position2176),
                            );
                            ml_reference_projection2176
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(source_ml_source2_data_property_value33431, _) => {
                                let source_ml_source2_data_property_value33431 = *source_ml_source2_data_property_value33431;
                                source_ml_source2_data_property_value33431.clone()
                            }
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCall(source_ml_source2_data_property_value33432, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value33432 = *source_ml_source2_data_property_value33432;
                                source_ml_source2_data_property_value33432.clone()
                            }
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionAllocate(source_ml_source2_data_property_value33433, _, _) => {
                                let source_ml_source2_data_property_value33433 = *source_ml_source2_data_property_value33433;
                                source_ml_source2_data_property_value33433.clone()
                            }
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedAuthority(source_ml_source2_data_property_value33434, _, _) => {
                                let source_ml_source2_data_property_value33434 = *source_ml_source2_data_property_value33434;
                                source_ml_source2_data_property_value33434.clone()
                            }
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedImplementation(source_ml_source2_data_property_value33435, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value33435 = *source_ml_source2_data_property_value33435;
                                source_ml_source2_data_property_value33435.clone()
                            }
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCompleted(source_ml_source2_data_property_value33436, _) => {
                                let source_ml_source2_data_property_value33436 = *source_ml_source2_data_property_value33436;
                                source_ml_source2_data_property_value33436.clone()
                            }
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionThrow(source_ml_source2_data_property_value33437, _) => {
                                let source_ml_source2_data_property_value33437 = *source_ml_source2_data_property_value33437;
                                source_ml_source2_data_property_value33437.clone()
                            }
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionError(source_ml_source2_data_property_value33438, _, _, _) => {
                                let source_ml_source2_data_property_value33438 = *source_ml_source2_data_property_value33438;
                                source_ml_source2_data_property_value33438.clone()
                            }
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(source_ml_source2_data_property_value33439, _) => {
                                let source_ml_source2_data_property_value33439 = *source_ml_source2_data_property_value33439;
                                source_ml_source2_data_property_value33439.clone()
                            }
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionResourceLimit(source_ml_source2_data_property_value33440, _, _) => {
                                let source_ml_source2_data_property_value33440 = *source_ml_source2_data_property_value33440;
                                source_ml_source2_data_property_value33440.clone()
                            }
                        }), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    {
                        let pending = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value33441,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33441 =
                                    *source_ml_source2_data_property_value33441;
                                source_ml_source2_data_property_value33441.clone()
                            }
                        };
                        if ((match pending.clone() {
                            self::JavaScriptCallPending::WaitArrayTransform(_, _, _, _) => false,
                            _ => true,
                        } && match pending.clone() {
                            self::JavaScriptCallPending::WaitArraySlice(_, _, _, _) => false,
                            _ => true,
                        }) && match pending.clone() {
                            self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, _) => {
                                false
                            }
                            _ => true,
                        }) {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(source_ml_source2_data_property_value33454, _) => {
                                        let source_ml_source2_data_property_value33454 = *source_ml_source2_data_property_value33454;
                                        source_ml_source2_data_property_value33454.clone()
                                    }
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCall(source_ml_source2_data_property_value33455, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value33455 = *source_ml_source2_data_property_value33455;
                                        source_ml_source2_data_property_value33455.clone()
                                    }
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionAllocate(source_ml_source2_data_property_value33456, _, _) => {
                                        let source_ml_source2_data_property_value33456 = *source_ml_source2_data_property_value33456;
                                        source_ml_source2_data_property_value33456.clone()
                                    }
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedAuthority(source_ml_source2_data_property_value33457, _, _) => {
                                        let source_ml_source2_data_property_value33457 = *source_ml_source2_data_property_value33457;
                                        source_ml_source2_data_property_value33457.clone()
                                    }
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedImplementation(source_ml_source2_data_property_value33458, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value33458 = *source_ml_source2_data_property_value33458;
                                        source_ml_source2_data_property_value33458.clone()
                                    }
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCompleted(source_ml_source2_data_property_value33459, _) => {
                                        let source_ml_source2_data_property_value33459 = *source_ml_source2_data_property_value33459;
                                        source_ml_source2_data_property_value33459.clone()
                                    }
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionThrow(source_ml_source2_data_property_value33460, _) => {
                                        let source_ml_source2_data_property_value33460 = *source_ml_source2_data_property_value33460;
                                        source_ml_source2_data_property_value33460.clone()
                                    }
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionError(source_ml_source2_data_property_value33461, _, _, _) => {
                                        let source_ml_source2_data_property_value33461 = *source_ml_source2_data_property_value33461;
                                        source_ml_source2_data_property_value33461.clone()
                                    }
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(source_ml_source2_data_property_value33462, _) => {
                                        let source_ml_source2_data_property_value33462 = *source_ml_source2_data_property_value33462;
                                        source_ml_source2_data_property_value33462.clone()
                                    }
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionResourceLimit(source_ml_source2_data_property_value33463, _, _) => {
                                        let source_ml_source2_data_property_value33463 = *source_ml_source2_data_property_value33463;
                                        source_ml_source2_data_property_value33463.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110]))
                        } else {
                            {
                                let directory =
                                    self::read_java_script_call_array_transform_directory(
                                        program.clone(),
                                        current.clone(),
                                        ((match pending.clone() {
                                            self::JavaScriptCallPending::WaitArrayConcatenation(
                                                source_ml_source2_data_property_value33464,
                                                _,
                                                _,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value33464,))
                                            }
                                            self::JavaScriptCallPending::WaitArraySlice(
                                                source_ml_source2_data_property_value33465,
                                                _,
                                                _,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value33465,))
                                            }
                                            self::JavaScriptCallPending::WaitArrayTransform(
                                                source_ml_source2_data_property_value33466,
                                                _,
                                                _,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value33466,))
                                            }
                                            _ => None::<(f64,)>,
                                        })
                                        .clone()
                                        .expect("a proven present value"))
                                        .0,
                                        (((match pending.clone() {
                                            self::JavaScriptCallPending::WaitArrayConcatenation(
                                                _,
                                                source_ml_source2_data_property_value33467,
                                                _,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value33467
                                                    .clone(),))
                                            }
                                            self::JavaScriptCallPending::WaitArraySlice(
                                                _,
                                                source_ml_source2_data_property_value33468,
                                                _,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value33468
                                                    .clone(),))
                                            }
                                            self::JavaScriptCallPending::WaitArrayTransform(
                                                _,
                                                source_ml_source2_data_property_value33469,
                                                _,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value33469
                                                    .clone(),))
                                            }
                                            _ => None::<(self::ml::Big,)>,
                                        })
                                        .clone()
                                        .expect("a proven present value"))
                                        .0)
                                            .clone(),
                                    );
                                if match (directory.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(source_ml_source2_data_property_value33470, _) => {
                                                let source_ml_source2_data_property_value33470 = *source_ml_source2_data_property_value33470;
                                                source_ml_source2_data_property_value33470.clone()
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCall(source_ml_source2_data_property_value33471, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value33471 = *source_ml_source2_data_property_value33471;
                                                source_ml_source2_data_property_value33471.clone()
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionAllocate(source_ml_source2_data_property_value33472, _, _) => {
                                                let source_ml_source2_data_property_value33472 = *source_ml_source2_data_property_value33472;
                                                source_ml_source2_data_property_value33472.clone()
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedAuthority(source_ml_source2_data_property_value33473, _, _) => {
                                                let source_ml_source2_data_property_value33473 = *source_ml_source2_data_property_value33473;
                                                source_ml_source2_data_property_value33473.clone()
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedImplementation(source_ml_source2_data_property_value33474, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value33474 = *source_ml_source2_data_property_value33474;
                                                source_ml_source2_data_property_value33474.clone()
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCompleted(source_ml_source2_data_property_value33475, _) => {
                                                let source_ml_source2_data_property_value33475 = *source_ml_source2_data_property_value33475;
                                                source_ml_source2_data_property_value33475.clone()
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionThrow(source_ml_source2_data_property_value33476, _) => {
                                                let source_ml_source2_data_property_value33476 = *source_ml_source2_data_property_value33476;
                                                source_ml_source2_data_property_value33476.clone()
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionError(source_ml_source2_data_property_value33477, _, _, _) => {
                                                let source_ml_source2_data_property_value33477 = *source_ml_source2_data_property_value33477;
                                                source_ml_source2_data_property_value33477.clone()
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(source_ml_source2_data_property_value33478, _) => {
                                                let source_ml_source2_data_property_value33478 = *source_ml_source2_data_property_value33478;
                                                source_ml_source2_data_property_value33478.clone()
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionResourceLimit(source_ml_source2_data_property_value33479, _, _) => {
                                                let source_ml_source2_data_property_value33479 = *source_ml_source2_data_property_value33479;
                                                source_ml_source2_data_property_value33479.clone()
                                            }
                                        }), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                } else {
                                    match transition.clone() {
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(heap, state) => {
                                                let heap = *heap;
                                                let state = *state;
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field6.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::replace_java_script_pending_species_construction(pending.clone(), state.clone())))))
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCall(heap_2, state_2, callee, receiver, arguments, constructing) => {
                                                let heap_2 = *heap_2;
                                                let state_2 = *state_2;
                                                let callee = *callee;
                                                let receiver = *receiver;
                                                {
                                                    let next = self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field6.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::replace_java_script_pending_species_construction(pending.clone(), state_2.clone())));
                                                    if constructing {
                                                        self::push_java_script_species_constructor_body(program.clone(), next.clone(), callee.clone(), receiver.clone(), arguments.clone())
                                                    } else {
                                                        self::push_java_script_call(program.clone(), next.clone(), callee.clone(), receiver.clone(), arguments.clone())
                                                    }
                                                }
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionAllocate(heap_3, state_3, prototype) => {
                                                let heap_3 = *heap_3;
                                                let state_3 = *state_3;
                                                let prototype = *prototype;
                                                self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field6.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::replace_java_script_pending_species_construction(pending.clone(), state_3.clone())))), Box::new(prototype.clone()))
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedAuthority(heap_4, state_4, constructor) => {
                                                let heap_4 = *heap_4;
                                                let state_4 = *state_4;
                                                let constructor = *constructor;
                                                self::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(Box::new(self::replace_java_script_call_frame(heap_4.clone(), shared_constructor_field6.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::replace_java_script_pending_species_construction(pending.clone(), state_4.clone())))), Box::new(constructor.clone()))
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedImplementation(heap_5, state_5, constructor_2, transition_new_target, arguments_2, transition_operation, transition_reason) => {
                                                let heap_5 = *heap_5;
                                                let state_5 = *state_5;
                                                let constructor_2 = *constructor_2;
                                                {
                                                    let actual = self::read_java_script_known_array_constructor_fact(self::classify_java_script_array_species_constructor(heap_5.clone(), directory.clone().expect("a proven present value"), constructor_2.clone()));
                                                    if match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_5.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 115, 112, 101, 99, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 108, 101, 103, 97, 116, 101, 100, 32, 116, 97, 114, 103, 101, 116, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                    } else {
                                                        {
                                                            let next_2 = self::replace_java_script_call_frame(heap_5.clone(), shared_constructor_field6.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::replace_java_script_pending_species_construction(pending.clone(), state_5.clone())));
                                                            {
                                                                let new_target = ((transition_new_target.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                                                {
                                                                    let operation = ((transition_operation.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                                                    {
                                                                        let reason = ((transition_reason.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                                                        {
                                                                            let source_ml_source1_temporary_constructor_field1428 = (((arguments_2.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Number(match state_5.clone() {
                                                                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, source_ml_source2_data_property_value33488, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value33488
                                                                                }
                                                                            })])));
                                                                            {
                                                                                let source_ml_source1_temporary_constructor_field1429 = {
                                                                                    let source_ml_source2_nominal_spread33489 = actual.clone().expect("a proven present value");
                                                                                    match source_ml_source2_nominal_spread33489.clone() {
                                                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5) => {
                                                                                            {
                                                                                                let source_ml_source2_nominal_field33490 = shared_constructor_field1.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field33492 = shared_constructor_field2;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field33494 = shared_constructor_field3.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field33496 = shared_constructor_field4;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field33498 = shared_constructor_field5.clone();
                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(source_ml_source2_nominal_field33490.clone(), source_ml_source2_nominal_field33492, source_ml_source2_nominal_field33494.clone(), source_ml_source2_nominal_field33496, source_ml_source2_nominal_field33498.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                };
                                                                                if match (new_target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                    if {
                                                                                        let source_ml_source2_comparison_operand33501 = None::<()>;
                                                                                        match (operation.clone(), source_ml_source2_comparison_operand33501.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                    } {
                                                                                        if {
                                                                                            let source_ml_source2_comparison_operand33503 = None::<()>;
                                                                                            match (reason.clone(), source_ml_source2_comparison_operand33503.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                        } {
                                                                                            self::JavaScriptCallMachineTransition::MachineConstructArraySpecies(Box::new(next_2.clone()), Box::new(constructor_2.clone()), source_ml_source1_temporary_constructor_field1428.clone(), Box::new(source_ml_source1_temporary_constructor_field1429.clone()), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::ml_string::Text>,)>)
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source2_constructor_field33512 = Some(reason.clone().expect("a proven present value"));
                                                                                                self::JavaScriptCallMachineTransition::MachineConstructArraySpecies(Box::new(next_2.clone()), Box::new(constructor_2.clone()), source_ml_source1_temporary_constructor_field1428.clone(), Box::new(source_ml_source1_temporary_constructor_field1429.clone()), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::ml_string::Text>,)>, Some((source_ml_source2_constructor_field33512.clone(),)))
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source1_temporary_constructor_field1430 = operation.clone().expect("a proven present value");
                                                                                            if {
                                                                                                let source_ml_source2_comparison_operand33514 = None::<()>;
                                                                                                match (reason.clone(), source_ml_source2_comparison_operand33514.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                            } {
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field33519 = Some(source_ml_source1_temporary_constructor_field1430.clone());
                                                                                                    self::JavaScriptCallMachineTransition::MachineConstructArraySpecies(Box::new(next_2.clone()), Box::new(constructor_2.clone()), source_ml_source1_temporary_constructor_field1428.clone(), Box::new(source_ml_source1_temporary_constructor_field1429.clone()), None::<(Option<self::JavaScriptSemanticValue>,)>, Some((source_ml_source2_constructor_field33519.clone(),)), None::<(Option<self::ml_string::Text>,)>)
                                                                                                }
                                                                                            } else {
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field33524 = Some(source_ml_source1_temporary_constructor_field1430.clone());
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field33525 = Some(reason.clone().expect("a proven present value"));
                                                                                                        self::JavaScriptCallMachineTransition::MachineConstructArraySpecies(Box::new(next_2.clone()), Box::new(constructor_2.clone()), source_ml_source1_temporary_constructor_field1428.clone(), Box::new(source_ml_source1_temporary_constructor_field1429.clone()), None::<(Option<self::JavaScriptSemanticValue>,)>, Some((source_ml_source2_constructor_field33524.clone(),)), Some((source_ml_source2_constructor_field33525.clone(),)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source1_temporary_constructor_field1431 = new_target.clone().expect("a proven present value");
                                                                                        if {
                                                                                            let source_ml_source2_comparison_operand33527 = None::<()>;
                                                                                            match (operation.clone(), source_ml_source2_comparison_operand33527.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                        } {
                                                                                            if {
                                                                                                let source_ml_source2_comparison_operand33529 = None::<()>;
                                                                                                match (reason.clone(), source_ml_source2_comparison_operand33529.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                            } {
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field33534 = Some(source_ml_source1_temporary_constructor_field1431.clone());
                                                                                                    self::JavaScriptCallMachineTransition::MachineConstructArraySpecies(Box::new(next_2.clone()), Box::new(constructor_2.clone()), source_ml_source1_temporary_constructor_field1428.clone(), Box::new(source_ml_source1_temporary_constructor_field1429.clone()), Some((source_ml_source2_constructor_field33534.clone(),)), None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::ml_string::Text>,)>)
                                                                                                }
                                                                                            } else {
                                                                                                {
                                                                                                    let source_ml_source2_constructor_field33539 = Some(source_ml_source1_temporary_constructor_field1431.clone());
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field33540 = Some(reason.clone().expect("a proven present value"));
                                                                                                        self::JavaScriptCallMachineTransition::MachineConstructArraySpecies(Box::new(next_2.clone()), Box::new(constructor_2.clone()), source_ml_source1_temporary_constructor_field1428.clone(), Box::new(source_ml_source1_temporary_constructor_field1429.clone()), Some((source_ml_source2_constructor_field33539.clone(),)), None::<(Option<self::ml_string::Text>,)>, Some((source_ml_source2_constructor_field33540.clone(),)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source1_temporary_constructor_field1432 = operation.clone().expect("a proven present value");
                                                                                                if {
                                                                                                    let source_ml_source2_comparison_operand33542 = None::<()>;
                                                                                                    match (reason.clone(), source_ml_source2_comparison_operand33542.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                                } {
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field33547 = Some(source_ml_source1_temporary_constructor_field1431.clone());
                                                                                                        {
                                                                                                            let source_ml_source2_constructor_field33548 = Some(source_ml_source1_temporary_constructor_field1432.clone());
                                                                                                            self::JavaScriptCallMachineTransition::MachineConstructArraySpecies(Box::new(next_2.clone()), Box::new(constructor_2.clone()), source_ml_source1_temporary_constructor_field1428.clone(), Box::new(source_ml_source1_temporary_constructor_field1429.clone()), Some((source_ml_source2_constructor_field33547.clone(),)), Some((source_ml_source2_constructor_field33548.clone(),)), None::<(Option<self::ml_string::Text>,)>)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_source2_constructor_field33553 = Some(source_ml_source1_temporary_constructor_field1431.clone());
                                                                                                        {
                                                                                                            let source_ml_source2_constructor_field33554 = Some(source_ml_source1_temporary_constructor_field1432.clone());
                                                                                                            {
                                                                                                                let source_ml_source2_constructor_field33555 = Some(reason.clone().expect("a proven present value"));
                                                                                                                self::JavaScriptCallMachineTransition::MachineConstructArraySpecies(Box::new(next_2.clone()), Box::new(constructor_2.clone()), source_ml_source1_temporary_constructor_field1428.clone(), Box::new(source_ml_source1_temporary_constructor_field1429.clone()), Some((source_ml_source2_constructor_field33553.clone(),)), Some((source_ml_source2_constructor_field33554.clone(),)), Some((source_ml_source2_constructor_field33555.clone(),)))
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
                                                    }
                                                }
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCompleted(_, value) => {
                                                let value = *value;
                                                self::resume_java_script_call_array_species_operation(program.clone(), current.clone(), frame.clone().expect("a proven present value"), directory.clone().expect("a proven present value"), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone())), true)
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionThrow(_, value_2) => {
                                                let value_2 = *value_2;
                                                self::throw_java_script_call_value(current.clone(), value_2.clone())
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionError(_, error_class, reason_2, realm) => {
                                                self::request_java_script_call_error(current.clone(), (((realm.clone().unwrap_or_else(|| (None::<self::ml::Big>,))).0).clone()).clone().unwrap_or_else(|| (((match pending.clone() {
                                                    self::JavaScriptCallPending::WaitArrayConcatenation(_, source_ml_source2_data_property_value33556, _, _) => {
                                                        Some((source_ml_source2_data_property_value33556.clone(),))
                                                    }
                                                    self::JavaScriptCallPending::WaitArraySlice(_, source_ml_source2_data_property_value33557, _, _) => {
                                                        Some((source_ml_source2_data_property_value33557.clone(),))
                                                    }
                                                    self::JavaScriptCallPending::WaitArrayTransform(_, source_ml_source2_data_property_value33558, _, _) => {
                                                        Some((source_ml_source2_data_property_value33558.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone()), &(error_class), &(reason_2))
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(heap_6, reason_3) => {
                                                let heap_6 = *heap_6;
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_6.clone()), reason_3.clone())
                                            }
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionResourceLimit(heap_7, state_6, reason_4) => {
                                                let heap_7 = *heap_7;
                                                let state_6 = *state_6;
                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(heap_7.clone(), shared_constructor_field6.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::replace_java_script_pending_species_construction(pending.clone(), state_6.clone())))), reason_4.clone())
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
pub fn push_java_script_species_constructor_body(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    constructor: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field11 = *shared_constructor_field11;
            {
                let parent = {
                    let source_ml_source2_array_read_value33562 =
                        shared_constructor_field12.clone();
                    {
                        let source_ml_source2_array_read_index33563 = ({
                            let ml_reference_array2177 = shared_constructor_field12.clone();
                            let ml_reference_projection2177 =
                                ml_reference_array2177.borrow().len() as f64;
                            ml_reference_projection2177
                        } - 1f64);
                        {
                            let ml_reference_array2178 =
                                source_ml_source2_array_read_value33562.clone();
                            let ml_reference_position2178 = source_ml_source2_array_read_index33563;
                            let ml_reference_projection2178 = self::ml_array::optional_at(
                                &*ml_reference_array2178.borrow(),
                                self::ml_array::number_index(ml_reference_position2178),
                            );
                            ml_reference_projection2178
                        }
                    }
                };
                {
                    let operation = if match (parent.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        None::<self::JavaScriptConstructionState>
                    } else {
                        self::read_java_script_pending_construction(
                            match parent.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value33564,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value33564 =
                                        *source_ml_source2_data_property_value33564;
                                    source_ml_source2_data_property_value33564.clone()
                                }
                            },
                        )
                    };
                    if !(match (operation.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::push_java_script_construction_body(
                            program.clone(),
                            machine.clone(),
                            constructor.clone(),
                            receiver.clone(),
                            arguments_.clone(),
                            match operation.clone().expect("a proven present value") {
                                self::JavaScriptConstructionState::ConstructionState(
                                    _,
                                    source_ml_source2_data_property_value33565,
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
                                    let source_ml_source2_data_property_value33565 =
                                        *source_ml_source2_data_property_value33565;
                                    source_ml_source2_data_property_value33565.clone()
                                }
                            },
                        )
                    } else {
                        if (((match constructor.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        } || !(((match constructor.clone() {
                            self::JavaScriptSemanticValue::Object(
                                _,
                                source_ml_source2_data_property_value33568,
                            ) => Some((source_ml_source2_data_property_value33568,)),
                            _ => None::<(bool,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0)) || match receiver.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }) || ((match receiver.clone() {
                            self::JavaScriptSemanticValue::Object(
                                _,
                                source_ml_source2_data_property_value33571,
                            ) => Some((source_ml_source2_data_property_value33571,)),
                            _ => None::<(bool,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0)
                        {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field11.clone()),
                                self::ml_string::Text::from_units(vec![
                                    115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116,
                                    114, 117, 99, 116, 111, 114, 32, 98, 111, 100, 121, 32, 114,
                                    101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                    116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32,
                                    97, 110, 100, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32,
                                    114, 101, 99, 101, 105, 118, 101, 114,
                                ]),
                            )
                        } else {
                            {
                                let fn_ = self::read_java_script_executable_call_function(
                                    program.clone(),
                                    shared_constructor_field11.clone(),
                                    (((match constructor.clone() {
                                        self::JavaScriptSemanticValue::Object(
                                            source_ml_source2_data_property_value33574,
                                            _,
                                        ) => Some((
                                            source_ml_source2_data_property_value33574.clone(),
                                        )),
                                        _ => None::<(self::ml::Big,)>,
                                    })
                                    .clone()
                                    .expect("a proven present value"))
                                    .0)
                                        .clone(),
                                );
                                if (match (fn_.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } || (((match fn_.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(
                                        _,
                                        _,
                                        source_ml_source2_data_property_value33576,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value33576.clone(),
                                }) != (self::ml_string::Text::from_units(vec![
                                    115, 116, 114, 105, 99, 116,
                                ])))
                                    && !(self::test_java_script_ordinary_class_constructor_mode(
                                        &(match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(
                                                _,
                                                _,
                                                source_ml_source2_data_property_value33577,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value33577.clone(),
                                        }),
                                    ))))
                                {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field11.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            115, 112, 101, 99, 105, 101, 115, 32, 98, 97, 115, 101,
                                            32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105,
                                            111, 110, 32, 117, 115, 101, 115, 32, 97, 99, 116, 117,
                                            97, 108, 32, 99, 104, 101, 99, 107, 101, 100, 32, 111,
                                            114, 100, 105, 110, 97, 114, 121, 32, 102, 117, 110,
                                            99, 116, 105, 111, 110, 47, 99, 108, 97, 115, 115, 32,
                                            99, 111, 100, 101,
                                        ]),
                                    )
                                } else {
                                    {
                                        let source_ml_source2_s33582 = if self::test_java_script_ordinary_class_constructor_mode(&(match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value33583, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value33583.clone()
                                            }
                                        })) {
                                            {
                                                let error = self::read_java_script_ordinary_class_receiver_error(shared_constructor_field11.clone(), constructor.clone(), receiver.clone());
                                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), error.clone())))
                                                } else {
                                                    self::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434Next
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434Next
                                        };
                                        match source_ml_source2_s33582.clone() {
                                            self::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434Next => {
                                                {
                                                    let slots = match fn_.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value33586, _, _, _, _) => {
                                                            source_ml_source2_data_property_value33586
                                                        }
                                                    };
                                                    {
                                                        let arguments_count = { let ml_reference_array2179 = arguments_.clone(); let ml_reference_projection2179 = ml_reference_array2179.borrow().len() as f64; ml_reference_projection2179 };
                                                        {
                                                            let source_ml_source1_temporary_values1433 = shared_constructor_field12.clone();
                                                            {
                                                                let source_ml_source1_temporary_index1433 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let source_ml_source2_s33588 = self::source_ml_source1_temporary_push_java_script_species_constructor_body_loop1435(slots, arguments_count, source_ml_source1_temporary_values1433.clone(), source_ml_source1_temporary_index1433.clone());
                                                                    match source_ml_source2_s33588.clone() {
                                                                        self::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyLoop1435Result::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyLoop1435Done(slots_2, arguments_count_2) => {
                                                                            if (((({ let ml_reference_array2180 = shared_constructor_field12.clone(); let ml_reference_projection2180 = ml_reference_array2180.borrow().len() as f64; ml_reference_projection2180 }) >= (1024f64)) || (slots_2 > (1048576f64))) || (arguments_count_2 > (1048576f64))) {
                                                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 97, 108, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 97, 99, 107, 32, 98, 117, 100, 103, 101, 116]))
                                                                            } else {
                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field11.clone()), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                    let source_ml_source2_concat_value33615 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_concat_value33616 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                                            let source_ml_source2_nominal_spread33593 = self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_.clone());
                                                                                            match source_ml_source2_nominal_spread33593.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                                                    let shared_constructor_field6 = *shared_constructor_field6;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field33594 = shared_constructor_field1.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field33596 = shared_constructor_field2;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field33598 = shared_constructor_field3.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field33600 = shared_constructor_field4.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field33602 = shared_constructor_field5.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field33604 = shared_constructor_field6.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field33606 = shared_constructor_field7.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field33608 = shared_constructor_field8.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field33610 = shared_constructor_field9.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field33612 = shared_constructor_field10.clone();
                                                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field33594.clone(), source_ml_source2_nominal_field33596, source_ml_source2_nominal_field33598.clone(), source_ml_source2_nominal_field33600.clone(), source_ml_source2_nominal_field33602.clone(), Box::new(source_ml_source2_nominal_field33604.clone()), source_ml_source2_nominal_field33606.clone(), source_ml_source2_nominal_field33608.clone(), source_ml_source2_nominal_field33610.clone(), Some((Some(constructor.clone()),)))
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
                                                                                        }]));
                                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), { let ml_reference_value2181 = source_ml_source2_concat_value33615.borrow().clone(); ml_reference_value2181 }), { let ml_reference_value2182 = source_ml_source2_concat_value33616.borrow().clone(); ml_reference_value2182 })
                                                                                    }
                                                                                })))))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434::MlSource1TemporaryPushJavaScriptSpeciesConstructorBodyJoin1434Return(source_ml_source1_temporary_result1435) => {
                                                let source_ml_source1_temporary_result1435 = *source_ml_source1_temporary_result1435;
                                                source_ml_source1_temporary_result1435.clone()
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
