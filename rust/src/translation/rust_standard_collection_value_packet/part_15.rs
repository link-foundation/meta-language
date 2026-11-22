// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55;
use super::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result;
use super::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Result;
use super::RustStandardCollectionEqualityState;
use super::RustStandardCollectionEqualityTransition;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::advance_rust_standard_collection_equality;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::read_rust_standard_collection_equality_work;
use super::read_rust_standard_collection_scalar_equality_work;
use super::test_rust_standard_collection_scalar_equality;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop54(
    mut map: bool,
    mut work: f64,
    mut left: Option<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut found: bool,
    mut source_ml_source1_temporary_values48: Vec<(
        self::RustStandardCollectionScalar,
        Option<(Option<self::RustStandardCollectionScalar>,)>,
    )>,
    mut source_ml_source1_temporary_index48: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Result {
    loop {
        return if source_ml_source1_temporary_index48
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values48.len() as u128))
        {
            {
                let right = self::ml_array::at(
                    &source_ml_source1_temporary_values48,
                    source_ml_source1_temporary_index48.to_index(),
                );
                {
                    let work_2 = work
                        + self::read_rust_standard_collection_scalar_equality_work(
                            {
                                let source_ml_source2_nullish_field_receiver1058 = (left.clone(),);
                                if match (source_ml_source2_nullish_field_receiver1058.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source2_nullish_field_present1059 =
                                            ((source_ml_source2_nullish_field_receiver1058
                                                .clone())
                                            .0)
                                                .clone()
                                                .expect("a proven present value");
                                        (source_ml_source2_nullish_field_present1059).0
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (self::ml_string::Text::from_units(vec![
                                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32,
                                            112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32,
                                            111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101,
                                            100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39,
                                            107, 101, 121, 39, 41,
                                        ]))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            },
                            (right.clone()).0,
                        );
                    if self::test_rust_standard_collection_scalar_equality(
                        {
                            let source_ml_source2_nullish_field_receiver1060 = (left.clone(),);
                            if match (source_ml_source2_nullish_field_receiver1060.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present1061 =
                                        ((source_ml_source2_nullish_field_receiver1060.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    (source_ml_source2_nullish_field_present1061).0
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
                        (right.clone()).0,
                    ) {
                        {
                            let source_ml_source2_s1064 = if map {
                                {
                                    let left_value = {
                                        let source_ml_source2_nullish_field_receiver1065 =
                                            (left.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver1065
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
                                                let source_ml_source2_nullish_field_present1066 = ((source_ml_source2_nullish_field_receiver1065.clone()).0).clone().expect("a proven present value");
                                                ((((source_ml_source2_nullish_field_present1066).1).clone().unwrap_or_else(|| (None::<self::RustStandardCollectionScalar>,))).0).clone()
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
                                        let right_value =
                                            ((((right.clone()).1).clone().unwrap_or_else(|| {
                                                (None::<self::RustStandardCollectionScalar>,)
                                            }))
                                            .0)
                                                .clone();
                                        {
                                            let invalid = self::ml_string::Text::from_units(vec![
                                                101, 113, 117, 97, 108, 105, 116, 121, 32, 112,
                                                114, 101, 102, 105, 120, 32, 114, 101, 116, 97,
                                                105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                                101, 113, 117, 97, 108, 32, 111, 119, 110, 101,
                                                100, 32, 118, 97, 108, 117, 101, 115,
                                            ]);
                                            if match (left_value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55Return(invalid.clone())
                                            } else if match (right_value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55Return(invalid.clone())
                                                } else if (self::test_rust_standard_collection_scalar_equality(left_value.clone().expect("a proven present value"), right_value.clone().expect("a proven present value"))) {
                                                        {
                                                            let work_3 = work_2 + self::read_rust_standard_collection_scalar_equality_work(left_value.clone().expect("a proven present value"), right_value.clone().expect("a proven present value"));
                                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55Next(work_3)
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55Return(invalid.clone())
                                                    }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55Next(work_2)
                            };
                            match source_ml_source2_s1064 {
                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55Next(work_4) => {
                                    {
                                        let found_2 = true;
                                        self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Done(work_4, found_2)
                                    }
                                }
                                self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin55Return(source_ml_source1_temporary_result55) => {
                                    self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Return(source_ml_source1_temporary_result55.clone())
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_temporary_index48_2 =
                                source_ml_source1_temporary_index48
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (work, source_ml_source1_temporary_index48) =
                                    (work_2, source_ml_source1_temporary_index48_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Done(work, found)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop51(
    mut state: self::RustStandardCollectionEqualityState,
    mut map: bool,
    mut work: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result {
    loop {
        return if index
            < (match state.clone() {
                self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value1068,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value1068,
            }) {
            {
                let left = self::ml_array::optional_at(
                    &match match state.clone() {
                        self::RustStandardCollectionEqualityState::RustCollectionEqualityState(
                            source_ml_source2_data_property_value1069,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value1069 =
                                *source_ml_source2_data_property_value1069;
                            source_ml_source2_data_property_value1069.clone()
                        }
                    } {
                        self::RustStandardCollectionState::RustStandardCollectionState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value1070,
                        ) => source_ml_source2_data_property_value1070,
                    },
                    self::ml_array::number_index(index),
                );
                {
                    let found = false;
                    {
                        let source_ml_source1_temporary_values48 = match match state.clone() {
                            self::RustStandardCollectionEqualityState::RustCollectionEqualityState(_, source_ml_source2_data_property_value1071, _, _, _, _, _) => {
                                let source_ml_source2_data_property_value1071 = *source_ml_source2_data_property_value1071;
                                source_ml_source2_data_property_value1071.clone()
                            }
                        } {
                            self::RustStandardCollectionState::RustStandardCollectionState(_, _, _, _, _, _, source_ml_source2_data_property_value1072) => {
                                source_ml_source2_data_property_value1072
                            }
                        };
                        {
                            let source_ml_source1_temporary_index48 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s1073 = self::source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop54(map, work, left, found, source_ml_source1_temporary_values48, source_ml_source1_temporary_index48);
                                match source_ml_source2_s1073 {
                                    self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Done(work_2, found_2) => {
                                        if found_2 {
                                            {
                                                let index_2 = index + 1f64;
                                                {
                                                    (work, index) = (work_2, index_2);
                                                    continue;
                                                }
                                            }
                                        }
{
                                            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Return(self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108, 105, 116, 121, 32, 112, 114, 101, 102, 105, 120, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 101, 113, 117, 97, 108, 32, 109, 101, 109, 98, 101, 114, 115, 104, 105, 112]))
                                        }
                                    }
                                    self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop54Return(source_ml_source1_temporary_result54) => {
                                        self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Return(source_ml_source1_temporary_result54.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop51Done(work)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50 {
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50Next,
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorJoin50Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Result {
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Done,
    MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_collection_equality_state_error_loop49(
    source_ml_source1_temporary_values45: &[f64],
    mut source_ml_source1_temporary_index45: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Result {
    loop {
        return if source_ml_source1_temporary_index45
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values45.len() as u128))
        {
            {
                let coordinate = self::ml_array::at(
                    source_ml_source1_temporary_values45,
                    source_ml_source1_temporary_index45.to_index(),
                );
                if (!(self::ml_math::is_safe_integer(coordinate)) || (coordinate < 0f64)) || {
                    let source_ml_source2_same_value_operand1074 = coordinate;
                    {
                        let source_ml_source2_same_value_operand1075 = -0f64;
                        (((source_ml_source2_same_value_operand1074
                            == source_ml_source2_same_value_operand1075)
                            && ((source_ml_source2_same_value_operand1074 != 0f64)
                                || ((1f64 / source_ml_source2_same_value_operand1074)
                                    == (1f64 / source_ml_source2_same_value_operand1075))))
                            || ((source_ml_source2_same_value_operand1074
                                != source_ml_source2_same_value_operand1074)
                                && (source_ml_source2_same_value_operand1075
                                    != source_ml_source2_same_value_operand1075)))
                    }
                } {
                    self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 32, 110, 111, 110, 110, 101, 103, 97, 116, 105, 118, 101, 32, 99, 117, 114, 115, 111, 114, 115, 32, 97, 110, 100, 32, 99, 104, 97, 114, 103, 101, 100, 32, 119, 111, 114, 107]))
                } else {
                    {
                        let source_ml_source1_temporary_index45_2 =
                            source_ml_source1_temporary_index45.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index45 =
                                source_ml_source1_temporary_index45_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Result::MlSource1TemporaryReadRustStandardCollectionEqualityStateErrorLoop49Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56 {
    MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Next,
    MlSource1TemporaryReadRustStandardCollectionScalarEqualityWorkJoin56Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57 {
    MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57Next,
    MlSource1TemporaryReadRustStandardCollectionEqualityWorkJoin57Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_run_rust_standard_collection_equality_loop58(
    mut current: self::RustStandardCollectionEqualityState,
    mut remaining: f64,
) -> self::RustStandardCollectionEqualityTransition {
    loop {
        return {
            let cost = self::read_rust_standard_collection_equality_work(current.clone());
            if cost > remaining {
                self::RustStandardCollectionEqualityTransition::RustCollectionEqualityResourceLimit(
                    Box::new(current.clone()),
                    self::ml_string::Text::from_units(vec![
                        99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111,
                        110, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 101, 120, 99, 101, 101,
                        100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                        115, 99, 97, 108, 97, 114, 32, 97, 110, 100, 32, 83, 116, 114, 105, 110,
                        103, 32, 119, 111, 114, 107, 32, 98, 117, 100, 103, 101, 116,
                    ]),
                )
            } else {
                {
                    let transition =
                        self::advance_rust_standard_collection_equality(current.clone());
                    match transition.clone() {
                        self::RustStandardCollectionEqualityTransition::RustCollectionEqualityContinue(state) => {
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
                        self::RustStandardCollectionEqualityTransition::RustCollectionEqualityCompleted(source_ml_source2_f1076) => {
                            transition.clone()
                        }
                        self::RustStandardCollectionEqualityTransition::RustCollectionEqualityInvalid(source_ml_source2_f1077) => {
                            transition.clone()
                        }
                        self::RustStandardCollectionEqualityTransition::RustCollectionEqualityResourceLimit(source_ml_source2_f1078, source_ml_source2_f1079) => {
                            let source_ml_source2_f1078 = *source_ml_source2_f1078;
                            transition.clone()
                        }
                    }
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_encode_rust_standard_collection_text_loop59(
    value: &self::ml_string::Text,
    mut units: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (value.length()) {
            {
                let units_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![{
                            let source_ml_source2_string_unit_operand1080 = value.clone();
                            {
                                let source_ml_source2_string_unit_operand1081 = index;
                                (std::rc::Rc::new(
                                    move |text: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index_2 = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                    let source_ml_source2_string_unit_operand1087 = {
                                        let source_ml_source2_string_range_operand1082 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand1083 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand1084 = index_2 + 1f64;
                                                {
                                                    let source_ml_source2_string_range_position1085 = source_ml_source2_string_range_operand1083;
                                                    {
                                                        let source_ml_source2_string_range_position1086 = source_ml_source2_string_range_operand1084;
                                                        source_ml_source2_string_range_operand1082.slice(source_ml_source2_string_range_position1085, source_ml_source2_string_range_position1086)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand1088 = 0f64;
                                        source_ml_source2_string_unit_operand1087.code_point_at(source_ml_source2_string_unit_operand1088)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source2_string_unit_operand1080,
                                    source_ml_source2_string_unit_operand1081,
                                )
                            }
                        }],
                        units,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (units, index) = (units_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { units };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_decode_rust_standard_collection_text_loop61(
    mut value: self::ml_string::Text,
    source_ml_source1_temporary_values60: &[f64],
    mut source_ml_source1_temporary_index60: self::ml::Big,
) -> self::ml_string::Text {
    loop {
        if source_ml_source1_temporary_index60
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values60.len() as u128))
        {
            {
                let unit = self::ml_array::at(
                    source_ml_source1_temporary_values60,
                    source_ml_source1_temporary_index60.to_index(),
                );
                {
                    let value_2 = value.concat(&{
                        let source_ml_source2_character_argument1089 = unit;
                        self::ml_string::Text::from_units(
                            self::ml_utf16_text::character_units_utf16_text(&vec![
                                source_ml_source2_character_argument1089,
                            ])
                            .into_iter()
                            .map(|ml_unit| ml_unit as u16)
                            .collect(),
                        )
                    });
                    {
                        let source_ml_source1_temporary_index60_2 =
                            source_ml_source1_temporary_index60.add(&self::ml::Big::from_i128(1));
                        {
                            (value, source_ml_source1_temporary_index60) =
                                (value_2, source_ml_source1_temporary_index60_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { value.clone() };
    }
}
