// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorClassification;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesCall;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptArraySpeciesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::classify_java_script_array_species_constructor;
use super::continue_java_script_array_species_get;
use super::copy_java_script_array_constructor_fact;
use super::ml;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_array_species_state_error;
use super::read_java_script_known_array_constructor_fact;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;
use super::request_java_script_array_species_default;
use super::source_ml_source3_record_order;
use super::test_java_script_array_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_species_selection(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySpeciesState,
) -> self::JavaScriptArraySpeciesTransition {
    match state.clone() {
        self::JavaScriptArraySpeciesState::ArraySpeciesState(
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
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            {
                let error = self::read_java_script_array_species_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptArraySpeciesState::ArraySpeciesState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value4143,
                            ) => ((source_ml_source2_data_property_value4143
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptArraySpeciesTransition::ArraySpeciesCall(
                            Box::new(heap.clone()),
                            Box::new(state.clone()),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value4144,
                                    ) => ((source_ml_source2_data_property_value4144
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArraySpeciesCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(
                                        source_ml_source2_data_property_value4145,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value4145 =
                                            *source_ml_source2_data_property_value4145;
                                        source_ml_source2_data_property_value4145.clone()
                                    }
                                },
                            ),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value4146,
                                    ) => ((source_ml_source2_data_property_value4146
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArraySpeciesCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(
                                        _,
                                        source_ml_source2_data_property_value4147,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value4147 =
                                            *source_ml_source2_data_property_value4147;
                                        source_ml_source2_data_property_value4147.clone()
                                    }
                                },
                            ),
                            match (match state.clone() {
                                self::JavaScriptArraySpeciesState::ArraySpeciesState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value4148,
                                ) => ((source_ml_source2_data_property_value4148
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptArraySpeciesCall>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value")
                            {
                                self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value4149,
                                ) => source_ml_source2_data_property_value4149.clone(),
                            },
                        )
                    } else {
                        if (shared_constructor_field8
                            == (self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100])))
                        {
                            {
                                let brand = self::test_java_script_array_value(
                                    heap.clone(),
                                    shared_constructor_field1.clone(),
                                );
                                if (((({
                                    let ml_reference_value366 = brand.borrow().clone();
                                    ml_reference_value366
                                })
                                .0)
                                    .1)
                                    != (self::ml_string::Text::from_units(vec![])))
                                {
                                    self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(
                                        Box::new(heap.clone()),
                                        (({
                                            let ml_reference_value367 = brand.borrow().clone();
                                            ml_reference_value367
                                        })
                                        .0)
                                            .1,
                                    )
                                } else {
                                    if !((({
                                        let ml_reference_value368 = brand.borrow().clone();
                                        ml_reference_value368
                                    })
                                    .0)
                                        .0)
                                    {
                                        self::request_java_script_array_species_default(
                                            heap.clone(),
                                            state.clone(),
                                        )
                                    } else {
                                        self::JavaScriptArraySpeciesTransition::ArraySpeciesContinue(
                                            Box::new(heap.clone()),
                                            Box::new({
                                                let source_ml_source2_nominal_field4154 =
                                                    shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field4156 =
                                                        shared_constructor_field2;
                                                    {
                                                        let source_ml_source2_nominal_field4158 =
                                                            shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field4160 =
                                                                shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field4162 =
                                                                    shared_constructor_field5
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field4164 =
                                                                        shared_constructor_field6
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field4166 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field4168 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field4170 = shared_constructor_field9.clone();
                                                                                self::JavaScriptArraySpeciesState::ArraySpeciesState(Box::new(source_ml_source2_nominal_field4154.clone()), source_ml_source2_nominal_field4156, source_ml_source2_nominal_field4158.clone(), Box::new(source_ml_source2_nominal_field4160.clone()), Box::new(source_ml_source2_nominal_field4162.clone()), Box::new(source_ml_source2_nominal_field4164.clone()), Box::new(source_ml_source2_nominal_field4166.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]), source_ml_source2_nominal_field4170.clone())
                                                                            }
                                                                        }
                                                                    }
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
                        } else {
                            if ((shared_constructor_field8
                                == (self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                                ])))
                                || (shared_constructor_field8
                                    == (self::ml_string::Text::from_units(vec![
                                        115, 112, 101, 99, 105, 101, 115,
                                    ]))))
                            {
                                {
                                    let receiver = if (shared_constructor_field8
                                        == (self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                                        ]))) {
                                        shared_constructor_field1.clone()
                                    } else {
                                        shared_constructor_field7.clone()
                                    };
                                    {
                                        let key = if (shared_constructor_field8
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                                            ]))) {
                                            self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                                std::cell::RefCell::new(vec![
                                                    99f64, 111f64, 110f64, 115f64, 116f64, 114f64,
                                                    117f64, 99f64, 116f64, 111f64, 114f64,
                                                ]),
                                            ))
                                        } else {
                                            shared_constructor_field6.clone()
                                        };
                                        match receiver.clone() {
                                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                                {
                                                    let get = self::plan_java_script_ordinary_get(heap.clone(), identity.clone(), key.clone(), receiver.clone());
                                                    if (((({ let ml_reference_value369 = get.borrow().clone(); ml_reference_value369 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(Box::new(heap.clone()), (({ let ml_reference_value370 = get.borrow().clone(); ml_reference_value370 }).0).3)
                                                    } else {
                                                        if (((({ let ml_reference_value371 = get.borrow().clone(); ml_reference_value371 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                            {
                                                                let call = self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(Box::new((({ let ml_reference_value372 = get.borrow().clone(); ml_reference_value372 }).0).2), Box::new((({ let ml_reference_value373 = get.borrow().clone(); ml_reference_value373 }).0).4), (({ let ml_reference_value374 = get.borrow().clone(); ml_reference_value374 }).0).1);
                                                                self::JavaScriptArraySpeciesTransition::ArraySpeciesCall(Box::new(heap.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field4186 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field4188 = shared_constructor_field2;
                                                                        {
                                                                            let source_ml_source2_nominal_field4190 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field4192 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field4194 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4196 = shared_constructor_field6.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4198 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4200 = shared_constructor_field8.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field4202 = shared_constructor_field9.clone();
                                                                                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(Box::new(source_ml_source2_nominal_field4186.clone()), source_ml_source2_nominal_field4188, source_ml_source2_nominal_field4190.clone(), Box::new(source_ml_source2_nominal_field4192.clone()), Box::new(source_ml_source2_nominal_field4194.clone()), Box::new(source_ml_source2_nominal_field4196.clone()), Box::new(source_ml_source2_nominal_field4198.clone()), source_ml_source2_nominal_field4200.clone(), Some((Some(call.clone()),)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }), Box::new((({ let ml_reference_value375 = get.borrow().clone(); ml_reference_value375 }).0).2), Box::new((({ let ml_reference_value376 = get.borrow().clone(); ml_reference_value376 }).0).4), (({ let ml_reference_value377 = get.borrow().clone(); ml_reference_value377 }).0).1)
                                                            }
                                                        } else {
                                                            if (((({ let ml_reference_value378 = get.borrow().clone(); ml_reference_value378 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                            } else {
                                                                self::continue_java_script_array_species_get(heap.clone(), state.clone(), (({ let ml_reference_value379 = get.borrow().clone(); ml_reference_value379 }).0).5)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                            }
                                        }
                                    }
                                }
                            } else {
                                if {
                                    let source_ml_source2_s4206 = shared_constructor_field7.clone();
                                    match source_ml_source2_s4206.clone() {
                                        self::JavaScriptSemanticValue::Undefined => true,
                                        _ => false,
                                    }
                                } {
                                    self::request_java_script_array_species_default(
                                        heap.clone(),
                                        state.clone(),
                                    )
                                } else {
                                    {
                                        let classification =
                                            self::classify_java_script_array_species_constructor(
                                                heap.clone(),
                                                directory.clone(),
                                                shared_constructor_field7.clone(),
                                            );
                                        match classification.clone() {
                                            self::JavaScriptArrayConstructorClassification::ArrayConstructorUnknown(constructor) => {
                                                self::JavaScriptArraySpeciesTransition::ArraySpeciesNeedAuthority(Box::new(heap.clone()), Box::new(state.clone()), Box::new((((constructor.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()).clone().unwrap_or_else(|| shared_constructor_field7.clone())))
                                            }
                                            _ => {
                                                {
                                                    let fact = self::read_java_script_known_array_constructor_fact(classification.clone());
                                                    if (shared_constructor_field8 == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110]))) {
                                                        if (((!(match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match fact.clone().expect("a proven present value") {
                                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value4212, _, _, _) => {
                                                                source_ml_source2_data_property_value4212
                                                            }
                                                        }) && match fact.clone().expect("a proven present value") {
                                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value4213, _) => {
                                                                source_ml_source2_data_property_value4213
                                                            }
                                                        }) && ((match fact.clone().expect("a proven present value") {
                                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value4216, _, _) => {
                                                                source_ml_source2_data_property_value4216.clone()
                                                            }
                                                        }) != shared_constructor_field3)) {
                                                            self::request_java_script_array_species_default(heap.clone(), state.clone())
                                                        } else {
                                                            if {
                                                                let source_ml_source2_s4219 = shared_constructor_field7.clone();
                                                                match source_ml_source2_s4219.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                }
                                                            } {
                                                                self::JavaScriptArraySpeciesTransition::ArraySpeciesContinue(Box::new(heap.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field4224 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field4226 = shared_constructor_field2;
                                                                        {
                                                                            let source_ml_source2_nominal_field4228 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field4230 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field4232 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4234 = shared_constructor_field6.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4236 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field4238 = shared_constructor_field8.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field4240 = shared_constructor_field9.clone();
                                                                                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(Box::new(source_ml_source2_nominal_field4224.clone()), source_ml_source2_nominal_field4226, source_ml_source2_nominal_field4228.clone(), Box::new(source_ml_source2_nominal_field4230.clone()), Box::new(source_ml_source2_nominal_field4232.clone()), Box::new(source_ml_source2_nominal_field4234.clone()), Box::new(source_ml_source2_nominal_field4236.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field4240.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }))
                                                            } else {
                                                                self::JavaScriptArraySpeciesTransition::ArraySpeciesError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                            }
                                                        }
                                                    } else {
                                                        if (match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match fact.clone().expect("a proven present value") {
                                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value4243, _, _, _) => {
                                                                source_ml_source2_data_property_value4243
                                                            }
                                                        })) {
                                                            self::JavaScriptArraySpeciesTransition::ArraySpeciesError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                        } else {
                                                            self::JavaScriptArraySpeciesTransition::ArraySpeciesConstruct(Box::new(heap.clone()), Box::new(state.clone()), Box::new(shared_constructor_field7.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Number(shared_constructor_field2)])), Box::new(self::copy_java_script_array_constructor_fact(fact.clone().expect("a proven present value"))))
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
pub fn resume_java_script_array_species_getter(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySpeciesState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArraySpeciesTransition {
    {
        let error = self::read_java_script_array_species_state_error(
            heap.clone(),
            directory.clone(),
            state.clone(),
        );
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            if match (
                match state.clone() {
                    self::JavaScriptArraySpeciesState::ArraySpeciesState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value4246,
                    ) => ((source_ml_source2_data_property_value4246
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        115, 112, 101, 99, 105, 101, 115, 32, 114, 101, 115, 117, 109, 112, 116,
                        105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                        97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                        103, 101, 116, 116, 101, 114,
                    ]),
                )
            } else {
                {
                    let invalid = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match completion.clone() {
                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                source_ml_source2_data_property_value4247,
                            ) => {
                                let source_ml_source2_data_property_value4247 =
                                    *source_ml_source2_data_property_value4247;
                                source_ml_source2_data_property_value4247.clone()
                            }
                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                source_ml_source2_data_property_value4248,
                            ) => {
                                let source_ml_source2_data_property_value4248 =
                                    *source_ml_source2_data_property_value4248;
                                source_ml_source2_data_property_value4248.clone()
                            }
                        },
                    );
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(
                            Box::new(heap.clone()),
                            invalid.clone(),
                        )
                    } else {
                        match completion.clone() {
                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                value,
                            ) => {
                                let value = *value;
                                self::continue_java_script_array_species_get(
                                    heap.clone(),
                                    state.clone(),
                                    value.clone(),
                                )
                            }
                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                value_2,
                            ) => {
                                let value_2 = *value_2;
                                self::JavaScriptArraySpeciesTransition::ArraySpeciesThrow(
                                    Box::new(heap.clone()),
                                    Box::new(value_2.clone()),
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
pub fn read_java_script_descriptor_field(
    index: f64,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            self::ml_string::Text,
            self::ml_string::Text,
            std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
        ),
        Vec<f64>,
    )>,
> {
    if (index == (0f64)) {
        std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_field4251 = std::rc::Rc::new(std::cell::RefCell::new(vec![
                101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64,
            ]));
            {
                let source_ml_source3_record_field377 =
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source3_record_field378 =
                        self::ml_string::Text::from_units(vec![
                            101, 110, 117, 109, 101, 114, 97, 98, 108, 101,
                        ]);
                    {
                        let source_ml_source3_record_field379 = source_ml_source2_field4251.clone();
                        ((source_ml_source3_record_field377.clone(), source_ml_source3_record_field378.clone(), source_ml_source3_record_field379.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![107f64, 105f64, 110f64, 100f64], vec![110f64, 97f64, 109f64, 101f64], vec![117f64, 110f64, 105f64, 116f64, 115f64]], vec![1f64, 0f64, 2f64]))
                    }
                }
            }
        }))
    } else {
        if (index == (1f64)) {
            std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source2_field4254 = std::rc::Rc::new(std::cell::RefCell::new(vec![
                    99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64,
                    108f64, 101f64,
                ]));
                {
                    let source_ml_source3_record_field380 =
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source3_record_field381 =
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101,
                            ]);
                        {
                            let source_ml_source3_record_field382 =
                                source_ml_source2_field4254.clone();
                            ((source_ml_source3_record_field380.clone(), source_ml_source3_record_field381.clone(), source_ml_source3_record_field382.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![107f64, 105f64, 110f64, 100f64], vec![110f64, 97f64, 109f64, 101f64], vec![117f64, 110f64, 105f64, 116f64, 115f64]], vec![1f64, 0f64, 2f64]))
                        }
                    }
                }
            }))
        } else {
            if (index == (2f64)) {
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source2_field4257 =
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            118f64, 97f64, 108f64, 117f64, 101f64,
                        ]));
                    {
                        let source_ml_source3_record_field383 =
                            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source3_record_field384 =
                                self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                            {
                                let source_ml_source3_record_field385 =
                                    source_ml_source2_field4257.clone();
                                ((source_ml_source3_record_field383.clone(), source_ml_source3_record_field384.clone(), source_ml_source3_record_field385.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![107f64, 105f64, 110f64, 100f64], vec![110f64, 97f64, 109f64, 101f64], vec![117f64, 110f64, 105f64, 116f64, 115f64]], vec![1f64, 0f64, 2f64]))
                            }
                        }
                    }
                }))
            } else {
                if (index == (3f64)) {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_field4260 =
                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64,
                            ]));
                        {
                            let source_ml_source3_record_field386 =
                                self::ml_string::Text::from_units(vec![
                                    98, 111, 111, 108, 101, 97, 110,
                                ]);
                            {
                                let source_ml_source3_record_field387 =
                                    self::ml_string::Text::from_units(vec![
                                        119, 114, 105, 116, 97, 98, 108, 101,
                                    ]);
                                {
                                    let source_ml_source3_record_field388 =
                                        source_ml_source2_field4260.clone();
                                    ((source_ml_source3_record_field386.clone(), source_ml_source3_record_field387.clone(), source_ml_source3_record_field388.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![107f64, 105f64, 110f64, 100f64], vec![110f64, 97f64, 109f64, 101f64], vec![117f64, 110f64, 105f64, 116f64, 115f64]], vec![1f64, 0f64, 2f64]))
                                }
                            }
                        }
                    }))
                } else {
                    if (index == (4f64)) {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field4263 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    103f64, 101f64, 116f64,
                                ]));
                            {
                                let source_ml_source3_record_field389 =
                                    self::ml_string::Text::from_units(vec![
                                        97, 99, 99, 101, 115, 115, 111, 114,
                                    ]);
                                {
                                    let source_ml_source3_record_field390 =
                                        self::ml_string::Text::from_units(vec![103, 101, 116]);
                                    {
                                        let source_ml_source3_record_field391 =
                                            source_ml_source2_field4263.clone();
                                        ((source_ml_source3_record_field389.clone(), source_ml_source3_record_field390.clone(), source_ml_source3_record_field391.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![107f64, 105f64, 110f64, 100f64], vec![110f64, 97f64, 109f64, 101f64], vec![117f64, 110f64, 105f64, 116f64, 115f64]], vec![1f64, 0f64, 2f64]))
                                    }
                                }
                            }
                        }))
                    } else {
                        if (index == (5f64)) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field4266 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        115f64, 101f64, 116f64,
                                    ]));
                                {
                                    let source_ml_source3_record_field392 =
                                        self::ml_string::Text::from_units(vec![
                                            97, 99, 99, 101, 115, 115, 111, 114,
                                        ]);
                                    {
                                        let source_ml_source3_record_field393 =
                                            self::ml_string::Text::from_units(vec![115, 101, 116]);
                                        {
                                            let source_ml_source3_record_field394 =
                                                source_ml_source2_field4266.clone();
                                            ((source_ml_source3_record_field392.clone(), source_ml_source3_record_field393.clone(), source_ml_source3_record_field394.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![107f64, 105f64, 110f64, 100f64], vec![110f64, 97f64, 109f64, 101f64], vec![117f64, 110f64, 105f64, 116f64, 115f64]], vec![1f64, 0f64, 2f64]))
                                        }
                                    }
                                }
                            }))
                        } else {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_field4269 =
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let source_ml_source3_record_field395 =
                                        self::ml_string::Text::from_units(vec![
                                            105, 110, 118, 97, 108, 105, 100,
                                        ]);
                                    {
                                        let source_ml_source3_record_field396 =
                                            self::ml_string::Text::from_units(vec![]);
                                        {
                                            let source_ml_source3_record_field397 =
                                                source_ml_source2_field4269.clone();
                                            ((source_ml_source3_record_field395.clone(), source_ml_source3_record_field396.clone(), source_ml_source3_record_field397.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![107f64, 105f64, 110f64, 100f64], vec![110f64, 97f64, 109f64, 101f64], vec![117f64, 110f64, 105f64, 116f64, 115f64]], vec![1f64, 0f64, 2f64]))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_semantic_boolean(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone()))
        != (self::ml_string::Text::from_units(vec![])))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                84, 111, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 113, 117, 105, 114, 101,
                115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110,
                116, 105, 99, 32, 118, 97, 108, 117, 101,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Boolean(value_value) => value_value,
            self::JavaScriptSemanticValue::Number(value_value_2) => {
                ((value_value_2 != (0f64)) && !(value_value_2.is_nan()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_3) => {
                (value_value_3 != (self::ml::Big::from_i128(0)))
            }
            self::JavaScriptSemanticValue::Text(units) => {
                (({
                    let ml_reference_array380 = units.clone();
                    let ml_reference_projection380 = ml_reference_array380.borrow().len() as f64;
                    ml_reference_projection380
                }) != (0f64))
            }
            self::JavaScriptSemanticValue::SourceText(_, text) => ((text.length()) != (0f64)),
            self::JavaScriptSemanticValue::Symbol(_) => true,
            self::JavaScriptSemanticValue::Object(_, _) => true,
            _ => false,
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> std::rc::Rc<
    std::cell::RefCell<(
        (
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
        ),
        Vec<f64>,
    )>,
> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source2_field4279 = self::JavaScriptSemanticValue::Undefined;
        {
            let source_ml_source2_field4283 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source2_field4285 = self::JavaScriptSemanticValue::Undefined;
                {
                    let source_ml_source3_record_field398 = false;
                    {
                        let source_ml_source3_record_field399 = false;
                        {
                            let source_ml_source3_record_field400 = false;
                            {
                                let source_ml_source3_record_field401 = false;
                                {
                                    let source_ml_source3_record_field402 =
                                        source_ml_source2_field4283.clone();
                                    {
                                        let source_ml_source3_record_field403 = false;
                                        {
                                            let source_ml_source3_record_field404 =
                                                source_ml_source2_field4285.clone();
                                            {
                                                let source_ml_source3_record_field405 = false;
                                                {
                                                    let source_ml_source3_record_field406 =
                                                        source_ml_source2_field4279.clone();
                                                    {
                                                        let source_ml_source3_record_field407 =
                                                            false;
                                                        {
                                                            let source_ml_source3_record_field408 =
                                                                false;
                                                            {
                                                                let source_ml_source3_record_field409 =
                                                                    false;
                                                                ((source_ml_source3_record_field398, source_ml_source3_record_field399, source_ml_source3_record_field400, source_ml_source3_record_field401, source_ml_source3_record_field402.clone(), source_ml_source3_record_field403, source_ml_source3_record_field404.clone(), source_ml_source3_record_field405, source_ml_source3_record_field406.clone(), source_ml_source3_record_field407, source_ml_source3_record_field408, source_ml_source3_record_field409), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64, 3f64, 2f64, 9f64, 8f64, 11f64, 10f64, 5f64, 4f64, 7f64, 6f64]))
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
    }))
}
