// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39;
use super::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result;
use super::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40;
use super::RustStandardCollectionOrderingOrigin;
use super::RustStandardCollectionOrderingState;
use super::RustStandardCollectionOrderingTransition;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::compare_rust_standard_collection_scalars;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::ordering_work;
use super::read_rust_standard_collection_kind;
use super::read_rust_standard_collection_ordering_origin_error;
use super::source_ml_source1_temporary_read_rust_standard_collection_ordering_state_error_loop38;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_ordering_state_error(
    state: self::RustStandardCollectionOrderingState,
) -> self::ml_string::Text {
    {
        let ml_product_condition_1 = {
            let source_ml_source2_optional_chain_receiver354 = (state.clone(),);
            {
                let source_ml_source1_temporary_optional_chain37 =
                    (source_ml_source2_optional_chain_receiver354).0;
                match source_ml_source1_temporary_optional_chain37 {
                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => self::ml_string::Text::from_units(vec![
                        114, 117, 115, 116, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 79,
                        114, 100, 101, 114, 105, 110, 103, 83, 116, 97, 116, 101,
                    ]),
                }
            }
        };
        if ml_product_condition_1 {
            self::ml_string::Text::from_units(vec![
                111, 114, 100, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116,
                101, 32, 115, 111, 117, 114, 99, 101, 32, 115, 116, 97, 116, 101,
            ])
        } else {
            {
                let source = self::read_rust_standard_collection_ordering_origin_error(
                    match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            source_ml_source2_data_property_value355,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value355 =
                                *source_ml_source2_data_property_value355;
                            source_ml_source2_data_property_value355.clone()
                        }
                    },
                    match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            source_ml_source2_data_property_value356,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value356 =
                                *source_ml_source2_data_property_value356;
                            source_ml_source2_data_property_value356.clone()
                        }
                    },
                    match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            _,
                            source_ml_source2_data_property_value357,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value357 =
                                *source_ml_source2_data_property_value357;
                            source_ml_source2_data_property_value357.clone()
                        }
                    },
                );
                {
                    let ml_library_inequality_1 =
                        source != (self::ml_string::Text::from_units(vec![]));
                    if ml_library_inequality_1 {
                        source.clone()
                    } else {
                        {
                            let length = self::ml_math::min(((match match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(source_ml_source2_data_property_value358, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value358 = *source_ml_source2_data_property_value358;
                            source_ml_source2_data_property_value358.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value359) => {
                            source_ml_source2_data_property_value359
                        }
                    }).len() as f64), ((match match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, source_ml_source2_data_property_value360, _, _, _, _) => {
                            let source_ml_source2_data_property_value360 = *source_ml_source2_data_property_value360;
                            source_ml_source2_data_property_value360.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value361) => {
                            source_ml_source2_data_property_value361
                        }
                    }).len() as f64));
                            {
                                let map = (self::read_rust_standard_collection_kind(&(match match match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(source_ml_source2_data_property_value365, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value365 = *source_ml_source2_data_property_value365;
                                source_ml_source2_data_property_value365.clone()
                            }
                        } {
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value366, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value366 = *source_ml_source2_data_property_value366;
                                source_ml_source2_data_property_value366.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value367, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value367.clone()
                            }
                        }))) == (self::ml_string::Text::from_units(vec![109, 97, 112]));
                                if (((((((!(self::ml_math::is_safe_integer(match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value368, _, _) => {
                                source_ml_source2_data_property_value368
                            }
                        })) || ((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value370, _, _) => {
                                source_ml_source2_data_property_value370
                            }
                        }) < 0f64)) || ((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value372, _, _) => {
                                source_ml_source2_data_property_value372
                            }
                        }) > length)) || {
                            let source_ml_source2_same_value_operand374 = match state.clone() {
                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value373, _, _) => {
                                    source_ml_source2_data_property_value373
                                }
                            };
                            {
                                let source_ml_source2_same_value_operand375 = -0f64;
                                (((source_ml_source2_same_value_operand374 == source_ml_source2_same_value_operand375) && ((source_ml_source2_same_value_operand374 != 0f64) || ((1f64 / source_ml_source2_same_value_operand374) == (1f64 / source_ml_source2_same_value_operand375)))) || ((source_ml_source2_same_value_operand374 != source_ml_source2_same_value_operand374) && (source_ml_source2_same_value_operand375 != source_ml_source2_same_value_operand375)))
                            }
                        }) || !(self::ml_math::is_safe_integer(match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value376) => {
                                source_ml_source2_data_property_value376
                            }
                        }))) || ((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value378) => {
                                source_ml_source2_data_property_value378
                            }
                        }) < 0f64)) || {
                            let source_ml_source2_same_value_operand380 = match state.clone() {
                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value379) => {
                                    source_ml_source2_data_property_value379
                                }
                            };
                            {
                                let source_ml_source2_same_value_operand381 = -0f64;
                                (((source_ml_source2_same_value_operand380 == source_ml_source2_same_value_operand381) && ((source_ml_source2_same_value_operand380 != 0f64) || ((1f64 / source_ml_source2_same_value_operand380) == (1f64 / source_ml_source2_same_value_operand381)))) || ((source_ml_source2_same_value_operand380 != source_ml_source2_same_value_operand380) && (source_ml_source2_same_value_operand381 != source_ml_source2_same_value_operand381)))
                            }
                        }) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![107, 101, 121]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])], &match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value382, _) => {
                                source_ml_source2_data_property_value382.clone()
                            }
                        }, 0f64, false, |a, b| a == b))) || (((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value384, _) => {
                                source_ml_source2_data_property_value384.clone()
                            }
                        }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) && (!map || ((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value386, _, _) => {
                                source_ml_source2_data_property_value386
                            }
                        }) == length))) {
                            self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 98, 111, 117, 110, 100, 101, 100, 32, 101, 120, 97, 99, 116, 32, 99, 117, 114, 115, 111, 114, 115, 32, 99, 104, 97, 114, 103, 101, 100, 32, 119, 111, 114, 107, 32, 97, 110, 100, 32, 109, 97, 112, 32, 118, 97, 108, 117, 101, 32, 112, 104, 97, 115, 101])
                        } else {
                            {
                                let work = 0f64;
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source2_s387 = self::source_ml_source1_temporary_read_rust_standard_collection_ordering_state_error_loop38(state.clone(), map, work, index);
                                        match source_ml_source2_s387 {
                                            self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Done(work_2) => {
                                                {
                                                    let source_ml_source2_s408 = if (match state.clone() {
                                                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value410, _) => {
                                                            source_ml_source2_data_property_value410.clone()
                                                        }
                                                    }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                                                        {
                                                            let first = self::ml_array::optional_at(&match match state.clone() {
                                                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(source_ml_source2_data_property_value411, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value411 = *source_ml_source2_data_property_value411;
                                                                    source_ml_source2_data_property_value411.clone()
                                                                }
                                                            } {
                                                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value412) => {
                                                                    source_ml_source2_data_property_value412
                                                                }
                                                            }, self::ml_array::number_index(match state.clone() {
                                                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value413, _, _) => {
                                                                    source_ml_source2_data_property_value413
                                                                }
                                                            }));
                                                            {
                                                                let second = self::ml_array::optional_at(&match match state.clone() {
                                                                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, source_ml_source2_data_property_value414, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value414 = *source_ml_source2_data_property_value414;
                                                                        source_ml_source2_data_property_value414.clone()
                                                                    }
                                                                } {
                                                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value415) => {
                                                                        source_ml_source2_data_property_value415
                                                                    }
                                                                }, self::ml_array::number_index(match state.clone() {
                                                                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value416, _, _) => {
                                                                        source_ml_source2_data_property_value416
                                                                    }
                                                                }));
                                                                if (self::compare_rust_standard_collection_scalars({
                                                                    let source_ml_source2_nullish_field_receiver421 = (first.clone(),);
                                                                    if match (source_ml_source2_nullish_field_receiver421.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                        {
                                                                            let source_ml_source2_nullish_field_present422 = ((source_ml_source2_nullish_field_receiver421.clone()).0).clone().expect("a proven present value");
                                                                            (source_ml_source2_nullish_field_present422).0
                                                                        }
                                                                    } else {
                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                    }
                                                                }, {
                                                                    let source_ml_source2_nullish_field_receiver423 = (second.clone(),);
                                                                    if match (source_ml_source2_nullish_field_receiver423.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                        {
                                                                            let source_ml_source2_nullish_field_present424 = ((source_ml_source2_nullish_field_receiver423.clone()).0).clone().expect("a proven present value");
                                                                            (source_ml_source2_nullish_field_present424).0
                                                                        }
                                                                    } else {
                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                    }
                                                                })) != 0f64 {
                                                                    self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39Return(self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 118, 97, 108, 117, 101, 32, 112, 104, 97, 115, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 113, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 107, 101, 121, 115]))
                                                                } else {
                                                                    {
                                                                        let work_3 = work_2 + self::ordering_work({
                                                                            let source_ml_source2_nullish_field_receiver425 = (first.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver425.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present426 = ((source_ml_source2_nullish_field_receiver425.clone()).0).clone().expect("a proven present value");
                                                                                    (source_ml_source2_nullish_field_present426).0
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }, {
                                                                            let source_ml_source2_nullish_field_receiver427 = (second.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver427.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present428 = ((source_ml_source2_nullish_field_receiver427.clone()).0).clone().expect("a proven present value");
                                                                                    (source_ml_source2_nullish_field_present428).0
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        });
                                                                        self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39Next(work_3)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39Next(work_2)
                                                    };
                                                    match source_ml_source2_s408 {
                                                        self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39Next(work_4) => {
                                                            if work_4 == (match state.clone() {
                                                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value430) => {
                                                                    source_ml_source2_data_property_value430
                                                                }
                                                            }) {
                                                                self::ml_string::Text::from_units(vec![])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 99, 104, 97, 114, 103, 101, 100, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 101, 113, 117, 97, 108, 32, 112, 114, 101, 102, 105, 120])
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39Return(source_ml_source1_temporary_result39) => {
                                                            source_ml_source1_temporary_result39.clone()
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Return(source_ml_source1_temporary_result39_2) => {
                                                source_ml_source1_temporary_result39_2.clone()
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
pub fn begin_rust_standard_collection_ordering(
    left: self::RustStandardCollectionState,
    right: self::RustStandardCollectionState,
    origin: self::RustStandardCollectionOrderingOrigin,
) -> self::RustStandardCollectionOrderingTransition {
    {
        let state = self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
            Box::new(left.clone()),
            Box::new(right.clone()),
            Box::new(origin.clone()),
            0f64,
            self::ml_string::Text::from_units(vec![107, 101, 121]),
            0f64,
        );
        {
            let error = self::read_rust_standard_collection_ordering_state_error(state.clone());
            if error == (self::ml_string::Text::from_units(vec![])) {
                self::RustStandardCollectionOrderingTransition::RustCollectionOrderingContinue(
                    Box::new(state.clone()),
                )
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
pub fn read_rust_standard_collection_ordering_work(
    state: self::RustStandardCollectionOrderingState,
) -> f64 {
    if (match state.clone() {
        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
            _,
            _,
            _,
            source_ml_source2_data_property_value436,
            _,
            _,
        ) => source_ml_source2_data_property_value436,
    }) == (self::ml_math::min(
        ((match match state.clone() {
            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                source_ml_source2_data_property_value437,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value437 =
                    *source_ml_source2_data_property_value437;
                source_ml_source2_data_property_value437.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value438,
            ) => source_ml_source2_data_property_value438,
        })
        .len() as f64),
        ((match match state.clone() {
            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                _,
                source_ml_source2_data_property_value439,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value439 =
                    *source_ml_source2_data_property_value439;
                source_ml_source2_data_property_value439.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value440,
            ) => source_ml_source2_data_property_value440,
        })
        .len() as f64),
    )) {
        0f64
    } else {
        {
            let first = self::ml_array::optional_at(
                &match match state.clone() {
                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                        source_ml_source2_data_property_value441,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value441 =
                            *source_ml_source2_data_property_value441;
                        source_ml_source2_data_property_value441.clone()
                    }
                } {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value442,
                    ) => source_ml_source2_data_property_value442,
                },
                self::ml_array::number_index(match state.clone() {
                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value443,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value443,
                }),
            );
            {
                let second = self::ml_array::optional_at(
                    &match match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            source_ml_source2_data_property_value444,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value444 =
                                *source_ml_source2_data_property_value444;
                            source_ml_source2_data_property_value444.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value445,
                        ) => source_ml_source2_data_property_value445,
                    },
                    self::ml_array::number_index(match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value446,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value446,
                    }),
                );
                {
                    let source_ml_source2_s453 = if (match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value455,
                            _,
                        ) => source_ml_source2_data_property_value455.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        118, 97, 108, 117, 101,
                    ])) {
                        {
                            let first_value = {
                                let source_ml_source2_nullish_field_receiver456 = (first.clone(),);
                                if match (source_ml_source2_nullish_field_receiver456.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source2_nullish_field_present457 =
                                            ((source_ml_source2_nullish_field_receiver456.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        ((((source_ml_source2_nullish_field_present457).1)
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::RustStandardCollectionScalar>,)
                                            }))
                                        .0)
                                            .clone()
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (self::ml_string::Text::from_units(vec![
                                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32,
                                            112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32,
                                            111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101,
                                            100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39,
                                            118, 97, 108, 117, 101, 39, 41,
                                        ]))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            };
                            {
                                let second_value = {
                                    let source_ml_source2_nullish_field_receiver458 =
                                        (second.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver458.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present459 =
                                                ((source_ml_source2_nullish_field_receiver458
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            ((((source_ml_source2_nullish_field_present459).1)
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    (None::<self::RustStandardCollectionScalar>,)
                                                }))
                                            .0)
                                                .clone()
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 118, 97, 108,
                                                    117, 101, 39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                };
                                if (match (first_value.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    self::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40Next
                                } else if (match (second_value.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    self::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40Next
                                } else {
                                    self::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40Return(self::ordering_work(first_value.clone().expect("a proven present value"), second_value.clone().expect("a proven present value")))
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40Next
                    };
                    match source_ml_source2_s453 {
                        self::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40Next => {
                            self::ordering_work({
                                let source_ml_source2_nullish_field_receiver460 = (first.clone(),);
                                if match (source_ml_source2_nullish_field_receiver460.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present461 = ((source_ml_source2_nullish_field_receiver460.clone()).0).clone().expect("a proven present value");
                                        (source_ml_source2_nullish_field_present461).0
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                }
                            }, {
                                let source_ml_source2_nullish_field_receiver462 = (second.clone(),);
                                if match (source_ml_source2_nullish_field_receiver462.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present463 = ((source_ml_source2_nullish_field_receiver462.clone()).0).clone().expect("a proven present value");
                                        (source_ml_source2_nullish_field_present463).0
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                }
                            })
                        }
                        self::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40Return(source_ml_source1_temporary_result40) => {
                            source_ml_source1_temporary_result40
                        }
                    }
                }
            }
        }
    }
}
