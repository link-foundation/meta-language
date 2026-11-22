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
                source_ml_source2_data_property_value782,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value782 =
                    *source_ml_source2_data_property_value782;
                source_ml_source2_data_property_value782.clone()
            }
        } {
            self::RustStandardCollectionEqualityOrigin::RustStandardCollectionEqualityOrigin(
                _,
                _,
                source_ml_source2_data_property_value783,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source2_data_property_value783.clone(),
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
            source_ml_source2_data_property_value788,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let source_ml_source2_data_property_value788 =
                *source_ml_source2_data_property_value788;
            source_ml_source2_data_property_value788.clone()
        }
    } {
        self::RustStandardCollectionState::RustStandardCollectionState(
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value789,
        ) => source_ml_source2_data_property_value789,
    })
    .len() as f64)
        != ((match match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                source_ml_source2_data_property_value790,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value790 =
                    *source_ml_source2_data_property_value790;
                source_ml_source2_data_property_value790.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value791,
            ) => source_ml_source2_data_property_value791,
        })
        .len() as f64)
    {
        self::finish_rust_standard_collection_equality(state.clone(), false)
    } else if (match state.clone() {
        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            _,
            _,
            _,
            source_ml_source2_data_property_value795,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value795,
    }) == ((match match state.clone() {
        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            source_ml_source2_data_property_value796,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let source_ml_source2_data_property_value796 =
                *source_ml_source2_data_property_value796;
            source_ml_source2_data_property_value796.clone()
        }
    } {
        self::RustStandardCollectionState::RustStandardCollectionState(
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value797,
        ) => source_ml_source2_data_property_value797,
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
            source_ml_source2_data_property_value801,
            _,
            _,
        ) => source_ml_source2_data_property_value801,
    }) == ((match match state.clone() {
        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            _,
            source_ml_source2_data_property_value802,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let source_ml_source2_data_property_value802 =
                *source_ml_source2_data_property_value802;
            source_ml_source2_data_property_value802.clone()
        }
    } {
        self::RustStandardCollectionState::RustStandardCollectionState(
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value803,
        ) => source_ml_source2_data_property_value803,
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
                            source_ml_source2_data_property_value805,
                        ) => source_ml_source2_data_property_value805,
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
                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value806, _, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value806 = *source_ml_source2_data_property_value806;
                                    source_ml_source2_data_property_value806.clone()
                                }
                            } {
                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value807) => {
                                    source_ml_source2_data_property_value807
                                }
                            }, self::ml_array::number_index(match state.clone() {
                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value808, _, _, _) => {
                                    source_ml_source2_data_property_value808
                                }
                            }));
                    {
                        let right = self::ml_array::optional_at(&match match state.clone() {
                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value809, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value809 = *source_ml_source2_data_property_value809;
                                        source_ml_source2_data_property_value809.clone()
                                    }
                                } {
                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value810) => {
                                        source_ml_source2_data_property_value810
                                    }
                                }, self::ml_array::number_index(match state.clone() {
                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value811, _, _) => {
                                        source_ml_source2_data_property_value811
                                    }
                                }));
                        {
                            let next = {
                                let source_ml_source2_nominal_spread813 = state.clone();
                                {
                                    let source_ml_source2_nominal_field814 = match source_ml_source2_nominal_spread813.clone() {
                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value815, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value815 = *source_ml_source2_data_property_value815;
                                                    source_ml_source2_data_property_value815.clone()
                                                }
                                            };
                                    {
                                        let source_ml_source2_nominal_field816 = match source_ml_source2_nominal_spread813.clone() {
                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value817, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value817 = *source_ml_source2_data_property_value817;
                                                        source_ml_source2_data_property_value817.clone()
                                                    }
                                                };
                                        {
                                            let source_ml_source2_nominal_field818 = match source_ml_source2_nominal_spread813.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value819, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value819 = *source_ml_source2_data_property_value819;
                                                            source_ml_source2_data_property_value819.clone()
                                                        }
                                                    };
                                            {
                                                let source_ml_source2_nominal_field820 = match source_ml_source2_nominal_spread813.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value821, _, _, _) => {
                                                                source_ml_source2_data_property_value821
                                                            }
                                                        };
                                                {
                                                    let source_ml_source2_nominal_field822 = match source_ml_source2_nominal_spread813.clone() {
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value823, _, _) => {
                                                                    source_ml_source2_data_property_value823
                                                                }
                                                            };
                                                    {
                                                        let source_ml_source2_nominal_field824 = match source_ml_source2_nominal_spread813.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value825, _) => {
                                                                        source_ml_source2_data_property_value825.clone()
                                                                    }
                                                                };
                                                        {
                                                            let source_ml_source2_nominal_field826 = match source_ml_source2_nominal_spread813.clone() {
                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value827) => {
                                                                            source_ml_source2_data_property_value827
                                                                        }
                                                                    };
                                                            {
                                                                let source_ml_source2_nominal_override828 = match state.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value812) => {
                                                                                source_ml_source2_data_property_value812
                                                                            }
                                                                        } + cost;
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field814), Box::new(source_ml_source2_nominal_field816), Box::new(source_ml_source2_nominal_field818), source_ml_source2_nominal_field820, source_ml_source2_nominal_field822, source_ml_source2_nominal_field824, source_ml_source2_nominal_override828)
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
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value830, _) => {
                                            source_ml_source2_data_property_value830.clone()
                                        }
                                    }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                                        {
                                            let left_value = {
                                                let source_ml_source2_nullish_field_receiver831 = (left.clone(),);
                                                if match (source_ml_source2_nullish_field_receiver831.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present832 = ((source_ml_source2_nullish_field_receiver831.clone()).0).clone().expect("a proven present value");
                                                        ((((source_ml_source2_nullish_field_present832).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41])).0, message_present: true }).raise()
                                                }
                                            };
                                            {
                                                let right_value = {
                                                    let source_ml_source2_nullish_field_receiver833 = (right.clone(),);
                                                    if match (source_ml_source2_nullish_field_receiver833.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source2_nullish_field_present834 = ((source_ml_source2_nullish_field_receiver833.clone()).0).clone().expect("a proven present value");
                                                            ((((source_ml_source2_nullish_field_present834).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
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
                                                                    let source_ml_source2_nominal_spread836 = next.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field837 = match source_ml_source2_nominal_spread836.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value838, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value838 = *source_ml_source2_data_property_value838;
                                                                                source_ml_source2_data_property_value838.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field839 = match source_ml_source2_nominal_spread836.clone() {
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value840, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value840 = *source_ml_source2_data_property_value840;
                                                                                    source_ml_source2_data_property_value840.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field841 = match source_ml_source2_nominal_spread836.clone() {
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value842, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value842 = *source_ml_source2_data_property_value842;
                                                                                        source_ml_source2_data_property_value842.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field843 = match source_ml_source2_nominal_spread836.clone() {
                                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value844, _, _, _) => {
                                                                                            source_ml_source2_data_property_value844
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field845 = match source_ml_source2_nominal_spread836.clone() {
                                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value846, _, _) => {
                                                                                                source_ml_source2_data_property_value846
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field847 = match source_ml_source2_nominal_spread836.clone() {
                                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value848, _) => {
                                                                                                    source_ml_source2_data_property_value848.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field849 = match source_ml_source2_nominal_spread836.clone() {
                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value850) => {
                                                                                                        source_ml_source2_data_property_value850
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override851 = match state.clone() {
                                                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value835, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value835
                                                                                                        }
                                                                                                    } + 1f64;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override852 = 0f64;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override853 = self::ml_string::Text::from_units(vec![115, 101, 97, 114, 99, 104]);
                                                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field837), Box::new(source_ml_source2_nominal_field839), Box::new(source_ml_source2_nominal_field841), source_ml_source2_nominal_override851, source_ml_source2_nominal_override852, source_ml_source2_nominal_override853, source_ml_source2_nominal_field849)
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
                                            let source_ml_source2_nullish_field_receiver854 = (left.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver854.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present855 = ((source_ml_source2_nullish_field_receiver854.clone()).0).clone().expect("a proven present value");
                                                    (source_ml_source2_nullish_field_present855).0
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }, {
                                            let source_ml_source2_nullish_field_receiver856 = (right.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver856.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present857 = ((source_ml_source2_nullish_field_receiver856.clone()).0).clone().expect("a proven present value");
                                                    (source_ml_source2_nullish_field_present857).0
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                            }
                                        })) {if (self::read_rust_standard_collection_kind(&(self::resolve_rust_standard_collection_owner(&(match match match state.clone() {
                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value878, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value878 = *source_ml_source2_data_property_value878;
                                                    source_ml_source2_data_property_value878.clone()
                                                }
                                            } {
                                                self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value879, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value879 = *source_ml_source2_data_property_value879;
                                                    source_ml_source2_data_property_value879.clone()
                                                }
                                            } {
                                                self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value880, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value880.clone()
                                                }
                                            }), true, false)))) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                self::RustStandardCollectionEqualityTransition::RustCollectionEqualityContinue(Box::new({
                                                    let source_ml_source2_nominal_spread881 = next.clone();
                                                    {
                                                        let source_ml_source2_nominal_field882 = match source_ml_source2_nominal_spread881.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value883, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value883 = *source_ml_source2_data_property_value883;
                                                                source_ml_source2_data_property_value883.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field884 = match source_ml_source2_nominal_spread881.clone() {
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value885, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value885 = *source_ml_source2_data_property_value885;
                                                                    source_ml_source2_data_property_value885.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field886 = match source_ml_source2_nominal_spread881.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value887, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value887 = *source_ml_source2_data_property_value887;
                                                                        source_ml_source2_data_property_value887.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field888 = match source_ml_source2_nominal_spread881.clone() {
                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value889, _, _, _) => {
                                                                            source_ml_source2_data_property_value889
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field890 = match source_ml_source2_nominal_spread881.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value891, _, _) => {
                                                                                source_ml_source2_data_property_value891
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field892 = match source_ml_source2_nominal_spread881.clone() {
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value893, _) => {
                                                                                    source_ml_source2_data_property_value893.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field894 = match source_ml_source2_nominal_spread881.clone() {
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value895) => {
                                                                                        source_ml_source2_data_property_value895
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override896 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field882), Box::new(source_ml_source2_nominal_field884), Box::new(source_ml_source2_nominal_field886), source_ml_source2_nominal_field888, source_ml_source2_nominal_field890, source_ml_source2_nominal_override896, source_ml_source2_nominal_field894)
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
                                                    let source_ml_source2_nominal_spread898 = next.clone();
                                                    {
                                                        let source_ml_source2_nominal_field899 = match source_ml_source2_nominal_spread898.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value900, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value900 = *source_ml_source2_data_property_value900;
                                                                source_ml_source2_data_property_value900.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field901 = match source_ml_source2_nominal_spread898.clone() {
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value902, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value902 = *source_ml_source2_data_property_value902;
                                                                    source_ml_source2_data_property_value902.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field903 = match source_ml_source2_nominal_spread898.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value904, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value904 = *source_ml_source2_data_property_value904;
                                                                        source_ml_source2_data_property_value904.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field905 = match source_ml_source2_nominal_spread898.clone() {
                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value906, _, _, _) => {
                                                                            source_ml_source2_data_property_value906
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field907 = match source_ml_source2_nominal_spread898.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value908, _, _) => {
                                                                                source_ml_source2_data_property_value908
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field909 = match source_ml_source2_nominal_spread898.clone() {
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value910, _) => {
                                                                                    source_ml_source2_data_property_value910.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field911 = match source_ml_source2_nominal_spread898.clone() {
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value912) => {
                                                                                        source_ml_source2_data_property_value912
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override913 = match state.clone() {
                                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value897, _, _, _) => {
                                                                                            source_ml_source2_data_property_value897
                                                                                        }
                                                                                    } + 1f64;
                                                                                    {
                                                                                        let source_ml_source2_nominal_override914 = 0f64;
                                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field899), Box::new(source_ml_source2_nominal_field901), Box::new(source_ml_source2_nominal_field903), source_ml_source2_nominal_override913, source_ml_source2_nominal_override914, source_ml_source2_nominal_field909, source_ml_source2_nominal_field911)
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
                                                let source_ml_source2_nominal_spread859 = next.clone();
                                                {
                                                    let source_ml_source2_nominal_field860 = match source_ml_source2_nominal_spread859.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value861, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value861 = *source_ml_source2_data_property_value861;
                                                            source_ml_source2_data_property_value861.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field862 = match source_ml_source2_nominal_spread859.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value863, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value863 = *source_ml_source2_data_property_value863;
                                                                source_ml_source2_data_property_value863.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field864 = match source_ml_source2_nominal_spread859.clone() {
                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, source_ml_source2_data_property_value865, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value865 = *source_ml_source2_data_property_value865;
                                                                    source_ml_source2_data_property_value865.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field866 = match source_ml_source2_nominal_spread859.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value867, _, _, _) => {
                                                                        source_ml_source2_data_property_value867
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field868 = match source_ml_source2_nominal_spread859.clone() {
                                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value869, _, _) => {
                                                                            source_ml_source2_data_property_value869
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field870 = match source_ml_source2_nominal_spread859.clone() {
                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value871, _) => {
                                                                                source_ml_source2_data_property_value871.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field872 = match source_ml_source2_nominal_spread859.clone() {
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value873) => {
                                                                                    source_ml_source2_data_property_value873
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_override874 = match state.clone() {
                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value858, _, _) => {
                                                                                        source_ml_source2_data_property_value858
                                                                                    }
                                                                                } + 1f64;
                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(Box::new(source_ml_source2_nominal_field860), Box::new(source_ml_source2_nominal_field862), Box::new(source_ml_source2_nominal_field864), source_ml_source2_nominal_field866, source_ml_source2_nominal_override874, source_ml_source2_nominal_field870, source_ml_source2_nominal_field872)
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
            let source_ml_source2_same_value_operand915 = budget;
            {
                let source_ml_source2_same_value_operand916 = -0f64;
                ((((source_ml_source2_same_value_operand915)
                    .partial_cmp(&(source_ml_source2_same_value_operand916))
                    == Some(std::cmp::Ordering::Equal))
                    && ((source_ml_source2_same_value_operand915 != 0f64)
                        || ((1f64 / source_ml_source2_same_value_operand915)
                            == (1f64 / source_ml_source2_same_value_operand916))))
                    || ((source_ml_source2_same_value_operand915
                        != source_ml_source2_same_value_operand915)
                        && (source_ml_source2_same_value_operand916
                            != source_ml_source2_same_value_operand916)))
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
                source_ml_source2_f917,
            ) => beginning.clone(),
            self::RustStandardCollectionEqualityTransition::RustCollectionEqualityInvalid(
                source_ml_source2_f918,
            ) => beginning.clone(),
            self::RustStandardCollectionEqualityTransition::RustCollectionEqualityResourceLimit(
                source_ml_source2_f919,
                source_ml_source2_f920,
            ) => {
                let source_ml_source2_f919 = *source_ml_source2_f919;
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
        self::RustStandardCollectionTransition::RustCollectionCreated(source_ml_source2_f921) => {
            let source_ml_source2_f921 = *source_ml_source2_f921;
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
                    let source_ml_source2_constructor_field922 = reason_2.clone();
                    self::RustStandardCollectionPacketTransition::RustCollectionResourceLimit(
                        None::<(Option<self::RustStandardCollectionState>,)>,
                        source_ml_source2_constructor_field922,
                    )
                }
            } else {
                {
                    let source_ml_source2_constructor_field923 =
                        Some(state_2.clone().expect("a proven present value"));
                    {
                        let source_ml_source2_constructor_field924 = reason_2.clone();
                        self::RustStandardCollectionPacketTransition::RustCollectionResourceLimit(
                            Some(source_ml_source2_constructor_field923),
                            source_ml_source2_constructor_field924,
                        )
                    }
                }
            }
        }
    }
}
