// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlOptional;
use super::RustStandardCollectionCallOrigin;
use super::RustStandardCollectionEqualityOrigin;
use super::RustStandardCollectionEqualityTransition;
use super::RustStandardCollectionOrderingOrigin;
use super::RustStandardCollectionOrderingTransition;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionPacketTransition;
use super::RustStandardCollectionScalar;
use super::RustStandardCollectionState;
use super::RustStandardCollectionTransition;
use super::RustStandardCollectionValue;
use super::accept_rust_standard_collection_scalar_origin;
use super::create_rust_standard_collection_packet;
use super::execute_rust_standard_collection_equality;
use super::execute_rust_standard_collection_operation;
use super::execute_rust_standard_collection_ordered_borrow;
use super::execute_rust_standard_collection_ordered_snapshot;
use super::execute_rust_standard_collection_ordering;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::project_rust_standard_collection_option;
use super::read_required_number_fact;
use super::read_rust_standard_collection_domain_error;
use super::source_ml_source1_temporary_decode_rust_standard_collection_text_loop61;
use super::source_ml_source1_temporary_encode_rust_standard_collection_text_loop59;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_rust_standard_collection_value_packet(
    state: self::RustStandardCollectionState,
    method: &self::ml_string::Text,
    arguments_: Vec<self::RustStandardCollectionScalar>,
    call: self::RustStandardCollectionCallOrigin,
    projection: &self::ml_string::Text,
) -> self::RustStandardCollectionPacketTransition {
    {
        let ordered = self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![105, 116, 101, 114]),
                self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]),
            ],
            method,
            0f64,
            false,
            |a, b| a == b,
        );
        if ordered {
            {
                let ml_library_inequality_1 = (arguments_.len() as f64) != 0f64;
                if ml_library_inequality_1 {
                    self::RustStandardCollectionPacketTransition::RustCollectionInvalid(
                        self::ml_string::Text::from_units(vec![
                            111, 114, 100, 101, 114, 101, 100, 32, 105, 116, 101, 114, 97, 116,
                            111, 114, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 114,
                            101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                            97, 108, 32, 101, 109, 112, 116, 121, 32, 97, 114, 103, 117, 109, 101,
                            110, 116, 32, 108, 105, 115, 116,
                        ]),
                    )
                } else {
                    {
                        let result = if (*projection)
                            == (self::ml_string::Text::from_units(vec![
                                98, 111, 114, 114, 111, 119, 101, 100,
                            ])) {
                            self::execute_rust_standard_collection_ordered_borrow(
                                state.clone(),
                                method,
                                call.clone(),
                            )
                        } else {
                            self::execute_rust_standard_collection_ordered_snapshot(
                                state.clone(),
                                method,
                                call.clone(),
                                projection,
                            )
                        };
                        self::create_rust_standard_collection_packet(result)
                    }
                }
            }
        } else if (((*method) == (self::ml_string::Text::from_units(vec![103, 101, 116])))
            && !(self::ml_array::includes(
                &vec![
                    self::ml_string::Text::from_units(vec![99, 111, 112, 105, 101, 100]),
                    self::ml_string::Text::from_units(vec![99, 108, 111, 110, 101, 100]),
                ],
                projection,
                0f64,
                false,
                |a, b| a == b,
            )))
            || (((*method) != (self::ml_string::Text::from_units(vec![103, 101, 116])))
                && ((*projection) != (self::ml_string::Text::from_units(vec![]))))
        {
            self::RustStandardCollectionPacketTransition::RustCollectionInvalid(
                self::ml_string::Text::from_units(vec![
                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 111, 117, 114, 99,
                    101, 32, 112, 97, 99, 107, 101, 116, 115, 32, 114, 101, 113, 117, 105, 114,
                    101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 105, 109, 109, 101, 100,
                    105, 97, 116, 101, 32, 103, 101, 116, 32, 99, 111, 112, 105, 101, 100, 47, 99,
                    108, 111, 110, 101, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110,
                    32, 111, 114, 32, 110, 111, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111,
                    110,
                ]),
            )
        } else {
            {
                let result_2 = self::execute_rust_standard_collection_operation(
                    state.clone(),
                    method,
                    arguments_,
                    call.clone(),
                );
                match result_2.clone() {
                    self::RustStandardCollectionTransition::RustCollectionCreated(
                        source_ml_source2_f925,
                    ) => {
                        let source_ml_source2_f925 = *source_ml_source2_f925;
                        self::create_rust_standard_collection_packet(result_2.clone())
                    }
                    self::RustStandardCollectionTransition::RustCollectionNormal(
                        state_2,
                        value,
                        source_ml_source2_f926,
                    ) => {
                        let state_2 = *state_2;
                        let value = *value;
                        if (*method) == (self::ml_string::Text::from_units(vec![103, 101, 116])) {
                            if match value.clone() {
                                self::RustStandardCollectionValue::RustCollectionOrderedValues(
                                    source_ml_source2_f930,
                                ) => true,
                                self::RustStandardCollectionValue::RustCollectionOrderedEntries(
                                    source_ml_source2_f931,
                                ) => true,
                                self::RustStandardCollectionValue::RustCollectionUnit => true,
                                self::RustStandardCollectionValue::RustCollectionBoolean(
                                    source_ml_source2_f932,
                                ) => true,
                                self::RustStandardCollectionValue::RustCollectionInteger(
                                    source_ml_source2_f933,
                                ) => true,
                                self::RustStandardCollectionValue::RustCollectionString(
                                    source_ml_source2_f934,
                                ) => true,
                                self::RustStandardCollectionValue::RustCollectionOption(
                                    source_ml_source2_f935,
                                    source_ml_source2_f936,
                                ) => false,
                                self::RustStandardCollectionValue::RustStandardCollectionState(
                                    source_ml_source2_f937,
                                    source_ml_source2_f938,
                                    source_ml_source2_f939,
                                    source_ml_source2_f940,
                                    source_ml_source2_f941,
                                    source_ml_source2_f942,
                                    source_ml_source2_f943,
                                ) => {
                                    let source_ml_source2_f937 = *source_ml_source2_f937;
                                    true
                                }
                            } {
                                self::RustStandardCollectionPacketTransition::RustCollectionInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        103, 101, 116, 32, 112, 97, 99, 107, 101, 116, 115, 32,
                                        114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114,
                                        32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 114, 114, 111,
                                        119, 101, 100, 32, 79, 112, 116, 105, 111, 110, 32, 114,
                                        101, 115, 117, 108, 116,
                                    ]),
                                )
                            } else {
                                {
                                    let projected = self::project_rust_standard_collection_option(
                                        state_2.clone(),
                                        value.clone(),
                                        projection,
                                    );
                                    match projected.clone() {
                                            self::RustStandardCollectionTransition::RustCollectionCreated(source_ml_source2_f944) => {
                                                let source_ml_source2_f944 = *source_ml_source2_f944;
                                                self::RustStandardCollectionPacketTransition::RustCollectionInvalid(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 114, 109, 97, 108, 32, 111, 114, 32, 114, 101, 102, 117, 115, 97, 108, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110]))
                                            }
                                            self::RustStandardCollectionTransition::RustCollectionNormal(source_ml_source2_f945, source_ml_source2_f946, source_ml_source2_f947) => {
                                                let source_ml_source2_f945 = *source_ml_source2_f945;
                                                let source_ml_source2_f946 = *source_ml_source2_f946;
                                                self::create_rust_standard_collection_packet(projected.clone())
                                            }
                                            self::RustStandardCollectionTransition::RustCollectionInvalid(source_ml_source2_f948) => {
                                                self::create_rust_standard_collection_packet(projected.clone())
                                            }
                                            self::RustStandardCollectionTransition::RustCollectionResourceLimit(source_ml_source2_f949, source_ml_source2_f950) => {
                                                self::create_rust_standard_collection_packet(projected.clone())
                                            }
                                        }
                                }
                            }
                        } else {
                            self::create_rust_standard_collection_packet(result_2.clone())
                        }
                    }
                    self::RustStandardCollectionTransition::RustCollectionInvalid(
                        source_ml_source2_f927,
                    ) => self::create_rust_standard_collection_packet(result_2.clone()),
                    self::RustStandardCollectionTransition::RustCollectionResourceLimit(
                        source_ml_source2_f928,
                        source_ml_source2_f929,
                    ) => self::create_rust_standard_collection_packet(result_2.clone()),
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn encode_rust_standard_collection_text(
    value: &self::ml_string::Text,
) -> self::RustStandardCollectionScalar {
    {
        let units = Vec::<f64>::new();
        {
            let index = 0f64;
            {
                let units_2 =
                    self::source_ml_source1_temporary_encode_rust_standard_collection_text_loop59(
                        value, units, index,
                    );
                self::RustStandardCollectionScalar::RustCollectionString(units_2)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decode_rust_standard_collection_text(units: &[f64]) -> self::ml_string::Text {
    {
        let value = self::ml_string::Text::from_units(vec![]);
        {
            let source_ml_source1_temporary_values60 = units.to_vec();
            {
                let source_ml_source1_temporary_index60 = self::ml::Big::from_i128(0);
                {
                    self::source_ml_source1_temporary_decode_rust_standard_collection_text_loop61(
                        value,
                        &(source_ml_source1_temporary_values60),
                        source_ml_source1_temporary_index60,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_rust_standard_collection_value_packets(
    left: self::RustStandardCollectionState,
    right: self::RustStandardCollectionState,
    origin: self::RustStandardCollectionEqualityOrigin,
    budget: f64,
) -> self::RustStandardCollectionEqualityTransition {
    self::execute_rust_standard_collection_equality(
        left.clone(),
        right.clone(),
        origin.clone(),
        budget,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_rust_standard_collection_ordered_value_packets(
    left: self::RustStandardCollectionState,
    right: self::RustStandardCollectionState,
    origin: self::RustStandardCollectionOrderingOrigin,
    budget: f64,
) -> self::RustStandardCollectionOrderingTransition {
    self::execute_rust_standard_collection_ordering(
        left.clone(),
        right.clone(),
        origin.clone(),
        budget,
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Result {
    MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Done,
    MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_collection_source_origin_error_loop7(
    mut origin: self::RustStandardCollectionOrigin,
    key_domain: &self::ml_string::Text,
    value_domain: &self::ml_string::Text,
    mut source: self::MlOptional<(
        Vec<(
            Vec<(self::ml_string::Text, Vec<self::ml_string::Text>)>,
            self::ml_string::Text,
            Vec<self::ml_string::Text>,
        )>,
        self::ml_string::Text,
        Vec<self::ml_string::Text>,
    )>,
    mut count: f64,
    invalid_argument: &self::ml_string::Text,
    mut index: f64,
) -> self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Result {
    loop {
        return if index < count {
            {
                let child = self::ml_array::optional_at(
                    (match &origin {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value951,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value951.as_slice(),
                    }),
                    self::ml_array::number_index(index),
                );
                {
                    let declared = self::ml_array::optional_at(
                        &{
                            let source_ml_source2_nullish_field_receiver952 = (source.clone(),);
                            if match (source_ml_source2_nullish_field_receiver952.clone()).0 {
                                self::MlOptional::Value(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                self::MlOptional::Null => false,
                                self::MlOptional::Undefined => false,
                            } {
                                {
                                    let source_ml_source2_nullish_field_present953 =
                                        match (source_ml_source2_nullish_field_receiver952.clone())
                                            .0
                                        {
                                            self::MlOptional::Value(ml_present) => ml_present,
                                            _ => unreachable!("a proven present value"),
                                        };
                                    (source_ml_source2_nullish_field_present953).0
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units:
                                        (if (match (source_ml_source2_nullish_field_receiver952
                                            .clone())
                                        .0
                                        {
                                            self::MlOptional::Value(ml_present) => {
                                                let _ = ml_present;
                                                self::ml_string::Text::from_units(vec![
                                                    111, 98, 106, 101, 99, 116,
                                                ])
                                            }
                                            self::MlOptional::Null => {
                                                self::ml_string::Text::from_units(vec![
                                                    111, 98, 106, 101, 99, 116,
                                                ])
                                            }
                                            self::MlOptional::Undefined => {
                                                self::ml_string::Text::from_units(vec![
                                                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                                                ])
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])) {
                                            self::ml_string::Text::from_units(vec![
                                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100,
                                                32, 112, 114, 111, 112, 101, 114, 116, 105, 101,
                                                115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40,
                                                114, 101, 97, 100, 105, 110, 103, 32, 39, 97, 114,
                                                103, 117, 109, 101, 110, 116, 115, 39, 41,
                                            ])
                                        } else {
                                            self::ml_string::Text::from_units(vec![
                                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100,
                                                32, 112, 114, 111, 112, 101, 114, 116, 105, 101,
                                                115, 32, 111, 102, 32, 117, 110, 100, 101, 102,
                                                105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                                110, 103, 32, 39, 97, 114, 103, 117, 109, 101, 110,
                                                116, 115, 39, 41,
                                            ])
                                        })
                                        .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        },
                        self::ml_array::number_index(index),
                    );
                    if match (child.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Return(invalid_argument.clone())
                    } else if match (declared.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Return(invalid_argument.clone())
                    } else if ((!(self::accept_rust_standard_collection_scalar_origin(
                        child.clone().expect("a proven present value"),
                        &(if index == 0f64 {
                            key_domain.clone()
                        } else {
                            value_domain.clone()
                        }),
                    )) || (((declared.clone().expect("a proven present value")).1)
                        != (((child.clone().expect("a proven present value")).1).1)))
                        || ((self::ml_string::join(
                            &(declared.clone().expect("a proven present value")).2,
                            &self::ml_string::Text::from_units(vec![58, 58]),
                        )) != (self::ml_string::join(
                            &((child.clone().expect("a proven present value")).1).2,
                            &self::ml_string::Text::from_units(vec![58, 58]),
                        ))))
                        || ((((declared.clone().expect("a proven present value")).0).len() as f64)
                            != 0f64)
                    {
                        self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Return(invalid_argument.clone())
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
        } else {
            self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Result {
    MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Done,
    MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_collection_source_origin_error_loop6(
    mut pointer_width: f64,
    source_ml_source1_temporary_values5: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index5: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Result {
    loop {
        return if source_ml_source1_temporary_index5
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values5.len() as u128))
        {
            {
                let domain = self::ml_array::at(
                    source_ml_source1_temporary_values5,
                    source_ml_source1_temporary_index5.to_index(),
                );
                {
                    let error =
                        self::read_rust_standard_collection_domain_error(&(domain), pointer_width);
                    if error == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_source1_temporary_index5_2 =
                                source_ml_source1_temporary_index5
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index5 =
                                    source_ml_source1_temporary_index5_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Return(error.clone())
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9 {
    MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Next,
    MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13 {
    MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13Next,
    MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_standard_collection_state_packet_loop14(
    mut roots: f64,
    mut positions: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < roots {
            {
                let positions_2 = {
                    let (ml_append_right, ml_append_left) = (vec![(index + 1f64)], positions);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (positions, index) = (positions_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { positions };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Result {
    MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Done(f64, f64),
    MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compare_rust_standard_collection_text_units_loop15(
    left: &[f64],
    right: &[f64],
    mut first: f64,
    mut second: f64,
) -> self::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Result {
    loop {
        return if (first < (left.len() as f64)) && (second < (right.len() as f64)) {
            {
                let first_unit = self::read_required_number_fact(left, first);
                {
                    let second_unit = self::read_required_number_fact(right, second);
                    {
                        let first_pair = (55_296f64..=56_319f64).contains(&first_unit);
                        {
                            let second_pair = (55_296f64..=56_319f64).contains(&second_unit);
                            {
                                let first_scalar = if first_pair {
                                    ((65_536f64 + ((first_unit - 55_296f64) * 1024f64))
                                        + self::read_required_number_fact(left, (first + 1f64)))
                                        - 56_320f64
                                } else {
                                    first_unit
                                };
                                {
                                    let second_scalar = if second_pair {
                                        ((65_536f64 + ((second_unit - 55_296f64) * 1024f64))
                                            + self::read_required_number_fact(
                                                right,
                                                (second + 1f64),
                                            ))
                                            - 56_320f64
                                    } else {
                                        second_unit
                                    };
                                    {
                                        let ml_library_inequality_1 = first_scalar != second_scalar;
                                        if ml_library_inequality_1 {
                                            self::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Result::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Return(if first_scalar < second_scalar {
                                            -1f64
                                        } else {
                                            1f64
                                        })
                                        } else {
                                            {
                                                let first_2 =
                                                    first + if first_pair { 2f64 } else { 1f64 };
                                                {
                                                    let second_2 = second
                                                        + if second_pair { 2f64 } else { 1f64 };
                                                    {
                                                        (first, second) = (first_2, second_2);
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
                }
            }
        } else {
            self::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Result::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Done(first, second)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Result {
    MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Done,
    MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_owned_local_text_units_loop16(
    units: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let unit =
                    (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if (55_296f64..=56_319f64).contains(&unit) {
                    {
                        let index_2 = index + 1f64;
                        {
                            let next = (self::ml_array::optional_at(
                                units,
                                self::ml_array::number_index(index_2),
                            ))
                            .unwrap_or_else(|| -1f64);
                            if (56_320f64..=57_343f64).contains(&next) {
                                {
                                    let index_3 = index_2 + 1f64;
                                    {
                                        index = index_3;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Result::MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Return(false)
                            }
                        }
                    }
                } else if (56_320f64..=57_343f64).contains(&unit) {
                    self::MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Result::MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Return(false)
                } else {
                    {
                        let index_4 = index + 1f64;
                        {
                            index = index_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Result::MlSource1TemporaryAcceptRustOwnedLocalTextUnitsLoop16Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_owned_local_text_input_loop17(
    text: &self::ml_string::Text,
    mut units: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (text.length()) {
            {
                let units_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![{
                            let source_ml_source2_string_unit_operand954 = text.clone();
                            {
                                let source_ml_source2_string_unit_operand955 = index;
                                (std::rc::Rc::new(
                                    move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index_2 = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                    let source_ml_source2_string_unit_operand961 = {
                                        let source_ml_source2_string_range_operand956 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand957 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand958 = index_2 + 1f64;
                                                {
                                                    let source_ml_source2_string_range_position959 = source_ml_source2_string_range_operand957;
                                                    {
                                                        let source_ml_source2_string_range_position960 = source_ml_source2_string_range_operand958;
                                                        source_ml_source2_string_range_operand956.slice(source_ml_source2_string_range_position959, source_ml_source2_string_range_position960)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand962 = 0f64;
                                        source_ml_source2_string_unit_operand961.code_point_at(source_ml_source2_string_unit_operand962)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source2_string_unit_operand954,
                                    source_ml_source2_string_unit_operand955,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Result {
    MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Done,
    MlSource1TemporaryTestRustStandardCollectionScalarEqualityLoop18Return(bool),
}
