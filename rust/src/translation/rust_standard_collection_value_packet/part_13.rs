// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Result;
use super::RustStandardCollectionCallOrigin;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::RustStandardCollectionTransition;
use super::compare_rust_standard_collection_scalars;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_rust_standard_collection_argument_origin_error;
use super::read_rust_standard_collection_scalar_error;
use super::read_rust_standard_collection_scalar_text_length;
use super::retain_rust_standard_collection_entry;
use super::test_rust_standard_collection_ordered;
use super::test_rust_standard_collection_scalar_equality;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_rust_standard_collection_scalar_equality_loop18(
    mut left: self::RustStandardCollectionScalar,
    mut right: self::RustStandardCollectionScalar,
    mut index: f64,
) -> self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Result {
    loop {
        return if index
            < ({
                let source_ml_source2_nullish_field_receiver978 = (match left.clone() {
                    self::RustStandardCollectionScalar::RustCollectionBoolean(_) => {
                        None::<Vec<f64>>
                    }
                    self::RustStandardCollectionScalar::RustCollectionInteger(_) => {
                        None::<Vec<f64>>
                    }
                    self::RustStandardCollectionScalar::RustCollectionString(
                        source_ml_source2_data_property_value977,
                    ) => Some(source_ml_source2_data_property_value977),
                },);
                if match (source_ml_source2_nullish_field_receiver978.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source2_nullish_field_present979 =
                            ((source_ml_source2_nullish_field_receiver978.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        (source_ml_source2_nullish_field_present979.len() as f64)
                    }
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111,
                            112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                            101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103,
                            32, 39, 108, 101, 110, 103, 116, 104, 39, 41,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                }
            }) {
            if {
                let source_ml_source2_comparison_operand988 = {
                    let source_ml_source2_nullish_field_receiver981 = (
                        match left.clone() {
                            self::RustStandardCollectionScalar::RustCollectionBoolean(_) => {
                                None::<Vec<f64>>
                            }
                            self::RustStandardCollectionScalar::RustCollectionInteger(_) => {
                                None::<Vec<f64>>
                            }
                            self::RustStandardCollectionScalar::RustCollectionString(
                                source_ml_source2_data_property_value980,
                            ) => Some(source_ml_source2_data_property_value980),
                        },
                        index,
                    );
                    {
                        let source_ml_source2_computed_property_key982 =
                            (source_ml_source2_nullish_field_receiver981.clone()).1;
                        if match (source_ml_source2_nullish_field_receiver981.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source2_nullish_field_present983 =
                                    ((source_ml_source2_nullish_field_receiver981.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                self::ml_array::optional_at(
                                    &source_ml_source2_nullish_field_present983,
                                    self::ml_array::number_index(
                                        source_ml_source2_computed_property_key982,
                                    ),
                                )
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (((self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39,
                                ]))
                                .concat(
                                    &self::ml_string::Text::from_text(&self::ml_number::js_number(
                                        (source_ml_source2_nullish_field_receiver981.clone()).1,
                                    )),
                                ))
                                .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                };
                {
                    let source_ml_source2_comparison_operand989 = {
                        let source_ml_source2_nullish_field_receiver985 = (
                            match right.clone() {
                                self::RustStandardCollectionScalar::RustCollectionBoolean(_) => {
                                    None::<Vec<f64>>
                                }
                                self::RustStandardCollectionScalar::RustCollectionInteger(_) => {
                                    None::<Vec<f64>>
                                }
                                self::RustStandardCollectionScalar::RustCollectionString(
                                    source_ml_source2_data_property_value984,
                                ) => Some(source_ml_source2_data_property_value984),
                            },
                            index,
                        );
                        {
                            let source_ml_source2_computed_property_key986 =
                                (source_ml_source2_nullish_field_receiver985.clone()).1;
                            if match (source_ml_source2_nullish_field_receiver985.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present987 =
                                        ((source_ml_source2_nullish_field_receiver985.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    self::ml_array::optional_at(
                                        &source_ml_source2_nullish_field_present987,
                                        self::ml_array::number_index(
                                            source_ml_source2_computed_property_key986,
                                        ),
                                    )
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (((self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39,
                                    ]))
                                    .concat(&self::ml_string::Text::from_text(
                                        &self::ml_number::js_number(
                                            (source_ml_source2_nullish_field_receiver985.clone()).1,
                                        ),
                                    )))
                                    .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        }
                    };
                    !(match (
                        source_ml_source2_comparison_operand988,
                        source_ml_source2_comparison_operand989,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Result::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Return(false)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Result::MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19 {
    MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19Next,
    MlSource1TemporaryTestRustStandardCollectionScalarEqualityJoin19Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindRustStandardCollectionEntryLoop20Result {
    MlSource1TemporaryFindRustStandardCollectionEntryLoop20Done,
    MlSource1TemporaryFindRustStandardCollectionEntryLoop20Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_rust_standard_collection_entry_loop20(
    mut entries: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut key: self::RustStandardCollectionScalar,
    mut index: f64,
) -> self::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Result {
    loop {
        return if index < (entries.len() as f64) {
            if self::test_rust_standard_collection_scalar_equality(
                {
                    let source_ml_source2_nullish_field_receiver990 = (
                        self::ml_array::optional_at(&entries, self::ml_array::number_index(index)),
                    );
                    if match (source_ml_source2_nullish_field_receiver990.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        {
                            let source_ml_source2_nullish_field_present991 =
                                ((source_ml_source2_nullish_field_receiver990.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            (source_ml_source2_nullish_field_present991).0
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114,
                                111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110,
                                100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                110, 103, 32, 39, 107, 101, 121, 39, 41,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    }
                },
                key.clone(),
            ) {
                self::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Result::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Return(index)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Result::MlSource1TemporaryFindRustStandardCollectionEntryLoop20Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result {
    MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Done(f64),
    MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Result {
    MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Done,
    MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_collection_state_error_loop24(
    mut state: self::RustStandardCollectionState,
    mut index: f64,
    mut entry: Option<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut previous: f64,
) -> self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Result {
    loop {
        return if previous < index {
            if self::test_rust_standard_collection_scalar_equality(
                {
                    let source_ml_source2_nullish_field_receiver993 =
                        (self::ml_array::optional_at(
                            (match &state {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value992,
                                ) => source_ml_source2_data_property_value992.as_slice(),
                            }),
                            self::ml_array::number_index(previous),
                        ),);
                    if match (source_ml_source2_nullish_field_receiver993.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        {
                            let source_ml_source2_nullish_field_present994 =
                                ((source_ml_source2_nullish_field_receiver993.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            (source_ml_source2_nullish_field_present994).0
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114,
                                111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110,
                                100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                110, 103, 32, 39, 107, 101, 121, 39, 41,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    }
                },
                {
                    let source_ml_source2_nullish_field_receiver995 = (entry.clone(),);
                    if match (source_ml_source2_nullish_field_receiver995.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        {
                            let source_ml_source2_nullish_field_present996 =
                                ((source_ml_source2_nullish_field_receiver995.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            (source_ml_source2_nullish_field_present996).0
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114,
                                111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110,
                                100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                110, 103, 32, 39, 107, 101, 121, 39, 41,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    }
                },
            ) {
                self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 97, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110, 101, 32, 101, 110, 116, 114, 121, 32, 102, 111, 114, 32, 101, 97, 99, 104, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 69, 113, 32, 107, 101, 121]))
            } else {
                {
                    let previous_2 = previous + 1f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23 {
    MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23Next(f64),
    MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_collection_state_error_loop22(
    mut state: self::RustStandardCollectionState,
    kind: &self::ml_string::Text,
    mut units: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result {
    loop {
        return if index
            < ((match &state {
                self::RustStandardCollectionState::RustStandardCollectionState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value998,
                ) => source_ml_source2_data_property_value998.as_slice(),
            })
            .len() as f64)
        {
            {
                let entry = self::ml_array::optional_at(
                    (match &state {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value999,
                        ) => source_ml_source2_data_property_value999.as_slice(),
                    }),
                    self::ml_array::number_index(index),
                );
                if match (entry.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 97, 116, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 101, 110, 116, 114, 121, 32, 97, 116, 32, 101, 97, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                } else {
                    {
                        let key_error = self::read_rust_standard_collection_scalar_error(
                            Some((entry.clone().expect("a proven present value")).0),
                            &(match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value1000,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value1000.clone(),
                            }),
                            match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value1001,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value1001,
                            },
                        );
                        {
                            let ml_library_inequality_2 =
                                key_error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_2 {
                                self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Return(key_error.clone())
                            } else {
                                {
                                    let units_2 = units
                                        + self::read_rust_standard_collection_scalar_text_length(
                                            (entry.clone().expect("a proven present value")).0,
                                        );
                                    {
                                        let value =
                                            ((((entry.clone().expect("a proven present value"))
                                                .1)
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    (None::<self::RustStandardCollectionScalar>,)
                                                }))
                                            .0)
                                                .clone();
                                        {
                                            let source_ml_source2_s1004 = if (*kind)
                                                == (self::ml_string::Text::from_units(vec![
                                                    109, 97, 112,
                                                ])) {
                                                if match (value.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } {
                                                    self::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23Return(self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 77, 97, 112, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 118, 97, 108, 117, 101, 115]))
                                                } else {
                                                    {
                                                        let value_error = self::read_rust_standard_collection_scalar_error(Some(value.clone().expect("a proven present value")), &(match state.clone() {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, source_ml_source2_data_property_value1005, _, _, _) => {
                                                            source_ml_source2_data_property_value1005.clone()
                                                        }
                                                    }), match state.clone() {
                                                        self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value1006, _, _) => {
                                                            source_ml_source2_data_property_value1006
                                                        }
                                                    });
                                                        {
                                                            let ml_library_inequality_1 = value_error != (self::ml_string::Text::from_units(vec![])) ;
                                                            if ml_library_inequality_1 {
                                                                self::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23Return(value_error.clone())
                                                            } else {
                                                                {
                                                                    let units_3 = units_2 + self::read_rust_standard_collection_scalar_text_length(value.clone().expect("a proven present value"));
                                                                    self::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23Next(units_3)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else if (match (value.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            }) {
                                                self::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23Next(units_2)
                                            } else {
                                                self::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23Return(self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 83, 101, 116, 32, 101, 110, 116, 114, 105, 101, 115, 32, 100, 111, 32, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 115, 116, 111, 114, 101, 100, 32, 109, 97, 112, 32, 118, 97, 108, 117, 101, 115]))
                                            };
                                            match source_ml_source2_s1004 {
                                            self::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23Next(units_4) => {
                                                if (self::test_rust_standard_collection_ordered(&(match match state.clone() {
                                                    self::RustStandardCollectionState::RustStandardCollectionState(source_ml_source2_data_property_value1007, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value1007 = *source_ml_source2_data_property_value1007;
                                                        source_ml_source2_data_property_value1007.clone()
                                                    }
                                                } {
                                                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, source_ml_source2_data_property_value1008, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value1008.clone()
                                                    }
                                                })) && (index > 0f64)) && ((self::compare_rust_standard_collection_scalars({
                                                    let source_ml_source2_nullish_field_receiver1013 = (self::ml_array::optional_at((match &state { self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value1012) => source_ml_source2_data_property_value1012.as_slice() }), self::ml_array::number_index(index - 1f64)),);
                                                    if match (source_ml_source2_nullish_field_receiver1013.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source2_nullish_field_present1014 = ((source_ml_source2_nullish_field_receiver1013.clone()).0).clone().expect("a proven present value");
                                                            (source_ml_source2_nullish_field_present1014).0
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                                                    }
                                                }, (entry.clone().expect("a proven present value")).0)) >= 0f64) {
                                                    self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Return(self::ml_string::Text::from_units(vec![66, 84, 114, 101, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 115, 116, 114, 105, 99, 116, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 79, 114, 100, 32, 105, 110, 99, 108, 117, 100, 105, 110, 103, 32, 85, 84, 70, 56, 32, 83, 116, 114, 105, 110, 103, 32, 111, 114, 100, 101, 114, 105, 110, 103]))
                                                } else {
                                                    {
                                                        let previous = 0f64;
                                                        {
                                                            let source_ml_source2_s1015 = self::source_ml_source1_temporary_read_rust_standard_collection_state_error_loop24(state.clone(), index, Some(entry.clone().expect("a proven present value")), previous);
                                                            match source_ml_source2_s1015 {
                                                                self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Done => {
                                                                    {
                                                                        let index_2 = index + 1f64;
                                                                        {
                                                                            (units, index) = (units_4, index_2);
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop24Return(source_ml_source1_temporary_result24) => {
                                                                    self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Return(source_ml_source1_temporary_result24.clone())
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23::MlSource1TemporaryReadRustStandardCollectionStateErrorJoin23Return(source_ml_source1_temporary_result24_2) => {
                                                self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Return(source_ml_source1_temporary_result24_2.clone())
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
        } else {
            self::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Result::MlSource1TemporaryReadRustStandardCollectionStateErrorLoop22Done(units)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_retain_rust_standard_collection_insertion_loop28(
    mut key: self::RustStandardCollectionScalar,
    mut insertion: f64,
    mut source_ml_source1_temporary_values26: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut source_ml_source1_temporary_index26: self::ml::Big,
) -> f64 {
    loop {
        return if source_ml_source1_temporary_index26
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values26.len() as u128))
        {
            {
                let entry = self::ml_array::at(
                    &source_ml_source1_temporary_values26,
                    source_ml_source1_temporary_index26.to_index(),
                );
                if (self::compare_rust_standard_collection_scalars((entry).0, key.clone())) >= 0f64
                {
                    insertion
                } else {
                    {
                        let insertion_2 = insertion + 1f64;
                        {
                            let source_ml_source1_temporary_index26_2 =
                                source_ml_source1_temporary_index26
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (insertion, source_ml_source1_temporary_index26) =
                                    (insertion_2, source_ml_source1_temporary_index26_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            insertion
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_retain_rust_standard_collection_insertion_loop29(
    mut replacement: (
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    ),
    mut present: bool,
    mut insertion: f64,
    mut entries: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut index: f64,
    mut source_ml_source1_temporary_values27: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut source_ml_source1_temporary_index27: self::ml::Big,
) -> Vec<(
    self::RustStandardCollectionScalar,
    Option<(Option<self::RustStandardCollectionScalar>,)>,
)> {
    loop {
        if source_ml_source1_temporary_index27
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values27.len() as u128))
        {
            {
                let entry = self::ml_array::at(
                    &source_ml_source1_temporary_values27,
                    source_ml_source1_temporary_index27.to_index(),
                );
                {
                    let entries_3 =
                        if (index).partial_cmp(&(insertion)) == Some(std::cmp::Ordering::Equal) {
                            {
                                {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![replacement.clone()], entries);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                }
                            }
                        } else {
                            entries
                        };
                    {
                        let entries_5 = if !present || (index != insertion) {
                            {
                                {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![self::retain_rust_standard_collection_entry(entry)],
                                        entries_3,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                }
                            }
                        } else {
                            entries_3
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                let source_ml_source1_temporary_index27_2 =
                                    source_ml_source1_temporary_index27
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (entries, index, source_ml_source1_temporary_index27) =
                                        (entries_5, index_2, source_ml_source1_temporary_index27_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { entries };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_rust_standard_collection_operation_loop32(
    mut position: f64,
    mut entries: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut index: f64,
    mut source_ml_source1_temporary_values30: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut source_ml_source1_temporary_index30: self::ml::Big,
) -> Vec<(
    self::RustStandardCollectionScalar,
    Option<(Option<self::RustStandardCollectionScalar>,)>,
)> {
    loop {
        if source_ml_source1_temporary_index30
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values30.len() as u128))
        {
            {
                let entry = self::ml_array::at(
                    &source_ml_source1_temporary_values30,
                    source_ml_source1_temporary_index30.to_index(),
                );
                {
                    let entries_3 = if index == position {
                        entries
                    } else {
                        {
                            {
                                let (ml_append_right, ml_append_left) = (
                                    vec![self::retain_rust_standard_collection_entry(entry)],
                                    entries,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            let source_ml_source1_temporary_index30_2 =
                                source_ml_source1_temporary_index30
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (entries, index, source_ml_source1_temporary_index30) =
                                    (entries_3, index_2, source_ml_source1_temporary_index30_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { entries };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Result {
    MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Done,
    MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Return(
        Box<self::RustStandardCollectionTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_rust_standard_collection_operation_loop31(
    mut state: self::RustStandardCollectionState,
    mut arguments_: Vec<self::RustStandardCollectionScalar>,
    mut call: self::RustStandardCollectionCallOrigin,
    mut borrowed: bool,
    mut index: f64,
) -> self::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Result {
    loop {
        return if index < (arguments_.len() as f64) {
            {
                let domain = if index == 0f64 {
                    match state.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            source_ml_source2_data_property_value1016,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value1016.clone(),
                    }
                } else {
                    match state.clone() {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value1017,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value1017.clone(),
                    }
                };
                {
                    let argument = self::read_rust_standard_collection_argument_origin_error(self::ml_array::optional_at((match &call { self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value1018, _, _) => source_ml_source2_data_property_value1018.as_slice() }), self::ml_array::number_index(index)), &(domain), borrowed);
                    {
                        let ml_library_inequality_2 =
                            argument != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                            self::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Result::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Return(Box::new(self::RustStandardCollectionTransition::RustCollectionInvalid(argument.clone())))
                        } else {
                            {
                                let scalar = self::read_rust_standard_collection_scalar_error(self::ml_array::optional_at(&arguments_, self::ml_array::number_index(index)), &(domain), match state.clone() {
                                self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, source_ml_source2_data_property_value1019, _, _) => {
                                    source_ml_source2_data_property_value1019
                                }
                            });
                                {
                                    let ml_library_inequality_1 =
                                        scalar != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_1 {
                                        self::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Result::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Return(Box::new(self::RustStandardCollectionTransition::RustCollectionInvalid(scalar.clone())))
                                    } else {
                                        {
                                            let index_2 = index + 1f64;
                                            {
                                                index = index_2;
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Result::MlSource1TemporaryExecuteRustStandardCollectionOperationLoop31Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Result {
    MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Done(
        Vec<self::RustStandardCollectionScalar>,
    ),
    MlSource1TemporaryExecuteRustStandardCollectionOrderedSnapshotLoop34Return(
        Box<self::RustStandardCollectionTransition>,
    ),
}
