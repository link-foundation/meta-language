// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::RustStandardCollectionEqualityOrigin;
use super::RustStandardCollectionEqualityState;
use super::RustStandardCollectionEqualityTransition;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionPacketTransition;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::RustStandardCollectionTransition;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_rust_standard_collection_equality_state_error;
use super::read_rust_standard_collection_equality_work;
use super::read_rust_standard_collection_kind;
use super::resolve_rust_standard_collection_owner;
use super::source_ml_source1_temporary_run_rust_standard_collection_equality_loop58;
use super::test_rust_standard_collection_scalar_equality;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_rust_standard_collection_equality(
    state: self::RustStandardCollectionEqualityState,
    equal: bool,
) -> self::RustStandardCollectionEqualityTransition {
    self::RustStandardCollectionEqualityTransition::RustCollectionEqualityCompleted(
        if (match match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                _,
                source_ml_source2_data_property_value793,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value793 =
                    *source_ml_source2_data_property_value793;
                source_ml_source2_data_property_value793.clone()
            }
        } {
            self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(
                _,
                _,
                source_ml_source2_data_property_value794,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source2_data_property_value794.clone(),
        }) == (self::ml_string::Text::from_units(vec![
            110, 111, 116, 32, 101, 113, 117, 97, 108,
        ])) {
            !equal
        } else {
            equal
        },
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_rust_standard_collection_equality(
    left: self::RustStandardCollectionState,
    right: self::RustStandardCollectionState,
    origin: self::RustStandardCollectionEqualityOrigin,
) -> self::RustStandardCollectionEqualityTransition {
    {
        let state = self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            Box::new(left.clone()),
            Box::new(right.clone()),
            Box::new(origin.clone()),
            0f64,
            0f64,
            self::ml_string::Text::from_units(vec![115, 101, 97, 114, 99, 104]),
            0f64,
        );
        {
            let error = self::read_rust_standard_collection_equality_state_error(state.clone());
            if error == (self::ml_string::Text::from_units(vec![])) {
                self::RustStandardCollectionEqualityTransition::RustCollectionEqualityContinue(
                    Box::new(state.clone()),
                )
            } else {
                self::RustStandardCollectionEqualityTransition::RustCollectionEqualityInvalid(
                    error.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_rust_standard_collection_equality(
    state: self::RustStandardCollectionEqualityState,
) -> self::RustStandardCollectionEqualityTransition {
    {
        let error = self::read_rust_standard_collection_equality_state_error(state.clone());
        if error == (self::ml_string::Text::from_units(vec![])) {
            self::advance_rust_standard_collection_equality(state.clone())
        } else {
            self::RustStandardCollectionEqualityTransition::RustCollectionEqualityInvalid(
                error.clone(),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn advance_rust_standard_collection_equality(
    state: self::RustStandardCollectionEqualityState,
) -> self::RustStandardCollectionEqualityTransition {
    if ((match match state.clone() {
        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            source_ml_source2_data_property_value799,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let source_ml_source2_data_property_value799 =
                *source_ml_source2_data_property_value799;
            source_ml_source2_data_property_value799.clone()
        }
    } {
        self::RustStandardCollectionState::RustStandardCollectionState(
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value800,
        ) => source_ml_source2_data_property_value800,
    })
    .len() as f64)
        != ((match match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                source_ml_source2_data_property_value801,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value801 =
                    *source_ml_source2_data_property_value801;
                source_ml_source2_data_property_value801.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value802,
            ) => source_ml_source2_data_property_value802,
        })
        .len() as f64)
    {
        self::finish_rust_standard_collection_equality(state.clone(), false)
    } else if (match state.clone() {
        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            _,
            _,
            _,
            source_ml_source2_data_property_value806,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value806,
    }) == ((match match state.clone() {
        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            source_ml_source2_data_property_value807,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let source_ml_source2_data_property_value807 =
                *source_ml_source2_data_property_value807;
            source_ml_source2_data_property_value807.clone()
        }
    } {
        self::RustStandardCollectionState::RustStandardCollectionState(
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value808,
        ) => source_ml_source2_data_property_value808,
    })
    .len() as f64)
    {
        self::finish_rust_standard_collection_equality(state.clone(), true)
    } else if (match state.clone() {
        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value812,
            _,
            _,
        ) => source_ml_source2_data_property_value812,
    }) == ((match match state.clone() {
        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            _,
            source_ml_source2_data_property_value813,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let source_ml_source2_data_property_value813 =
                *source_ml_source2_data_property_value813;
            source_ml_source2_data_property_value813.clone()
        }
    } {
        self::RustStandardCollectionState::RustStandardCollectionState(
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value814,
        ) => source_ml_source2_data_property_value814,
    })
    .len() as f64)
    {
        self::finish_rust_standard_collection_equality(state.clone(), false)
    } else {
        {
            let cost = self::read_rust_standard_collection_equality_work(state.clone());
            if cost
                > (9_007_199_254_740_991f64
                    - match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value816,
                        ) => source_ml_source2_data_property_value816,
                    })
            {
                self::RustStandardCollectionEqualityTransition::RustCollectionEqualityResourceLimit(
                    Box::new(state.clone()),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108,
                        105, 116, 121, 32, 99, 104, 97, 114, 103, 101, 100, 32, 119, 111, 114, 107,
                        32, 101, 120, 99, 101, 101, 100, 115, 32, 101, 120, 97, 99, 116, 32, 115,
                        111, 117, 114, 99, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101,
                        115,
                    ]),
                )
            } else {
                {
                    let left = self::ml_array::optional_at(&match match state.clone() {
                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value817, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value817 = *source_ml_source2_data_property_value817;
                                    source_ml_source2_data_property_value817.clone()
                                }
                            } {
                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value818) => {
                                    source_ml_source2_data_property_value818
                                }
                            }, self::ml_array::number_index(match state.clone() {
                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value819, _, _, _) => {
                                    source_ml_source2_data_property_value819
                                }
                            }));
                    {
                        let right = self::ml_array::optional_at(&match match state.clone() {
                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value820, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value820 = *source_ml_source2_data_property_value820;
                                        source_ml_source2_data_property_value820.clone()
                                    }
                                } {
                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value821) => {
                                        source_ml_source2_data_property_value821
                                    }
                                }, self::ml_array::number_index(match state.clone() {
                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value822, _, _) => {
                                        source_ml_source2_data_property_value822
                                    }
                                }));
                        {
                            let next = {
                                let source_ml_source2_nominal_spread824 = state.clone();
                                {
                                    let source_ml_source2_nominal_field825 = match source_ml_source2_nominal_spread824.clone() {
                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value826, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value826 = *source_ml_source2_data_property_value826;
                                                    source_ml_source2_data_property_value826.clone()
                                                }
                                            };
                                    {
                                        let source_ml_source2_nominal_field827 = match source_ml_source2_nominal_spread824.clone() {
                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value828, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value828 = *source_ml_source2_data_property_value828;
                                                        source_ml_source2_data_property_value828.clone()
                                                    }
                                                };
                                        {
                                            let source_ml_source2_nominal_field829 = match source_ml_source2_nominal_spread824.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value830, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value830 = *source_ml_source2_data_property_value830;
                                                            source_ml_source2_data_property_value830.clone()
                                                        }
                                                    };
                                            {
                                                let source_ml_source2_nominal_field831 = match source_ml_source2_nominal_spread824.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value832, _, _, _) => {
                                                                source_ml_source2_data_property_value832
                                                            }
                                                        };
                                                {
                                                    let source_ml_source2_nominal_field833 = match source_ml_source2_nominal_spread824.clone() {
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value834, _, _) => {
                                                                    source_ml_source2_data_property_value834
                                                                }
                                                            };
                                                    {
                                                        let source_ml_source2_nominal_field835 = match source_ml_source2_nominal_spread824.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value836, _) => {
                                                                        source_ml_source2_data_property_value836.clone()
                                                                    }
                                                                };
                                                        {
                                                            let source_ml_source2_nominal_field837 = match source_ml_source2_nominal_spread824.clone() {
                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value838) => {
                                                                            source_ml_source2_data_property_value838
                                                                        }
                                                                    };
                                                            {
                                                                let source_ml_source2_nominal_override839 = match state.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value823) => {
                                                                                source_ml_source2_data_property_value823
                                                                            }
                                                                        } + cost;
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field825), Box::new(source_ml_source2_nominal_field827), Box::new(source_ml_source2_nominal_field829), source_ml_source2_nominal_field831, source_ml_source2_nominal_field833, source_ml_source2_nominal_field835, source_ml_source2_nominal_override839)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            if (match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value841, _) => {
                                            source_ml_source2_data_property_value841.clone()
                                        }
                                    }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                                        {
                                            let left_value = {
                                                let source_ml_source2_nullish_field_receiver842 = (left.clone(),);
                                                if match (source_ml_source2_nullish_field_receiver842.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present843 = ((source_ml_source2_nullish_field_receiver842.clone()).0).clone().expect("a proven present value");
                                                        ((((source_ml_source2_nullish_field_present843).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41])).0, message_present: true }).raise()
                                                }
                                            };
                                            {
                                                let right_value = {
                                                    let source_ml_source2_nullish_field_receiver844 = (right.clone(),);
                                                    if match (source_ml_source2_nullish_field_receiver844.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source2_nullish_field_present845 = ((source_ml_source2_nullish_field_receiver844.clone()).0).clone().expect("a proven present value");
                                                            ((((source_ml_source2_nullish_field_present845).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41])).0, message_present: true }).raise()
                                                    }
                                                };
                                                {
                                                    let invalid = self::ml_string::Text::from_units(vec![109, 97, 112, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 112, 114, 101, 115, 101, 114, 118, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 118, 97, 108, 117, 101, 115]);
                                                    if match (left_value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::RustStandardCollectionEqualityTransition::RustCollectionEqualityInvalid(invalid.clone())
                                                    } else if match (right_value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::RustStandardCollectionEqualityTransition::RustCollectionEqualityInvalid(invalid.clone())
                                                        } else if (self::test_rust_standard_collection_scalar_equality(left_value.clone().expect("a proven present value"), right_value.clone().expect("a proven present value"))) {
                                                                self::RustStandardCollectionEqualityTransition::RustCollectionEqualityContinue(Box::new({
                                                                    let source_ml_source2_nominal_spread847 = next.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field848 = match source_ml_source2_nominal_spread847.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value849, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value849 = *source_ml_source2_data_property_value849;
                                                                                source_ml_source2_data_property_value849.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field850 = match source_ml_source2_nominal_spread847.clone() {
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value851, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value851 = *source_ml_source2_data_property_value851;
                                                                                    source_ml_source2_data_property_value851.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field852 = match source_ml_source2_nominal_spread847.clone() {
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value853, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value853 = *source_ml_source2_data_property_value853;
                                                                                        source_ml_source2_data_property_value853.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field854 = match source_ml_source2_nominal_spread847.clone() {
                                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value855, _, _, _) => {
                                                                                            source_ml_source2_data_property_value855
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field856 = match source_ml_source2_nominal_spread847.clone() {
                                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value857, _, _) => {
                                                                                                source_ml_source2_data_property_value857
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field858 = match source_ml_source2_nominal_spread847.clone() {
                                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value859, _) => {
                                                                                                    source_ml_source2_data_property_value859.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field860 = match source_ml_source2_nominal_spread847.clone() {
                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value861) => {
                                                                                                        source_ml_source2_data_property_value861
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override862 = match state.clone() {
                                                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value846, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value846
                                                                                                        }
                                                                                                    } + 1f64;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override863 = 0f64;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override864 = self::ml_string::Text::from_units(vec![115, 101, 97, 114, 99, 104]);
                                                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field848), Box::new(source_ml_source2_nominal_field850), Box::new(source_ml_source2_nominal_field852), source_ml_source2_nominal_override862, source_ml_source2_nominal_override863, source_ml_source2_nominal_override864, source_ml_source2_nominal_field860)
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
                                                            } else {
                                                                self::finish_rust_standard_collection_equality(state.clone(), false)
                                                            }
                                                }
                                            }
                                        }
                                    } else if (self::test_rust_standard_collection_scalar_equality({
                                            let source_ml_source2_nullish_field_receiver865 = (left.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver865.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present866 = ((source_ml_source2_nullish_field_receiver865.clone()).0).clone().expect("a proven present value");
                                                    (source_ml_source2_nullish_field_present866).0
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }, {
                                            let source_ml_source2_nullish_field_receiver867 = (right.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver867.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present868 = ((source_ml_source2_nullish_field_receiver867.clone()).0).clone().expect("a proven present value");
                                                    (source_ml_source2_nullish_field_present868).0
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                            }
                                        })) {if (self::read_rust_standard_collection_kind(&(self::resolve_rust_standard_collection_owner(&(match match match state.clone() {
                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value889, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value889 = *source_ml_source2_data_property_value889;
                                                    source_ml_source2_data_property_value889.clone()
                                                }
                                            } {
                                                self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value890, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value890 = *source_ml_source2_data_property_value890;
                                                    source_ml_source2_data_property_value890.clone()
                                                }
                                            } {
                                                self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value891, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value891.clone()
                                                }
                                            }), true, false)))) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                self::RustStandardCollectionEqualityTransition::RustCollectionEqualityContinue(Box::new({
                                                    let source_ml_source2_nominal_spread892 = next.clone();
                                                    {
                                                        let source_ml_source2_nominal_field893 = match source_ml_source2_nominal_spread892.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value894, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value894 = *source_ml_source2_data_property_value894;
                                                                source_ml_source2_data_property_value894.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field895 = match source_ml_source2_nominal_spread892.clone() {
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value896, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value896 = *source_ml_source2_data_property_value896;
                                                                    source_ml_source2_data_property_value896.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field897 = match source_ml_source2_nominal_spread892.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value898, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value898 = *source_ml_source2_data_property_value898;
                                                                        source_ml_source2_data_property_value898.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field899 = match source_ml_source2_nominal_spread892.clone() {
                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value900, _, _, _) => {
                                                                            source_ml_source2_data_property_value900
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field901 = match source_ml_source2_nominal_spread892.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value902, _, _) => {
                                                                                source_ml_source2_data_property_value902
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field903 = match source_ml_source2_nominal_spread892.clone() {
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value904, _) => {
                                                                                    source_ml_source2_data_property_value904.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field905 = match source_ml_source2_nominal_spread892.clone() {
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value906) => {
                                                                                        source_ml_source2_data_property_value906
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override907 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field893), Box::new(source_ml_source2_nominal_field895), Box::new(source_ml_source2_nominal_field897), source_ml_source2_nominal_field899, source_ml_source2_nominal_field901, source_ml_source2_nominal_override907, source_ml_source2_nominal_field905)
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
                                                self::RustStandardCollectionEqualityTransition::RustCollectionEqualityContinue(Box::new({
                                                    let source_ml_source2_nominal_spread909 = next.clone();
                                                    {
                                                        let source_ml_source2_nominal_field910 = match source_ml_source2_nominal_spread909.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value911, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value911 = *source_ml_source2_data_property_value911;
                                                                source_ml_source2_data_property_value911.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field912 = match source_ml_source2_nominal_spread909.clone() {
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value913, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value913 = *source_ml_source2_data_property_value913;
                                                                    source_ml_source2_data_property_value913.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field914 = match source_ml_source2_nominal_spread909.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value915, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value915 = *source_ml_source2_data_property_value915;
                                                                        source_ml_source2_data_property_value915.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field916 = match source_ml_source2_nominal_spread909.clone() {
                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value917, _, _, _) => {
                                                                            source_ml_source2_data_property_value917
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field918 = match source_ml_source2_nominal_spread909.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value919, _, _) => {
                                                                                source_ml_source2_data_property_value919
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field920 = match source_ml_source2_nominal_spread909.clone() {
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value921, _) => {
                                                                                    source_ml_source2_data_property_value921.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field922 = match source_ml_source2_nominal_spread909.clone() {
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value923) => {
                                                                                        source_ml_source2_data_property_value923
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override924 = match state.clone() {
                                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value908, _, _, _) => {
                                                                                            source_ml_source2_data_property_value908
                                                                                        }
                                                                                    } + 1f64;
                                                                                    {
                                                                                        let source_ml_source2_nominal_override925 = 0f64;
                                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field910), Box::new(source_ml_source2_nominal_field912), Box::new(source_ml_source2_nominal_field914), source_ml_source2_nominal_override924, source_ml_source2_nominal_override925, source_ml_source2_nominal_field920, source_ml_source2_nominal_field922)
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
                                            }} else {
                                            self::RustStandardCollectionEqualityTransition::RustCollectionEqualityContinue(Box::new({
                                                let source_ml_source2_nominal_spread870 = next.clone();
                                                {
                                                    let source_ml_source2_nominal_field871 = match source_ml_source2_nominal_spread870.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value872, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value872 = *source_ml_source2_data_property_value872;
                                                            source_ml_source2_data_property_value872.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field873 = match source_ml_source2_nominal_spread870.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value874, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value874 = *source_ml_source2_data_property_value874;
                                                                source_ml_source2_data_property_value874.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field875 = match source_ml_source2_nominal_spread870.clone() {
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value876, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value876 = *source_ml_source2_data_property_value876;
                                                                    source_ml_source2_data_property_value876.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field877 = match source_ml_source2_nominal_spread870.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value878, _, _, _) => {
                                                                        source_ml_source2_data_property_value878
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field879 = match source_ml_source2_nominal_spread870.clone() {
                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value880, _, _) => {
                                                                            source_ml_source2_data_property_value880
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field881 = match source_ml_source2_nominal_spread870.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value882, _) => {
                                                                                source_ml_source2_data_property_value882.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field883 = match source_ml_source2_nominal_spread870.clone() {
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value884) => {
                                                                                    source_ml_source2_data_property_value884
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override885 = match state.clone() {
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value869, _, _) => {
                                                                                        source_ml_source2_data_property_value869
                                                                                    }
                                                                                } + 1f64;
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field871), Box::new(source_ml_source2_nominal_field873), Box::new(source_ml_source2_nominal_field875), source_ml_source2_nominal_field877, source_ml_source2_nominal_override885, source_ml_source2_nominal_field881, source_ml_source2_nominal_field883)
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
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn run_rust_standard_collection_equality(
    state: self::RustStandardCollectionEqualityState,
    budget: f64,
) -> self::RustStandardCollectionEqualityTransition {
    if ((!(self::ml_math::is_safe_integer(budget)) || (budget < 0f64)) || (budget > 67_108_864f64))
        || {
            let source_ml_source2_same_value_operand926 = budget;
            {
                let source_ml_source2_same_value_operand927 = -0f64;
                ((((source_ml_source2_same_value_operand926)
                    .partial_cmp(&(source_ml_source2_same_value_operand927))
                    == Some(std::cmp::Ordering::Equal))
                    && ((source_ml_source2_same_value_operand926 != 0f64)
                        || ((1f64 / source_ml_source2_same_value_operand926)
                            == (1f64 / source_ml_source2_same_value_operand927))))
                    || ((source_ml_source2_same_value_operand926
                        != source_ml_source2_same_value_operand926)
                        && (source_ml_source2_same_value_operand927
                            != source_ml_source2_same_value_operand927)))
            }
        }
    {
        self::RustStandardCollectionEqualityTransition::RustCollectionEqualityInvalid(
            self::ml_string::Text::from_units(vec![
                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108, 105,
                116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120,
                112, 108, 105, 99, 105, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 110, 111,
                110, 110, 101, 103, 97, 116, 105, 118, 101, 32, 119, 111, 114, 107, 32, 98, 117,
                100, 103, 101, 116,
            ]),
        )
    } else {
        {
            let error = self::read_rust_standard_collection_equality_state_error(state.clone());
            if error == (self::ml_string::Text::from_units(vec![])) {
                {
                    let current = state.clone();
                    {
                        let remaining = budget;
                        self::source_ml_source1_temporary_run_rust_standard_collection_equality_loop58(current, remaining)
                    }
                }
            } else {
                self::RustStandardCollectionEqualityTransition::RustCollectionEqualityInvalid(
                    error.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_rust_standard_collection_equality(
    left: self::RustStandardCollectionState,
    right: self::RustStandardCollectionState,
    origin: self::RustStandardCollectionEqualityOrigin,
    budget: f64,
) -> self::RustStandardCollectionEqualityTransition {
    {
        let beginning = self::begin_rust_standard_collection_equality(
            left.clone(),
            right.clone(),
            origin.clone(),
        );
        match beginning.clone() {
            self::RustStandardCollectionEqualityTransition::RustCollectionEqualityContinue(
                state,
            ) => {
                let state = *state;
                self::run_rust_standard_collection_equality(state.clone(), budget)
            }
            self::RustStandardCollectionEqualityTransition::RustCollectionEqualityCompleted(
                source_ml_source2_f928,
            ) => beginning.clone(),
            self::RustStandardCollectionEqualityTransition::RustCollectionEqualityInvalid(
                source_ml_source2_f929,
            ) => beginning.clone(),
            self::RustStandardCollectionEqualityTransition::RustCollectionEqualityResourceLimit(
                source_ml_source2_f930,
                source_ml_source2_f931,
            ) => {
                let source_ml_source2_f930 = *source_ml_source2_f930;
                beginning.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_rust_standard_collection_packet(
    result: self::RustStandardCollectionTransition,
) -> self::RustStandardCollectionPacketTransition {
    match result.clone() {
        self::RustStandardCollectionTransition::RustCollectionCreated(source_ml_source2_f932) => {
            let source_ml_source2_f932 = *source_ml_source2_f932;
            self::RustStandardCollectionPacketTransition::RustCollectionInvalid(
                self::ml_string::Text::from_units(vec![
                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 109, 101, 116, 104, 111,
                    100, 32, 112, 97, 99, 107, 101, 116, 115, 32, 99, 97, 110, 110, 111, 116, 32,
                    115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 99, 111, 110, 115, 116,
                    114, 117, 99, 116, 111, 114, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111,
                    110,
                ]),
            )
        }
        self::RustStandardCollectionTransition::RustCollectionNormal(state, value, writeback) => {
            let state = *state;
            let value = *value;
            self::RustStandardCollectionPacketTransition::RustCollectionPacketNormal(
                (value.clone(), state.clone()),
                writeback,
            )
        }
        self::RustStandardCollectionTransition::RustCollectionInvalid(reason) => {
            self::RustStandardCollectionPacketTransition::RustCollectionInvalid(reason.clone())
        }
        self::RustStandardCollectionTransition::RustCollectionResourceLimit(
            result_state,
            reason_2,
        ) => {
            let state_2 = ((result_state
                .clone()
                .unwrap_or_else(|| (None::<self::RustStandardCollectionState>,)))
            .0)
                .clone();
            if match (state_2.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                {
                    let source_ml_source2_constructor_field933 = reason_2.clone();
                    self::RustStandardCollectionPacketTransition::RustCollectionResourceLimit(
                        None::<(Option<self::RustStandardCollectionState>,)>,
                        source_ml_source2_constructor_field933,
                    )
                }
            } else {
                {
                    let source_ml_source2_constructor_field934 =
                        Some(state_2.clone().expect("a proven present value"));
                    {
                        let source_ml_source2_constructor_field935 = reason_2.clone();
                        self::RustStandardCollectionPacketTransition::RustCollectionResourceLimit(
                            Some(source_ml_source2_constructor_field934),
                            source_ml_source2_constructor_field935,
                        )
                    }
                }
            }
        }
    }
}
