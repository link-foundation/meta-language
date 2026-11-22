// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionCompletion;
use super::JavaScriptFunctionBindCall;
use super::JavaScriptFunctionBindState;
use super::JavaScriptFunctionBindTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Result;
use super::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_function_bind_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_weak_collection_operation_kind;
use super::read_java_script_weak_key_eligibility;
use super::replace_java_script_ordinary_object;
use super::retain_java_script_function_bind_metadata;
use super::source_ml_source1_temporary_allocate_java_script_weak_collection_loop294;
use super::source_ml_source1_temporary_execute_java_script_weak_collection_operation_loop297;
use super::source_ml_source1_temporary_find_java_script_weak_collection_key_loop292;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_function_bind_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptFunctionBindState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptFunctionBindTransition {
    {
        let error = {
            let source_ml_source2_logical7363 =
                self::read_java_script_function_bind_state_error(heap.clone(), state.clone());
            if !source_ml_source2_logical7363.0.is_empty() {
                source_ml_source2_logical7363.clone()
            } else {
                self::read_java_script_ordinary_value_error(
                    heap.clone(),
                    match completion.clone() {
                        self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                            source_ml_source2_data_property_value7361,
                        ) => {
                            let source_ml_source2_data_property_value7361 =
                                *source_ml_source2_data_property_value7361;
                            source_ml_source2_data_property_value7361.clone()
                        }
                        self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                            source_ml_source2_data_property_value7362,
                        ) => {
                            let source_ml_source2_data_property_value7362 =
                                *source_ml_source2_data_property_value7362;
                            source_ml_source2_data_property_value7362.clone()
                        }
                    },
                )
            }
        };
        if ((error != (self::ml_string::Text::from_units(vec![])))
            || match (
                match state.clone() {
                    self::JavaScriptFunctionBindState::FunctionBindState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value7364,
                    ) => ((source_ml_source2_data_property_value7364
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptFunctionBindCall>,)))
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
            self::JavaScriptFunctionBindTransition::FunctionBindInvalid(
                Box::new(heap.clone()),
                if !error.0.is_empty() {
                    error.clone()
                } else {
                    self::ml_string::Text::from_units(vec![
                        98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 115, 117, 109, 101, 115,
                        32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                        32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 109, 101, 116, 97, 100, 97,
                        116, 97, 32, 103, 101, 116, 116, 101, 114,
                    ])
                },
            )
        } else {
            match completion.clone() {
                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                    let value = *value;
                    self::retain_java_script_function_bind_metadata(
                        heap.clone(),
                        state.clone(),
                        value.clone(),
                    )
                }
                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                    let value_2 = *value_2;
                    self::JavaScriptFunctionBindTransition::FunctionBindThrow(
                        Box::new(heap.clone()),
                        Box::new(value_2.clone()),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_weak_collection_key(
    entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    if (match key.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => false,
        _ => true,
    } && match key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => false,
        _ => true,
    }) {
        (-1f64)
    } else {
        {
            let low = 0f64;
            {
                let high = {
                    let ml_reference_array578 = entries.clone();
                    let ml_reference_projection578 = ml_reference_array578.borrow().len() as f64;
                    ml_reference_projection578
                };
                {
                    let source_ml_source2_s7369 = self::source_ml_source1_temporary_find_java_script_weak_collection_key_loop292(entries.clone(), key.clone(), low, high);
                    match source_ml_source2_s7369.clone() {
                        self::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Result::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Done(low_2) => {
                            ((-low_2) - 1f64)
                        }
                        self::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Result::MlSource1TemporaryFindJavaScriptWeakCollectionKeyLoop292Return(source_ml_source1_temporary_result292) => {
                            source_ml_source1_temporary_result292
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_weak_collection(
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
                if !(self::ml_array::includes(
                    &{
                        let ml_reference_value579 =
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                self::ml_string::Text::from_units(vec![
                                    87, 101, 97, 107, 77, 97, 112,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    87, 101, 97, 107, 83, 101, 116,
                                ]),
                            ])))
                            .borrow()
                            .clone();
                        ml_reference_value579
                    },
                    kind,
                    0f64,
                    false,
                    |a, b| a == b,
                )) {
                    self::JavaScriptCollectionCompletion::CollectionInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            119, 101, 97, 107, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                            32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                            97, 108, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 97,
                            110, 100, 97, 114, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111,
                            110, 32, 107, 105, 110, 100,
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
                                let ml_reference_value580 =
                                    shared_constructor_field2.borrow().clone();
                                ml_reference_value580
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
                                119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111,
                                110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110,
                                101, 101, 100, 115, 32, 97, 32, 102, 114, 101, 115, 104, 32, 97,
                                99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 105,
                                100, 101, 110, 116, 105, 116, 121,
                            ]),
                        )
                    } else {
                        if (({
                            let ml_reference_array581 = shared_constructor_field1.clone();
                            let ml_reference_projection581 =
                                ml_reference_array581.borrow().len() as f64;
                            ml_reference_projection581
                        }) >= (4096f64))
                        {
                            self::JavaScriptCollectionCompletion::CollectionResourceLimit(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105,
                                    111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                                    32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32,
                                    115, 104, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116, 32,
                                    100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100,
                                    103, 101, 116,
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
                                        119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116,
                                        105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                        111, 110, 32, 107, 101, 101, 112, 115, 32, 105, 116, 115,
                                        32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110,
                                        97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112,
                                        101,
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
                                                    87, 101, 97, 107, 77, 97, 112,
                                                ]))) {
                                                self::JavaScriptOrdinaryObject::WeakMapObject(
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
                                                    std::rc::Rc::new(
                                                        std::cell::RefCell::new(Vec::<
                                                            self::JavaScriptWeakCollectionEntry,
                                                        >::new(
                                                        )),
                                                    ),
                                                )
                                            } else {
                                                self::JavaScriptOrdinaryObject::WeakSetObject(
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
                                                    std::rc::Rc::new(
                                                        std::cell::RefCell::new(Vec::<
                                                            self::JavaScriptWeakCollectionEntry,
                                                        >::new(
                                                        )),
                                                    ),
                                                )
                                            };
                                            {
                                                let index = 0f64;
                                                {
                                                    let index_2 = self::source_ml_source1_temporary_allocate_java_script_weak_collection_loop294(heap.clone(), identity.clone(), index);
                                                    {
                                                        let updated = {
                                                            let source_ml_source2_nominal_field7386 =
                                                                shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field7388 =
                                                                    shared_constructor_field2
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field7390 =
                                                                        shared_constructor_field3
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field7392 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field7394 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field7396 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override7398 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source2_concat_value7382 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value582 = shared_constructor_field1.borrow().clone(); ml_reference_value582 }, 0f64, index_2)));
                                                                                            {
                                                                                                let source_ml_source2_concat_value7383 = std::rc::Rc::new(std::cell::RefCell::new(vec![object.clone()]));
                                                                                                {
                                                                                                    let source_ml_source2_concat_value7384 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value583 = shared_constructor_field1.borrow().clone(); ml_reference_value583 }, index_2, f64::INFINITY)));
                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { let ml_reference_value584 = source_ml_source2_concat_value7382.borrow().clone(); ml_reference_value584 }), { let ml_reference_value585 = source_ml_source2_concat_value7383.borrow().clone(); ml_reference_value585 }), { let ml_reference_value586 = source_ml_source2_concat_value7384.borrow().clone(); ml_reference_value586 })
                                                                                                }
                                                                                            }
                                                                                        }));
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_override7398.clone(), source_ml_source2_nominal_field7388.clone(), source_ml_source2_nominal_field7390.clone(), source_ml_source2_nominal_field7392.clone(), source_ml_source2_nominal_field7394.clone(), source_ml_source2_nominal_field7396.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let failure = self::read_java_script_ordinary_heap_error(updated.clone());
                                                            if (failure == (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(updated.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
                                                                } else {
                                                                    self::JavaScriptCollectionCompletion::CollectionInvalid(Box::new(heap.clone()), failure.clone())
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
pub fn replace_java_script_weak_collection_entries(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    weak_entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>,
) -> self::JavaScriptOrdinaryHeap {
    if (match object.clone() {
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => false,
        _ => true,
    } && match object.clone() {
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => false,
        _ => true,
    }) {
        heap.clone()
    } else {
        {
            let identity = (((match object.clone() {
                self::JavaScriptOrdinaryObject::WeakMapObject(
                    source_ml_source2_data_property_value7411,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => Some((source_ml_source2_data_property_value7411.clone(),)),
                self::JavaScriptOrdinaryObject::WeakSetObject(
                    source_ml_source2_data_property_value7412,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => Some((source_ml_source2_data_property_value7412.clone(),)),
                _ => None::<(self::ml::Big,)>,
            })
            .clone()
            .expect("a proven present value"))
            .0)
                .clone();
            {
                let prototype = (((match object.clone() {
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        source_ml_source2_data_property_value7413,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value7413 =
                            *source_ml_source2_data_property_value7413;
                        Some((source_ml_source2_data_property_value7413.clone(),))
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        source_ml_source2_data_property_value7414,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value7414 =
                            *source_ml_source2_data_property_value7414;
                        Some((source_ml_source2_data_property_value7414.clone(),))
                    }
                    _ => None::<(self::JavaScriptSemanticValue,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0)
                    .clone();
                {
                    let extensible = ((match object.clone() {
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            _,
                            _,
                            source_ml_source2_data_property_value7415,
                            _,
                            _,
                            _,
                        ) => Some((source_ml_source2_data_property_value7415,)),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            _,
                            _,
                            source_ml_source2_data_property_value7416,
                            _,
                            _,
                            _,
                        ) => Some((source_ml_source2_data_property_value7416,)),
                        _ => None::<(bool,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0;
                    {
                        let callable = ((match object.clone() {
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value7417,
                                _,
                                _,
                            ) => Some((source_ml_source2_data_property_value7417,)),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value7418,
                                _,
                                _,
                            ) => Some((source_ml_source2_data_property_value7418,)),
                            _ => None::<(bool,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0;
                        {
                            let properties = (((match object.clone() {
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value7419,
                                    _,
                                ) => Some((source_ml_source2_data_property_value7419.clone(),)),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value7420,
                                    _,
                                ) => Some((source_ml_source2_data_property_value7420.clone(),)),
                                _ => {
                                    None::<(
                                        std::rc::Rc<
                                            std::cell::RefCell<
                                                Vec<self::JavaScriptOrdinaryProperty>,
                                            >,
                                        >,
                                    )>
                                }
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0)
                                .clone();
                            {
                                let replacement = if match object.clone() {
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => true,
                                    _ => false,
                                } {
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        identity.clone(),
                                        Box::new(prototype.clone()),
                                        extensible,
                                        callable,
                                        properties.clone(),
                                        weak_entries.clone(),
                                    )
                                } else {
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        identity.clone(),
                                        Box::new(prototype.clone()),
                                        extensible,
                                        callable,
                                        properties.clone(),
                                        weak_entries.clone(),
                                    )
                                };
                                self::replace_java_script_ordinary_object(
                                    heap.clone(),
                                    replacement.clone(),
                                )
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
pub fn read_java_script_weak_collection_entries(
    object: self::JavaScriptOrdinaryObject,
    kind: &self::ml_string::Text,
) -> Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>> {
    match object.clone() {
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, weak_entries) => {
            if ((*kind) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112])))
            {
                Some(weak_entries.clone())
            } else {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>>
            }
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, weak_entries_2) => {
            if ((*kind)
                == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])))
            {
                Some(weak_entries_2.clone())
            } else {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>>
            }
        }
        _ => None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>>,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_weak_collection_operation(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    operation: &self::ml_string::Text,
    args: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCollectionCompletion {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCollectionCompletion::CollectionInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let kind = self::read_java_script_weak_collection_operation_kind(operation);
                    if (kind == (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCollectionCompletion::CollectionInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111,
                                110, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101,
                                116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32,
                                109, 101, 116, 104, 111, 100, 32, 111, 119, 110, 101, 114,
                            ]),
                        )
                    } else {
                        if (({
                            let ml_reference_array587 = args.clone();
                            let ml_reference_projection587 =
                                ml_reference_array587.borrow().len() as f64;
                            ml_reference_projection587
                        }) > (4096f64))
                        {
                            self::JavaScriptCollectionCompletion::CollectionResourceLimit(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105,
                                    111, 110, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32,
                                    101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32,
                                    101, 120, 112, 108, 105, 99, 105, 116, 32, 101, 120, 101, 99,
                                    117, 116, 105, 111, 110, 32, 112, 114, 111, 102, 105, 108, 101,
                                ]),
                            )
                        } else {
                            {
                                let source_ml_source1_temporary_values295 =
                                    std::rc::Rc::new(std::cell::RefCell::new(
                                        self::ml_array::append(vec![receiver.clone()], {
                                            let ml_reference_value588 = args.borrow().clone();
                                            ml_reference_value588
                                        }),
                                    ));
                                {
                                    let source_ml_source1_temporary_index295 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s7437 = self::source_ml_source1_temporary_execute_java_script_weak_collection_operation_loop297(heap.clone(), source_ml_source1_temporary_values295.clone(), source_ml_source1_temporary_index295.clone());
                                        match source_ml_source2_s7437.clone() {
                                                self::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Result::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Done => {
                                                    {
                                                        let object = match receiver.clone() {
                                                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                                                {
                                                                    let source_ml_source2_array_read_value7441 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index7442 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                                        { let ml_reference_array589 = source_ml_source2_array_read_value7441.clone(); let ml_reference_position589 = source_ml_source2_array_read_index7442; let ml_reference_projection589 = self::ml_array::optional_at(&*ml_reference_array589.borrow(), self::ml_array::number_index(ml_reference_position589)); ml_reference_projection589 }
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                None::<self::JavaScriptOrdinaryObject>
                                                            }
                                                        };
                                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::JavaScriptCollectionCompletion::CollectionTypeError(Box::new(heap.clone()), operation.concat(&self::ml_string::Text::from_units(vec![32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])))
                                                        } else {
                                                            {
                                                                let stored = self::read_java_script_weak_collection_entries(object.clone().expect("a proven present value"), &(kind));
                                                                if match (stored.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptCollectionCompletion::CollectionTypeError(Box::new(heap.clone()), operation.concat(&self::ml_string::Text::from_units(vec![32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])))
                                                                } else {
                                                                    {
                                                                        let key = ({
                                                                            let source_ml_source2_array_read_index7444 = 0f64;
                                                                            { let ml_reference_array590 = args.clone(); let ml_reference_position590 = source_ml_source2_array_read_index7444; let ml_reference_projection590 = self::ml_array::optional_at(&*ml_reference_array590.borrow(), self::ml_array::number_index(ml_reference_position590)); ml_reference_projection590 }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                        {
                                                                            let eligibility = self::read_java_script_weak_key_eligibility(heap.clone(), key.clone());
                                                                            {
                                                                                let inserting = (((*operation) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 115, 101, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 46, 97, 100, 100]))));
                                                                                if (eligibility == (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))) {
                                                                                    self::JavaScriptCollectionCompletion::CollectionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 114, 32, 117, 110, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 102, 97, 99, 116]))
                                                                                } else {
                                                                                    if (eligibility == (self::ml_string::Text::from_units(vec![105, 110, 101, 108, 105, 103, 105, 98, 108, 101]))) {
                                                                                        if inserting {
                                                                                            self::JavaScriptCollectionCompletion::CollectionTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114, 32, 110, 111, 110, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121]))
                                                                                        } else {
                                                                                            self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(heap.clone()), Box::new(if ((*operation) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 103, 101, 116]))) {
                                                                                                self::JavaScriptSemanticValue::Undefined
                                                                                            } else {
                                                                                                self::JavaScriptSemanticValue::Boolean(false)
                                                                                            }))
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let at = self::find_java_script_weak_collection_key(stored.clone().expect("a proven present value"), key.clone());
                                                                                            {
                                                                                                let entry = {
                                                                                                    let source_ml_source2_array_read_value7445 = stored.clone().expect("a proven present value");
                                                                                                    { let ml_reference_array591 = source_ml_source2_array_read_value7445.clone(); let ml_reference_position591 = at; let ml_reference_projection591 = self::ml_array::optional_at(&*ml_reference_array591.borrow(), self::ml_array::number_index(ml_reference_position591)); ml_reference_projection591 }
                                                                                                };
                                                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 103, 101, 116]))) {
                                                                                                    self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(heap.clone()), Box::new(({
                                                                                                        let source_ml_source2_optional_chain_receiver7447 = (entry.clone(),);
                                                                                                        if match (source_ml_source2_optional_chain_receiver7447.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                            Some({
                                                                                                                let source_ml_source1_temporary_optional_chain296 = ((source_ml_source2_optional_chain_receiver7447.clone()).0).clone().expect("a proven present value");
                                                                                                                match source_ml_source1_temporary_optional_chain296.clone() {
                                                                                                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, source_ml_source2_data_property_value7448) => {
                                                                                                                        let source_ml_source2_data_property_value7448 = *source_ml_source2_data_property_value7448;
                                                                                                                        source_ml_source2_data_property_value7448.clone()
                                                                                                                    }
                                                                                                                }
                                                                                                            })
                                                                                                        } else {
                                                                                                            None::<self::JavaScriptSemanticValue>
                                                                                                        }
                                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                                                                                } else {
                                                                                                    if (((*operation) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 104, 97, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 46, 104, 97, 115])))) {
                                                                                                        self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Boolean((at >= (0f64)))))
                                                                                                    } else {
                                                                                                        if (((*operation) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 100, 101, 108, 101, 116, 101]))) || ((*operation) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 46, 100, 101, 108, 101, 116, 101])))) {
                                                                                                            self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(if (at < (0f64)) {
                                                                                                                heap.clone()
                                                                                                            } else {
                                                                                                                self::replace_java_script_weak_collection_entries(heap.clone(), object.clone().expect("a proven present value"), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values592 = { let ml_reference_value593 = (stored.clone().expect("a proven present value")).borrow().clone(); ml_reference_value593 }; let ml_callback592 = { let at = at; std::rc::Rc::new(move |x_: self::JavaScriptWeakCollectionEntry, index: f64| -> bool { (index != at) }) }; ml_values592.iter().cloned().enumerate().filter_map(|(ml_index592, ml_value592)| if ml_callback592(ml_value592.clone(), (ml_index592 as f64)) { Some(ml_value592) } else { None }).collect::<Vec<_>>() })))
                                                                                                            }), Box::new(self::JavaScriptSemanticValue::Boolean((at >= (0f64)))))
                                                                                                        } else {
                                                                                                            if ((at < (0f64)) && (({ let ml_reference_array594 = stored.clone().expect("a proven present value"); let ml_reference_projection594 = ml_reference_array594.borrow().len() as f64; ml_reference_projection594 }) >= (65536f64))) {
                                                                                                                self::JavaScriptCollectionCompletion::CollectionResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 108, 105, 118, 101, 32, 115, 108, 111, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let value = if (kind == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]))) {
                                                                                                                        ({
                                                                                                                            let source_ml_source2_array_read_index7450 = 1f64;
                                                                                                                            { let ml_reference_array595 = args.clone(); let ml_reference_position595 = source_ml_source2_array_read_index7450; let ml_reference_projection595 = self::ml_array::optional_at(&*ml_reference_array595.borrow(), self::ml_array::number_index(ml_reference_position595)); ml_reference_projection595 }
                                                                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                                                                                                    } else {
                                                                                                                        self::JavaScriptSemanticValue::Undefined
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let next = self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(Box::new(key.clone()), Box::new(value.clone()));
                                                                                                                        {
                                                                                                                            let weak_entries = if (at < (0f64)) {
                                                                                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                                    let source_ml_source2_concat_value7451 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value596 = (stored.clone().expect("a proven present value")).borrow().clone(); ml_reference_value596 }, 0f64, ((-at) - 1f64))));
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_concat_value7452 = std::rc::Rc::new(std::cell::RefCell::new(vec![next.clone()]));
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_concat_value7453 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value597 = (stored.clone().expect("a proven present value")).borrow().clone(); ml_reference_value597 }, ((-at) - 1f64), f64::INFINITY)));
                                                                                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptWeakCollectionEntry>::new(), { let ml_reference_value598 = source_ml_source2_concat_value7451.borrow().clone(); ml_reference_value598 }), { let ml_reference_value599 = source_ml_source2_concat_value7452.borrow().clone(); ml_reference_value599 }), { let ml_reference_value600 = source_ml_source2_concat_value7453.borrow().clone(); ml_reference_value600 })
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }))
                                                                                                                            } else {
                                                                                                                                std::rc::Rc::new(std::cell::RefCell::new({ let ml_values601 = { let ml_reference_value602 = (stored.clone().expect("a proven present value")).borrow().clone(); ml_reference_value602 }; let ml_callback601 = { let at = at; let next = next.clone(); std::rc::Rc::new(move |current: self::JavaScriptWeakCollectionEntry, index: f64| -> self::JavaScriptWeakCollectionEntry { if (index == at) {
                                                                                                                                    next.clone()
                                                                                                                                } else {
                                                                                                                                    current.clone()
                                                                                                                                } }) }; ml_values601.iter().cloned().enumerate().map(|(ml_index601, ml_value601)| ml_callback601(ml_value601, (ml_index601 as f64))).collect::<Vec<_>>() }))
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let updated = self::replace_java_script_weak_collection_entries(heap.clone(), object.clone().expect("a proven present value"), weak_entries.clone());
                                                                                                                                {
                                                                                                                                    let failure = self::read_java_script_ordinary_heap_error(updated.clone());
                                                                                                                                    if (failure == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                        self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(updated.clone()), Box::new(receiver.clone()))
                                                                                                                                    } else {
                                                                                                                                        self::JavaScriptCollectionCompletion::CollectionResourceLimit(Box::new(heap.clone()), failure.clone())
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
                                                self::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Result::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Return(source_ml_source1_temporary_result297) => {
                                                    let source_ml_source1_temporary_result297 = *source_ml_source1_temporary_result297;
                                                    source_ml_source1_temporary_result297.clone()
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
