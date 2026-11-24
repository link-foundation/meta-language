// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptConstructorRealm;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::adapt_java_script_bound_construction;
use super::clear_java_script_construction_child;
use super::ml_array;
use super::ml_string;
use super::select_java_script_construction_root;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_construction_prototype_root(
    root: self::JavaScriptConstructorRealm,
    state: self::JavaScriptConstructionState,
    object_self: bool,
) -> Option<self::JavaScriptSemanticValue> {
    match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
            _,
            _,
            shared_constructor_field3,
            _,
            shared_constructor_field4,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match root.clone() {
            self::JavaScriptConstructorRealm::ConstructorRealm(
                _,
                shared_constructor_field1,
                shared_constructor_field2,
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
                if object_self {
                    {
                        let input = ({
                            let source_ml_source2_array_read_value19303 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_array_read_index19304 = 0f64;
                                {
                                    let ml_reference_array990 =
                                        source_ml_source2_array_read_value19303.clone();
                                    let ml_reference_position990 =
                                        source_ml_source2_array_read_index19304;
                                    let ml_reference_projection990 = self::ml_array::optional_at(
                                        &*ml_reference_array990.borrow(),
                                        self::ml_array::number_index(ml_reference_position990),
                                    );
                                    ml_reference_projection990
                                }
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match input.clone() {
                            self::JavaScriptSemanticValue::Boolean(_) => match root.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value19312,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value19312
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            },
                            self::JavaScriptSemanticValue::Number(_) => match root.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value19313,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value19313
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            },
                            self::JavaScriptSemanticValue::Integer(_) => match root.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value19314,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value19314
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            },
                            self::JavaScriptSemanticValue::Text(_) => match root.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value19315,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value19315
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            },
                            self::JavaScriptSemanticValue::SourceText(_, _) => match root.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value19316,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value19316
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            },
                            self::JavaScriptSemanticValue::Symbol(_) => match root.clone() {
                                self::JavaScriptConstructorRealm::ConstructorRealm(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value19317,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value19317
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            },
                            _ => Some(shared_constructor_field1.clone()),
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_switch984 =
                            shared_constructor_field4.clone();
                        if (source_ml_source1_temporary_switch984
                            == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])))
                        {
                            Some(shared_constructor_field2.clone())
                        } else {
                            if (source_ml_source1_temporary_switch984
                                == (self::ml_string::Text::from_units(vec![
                                    78, 117, 109, 98, 101, 114,
                                ])))
                            {
                                match root.clone() {
                                    self::JavaScriptConstructorRealm::ConstructorRealm(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value19321,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value19321
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptSemanticValue>,)
                                        }))
                                    .0)
                                        .clone(),
                                }
                            } else {
                                if (source_ml_source1_temporary_switch984
                                    == (self::ml_string::Text::from_units(vec![
                                        66, 111, 111, 108, 101, 97, 110,
                                    ])))
                                {
                                    match root.clone() {
                                        self::JavaScriptConstructorRealm::ConstructorRealm(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value19322,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value19322
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptSemanticValue>,)
                                            }))
                                        .0)
                                            .clone(),
                                    }
                                } else {
                                    if (source_ml_source1_temporary_switch984
                                        == (self::ml_string::Text::from_units(vec![
                                            83, 116, 114, 105, 110, 103,
                                        ])))
                                    {
                                        match root.clone() {
                                            self::JavaScriptConstructorRealm::ConstructorRealm(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source2_data_property_value19323,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => ((source_ml_source2_data_property_value19323
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    (None::<self::JavaScriptSemanticValue>,)
                                                }))
                                            .0)
                                                .clone(),
                                        }
                                    } else {
                                        if (source_ml_source1_temporary_switch984
                                            == (self::ml_string::Text::from_units(vec![
                                                77, 97, 112,
                                            ])))
                                        {
                                            match root.clone() {
                                                    self::JavaScriptConstructorRealm::ConstructorRealm(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19324, _, _, _) => {
                                                        ((source_ml_source2_data_property_value19324.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                }
                                        } else {
                                            if (source_ml_source1_temporary_switch984
                                                == (self::ml_string::Text::from_units(vec![
                                                    83, 101, 116,
                                                ])))
                                            {
                                                match root.clone() {
                                                        self::JavaScriptConstructorRealm::ConstructorRealm(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19325, _, _) => {
                                                            ((source_ml_source2_data_property_value19325.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    }
                                            } else {
                                                if (source_ml_source1_temporary_switch984
                                                    == (self::ml_string::Text::from_units(vec![
                                                        87, 101, 97, 107, 77, 97, 112,
                                                    ])))
                                                {
                                                    match root.clone() {
                                                            self::JavaScriptConstructorRealm::ConstructorRealm(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19326, _) => {
                                                                ((source_ml_source2_data_property_value19326.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                            }
                                                        }
                                                } else {
                                                    if (source_ml_source1_temporary_switch984
                                                        == (self::ml_string::Text::from_units(
                                                            vec![87, 101, 97, 107, 83, 101, 116],
                                                        )))
                                                    {
                                                        match root.clone() {
                                                                self::JavaScriptConstructorRealm::ConstructorRealm(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19327) => {
                                                                    ((source_ml_source2_data_property_value19327.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                }
                                                            }
                                                    } else {
                                                        Some(shared_constructor_field1.clone())
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_construction_prototype(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptConstructionState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptConstructionTransition {
    if !(match (
        match state.clone() {
            self::JavaScriptConstructionState::ConstructionState(
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
                source_ml_source2_data_property_value19329,
            ) => ((source_ml_source2_data_property_value19329
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
            .0)
                .clone(),
        },
        None::<()>,
    ) {
        (Some(_ml_left), Some(_ml_right)) => false,
        (None, None) => true,
        _ => false,
    }) {
        self::adapt_java_script_bound_construction(
            state.clone(),
            self::continue_java_script_construction_prototype(
                heap.clone(),
                directory.clone(),
                (match state.clone() {
                    self::JavaScriptConstructionState::ConstructionState(
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
                        source_ml_source2_data_property_value19330,
                    ) => ((source_ml_source2_data_property_value19330
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                    .0)
                        .clone(),
                })
                .clone()
                .expect("a proven present value"),
                value.clone(),
            ),
        )
    } else {
        {
            let base = {
                let source_ml_source2_nominal_spread19333 =
                    self::clear_java_script_construction_child(state.clone());
                match source_ml_source2_nominal_spread19333.clone() {
                    self::JavaScriptConstructionState::ConstructionState(
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
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        let shared_constructor_field6 = *shared_constructor_field6;
                        let shared_constructor_field7 = *shared_constructor_field7;
                        let shared_constructor_field8 = *shared_constructor_field8;
                        {
                            let source_ml_source2_nominal_field19334 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field19336 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field19338 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field19340 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field19342 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field19344 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field19346 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field19348 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field19350 =
                                                                shared_constructor_field9;
                                                            {
                                                                let source_ml_source2_nominal_field19352 =
                                                                    shared_constructor_field10
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field19354 =
                                                                        shared_constructor_field11
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field19356 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field19358 = shared_constructor_field13.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override19361 = if match value.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        true
                                                                                    }
                                                                                    _ => {
                                                                                        false
                                                                                    }
                                                                                } {
                                                                                    self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101])
                                                                                } else {
                                                                                    self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 102, 97, 108, 108, 98, 97, 99, 107])
                                                                                };
                                                                                self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19334.clone()), Box::new(source_ml_source2_nominal_field19336.clone()), source_ml_source2_nominal_field19338.clone(), source_ml_source2_nominal_override19361.clone(), source_ml_source2_nominal_field19342.clone(), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field19346.clone()), Box::new(source_ml_source2_nominal_field19348.clone()), source_ml_source2_nominal_field19350, source_ml_source2_nominal_field19352.clone(), source_ml_source2_nominal_field19354.clone(), source_ml_source2_nominal_field19356.clone(), source_ml_source2_nominal_field19358.clone())
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
            if match value.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => true,
                _ => false,
            } {
                self::JavaScriptConstructionTransition::ConstructionContinue(
                    Box::new(heap.clone()),
                    Box::new(base.clone()),
                )
            } else {
                self::select_java_script_construction_root(
                    heap.clone(),
                    directory.clone(),
                    base.clone(),
                    false,
                )
            }
        }
    }
}
