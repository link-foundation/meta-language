// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptCollectionCompletion;
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptSemanticValue;
use super::accept_java_script_array_creation_wrapper;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_creation_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::resume_java_script_primitive_wrapper;
use super::source_ml_source1_temporary_allocate_java_script_collection_loop423;
use super::step_java_script_array_creation;
use super::throw_java_script_array_creation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_creation_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    identity: self::ml::Big,
) -> self::JavaScriptArrayCreationTransition {
    {
        let error = self::read_java_script_array_creation_state_error(heap.clone(), state.clone());
        if (((error != (self::ml_string::Text::from_units(vec![])))
            || ((match state.clone() {
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
                    source_ml_source2_data_property_value17034,
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
                ) => source_ml_source2_data_property_value17034.clone(),
            }) != (self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]))))
            || match (
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
                        source_ml_source2_data_property_value17035,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value17035
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperState>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            })
        {
            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                Box::new(heap.clone()),
                if !error.0.is_empty() {
                    error.clone()
                } else {
                    self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 97, 108, 108, 111, 99,
                        97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                        115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 109, 112, 111, 114, 97,
                        114, 121, 32, 84, 111, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117,
                        101, 115, 116,
                    ])
                },
            )
        } else {
            {
                let request = self::step_java_script_array_creation(heap.clone(), state.clone());
                match request.clone() {
                    self::JavaScriptArrayCreationTransition::ArrayCreationAllocate(_, _, _) => {
                        self::accept_java_script_array_creation_wrapper(
                            state.clone(),
                            self::resume_java_script_primitive_wrapper(
                                heap.clone(),
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
                                        source_ml_source2_data_property_value17040,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value17040
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptPrimitiveWrapperState>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value"),
                                identity.clone(),
                            ),
                        )
                    }
                    _ => {
                        if match request.clone() {
                            self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(
                                _,
                                _,
                                _,
                            ) => true,
                            _ => false,
                        } {
                            request.clone()
                        } else {
                            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 79, 98, 106,
                                    101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32,
                                    114, 101, 115, 112, 111, 110, 115, 101, 32, 114, 101, 116, 97,
                                    105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 116, 101, 109, 112, 111, 114, 97, 114, 121, 32, 97,
                                    108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113,
                                    117, 101, 115, 116,
                                ]),
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
pub fn provide_java_script_array_creation_primitive_prototype(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayCreationTransition {
    match state.clone() {
        self::JavaScriptArrayCreationState::ArrayCreationState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
            shared_constructor_field20,
            shared_constructor_field21,
            shared_constructor_field22,
            shared_constructor_field23,
            shared_constructor_field24,
            shared_constructor_field25,
            shared_constructor_field26,
            shared_constructor_field27,
            shared_constructor_field28,
            shared_constructor_field29,
            shared_constructor_field30,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field15 = *shared_constructor_field15;
            let shared_constructor_field16 = *shared_constructor_field16;
            let shared_constructor_field17 = *shared_constructor_field17;
            let shared_constructor_field18 = *shared_constructor_field18;
            let shared_constructor_field19 = *shared_constructor_field19;
            {
                let error = {
                    let source_ml_source2_logical17044 =
                        self::read_java_script_array_creation_state_error(
                            heap.clone(),
                            state.clone(),
                        );
                    if !source_ml_source2_logical17044.0.is_empty() {
                        source_ml_source2_logical17044.clone()
                    } else {
                        self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone())
                    }
                };
                if (((((error != (self::ml_string::Text::from_units(vec![])))
                    || (shared_constructor_field12
                        != (self::ml_string::Text::from_units(vec![
                            109, 101, 116, 104, 111, 100,
                        ]))))
                    || {
                        let source_ml_source2_s17048 = shared_constructor_field7.clone();
                        match source_ml_source2_s17048.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => true,
                            _ => false,
                        }
                    })
                    || !(match (
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
                                source_ml_source2_data_property_value17052,
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
                            ) => ((source_ml_source2_data_property_value17052
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }))
                    || match prototype.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => false,
                        _ => true,
                    })
                {
                    self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                        Box::new(heap.clone()),
                        if !error.0.is_empty() {
                            error.clone()
                        } else {
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 112, 114, 105,
                                109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 32,
                                71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32,
                                112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 114, 101, 113,
                                117, 101, 115, 116,
                            ])
                        },
                    )
                } else {
                    self::step_java_script_array_creation(heap.clone(), {
                        let source_ml_source2_nominal_field17057 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_nominal_field17059 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_nominal_field17061 =
                                    shared_constructor_field3.clone();
                                {
                                    let source_ml_source2_nominal_field17063 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field17065 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field17067 =
                                                shared_constructor_field6.clone();
                                            {
                                                let source_ml_source2_nominal_field17069 =
                                                    shared_constructor_field7.clone();
                                                {
                                                    let source_ml_source2_nominal_field17071 =
                                                        shared_constructor_field8.clone();
                                                    {
                                                        let source_ml_source2_nominal_field17073 =
                                                            shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_nominal_field17075 =
                                                                shared_constructor_field10;
                                                            {
                                                                let source_ml_source2_nominal_field17077 =
                                                                    shared_constructor_field11;
                                                                {
                                                                    let source_ml_source2_nominal_field17079 =
                                                                        shared_constructor_field12
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field17081 = shared_constructor_field13;
                                                                        {
                                                                            let source_ml_source2_nominal_field17083 = shared_constructor_field14;
                                                                            {
                                                                                let source_ml_source2_nominal_field17085 = shared_constructor_field15.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field17087 = shared_constructor_field16.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field17089 = shared_constructor_field17.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field17091 = shared_constructor_field18.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field17093 = shared_constructor_field19.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field17095 = shared_constructor_field20.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field17097 = shared_constructor_field21.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field17099 = shared_constructor_field22.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field17101 = shared_constructor_field23.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field17103 = shared_constructor_field24.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field17105 = shared_constructor_field25.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field17107 = shared_constructor_field26.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field17109 = shared_constructor_field27.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field17111 = shared_constructor_field28.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field17113 = shared_constructor_field29.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field17115 = shared_constructor_field30.clone();
                                                                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field17057.clone(), Box::new(source_ml_source2_nominal_field17059.clone()), source_ml_source2_nominal_field17061.clone(), Box::new(source_ml_source2_nominal_field17063.clone()), Box::new(source_ml_source2_nominal_field17065.clone()), Box::new(source_ml_source2_nominal_field17067.clone()), Box::new(source_ml_source2_nominal_field17069.clone()), Box::new(source_ml_source2_nominal_field17071.clone()), Box::new(source_ml_source2_nominal_field17073.clone()), source_ml_source2_nominal_field17075, source_ml_source2_nominal_field17077, source_ml_source2_nominal_field17079.clone(), source_ml_source2_nominal_field17081, source_ml_source2_nominal_field17083, Box::new(source_ml_source2_nominal_field17085.clone()), Box::new(source_ml_source2_nominal_field17087.clone()), Box::new(source_ml_source2_nominal_field17089.clone()), Box::new(source_ml_source2_nominal_field17091.clone()), Box::new(source_ml_source2_nominal_field17093.clone()), Some((Some(prototype.clone()),)), source_ml_source2_nominal_field17097.clone(), source_ml_source2_nominal_field17099.clone(), source_ml_source2_nominal_field17101.clone(), source_ml_source2_nominal_field17103.clone(), source_ml_source2_nominal_field17105.clone(), source_ml_source2_nominal_field17107.clone(), source_ml_source2_nominal_field17109.clone(), source_ml_source2_nominal_field17111.clone(), source_ml_source2_nominal_field17113.clone(), source_ml_source2_nominal_field17115.clone())
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
                    })
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_creation_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayCreationTransition {
    {
        let error = {
            let source_ml_source2_logical17118 =
                self::read_java_script_array_creation_state_error(heap.clone(), state.clone());
            if !source_ml_source2_logical17118.0.is_empty() {
                source_ml_source2_logical17118.clone()
            } else {
                self::read_java_script_ordinary_value_error(heap.clone(), value.clone())
            }
        };
        if ((((error != (self::ml_string::Text::from_units(vec![])))
            || ((match state.clone() {
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
                    source_ml_source2_data_property_value17120,
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
                ) => source_ml_source2_data_property_value17120.clone(),
            }) != (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))))
            || match value.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => false,
                _ => true,
            })
            || ((match value.clone() {
                self::JavaScriptSemanticValue::Object(
                    _,
                    source_ml_source2_data_property_value17123,
                ) => Some((source_ml_source2_data_property_value17123,)),
                _ => None::<(bool,)>,
            })
            .clone()
            .expect("a proven present value"))
            .0)
        {
            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                Box::new(heap.clone()),
                if !error.0.is_empty() {
                    error.clone()
                } else {
                    self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 99, 108, 101, 97, 110,
                        117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                        99, 116, 117, 97, 108, 32, 109, 97, 116, 101, 114, 105, 97, 108, 105, 122,
                        101, 100, 32, 114, 101, 97, 108, 109, 32, 69, 114, 114, 111, 114,
                    ])
                },
            )
        } else {
            self::throw_java_script_array_creation(heap.clone(), state.clone(), value.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_collection(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
    kind: &self::ml_string::Text,
) -> self::JavaScriptCollectionCompletion {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCollectionCompletion::CollectionInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                if (((*kind) != (self::ml_string::Text::from_units(vec![77, 97, 112])))
                    && ((*kind) != (self::ml_string::Text::from_units(vec![83, 101, 116]))))
                {
                    self::JavaScriptCollectionCompletion::CollectionInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111,
                            99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116,
                            115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116,
                            101, 114, 101, 100, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116,
                            32, 107, 105, 110, 100,
                        ]),
                    )
                } else {
                    if ((((identity <= (self::ml::Big::from_i128(0)))
                        || (identity > (self::ml::Big::from_i128(18446744073709551615))))
                        || ((self::find_java_script_ordinary_object(
                            shared_constructor_field1.clone(),
                            identity.clone(),
                        )) >= (0f64)))
                        || self::ml_array::includes(
                            &{
                                let ml_reference_value883 =
                                    shared_constructor_field2.borrow().clone();
                                ml_reference_value883
                            },
                            &identity.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))
                    {
                        self::JavaScriptCollectionCompletion::CollectionInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108,
                                111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32,
                                97, 32, 102, 114, 101, 115, 104, 32, 101, 120, 116, 101, 114, 110,
                                97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110,
                                116, 105, 116, 121,
                            ]),
                        )
                    } else {
                        if (({
                            let ml_reference_array884 = shared_constructor_field1.clone();
                            let ml_reference_projection884 =
                                ml_reference_array884.borrow().len() as f64;
                            ml_reference_projection884
                        }) >= (4096f64))
                        {
                            self::JavaScriptCollectionCompletion::CollectionResourceLimit(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108,
                                    108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101,
                                    101, 100, 115, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97,
                                    108, 32, 104, 101, 97, 112, 32, 100, 105, 114, 101, 99, 116,
                                    111, 114, 121, 32, 98, 117, 100, 103, 101, 116,
                                ]),
                            )
                        } else {
                            if (match prototype.clone() {
                                self::JavaScriptSemanticValue::Null => false,
                                _ => true,
                            } && match prototype.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            }) {
                                self::JavaScriptCollectionCompletion::CollectionInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97,
                                        108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101,
                                        116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                        117, 97, 108, 32, 110, 117, 108, 108, 32, 111, 114, 32, 79,
                                        98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116,
                                        121, 112, 101,
                                    ]),
                                )
                            } else {
                                {
                                    let invalid = self::read_java_script_ordinary_value_error(
                                        heap.clone(),
                                        prototype.clone(),
                                    );
                                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptCollectionCompletion::CollectionInvalid(
                                            Box::new(heap.clone()),
                                            invalid.clone(),
                                        )
                                    } else {
                                        {
                                            let object = if ((*kind)
                                                == (self::ml_string::Text::from_units(vec![
                                                    77, 97, 112,
                                                ]))) {
                                                self::JavaScriptOrdinaryObject::MapObject(
                                                    identity.clone(),
                                                    Box::new(prototype.clone()),
                                                    true,
                                                    false,
                                                    std::rc::Rc::new(
                                                        std::cell::RefCell::new(Vec::<
                                                            self::JavaScriptOrdinaryProperty,
                                                        >::new(
                                                        )),
                                                    ),
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        Vec::<self::JavaScriptCollectionEntry>::new(
                                                        ),
                                                    )),
                                                )
                                            } else {
                                                self::JavaScriptOrdinaryObject::SetObject(
                                                    identity.clone(),
                                                    Box::new(prototype.clone()),
                                                    true,
                                                    false,
                                                    std::rc::Rc::new(
                                                        std::cell::RefCell::new(Vec::<
                                                            self::JavaScriptOrdinaryProperty,
                                                        >::new(
                                                        )),
                                                    ),
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        Vec::<self::JavaScriptCollectionEntry>::new(
                                                        ),
                                                    )),
                                                )
                                            };
                                            {
                                                let index = 0f64;
                                                {
                                                    let index_2 = self::source_ml_source1_temporary_allocate_java_script_collection_loop423(heap.clone(), identity.clone(), index);
                                                    {
                                                        let updated = {
                                                            let source_ml_source2_nominal_field17141 =
                                                                shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field17143 =
                                                                    shared_constructor_field2
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field17145 =
                                                                        shared_constructor_field3
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field17147 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field17149 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field17151 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override17153 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source2_concat_value17137 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value885 = shared_constructor_field1.borrow().clone(); ml_reference_value885 }, 0f64, index_2)));
                                                                                            {
                                                                                                let source_ml_source2_concat_value17138 = std::rc::Rc::new(std::cell::RefCell::new(vec![object.clone()]));
                                                                                                {
                                                                                                    let source_ml_source2_concat_value17139 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value886 = shared_constructor_field1.borrow().clone(); ml_reference_value886 }, index_2, f64::INFINITY)));
                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { let ml_reference_value887 = source_ml_source2_concat_value17137.borrow().clone(); ml_reference_value887 }), { let ml_reference_value888 = source_ml_source2_concat_value17138.borrow().clone(); ml_reference_value888 }), { let ml_reference_value889 = source_ml_source2_concat_value17139.borrow().clone(); ml_reference_value889 })
                                                                                                }
                                                                                            }
                                                                                        }));
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_override17153.clone(), source_ml_source2_nominal_field17143.clone(), source_ml_source2_nominal_field17145.clone(), source_ml_source2_nominal_field17147.clone(), source_ml_source2_nominal_field17149.clone(), source_ml_source2_nominal_field17151.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let invalid_prototype = self::read_java_script_ordinary_heap_error(updated.clone());
                                                            if (invalid_prototype != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::JavaScriptCollectionCompletion::CollectionInvalid(Box::new(heap.clone()), invalid_prototype.clone())
                                                                } else {
                                                                    self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(updated.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
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
