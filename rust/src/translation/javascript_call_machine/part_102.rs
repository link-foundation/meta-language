// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIteratorCompletion;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptObjectFromEntriesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::accept_java_script_object_from_entries_iterator;
use super::begin_java_script_synchronous_iterator_close;
use super::clear_java_script_object_from_entries_child;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn throw_java_script_object_from_entries(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectFromEntriesTransition {
    {
        let base = self::clear_java_script_object_from_entries_child(state.clone());
        match base.clone() {
            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
            ) => {
                let shared_constructor_field1 = *shared_constructor_field1;
                let shared_constructor_field2 = *shared_constructor_field2;
                let shared_constructor_field3 = *shared_constructor_field3;
                let shared_constructor_field4 = *shared_constructor_field4;
                let shared_constructor_field5 = *shared_constructor_field5;
                let shared_constructor_field7 = *shared_constructor_field7;
                let shared_constructor_field8 = *shared_constructor_field8;
                let shared_constructor_field9 = *shared_constructor_field9;
                let shared_constructor_field10 = *shared_constructor_field10;
                if (match (
                    match state.clone() {
                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                            source_ml_source2_data_property_value20446,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value20446
                            .clone()
                            .unwrap_or_else(|| {
                                (None::<self::JavaScriptSynchronousIteratorRecord>,)
                            }))
                        .0)
                            .clone(),
                    },
                    None::<()>,
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } || match (match state.clone() {
                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                        source_ml_source2_data_property_value20447,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value20447
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,)))
                    .0)
                        .clone(),
                })
                .clone()
                .expect("a proven present value")
                {
                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
                        _,
                        _,
                        source_ml_source2_data_property_value20448,
                    ) => source_ml_source2_data_property_value20448,
                }) {
                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesThrow(
                        Box::new(heap.clone()),
                        Box::new(value.clone()),
                    )
                } else {
                    {
                        let transition = self::begin_java_script_synchronous_iterator_close(
                            heap.clone(),
                            (match state.clone() {
                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                                    source_ml_source2_data_property_value20449,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value20449
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptSynchronousIteratorRecord>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value"),
                            self::JavaScriptIteratorCompletion::IteratorCompletion(
                                self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]),
                                Box::new(value.clone()),
                                (-1f64),
                            ),
                        );
                        self::accept_java_script_object_from_entries_iterator(
                            heap.clone(),
                            {
                                let source_ml_source2_nominal_field20451 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field20453 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field20455 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field20457 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field20459 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field20461 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20463 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20465 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20467 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20469 =
                                                                        shared_constructor_field10
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20471 = shared_constructor_field11.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20473 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20475 = shared_constructor_field13.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20477 = shared_constructor_field14.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20479 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20481 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field20483 = shared_constructor_field17.clone();
                                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20451.clone()), Box::new(source_ml_source2_nominal_field20453.clone()), Box::new(source_ml_source2_nominal_field20455.clone()), Box::new(source_ml_source2_nominal_field20457.clone()), Box::new(source_ml_source2_nominal_field20459.clone()), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), Box::new(source_ml_source2_nominal_field20463.clone()), Box::new(source_ml_source2_nominal_field20465.clone()), Box::new(source_ml_source2_nominal_field20467.clone()), Box::new(source_ml_source2_nominal_field20469.clone()), source_ml_source2_nominal_field20471.clone(), source_ml_source2_nominal_field20473.clone(), source_ml_source2_nominal_field20475.clone(), source_ml_source2_nominal_field20477.clone(), Some((Some(value.clone()),)), source_ml_source2_nominal_field20481.clone(), source_ml_source2_nominal_field20483.clone())
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
                            },
                            transition.clone(),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn fail_java_script_object_from_entries(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
    error_class: &self::ml_string::Text,
    reason: &self::ml_string::Text,
) -> self::JavaScriptObjectFromEntriesTransition {
    if (match (
        match state.clone() {
            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                source_ml_source2_data_property_value20487,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => ((source_ml_source2_data_property_value20487
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,)))
            .0)
                .clone(),
        },
        None::<()>,
    ) {
        (Some(_ml_left), Some(_ml_right)) => false,
        (None, None) => true,
        _ => false,
    } || match (match state.clone() {
        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
            source_ml_source2_data_property_value20488,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => ((source_ml_source2_data_property_value20488
            .clone()
            .unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,)))
        .0)
            .clone(),
    })
    .clone()
    .expect("a proven present value")
    {
        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
            _,
            _,
            source_ml_source2_data_property_value20489,
        ) => source_ml_source2_data_property_value20489,
    }) {
        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesError(
            Box::new(heap.clone()),
            error_class.clone(),
            reason.clone(),
        )
    } else {
        {
            let next = {
                let source_ml_source2_nominal_spread20490 =
                    self::clear_java_script_object_from_entries_child(state.clone());
                match source_ml_source2_nominal_spread20490.clone() {
                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        let shared_constructor_field3 = *shared_constructor_field3;
                        let shared_constructor_field4 = *shared_constructor_field4;
                        let shared_constructor_field5 = *shared_constructor_field5;
                        let shared_constructor_field7 = *shared_constructor_field7;
                        let shared_constructor_field8 = *shared_constructor_field8;
                        let shared_constructor_field9 = *shared_constructor_field9;
                        let shared_constructor_field10 = *shared_constructor_field10;
                        {
                            let source_ml_source2_nominal_field20491 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field20493 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field20495 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field20497 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field20499 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field20501 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field20503 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field20505 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field20507 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field20509 =
                                                                    shared_constructor_field10
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field20511 =
                                                                        shared_constructor_field11
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field20513 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field20515 = shared_constructor_field13.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field20517 = shared_constructor_field14.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field20519 = shared_constructor_field15.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field20521 = shared_constructor_field16.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field20523 = shared_constructor_field17.clone();
                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field20491.clone()), Box::new(source_ml_source2_nominal_field20493.clone()), Box::new(source_ml_source2_nominal_field20495.clone()), Box::new(source_ml_source2_nominal_field20497.clone()), Box::new(source_ml_source2_nominal_field20499.clone()), self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), Box::new(source_ml_source2_nominal_field20503.clone()), Box::new(source_ml_source2_nominal_field20505.clone()), Box::new(source_ml_source2_nominal_field20507.clone()), Box::new(source_ml_source2_nominal_field20509.clone()), source_ml_source2_nominal_field20511.clone(), source_ml_source2_nominal_field20513.clone(), source_ml_source2_nominal_field20515.clone(), source_ml_source2_nominal_field20517.clone(), source_ml_source2_nominal_field20519.clone(), Some((Some(error_class.clone()),)), Some((Some(reason.clone()),)))
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
            };
            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesNeedError(
                Box::new(heap.clone()),
                Box::new(next.clone()),
                error_class.clone(),
                reason.clone(),
            )
        }
    }
}
