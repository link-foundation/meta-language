// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50;
use super::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52;
use super::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Result;
use super::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result;
use super::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Result;
use super::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57;
use super::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56;
use super::RustStandardCollectionEqualityState;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_rust_standard_collection_equality_origin_error;
use super::read_rust_standard_collection_kind;
use super::resolve_rust_standard_collection_owner;
use super::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop49;
use super::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop51;
use super::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop53;
use super::test_rust_standard_collection_scalar_equality;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_equality_state_error(
    state: self::RustStandardCollectionEqualityState,
) -> self::ml_string::Text {
    {
        let ml_product_condition_1 = {
            let source_ml_source2_optional_chain_receiver603 = (state.clone(),);
            {
                let source_ml_source1_temporary_optional_chain44 =
                    (source_ml_source2_optional_chain_receiver603).0;
                match source_ml_source1_temporary_optional_chain44 {
                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => self::ml_string::Text::from_units(vec![
                        114, 117, 115, 116, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 69,
                        113, 117, 97, 108, 105, 116, 121, 83, 116, 97, 116, 101,
                    ]),
                }
            }
        };
        if ml_product_condition_1 {
            self::ml_string::Text::from_units(vec![
                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108, 105,
                116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 116, 97, 116, 101,
            ])
        } else {
            {
                let source = self::read_rust_standard_collection_equality_origin_error(
                    match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            source_ml_source2_data_property_value604,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value604 =
                                *source_ml_source2_data_property_value604;
                            source_ml_source2_data_property_value604.clone()
                        }
                    },
                    match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            source_ml_source2_data_property_value605,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value605 =
                                *source_ml_source2_data_property_value605;
                            source_ml_source2_data_property_value605.clone()
                        }
                    },
                    match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            _,
                            source_ml_source2_data_property_value606,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value606 =
                                *source_ml_source2_data_property_value606;
                            source_ml_source2_data_property_value606.clone()
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
                            let source_ml_source1_temporary_values45 = vec![match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value607, _, _, _) => {
                            source_ml_source2_data_property_value607
                        }
                    }, match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value608, _, _) => {
                            source_ml_source2_data_property_value608
                        }
                    }, match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value609) => {
                            source_ml_source2_data_property_value609
                        }
                    }];
                            {
                                let source_ml_source1_temporary_index45 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s610 = self::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop49(&(source_ml_source1_temporary_values45), source_ml_source1_temporary_index45);
                                    match source_ml_source2_s610 {
                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Done => {
                                    if (((match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value614, _, _, _) => {
                                            source_ml_source2_data_property_value614
                                        }
                                    }) > ((match match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value615, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value615 = *source_ml_source2_data_property_value615;
                                            source_ml_source2_data_property_value615.clone()
                                        }
                                    } {
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value616) => {
                                            source_ml_source2_data_property_value616
                                        }
                                    }).len() as f64)) || ((match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value620, _, _) => {
                                            source_ml_source2_data_property_value620
                                        }
                                    }) > ((match match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value621, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value621 = *source_ml_source2_data_property_value621;
                                            source_ml_source2_data_property_value621.clone()
                                        }
                                    } {
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value622) => {
                                            source_ml_source2_data_property_value622
                                        }
                                    }).len() as f64))) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![115, 101, 97, 114, 99, 104]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])], &match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value623, _) => {
                                            source_ml_source2_data_property_value623.clone()
                                        }
                                    }, 0f64, false, |a, b| a == b)) {
                                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 99, 117, 114, 115, 111, 114, 115, 32, 114, 101, 109, 97, 105, 110, 32, 119, 105, 116, 104, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 112, 101, 114, 97, 110, 100, 115])
                                    } else {
                                        {
                                            let source_ml_source2_s644 = if (match state.clone() {
                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value646, _) => {
                                                    source_ml_source2_data_property_value646.clone()
                                                }
                                            }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                                                {
                                                    let left = self::ml_array::optional_at(&match match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value647, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value647 = *source_ml_source2_data_property_value647;
                                                            source_ml_source2_data_property_value647.clone()
                                                        }
                                                    } {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value648) => {
                                                            source_ml_source2_data_property_value648
                                                        }
                                                    }, self::ml_array::number_index(match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value649, _, _, _) => {
                                                            source_ml_source2_data_property_value649
                                                        }
                                                    }));
                                                    {
                                                        let right = self::ml_array::optional_at(&match match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value650, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value650 = *source_ml_source2_data_property_value650;
                                                                source_ml_source2_data_property_value650.clone()
                                                            }
                                                        } {
                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value651) => {
                                                                source_ml_source2_data_property_value651
                                                            }
                                                        }, self::ml_array::number_index(match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value652, _, _) => {
                                                                source_ml_source2_data_property_value652
                                                            }
                                                        }));
                                                        if ((((self::read_rust_standard_collection_kind(&(self::resolve_rust_standard_collection_owner(&(match match match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value656, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value656 = *source_ml_source2_data_property_value656;
                                                                source_ml_source2_data_property_value656.clone()
                                                            }
                                                        } {
                                                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value657, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value657 = *source_ml_source2_data_property_value657;
                                                                source_ml_source2_data_property_value657.clone()
                                                            }
                                                        } {
                                                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value658, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value658.clone()
                                                            }
                                                        }), true, false)))) != (self::ml_string::Text::from_units(vec![109, 97, 112]))) || match ({
                                                            let source_ml_source2_optional_chain_receiver659 = (left.clone(),);
                                                            match if match (source_ml_source2_optional_chain_receiver659.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                Some({
                                                                    let source_ml_source1_temporary_optional_chain46 = ((source_ml_source2_optional_chain_receiver659.clone()).0).clone().expect("a proven present value");
                                                                    ((((source_ml_source1_temporary_optional_chain46).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                                                })
                                                            } else {
                                                                None::<self::RustStandardCollectionScalar>
                                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match ({
                                                            let source_ml_source2_optional_chain_receiver660 = (right.clone(),);
                                                            match if match (source_ml_source2_optional_chain_receiver660.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                Some({
                                                                    let source_ml_source1_temporary_optional_chain47 = ((source_ml_source2_optional_chain_receiver660.clone()).0).clone().expect("a proven present value");
                                                                    ((((source_ml_source1_temporary_optional_chain47).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                                                })
                                                            } else {
                                                                None::<self::RustStandardCollectionScalar>
                                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_rust_standard_collection_scalar_equality({
                                                            let source_ml_source2_nullish_field_receiver661 = (left.clone(),);
                                                            if match (source_ml_source2_nullish_field_receiver661.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source2_nullish_field_present662 = ((source_ml_source2_nullish_field_receiver661.clone()).0).clone().expect("a proven present value");
                                                                    (source_ml_source2_nullish_field_present662).0
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                            }
                                                        }, {
                                                            let source_ml_source2_nullish_field_receiver663 = (right.clone(),);
                                                            if match (source_ml_source2_nullish_field_receiver663.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source2_nullish_field_present664 = ((source_ml_source2_nullish_field_receiver663.clone()).0).clone().expect("a proven present value");
                                                                    (source_ml_source2_nullish_field_present664).0
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                            }
                                                        })) {
                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50Return(self::ml_string::Text::from_units(vec![109, 97, 112, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 118, 97, 108, 117, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 113, 117, 97, 108, 45, 107, 101, 121, 32, 112, 97, 105, 114]))
                                                        } else {
                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50Next
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50Next
                                            };
                                            match source_ml_source2_s644 {
                                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50Next => {
                                                    if (((match match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value669, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value669 = *source_ml_source2_data_property_value669;
                                                            source_ml_source2_data_property_value669.clone()
                                                        }
                                                    } {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value670) => {
                                                            source_ml_source2_data_property_value670
                                                        }
                                                    }).len() as f64) != ((match match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value671, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value671 = *source_ml_source2_data_property_value671;
                                                            source_ml_source2_data_property_value671.clone()
                                                        }
                                                    } {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value672) => {
                                                            source_ml_source2_data_property_value672
                                                        }
                                                    }).len() as f64)) && (((((match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value674, _, _, _) => {
                                                            source_ml_source2_data_property_value674
                                                        }
                                                    }) != 0f64) || ((match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value676, _, _) => {
                                                            source_ml_source2_data_property_value676
                                                        }
                                                    }) != 0f64)) || ((match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value678, _) => {
                                                            source_ml_source2_data_property_value678.clone()
                                                        }
                                                    }) != (self::ml_string::Text::from_units(vec![115, 101, 97, 114, 99, 104])))) || ((match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value680) => {
                                                            source_ml_source2_data_property_value680
                                                        }
                                                    }) != 0f64)) {
                                                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 115, 105, 122, 101, 32, 112, 114, 101, 102, 108, 105, 103, 104, 116, 32, 112, 114, 101, 99, 101, 100, 101, 115, 32, 101, 118, 101, 114, 121, 32, 107, 101, 121, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110])
                                                    } else if ((match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value684, _, _, _) => {
                                                                source_ml_source2_data_property_value684
                                                            }
                                                        }) == ((match match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value685, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value685 = *source_ml_source2_data_property_value685;
                                                                source_ml_source2_data_property_value685.clone()
                                                            }
                                                        } {
                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value686) => {
                                                                source_ml_source2_data_property_value686
                                                            }
                                                        }).len() as f64)) && (((match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value688, _, _) => {
                                                                source_ml_source2_data_property_value688
                                                            }
                                                        }) != 0f64) || ((match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value690, _) => {
                                                                source_ml_source2_data_property_value690.clone()
                                                            }
                                                        }) != (self::ml_string::Text::from_units(vec![115, 101, 97, 114, 99, 104])))) {
                                                            self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 112, 114, 101, 102, 105, 120, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 97, 110, 111, 110, 105, 99, 97, 108, 32, 110, 101, 120, 116, 32, 99, 117, 114, 115, 111, 114])
                                                        } else {
                                                            {
                                                                let map = (self::read_rust_standard_collection_kind(&(self::resolve_rust_standard_collection_owner(&(match match match state.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value694, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value694 = *source_ml_source2_data_property_value694;
                                                                        source_ml_source2_data_property_value694.clone()
                                                                    }
                                                                } {
                                                                    self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value695, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value695 = *source_ml_source2_data_property_value695;
                                                                        source_ml_source2_data_property_value695.clone()
                                                                    }
                                                                } {
                                                                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value696, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value696.clone()
                                                                    }
                                                                }), true, false)))) == (self::ml_string::Text::from_units(vec![109, 97, 112]));
                                                                {
                                                                    let work = 0f64;
                                                                    {
                                                                        let index = 0f64;
                                                                        {
                                                                            let source_ml_source2_s697 = self::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop51(state.clone(), map, work, index);
                                                                            match source_ml_source2_s697 {
                                                                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Done(work_2) => {
                                                                                    {
                                                                                        let source_ml_source2_s717 = if (match state.clone() {
                                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value721, _, _, _) => {
                                                                                                source_ml_source2_data_property_value721
                                                                                            }
                                                                                        }) < ((match match state.clone() {
                                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value722, _, _, _, _, _, _) => {
                                                                                                let source_ml_source2_data_property_value722 = *source_ml_source2_data_property_value722;
                                                                                                source_ml_source2_data_property_value722.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value723) => {
                                                                                                source_ml_source2_data_property_value723
                                                                                            }
                                                                                        }).len() as f64) {
                                                                                            {
                                                                                                let left_2 = self::ml_array::optional_at(&match match state.clone() {
                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value724, _, _, _, _, _, _) => {
                                                                                                        let source_ml_source2_data_property_value724 = *source_ml_source2_data_property_value724;
                                                                                                        source_ml_source2_data_property_value724.clone()
                                                                                                    }
                                                                                                } {
                                                                                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value725) => {
                                                                                                        source_ml_source2_data_property_value725
                                                                                                    }
                                                                                                }, self::ml_array::number_index(match state.clone() {
                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value726, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value726
                                                                                                    }
                                                                                                }));
                                                                                                {
                                                                                                    let index_2 = 0f64;
                                                                                                    {
                                                                                                        let source_ml_source2_s727 = self::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop53(state.clone(), work_2, left_2.clone(), index_2);
                                                                                                        match source_ml_source2_s727 {
                                                                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Done(work_3) => {
                                                                                                                if (match state.clone() {
                                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value729, _) => {
                                                                                                                        source_ml_source2_data_property_value729.clone()
                                                                                                                    }
                                                                                                                }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                                                                                                                    {
                                                                                                                        let work_4 = work_3 + self::read_rust_standard_collection_scalar_equality_work({
                                                                                                                            let source_ml_source2_nullish_field_receiver730 = (left_2.clone(),);
                                                                                                                            if match (source_ml_source2_nullish_field_receiver730.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nullish_field_present731 = ((source_ml_source2_nullish_field_receiver730.clone()).0).clone().expect("a proven present value");
                                                                                                                                    (source_ml_source2_nullish_field_present731).0
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                                                                            }
                                                                                                                        }, {
                                                                                                                            let source_ml_source2_nullish_field_receiver735 = (self::ml_array::optional_at(&match match state.clone() {
                                                                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value732, _, _, _, _, _) => {
                                                                                                                                    let source_ml_source2_data_property_value732 = *source_ml_source2_data_property_value732;
                                                                                                                                    source_ml_source2_data_property_value732.clone()
                                                                                                                                }
                                                                                                                            } {
                                                                                                                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value733) => {
                                                                                                                                    source_ml_source2_data_property_value733
                                                                                                                                }
                                                                                                                            }, self::ml_array::number_index(match state.clone() {
                                                                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value734, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value734
                                                                                                                                }
                                                                                                                            })),);
                                                                                                                            if match (source_ml_source2_nullish_field_receiver735.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nullish_field_present736 = ((source_ml_source2_nullish_field_receiver735.clone()).0).clone().expect("a proven present value");
                                                                                                                                    (source_ml_source2_nullish_field_present736).0
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                                                                            }
                                                                                                                        });
                                                                                                                        self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52Next(work_4)
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52Next(work_3)
                                                                                                                }
                                                                                                            }
                                                                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Return(source_ml_source1_temporary_result53) => {
                                                                                                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52Return(source_ml_source1_temporary_result53.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52Next(work_2)
                                                                                        };
                                                                                        match source_ml_source2_s717 {
                                                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52Next(work_5) => {
                                                                                                if work_5 != (match state.clone() {
                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value738) => {
                                                                                                        source_ml_source2_data_property_value738
                                                                                                    }
                                                                                                }) {
                                                                                                    self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108, 105, 116, 121, 32, 99, 104, 97, 114, 103, 101, 100, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 111, 115, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 112, 114, 101, 102, 105, 120])
                                                                                                } else {
                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52Return(source_ml_source1_temporary_result52) => {
                                                                                                source_ml_source1_temporary_result52.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Return(source_ml_source1_temporary_result53_2) => {
                                                                                    source_ml_source1_temporary_result53_2.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                }
                                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50Return(source_ml_source1_temporary_result55) => {
                                                    source_ml_source1_temporary_result55.clone()
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Return(source_ml_source1_temporary_result55_2) => {
                                    source_ml_source1_temporary_result55_2.clone()
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
pub fn read_rust_standard_collection_scalar_equality_work(
    left: self::RustStandardCollectionScalar,
    right: self::RustStandardCollectionScalar,
) -> f64 {
    {
        let source_ml_source2_s743 = match left.clone() {
            self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f744) => {
                self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next
            }
            self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f745) => {
                self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next
            }
            self::RustStandardCollectionScalar::RustCollectionString(left_units) => {
                match right.clone() {
                    self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f746) => {
                        self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next
                    }
                    self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f747) => {
                        self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next
                    }
                    self::RustStandardCollectionScalar::RustCollectionString(units) => {
                        self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Return(self::ml_math::max(1f64, ((left_units.len() as f64) + (units.len() as f64))))
                    }
                }
            }
        };
        match source_ml_source2_s743 {
            self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next => {
                1f64
            }
            self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Return(source_ml_source1_temporary_result56) => {
                source_ml_source1_temporary_result56
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_equality_work(
    state: self::RustStandardCollectionEqualityState,
) -> f64 {
    if ((((match match state.clone() {
        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
            source_ml_source2_data_property_value752,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let source_ml_source2_data_property_value752 =
                *source_ml_source2_data_property_value752;
            source_ml_source2_data_property_value752.clone()
        }
    } {
        self::RustStandardCollectionState::RustStandardCollectionState(
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value753,
        ) => source_ml_source2_data_property_value753,
    })
    .len() as f64)
        != ((match match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                source_ml_source2_data_property_value754,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value754 =
                    *source_ml_source2_data_property_value754;
                source_ml_source2_data_property_value754.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value755,
            ) => source_ml_source2_data_property_value755,
        })
        .len() as f64))
        || ((match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                _,
                _,
                source_ml_source2_data_property_value759,
                _,
                _,
                _,
            ) => source_ml_source2_data_property_value759,
        }) == ((match match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                source_ml_source2_data_property_value760,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value760 =
                    *source_ml_source2_data_property_value760;
                source_ml_source2_data_property_value760.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value761,
            ) => source_ml_source2_data_property_value761,
        })
        .len() as f64)))
        || ((match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value765,
                _,
                _,
            ) => source_ml_source2_data_property_value765,
        }) == ((match match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                source_ml_source2_data_property_value766,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value766 =
                    *source_ml_source2_data_property_value766;
                source_ml_source2_data_property_value766.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value767,
            ) => source_ml_source2_data_property_value767,
        })
        .len() as f64))
    {
        0f64
    } else {
        {
            let left = self::ml_array::optional_at(
                &match match state.clone() {
                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                        source_ml_source2_data_property_value768,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value768 =
                            *source_ml_source2_data_property_value768;
                        source_ml_source2_data_property_value768.clone()
                    }
                } {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value769,
                    ) => source_ml_source2_data_property_value769,
                },
                self::ml_array::number_index(match state.clone() {
                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value770,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value770,
                }),
            );
            {
                let right = self::ml_array::optional_at(
                    &match match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            source_ml_source2_data_property_value771,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value771 =
                                *source_ml_source2_data_property_value771;
                            source_ml_source2_data_property_value771.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value772,
                        ) => source_ml_source2_data_property_value772,
                    },
                    self::ml_array::number_index(match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value773,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value773,
                    }),
                );
                {
                    let source_ml_source2_s780 = if (match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value782,
                            _,
                        ) => source_ml_source2_data_property_value782.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        118, 97, 108, 117, 101,
                    ])) {
                        {
                            let left_value = {
                                let source_ml_source2_nullish_field_receiver783 = (left.clone(),);
                                if match (source_ml_source2_nullish_field_receiver783.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source2_nullish_field_present784 =
                                            ((source_ml_source2_nullish_field_receiver783.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        ((((source_ml_source2_nullish_field_present784).1)
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
                                let right_value = {
                                    let source_ml_source2_nullish_field_receiver785 =
                                        (right.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver785.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present786 =
                                                ((source_ml_source2_nullish_field_receiver785
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            ((((source_ml_source2_nullish_field_present786).1)
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
                                if (match (left_value.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    self::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57Next
                                } else if (match (right_value.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    self::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57Next
                                } else {
                                    self::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57Return(self::read_rust_standard_collection_scalar_equality_work(left_value.clone().expect("a proven present value"), right_value.clone().expect("a proven present value")))
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57Next
                    };
                    match source_ml_source2_s780 {
                        self::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57Next => {
                            self::read_rust_standard_collection_scalar_equality_work({
                                let source_ml_source2_nullish_field_receiver787 = (left.clone(),);
                                if match (source_ml_source2_nullish_field_receiver787.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present788 = ((source_ml_source2_nullish_field_receiver787.clone()).0).clone().expect("a proven present value");
                                        (source_ml_source2_nullish_field_present788).0
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                }
                            }, {
                                let source_ml_source2_nullish_field_receiver789 = (right.clone(),);
                                if match (source_ml_source2_nullish_field_receiver789.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present790 = ((source_ml_source2_nullish_field_receiver789.clone()).0).clone().expect("a proven present value");
                                        (source_ml_source2_nullish_field_present790).0
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                }
                            })
                        }
                        self::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57Return(source_ml_source1_temporary_result57) => {
                            source_ml_source1_temporary_result57
                        }
                    }
                }
            }
        }
    }
}
