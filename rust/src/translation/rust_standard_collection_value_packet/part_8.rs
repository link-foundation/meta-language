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
use super::read_rust_standard_collection_source_version_error;
use super::read_rust_standard_collection_state_error;
use super::resolve_rust_standard_collection_owner;
use super::source_ml_source1_temporary_read_rust_standard_collection_ordering_state_error_loop38;
use super::test_rust_standard_collection_ordered;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_ordering_origin_error(
    left: self::RustStandardCollectionState,
    right: self::RustStandardCollectionState,
    origin: self::RustStandardCollectionOrderingOrigin,
) -> self::ml_string::Text {
    {
        let first = self::read_rust_standard_collection_state_error(left.clone());
        {
            let ml_library_inequality_2 = first != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                first.clone()
            } else {
                {
                    let second = self::read_rust_standard_collection_state_error(right.clone());
                    {
                        let ml_library_inequality_1 =
                            second != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                    second.clone()
                } else if ((((!(self::test_rust_standard_collection_ordered(&(match match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value319, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value319 = *source_ml_source2_data_property_value319;
                            source_ml_source2_data_property_value319.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value320, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value320.clone()
                        }
                    }))) || ((self::resolve_rust_standard_collection_owner(&(match match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value325, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value325 = *source_ml_source2_data_property_value325;
                            source_ml_source2_data_property_value325.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value326, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value326.clone()
                        }
                    }), true, false)) != (self::resolve_rust_standard_collection_owner(&(match match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value327, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value327 = *source_ml_source2_data_property_value327;
                            source_ml_source2_data_property_value327.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value328, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value328.clone()
                        }
                    }), true, false)))) || ((match match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value333, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value333 = *source_ml_source2_data_property_value333;
                            source_ml_source2_data_property_value333.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, source_ml_source2_data_property_value334, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value334.clone()
                        }
                    }) != (match match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value335, _, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value335 = *source_ml_source2_data_property_value335;
                            source_ml_source2_data_property_value335.clone()
                        }
                    } {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, source_ml_source2_data_property_value336, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value336.clone()
                        }
                    }))) || ((match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value339, _, _, _, _) => {
                            source_ml_source2_data_property_value339.clone()
                        }
                    }) != (match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, source_ml_source2_data_property_value340, _, _, _, _) => {
                            source_ml_source2_data_property_value340.clone()
                        }
                    }))) || ((match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value343, _, _, _) => {
                            source_ml_source2_data_property_value343.clone()
                        }
                    }) != (match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value344, _, _, _) => {
                            source_ml_source2_data_property_value344.clone()
                        }
                    }))) || ((match left.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value347, _, _) => {
                            source_ml_source2_data_property_value347
                        }
                    }) != (match right.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value348, _, _) => {
                            source_ml_source2_data_property_value348
                        }
                    })) {
                        self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 66, 84, 114, 101, 101, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 99, 108, 111, 115, 101, 100, 32, 107, 101, 121, 47, 118, 97, 108, 117, 101, 32, 100, 111, 109, 97, 105, 110, 115])
                    } else if ((((((({
                            let source_ml_source2_optional_chain_receiver350 = (origin.clone(),);
                            {
                                let source_ml_source1_temporary_optional_chain36 = (source_ml_source2_optional_chain_receiver350).0;
                                match source_ml_source1_temporary_optional_chain36 {
                                    self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 79, 114, 100, 101, 114, 105, 110, 103, 79, 114, 105, 103, 105, 110])
                                    }
                                }
                            }
                        }) != (self::ml_string::Text::from_units(vec![114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 79, 114, 100, 101, 114, 105, 110, 103, 79, 114, 105, 103, 105, 110]))) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![99, 111, 114, 101, 58, 58, 99, 109, 112, 58, 58, 79, 114, 100]), self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 99, 109, 112, 58, 58, 79, 114, 100]), self::ml_string::Text::from_units(vec![99, 111, 114, 101, 58, 58, 99, 109, 112, 58, 58, 80, 97, 114, 116, 105, 97, 108, 79, 114, 100]), self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 99, 109, 112, 58, 58, 80, 97, 114, 116, 105, 97, 108, 79, 114, 100])], &match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(source_ml_source2_data_property_value351, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value351.clone()
                            }
                        }, 0f64, false, |a, b| a == b))) || ((match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, source_ml_source2_data_property_value355, _, _, _, _, _) => {
                                source_ml_source2_data_property_value355.clone()
                            }
                        }) != (match match left.clone() {
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value356, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value356 = *source_ml_source2_data_property_value356;
                                source_ml_source2_data_property_value356.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, source_ml_source2_data_property_value357, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value357.clone()
                            }
                        }))) || !(match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, source_ml_source2_data_property_value358, _, _, _) => {
                                source_ml_source2_data_property_value358
                            }
                        })) || !(match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, _, source_ml_source2_data_property_value359, _, _) => {
                                source_ml_source2_data_property_value359
                            }
                        })) || !(match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, _, _, source_ml_source2_data_property_value360, _) => {
                                source_ml_source2_data_property_value360
                            }
                        })) || ((match origin.clone() {
                            self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, _, _, _, _, source_ml_source2_data_property_value362) => {
                                source_ml_source2_data_property_value362
                            }
                        }) != (false)) {
                            self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 99, 108, 111, 115, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 79, 114, 100, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 119, 105, 116, 104, 111, 117, 116, 32, 99, 117, 115, 116, 111, 109, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110])
                        } else {
                            self::read_rust_standard_collection_source_version_error(match origin.clone() {
                                self::RustStandardCollectionOrderingOrigin::RustStandardCollectionOrderingOrigin(_, _, source_ml_source2_data_property_value363, _, _, _, _) => {
                                    source_ml_source2_data_property_value363
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_ordering_state_error(
    state: self::RustStandardCollectionOrderingState,
) -> self::ml_string::Text {
    {
        let ml_product_condition_1 = {
            let source_ml_source2_optional_chain_receiver365 = (state.clone(),);
            {
                let source_ml_source1_temporary_optional_chain37 =
                    (source_ml_source2_optional_chain_receiver365).0;
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
                            source_ml_source2_data_property_value366,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value366 =
                                *source_ml_source2_data_property_value366;
                            source_ml_source2_data_property_value366.clone()
                        }
                    },
                    match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            source_ml_source2_data_property_value367,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value367 =
                                *source_ml_source2_data_property_value367;
                            source_ml_source2_data_property_value367.clone()
                        }
                    },
                    match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            _,
                            source_ml_source2_data_property_value368,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value368 =
                                *source_ml_source2_data_property_value368;
                            source_ml_source2_data_property_value368.clone()
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
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(source_ml_source2_data_property_value369, _, _, _, _, _) => {
                            let source_ml_source2_data_property_value369 = *source_ml_source2_data_property_value369;
                            source_ml_source2_data_property_value369.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value370) => {
                            source_ml_source2_data_property_value370
                        }
                    }).len() as f64), ((match match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, source_ml_source2_data_property_value371, _, _, _, _) => {
                            let source_ml_source2_data_property_value371 = *source_ml_source2_data_property_value371;
                            source_ml_source2_data_property_value371.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value372) => {
                            source_ml_source2_data_property_value372
                        }
                    }).len() as f64));
                            {
                                let map = (self::read_rust_standard_collection_kind(&(match match match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(source_ml_source2_data_property_value376, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value376 = *source_ml_source2_data_property_value376;
                                source_ml_source2_data_property_value376.clone()
                            }
                        } {
                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value377, _, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value377 = *source_ml_source2_data_property_value377;
                                source_ml_source2_data_property_value377.clone()
                            }
                        } {
                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value378, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value378.clone()
                            }
                        }))) == (self::ml_string::Text::from_units(vec![109, 97, 112]));
                                if (((((((!(self::ml_math::is_safe_integer(match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value379, _, _) => {
                                source_ml_source2_data_property_value379
                            }
                        })) || ((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value381, _, _) => {
                                source_ml_source2_data_property_value381
                            }
                        }) < 0f64)) || ((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value383, _, _) => {
                                source_ml_source2_data_property_value383
                            }
                        }) > length)) || {
                            let source_ml_source2_same_value_operand385 = match state.clone() {
                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value384, _, _) => {
                                    source_ml_source2_data_property_value384
                                }
                            };
                            {
                                let source_ml_source2_same_value_operand386 = -0f64;
                                (((source_ml_source2_same_value_operand385 == source_ml_source2_same_value_operand386) && ((source_ml_source2_same_value_operand385 != 0f64) || ((1f64 / source_ml_source2_same_value_operand385) == (1f64 / source_ml_source2_same_value_operand386)))) || ((source_ml_source2_same_value_operand385 != source_ml_source2_same_value_operand385) && (source_ml_source2_same_value_operand386 != source_ml_source2_same_value_operand386)))
                            }
                        }) || !(self::ml_math::is_safe_integer(match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value387) => {
                                source_ml_source2_data_property_value387
                            }
                        }))) || ((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value389) => {
                                source_ml_source2_data_property_value389
                            }
                        }) < 0f64)) || {
                            let source_ml_source2_same_value_operand391 = match state.clone() {
                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value390) => {
                                    source_ml_source2_data_property_value390
                                }
                            };
                            {
                                let source_ml_source2_same_value_operand392 = -0f64;
                                (((source_ml_source2_same_value_operand391 == source_ml_source2_same_value_operand392) && ((source_ml_source2_same_value_operand391 != 0f64) || ((1f64 / source_ml_source2_same_value_operand391) == (1f64 / source_ml_source2_same_value_operand392)))) || ((source_ml_source2_same_value_operand391 != source_ml_source2_same_value_operand391) && (source_ml_source2_same_value_operand392 != source_ml_source2_same_value_operand392)))
                            }
                        }) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![107, 101, 121]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])], &match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value393, _) => {
                                source_ml_source2_data_property_value393.clone()
                            }
                        }, 0f64, false, |a, b| a == b))) || (((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value395, _) => {
                                source_ml_source2_data_property_value395.clone()
                            }
                        }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) && (!map || ((match state.clone() {
                            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value397, _, _) => {
                                source_ml_source2_data_property_value397
                            }
                        }) == length))) {
                            self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 98, 111, 117, 110, 100, 101, 100, 32, 101, 120, 97, 99, 116, 32, 99, 117, 114, 115, 111, 114, 115, 32, 99, 104, 97, 114, 103, 101, 100, 32, 119, 111, 114, 107, 32, 97, 110, 100, 32, 109, 97, 112, 32, 118, 97, 108, 117, 101, 32, 112, 104, 97, 115, 101])
                        } else {
                            {
                                let work = 0f64;
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source2_s398 = self::source_ml_source1_temporary_read_rust_standard_collection_ordering_state_error_loop38(state.clone(), map, work, index);
                                        match source_ml_source2_s398 {
                                            self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Done(work_2) => {
                                                {
                                                    let source_ml_source2_s419 = if (match state.clone() {
                                                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, source_ml_source2_data_property_value421, _) => {
                                                            source_ml_source2_data_property_value421.clone()
                                                        }
                                                    }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                                                        {
                                                            let first = self::ml_array::optional_at(&match match state.clone() {
                                                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(source_ml_source2_data_property_value422, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value422 = *source_ml_source2_data_property_value422;
                                                                    source_ml_source2_data_property_value422.clone()
                                                                }
                                                            } {
                                                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value423) => {
                                                                    source_ml_source2_data_property_value423
                                                                }
                                                            }, self::ml_array::number_index(match state.clone() {
                                                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value424, _, _) => {
                                                                    source_ml_source2_data_property_value424
                                                                }
                                                            }));
                                                            {
                                                                let second = self::ml_array::optional_at(&match match state.clone() {
                                                                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, source_ml_source2_data_property_value425, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value425 = *source_ml_source2_data_property_value425;
                                                                        source_ml_source2_data_property_value425.clone()
                                                                    }
                                                                } {
                                                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value426) => {
                                                                        source_ml_source2_data_property_value426
                                                                    }
                                                                }, self::ml_array::number_index(match state.clone() {
                                                                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, source_ml_source2_data_property_value427, _, _) => {
                                                                        source_ml_source2_data_property_value427
                                                                    }
                                                                }));
                                                                if (self::compare_rust_standard_collection_scalars({
                                                                    let source_ml_source2_nullish_field_receiver432 = (first.clone(),);
                                                                    if match (source_ml_source2_nullish_field_receiver432.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                        {
                                                                            let source_ml_source2_nullish_field_present433 = ((source_ml_source2_nullish_field_receiver432.clone()).0).clone().expect("a proven present value");
                                                                            (source_ml_source2_nullish_field_present433).0
                                                                        }
                                                                    } else {
                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                    }
                                                                }, {
                                                                    let source_ml_source2_nullish_field_receiver434 = (second.clone(),);
                                                                    if match (source_ml_source2_nullish_field_receiver434.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                        {
                                                                            let source_ml_source2_nullish_field_present435 = ((source_ml_source2_nullish_field_receiver434.clone()).0).clone().expect("a proven present value");
                                                                            (source_ml_source2_nullish_field_present435).0
                                                                        }
                                                                    } else {
                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                    }
                                                                })) != 0f64 {
                                                                    self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39Return(self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 118, 97, 108, 117, 101, 32, 112, 104, 97, 115, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 113, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 107, 101, 121, 115]))
                                                                } else {
                                                                    {
                                                                        let work_3 = work_2 + self::ordering_work({
                                                                            let source_ml_source2_nullish_field_receiver436 = (first.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver436.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present437 = ((source_ml_source2_nullish_field_receiver436.clone()).0).clone().expect("a proven present value");
                                                                                    (source_ml_source2_nullish_field_present437).0
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }, {
                                                                            let source_ml_source2_nullish_field_receiver438 = (second.clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver438.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present439 = ((source_ml_source2_nullish_field_receiver438.clone()).0).clone().expect("a proven present value");
                                                                                    (source_ml_source2_nullish_field_present439).0
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
                                                    match source_ml_source2_s419 {
                                                        self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39Next(work_4) => {
                                                            if work_4 == (match state.clone() {
                                                                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, _, _, _, _, source_ml_source2_data_property_value441) => {
                                                                    source_ml_source2_data_property_value441
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
            source_ml_source2_data_property_value447,
            _,
            _,
        ) => source_ml_source2_data_property_value447,
    }) == (self::ml_math::min(
        ((match match state.clone() {
            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                source_ml_source2_data_property_value448,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value448 =
                    *source_ml_source2_data_property_value448;
                source_ml_source2_data_property_value448.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value449,
            ) => source_ml_source2_data_property_value449,
        })
        .len() as f64),
        ((match match state.clone() {
            self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                _,
                source_ml_source2_data_property_value450,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value450 =
                    *source_ml_source2_data_property_value450;
                source_ml_source2_data_property_value450.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value451,
            ) => source_ml_source2_data_property_value451,
        })
        .len() as f64),
    )) {
        0f64
    } else {
        {
            let first = self::ml_array::optional_at(
                &match match state.clone() {
                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                        source_ml_source2_data_property_value452,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value452 =
                            *source_ml_source2_data_property_value452;
                        source_ml_source2_data_property_value452.clone()
                    }
                } {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value453,
                    ) => source_ml_source2_data_property_value453,
                },
                self::ml_array::number_index(match state.clone() {
                    self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value454,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value454,
                }),
            );
            {
                let second = self::ml_array::optional_at(
                    &match match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            source_ml_source2_data_property_value455,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value455 =
                                *source_ml_source2_data_property_value455;
                            source_ml_source2_data_property_value455.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value456,
                        ) => source_ml_source2_data_property_value456,
                    },
                    self::ml_array::number_index(match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value457,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value457,
                    }),
                );
                {
                    let source_ml_source2_s464 = if (match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value466,
                            _,
                        ) => source_ml_source2_data_property_value466.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        118, 97, 108, 117, 101,
                    ])) {
                        {
                            let first_value = {
                                let source_ml_source2_nullish_field_receiver467 = (first.clone(),);
                                if match (source_ml_source2_nullish_field_receiver467.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source2_nullish_field_present468 =
                                            ((source_ml_source2_nullish_field_receiver467.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        ((((source_ml_source2_nullish_field_present468).1)
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
                                    let source_ml_source2_nullish_field_receiver469 =
                                        (second.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver469.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present470 =
                                                ((source_ml_source2_nullish_field_receiver469
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            ((((source_ml_source2_nullish_field_present470).1)
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
                    match source_ml_source2_s464 {
                        self::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40::MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40Next => {
                            self::ordering_work({
                                let source_ml_source2_nullish_field_receiver471 = (first.clone(),);
                                if match (source_ml_source2_nullish_field_receiver471.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present472 = ((source_ml_source2_nullish_field_receiver471.clone()).0).clone().expect("a proven present value");
                                        (source_ml_source2_nullish_field_present472).0
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                }
                            }, {
                                let source_ml_source2_nullish_field_receiver473 = (second.clone(),);
                                if match (source_ml_source2_nullish_field_receiver473.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present474 = ((source_ml_source2_nullish_field_receiver473.clone()).0).clone().expect("a proven present value");
                                        (source_ml_source2_nullish_field_present474).0
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
