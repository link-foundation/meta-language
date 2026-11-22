// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySliceCall;
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySliceTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::accept_final_set;
use super::accept_scalar;
use super::accept_species;
use super::allocation_request;
use super::copy_java_script_array_constructor_fact;
use super::finish_java_script_scalar_primitive;
use super::ml_string;
use super::read_java_script_array_slice_state_error;
use super::step_java_script_array_slice_property_access;
use super::step_java_script_array_species_selection;
use super::step_java_script_object_spread;
use super::step_java_script_scalar_protocol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_slice(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySliceState,
) -> self::JavaScriptArraySliceTransition {
    match state.clone() {
        self::JavaScriptArraySliceState::ArraySliceState(
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field1,
            _,
            _,
            _,
            _,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error = self::read_java_script_array_slice_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptArraySliceState::ArraySliceState(
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
                                source_ml_source2_data_property_value12334,
                            ) => ((source_ml_source2_data_property_value12334
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptArraySliceTransition::ArraySliceCall(
                            Box::new(heap.clone()),
                            Box::new(state.clone()),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArraySliceState::ArraySliceState(
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
                                        source_ml_source2_data_property_value12335,
                                    ) => ((source_ml_source2_data_property_value12335
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArraySliceCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArraySliceCall::ArraySliceCall(
                                        source_ml_source2_data_property_value12336,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value12336 =
                                            *source_ml_source2_data_property_value12336;
                                        source_ml_source2_data_property_value12336.clone()
                                    }
                                },
                            ),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArraySliceState::ArraySliceState(
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
                                        source_ml_source2_data_property_value12337,
                                    ) => ((source_ml_source2_data_property_value12337
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArraySliceCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArraySliceCall::ArraySliceCall(
                                        _,
                                        source_ml_source2_data_property_value12338,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value12338 =
                                            *source_ml_source2_data_property_value12338;
                                        source_ml_source2_data_property_value12338.clone()
                                    }
                                },
                            ),
                            match (match state.clone() {
                                self::JavaScriptArraySliceState::ArraySliceState(
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
                                    source_ml_source2_data_property_value12339,
                                ) => ((source_ml_source2_data_property_value12339
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,)))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value")
                            {
                                self::JavaScriptArraySliceCall::ArraySliceCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value12340,
                                ) => source_ml_source2_data_property_value12340.clone(),
                            },
                        )
                    } else {
                        if (shared_constructor_field3
                            == (self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ])))
                        {
                            if match (
                                match state.clone() {
                                    self::JavaScriptArraySliceState::ArraySliceState(
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
                                        source_ml_source2_data_property_value12343,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value12343
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptPrimitiveConversionState>,)
                                        }))
                                    .0)
                                        .clone(),
                                },
                                None::<()>,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110,
                                        115, 32, 105, 116, 115, 32, 115, 99, 97, 108, 97, 114, 32,
                                        99, 104, 105, 108, 100,
                                    ]),
                                )
                            } else {
                                self::accept_scalar(heap.clone(), directory.clone(), state.clone(), self::step_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12344, _, _, _, _, _, _) => {
                                        ((source_ml_source2_data_property_value12344.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value"), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                            }
                        } else {
                            if (shared_constructor_field3
                                == (self::ml_string::Text::from_units(vec![
                                    112, 114, 105, 109, 105, 116, 105, 118, 101,
                                ])))
                            {
                                self::accept_scalar(
                                    heap.clone(),
                                    directory.clone(),
                                    state.clone(),
                                    self::finish_java_script_scalar_primitive(
                                        &(self::ml_string::Text::from_units(vec![
                                            99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109,
                                            105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101,
                                            114,
                                        ])),
                                        shared_constructor_field2.clone(),
                                    ),
                                )
                            } else {
                                if (shared_constructor_field3
                                    == (self::ml_string::Text::from_units(vec![
                                        115, 112, 101, 99, 105, 101, 115,
                                    ])))
                                {
                                    if match (
                                        match state.clone() {
                                            self::JavaScriptArraySliceState::ArraySliceState(
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
                                                source_ml_source2_data_property_value12350,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => ((source_ml_source2_data_property_value12350
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    (None::<self::JavaScriptArraySpeciesState>,)
                                                }))
                                            .0)
                                                .clone(),
                                        },
                                        None::<()>,
                                    ) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                            Box::new(heap.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105,
                                                110, 115, 32, 105, 116, 115, 32, 115, 112, 101, 99,
                                                105, 101, 115, 32, 99, 104, 105, 108, 100,
                                            ]),
                                        )
                                    } else {
                                        self::accept_species(heap.clone(), state.clone(), self::step_java_script_array_species_selection(heap.clone(), directory.clone(), (match state.clone() {
                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12351, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value12351.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value")))
                                    }
                                } else {
                                    if (shared_constructor_field3
                                        == (self::ml_string::Text::from_units(vec![
                                            97, 108, 108, 111, 99, 97, 116, 101,
                                        ])))
                                    {
                                        self::allocation_request(heap.clone(), state.clone())
                                    } else {
                                        if (shared_constructor_field3
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 115, 116, 114, 117, 99, 116,
                                            ])))
                                        {
                                            {
                                                let constructor = if {
                                                    let source_ml_source2_own_property_operands12356 = (
                                                        state.clone(),
                                                        self::ml_string::Text::from_units(vec![
                                                            99, 111, 110, 115, 116, 114, 117, 99,
                                                            116, 111, 114,
                                                        ]),
                                                    );
                                                    {
                                                        let source_ml_source2_own_property_key12357 = ((source_ml_source2_own_property_operands12356.clone()).1,);
                                                        match (source_ml_source2_own_property_operands12356.clone()).0 {
                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_owned_constructor_field12358, source_ml_source2_owned_constructor_field12359, source_ml_source2_owned_constructor_field12360, source_ml_source2_owned_constructor_field12361, source_ml_source2_owned_constructor_field12362, source_ml_source2_owned_constructor_field12363, source_ml_source2_owned_constructor_field12364) => {
                                                                ((((((((((((((((((((((((((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![36]))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![101, 110, 100])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![101, 110, 100, 80, 114, 101, 115, 101, 110, 116])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 75, 101, 121])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![101, 110, 100, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![112, 104, 97, 115, 101])))) || (((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 75, 105, 110, 100])))) || ((((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match source_ml_source2_owned_constructor_field12358.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && match source_ml_source2_owned_constructor_field12359.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101]))) && match source_ml_source2_owned_constructor_field12360.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && match source_ml_source2_owned_constructor_field12361.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 70, 97, 99, 116]))) && match source_ml_source2_owned_constructor_field12362.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116]))) && match source_ml_source2_owned_constructor_field12363.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key12357.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) && match source_ml_source2_owned_constructor_field12364.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }))
                                                            }
                                                        }
                                                    }
                                                } {
                                                    match state.clone() {
                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12365, _, _, _) => {
                                                            ((source_ml_source2_data_property_value12365.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    }
                                                } else {
                                                    None::<self::JavaScriptSemanticValue>
                                                };
                                                {
                                                    let fact = match state.clone() {
                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12366, _, _) => {
                                                            ((source_ml_source2_data_property_value12366.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConstructorFact>,))).0).clone()
                                                        }
                                                    };
                                                    if (match (constructor.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    } || match (fact.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }) {
                                                        self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 113, 117, 101, 115, 116]))
                                                    } else {
                                                        self::JavaScriptArraySliceTransition::ArraySliceConstruct(Box::new(heap.clone()), Box::new(state.clone()), Box::new(constructor.clone().expect("a proven present value")), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Number(shared_constructor_field1)])), Box::new(self::copy_java_script_array_constructor_fact(fact.clone().expect("a proven present value"))))
                                                    }
                                                }
                                            }
                                        } else {
                                            if (shared_constructor_field3
                                                == (self::ml_string::Text::from_units(vec![
                                                    102, 105, 110, 105, 115, 104,
                                                ])))
                                            {
                                                if match (match state.clone() {
                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12370, _) => {
                                                        ((source_ml_source2_data_property_value12370.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116]))
                                                } else {
                                                    self::accept_final_set(state.clone(), self::step_java_script_object_spread(heap.clone(), (match state.clone() {
                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12371, _) => {
                                                            ((source_ml_source2_data_property_value12371.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                        }
                                                    }).clone().expect("a proven present value")))
                                                }
                                            } else {
                                                self::step_java_script_array_slice_property_access(
                                                    heap.clone(),
                                                    directory.clone(),
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
