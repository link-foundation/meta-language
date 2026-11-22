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
            let source_ml_source2_optional_chain_receiver592 = (state.clone(),);
            {
                let source_ml_source1_temporary_optional_chain44 =
                    (source_ml_source2_optional_chain_receiver592).0;
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
                            source_ml_source2_data_property_value593,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value593 =
                                *source_ml_source2_data_property_value593;
                            source_ml_source2_data_property_value593.clone()
                        }
                    },
                    match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            source_ml_source2_data_property_value594,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value594 =
                                *source_ml_source2_data_property_value594;
                            source_ml_source2_data_property_value594.clone()
                        }
                    },
                    match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            _,
                            source_ml_source2_data_property_value595,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value595 =
                                *source_ml_source2_data_property_value595;
                            source_ml_source2_data_property_value595.clone()
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
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value596, _, _, _) => {
                            source_ml_source2_data_property_value596
                        }
                    }, match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value597, _, _) => {
                            source_ml_source2_data_property_value597
                        }
                    }, match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value598) => {
                            source_ml_source2_data_property_value598
                        }
                    }];
                            {
                                let source_ml_source1_temporary_index45 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s599 = self::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop49(&(source_ml_source1_temporary_values45), source_ml_source1_temporary_index45);
                                    match source_ml_source2_s599 {
                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Done => {
                                    if (((match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value603, _, _, _) => {
                                            source_ml_source2_data_property_value603
                                        }
                                    }) > ((match match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value604, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value604 = *source_ml_source2_data_property_value604;
                                            source_ml_source2_data_property_value604.clone()
                                        }
                                    } {
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value605) => {
                                            source_ml_source2_data_property_value605
                                        }
                                    }).len() as f64)) || ((match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value609, _, _) => {
                                            source_ml_source2_data_property_value609
                                        }
                                    }) > ((match match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value610, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value610 = *source_ml_source2_data_property_value610;
                                            source_ml_source2_data_property_value610.clone()
                                        }
                                    } {
                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value611) => {
                                            source_ml_source2_data_property_value611
                                        }
                                    }).len() as f64))) || !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![115, 101, 97, 114, 99, 104]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])], &match state.clone() {
                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value612, _) => {
                                            source_ml_source2_data_property_value612.clone()
                                        }
                                    }, 0f64, false, |a, b| a == b)) {
                                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 99, 117, 114, 115, 111, 114, 115, 32, 114, 101, 109, 97, 105, 110, 32, 119, 105, 116, 104, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 112, 101, 114, 97, 110, 100, 115])
                                    } else {
                                        {
                                            let source_ml_source2_s633 = if (match state.clone() {
                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value635, _) => {
                                                    source_ml_source2_data_property_value635.clone()
                                                }
                                            }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                                                {
                                                    let left = self::ml_array::optional_at(&match match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value636, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value636 = *source_ml_source2_data_property_value636;
                                                            source_ml_source2_data_property_value636.clone()
                                                        }
                                                    } {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value637) => {
                                                            source_ml_source2_data_property_value637
                                                        }
                                                    }, self::ml_array::number_index(match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value638, _, _, _) => {
                                                            source_ml_source2_data_property_value638
                                                        }
                                                    }));
                                                    {
                                                        let right = self::ml_array::optional_at(&match match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value639, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value639 = *source_ml_source2_data_property_value639;
                                                                source_ml_source2_data_property_value639.clone()
                                                            }
                                                        } {
                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value640) => {
                                                                source_ml_source2_data_property_value640
                                                            }
                                                        }, self::ml_array::number_index(match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value641, _, _) => {
                                                                source_ml_source2_data_property_value641
                                                            }
                                                        }));
                                                        if ((((self::read_rust_standard_collection_kind(&(self::resolve_rust_standard_collection_owner(&(match match match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value645, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value645 = *source_ml_source2_data_property_value645;
                                                                source_ml_source2_data_property_value645.clone()
                                                            }
                                                        } {
                                                            self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value646, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value646 = *source_ml_source2_data_property_value646;
                                                                source_ml_source2_data_property_value646.clone()
                                                            }
                                                        } {
                                                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value647, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value647.clone()
                                                            }
                                                        }), true, false)))) != (self::ml_string::Text::from_units(vec![109, 97, 112]))) || match ({
                                                            let source_ml_source2_optional_chain_receiver648 = (left.clone(),);
                                                            match if match (source_ml_source2_optional_chain_receiver648.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                Some({
                                                                    let source_ml_source1_temporary_optional_chain46 = ((source_ml_source2_optional_chain_receiver648.clone()).0).clone().expect("a proven present value");
                                                                    ((((source_ml_source1_temporary_optional_chain46).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                                                })
                                                            } else {
                                                                None::<self::RustStandardCollectionScalar>
                                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match ({
                                                            let source_ml_source2_optional_chain_receiver649 = (right.clone(),);
                                                            match if match (source_ml_source2_optional_chain_receiver649.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                Some({
                                                                    let source_ml_source1_temporary_optional_chain47 = ((source_ml_source2_optional_chain_receiver649.clone()).0).clone().expect("a proven present value");
                                                                    ((((source_ml_source1_temporary_optional_chain47).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                                                })
                                                            } else {
                                                                None::<self::RustStandardCollectionScalar>
                                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_rust_standard_collection_scalar_equality({
                                                            let source_ml_source2_nullish_field_receiver650 = (left.clone(),);
                                                            if match (source_ml_source2_nullish_field_receiver650.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source2_nullish_field_present651 = ((source_ml_source2_nullish_field_receiver650.clone()).0).clone().expect("a proven present value");
                                                                    (source_ml_source2_nullish_field_present651).0
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                            }
                                                        }, {
                                                            let source_ml_source2_nullish_field_receiver652 = (right.clone(),);
                                                            if match (source_ml_source2_nullish_field_receiver652.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source2_nullish_field_present653 = ((source_ml_source2_nullish_field_receiver652.clone()).0).clone().expect("a proven present value");
                                                                    (source_ml_source2_nullish_field_present653).0
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
                                            match source_ml_source2_s633 {
                                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50Next => {
                                                    if (((match match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value658, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value658 = *source_ml_source2_data_property_value658;
                                                            source_ml_source2_data_property_value658.clone()
                                                        }
                                                    } {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value659) => {
                                                            source_ml_source2_data_property_value659
                                                        }
                                                    }).len() as f64) != ((match match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value660, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value660 = *source_ml_source2_data_property_value660;
                                                            source_ml_source2_data_property_value660.clone()
                                                        }
                                                    } {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value661) => {
                                                            source_ml_source2_data_property_value661
                                                        }
                                                    }).len() as f64)) && (((((match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value663, _, _, _) => {
                                                            source_ml_source2_data_property_value663
                                                        }
                                                    }) != 0f64) || ((match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value665, _, _) => {
                                                            source_ml_source2_data_property_value665
                                                        }
                                                    }) != 0f64)) || ((match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value667, _) => {
                                                            source_ml_source2_data_property_value667.clone()
                                                        }
                                                    }) != (self::ml_string::Text::from_units(vec![115, 101, 97, 114, 99, 104])))) || ((match state.clone() {
                                                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value669) => {
                                                            source_ml_source2_data_property_value669
                                                        }
                                                    }) != 0f64)) {
                                                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 115, 105, 122, 101, 32, 112, 114, 101, 102, 108, 105, 103, 104, 116, 32, 112, 114, 101, 99, 101, 100, 101, 115, 32, 101, 118, 101, 114, 121, 32, 107, 101, 121, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110])
                                                    } else if ((match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value673, _, _, _) => {
                                                                source_ml_source2_data_property_value673
                                                            }
                                                        }) == ((match match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value674, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value674 = *source_ml_source2_data_property_value674;
                                                                source_ml_source2_data_property_value674.clone()
                                                            }
                                                        } {
                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value675) => {
                                                                source_ml_source2_data_property_value675
                                                            }
                                                        }).len() as f64)) && (((match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value677, _, _) => {
                                                                source_ml_source2_data_property_value677
                                                            }
                                                        }) != 0f64) || ((match state.clone() {
                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value679, _) => {
                                                                source_ml_source2_data_property_value679.clone()
                                                            }
                                                        }) != (self::ml_string::Text::from_units(vec![115, 101, 97, 114, 99, 104])))) {
                                                            self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 112, 114, 101, 102, 105, 120, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 97, 110, 111, 110, 105, 99, 97, 108, 32, 110, 101, 120, 116, 32, 99, 117, 114, 115, 111, 114])
                                                        } else {
                                                            {
                                                                let map = (self::read_rust_standard_collection_kind(&(self::resolve_rust_standard_collection_owner(&(match match match state.clone() {
                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value683, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value683 = *source_ml_source2_data_property_value683;
                                                                        source_ml_source2_data_property_value683.clone()
                                                                    }
                                                                } {
                                                                    self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value684, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value684 = *source_ml_source2_data_property_value684;
                                                                        source_ml_source2_data_property_value684.clone()
                                                                    }
                                                                } {
                                                                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value685, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value685.clone()
                                                                    }
                                                                }), true, false)))) == (self::ml_string::Text::from_units(vec![109, 97, 112]));
                                                                {
                                                                    let work = 0f64;
                                                                    {
                                                                        let index = 0f64;
                                                                        {
                                                                            let source_ml_source2_s686 = self::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop51(state.clone(), map, work, index);
                                                                            match source_ml_source2_s686 {
                                                                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Done(work_2) => {
                                                                                    {
                                                                                        let source_ml_source2_s706 = if (match state.clone() {
                                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value710, _, _, _) => {
                                                                                                source_ml_source2_data_property_value710
                                                                                            }
                                                                                        }) < ((match match state.clone() {
                                                                                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value711, _, _, _, _, _, _) => {
                                                                                                let source_ml_source2_data_property_value711 = *source_ml_source2_data_property_value711;
                                                                                                source_ml_source2_data_property_value711.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value712) => {
                                                                                                source_ml_source2_data_property_value712
                                                                                            }
                                                                                        }).len() as f64) {
                                                                                            {
                                                                                                let left_2 = self::ml_array::optional_at(&match match state.clone() {
                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(source_ml_source2_data_property_value713, _, _, _, _, _, _) => {
                                                                                                        let source_ml_source2_data_property_value713 = *source_ml_source2_data_property_value713;
                                                                                                        source_ml_source2_data_property_value713.clone()
                                                                                                    }
                                                                                                } {
                                                                                                    self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value714) => {
                                                                                                        source_ml_source2_data_property_value714
                                                                                                    }
                                                                                                }, self::ml_array::number_index(match state.clone() {
                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, source_ml_source2_data_property_value715, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value715
                                                                                                    }
                                                                                                }));
                                                                                                {
                                                                                                    let index_2 = 0f64;
                                                                                                    {
                                                                                                        let source_ml_source2_s716 = self::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop53(state.clone(), work_2, left_2.clone(), index_2);
                                                                                                        match source_ml_source2_s716 {
                                                                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Done(work_3) => {
                                                                                                                if (match state.clone() {
                                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, source_ml_source2_data_property_value718, _) => {
                                                                                                                        source_ml_source2_data_property_value718.clone()
                                                                                                                    }
                                                                                                                }) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])) {
                                                                                                                    {
                                                                                                                        let work_4 = work_3 + self::read_rust_standard_collection_scalar_equality_work({
                                                                                                                            let source_ml_source2_nullish_field_receiver719 = (left_2.clone(),);
                                                                                                                            if match (source_ml_source2_nullish_field_receiver719.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nullish_field_present720 = ((source_ml_source2_nullish_field_receiver719.clone()).0).clone().expect("a proven present value");
                                                                                                                                    (source_ml_source2_nullish_field_present720).0
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                                                                                            }
                                                                                                                        }, {
                                                                                                                            let source_ml_source2_nullish_field_receiver724 = (self::ml_array::optional_at(&match match state.clone() {
                                                                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value721, _, _, _, _, _) => {
                                                                                                                                    let source_ml_source2_data_property_value721 = *source_ml_source2_data_property_value721;
                                                                                                                                    source_ml_source2_data_property_value721.clone()
                                                                                                                                }
                                                                                                                            } {
                                                                                                                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value722) => {
                                                                                                                                    source_ml_source2_data_property_value722
                                                                                                                                }
                                                                                                                            }, self::ml_array::number_index(match state.clone() {
                                                                                                                                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, source_ml_source2_data_property_value723, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value723
                                                                                                                                }
                                                                                                                            })),);
                                                                                                                            if match (source_ml_source2_nullish_field_receiver724.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nullish_field_present725 = ((source_ml_source2_nullish_field_receiver724.clone()).0).clone().expect("a proven present value");
                                                                                                                                    (source_ml_source2_nullish_field_present725).0
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
                                                                                        match source_ml_source2_s706 {
                                                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52Next(work_5) => {
                                                                                                if work_5 != (match state.clone() {
                                                                                                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, _, _, _, _, _, source_ml_source2_data_property_value727) => {
                                                                                                        source_ml_source2_data_property_value727
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
        let source_ml_source2_s732 = match left.clone() {
            self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f733) => {
                self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next
            }
            self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f734) => {
                self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next
            }
            self::RustStandardCollectionScalar::RustCollectionString(left_units) => {
                match right.clone() {
                    self::RustStandardCollectionScalar::RustCollectionBoolean(source_ml_source2_f735) => {
                        self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next
                    }
                    self::RustStandardCollectionScalar::RustCollectionInteger(source_ml_source2_f736) => {
                        self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next
                    }
                    self::RustStandardCollectionScalar::RustCollectionString(units) => {
                        self::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56::MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Return(self::ml_math::max(1f64, ((left_units.len() as f64) + (units.len() as f64))))
                    }
                }
            }
        };
        match source_ml_source2_s732 {
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
            source_ml_source2_data_property_value741,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let source_ml_source2_data_property_value741 =
                *source_ml_source2_data_property_value741;
            source_ml_source2_data_property_value741.clone()
        }
    } {
        self::RustStandardCollectionState::RustStandardCollectionState(
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value742,
        ) => source_ml_source2_data_property_value742,
    })
    .len() as f64)
        != ((match match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                source_ml_source2_data_property_value743,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value743 =
                    *source_ml_source2_data_property_value743;
                source_ml_source2_data_property_value743.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value744,
            ) => source_ml_source2_data_property_value744,
        })
        .len() as f64))
        || ((match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                _,
                _,
                source_ml_source2_data_property_value748,
                _,
                _,
                _,
            ) => source_ml_source2_data_property_value748,
        }) == ((match match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                source_ml_source2_data_property_value749,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value749 =
                    *source_ml_source2_data_property_value749;
                source_ml_source2_data_property_value749.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value750,
            ) => source_ml_source2_data_property_value750,
        })
        .len() as f64)))
        || ((match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value754,
                _,
                _,
            ) => source_ml_source2_data_property_value754,
        }) == ((match match state.clone() {
            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                _,
                source_ml_source2_data_property_value755,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value755 =
                    *source_ml_source2_data_property_value755;
                source_ml_source2_data_property_value755.clone()
            }
        } {
            self::RustStandardCollectionState::RustStandardCollectionState(
                _,
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value756,
            ) => source_ml_source2_data_property_value756,
        })
        .len() as f64))
    {
        0f64
    } else {
        {
            let left = self::ml_array::optional_at(
                &match match state.clone() {
                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                        source_ml_source2_data_property_value757,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value757 =
                            *source_ml_source2_data_property_value757;
                        source_ml_source2_data_property_value757.clone()
                    }
                } {
                    self::RustStandardCollectionState::RustStandardCollectionState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value758,
                    ) => source_ml_source2_data_property_value758,
                },
                self::ml_array::number_index(match state.clone() {
                    self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value759,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value759,
                }),
            );
            {
                let right = self::ml_array::optional_at(
                    &match match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            source_ml_source2_data_property_value760,
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
                    },
                    self::ml_array::number_index(match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value762,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value762,
                    }),
                );
                {
                    let source_ml_source2_s769 = if (match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value771,
                            _,
                        ) => source_ml_source2_data_property_value771.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        118, 97, 108, 117, 101,
                    ])) {
                        {
                            let left_value = {
                                let source_ml_source2_nullish_field_receiver772 = (left.clone(),);
                                if match (source_ml_source2_nullish_field_receiver772.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source2_nullish_field_present773 =
                                            ((source_ml_source2_nullish_field_receiver772.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        ((((source_ml_source2_nullish_field_present773).1)
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
                                    let source_ml_source2_nullish_field_receiver774 =
                                        (right.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver774.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present775 =
                                                ((source_ml_source2_nullish_field_receiver774
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            ((((source_ml_source2_nullish_field_present775).1)
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
                    match source_ml_source2_s769 {
                        self::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57::MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57Next => {
                            self::read_rust_standard_collection_scalar_equality_work({
                                let source_ml_source2_nullish_field_receiver776 = (left.clone(),);
                                if match (source_ml_source2_nullish_field_receiver776.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present777 = ((source_ml_source2_nullish_field_receiver776.clone()).0).clone().expect("a proven present value");
                                        (source_ml_source2_nullish_field_present777).0
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                }
                            }, {
                                let source_ml_source2_nullish_field_receiver778 = (right.clone(),);
                                if match (source_ml_source2_nullish_field_receiver778.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present779 = ((source_ml_source2_nullish_field_receiver778.clone()).0).clone().expect("a proven present value");
                                        (source_ml_source2_nullish_field_present779).0
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
