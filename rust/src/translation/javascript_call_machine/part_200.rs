// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationFallback;
use super::JavaScriptArrayCreationState;
use super::JavaScriptCallCompletion;
use super::JavaScriptCallCompletionTransition;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionState;
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorRegionsUpdate;
use super::JavaScriptObjectFromEntriesCall;
use super::JavaScriptObjectFromEntriesErrorRequest;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryPushJavaScriptCallLoop1502Result;
use super::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499;
use super::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result;
use super::create_java_script_call_frame;
use super::finish_java_script_array_iterator_cursor;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_call_constructor_failure;
use super::read_java_script_executable_call_function;
use super::read_java_script_ordinary_value_error;
use super::remove_java_script_iterator_region;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::source_ml_source1_temporary_push_java_script_call_loop1502;
use super::source_ml_source1_temporary_unwind_java_script_call_completion_loop1496;
use super::test_java_script_ordinary_class_constructor_mode;
use super::update_java_script_iterator_region;
use super::write_java_script_call_iterator_result;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn unwind_java_script_call_completion(
    machine: self::JavaScriptCallMachine,
    completion: self::JavaScriptCallCompletion,
) -> self::JavaScriptCallCompletionTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field3,
            shared_constructor_field4,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            if (((((((!(self::ml_array::includes(
                &{
                    let ml_reference_value2281 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]),
                        self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]),
                        self::ml_string::Text::from_units(vec![106, 117, 109, 112]),
                    ])))
                    .borrow()
                    .clone();
                    ml_reference_value2281
                },
                &match completion.clone() {
                    self::JavaScriptCallCompletion::CallCompletion(
                        source_ml_source2_data_property_value35419,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value35419.clone(),
                },
                0f64,
                false,
                |a, b| a == b,
            )) || ((self::read_java_script_ordinary_value_error(
                shared_constructor_field3.clone(),
                match completion.clone() {
                    self::JavaScriptCallCompletion::CallCompletion(
                        _,
                        source_ml_source2_data_property_value35423,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value35423 =
                            *source_ml_source2_data_property_value35423;
                        source_ml_source2_data_property_value35423.clone()
                    }
                },
            )) != (self::ml_string::Text::from_units(vec![]))))
                || !(self::ml_math::is_safe_integer(match completion.clone() {
                    self::JavaScriptCallCompletion::CallCompletion(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value35424,
                    ) => source_ml_source2_data_property_value35424,
                })))
                || {
                    let source_ml_source2_same_value_operand35426 = match completion.clone() {
                        self::JavaScriptCallCompletion::CallCompletion(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value35425,
                        ) => source_ml_source2_data_property_value35425,
                    };
                    {
                        let source_ml_source2_same_value_operand35427 = (-0f64);
                        (((source_ml_source2_same_value_operand35426
                            == source_ml_source2_same_value_operand35427)
                            && ((source_ml_source2_same_value_operand35426 != (0f64))
                                || ((1f64 / source_ml_source2_same_value_operand35426)
                                    == (1f64 / source_ml_source2_same_value_operand35427))))
                            || ((source_ml_source2_same_value_operand35426
                                != source_ml_source2_same_value_operand35426)
                                && (source_ml_source2_same_value_operand35427
                                    != source_ml_source2_same_value_operand35427)))
                    }
                })
                || ((match completion.clone() {
                    self::JavaScriptCallCompletion::CallCompletion(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value35429,
                    ) => source_ml_source2_data_property_value35429,
                }) < (0f64)))
                || ((match completion.clone() {
                    self::JavaScriptCallCompletion::CallCompletion(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value35431,
                    ) => source_ml_source2_data_property_value35431,
                }) > (1024f64)))
                || (((match completion.clone() {
                    self::JavaScriptCallCompletion::CallCompletion(
                        source_ml_source2_data_property_value35433,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value35433.clone(),
                }) == (self::ml_string::Text::from_units(vec![106, 117, 109, 112])))
                    && (((!(self::ml_math::is_safe_integer(match completion.clone() {
                        self::JavaScriptCallCompletion::CallCompletion(
                            _,
                            _,
                            source_ml_source2_data_property_value35434,
                            _,
                        ) => source_ml_source2_data_property_value35434,
                    })) || ((match completion.clone() {
                        self::JavaScriptCallCompletion::CallCompletion(
                            _,
                            _,
                            source_ml_source2_data_property_value35436,
                            _,
                        ) => source_ml_source2_data_property_value35436,
                    }) < (0f64)))
                        || ((match completion.clone() {
                            self::JavaScriptCallCompletion::CallCompletion(
                                _,
                                _,
                                source_ml_source2_data_property_value35438,
                                _,
                            ) => source_ml_source2_data_property_value35438,
                        }) >= (65536f64)))
                        || {
                            let source_ml_source2_same_value_operand35440 = match completion.clone()
                            {
                                self::JavaScriptCallCompletion::CallCompletion(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value35439,
                                    _,
                                ) => source_ml_source2_data_property_value35439,
                            };
                            {
                                let source_ml_source2_same_value_operand35441 = (-0f64);
                                (((source_ml_source2_same_value_operand35440
                                    == source_ml_source2_same_value_operand35441)
                                    && ((source_ml_source2_same_value_operand35440 != (0f64))
                                        || ((1f64 / source_ml_source2_same_value_operand35440)
                                            == (1f64
                                                / source_ml_source2_same_value_operand35441))))
                                    || ((source_ml_source2_same_value_operand35440
                                        != source_ml_source2_same_value_operand35440)
                                        && (source_ml_source2_same_value_operand35441
                                            != source_ml_source2_same_value_operand35441)))
                            }
                        })))
                || (((match completion.clone() {
                    self::JavaScriptCallCompletion::CallCompletion(
                        source_ml_source2_data_property_value35443,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value35443.clone(),
                }) != (self::ml_string::Text::from_units(vec![106, 117, 109, 112])))
                    && (((match completion.clone() {
                        self::JavaScriptCallCompletion::CallCompletion(
                            _,
                            _,
                            source_ml_source2_data_property_value35445,
                            _,
                        ) => source_ml_source2_data_property_value35445,
                    }) != (-1f64))
                        || ((match completion.clone() {
                            self::JavaScriptCallCompletion::CallCompletion(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value35447,
                            ) => source_ml_source2_data_property_value35447,
                        }) != (0f64)))))
            {
                self::JavaScriptCallCompletionTransition::MachineInvalid(
                    Box::new(shared_constructor_field3.clone()),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 117, 110, 119, 105,
                        110, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                        97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 115, 32, 97, 110,
                        100, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32,
                        99, 111, 110, 116, 114, 111, 108, 32, 99, 111, 111, 114, 100, 105, 110, 97,
                        116, 101, 115,
                    ]),
                )
            } else {
                {
                    let frames = shared_constructor_field4.clone();
                    {
                        let heap = shared_constructor_field3.clone();
                        {
                            let source_ml_source2_s35451 = self::source_ml_source1_temporary_unwind_java_script_call_completion_loop1496(completion.clone(), frames.clone(), heap.clone());
                            match source_ml_source2_s35451.clone() {
                                self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Done(completion_2, heap_2) => {
                                    let completion_2 = *completion_2;
                                    let heap_2 = *heap_2;
                                    match completion_2.clone() {
                                        self::JavaScriptCallCompletion::CallCompletion(shared_constructor_field1, shared_constructor_field2, _, _) => {
                                            let shared_constructor_field2 = *shared_constructor_field2;
                                            if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]))) {
                                                self::JavaScriptCallCompletionTransition::MachineThrow(Box::new(heap_2.clone()), Box::new(shared_constructor_field2.clone()))
                                            } else {
                                                self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_2.clone()), self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110, 47, 99, 111, 110, 116, 114, 111, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Return(source_ml_source1_temporary_result1496) => {
                                    let source_ml_source1_temporary_result1496 = *source_ml_source1_temporary_result1496;
                                    source_ml_source1_temporary_result1496.clone()
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
pub fn request_java_script_call_error(
    machine: self::JavaScriptCallMachine,
    realm: self::ml::Big,
    error_class: &self::ml_string::Text,
    reason: &self::ml_string::Text,
) -> self::JavaScriptCallMachineTransition {
    {
        let frame = {
            let source_ml_source2_array_read_value35457 = match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value35455,
                ) => source_ml_source2_data_property_value35455.clone(),
            };
            {
                let source_ml_source2_array_read_index35458 = ({
                    let ml_reference_array2282 = match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(
                            _,
                            source_ml_source2_data_property_value35456,
                        ) => source_ml_source2_data_property_value35456.clone(),
                    };
                    let ml_reference_projection2282 = ml_reference_array2282.borrow().len() as f64;
                    ml_reference_projection2282
                } - 1f64);
                {
                    let ml_reference_array2283 = source_ml_source2_array_read_value35457.clone();
                    let ml_reference_position2283 = source_ml_source2_array_read_index35458;
                    let ml_reference_projection2283 = self::ml_array::optional_at(
                        &*ml_reference_array2283.borrow(),
                        self::ml_array::number_index(ml_reference_position2283),
                    );
                    ml_reference_projection2283
                }
            }
        };
        if match (frame.clone(), None::<()>) {
            (Some(_ml_left), Some(_ml_right)) => false,
            (None, None) => true,
            _ => false,
        } {
            self::JavaScriptCallMachineTransition::MachineInvalid(
                Box::new(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(
                        source_ml_source2_data_property_value35459,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value35459 =
                            *source_ml_source2_data_property_value35459;
                        source_ml_source2_data_property_value35459.clone()
                    }
                }),
                self::ml_string::Text::from_units(vec![
                    69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32,
                    114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                    117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101,
                ]),
            )
        } else {
            {
                let initial_pending = match frame.clone().expect("a proven present value") {
                    self::JavaScriptCallFrame::CallFrame(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value35460,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value35460 =
                            *source_ml_source2_data_property_value35460;
                        source_ml_source2_data_property_value35460.clone()
                    }
                };
                {
                    let source_ml_source2_s35527 = match initial_pending.clone() {
                        self::JavaScriptCallPending::WaitIteratorProtocol(destination, done, slot, operation, _, state) => {
                            let state = *state;
                            match state.clone() {
                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, shared_constructor_field11, _, shared_constructor_field12, _, _) => {
                                    let shared_constructor_field11 = *shared_constructor_field11;
                                    let shared_constructor_field12 = *shared_constructor_field12;
                                    {
                                        let regions = (match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value35531, _, _, _) => {
                                                ((source_ml_source2_data_property_value35531.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new())));
                                        {
                                            let update = if (operation == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                                                self::remove_java_script_iterator_region(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value35532, _) => {
                                                        let source_ml_source2_data_property_value35532 = *source_ml_source2_data_property_value35532;
                                                        source_ml_source2_data_property_value35532.clone()
                                                    }
                                                }, regions.clone(), slot)
                                            } else {
                                                if (operation == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) {
                                                    self::update_java_script_iterator_region(match machine.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value35533, _) => {
                                                            let source_ml_source2_data_property_value35533 = *source_ml_source2_data_property_value35533;
                                                            source_ml_source2_data_property_value35533.clone()
                                                        }
                                                    }, regions.clone(), slot, {
                                                        let source_ml_source2_nominal_spread35535 = shared_constructor_field11.clone();
                                                        match source_ml_source2_nominal_spread35535.clone() {
                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                {
                                                                    let source_ml_source2_nominal_field35536 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field35538 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field35540 = shared_constructor_field3;
                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(source_ml_source2_nominal_field35536.clone()), Box::new(source_ml_source2_nominal_field35538.clone()), true)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    })
                                                } else {
                                                    self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions.clone())
                                                }
                                            };
                                            match update.clone() {
                                                self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_2) => {
                                                    {
                                                        let frame_2 = {
                                                            let source_ml_source2_nominal_spread35543 = frame.clone().expect("a proven present value");
                                                            match source_ml_source2_nominal_spread35543.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                    let shared_constructor_field6 = *shared_constructor_field6;
                                                                    {
                                                                        let source_ml_source2_nominal_field35544 = shared_constructor_field1_2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field35546 = shared_constructor_field2_2;
                                                                            {
                                                                                let source_ml_source2_nominal_field35548 = shared_constructor_field3_2.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field35550 = shared_constructor_field4.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field35552 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field35554 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field35556 = shared_constructor_field7.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field35558 = shared_constructor_field8.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field35560 = shared_constructor_field9.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field35562 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override35565 = self::JavaScriptCallPending::CallIdle;
                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field35544.clone(), source_ml_source2_nominal_field35546, source_ml_source2_nominal_field35548.clone(), source_ml_source2_nominal_field35550.clone(), source_ml_source2_nominal_field35552.clone(), Box::new(source_ml_source2_nominal_override35565.clone()), Some((Some(regions_2.clone()),)), source_ml_source2_nominal_field35558.clone(), source_ml_source2_nominal_field35560.clone(), source_ml_source2_nominal_field35562.clone())
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
                                                        };
                                                        {
                                                            let machine_2 = self::replace_java_script_call_frame(match machine.clone() {
                                                                self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value35566, _) => {
                                                                    let source_ml_source2_data_property_value35566 = *source_ml_source2_data_property_value35566;
                                                                    source_ml_source2_data_property_value35566.clone()
                                                                }
                                                            }, match machine.clone() {
                                                                self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value35567) => {
                                                                    source_ml_source2_data_property_value35567.clone()
                                                                }
                                                            }, frame_2.clone());
                                                            match machine_2.clone() {
                                                                self::JavaScriptCallMachine::CallMachine(shared_constructor_field1_3, shared_constructor_field2_3) => {
                                                                    let shared_constructor_field1_3 = *shared_constructor_field1_3;
                                                                    if ((operation == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) && ((match shared_constructor_field12.clone() {
                                                                        self::JavaScriptIteratorCompletion::IteratorCompletion(source_ml_source2_data_property_value35571, _, _) => {
                                                                            source_ml_source2_data_property_value35571.clone()
                                                                        }
                                                                    }) == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119])))) {
                                                                        self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Return(Box::new(self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1_3.clone(), shared_constructor_field2_3.clone(), self::write_java_script_call_iterator_result(frame_2.clone(), destination, done, self::JavaScriptSemanticValue::Undefined, true))))))
                                                                    } else {
                                                                        self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Next(Box::new(machine_2.clone()), Box::new(frame_2.clone()))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match machine.clone() {
                                                        self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value35574, _) => {
                                                            let source_ml_source2_data_property_value35574 = *source_ml_source2_data_property_value35574;
                                                            source_ml_source2_data_property_value35574.clone()
                                                        }
                                                    }), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 97, 105, 108, 117, 114, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 103, 105, 111, 110]))))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallPending::WaitArrayCreation(destination_2, initial_pending_realm, state_2, construction, initial_pending_fallback, _) => {
                            let state_2 = *state_2;
                            if (!(match (((construction.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match (match state_2.clone() {
                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35575, _, _, _) => {
                                    ((source_ml_source2_data_property_value35575.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,))).0).clone()
                                }
                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Next(Box::new(machine.clone()), Box::new(frame.clone().expect("a proven present value")))
                            } else {
                                {
                                    let fallback = ((initial_pending_fallback.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationFallback>,))).0).clone();
                                    {
                                        let error = self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(realm.clone(), error_class.clone(), reason.clone());
                                        {
                                            let pending = if match (fallback.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                {
                                                    let source_ml_source2_constructor_field35579 = Some(error.clone());
                                                    self::JavaScriptCallPending::WaitArrayCreation(destination_2, initial_pending_realm.clone(), Box::new(state_2.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, Some((source_ml_source2_constructor_field35579.clone(),)))
                                                }
                                            } else {
                                                {
                                                    let source_ml_source2_constructor_field35583 = Some(error.clone());
                                                    {
                                                        let source_ml_source2_constructor_field35584 = Some(fallback.clone().expect("a proven present value"));
                                                        self::JavaScriptCallPending::WaitArrayCreation(destination_2, initial_pending_realm.clone(), Box::new(state_2.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, Some((source_ml_source2_constructor_field35584.clone(),)), Some((source_ml_source2_constructor_field35583.clone(),)))
                                                    }
                                                }
                                            };
                                            {
                                                let next = self::replace_java_script_call_frame(match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value35585, _) => {
                                                        let source_ml_source2_data_property_value35585 = *source_ml_source2_data_property_value35585;
                                                        source_ml_source2_data_property_value35585.clone()
                                                    }
                                                }, match machine.clone() {
                                                    self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value35586) => {
                                                        source_ml_source2_data_property_value35586.clone()
                                                    }
                                                }, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), pending.clone()));
                                                self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Return(Box::new(self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(next.clone()), realm.clone(), error_class.clone(), reason.clone())))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallPending::WaitObjectFromEntries(destination_3, initial_pending_realm_2, state_3, _) => {
                            let state_3 = *state_3;
                            if match (match state_3.clone() {
                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35587, _, _, _) => {
                                    ((source_ml_source2_data_property_value35587.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                                }
                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Next(Box::new(machine.clone()), Box::new(frame.clone().expect("a proven present value")))
                            } else {
                                {
                                    let pending_2 = {
                                        let source_ml_source2_constructor_field35591 = Some(self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(realm.clone(), error_class.clone(), reason.clone()));
                                        self::JavaScriptCallPending::WaitObjectFromEntries(destination_3, initial_pending_realm_2.clone(), Box::new(state_3.clone()), Some((source_ml_source2_constructor_field35591.clone(),)))
                                    };
                                    {
                                        let next_2 = self::replace_java_script_call_frame(match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value35592, _) => {
                                                let source_ml_source2_data_property_value35592 = *source_ml_source2_data_property_value35592;
                                                source_ml_source2_data_property_value35592.clone()
                                            }
                                        }, match machine.clone() {
                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value35593) => {
                                                source_ml_source2_data_property_value35593.clone()
                                            }
                                        }, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), pending_2.clone()));
                                        self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Return(Box::new(self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(next_2.clone()), realm.clone(), error_class.clone(), reason.clone())))
                                    }
                                }
                            }
                        }
                        _ => {
                            self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Next(Box::new(machine.clone()), Box::new(frame.clone().expect("a proven present value")))
                        }
                    };
                    match source_ml_source2_s35527.clone() {
                        self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Next(machine_3, frame_3) => {
                            let machine_3 = *machine_3;
                            let frame_3 = *frame_3;
                            match machine_3.clone() {
                                self::JavaScriptCallMachine::CallMachine(shared_constructor_field1_4, shared_constructor_field2_4) => {
                                    let shared_constructor_field1_4 = *shared_constructor_field1_4;
                                    {
                                        let current_pending = match frame_3.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value35594, _, _, _, _) => {
                                                let source_ml_source2_data_property_value35594 = *source_ml_source2_data_property_value35594;
                                                source_ml_source2_data_property_value35594.clone()
                                            }
                                        };
                                        {
                                            let heap = match current_pending.clone() {
                                                self::JavaScriptCallPending::WaitArrayIteratorNext(_, _, state_4) => {
                                                    let state_4 = *state_4;
                                                    self::finish_java_script_array_iterator_cursor(shared_constructor_field1_4.clone(), state_4.clone(), true)
                                                }
                                                _ => {
                                                    shared_constructor_field1_4.clone()
                                                }
                                            };
                                            {
                                                let next_3 = self::replace_java_script_call_frame(heap.clone(), shared_constructor_field2_4.clone(), self::replace_java_script_call_pending(frame_3.clone(), self::JavaScriptCallPending::WaitError(realm.clone(), error_class.clone(), reason.clone())));
                                                self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(next_3.clone()), realm.clone(), error_class.clone(), reason.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499::MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Return(source_ml_source1_temporary_result1499) => {
                            let source_ml_source1_temporary_result1499 = *source_ml_source1_temporary_result1499;
                            source_ml_source1_temporary_result1499.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_call(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value35602 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index35603 = ({
                            let ml_reference_array2284 = shared_constructor_field2.clone();
                            let ml_reference_projection2284 =
                                ml_reference_array2284.borrow().len() as f64;
                            ml_reference_projection2284
                        } - 1f64);
                        {
                            let ml_reference_array2285 =
                                source_ml_source2_array_read_value35602.clone();
                            let ml_reference_position2285 = source_ml_source2_array_read_index35603;
                            let ml_reference_projection2285 = self::ml_array::optional_at(
                                &*ml_reference_array2285.borrow(),
                                self::ml_array::number_index(ml_reference_position2285),
                            );
                            ml_reference_projection2285
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
                            97, 32, 110, 101, 115, 116, 101, 100, 32, 99, 97, 108, 108, 32, 110,
                            101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                            32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101,
                            110, 116,
                        ]),
                    )
                } else {
                    {
                        let owner = self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field1.clone(),
                            match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    source_ml_source2_data_property_value35606,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value35606.clone(),
                            },
                        );
                        if match (owner.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 110,
                                    101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105,
                                    115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101,
                                    32, 111, 119, 110, 101, 114,
                                ]),
                            )
                        } else {
                            match callee.clone() {
                                self::JavaScriptSemanticValue::Object(identity, callable) => {
                                    if !callable {
                                        self::request_java_script_call_error(
                                            machine.clone(),
                                            match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(
                                                    _,
                                                    source_ml_source2_data_property_value35608,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source2_data_property_value35608
                                                    .clone(),
                                            },
                                            &(self::ml_string::Text::from_units(vec![
                                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                                            ])),
                                            &(self::ml_string::Text::from_units(vec![
                                                116, 104, 101, 32, 115, 101, 108, 101, 99, 116,
                                                101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105,
                                                115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97,
                                                98, 108, 101,
                                            ])),
                                        )
                                    } else {
                                        {
                                            let fn_ =
                                                self::read_java_script_executable_call_function(
                                                    program.clone(),
                                                    shared_constructor_field1.clone(),
                                                    identity.clone(),
                                                );
                                            if match (fn_.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                                            } else {
                                                {
                                                    let setup = self::read_java_script_call_constructor_failure(shared_constructor_field1.clone(), fn_.clone().expect("a proven present value"));
                                                    if !(match (setup.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }) {
                                                        self::request_java_script_call_error(machine.clone(), match setup.clone().expect("a proven present value") {
                                                            self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(source_ml_source2_data_property_value35612, _, _) => {
                                                                source_ml_source2_data_property_value35612.clone()
                                                            }
                                                        }, &(match setup.clone().expect("a proven present value") {
                                                            self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(_, source_ml_source2_data_property_value35613, _) => {
                                                                source_ml_source2_data_property_value35613.clone()
                                                            }
                                                        }), &(match setup.clone().expect("a proven present value") {
                                                            self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(_, _, source_ml_source2_data_property_value35614) => {
                                                                source_ml_source2_data_property_value35614.clone()
                                                            }
                                                        }))
                                                    } else {
                                                        if self::test_java_script_ordinary_class_constructor_mode(&(match fn_.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value35615, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value35615.clone()
                                                            }
                                                        })) {
                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 97, 109, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110]))
                                                        } else {
                                                            {
                                                                let slots = match fn_.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value35617, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value35617
                                                                    }
                                                                };
                                                                {
                                                                    let argument_slots = { let ml_reference_array2286 = arguments_.clone(); let ml_reference_projection2286 = ml_reference_array2286.borrow().len() as f64; ml_reference_projection2286 };
                                                                    {
                                                                        let source_ml_source1_temporary_values1500 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source1_temporary_index1500 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s35619 = self::source_ml_source1_temporary_push_java_script_call_loop1502(slots, argument_slots, source_ml_source1_temporary_values1500.clone(), source_ml_source1_temporary_index1500.clone());
                                                                                match source_ml_source2_s35619.clone() {
                                                                                    self::MlSource1TemporaryPushJavaScriptCallLoop1502Result::MlSource1TemporaryPushJavaScriptCallLoop1502Done(slots_2, argument_slots_2) => {
                                                                                        if (((({ let ml_reference_array2287 = shared_constructor_field2.clone(); let ml_reference_projection2287 = ml_reference_array2287.borrow().len() as f64; ml_reference_projection2287 }) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                        } else {
                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field1.clone()), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                let source_ml_source2_concat_value35624 = shared_constructor_field2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_concat_value35625 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_.clone())]));
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), { let ml_reference_value2288 = source_ml_source2_concat_value35624.borrow().clone(); ml_reference_value2288 }), { let ml_reference_value2289 = source_ml_source2_concat_value35625.borrow().clone(); ml_reference_value2289 })
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
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => self::request_java_script_call_error(
                                    machine.clone(),
                                    match owner.clone().expect("a proven present value") {
                                        self::JavaScriptCallFunction::CallFunction(
                                            _,
                                            source_ml_source2_data_property_value35626,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value35626.clone(),
                                    },
                                    &(self::ml_string::Text::from_units(vec![
                                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                                    ])),
                                    &(self::ml_string::Text::from_units(vec![
                                        116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100,
                                        32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111,
                                        116, 32, 99, 97, 108, 108, 97, 98, 108, 101,
                                    ])),
                                ),
                            }
                        }
                    }
                }
            }
        }
    }
}
