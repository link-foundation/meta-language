// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayIterationTransition;
use super::JavaScriptArraySpreadTransition;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::accept_java_script_collection_iterator_result_completion;
use super::begin_java_script_array_iterator_result;
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
pub fn accept_java_script_call_array_iteration(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    transition: self::JavaScriptArrayIterationTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptArrayIterationTransition::ArrayIteratorAllocated(
                    source_ml_source2_data_property_value33617,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33617 =
                        *source_ml_source2_data_property_value33617;
                    source_ml_source2_data_property_value33617.clone()
                }
                self::JavaScriptArrayIterationTransition::ArrayIteratorContinue(
                    source_ml_source2_data_property_value33618,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33618 =
                        *source_ml_source2_data_property_value33618;
                    source_ml_source2_data_property_value33618.clone()
                }
                self::JavaScriptArrayIterationTransition::ArrayIteratorCall(
                    source_ml_source2_data_property_value33619,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33619 =
                        *source_ml_source2_data_property_value33619;
                    source_ml_source2_data_property_value33619.clone()
                }
                self::JavaScriptArrayIterationTransition::ArrayIteratorComponents(
                    source_ml_source2_data_property_value33620,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33620 =
                        *source_ml_source2_data_property_value33620;
                    source_ml_source2_data_property_value33620.clone()
                }
                self::JavaScriptArrayIterationTransition::ArrayIteratorAllocationResourceLimit(
                    source_ml_source2_data_property_value33621,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33621 =
                        *source_ml_source2_data_property_value33621;
                    source_ml_source2_data_property_value33621.clone()
                }
                self::JavaScriptArrayIterationTransition::ArrayIteratorNeedBoxing(
                    source_ml_source2_data_property_value33622,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33622 =
                        *source_ml_source2_data_property_value33622;
                    source_ml_source2_data_property_value33622.clone()
                }
                self::JavaScriptArrayIterationTransition::ArrayIteratorThrow(
                    source_ml_source2_data_property_value33623,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33623 =
                        *source_ml_source2_data_property_value33623;
                    source_ml_source2_data_property_value33623.clone()
                }
                self::JavaScriptArrayIterationTransition::ArrayIteratorError(
                    source_ml_source2_data_property_value33624,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33624 =
                        *source_ml_source2_data_property_value33624;
                    source_ml_source2_data_property_value33624.clone()
                }
                self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                    source_ml_source2_data_property_value33625,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33625 =
                        *source_ml_source2_data_property_value33625;
                    source_ml_source2_data_property_value33625.clone()
                }
                self::JavaScriptArrayIterationTransition::ArrayIteratorResourceLimit(
                    source_ml_source2_data_property_value33626,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33626 =
                        *source_ml_source2_data_property_value33626;
                    source_ml_source2_data_property_value33626.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value33627,
                ) => source_ml_source2_data_property_value33627.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                let frame = {
                    let source_ml_source2_array_read_value33630 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_array_read_index33631 = ({
                            let ml_reference_array2183 = shared_constructor_field1.clone();
                            let ml_reference_projection2183 =
                                ml_reference_array2183.borrow().len() as f64;
                            ml_reference_projection2183
                        } - 1f64);
                        {
                            let ml_reference_array2184 =
                                source_ml_source2_array_read_value33630.clone();
                            let ml_reference_position2184 = source_ml_source2_array_read_index33631;
                            let ml_reference_projection2184 = self::ml_array::optional_at(
                                &*ml_reference_array2184.borrow(),
                                self::ml_array::number_index(ml_reference_position2184),
                            );
                            ml_reference_projection2184
                        }
                    }
                };
                {
                    let fn_ = if match (frame.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        None::<self::JavaScriptCallFunction>
                    } else {
                        self::read_java_script_executable_call_function(program.clone(), match transition.clone() {
                                self::JavaScriptArrayIterationTransition::ArrayIteratorAllocated(source_ml_source2_data_property_value33632, _) => {
                                    let source_ml_source2_data_property_value33632 = *source_ml_source2_data_property_value33632;
                                    source_ml_source2_data_property_value33632.clone()
                                }
                                self::JavaScriptArrayIterationTransition::ArrayIteratorContinue(source_ml_source2_data_property_value33633, _) => {
                                    let source_ml_source2_data_property_value33633 = *source_ml_source2_data_property_value33633;
                                    source_ml_source2_data_property_value33633.clone()
                                }
                                self::JavaScriptArrayIterationTransition::ArrayIteratorCall(source_ml_source2_data_property_value33634, _, _, _, _) => {
                                    let source_ml_source2_data_property_value33634 = *source_ml_source2_data_property_value33634;
                                    source_ml_source2_data_property_value33634.clone()
                                }
                                self::JavaScriptArrayIterationTransition::ArrayIteratorComponents(source_ml_source2_data_property_value33635, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value33635 = *source_ml_source2_data_property_value33635;
                                    source_ml_source2_data_property_value33635.clone()
                                }
                                self::JavaScriptArrayIterationTransition::ArrayIteratorAllocationResourceLimit(source_ml_source2_data_property_value33636, _) => {
                                    let source_ml_source2_data_property_value33636 = *source_ml_source2_data_property_value33636;
                                    source_ml_source2_data_property_value33636.clone()
                                }
                                self::JavaScriptArrayIterationTransition::ArrayIteratorNeedBoxing(source_ml_source2_data_property_value33637, _) => {
                                    let source_ml_source2_data_property_value33637 = *source_ml_source2_data_property_value33637;
                                    source_ml_source2_data_property_value33637.clone()
                                }
                                self::JavaScriptArrayIterationTransition::ArrayIteratorThrow(source_ml_source2_data_property_value33638, _) => {
                                    let source_ml_source2_data_property_value33638 = *source_ml_source2_data_property_value33638;
                                    source_ml_source2_data_property_value33638.clone()
                                }
                                self::JavaScriptArrayIterationTransition::ArrayIteratorError(source_ml_source2_data_property_value33639, _, _) => {
                                    let source_ml_source2_data_property_value33639 = *source_ml_source2_data_property_value33639;
                                    source_ml_source2_data_property_value33639.clone()
                                }
                                self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(source_ml_source2_data_property_value33640, _) => {
                                    let source_ml_source2_data_property_value33640 = *source_ml_source2_data_property_value33640;
                                    source_ml_source2_data_property_value33640.clone()
                                }
                                self::JavaScriptArrayIterationTransition::ArrayIteratorResourceLimit(source_ml_source2_data_property_value33641, _, _) => {
                                    let source_ml_source2_data_property_value33641 = *source_ml_source2_data_property_value33641;
                                    source_ml_source2_data_property_value33641.clone()
                                }
                            }, match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value33642, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value33642.clone()
                                }
                            })
                    };
                    {
                        let instruction = if (match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || match (fn_.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) {
                            None::<self::JavaScriptCallInstruction>
                        } else {
                            {
                                let source_ml_source2_array_read_value33645 =
                                    match fn_.clone().expect("a proven present value") {
                                        self::JavaScriptCallFunction::CallFunction(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value33643,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value33643.clone(),
                                    };
                                {
                                    let source_ml_source2_array_read_index33646 =
                                        match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(
                                                _,
                                                source_ml_source2_data_property_value33644,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value33644,
                                        };
                                    {
                                        let ml_reference_array2185 =
                                            source_ml_source2_array_read_value33645.clone();
                                        let ml_reference_position2185 =
                                            source_ml_source2_array_read_index33646;
                                        let ml_reference_projection2185 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array2185.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position2185,
                                                ),
                                            );
                                        ml_reference_projection2185
                                    }
                                }
                            }
                        };
                        if ((((match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || match (fn_.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) || match (instruction.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) || match instruction.clone().expect("a proven present value") {
                            self::JavaScriptCallInstruction::CollectionIteratorResult(
                                _,
                                _,
                                _,
                                _,
                            ) => false,
                            _ => true,
                        })
                            || (((((match instruction.clone().expect("a proven present value") {
                                self::JavaScriptCallInstruction::CollectionIteratorResult(
                                    _,
                                    source_ml_source2_data_property_value33652,
                                    _,
                                    _,
                                ) => Some((source_ml_source2_data_property_value33652.clone(),)),
                                _ => None::<(self::ml_string::Text,)>,
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0)
                                .clone())
                                != (self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121,
                                ]))))
                        {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorAllocated(source_ml_source2_data_property_value33653, _) => {
                                        let source_ml_source2_data_property_value33653 = *source_ml_source2_data_property_value33653;
                                        source_ml_source2_data_property_value33653.clone()
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorContinue(source_ml_source2_data_property_value33654, _) => {
                                        let source_ml_source2_data_property_value33654 = *source_ml_source2_data_property_value33654;
                                        source_ml_source2_data_property_value33654.clone()
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorCall(source_ml_source2_data_property_value33655, _, _, _, _) => {
                                        let source_ml_source2_data_property_value33655 = *source_ml_source2_data_property_value33655;
                                        source_ml_source2_data_property_value33655.clone()
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorComponents(source_ml_source2_data_property_value33656, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value33656 = *source_ml_source2_data_property_value33656;
                                        source_ml_source2_data_property_value33656.clone()
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorAllocationResourceLimit(source_ml_source2_data_property_value33657, _) => {
                                        let source_ml_source2_data_property_value33657 = *source_ml_source2_data_property_value33657;
                                        source_ml_source2_data_property_value33657.clone()
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorNeedBoxing(source_ml_source2_data_property_value33658, _) => {
                                        let source_ml_source2_data_property_value33658 = *source_ml_source2_data_property_value33658;
                                        source_ml_source2_data_property_value33658.clone()
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorThrow(source_ml_source2_data_property_value33659, _) => {
                                        let source_ml_source2_data_property_value33659 = *source_ml_source2_data_property_value33659;
                                        source_ml_source2_data_property_value33659.clone()
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorError(source_ml_source2_data_property_value33660, _, _) => {
                                        let source_ml_source2_data_property_value33660 = *source_ml_source2_data_property_value33660;
                                        source_ml_source2_data_property_value33660.clone()
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(source_ml_source2_data_property_value33661, _) => {
                                        let source_ml_source2_data_property_value33661 = *source_ml_source2_data_property_value33661;
                                        source_ml_source2_data_property_value33661.clone()
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorResourceLimit(source_ml_source2_data_property_value33662, _, _) => {
                                        let source_ml_source2_data_property_value33662 = *source_ml_source2_data_property_value33662;
                                        source_ml_source2_data_property_value33662.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        } else {
                            match transition.clone() {
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorAllocated(heap, _) => {
                                        let heap = *heap;
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 100, 118, 97, 110, 99, 101, 109, 101, 110, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorContinue(heap_2, state) => {
                                        let heap_2 = *heap_2;
                                        let state = *state;
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayIteratorNext(((match instruction.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(source_ml_source2_data_property_value33669, _, _, _) => {
                                                Some((source_ml_source2_data_property_value33669,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0, match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value33670, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value33670.clone()
                                            }
                                        }, Box::new(state.clone()))))))
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorCall(heap_3, state_2, callee, receiver, arguments) => {
                                        let heap_3 = *heap_3;
                                        let state_2 = *state_2;
                                        let callee = *callee;
                                        let receiver = *receiver;
                                        self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayIteratorNext(((match instruction.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(source_ml_source2_data_property_value33672, _, _, _) => {
                                                Some((source_ml_source2_data_property_value33672,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0, match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value33673, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value33673.clone()
                                            }
                                        }, Box::new(state_2.clone())))), callee.clone(), receiver.clone(), arguments.clone())
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorComponents(heap_4, receiver_2, done, key, value, entry) => {
                                        let heap_4 = *heap_4;
                                        let receiver_2 = *receiver_2;
                                        let key = *key;
                                        let value = *value;
                                        self::accept_java_script_collection_iterator_result_completion(program.clone(), current.clone(), frame.clone().expect("a proven present value"), ((match instruction.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(source_ml_source2_data_property_value33674, _, _, _) => {
                                                Some((source_ml_source2_data_property_value33674,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0, match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value33675, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value33675.clone()
                                            }
                                        }, self::begin_java_script_array_iterator_result(heap_4.clone(), receiver_2.clone(), done, key.clone(), value.clone(), entry, ({
                                            let source_ml_source2_array_read_value33678 = match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value33676, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value33676.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_array_read_index33679 = ((match instruction.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, source_ml_source2_data_property_value33677, _) => {
                                                        Some((source_ml_source2_data_property_value33677,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0;
                                                { let ml_reference_array2186 = source_ml_source2_array_read_value33678.clone(); let ml_reference_position2186 = source_ml_source2_array_read_index33679; let ml_reference_projection2186 = self::ml_array::optional_at(&*ml_reference_array2186.borrow(), self::ml_array::number_index(ml_reference_position2186)); ml_reference_projection2186 }
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                            let source_ml_source2_array_read_value33682 = match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value33680, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value33680.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_array_read_index33683 = ((match instruction.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, source_ml_source2_data_property_value33681) => {
                                                        Some((source_ml_source2_data_property_value33681,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0;
                                                { let ml_reference_array2187 = source_ml_source2_array_read_value33682.clone(); let ml_reference_position2187 = source_ml_source2_array_read_index33683; let ml_reference_projection2187 = self::ml_array::optional_at(&*ml_reference_array2187.borrow(), self::ml_array::number_index(ml_reference_position2187)); ml_reference_projection2187 }
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorAllocationResourceLimit(heap_5, _) => {
                                        let heap_5 = *heap_5;
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_5.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 100, 118, 97, 110, 99, 101, 109, 101, 110, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorNeedBoxing(heap_6, _) => {
                                        let heap_6 = *heap_6;
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_6.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 100, 118, 97, 110, 99, 101, 109, 101, 110, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorThrow(_, value_2) => {
                                        let value_2 = *value_2;
                                        self::throw_java_script_call_value(current.clone(), value_2.clone())
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorError(_, error_class, reason) => {
                                        self::request_java_script_call_error(current.clone(), match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value33684, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value33684.clone()
                                            }
                                        }, &(error_class), &(reason))
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(heap_7, reason_2) => {
                                        let heap_7 = *heap_7;
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_7.clone()), reason_2.clone())
                                    }
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorResourceLimit(heap_8, state_3, reason_3) => {
                                        let heap_8 = *heap_8;
                                        let state_3 = *state_3;
                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(heap_8.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayIteratorNext(((match instruction.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(source_ml_source2_data_property_value33686, _, _, _) => {
                                                Some((source_ml_source2_data_property_value33686,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0, match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value33687, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value33687.clone()
                                            }
                                        }, Box::new(state_3.clone()))))), reason_3.clone())
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
pub fn accept_java_script_call_array_spread(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptArraySpreadTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptArraySpreadTransition::ArraySpreadContinue(
                    source_ml_source2_data_property_value33688,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33688 =
                        *source_ml_source2_data_property_value33688;
                    source_ml_source2_data_property_value33688.clone()
                }
                self::JavaScriptArraySpreadTransition::ArraySpreadCall(
                    source_ml_source2_data_property_value33689,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33689 =
                        *source_ml_source2_data_property_value33689;
                    source_ml_source2_data_property_value33689.clone()
                }
                self::JavaScriptArraySpreadTransition::ArraySpreadPrimitiveGet(
                    source_ml_source2_data_property_value33690,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33690 =
                        *source_ml_source2_data_property_value33690;
                    source_ml_source2_data_property_value33690.clone()
                }
                self::JavaScriptArraySpreadTransition::ArraySpreadCompleted(
                    source_ml_source2_data_property_value33691,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33691 =
                        *source_ml_source2_data_property_value33691;
                    source_ml_source2_data_property_value33691.clone()
                }
                self::JavaScriptArraySpreadTransition::ArraySpreadThrow(
                    source_ml_source2_data_property_value33692,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33692 =
                        *source_ml_source2_data_property_value33692;
                    source_ml_source2_data_property_value33692.clone()
                }
                self::JavaScriptArraySpreadTransition::ArraySpreadError(
                    source_ml_source2_data_property_value33693,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33693 =
                        *source_ml_source2_data_property_value33693;
                    source_ml_source2_data_property_value33693.clone()
                }
                self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                    source_ml_source2_data_property_value33694,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33694 =
                        *source_ml_source2_data_property_value33694;
                    source_ml_source2_data_property_value33694.clone()
                }
                self::JavaScriptArraySpreadTransition::ArraySpreadResourceLimit(
                    source_ml_source2_data_property_value33695,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value33695 =
                        *source_ml_source2_data_property_value33695;
                    source_ml_source2_data_property_value33695.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value33696,
                ) => source_ml_source2_data_property_value33696.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                let frame = {
                    let source_ml_source2_array_read_value33699 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_array_read_index33700 = ({
                            let ml_reference_array2188 = shared_constructor_field1.clone();
                            let ml_reference_projection2188 =
                                ml_reference_array2188.borrow().len() as f64;
                            ml_reference_projection2188
                        } - 1f64);
                        {
                            let ml_reference_array2189 =
                                source_ml_source2_array_read_value33699.clone();
                            let ml_reference_position2189 = source_ml_source2_array_read_index33700;
                            let ml_reference_projection2189 = self::ml_array::optional_at(
                                &*ml_reference_array2189.borrow(),
                                self::ml_array::number_index(ml_reference_position2189),
                            );
                            ml_reference_projection2189
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
                            self::JavaScriptArraySpreadTransition::ArraySpreadContinue(
                                source_ml_source2_data_property_value33701,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33701 =
                                    *source_ml_source2_data_property_value33701;
                                source_ml_source2_data_property_value33701.clone()
                            }
                            self::JavaScriptArraySpreadTransition::ArraySpreadCall(
                                source_ml_source2_data_property_value33702,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33702 =
                                    *source_ml_source2_data_property_value33702;
                                source_ml_source2_data_property_value33702.clone()
                            }
                            self::JavaScriptArraySpreadTransition::ArraySpreadPrimitiveGet(
                                source_ml_source2_data_property_value33703,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33703 =
                                    *source_ml_source2_data_property_value33703;
                                source_ml_source2_data_property_value33703.clone()
                            }
                            self::JavaScriptArraySpreadTransition::ArraySpreadCompleted(
                                source_ml_source2_data_property_value33704,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33704 =
                                    *source_ml_source2_data_property_value33704;
                                source_ml_source2_data_property_value33704.clone()
                            }
                            self::JavaScriptArraySpreadTransition::ArraySpreadThrow(
                                source_ml_source2_data_property_value33705,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33705 =
                                    *source_ml_source2_data_property_value33705;
                                source_ml_source2_data_property_value33705.clone()
                            }
                            self::JavaScriptArraySpreadTransition::ArraySpreadError(
                                source_ml_source2_data_property_value33706,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33706 =
                                    *source_ml_source2_data_property_value33706;
                                source_ml_source2_data_property_value33706.clone()
                            }
                            self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                                source_ml_source2_data_property_value33707,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33707 =
                                    *source_ml_source2_data_property_value33707;
                                source_ml_source2_data_property_value33707.clone()
                            }
                            self::JavaScriptArraySpreadTransition::ArraySpreadResourceLimit(
                                source_ml_source2_data_property_value33708,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value33708 =
                                    *source_ml_source2_data_property_value33708;
                                source_ml_source2_data_property_value33708.clone()
                            }
                        }),
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101,
                            116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                            108, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101,
                            32, 102, 114, 97, 109, 101,
                        ]),
                    )
                } else {
                    match transition.clone() {
                        self::JavaScriptArraySpreadTransition::ArraySpreadContinue(heap, state) => {
                            let heap = *heap;
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArraySpread(
                                            destination,
                                            realm.clone(),
                                            Box::new(state.clone()),
                                        ),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptArraySpreadTransition::ArraySpreadCall(
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
                                        self::JavaScriptCallPending::WaitArraySpread(
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
                        self::JavaScriptArraySpreadTransition::ArraySpreadPrimitiveGet(
                            heap_3,
                            state_3,
                            receiver_2,
                            key,
                        ) => {
                            let heap_3 = *heap_3;
                            let state_3 = *state_3;
                            let receiver_2 = *receiver_2;
                            let key = *key;
                            self::JavaScriptCallMachineTransition::MachineNeedPrimitiveIteratorGet(
                                Box::new(self::replace_java_script_call_frame(
                                    heap_3.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArraySpread(
                                            destination,
                                            realm.clone(),
                                            Box::new(state_3.clone()),
                                        ),
                                    ),
                                )),
                                Box::new(receiver_2.clone()),
                                Box::new(key.clone()),
                            )
                        }
                        self::JavaScriptArraySpreadTransition::ArraySpreadCompleted(
                            heap_4,
                            index,
                        ) => {
                            let heap_4 = *heap_4;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap_4.clone(),
                                    shared_constructor_field1.clone(),
                                    self::write_java_script_call_register(
                                        frame.clone().expect("a proven present value"),
                                        destination,
                                        self::JavaScriptSemanticValue::Number(index),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptArraySpreadTransition::ArraySpreadThrow(_, value) => {
                            let value = *value;
                            self::throw_java_script_call_value(current.clone(), value.clone())
                        }
                        self::JavaScriptArraySpreadTransition::ArraySpreadError(
                            _,
                            error_class,
                            reason,
                        ) => self::request_java_script_call_error(
                            current.clone(),
                            realm.clone(),
                            &(error_class),
                            &(reason),
                        ),
                        self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                            heap_5,
                            reason_2,
                        ) => {
                            let heap_5 = *heap_5;
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(heap_5.clone()),
                                reason_2.clone(),
                            )
                        }
                        self::JavaScriptArraySpreadTransition::ArraySpreadResourceLimit(
                            heap_6,
                            state_4,
                            reason_3,
                        ) => {
                            let heap_6 = *heap_6;
                            let state_4 = *state_4;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(
                                Box::new(self::replace_java_script_call_frame(
                                    heap_6.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArraySpread(
                                            destination,
                                            realm.clone(),
                                            Box::new(state_4.clone()),
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
