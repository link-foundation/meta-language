// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentTransition;
use super::JavaScriptArrayCallbackTransition;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_integer_protocol_operation_error;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::source_ml_source1_temporary_read_java_script_selected_call_frame_error_loop1407;
use super::test_java_script_semantic_same_value;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn invert_java_script_call_relational_comparison(
    result: self::JavaScriptCallPrimitiveResult,
) -> self::JavaScriptCallPrimitiveResult {
    match result.clone() {
        self::JavaScriptCallPrimitiveResult::PrimitiveValue(order) => {
            let order = *order;
            match order.clone() {
                self::JavaScriptSemanticValue::Number(value) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                        self::JavaScriptSemanticValue::Number(if (value == (2f64)) {
                            2f64
                        } else {
                            if (value == (0f64)) { 0f64 } else { (-value) }
                        }),
                    ))
                }
                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 99, 111, 109, 112, 97,
                        114, 105, 115, 111, 110, 32, 105, 110, 118, 101, 114, 115, 105, 111, 110,
                        32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                        117, 97, 108, 32, 111, 114, 100, 101, 114, 105, 110, 103, 32, 99, 111, 100,
                        101,
                    ]),
                ),
            }
        }
        _ => result.clone(),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_protocol_operation_error(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (((((((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110,
            117, 109, 98, 101, 114,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                110, 117, 109, 98, 101, 114,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                110, 117, 109, 101, 114, 105, 99,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101,
                32, 118, 97, 108, 117, 101,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 118, 101, 114, 116, 32, 110, 117, 109, 98, 101, 114, 32, 112, 114,
                105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101,
            ]))))
        || ((self::read_java_script_integer_protocol_operation_error(operation))
            == (self::ml_string::Text::from_units(vec![]))))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
            114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
            108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99,
            101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_array_assignment(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptArrayAssignmentTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentContinue(
                    source_ml_source2_data_property_value32892,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32892 =
                        *source_ml_source2_data_property_value32892;
                    source_ml_source2_data_property_value32892.clone()
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCall(
                    source_ml_source2_data_property_value32893,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32893 =
                        *source_ml_source2_data_property_value32893;
                    source_ml_source2_data_property_value32893.clone()
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCompleted(
                    source_ml_source2_data_property_value32894,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32894 =
                        *source_ml_source2_data_property_value32894;
                    source_ml_source2_data_property_value32894.clone()
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentThrow(
                    source_ml_source2_data_property_value32895,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32895 =
                        *source_ml_source2_data_property_value32895;
                    source_ml_source2_data_property_value32895.clone()
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentError(
                    source_ml_source2_data_property_value32896,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32896 =
                        *source_ml_source2_data_property_value32896;
                    source_ml_source2_data_property_value32896.clone()
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                    source_ml_source2_data_property_value32897,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32897 =
                        *source_ml_source2_data_property_value32897;
                    source_ml_source2_data_property_value32897.clone()
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(
                    source_ml_source2_data_property_value32898,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32898 =
                        *source_ml_source2_data_property_value32898;
                    source_ml_source2_data_property_value32898.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value32899,
                ) => source_ml_source2_data_property_value32899.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                let frame = {
                    let source_ml_source2_array_read_value32902 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_array_read_index32903 = ({
                            let ml_reference_array2148 = shared_constructor_field1.clone();
                            let ml_reference_projection2148 =
                                ml_reference_array2148.borrow().len() as f64;
                            ml_reference_projection2148
                        } - 1f64);
                        {
                            let ml_reference_array2149 =
                                source_ml_source2_array_read_value32902.clone();
                            let ml_reference_position2149 = source_ml_source2_array_read_index32903;
                            let ml_reference_projection2149 = self::ml_array::optional_at(
                                &*ml_reference_array2149.borrow(),
                                self::ml_array::number_index(ml_reference_position2149),
                            );
                            ml_reference_projection2149
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentContinue(source_ml_source2_data_property_value32904, _) => {
                                let source_ml_source2_data_property_value32904 = *source_ml_source2_data_property_value32904;
                                source_ml_source2_data_property_value32904.clone()
                            }
                            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCall(source_ml_source2_data_property_value32905, _, _, _, _) => {
                                let source_ml_source2_data_property_value32905 = *source_ml_source2_data_property_value32905;
                                source_ml_source2_data_property_value32905.clone()
                            }
                            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCompleted(source_ml_source2_data_property_value32906, _, _) => {
                                let source_ml_source2_data_property_value32906 = *source_ml_source2_data_property_value32906;
                                source_ml_source2_data_property_value32906.clone()
                            }
                            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentThrow(source_ml_source2_data_property_value32907, _) => {
                                let source_ml_source2_data_property_value32907 = *source_ml_source2_data_property_value32907;
                                source_ml_source2_data_property_value32907.clone()
                            }
                            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentError(source_ml_source2_data_property_value32908, _, _) => {
                                let source_ml_source2_data_property_value32908 = *source_ml_source2_data_property_value32908;
                                source_ml_source2_data_property_value32908.clone()
                            }
                            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(source_ml_source2_data_property_value32909, _) => {
                                let source_ml_source2_data_property_value32909 = *source_ml_source2_data_property_value32909;
                                source_ml_source2_data_property_value32909.clone()
                            }
                            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(source_ml_source2_data_property_value32910, _, _) => {
                                let source_ml_source2_data_property_value32910 = *source_ml_source2_data_property_value32910;
                                source_ml_source2_data_property_value32910.clone()
                            }
                        }), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentContinue(
                            heap,
                            state,
                        ) => {
                            let heap = *heap;
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArrayAssignment(
                                            destination,
                                            realm.clone(),
                                            Box::new(state.clone()),
                                        ),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCall(
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
                                        self::JavaScriptCallPending::WaitArrayAssignment(
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
                        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCompleted(
                            heap_3,
                            success,
                            _,
                        ) => {
                            let heap_3 = *heap_3;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap_3.clone(),
                                    shared_constructor_field1.clone(),
                                    self::write_java_script_call_register(
                                        frame.clone().expect("a proven present value"),
                                        destination,
                                        self::JavaScriptSemanticValue::Boolean(success),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentThrow(
                            _,
                            value,
                        ) => {
                            let value = *value;
                            self::throw_java_script_call_value(current.clone(), value.clone())
                        }
                        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentError(
                            _,
                            error_class,
                            reason,
                        ) => self::request_java_script_call_error(
                            current.clone(),
                            realm.clone(),
                            &(error_class),
                            &(reason),
                        ),
                        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                            heap_4,
                            reason_2,
                        ) => {
                            let heap_4 = *heap_4;
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(heap_4.clone()),
                                reason_2.clone(),
                            )
                        }
                        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(
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
                                        self::JavaScriptCallPending::WaitArrayAssignment(
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
pub fn accept_java_script_call_array_callback(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptArrayCallbackTransition,
) -> self::JavaScriptCallMachineTransition {
    {
        let current = self::JavaScriptCallMachine::CallMachine(
            Box::new(match transition.clone() {
                self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(
                    source_ml_source2_data_property_value32918,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32918 =
                        *source_ml_source2_data_property_value32918;
                    source_ml_source2_data_property_value32918.clone()
                }
                self::JavaScriptArrayCallbackTransition::ArrayCallbackCall(
                    source_ml_source2_data_property_value32919,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32919 =
                        *source_ml_source2_data_property_value32919;
                    source_ml_source2_data_property_value32919.clone()
                }
                self::JavaScriptArrayCallbackTransition::ArrayCallbackCompleted(
                    source_ml_source2_data_property_value32920,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32920 =
                        *source_ml_source2_data_property_value32920;
                    source_ml_source2_data_property_value32920.clone()
                }
                self::JavaScriptArrayCallbackTransition::ArrayCallbackThrow(
                    source_ml_source2_data_property_value32921,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32921 =
                        *source_ml_source2_data_property_value32921;
                    source_ml_source2_data_property_value32921.clone()
                }
                self::JavaScriptArrayCallbackTransition::ArrayCallbackError(
                    source_ml_source2_data_property_value32922,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32922 =
                        *source_ml_source2_data_property_value32922;
                    source_ml_source2_data_property_value32922.clone()
                }
                self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                    source_ml_source2_data_property_value32923,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32923 =
                        *source_ml_source2_data_property_value32923;
                    source_ml_source2_data_property_value32923.clone()
                }
                self::JavaScriptArrayCallbackTransition::ArrayCallbackResourceLimit(
                    source_ml_source2_data_property_value32924,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value32924 =
                        *source_ml_source2_data_property_value32924;
                    source_ml_source2_data_property_value32924.clone()
                }
            }),
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    _,
                    source_ml_source2_data_property_value32925,
                ) => source_ml_source2_data_property_value32925.clone(),
            },
        );
        match current.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                let frame = {
                    let source_ml_source2_array_read_value32928 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_array_read_index32929 = ({
                            let ml_reference_array2150 = shared_constructor_field1.clone();
                            let ml_reference_projection2150 =
                                ml_reference_array2150.borrow().len() as f64;
                            ml_reference_projection2150
                        } - 1f64);
                        {
                            let ml_reference_array2151 =
                                source_ml_source2_array_read_value32928.clone();
                            let ml_reference_position2151 = source_ml_source2_array_read_index32929;
                            let ml_reference_projection2151 = self::ml_array::optional_at(
                                &*ml_reference_array2151.borrow(),
                                self::ml_array::number_index(ml_reference_position2151),
                            );
                            ml_reference_projection2151
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
                            self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(
                                source_ml_source2_data_property_value32930,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32930 =
                                    *source_ml_source2_data_property_value32930;
                                source_ml_source2_data_property_value32930.clone()
                            }
                            self::JavaScriptArrayCallbackTransition::ArrayCallbackCall(
                                source_ml_source2_data_property_value32931,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32931 =
                                    *source_ml_source2_data_property_value32931;
                                source_ml_source2_data_property_value32931.clone()
                            }
                            self::JavaScriptArrayCallbackTransition::ArrayCallbackCompleted(
                                source_ml_source2_data_property_value32932,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32932 =
                                    *source_ml_source2_data_property_value32932;
                                source_ml_source2_data_property_value32932.clone()
                            }
                            self::JavaScriptArrayCallbackTransition::ArrayCallbackThrow(
                                source_ml_source2_data_property_value32933,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32933 =
                                    *source_ml_source2_data_property_value32933;
                                source_ml_source2_data_property_value32933.clone()
                            }
                            self::JavaScriptArrayCallbackTransition::ArrayCallbackError(
                                source_ml_source2_data_property_value32934,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32934 =
                                    *source_ml_source2_data_property_value32934;
                                source_ml_source2_data_property_value32934.clone()
                            }
                            self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                                source_ml_source2_data_property_value32935,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32935 =
                                    *source_ml_source2_data_property_value32935;
                                source_ml_source2_data_property_value32935.clone()
                            }
                            self::JavaScriptArrayCallbackTransition::ArrayCallbackResourceLimit(
                                source_ml_source2_data_property_value32936,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value32936 =
                                    *source_ml_source2_data_property_value32936;
                                source_ml_source2_data_property_value32936.clone()
                            }
                        }),
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32,
                            114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97,
                            99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 105, 110,
                            116, 114, 105, 110, 115, 105, 99, 32, 102, 114, 97, 109, 101,
                        ]),
                    )
                } else {
                    match transition.clone() {
                        self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(
                            heap,
                            state,
                        ) => {
                            let heap = *heap;
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    heap.clone(),
                                    shared_constructor_field1.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitArrayCallback(
                                            destination,
                                            realm.clone(),
                                            Box::new(state.clone()),
                                        ),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptArrayCallbackTransition::ArrayCallbackCall(
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
                                        self::JavaScriptCallPending::WaitArrayCallback(
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
                        self::JavaScriptArrayCallbackTransition::ArrayCallbackCompleted(
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
                        self::JavaScriptArrayCallbackTransition::ArrayCallbackThrow(_, value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(current.clone(), value_2.clone())
                        }
                        self::JavaScriptArrayCallbackTransition::ArrayCallbackError(
                            _,
                            error_class,
                            reason,
                        ) => self::request_java_script_call_error(
                            current.clone(),
                            realm.clone(),
                            &(error_class),
                            &(reason),
                        ),
                        self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                            heap_4,
                            reason_2,
                        ) => {
                            let heap_4 = *heap_4;
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(heap_4.clone()),
                                reason_2.clone(),
                            )
                        }
                        self::JavaScriptArrayCallbackTransition::ArrayCallbackResourceLimit(
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
                                        self::JavaScriptCallPending::WaitArrayCallback(
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
pub fn read_java_script_selected_call_frame_error(
    fn_: self::JavaScriptCallFunction,
    frame: self::JavaScriptCallFrame,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field3,
            _,
            shared_constructor_field4,
            shared_constructor_field5,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(
                _,
                _,
                shared_constructor_field1,
                shared_constructor_field2,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let shared_constructor_field2 = *shared_constructor_field2;
                if (((match callee.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                } || (((((match callee.clone() {
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source2_data_property_value32947,
                        _,
                    ) => Some((source_ml_source2_data_property_value32947.clone(),)),
                    _ => None::<(self::ml::Big,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0)
                    .clone())
                    != shared_constructor_field3))
                    || (({
                        let ml_reference_array2152 = arguments_.clone();
                        let ml_reference_projection2152 =
                            ml_reference_array2152.borrow().len() as f64;
                        ml_reference_projection2152
                    }) != ({
                        let ml_reference_array2153 = shared_constructor_field5.clone();
                        let ml_reference_projection2153 =
                            ml_reference_array2153.borrow().len() as f64;
                        ml_reference_projection2153
                    })))
                    || !(self::test_java_script_semantic_same_value(
                        ({
                            let source_ml_source2_array_read_value32952 =
                                shared_constructor_field4.clone();
                            {
                                let source_ml_source2_array_read_index32953 = 0f64;
                                {
                                    let ml_reference_array2154 =
                                        source_ml_source2_array_read_value32952.clone();
                                    let ml_reference_position2154 =
                                        source_ml_source2_array_read_index32953;
                                    let ml_reference_projection2154 = self::ml_array::optional_at(
                                        &*ml_reference_array2154.borrow(),
                                        self::ml_array::number_index(ml_reference_position2154),
                                    );
                                    ml_reference_projection2154
                                }
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                        if (shared_constructor_field1
                            == (self::ml_string::Text::from_units(vec![
                                108, 101, 120, 105, 99, 97, 108,
                            ])))
                        {
                            shared_constructor_field2.clone()
                        } else {
                            receiver.clone()
                        },
                    )))
                {
                    self::ml_string::Text::from_units(vec![
                        97, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100,
                        32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                        117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 100, 101,
                        110, 116, 105, 116, 121, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32,
                        97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115,
                    ])
                } else {
                    {
                        let at = 0f64;
                        {
                            let source_ml_source2_s32957 = self::source_ml_source1_temporary_read_java_script_selected_call_frame_error_loop1407(frame.clone(), arguments_.clone(), at);
                            match source_ml_source2_s32957.clone() {
                                    self::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Result::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Result::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Return(source_ml_source1_temporary_result1407) => {
                                        source_ml_source1_temporary_result1407.clone()
                                    }
                                }
                        }
                    }
                }
            }
        },
    }
}
