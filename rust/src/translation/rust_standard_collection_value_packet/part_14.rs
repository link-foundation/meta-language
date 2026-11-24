// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Result;
use super::RustStandardCollectionEqualityState;
use super::RustStandardCollectionOrderingState;
use super::RustStandardCollectionOrderingTransition;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::RustStandardCollectionTransition;
use super::advance_rust_standard_collection_ordering;
use super::compare_rust_standard_collection_scalars;
use super::copy_rust_standard_collection_snapshot_scalar;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::ordering_work;
use super::read_rust_standard_collection_ordering_work;
use super::read_rust_standard_collection_scalar_equality_work;
use super::read_rust_standard_collection_snapshot_scalar;
use super::test_rust_standard_collection_scalar_equality;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_rust_standard_collection_ordered_snapshot_loop34(
    method: &self::ml_string::Text,
    mut values: Vec<self::RustStandardCollectionScalar>,
    mut source_ml_source1_temporary_values33: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut source_ml_source1_temporary_index33: self::ml::Big,
) -> self::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Result {
    loop {
        return if source_ml_source1_temporary_index33
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values33.len() as u128))
        {
            {
                let entry = self::ml_array::at(
                    &source_ml_source1_temporary_values33,
                    source_ml_source1_temporary_index33.to_index(),
                );
                {
                    let value = self::read_rust_standard_collection_snapshot_scalar(entry, method);
                    if match (value.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Result::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Return(Box::new(self::RustStandardCollectionTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 101, 100, 32, 109, 97, 112, 32, 118, 97, 108, 117, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 115, 99, 97, 108, 97, 114]))))
                    } else {
                        {
                            let values_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![self::copy_rust_standard_collection_snapshot_scalar(
                                        value.clone().expect("a proven present value"),
                                    )],
                                    values,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let source_ml_source1_temporary_index33_2 =
                                    source_ml_source1_temporary_index33
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (values, source_ml_source1_temporary_index33) =
                                        (values_2, source_ml_source1_temporary_index33_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Result::MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Done(values)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderingWorkJoin35 {
    MlSource1TemporaryOrderingWorkJoin35Next,
    MlSource1TemporaryOrderingWorkJoin35Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39 {
    MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39Next(f64),
    MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorJoin39Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result {
    MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Done(f64),
    MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_collection_ordering_state_error_loop38(
    mut state: self::RustStandardCollectionOrderingState,
    mut map: bool,
    mut work: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result {
    loop {
        return if index
            < (match state.clone() {
                self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1021,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1021,
            }) {
            {
                let first = self::ml_array::optional_at(
                    &match match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(
                            source_ml_source2_data_property_value1022,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value1022 =
                                *source_ml_source2_data_property_value1022;
                            source_ml_source2_data_property_value1022.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value1023,
                        ) => source_ml_source2_data_property_value1023,
                    },
                    self::ml_array::number_index(index),
                );
                {
                    let second = self::ml_array::optional_at(&match match state.clone() {
                        self::RustStandardCollectionOrderingState::RustCollectionOrderingState(_, source_ml_source2_data_property_value1024, _, _, _, _) => {
                            let source_ml_source2_data_property_value1024 = *source_ml_source2_data_property_value1024;
                            source_ml_source2_data_property_value1024.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value1025) => {
                            source_ml_source2_data_property_value1025
                        }
                    }, self::ml_array::number_index(index));
                    if (self::compare_rust_standard_collection_scalars(
                        {
                            let source_ml_source2_nullish_field_receiver1030 = (first.clone(),);
                            if match (source_ml_source2_nullish_field_receiver1030.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present1031 =
                                        ((source_ml_source2_nullish_field_receiver1030.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    (source_ml_source2_nullish_field_present1031).0
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121,
                                        39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        },
                        {
                            let source_ml_source2_nullish_field_receiver1032 = (second.clone(),);
                            if match (source_ml_source2_nullish_field_receiver1032.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present1033 =
                                        ((source_ml_source2_nullish_field_receiver1032.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    (source_ml_source2_nullish_field_present1033).0
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121,
                                        39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        },
                    )) != 0f64
                    {
                        self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Return(self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 105, 110, 103, 32, 99, 117, 114, 115, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 115, 107, 105, 112, 32, 116, 104, 101, 32, 102, 105, 114, 115, 116, 32, 117, 110, 101, 113, 117, 97, 108, 32, 107, 101, 121]))
                    } else {
                        {
                            let work_2 = work
                                + self::ordering_work(
                                    {
                                        let source_ml_source2_nullish_field_receiver1034 =
                                            (first.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver1034
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            {
                                                let source_ml_source2_nullish_field_present1035 = ((source_ml_source2_nullish_field_receiver1034.clone()).0).clone().expect("a proven present value");
                                                (source_ml_source2_nullish_field_present1035).0
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        67, 97, 110, 110, 111, 116, 32, 114, 101,
                                                        97, 100, 32, 112, 114, 111, 112, 101, 114,
                                                        116, 105, 101, 115, 32, 111, 102, 32, 117,
                                                        110, 100, 101, 102, 105, 110, 101, 100, 32,
                                                        40, 114, 101, 97, 100, 105, 110, 103, 32,
                                                        39, 107, 101, 121, 39, 41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        }
                                    },
                                    {
                                        let source_ml_source2_nullish_field_receiver1036 =
                                            (second.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver1036
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            {
                                                let source_ml_source2_nullish_field_present1037 = ((source_ml_source2_nullish_field_receiver1036.clone()).0).clone().expect("a proven present value");
                                                (source_ml_source2_nullish_field_present1037).0
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        67, 97, 110, 110, 111, 116, 32, 114, 101,
                                                        97, 100, 32, 112, 114, 111, 112, 101, 114,
                                                        116, 105, 101, 115, 32, 111, 102, 32, 117,
                                                        110, 100, 101, 102, 105, 110, 101, 100, 32,
                                                        40, 114, 101, 97, 100, 105, 110, 103, 32,
                                                        39, 107, 101, 121, 39, 41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        }
                                    },
                                );
                            if map {
                                {
                                    let first_value = {
                                        let source_ml_source2_nullish_field_receiver1038 =
                                            (first.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver1038
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            {
                                                let source_ml_source2_nullish_field_present1039 = ((source_ml_source2_nullish_field_receiver1038.clone()).0).clone().expect("a proven present value");
                                                ((((source_ml_source2_nullish_field_present1039).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        67, 97, 110, 110, 111, 116, 32, 114, 101,
                                                        97, 100, 32, 112, 114, 111, 112, 101, 114,
                                                        116, 105, 101, 115, 32, 111, 102, 32, 117,
                                                        110, 100, 101, 102, 105, 110, 101, 100, 32,
                                                        40, 114, 101, 97, 100, 105, 110, 103, 32,
                                                        39, 118, 97, 108, 117, 101, 39, 41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        }
                                    };
                                    {
                                        let second_value = {
                                            let source_ml_source2_nullish_field_receiver1040 =
                                                (second.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver1040
                                                .clone())
                                            .0
                                            {
                                                Some(ml_present) => {
                                                    let _ = ml_present.clone();
                                                    true
                                                }
                                                None => false,
                                            } {
                                                {
                                                    let source_ml_source2_nullish_field_present1041 = ((source_ml_source2_nullish_field_receiver1040.clone()).0).clone().expect("a proven present value");
                                                    ((((source_ml_source2_nullish_field_present1041).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException {
                                                    class_name: String::from("TypeError"),
                                                    message_units:
                                                        (self::ml_string::Text::from_units(vec![
                                                            67, 97, 110, 110, 111, 116, 32, 114,
                                                            101, 97, 100, 32, 112, 114, 111, 112,
                                                            101, 114, 116, 105, 101, 115, 32, 111,
                                                            102, 32, 117, 110, 100, 101, 102, 105,
                                                            110, 101, 100, 32, 40, 114, 101, 97,
                                                            100, 105, 110, 103, 32, 39, 118, 97,
                                                            108, 117, 101, 39, 41,
                                                        ]))
                                                        .0,
                                                    message_present: true,
                                                })
                                                .raise()
                                            }
                                        };
                                        {
                                            let invalid = self::ml_string::Text::from_units(vec![
                                                111, 114, 100, 101, 114, 105, 110, 103, 32, 99,
                                                117, 114, 115, 111, 114, 32, 99, 97, 110, 110, 111,
                                                116, 32, 115, 107, 105, 112, 32, 116, 104, 101, 32,
                                                102, 105, 114, 115, 116, 32, 117, 110, 101, 113,
                                                117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 109,
                                                97, 112, 32, 118, 97, 108, 117, 101,
                                            ]);
                                            if match (first_value.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Return(invalid.clone())
                                            } else if match (second_value.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Return(invalid.clone())
                                            } else {
                                                let ml_library_inequality_1 =
                                                    (self::compare_rust_standard_collection_scalars(
                                                        first_value
                                                            .clone()
                                                            .expect("a proven present value"),
                                                        second_value
                                                            .clone()
                                                            .expect("a proven present value"),
                                                    ))
                                                        != 0f64;
                                                if ml_library_inequality_1 {
                                                    self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Return(invalid.clone())
                                                } else {
                                                    {
                                                        let work_3 = work_2
                                                            + self::ordering_work(
                                                                first_value.clone().expect(
                                                                    "a proven present value",
                                                                ),
                                                                second_value.clone().expect(
                                                                    "a proven present value",
                                                                ),
                                                            );
                                                        {
                                                            let index_2 = index + 1f64;
                                                            {
                                                                (work, index) = (work_3, index_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let index_3 = index + 1f64;
                                    {
                                        (work, index) = (work_2, index_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Result::MlSource1TemporaryReadRustStandardCollectionOrderingStateErrorLoop38Done(work)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40 {
    MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40Next,
    MlSource1TemporaryReadRustStandardCollectionOrderingWorkJoin40Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41 {
    MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41Next(f64),
    MlSource1TemporaryAdvanceRustStandardCollectionOrderingJoin41Return(
        Box<self::RustStandardCollectionOrderingTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_run_rust_standard_collection_ordering_loop42(
    mut current: self::RustStandardCollectionOrderingState,
    mut remaining: f64,
) -> self::RustStandardCollectionOrderingTransition {
    loop {
        return {
            let cost = self::read_rust_standard_collection_ordering_work(current.clone());
            if cost > remaining {
                self::RustStandardCollectionOrderingTransition::RustCollectionOrderingResourceLimit(
                    Box::new(current.clone()),
                    self::ml_string::Text::from_units(vec![
                        66, 84, 114, 101, 101, 32, 111, 114, 100, 101, 114, 105, 110, 103, 32, 101,
                        120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99,
                        116, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 97,
                        110, 100, 32, 83, 116, 114, 105, 110, 103, 32, 119, 111, 114, 107, 32, 98,
                        117, 100, 103, 101, 116,
                    ]),
                )
            } else {
                {
                    let result = self::advance_rust_standard_collection_ordering(current.clone());
                    match result.clone() {
                        self::RustStandardCollectionOrderingTransition::RustCollectionOrderingContinue(state) => {
                            let state = *state;
                            {
                                let remaining_2 = remaining - cost;
                                {
                                    let current_2 = state.clone();
                                    {
                                        (current, remaining) = (current_2, remaining_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        self::RustStandardCollectionOrderingTransition::RustCollectionOrderingCompleted(source_ml_source2_f1042) => {
                            result.clone()
                        }
                        self::RustStandardCollectionOrderingTransition::RustCollectionOrderingInvalid(source_ml_source2_f1043) => {
                            result.clone()
                        }
                        self::RustStandardCollectionOrderingTransition::RustCollectionOrderingResourceLimit(source_ml_source2_f1044, source_ml_source2_f1045) => {
                            let source_ml_source2_f1044 = *source_ml_source2_f1044;
                            result.clone()
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52 {
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52Next(f64),
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin52Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Result {
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Done(f64),
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop53(
    mut state: self::RustStandardCollectionEqualityState,
    mut work: f64,
    mut left: Option<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut index: f64,
) -> self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Result {
    loop {
        return if index
            < (match state.clone() {
                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1047,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1047,
            }) {
            {
                let right = self::ml_array::optional_at(
                    &match match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            _,
                            source_ml_source2_data_property_value1048,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value1048 =
                                *source_ml_source2_data_property_value1048;
                            source_ml_source2_data_property_value1048.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value1049,
                        ) => source_ml_source2_data_property_value1049,
                    },
                    self::ml_array::number_index(index),
                );
                if self::test_rust_standard_collection_scalar_equality(
                    {
                        let source_ml_source2_nullish_field_receiver1050 = (left.clone(),);
                        if match (source_ml_source2_nullish_field_receiver1050.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present1051 =
                                    ((source_ml_source2_nullish_field_receiver1050.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                (source_ml_source2_nullish_field_present1051).0
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    },
                    {
                        let source_ml_source2_nullish_field_receiver1052 = (right.clone(),);
                        if match (source_ml_source2_nullish_field_receiver1052.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present1053 =
                                    ((source_ml_source2_nullish_field_receiver1052.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                (source_ml_source2_nullish_field_present1053).0
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    },
                ) {
                    self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Return(self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108, 105, 116, 121, 32, 115, 101, 97, 114, 99, 104, 32, 99, 117, 114, 115, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 115, 107, 105, 112, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 107, 101, 121]))
                } else {
                    {
                        let work_2 = work
                            + self::read_rust_standard_collection_scalar_equality_work(
                                {
                                    let source_ml_source2_nullish_field_receiver1054 =
                                        (left.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver1054.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present1055 =
                                                ((source_ml_source2_nullish_field_receiver1054
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            (source_ml_source2_nullish_field_present1055).0
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
                                                    97, 100, 105, 110, 103, 32, 39, 107, 101, 121,
                                                    39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                },
                                {
                                    let source_ml_source2_nullish_field_receiver1056 =
                                        (right.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver1056.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present1057 =
                                                ((source_ml_source2_nullish_field_receiver1056
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            (source_ml_source2_nullish_field_present1057).0
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
                                                    97, 100, 105, 110, 103, 32, 39, 107, 101, 121,
                                                    39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                },
                            );
                        {
                            let index_2 = index + 1f64;
                            {
                                (work, index) = (work_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop53Done(work)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result {
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Done(f64),
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Result {
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Done(f64, bool),
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55 {
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55Next(f64),
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55Return(
        self::ml_string::Text,
    ),
}
