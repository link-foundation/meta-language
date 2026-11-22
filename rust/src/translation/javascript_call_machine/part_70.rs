// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::continue_java_script_iterator_protocol_value;
use super::create_java_script_iterator_protocol_state;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_iterator_protocol_state_error;
use super::read_java_script_synchronous_iterator_record_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_synchronous_iterator_step(
    heap: self::JavaScriptOrdinaryHeap,
    record: self::JavaScriptSynchronousIteratorRecord,
    read_value: bool,
) -> self::JavaScriptIteratorProtocolTransition {
    match record.clone() {
        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            {
                let error = self::read_java_script_synchronous_iterator_record_error(
                    heap.clone(),
                    record.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let base = self::create_java_script_iterator_protocol_state(
                            &(self::ml_string::Text::from_units(vec![115, 116, 101, 112])),
                            &(self::ml_string::Text::from_units(vec![
                                110, 101, 120, 116, 32, 99, 97, 108, 108,
                            ])),
                            shared_constructor_field10.clone(),
                            self::JavaScriptSemanticValue::Undefined,
                            record.clone(),
                            self::JavaScriptIteratorCompletion::IteratorCompletion(
                                self::ml_string::Text::from_units(vec![
                                    110, 111, 114, 109, 97, 108,
                                ]),
                                Box::new(self::JavaScriptSemanticValue::Undefined),
                                (-1f64),
                            ),
                        );
                        match base.clone() {
                            self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                shared_constructor_field1,
                                shared_constructor_field2,
                                shared_constructor_field3,
                                shared_constructor_field4,
                                shared_constructor_field5,
                                shared_constructor_field6,
                                shared_constructor_field7,
                                shared_constructor_field8,
                                shared_constructor_field9,
                            ) => {
                                let shared_constructor_field3 = *shared_constructor_field3;
                                let shared_constructor_field4 = *shared_constructor_field4;
                                let shared_constructor_field5 = *shared_constructor_field5;
                                let shared_constructor_field6 = *shared_constructor_field6;
                                let shared_constructor_field7 = *shared_constructor_field7;
                                {
                                    let state = if read_value {
                                        base.clone()
                                    } else {
                                        {
                                            let source_ml_source2_nominal_field13402 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field13404 =
                                                    shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field13406 =
                                                        shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field13408 =
                                                            shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field13410 =
                                                                shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field13412 =
                                                                    shared_constructor_field6
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field13414 =
                                                                        shared_constructor_field7
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field13416 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field13418 = shared_constructor_field9.clone();
                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_nominal_field13402.clone(), source_ml_source2_nominal_field13404.clone(), Box::new(source_ml_source2_nominal_field13406.clone()), Box::new(source_ml_source2_nominal_field13408.clone()), Box::new(source_ml_source2_nominal_field13410.clone()), Box::new(source_ml_source2_nominal_field13412.clone()), Box::new(source_ml_source2_nominal_field13414.clone()), Some((Some(false),)), source_ml_source2_nominal_field13418.clone())
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
                                    if shared_constructor_field12 {
                                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(Box::new(heap.clone()), Box::new({
                                            let source_ml_source2_nominal_field13423 = shared_constructor_field10.clone();
                                            {
                                                let source_ml_source2_nominal_field13425 = shared_constructor_field11.clone();
                                                {
                                                    let source_ml_source2_nominal_field13427 = shared_constructor_field12;
                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(source_ml_source2_nominal_field13423.clone()), Box::new(source_ml_source2_nominal_field13425.clone()), source_ml_source2_nominal_field13427)
                                                }
                                            }
                                        }), true, Box::new(self::JavaScriptSemanticValue::Undefined))
                                    } else {
                                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(Box::new(heap.clone()), Box::new(state.clone()))
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
pub fn begin_java_script_synchronous_iterator_close(
    heap: self::JavaScriptOrdinaryHeap,
    record: self::JavaScriptSynchronousIteratorRecord,
    completion: self::JavaScriptIteratorCompletion,
) -> self::JavaScriptIteratorProtocolTransition {
    {
        let state = self::create_java_script_iterator_protocol_state(
            &(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])),
            &(self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110])),
            match record.clone() {
                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
                    source_ml_source2_data_property_value13429,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value13429 =
                        *source_ml_source2_data_property_value13429;
                    source_ml_source2_data_property_value13429.clone()
                }
            },
            self::JavaScriptSemanticValue::Undefined,
            record.clone(),
            completion.clone(),
        );
        {
            let error =
                self::read_java_script_iterator_protocol_state_error(heap.clone(), state.clone());
            if (error == (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn fail_java_script_iterator_protocol(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptIteratorProtocolState,
    reason: &self::ml_string::Text,
) -> self::JavaScriptIteratorProtocolTransition {
    match state.clone() {
        self::JavaScriptIteratorProtocolState::IteratorProtocol(
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            if ((shared_constructor_field4
                == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])))
                && ((match shared_constructor_field10.clone() {
                    self::JavaScriptIteratorCompletion::IteratorCompletion(
                        source_ml_source2_data_property_value13435,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value13435.clone(),
                }) == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]))))
            {
                self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(
                    Box::new(heap.clone()),
                    Box::new(shared_constructor_field8.clone()),
                    Box::new(shared_constructor_field10.clone()),
                )
            } else {
                self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(
                    Box::new(heap.clone()),
                    Box::new(
                        if (shared_constructor_field4
                            == (self::ml_string::Text::from_units(vec![115, 116, 101, 112])))
                        {
                            {
                                let source_ml_source2_nominal_field13450 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field13452 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field13454 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field13456 =
                                                shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_nominal_field13458 =
                                                    shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_field13460 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field13462 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field13464 =
                                                                shared_constructor_field11.clone();
                                                            {
                                                                let source_ml_source2_nominal_field13466 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_override13468 = {
                                                                        let source_ml_source2_nominal_spread13441 = shared_constructor_field8.clone();
                                                                        match source_ml_source2_nominal_spread13441.clone() {
                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                        let shared_constructor_field1 = *shared_constructor_field1;
                                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                                        {
                                                                            let source_ml_source2_nominal_field13442 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field13444 = shared_constructor_field2.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field13446 = shared_constructor_field3;
                                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(source_ml_source2_nominal_field13442.clone()), Box::new(source_ml_source2_nominal_field13444.clone()), true)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                    };
                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_nominal_field13450.clone(), source_ml_source2_nominal_field13452.clone(), Box::new(source_ml_source2_nominal_field13454.clone()), Box::new(source_ml_source2_nominal_field13456.clone()), Box::new(source_ml_source2_nominal_override13468.clone()), Box::new(source_ml_source2_nominal_field13460.clone()), Box::new(source_ml_source2_nominal_field13462.clone()), source_ml_source2_nominal_field13464.clone(), source_ml_source2_nominal_field13466.clone())
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
                        } else {
                            state.clone()
                        },
                    ),
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    reason.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_iterator_protocol_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptIteratorProtocolState,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    phase: &self::ml_string::Text,
) -> self::JavaScriptIteratorProtocolTransition {
    match state.clone() {
        self::JavaScriptIteratorProtocolState::IteratorProtocol(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            if (match callee.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => false,
                _ => true,
            } || !(((match callee.clone() {
                self::JavaScriptSemanticValue::Object(
                    _,
                    source_ml_source2_data_property_value13471,
                ) => Some((source_ml_source2_data_property_value13471,)),
                _ => None::<(bool,)>,
            })
            .clone()
            .expect("a proven present value"))
            .0))
            {
                self::fail_java_script_iterator_protocol(
                    heap.clone(),
                    state.clone(),
                    &(self::ml_string::Text::from_units(vec![
                        105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 112, 101, 114, 97, 116,
                        105, 111, 110, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97,
                        98, 108, 101,
                    ])),
                )
            } else {
                {
                    let call = self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                        Box::new(callee.clone()),
                        Box::new(receiver.clone()),
                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                            self::JavaScriptSemanticValue,
                        >::new())),
                    );
                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(
                        Box::new(heap.clone()),
                        Box::new({
                            let source_ml_source2_nominal_field13473 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field13475 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field13477 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field13479 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field13481 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field13483 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field13485 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field13487 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field13489 =
                                                                shared_constructor_field9.clone();
                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_nominal_field13473.clone(), phase.clone(), Box::new(source_ml_source2_nominal_field13477.clone()), Box::new(source_ml_source2_nominal_field13479.clone()), Box::new(source_ml_source2_nominal_field13481.clone()), Box::new(source_ml_source2_nominal_field13483.clone()), Box::new(source_ml_source2_nominal_field13485.clone()), source_ml_source2_nominal_field13487.clone(), Some((Some(call.clone()),)))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }),
                        Box::new(callee.clone()),
                        Box::new(receiver.clone()),
                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                            self::JavaScriptSemanticValue,
                        >::new())),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_iterator_protocol(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptIteratorProtocolState,
) -> self::JavaScriptIteratorProtocolTransition {
    match state.clone() {
        self::JavaScriptIteratorProtocolState::IteratorProtocol(
            _,
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            {
                let error = self::read_java_script_iterator_protocol_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value13493,
                            ) => ((source_ml_source2_data_property_value13493
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<self::JavaScriptIteratorSelectedCall>,)
                                }))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(
                            Box::new(heap.clone()),
                            Box::new(state.clone()),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value13494,
                                    ) => ((source_ml_source2_data_property_value13494
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptIteratorSelectedCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                                        source_ml_source2_data_property_value13495,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value13495 =
                                            *source_ml_source2_data_property_value13495;
                                        source_ml_source2_data_property_value13495.clone()
                                    }
                                },
                            ),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value13496,
                                    ) => ((source_ml_source2_data_property_value13496
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptIteratorSelectedCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                                        _,
                                        source_ml_source2_data_property_value13497,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value13497 =
                                            *source_ml_source2_data_property_value13497;
                                        source_ml_source2_data_property_value13497.clone()
                                    }
                                },
                            ),
                            match (match state.clone() {
                                self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value13498,
                                ) => ((source_ml_source2_data_property_value13498
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptIteratorSelectedCall>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value")
                            {
                                self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value13499,
                                ) => source_ml_source2_data_property_value13499.clone(),
                            },
                        )
                    } else {
                        if (shared_constructor_field1
                            == (self::ml_string::Text::from_units(vec![
                                110, 101, 120, 116, 32, 99, 97, 108, 108,
                            ])))
                        {
                            self::request_java_script_iterator_protocol_call(heap.clone(), state.clone(), match shared_constructor_field4.clone() {
                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value13503, _) => {
                                    let source_ml_source2_data_property_value13503 = *source_ml_source2_data_property_value13503;
                                    source_ml_source2_data_property_value13503.clone()
                                }
                            }, match shared_constructor_field4.clone() {
                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value13505, _, _) => {
                                    let source_ml_source2_data_property_value13505 = *source_ml_source2_data_property_value13505;
                                    source_ml_source2_data_property_value13505.clone()
                                }
                            }, &(self::ml_string::Text::from_units(vec![110, 101, 120, 116, 32, 99, 97, 108, 108])))
                        } else {
                            {
                                let receiver = if (shared_constructor_field1
                                    == (self::ml_string::Text::from_units(vec![
                                        109, 101, 116, 104, 111, 100,
                                    ]))) {
                                    shared_constructor_field2.clone()
                                } else {
                                    if ((shared_constructor_field1
                                        == (self::ml_string::Text::from_units(vec![
                                            110, 101, 120, 116,
                                        ])))
                                        || (shared_constructor_field1
                                            == (self::ml_string::Text::from_units(vec![
                                                114, 101, 116, 117, 114, 110,
                                            ]))))
                                    {
                                        match shared_constructor_field4.clone() {
                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value13514, _, _) => {
                                                let source_ml_source2_data_property_value13514 = *source_ml_source2_data_property_value13514;
                                                source_ml_source2_data_property_value13514.clone()
                                            }
                                        }
                                    } else {
                                        shared_constructor_field5.clone()
                                    }
                                };
                                {
                                    let key = if (shared_constructor_field1
                                        == (self::ml_string::Text::from_units(vec![
                                            109, 101, 116, 104, 111, 100,
                                        ]))) {
                                        shared_constructor_field3.clone()
                                    } else {
                                        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                            std::cell::RefCell::new({
                                                let ml_values788 =
                                                    shared_constructor_field1.code_points();
                                                let ml_callback788 = std::rc::Rc::new(
                                                    move |ch: self::ml_string::Text| -> f64 {
                                                        {
                                                            let source_ml_source2_string_unit_operand13520 =
                                                                ch.clone();
                                                            {
                                                                let source_ml_source2_string_unit_operand13521 =
                                                                    0f64;
                                                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                    let index = if position.is_nan() {
                                                        0f64
                                                    } else {
                                                        position.trunc()
                                                    };
                                                    if ((index < (0f64)) || (index >= (text.length()))) {
                                                        f64::NAN
                                                    } else {
                                                        ({
                                                            let source_ml_source2_string_unit_operand13527 = {
                                                                let source_ml_source2_string_range_operand13522 = text.clone();
                                                                {
                                                                    let source_ml_source2_string_range_operand13523 = index;
                                                                    {
                                                                        let source_ml_source2_string_range_operand13524 = (index + 1f64);
                                                                        {
                                                                            let source_ml_source2_string_range_position13525 = source_ml_source2_string_range_operand13523;
                                                                            {
                                                                                let source_ml_source2_string_range_position13526 = source_ml_source2_string_range_operand13524;
                                                                                source_ml_source2_string_range_operand13522.slice(source_ml_source2_string_range_position13525, source_ml_source2_string_range_position13526)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_string_unit_operand13528 = 0f64;
                                                                source_ml_source2_string_unit_operand13527.code_point_at(source_ml_source2_string_unit_operand13528)
                                                            }
                                                        }).clone().unwrap_or_else(|| f64::NAN)
                                                    }
                                                } }))(source_ml_source2_string_unit_operand13520.clone(), source_ml_source2_string_unit_operand13521)
                                                            }
                                                        }
                                                    },
                                                );
                                                ml_values788
                                                    .iter()
                                                    .cloned()
                                                    .enumerate()
                                                    .map(|(ml_index788, ml_value788)| {
                                                        ml_callback788(ml_value788)
                                                    })
                                                    .collect::<Vec<_>>()
                                            }),
                                        ))
                                    };
                                    match receiver.clone() {
                                        self::JavaScriptSemanticValue::Object(identity, _) => {
                                            {
                                                let get = self::plan_java_script_ordinary_get(heap.clone(), identity.clone(), key.clone(), receiver.clone());
                                                if (((({ let ml_reference_value789 = get.borrow().clone(); ml_reference_value789 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(Box::new(heap.clone()), (({ let ml_reference_value790 = get.borrow().clone(); ml_reference_value790 }).0).3)
                                                } else {
                                                    if (((({ let ml_reference_value791 = get.borrow().clone(); ml_reference_value791 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                        self::request_java_script_iterator_protocol_call(heap.clone(), state.clone(), (({ let ml_reference_value792 = get.borrow().clone(); ml_reference_value792 }).0).2, (({ let ml_reference_value793 = get.borrow().clone(); ml_reference_value793 }).0).4, &(shared_constructor_field1.concat(&self::ml_string::Text::from_units(vec![32, 103, 101, 116, 116, 101, 114]))))
                                                    } else {
                                                        if (((({ let ml_reference_value794 = get.borrow().clone(); ml_reference_value794 }).0).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                            self::continue_java_script_iterator_protocol_value(heap.clone(), state.clone(), (({ let ml_reference_value795 = get.borrow().clone(); ml_reference_value795 }).0).5)
                                                        } else {
                                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 103, 101, 116, 116, 101, 114]))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(Box::new(heap.clone()), Box::new(state.clone()), Box::new(receiver.clone()), Box::new(key.clone()))
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
