// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result;
use super::convert_java_script_array_unsigned_length;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_length_definition;
use super::plan_java_script_array_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_kind;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_read_java_script_array_definition_state_error_loop137;
use super::test_java_script_array_length;
use super::test_java_script_array_length_key;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_definition_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayDefinitionState,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                error.clone()
            } else if (match state.clone() {
                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, _, _) => {
                    false
                }
            } || !(self::ml_array::includes(
                &{
                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![
                            102, 105, 114, 115, 116, 32, 110, 117, 109, 98, 101, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            115, 101, 99, 111, 110, 100, 32, 110, 117, 109, 98, 101, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]),
                    ])))
                    .borrow()
                    .clone()
                },
                &match state.clone() {
                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value16068,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value16068.clone(),
                },
                0f64,
                false,
                |a, b| a == b,
            ))) || !(self::test_java_script_array_length(match state.clone() {
                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value16069,
                    _,
                ) => source_ml_source1_data_property_value16069,
            })) {
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110,
                    32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                    117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 110, 118, 101,
                    114, 115, 105, 111, 110, 32, 112, 104, 97, 115, 101,
                ])
            } else if {
                let source_ml_source1_s16071 = match state.clone() {
                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value16081,
                    ) => {
                        let source_ml_source1_data_property_value16081 =
                            *source_ml_source1_data_property_value16081;
                        source_ml_source1_data_property_value16081.clone()
                    }
                };
                match source_ml_source1_s16071 {
                    self::JavaScriptSemanticValue::Number(_) => false,
                    _ => true,
                }
            } {
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 109,
                    112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                    105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115, 101, 109, 97, 110, 116, 105,
                    99, 32, 78, 117, 109, 98, 101, 114,
                ])
            } else {
                {
                    let number_error = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match state.clone() {
                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value16082,
                            ) => {
                                let source_ml_source1_data_property_value16082 =
                                    *source_ml_source1_data_property_value16082;
                                source_ml_source1_data_property_value16082.clone()
                            }
                        },
                    );
                    {
                        let ml_library_inequality_2 =
                            number_error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                            number_error.clone()
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match state.clone() {
                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source1_data_property_value16083, _, _, _, _) => {
                                        let source_ml_source1_data_property_value16083 = *source_ml_source1_data_property_value16083;
                                        source_ml_source1_data_property_value16083.clone()
                                    }
                                });
                                {
                                    let ml_library_inequality_1 =
                                        key_error != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_1 {
                                        key_error.clone()
                                    } else {
                                        {
                                            let object = {
                                                let source_ml_source1_array_read_value16087 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16084, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value16084.clone()
                                                }
                                            };
                                                {
                                                    let source_ml_source1_array_read_index16088 = self::find_java_script_ordinary_object(match heap.clone() {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16085, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value16085.clone()
                                                    }
                                                }, match state.clone() {
                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source1_data_property_value16086, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value16086.clone()
                                                    }
                                                });
                                                    {
                                                        let ml_reference_array187 =
                                                            source_ml_source1_array_read_value16087;
                                                        let ml_reference_position187 =
                                                            source_ml_source1_array_read_index16088;
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array187.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position187,
                                                            ),
                                                        )
                                                    }
                                                }
                                            };
                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        } {
                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                                        } else if (self::read_java_script_property_descriptor_kind(match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16185, _, _, _) => {
                                                    source_ml_source1_data_property_value16185.clone()
                                                }
                                            })) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])) {
                                                self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 118, 97, 108, 105, 100, 32, 112, 97, 114, 116, 105, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114])
                                            } else {
                                                {
                                                    let source_ml_temporary_values136 = std::rc::Rc::new(std::cell::RefCell::new(vec![({ (match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16186, _, _, _) => {
                                                            source_ml_source1_data_property_value16186.clone()
                                                        }
                                                    }).borrow().clone() }).8, ({ (match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16187, _, _, _) => {
                                                            source_ml_source1_data_property_value16187.clone()
                                                        }
                                                    }).borrow().clone() }).4, ({ (match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16188, _, _, _) => {
                                                            source_ml_source1_data_property_value16188.clone()
                                                        }
                                                    }).borrow().clone() }).6]));
                                                    {
                                                        let source_ml_temporary_index136 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s16189 = self::source_ml_temporary_read_java_script_array_definition_state_error_loop137(heap.clone(), source_ml_temporary_values136, source_ml_temporary_index136);
                                                            match source_ml_source1_s16189 {
                                                                self::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Done => {
                                                                    if (({ (match state.clone() {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16190, _, _, _) => {
                                                                            source_ml_source1_data_property_value16190.clone()
                                                                        }
                                                                    }).borrow().clone() }).5 && !(self::test_java_script_semantic_accessor(({ (match state.clone() {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16191, _, _, _) => {
                                                                            source_ml_source1_data_property_value16191.clone()
                                                                        }
                                                                    }).borrow().clone() }).4))) || (({ (match state.clone() {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16192, _, _, _) => {
                                                                            source_ml_source1_data_property_value16192.clone()
                                                                        }
                                                                    }).borrow().clone() }).7 && !(self::test_java_script_semantic_accessor(({ (match state.clone() {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16193, _, _, _) => {
                                                                            source_ml_source1_data_property_value16193.clone()
                                                                        }
                                                                    }).borrow().clone() }).6))) {
                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 101, 115, 115, 111, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101, 115])
                                                                    } else if ((match state.clone() {
                                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source1_data_property_value16195, _, _) => {
                                                                                source_ml_source1_data_property_value16195.clone()
                                                                            }
                                                                        }) != (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) && (!(self::test_java_script_array_length_key(match state.clone() {
                                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source1_data_property_value16196, _, _, _, _) => {
                                                                                let source_ml_source1_data_property_value16196 = *source_ml_source1_data_property_value16196;
                                                                                source_ml_source1_data_property_value16196.clone()
                                                                            }
                                                                        })) || !(({ (match state.clone() {
                                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16197, _, _, _) => {
                                                                                source_ml_source1_data_property_value16197.clone()
                                                                            }
                                                                        }).borrow().clone() }).9)) {
                                                                            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 118, 97, 108, 117, 101, 32, 115, 116, 97, 114, 116, 115, 32, 116, 104, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 112, 114, 111, 116, 111, 99, 111, 108])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![])
                                                                        }
                                                                }
                                                                self::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlTemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Return(source_ml_temporary_result137) => {
                                                                    source_ml_temporary_result137.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn begin_java_script_array_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
) -> self::JavaScriptArrayCompletion {
    {
        let state = self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
            identity.clone(),
            Box::new(key.clone()),
            descriptor.clone(),
            if self::test_java_script_array_length_key(key.clone())
                && ({ descriptor.borrow().clone() }).9
            {
                self::ml_string::Text::from_units(vec![
                    102, 105, 114, 115, 116, 32, 110, 117, 109, 98, 101, 114,
                ])
            } else {
                self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])
            },
            0f64,
            Box::new(self::JavaScriptSemanticValue::Number(0f64)),
        );
        {
            let error =
                self::read_java_script_array_definition_state_error(heap.clone(), state.clone());
            if error == (self::ml_string::Text::from_units(vec![])) {
                self::JavaScriptArrayCompletion::ArrayDefinitionContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayDefinitionState,
) -> self::JavaScriptArrayCompletion {
    {
        let error =
            self::read_java_script_array_definition_state_error(heap.clone(), state.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            } else if (match state.clone() {
                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value16199,
                    _,
                    _,
                ) => source_ml_source1_data_property_value16199.clone(),
            }) != (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))
            {
                self::JavaScriptArrayCompletion::ArrayNumberRequest(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                    Box::new(
                        ({
                            (match state.clone() {
                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                                    _,
                                    _,
                                    source_ml_source1_data_property_value16200,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value16200.clone(),
                            })
                            .borrow()
                            .clone()
                        })
                        .8,
                    ),
                )
            } else {
                {
                    let object = {
                        let source_ml_source1_array_read_value16204 = match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value16201,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value16201.clone(),
                        };
                        {
                            let source_ml_source1_array_read_index16205 = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16202, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16202.clone()
                                }
                            }, match state.clone() {
                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source1_data_property_value16203, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16203.clone()
                                }
                            });
                            {
                                let ml_reference_array198 = source_ml_source1_array_read_value16204;
                                let ml_reference_position198 =
                                    source_ml_source1_array_read_index16205;
                                self::ml_array::optional_at(
                                    &*ml_reference_array198.borrow(),
                                    self::ml_array::number_index(ml_reference_position198),
                                )
                            }
                        }
                    };
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || match object.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => false,
                        _ => true,
                    } {
                        self::JavaScriptArrayCompletion::ArrayInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 114, 101, 99, 101, 105, 118, 101, 114,
                                32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105,
                                110, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 103,
                                108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114,
                                121,
                            ]),
                        )
                    } else {
                        {
                            let number_length = match state.clone() {
                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value16301,
                                ) => {
                                    let source_ml_source1_data_property_value16301 =
                                        *source_ml_source1_data_property_value16301;
                                    source_ml_source1_data_property_value16301.clone()
                                }
                            };
                            match number_length {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Number(value) => {
                                    let plan = if self::test_java_script_array_length_key(match state.clone() {
                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source1_data_property_value16310, _, _, _, _) => {
                                                let source_ml_source1_data_property_value16310 = *source_ml_source1_data_property_value16310;
                                                source_ml_source1_data_property_value16310.clone()
                                            }
                                        }) && ({ (match state.clone() {
                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16311, _, _, _) => {
                                                source_ml_source1_data_property_value16311.clone()
                                            }
                                        }).borrow().clone() }).9 {
                                            self::plan_java_script_array_length_definition(object.clone().expect("a proven present value"), match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16312, _, _, _) => {
                                                    source_ml_source1_data_property_value16312.clone()
                                                }
                                            }, match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, source_ml_source1_data_property_value16313, _) => {
                                                    source_ml_source1_data_property_value16313
                                                }
                                            }, value)
                                        } else {
                                            self::plan_java_script_array_property_definition(object.clone().expect("a proven present value"), match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source1_data_property_value16314, _, _, _, _) => {
                                                    let source_ml_source1_data_property_value16314 = *source_ml_source1_data_property_value16314;
                                                    source_ml_source1_data_property_value16314.clone()
                                                }
                                            }, match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16315, _, _, _) => {
                                                    source_ml_source1_data_property_value16315.clone()
                                                }
                                            })
                                        };
                                    if ((match plan.clone() {
                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value16317,
                                            _,
                                        ) => source_ml_source1_data_property_value16317.clone(),
                                    }) == (self::ml_string::Text::from_units(vec![
                                        82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                                    ]))) || ((match plan.clone() {
                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value16319,
                                            _,
                                        ) => source_ml_source1_data_property_value16319.clone(),
                                    }) == (self::ml_string::Text::from_units(vec![
                                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                                    ]))) {
                                        self::JavaScriptArrayCompletion::ArrayError(Box::new(heap.clone()), match plan.clone() {
                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, source_ml_source1_data_property_value16320, _) => {
                                                    source_ml_source1_data_property_value16320.clone()
                                                }
                                            }, match plan.clone() {
                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source1_data_property_value16321, _, _) => {
                                                    source_ml_source1_data_property_value16321.clone()
                                                }
                                            })
                                    } else if (match plan.clone() {
                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value16323,
                                            _,
                                        ) => source_ml_source1_data_property_value16323.clone(),
                                    }) == (self::ml_string::Text::from_units(vec![
                                        114, 101, 115, 111, 117, 114, 99, 101,
                                    ])) {
                                        {
                                            let source_ml_source1_constructor_field16326 = match plan.clone() {
                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source1_data_property_value16325, _, _) => {
                                                            source_ml_source1_data_property_value16325.clone()
                                                        }
                                                    };
                                            {
                                                let source_ml_source1_constructor_field16327 =
                                                    Some(state.clone());
                                                self::JavaScriptArrayCompletion::ArrayResourceLimit(
                                                    Box::new(heap.clone()),
                                                    source_ml_source1_constructor_field16326,
                                                    Some(source_ml_source1_constructor_field16327),
                                                )
                                            }
                                        }
                                    } else if (match plan.clone() {
                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value16329,
                                            _,
                                        ) => source_ml_source1_data_property_value16329.clone(),
                                    }) != (self::ml_string::Text::from_units(vec![]))
                                    {
                                        self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), match plan.clone() {
                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source1_data_property_value16330, _, _) => {
                                                            source_ml_source1_data_property_value16330.clone()
                                                        }
                                                    })
                                    } else {
                                        {
                                            let updated = self::replace_java_script_ordinary_object(heap.clone(), match plan.clone() {
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value16331, _, _, _, _) => {
                                                                let source_ml_source1_data_property_value16331 = *source_ml_source1_data_property_value16331;
                                                                source_ml_source1_data_property_value16331.clone()
                                                            }
                                                        });
                                            {
                                                let invalid =
                                                    self::read_java_script_ordinary_heap_error(
                                                        updated.clone(),
                                                    );
                                                {
                                                    let ml_library_inequality_1 = invalid
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        ));
                                                    if ml_library_inequality_1 {
                                                        {
                                                            let source_ml_source1_constructor_field16334 =
                                                                Some(state.clone());
                                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(Box::new(heap.clone()), invalid.clone(), Some(source_ml_source1_constructor_field16334,))
                                                        }
                                                    } else {
                                                        self::JavaScriptArrayCompletion::ArrayPropertyComplete(Box::new(updated.clone()), match plan.clone() {
                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value16335, _, _, _) => {
                                                                        source_ml_source1_data_property_value16335
                                                                    }
                                                                }, match plan.clone() {
                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value16336) => {
                                                                        source_ml_source1_data_property_value16336.clone()
                                                                    }
                                                                })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_property_number(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayDefinitionState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayCompletion {
    {
        let error =
            self::read_java_script_array_definition_state_error(heap.clone(), state.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            } else {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    {
                        let ml_library_inequality_1 =
                            invalid != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            self::JavaScriptArrayCompletion::ArrayInvalid(
                                Box::new(heap.clone()),
                                invalid.clone(),
                            )
                        } else {
                            match value.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(value_value) => {
                            if (match state.clone() {
                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source1_data_property_value16346, _, _) => {
                                    source_ml_source1_data_property_value16346.clone()
                                }
                            }) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])) {
                                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                            } else {
                                {
                                    let next = if (match state.clone() {
                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source1_data_property_value16348, _, _) => {
                                            source_ml_source1_data_property_value16348.clone()
                                        }
                                    }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 32, 110, 117, 109, 98, 101, 114])) {
                                        {
                                            let source_ml_source1_nominal_field16350 = match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source1_data_property_value16351, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value16351.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field16352 = match state.clone() {
                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source1_data_property_value16353, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value16353 = *source_ml_source1_data_property_value16353;
                                                        source_ml_source1_data_property_value16353.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_field16354 = match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16355, _, _, _) => {
                                                            source_ml_source1_data_property_value16355.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field16356 = match state.clone() {
                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source1_data_property_value16357, _, _) => {
                                                                source_ml_source1_data_property_value16357.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field16358 = match state.clone() {
                                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, source_ml_source1_data_property_value16359, _) => {
                                                                    source_ml_source1_data_property_value16359
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field16360 = match state.clone() {
                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, _, source_ml_source1_data_property_value16361) => {
                                                                        let source_ml_source1_data_property_value16361 = *source_ml_source1_data_property_value16361;
                                                                        source_ml_source1_data_property_value16361.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_override16363 = self::convert_java_script_array_unsigned_length(value_value);
                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source1_nominal_field16350, Box::new(source_ml_source1_nominal_field16352), source_ml_source1_nominal_field16354, self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 32, 110, 117, 109, 98, 101, 114]), source_ml_source1_nominal_override16363, Box::new(source_ml_source1_nominal_field16360))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let source_ml_source1_nominal_field16365 = match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source1_data_property_value16366, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value16366.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_field16367 = match state.clone() {
                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source1_data_property_value16368, _, _, _, _) => {
                                                        let source_ml_source1_data_property_value16368 = *source_ml_source1_data_property_value16368;
                                                        source_ml_source1_data_property_value16368.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_nominal_field16369 = match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source1_data_property_value16370, _, _, _) => {
                                                            source_ml_source1_data_property_value16370.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field16371 = match state.clone() {
                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source1_data_property_value16372, _, _) => {
                                                                source_ml_source1_data_property_value16372.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field16373 = match state.clone() {
                                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, source_ml_source1_data_property_value16374, _) => {
                                                                    source_ml_source1_data_property_value16374
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field16375 = match state.clone() {
                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, _, source_ml_source1_data_property_value16376) => {
                                                                        let source_ml_source1_data_property_value16376 = *source_ml_source1_data_property_value16376;
                                                                        source_ml_source1_data_property_value16376.clone()
                                                                    }
                                                                };
                                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source1_nominal_field16365, Box::new(source_ml_source1_nominal_field16367), source_ml_source1_nominal_field16369, self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]), source_ml_source1_nominal_field16373, Box::new(value.clone()))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    self::JavaScriptArrayCompletion::ArrayDefinitionContinue(Box::new(heap.clone()), Box::new(next))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                    }
                        }
                    }
                }
            }
        }
    }
}
