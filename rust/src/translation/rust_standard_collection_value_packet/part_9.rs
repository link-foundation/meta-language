// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41;
use super::RustStandardCollectionEqualityOrigin;
use super::RustStandardCollectionOrderingOrigin;
use super::RustStandardCollectionOrderingState;
use super::RustStandardCollectionOrderingTransition;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::begin_rust_standard_collection_ordering;
use super::compare_rust_standard_collection_scalars;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_rust_standard_collection_kind;
use super::read_rust_standard_collection_ordering_state_error;
use super::read_rust_standard_collection_ordering_work;
use super::read_rust_standard_collection_source_version_error;
use super::read_rust_standard_collection_state_error;
use super::resolve_rust_standard_collection_owner;
use super::source_ml_source1_temporary_run_rust_standard_collection_ordering_loop42;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn advance_rust_standard_collection_ordering(
    state: self::RustStandardCollectionOrderingState,
) -> self::RustStandardCollectionOrderingTransition {
    if (match state.clone() {
        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
            _,
            _,
            _,
            source_ml_source2_data_property_value469,
            _,
            _,
        ) => source_ml_source2_data_property_value469,
    }) == (self::ml_math::min(
        ((match match state.clone() {
            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                source_ml_source2_data_property_value470,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value470 =
                    *source_ml_source2_data_property_value470;
                source_ml_source2_data_property_value470.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value471,
            ) => source_ml_source2_data_property_value471,
        })
        .len() as f64),
        ((match match state.clone() {
            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                _,
                source_ml_source2_data_property_value472,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value472 =
                    *source_ml_source2_data_property_value472;
                source_ml_source2_data_property_value472.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value473,
            ) => source_ml_source2_data_property_value473,
        })
        .len() as f64),
    )) {
        self::RustStandardCollectionOrderingTransition::RustCollectionOrderingCompleted(
            if ((match match state.clone() {
                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                    source_ml_source2_data_property_value478,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value478 =
                        *source_ml_source2_data_property_value478;
                    source_ml_source2_data_property_value478.clone()
                }
            } {
                self::RustStandardCollectionState::RustStandardCollectionState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value479,
                ) => source_ml_source2_data_property_value479,
            })
            .len() as f64)
                == ((match match state.clone() {
                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                        _,
                        source_ml_source2_data_property_value480,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value480 =
                            *source_ml_source2_data_property_value480;
                        source_ml_source2_data_property_value480.clone()
                    }
                } {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value481,
                    ) => source_ml_source2_data_property_value481,
                })
                .len() as f64)
            {
                0f64
            } else if ((match match state.clone() {
                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                    source_ml_source2_data_property_value486,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value486 =
                        *source_ml_source2_data_property_value486;
                    source_ml_source2_data_property_value486.clone()
                }
            } {
                self::RustStandardCollectionState::RustStandardCollectionState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value487,
                ) => source_ml_source2_data_property_value487,
            })
            .len() as f64)
                < ((match match state.clone() {
                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                        _,
                        source_ml_source2_data_property_value488,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value488 =
                            *source_ml_source2_data_property_value488;
                        source_ml_source2_data_property_value488.clone()
                    }
                } {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value489,
                    ) => source_ml_source2_data_property_value489,
                })
                .len() as f64)
            {
                -1f64
            } else {
                1f64
            },
        )
    } else {
        {
            let first = self::ml_array::optional_at(
                &match match state.clone() {
                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                        source_ml_source2_data_property_value490,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value490 =
                            *source_ml_source2_data_property_value490;
                        source_ml_source2_data_property_value490.clone()
                    }
                } {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value491,
                    ) => source_ml_source2_data_property_value491,
                },
                self::ml_array::number_index(match state.clone() {
                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value492,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value492,
                }),
            );
            {
                let second = self::ml_array::optional_at(
                    &match match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            source_ml_source2_data_property_value493,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value493 =
                                *source_ml_source2_data_property_value493;
                            source_ml_source2_data_property_value493.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value494,
                        ) => source_ml_source2_data_property_value494,
                    },
                    self::ml_array::number_index(match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value495,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value495,
                    }),
                );
                {
                    let comparison = None::<()>;
                    {
                        let source_ml_source2_s506 = if (match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value508, _) => {
                                source_ml_source2_data_property_value508.clone()
                            }
                        }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                            {
                                let first_value = {
                                    let source_ml_source2_nullish_field_receiver509 = (first.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver509.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present510 = ((source_ml_source2_nullish_field_receiver509.clone()).0).clone().expect("a proven present value");
                                            ((((source_ml_source2_nullish_field_present510).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41])).0, message_present: true }).raise()
                                    }
                                };
                                {
                                    let second_value = {
                                        let source_ml_source2_nullish_field_receiver511 = (second.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver511.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source2_nullish_field_present512 = ((source_ml_source2_nullish_field_receiver511.clone()).0).clone().expect("a proven present value");
                                                ((((source_ml_source2_nullish_field_present512).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41])).0, message_present: true }).raise()
                                        }
                                    };
                                    {
                                        let invalid = self::ml_string::Text::from_units(vec![109, 97, 112, 32, 111, 114, 100, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 118, 97, 108, 117, 101, 115]);
                                        if match (first_value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41Return(Box::new(self::RustStandardCollectionOrderingTransition::RustCollectionOrderingInvalid(invalid.clone())))
                                        } else if match (second_value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41Return(Box::new(self::RustStandardCollectionOrderingTransition::RustCollectionOrderingInvalid(invalid.clone())))
                                            } else {
                                                {
                                                    let comparison_2 = self::compare_rust_standard_collection_scalars(first_value.clone().expect("a proven present value"), second_value.clone().expect("a proven present value"));
                                                    self::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41Next(comparison_2)
                                                }
                                            }
                                    }
                                }
                            }
                        } else {
                            {
                                let comparison_3 = self::compare_rust_standard_collection_scalars({
                                    let source_ml_source2_nullish_field_receiver513 = (first.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver513.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present514 = ((source_ml_source2_nullish_field_receiver513.clone()).0).clone().expect("a proven present value");
                                            (source_ml_source2_nullish_field_present514).0
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                    }
                                }, {
                                    let source_ml_source2_nullish_field_receiver515 = (second.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver515.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source2_nullish_field_present516 = ((source_ml_source2_nullish_field_receiver515.clone()).0).clone().expect("a proven present value");
                                            (source_ml_source2_nullish_field_present516).0
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                    }
                                });
                                self::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41Next(comparison_3)
                            }
                        };
                        match source_ml_source2_s506 {
                            self::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41Next(comparison_4) => {
                                {
let ml_library_inequality_1 = comparison_4 != 0f64 ;
if ml_library_inequality_1 {
                                    self::RustStandardCollectionOrderingTransition::RustCollectionOrderingCompleted(comparison_4)
                                } else {
                                    {
                                        let map = (self::read_rust_standard_collection_kind(&(match match match state.clone() {
                                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(source_ml_source2_data_property_value520, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value520 = *source_ml_source2_data_property_value520;
                                                source_ml_source2_data_property_value520.clone()
                                            }
                                        } {
                                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value521, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value521 = *source_ml_source2_data_property_value521;
                                                source_ml_source2_data_property_value521.clone()
                                            }
                                        } {
                                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value522, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value522.clone()
                                            }
                                        }))) == (self::ml_string::Text::from_units(vec![109, 97, 112]));
                                        {
                                            let next = {
                                                let source_ml_source2_nominal_spread529 = state.clone();
                                                {
                                                    let source_ml_source2_nominal_field530 = match source_ml_source2_nominal_spread529.clone() {
                                                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(source_ml_source2_data_property_value531, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value531 = *source_ml_source2_data_property_value531;
                                                            source_ml_source2_data_property_value531.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field532 = match source_ml_source2_nominal_spread529.clone() {
                                                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, source_ml_source2_data_property_value533, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value533 = *source_ml_source2_data_property_value533;
                                                                source_ml_source2_data_property_value533.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field534 = match source_ml_source2_nominal_spread529.clone() {
                                                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, source_ml_source2_data_property_value535, _, _, _) => {
                                                                    let source_ml_source2_data_property_value535 = *source_ml_source2_data_property_value535;
                                                                    source_ml_source2_data_property_value535.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field536 = match source_ml_source2_nominal_spread529.clone() {
                                                                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value537, _, _) => {
                                                                        source_ml_source2_data_property_value537
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field538 = match source_ml_source2_nominal_spread529.clone() {
                                                                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value539, _) => {
                                                                            source_ml_source2_data_property_value539.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field540 = match source_ml_source2_nominal_spread529.clone() {
                                                                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value541) => {
                                                                                source_ml_source2_data_property_value541
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override542 = match state.clone() {
                                                                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value523) => {
                                                                                    source_ml_source2_data_property_value523
                                                                                }
                                                                            } + self::read_rust_standard_collection_ordering_work(state.clone());
                                                                            {
                                                                                let source_ml_source2_nominal_override543 = match state.clone() {
                                                                                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value524, _, _) => {
                                                                                        source_ml_source2_data_property_value524
                                                                                    }
                                                                                } + if ((match state.clone() {
                                                                                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value526, _) => {
                                                                                        source_ml_source2_data_property_value526.clone()
                                                                                    }
                                                                                }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) || !map {
                                                                                    1f64
                                                                                } else {
                                                                                    0f64
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override544 = if ((match state.clone() {
                                                                                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value528, _) => {
                                                                                            source_ml_source2_data_property_value528.clone()
                                                                                        }
                                                                                    }) == (self::ml_string::Text::from_units(vec![107, 101, 121]))) && map {
                                                                                        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])
                                                                                    } else {
                                                                                        self::ml_string::Text::from_units(vec![107, 101, 121])
                                                                                    };
                                                                                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(Box::new(source_ml_source2_nominal_field530), Box::new(source_ml_source2_nominal_field532), Box::new(source_ml_source2_nominal_field534), source_ml_source2_nominal_override543, source_ml_source2_nominal_override544, source_ml_source2_nominal_override542)
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
                                            self::RustStandardCollectionOrderingTransition::RustCollectionOrderingContinue(Box::new(next))
                                        }
                                    }
                                }
}
                            }
                            self::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41::MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41Return(source_ml_source1_temporary_result41) => {
                                let source_ml_source1_temporary_result41 = *source_ml_source1_temporary_result41;
                                source_ml_source1_temporary_result41.clone()
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
pub fn step_rust_standard_collection_ordering(
    state: self::RustStandardCollectionOrderingState,
) -> self::RustStandardCollectionOrderingTransition {
    {
        let error = self::read_rust_standard_collection_ordering_state_error(state.clone());
        if error == (self::ml_string::Text::from_units(vec![])) {
            self::advance_rust_standard_collection_ordering(state.clone())
        } else {
            self::RustStandardCollectionOrderingTransition::RustCollectionOrderingInvalid(
                error.clone(),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn run_rust_standard_collection_ordering(
    state: self::RustStandardCollectionOrderingState,
    budget: f64,
) -> self::RustStandardCollectionOrderingTransition {
    if ((!(self::ml_math::is_safe_integer(budget)) || (budget < 0f64)) || (budget > 67_108_864f64))
        || {
            let source_ml_source2_same_value_operand545 = budget;
            {
                let source_ml_source2_same_value_operand546 = -0f64;
                ((((source_ml_source2_same_value_operand545)
                    .partial_cmp(&(source_ml_source2_same_value_operand546))
                    == Some(std::cmp::Ordering::Equal))
                    && ((source_ml_source2_same_value_operand545 != 0f64)
                        || ((1f64 / source_ml_source2_same_value_operand545)
                            == (1f64 / source_ml_source2_same_value_operand546))))
                    || ((source_ml_source2_same_value_operand545
                        != source_ml_source2_same_value_operand545)
                        && (source_ml_source2_same_value_operand546
                            != source_ml_source2_same_value_operand546)))
            }
        }
    {
        self::RustStandardCollectionOrderingTransition::RustCollectionOrderingInvalid(
            self::ml_string::Text::from_units(vec![
                111, 114, 100, 101, 114, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111,
                110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 98, 111, 117,
                110, 100, 101, 100, 32, 101, 120, 97, 99, 116, 32, 119, 111, 114, 107, 32, 98, 117,
                100, 103, 101, 116,
            ]),
        )
    } else {
        {
            let error = self::read_rust_standard_collection_ordering_state_error(state.clone());
            if error == (self::ml_string::Text::from_units(vec![])) {
                {
                    let current = state.clone();
                    {
                        let remaining = budget;
                        self::source_ml_source1_temporary_run_rust_standard_collection_ordering_loop42(current, remaining)
                    }
                }
            } else {
                self::RustStandardCollectionOrderingTransition::RustCollectionOrderingInvalid(
                    error.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_rust_standard_collection_ordering(
    left: self::RustStandardCollectionState,
    right: self::RustStandardCollectionState,
    origin: self::RustStandardCollectionOrderingOrigin,
    budget: f64,
) -> self::RustStandardCollectionOrderingTransition {
    {
        let initial = self::begin_rust_standard_collection_ordering(
            left.clone(),
            right.clone(),
            origin.clone(),
        );
        match initial.clone() {
            self::RustStandardCollectionOrderingTransition::RustCollectionOrderingContinue(
                state,
            ) => {
                let state = *state;
                self::run_rust_standard_collection_ordering(state.clone(), budget)
            }
            self::RustStandardCollectionOrderingTransition::RustCollectionOrderingCompleted(
                source_ml_source2_f547,
            ) => initial.clone(),
            self::RustStandardCollectionOrderingTransition::RustCollectionOrderingInvalid(
                source_ml_source2_f548,
            ) => initial.clone(),
            self::RustStandardCollectionOrderingTransition::RustCollectionOrderingResourceLimit(
                source_ml_source2_f549,
                source_ml_source2_f550,
            ) => {
                let source_ml_source2_f549 = *source_ml_source2_f549;
                initial.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_equality_origin_error(
    left: self::RustStandardCollectionState,
    right: self::RustStandardCollectionState,
    origin: self::RustStandardCollectionEqualityOrigin,
) -> self::ml_string::Text {
    {
        let first = self::read_rust_standard_collection_state_error(left.clone());
        {
            let ml_library_inequality_3 = first != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                first.clone()
            } else {
                {
                    let second = self::read_rust_standard_collection_state_error(right.clone());
                    {
                        let ml_library_inequality_2 =
                            second != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                    second.clone()
                } else if ((((((({
                        let source_ml_source2_optional_chain_receiver552 = (origin.clone(),);
                        {
                            let source_ml_source1_temporary_optional_chain43 = (source_ml_source2_optional_chain_receiver552).0;
                            match source_ml_source1_temporary_optional_chain43 {
                                self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(_, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 69, 113, 117, 97, 108, 105, 116, 121, 79, 114, 105, 103, 105, 110])
                                }
                            }
                        }
                    }) != (self::ml_string::Text::from_units(vec![114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 69, 113, 117, 97, 108, 105, 116, 121, 79, 114, 105, 103, 105, 110]))) || !(match origin.clone() {
                        self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(_, _, _, _, source_ml_source2_data_property_value553, _, _, _) => {
                            source_ml_source2_data_property_value553
                        }
                    })) || !(match origin.clone() {
                        self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(_, _, _, _, _, source_ml_source2_data_property_value554, _, _) => {
                            source_ml_source2_data_property_value554
                        }
                    })) || !(match origin.clone() {
                        self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(_, _, _, _, _, _, source_ml_source2_data_property_value555, _) => {
                            source_ml_source2_data_property_value555
                        }
                    })) || ((match origin.clone() {
                        self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(_, _, _, _, _, _, _, source_ml_source2_data_property_value557) => {
                            source_ml_source2_data_property_value557
                        }
                    }) != (false))) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![99, 111, 114, 101, 58, 58, 99, 109, 112, 58, 58, 80, 97, 114, 116, 105, 97, 108, 69, 113]), self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 99, 109, 112, 58, 58, 80, 97, 114, 116, 105, 97, 108, 69, 113])], &match origin.clone() {
                        self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(source_ml_source2_data_property_value558, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value558.clone()
                        }
                    }, 0f64, false, |a, b| a == b))) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108]), self::ml_string::Text::from_units(vec![110, 111, 116, 32, 101, 113, 117, 97, 108])], &match origin.clone() {
                        self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(_, _, source_ml_source2_data_property_value559, _, _, _, _, _) => {
                            source_ml_source2_data_property_value559.clone()
                        }
                    }, 0f64, false, |a, b| a == b)) {
                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 99, 108, 111, 115, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 80, 97, 114, 116, 105, 97, 108, 69, 113, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
                    } else {
                        {
                            let version = self::read_rust_standard_collection_source_version_error(match origin.clone() {
                                self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(_, _, _, source_ml_source2_data_property_value560, _, _, _, _) => {
                                    source_ml_source2_data_property_value560
                                }
                            });
                            {
let ml_library_inequality_1 = version != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                version.clone()
                            } else {
                                {
                                    let first_owner = self::resolve_rust_standard_collection_owner(&(match match left.clone() {
                                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value561, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value561 = *source_ml_source2_data_property_value561;
                                            source_ml_source2_data_property_value561.clone()
                                        }
                                    } {
                                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value562, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value562.clone()
                                        }
                                    }), true, false);
                                    {
                                        let second_owner = self::resolve_rust_standard_collection_owner(&(match match right.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value563, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value563 = *source_ml_source2_data_property_value563;
                                                source_ml_source2_data_property_value563.clone()
                                            }
                                        } {
                                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value564, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value564.clone()
                                            }
                                        }), true, false);
                                        if (((((first_owner != second_owner) || ((match left.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value567, _, _, _, _) => {
                                                source_ml_source2_data_property_value567.clone()
                                            }
                                        }) != (match right.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value568, _, _, _, _) => {
                                                source_ml_source2_data_property_value568.clone()
                                            }
                                        }))) || ((match left.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value571, _, _, _) => {
                                                source_ml_source2_data_property_value571.clone()
                                            }
                                        }) != (match right.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value572, _, _, _) => {
                                                source_ml_source2_data_property_value572.clone()
                                            }
                                        }))) || ((match left.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value575, _, _) => {
                                                source_ml_source2_data_property_value575
                                            }
                                        }) != (match right.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value576, _, _) => {
                                                source_ml_source2_data_property_value576
                                            }
                                        }))) || ((match match left.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value581, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value581 = *source_ml_source2_data_property_value581;
                                                source_ml_source2_data_property_value581.clone()
                                            }
                                        } {
                                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, source_ml_source2_data_property_value582, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value582.clone()
                                            }
                                        }) != (match match right.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value583, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value583 = *source_ml_source2_data_property_value583;
                                                source_ml_source2_data_property_value583.clone()
                                            }
                                        } {
                                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, source_ml_source2_data_property_value584, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value584.clone()
                                            }
                                        }))) || ((match origin.clone() {
                                            self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(_, source_ml_source2_data_property_value588, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value588.clone()
                                            }
                                        }) != (match match left.clone() {
                                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value589, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value589 = *source_ml_source2_data_property_value589;
                                                source_ml_source2_data_property_value589.clone()
                                            }
                                        } {
                                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, source_ml_source2_data_property_value590, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value590.clone()
                                            }
                                        })) {
                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 112, 114, 101, 115, 101, 114, 118, 101, 115, 32, 116, 104, 101, 32, 101, 120, 97, 99, 116, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 111, 119, 110, 101, 114, 32, 99, 111, 110, 99, 114, 101, 116, 101, 32, 100, 111, 109, 97, 105, 110, 115, 32, 97, 110, 100, 32, 112, 111, 105, 110, 116, 101, 114, 32, 119, 105, 100, 116, 104, 32, 119, 105, 116, 104, 105, 110, 32, 105, 116, 115, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 118, 101, 114, 115, 105, 111, 110, 115])
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
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
