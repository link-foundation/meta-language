// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorClassification;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesCall;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptArraySpeciesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Result;
use super::find_java_script_array_constructor_fact;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_species_state_error;
use super::source_ml_source1_temporary_read_java_script_constructor_authority_target_loop226;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn classify_java_script_array_species_constructor(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    candidate: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayConstructorClassification {
    match directory.clone() {
        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
            _,
            _,
            _,
            _,
            shared_constructor_field2,
            _,
        ) => {
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    if (match candidate.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => false,
                        _ => true,
                    } || !(((match candidate.clone() {
                        self::JavaScriptSemanticValue::Object(
                            _,
                            source_ml_source2_data_property_value4074,
                        ) => Some((source_ml_source2_data_property_value4074,)),
                        _ => None::<(bool,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0))
                    {
                        self::JavaScriptArrayConstructorClassification::ArrayConstructorNotConstructable
                    } else {
                        {
                            let target = self::read_java_script_constructor_authority_target(
                                heap.clone(),
                                candidate.clone(),
                            );
                            if ((match (target.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } || match target.clone().expect("a proven present value") {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            }) || !(((match target.clone().expect("a proven present value") {
                                self::JavaScriptSemanticValue::Object(
                                    _,
                                    source_ml_source2_data_property_value4077,
                                ) => Some((source_ml_source2_data_property_value4077,)),
                                _ => None::<(bool,)>,
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0))
                            {
                                self::JavaScriptArrayConstructorClassification::ArrayConstructorNotConstructable
                            } else {
                                {
                                    let object = {
                                        let source_ml_source2_array_read_value4081 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_array_read_index4082 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match target.clone().expect("a proven present value") {
                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value4080, _) => {
                                                    Some((source_ml_source2_data_property_value4080.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::ml::Big,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone());
                                            {
                                                let ml_reference_array364 =
                                                    source_ml_source2_array_read_value4081.clone();
                                                let ml_reference_position364 =
                                                    source_ml_source2_array_read_index4082;
                                                let ml_reference_projection364 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array364.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position364,
                                                        ),
                                                    );
                                                ml_reference_projection364
                                            }
                                        }
                                    };
                                    if (match (object.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ClosureObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => true,
                                        _ => false,
                                    }) {
                                        self::JavaScriptArrayConstructorClassification::ArrayConstructorNotConstructable
                                    } else {
                                        {
                                            let fact = {
                                                let source_ml_source2_array_read_value4096 =
                                                    shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_array_read_index4097 = self::find_java_script_array_constructor_fact(shared_constructor_field2.clone(), (((match target.clone().expect("a proven present value") {
                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value4095, _) => {
                                                            Some((source_ml_source2_data_property_value4095.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::ml::Big,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone());
                                                    {
                                                        let ml_reference_array365 =
                                                            source_ml_source2_array_read_value4096
                                                                .clone();
                                                        let ml_reference_position365 =
                                                            source_ml_source2_array_read_index4097;
                                                        let ml_reference_projection365 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array365.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position365,
                                                                ),
                                                            );
                                                        ml_reference_projection365
                                                    }
                                                }
                                            };
                                            if match (fact.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                {
                                                    let source_ml_source2_constructor_field4098 =
                                                        Some(
                                                            target
                                                                .clone()
                                                                .expect("a proven present value"),
                                                        );
                                                    self::JavaScriptArrayConstructorClassification::ArrayConstructorUnknown(Some((source_ml_source2_constructor_field4098.clone(),)))
                                                }
                                            } else {
                                                self::JavaScriptArrayConstructorClassification::ArrayConstructorKnown(Box::new(if (((((match target.clone().expect("a proven present value") {
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value4101, _) => {
                                                        Some((source_ml_source2_data_property_value4101.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone()) == ((((match candidate.clone() {
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value4102, _) => {
                                                        Some((source_ml_source2_data_property_value4102.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone())) {
                                                    fact.clone().expect("a proven present value")
                                                } else {
                                                    {
                                                        let source_ml_source2_constructor_field4104 = (((match candidate.clone() {
                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value4103, _) => {
                                                                Some((source_ml_source2_data_property_value4103.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml::Big,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone();
                                                        {
                                                            let source_ml_source2_constructor_field4106 = match fact.clone().expect("a proven present value") {
                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value4105, _, _, _) => {
                                                                    source_ml_source2_data_property_value4105
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_constructor_field4108 = match fact.clone().expect("a proven present value") {
                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value4107, _, _) => {
                                                                        source_ml_source2_data_property_value4107.clone()
                                                                    }
                                                                };
                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(source_ml_source2_constructor_field4104.clone(), source_ml_source2_constructor_field4106, source_ml_source2_constructor_field4108.clone(), false, None::<(Option<self::ml_string::Text>,)>)
                                                            }
                                                        }
                                                    }
                                                }))
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
pub fn read_java_script_known_array_constructor_fact(
    classification: self::JavaScriptArrayConstructorClassification,
) -> Option<self::JavaScriptArrayConstructorFact> {
    match classification.clone() {
        self::JavaScriptArrayConstructorClassification::ArrayConstructorKnown(fact) => {
            let fact = *fact;
            Some(fact.clone())
        }
        _ => None::<self::JavaScriptArrayConstructorFact>,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_constructor_authority_target(
    heap: self::JavaScriptOrdinaryHeap,
    candidate: self::JavaScriptSemanticValue,
) -> Option<self::JavaScriptSemanticValue> {
    {
        let depth = 0f64;
        {
            let source_ml_source2_s4110 = self::source_ml_source1_temporary_read_java_script_constructor_authority_target_loop226(heap.clone(), candidate.clone(), depth);
            match source_ml_source2_s4110.clone() {
                self::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Result::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Done => {
                    None::<self::JavaScriptSemanticValue>
                }
                self::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Result::MlSource1TemporaryReadJavaScriptConstructorAuthorityTargetLoop226Return(source_ml_source1_temporary_result226) => {
                    source_ml_source1_temporary_result226.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_species_selection(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    original: self::JavaScriptSemanticValue,
    length: f64,
) -> self::JavaScriptArraySpeciesTransition {
    match directory.clone() {
        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            {
                let state = {
                    let source_ml_source2_constructor_field4114 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_constructor_field4116 =
                            shared_constructor_field2.clone();
                        {
                            let source_ml_source2_constructor_field4118 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_constructor_field4120 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_constructor_field4121 =
                                        self::JavaScriptSemanticValue::Undefined;
                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(
                                        Box::new(original.clone()),
                                        length,
                                        source_ml_source2_constructor_field4114.clone(),
                                        Box::new(source_ml_source2_constructor_field4116.clone()),
                                        Box::new(source_ml_source2_constructor_field4118.clone()),
                                        Box::new(source_ml_source2_constructor_field4120.clone()),
                                        Box::new(source_ml_source2_constructor_field4121.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            98, 114, 97, 110, 100,
                                        ]),
                                        None::<(Option<self::JavaScriptArraySpeciesCall>,)>,
                                    )
                                }
                            }
                        }
                    }
                };
                {
                    let error = self::read_java_script_array_species_state_error(
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                    );
                    if (error == (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptArraySpeciesTransition::ArraySpeciesContinue(
                            Box::new(heap.clone()),
                            Box::new(state.clone()),
                        )
                    } else {
                        self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_array_species_default(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySpeciesState,
) -> self::JavaScriptArraySpeciesTransition {
    match state.clone() {
        self::JavaScriptArraySpeciesState::ArraySpeciesState(
            _,
            shared_constructor_field1,
            _,
            _,
            shared_constructor_field2,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            self::JavaScriptArraySpeciesTransition::ArraySpeciesAllocate(
                Box::new(heap.clone()),
                Box::new(state.clone()),
                Box::new(shared_constructor_field2.clone()),
                shared_constructor_field1,
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_species_get(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySpeciesState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArraySpeciesTransition {
    match state.clone() {
        self::JavaScriptArraySpeciesState::ArraySpeciesState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            _,
            shared_constructor_field7,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            {
                let candidate = if ((shared_constructor_field7
                    == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115])))
                    && match value.clone() {
                        self::JavaScriptSemanticValue::Null => true,
                        _ => false,
                    }) {
                    self::JavaScriptSemanticValue::Undefined
                } else {
                    value.clone()
                };
                {
                    let phase = if (shared_constructor_field7
                        == (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                        ]))) {
                        self::ml_string::Text::from_units(vec![
                            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 108, 97,
                            115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110,
                        ])
                    } else {
                        self::ml_string::Text::from_units(vec![
                            115, 112, 101, 99, 105, 101, 115, 32, 99, 108, 97, 115, 115, 105, 102,
                            105, 99, 97, 116, 105, 111, 110,
                        ])
                    };
                    self::JavaScriptArraySpeciesTransition::ArraySpeciesContinue(
                        Box::new(heap.clone()),
                        Box::new({
                            let source_ml_source2_constructor_field4130 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_constructor_field4132 =
                                    shared_constructor_field2;
                                {
                                    let source_ml_source2_constructor_field4134 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_constructor_field4136 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_constructor_field4138 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_constructor_field4140 =
                                                    shared_constructor_field6.clone();
                                                self::JavaScriptArraySpeciesState::ArraySpeciesState(
                                                    Box::new(
                                                        source_ml_source2_constructor_field4130
                                                            .clone(),
                                                    ),
                                                    source_ml_source2_constructor_field4132,
                                                    source_ml_source2_constructor_field4134.clone(),
                                                    Box::new(
                                                        source_ml_source2_constructor_field4136
                                                            .clone(),
                                                    ),
                                                    Box::new(
                                                        source_ml_source2_constructor_field4138
                                                            .clone(),
                                                    ),
                                                    Box::new(
                                                        source_ml_source2_constructor_field4140
                                                            .clone(),
                                                    ),
                                                    Box::new(candidate.clone()),
                                                    phase.clone(),
                                                    None::<(
                                                        Option<self::JavaScriptArraySpeciesCall>,
                                                    )>,
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                        }),
                    )
                }
            }
        }
    }
}
