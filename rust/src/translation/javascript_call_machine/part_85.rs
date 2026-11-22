// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationFallback;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::accept_java_script_array_creation_iterator;
use super::accept_java_script_array_creation_length;
use super::accept_java_script_array_creation_wrapper;
use super::begin_java_script_primitive_wrapper;
use super::begin_java_script_synchronous_iterator_step;
use super::fail_java_script_array_creation;
use super::finish_java_script_scalar_primitive;
use super::ml_string;
use super::read_java_script_array_creation_property;
use super::read_java_script_array_creation_state_error;
use super::step_java_script_array_creation_result;
use super::step_java_script_iterator_protocol;
use super::step_java_script_scalar_protocol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_creation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
) -> self::JavaScriptArrayCreationTransition {
    match state.clone() {
        self::JavaScriptArrayCreationState::ArrayCreationState(
            _,
            shared_constructor_field1,
            _,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            shared_constructor_field4,
            _,
            _,
            _,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            _,
            shared_constructor_field9,
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
            _,
            _,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field9 = *shared_constructor_field9;
            {
                let error =
                    self::read_java_script_array_creation_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value16585,
                                _,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value16585
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptArrayCreationTransition::ArrayCreationCall(
                            Box::new(heap.clone()),
                            Box::new(state.clone()),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value16586,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value16586
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArrayCreationCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(
                                        source_ml_source2_data_property_value16587,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value16587 =
                                            *source_ml_source2_data_property_value16587;
                                        source_ml_source2_data_property_value16587.clone()
                                    }
                                },
                            ),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value16588,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value16588
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArrayCreationCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(
                                        _,
                                        source_ml_source2_data_property_value16589,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value16589 =
                                            *source_ml_source2_data_property_value16589;
                                        source_ml_source2_data_property_value16589.clone()
                                    }
                                },
                            ),
                            match (match state.clone() {
                                self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value16590,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value16590
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptArrayCreationCall>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value")
                            {
                                self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value16591,
                                ) => source_ml_source2_data_property_value16591.clone(),
                            },
                        )
                    } else {
                        if (shared_constructor_field6
                            == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])))
                        {
                            self::JavaScriptArrayCreationTransition::ArrayCreationNeedError(
                                Box::new(heap.clone()),
                                Box::new(state.clone()),
                                (match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value16594,
                                        _,
                                    ) => ((source_ml_source2_data_property_value16594
                                        .clone()
                                        .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .unwrap_or_else(|| {
                                    self::ml_string::Text::from_units(vec![
                                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                                    ])
                                }),
                                (match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value16595,
                                    ) => ((source_ml_source2_data_property_value16595
                                        .clone()
                                        .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                            )
                        } else {
                            if !(match (
                                match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                        source_ml_source2_data_property_value16596,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value16596
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptIteratorProtocolState>,)
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
                                self::accept_java_script_array_creation_iterator(state.clone(), self::step_java_script_iterator_protocol(heap.clone(), (match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16597, _, _, _, _, _, _, _, _) => {
                                        ((source_ml_source2_data_property_value16597.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value")))
                            } else {
                                if (shared_constructor_field6
                                    == (self::ml_string::Text::from_units(vec![
                                        109, 101, 116, 104, 111, 100,
                                    ])))
                                {
                                    if {
                                        let source_ml_source2_s16601 =
                                            shared_constructor_field4.clone();
                                        match source_ml_source2_s16601.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => true,
                                            _ => false,
                                        }
                                    } {
                                        self::read_java_script_array_creation_property(
                                            heap.clone(),
                                            state.clone(),
                                            shared_constructor_field4.clone(),
                                            shared_constructor_field3.clone(),
                                            &(self::ml_string::Text::from_units(vec![
                                                109, 101, 116, 104, 111, 100,
                                            ])),
                                            shared_constructor_field4.clone(),
                                        )
                                    } else {
                                        if match (match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16608, _, _, _, _, _, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value16608.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::JavaScriptArrayCreationTransition::ArrayCreationPrimitiveGet(Box::new(heap.clone()), Box::new(state.clone()), Box::new(shared_constructor_field4.clone()), Box::new(shared_constructor_field3.clone()))
                                        } else {
                                            self::read_java_script_array_creation_property(heap.clone(), state.clone(), (match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16611, _, _, _, _, _, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value16611.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value"), shared_constructor_field3.clone(), &(self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])), shared_constructor_field4.clone())
                                        }
                                    }
                                } else {
                                    if (shared_constructor_field6
                                        == (self::ml_string::Text::from_units(vec![
                                            119, 114, 97, 112, 112, 101, 114,
                                        ])))
                                    {
                                        if match (match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16616, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value16616.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperState>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 84, 111, 79, 98, 106, 101, 99, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 119, 114, 97, 112, 112, 101, 114, 32, 115, 116, 97, 116, 101]))
                                        } else {
                                            self::accept_java_script_array_creation_wrapper(state.clone(), self::begin_java_script_primitive_wrapper(heap.clone(), match (match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16617, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value16617.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperState>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value") {
                                                self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(source_ml_source2_data_property_value16618, _) => {
                                                    let source_ml_source2_data_property_value16618 = *source_ml_source2_data_property_value16618;
                                                    source_ml_source2_data_property_value16618.clone()
                                                }
                                            }, match (match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16619, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value16619.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperState>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value") {
                                                self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(_, source_ml_source2_data_property_value16620) => {
                                                    let source_ml_source2_data_property_value16620 = *source_ml_source2_data_property_value16620;
                                                    source_ml_source2_data_property_value16620.clone()
                                                }
                                            }))
                                        }
                                    } else {
                                        if (shared_constructor_field6
                                            == (self::ml_string::Text::from_units(vec![
                                                108, 101, 110, 103, 116, 104,
                                            ])))
                                        {
                                            self::read_java_script_array_creation_property(
                                                heap.clone(),
                                                state.clone(),
                                                shared_constructor_field9.clone(),
                                                self::JavaScriptSemanticValue::Text(
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        vec![
                                                            108f64, 101f64, 110f64, 103f64, 116f64,
                                                            104f64,
                                                        ],
                                                    )),
                                                ),
                                                &(self::ml_string::Text::from_units(vec![
                                                    108, 101, 110, 103, 116, 104,
                                                ])),
                                                shared_constructor_field9.clone(),
                                            )
                                        } else {
                                            if (shared_constructor_field6
                                                == (self::ml_string::Text::from_units(vec![
                                                    108, 101, 110, 103, 116, 104, 32, 99, 111, 110,
                                                    118, 101, 114, 115, 105, 111, 110,
                                                ])))
                                            {
                                                if !(match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16627, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value16627.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                    self::accept_java_script_array_creation_length(heap.clone(), state.clone(), self::step_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16628, _, _, _, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value16628.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                        }
                                                    }).clone().expect("a proven present value"), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                } else {
                                                    if !(match (match state.clone() {
                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16629, _, _, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value16629.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                        self::accept_java_script_array_creation_length(heap.clone(), state.clone(), self::finish_java_script_scalar_primitive(&(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])), (match state.clone() {
                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16630, _, _, _, _, _, _) => {
                                                                ((source_ml_source2_data_property_value16630.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                            }
                                                        }).clone().expect("a proven present value")))
                                                    } else {
                                                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                                    }
                                                }
                                            } else {
                                                if (shared_constructor_field6
                                                    == (self::ml_string::Text::from_units(vec![
                                                        99, 111, 110, 115, 116, 114, 117, 99, 116,
                                                    ])))
                                                {
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(Box::new(heap.clone()), Box::new(state.clone()), Box::new(shared_constructor_field1.clone()), if shared_constructor_field5 {
                                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()))
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Number(shared_constructor_field8)]))
                                                    }, Box::new(self::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(Box::new(shared_constructor_field2.clone()), if shared_constructor_field5 {
                                                        0f64
                                                    } else {
                                                        shared_constructor_field8
                                                    })))
                                                } else {
                                                    if (shared_constructor_field6
                                                        == (self::ml_string::Text::from_units(
                                                            vec![105, 116, 101, 114, 97, 116, 101],
                                                        )))
                                                    {
                                                        if (shared_constructor_field7
                                                            >= (9007199254740991f64))
                                                        {
                                                            self::fail_java_script_array_creation(heap.clone(), state.clone(), &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 114, 101, 115, 117, 108, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 109, 97, 120, 105, 109, 117, 109, 32, 115, 97, 102, 101, 32, 105, 110, 100, 101, 120])))
                                                        } else {
                                                            if match (match state.clone() {
                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16643, _, _, _, _, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value16643.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 113, 117, 105, 114, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 99, 111, 114, 100]))
                                                            } else {
                                                                self::accept_java_script_array_creation_iterator(state.clone(), self::begin_java_script_synchronous_iterator_step(heap.clone(), (match state.clone() {
                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16644, _, _, _, _, _, _, _, _, _) => {
                                                                        ((source_ml_source2_data_property_value16644.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value"), true))
                                                            }
                                                        }
                                                    } else {
                                                        self::step_java_script_array_creation_result(
                                                            heap.clone(),
                                                            state.clone(),
                                                        )
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
