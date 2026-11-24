// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Result;
use super::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13;
use super::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9;
use super::RustStandardCollectionCallOrigin;
use super::RustStandardCollectionOrigin;
use super::RustStandardCollectionScalar;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_rust_standard_collection_method_arity;
use super::resolve_rust_standard_collection_owner;
use super::source_ml_source1_temporary_compare_rust_standard_collection_text_units_loop15;
use super::source_ml_source1_temporary_plan_rust_standard_collection_state_packet_loop14;
use super::test_rust_standard_collection_mutation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_call_origin_error(
    origin: self::RustStandardCollectionOrigin,
    root: &self::ml_string::Text,
    method: &self::ml_string::Text,
    arguments_count: f64,
    call: self::RustStandardCollectionCallOrigin,
) -> self::ml_string::Text {
    {
        let owner = self::resolve_rust_standard_collection_owner(
            &(match origin.clone() {
                self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value38,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value38.clone(),
            }),
            match origin.clone() {
                self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                    source_ml_source2_data_property_value39,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value39,
            },
            match origin.clone() {
                self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                    _,
                    _,
                    source_ml_source2_data_property_value40,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value40,
            },
        );
        {
            let arity = self::read_rust_standard_collection_method_arity(&(owner), method);
            if arity < 0f64 {
                self::ml_string::Text::from_units(vec![
                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97,
                    116, 105, 111, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 99, 117, 115,
                    116, 111, 109, 32, 72, 97, 115, 104, 47, 69, 113, 32, 97, 110, 100, 32, 117,
                    110, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 109, 101, 116, 104,
                    111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105,
                    114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 112, 97, 114, 97, 116, 101,
                    32, 115, 101, 109, 97, 110, 116, 105, 99, 115,
                ])
            } else if (((((((((({
                let source_ml_source2_optional_chain_receiver42 = (call.clone(),);
                {
                    let source_ml_source1_temporary_optional_chain8 =
                        (source_ml_source2_optional_chain_receiver42).0;
                    match source_ml_source1_temporary_optional_chain8 {
                            self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                self::ml_string::Text::from_units(vec![114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 67, 97, 108, 108, 79, 114, 105, 103, 105, 110])
                            }
                        }
                }
            }) != (self::ml_string::Text::from_units(vec![
                114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101,
                99, 116, 105, 111, 110, 67, 97, 108, 108, 79, 114, 105, 103, 105, 110,
            ]))) || !(match call.clone() {
                self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value43,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value43,
            })) || match call.clone() {
                self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value44,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value44,
            }) || (!(self::ml_array::includes(
                &vec![
                    self::ml_string::Text::from_units(vec![110, 101, 119]),
                    self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]),
                ],
                method,
                0f64,
                false,
                |a, b| a == b,
            )) && !(match call.clone() {
                self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value45,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value45,
            }))) || ((match call.clone() {
                self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(
                    source_ml_source2_data_property_value47,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value47.clone(),
            }) != owner))
                || ((match call.clone() {
                    self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(
                        _,
                        source_ml_source2_data_property_value50,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value50.clone(),
                }) != (match origin.clone() {
                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value51,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value51.clone(),
                })))
                || ((match call.clone() {
                    self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(
                        _,
                        _,
                        source_ml_source2_data_property_value53,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value53.clone(),
                }) != (*root)))
                || ((match call.clone() {
                    self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value55,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value55.clone(),
                }) != (*method)))
                || (arity != arguments_count))
                || (((match &call {
                    self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value57,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value57.as_slice(),
                })
                .len() as f64)
                    != arguments_count)
            {
                self::ml_string::Text::from_units(vec![
                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 115, 32,
                    114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99,
                    108, 97, 114, 97, 116, 105, 111, 110, 32, 116, 121, 112, 101, 32, 114, 101, 99,
                    101, 105, 118, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103, 32, 97, 118, 97,
                    105, 108, 97, 98, 105, 108, 105, 116, 121, 32, 97, 110, 100, 32, 99, 111, 109,
                    112, 108, 101, 116, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 119,
                    105, 116, 110, 101, 115, 115, 101, 115,
                ])
            } else {
                {
                    let dispatch = match call.clone() {
                            self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, source_ml_source2_data_property_value58, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value58.clone()
                            }
                        };
                    {
                        let source_ml_source2_s74 = if (*method)
                            == (self::ml_string::Text::from_units(vec![99, 108, 111, 110, 101]))
                        {
                            if (self::ml_array::includes(
                                &vec![
                                    self::ml_string::Text::from_units(vec![
                                        115, 116, 100, 58, 58, 99, 108, 111, 110, 101, 58, 58, 67,
                                        108, 111, 110, 101,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        99, 111, 114, 101, 58, 58, 99, 108, 111, 110, 101, 58, 58,
                                        67, 108, 111, 110, 101,
                                    ]),
                                ],
                                &dispatch.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            )) {
                                if (((match match call.clone() {
                                        self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value76, _) => {
                                            ((source_ml_source2_data_property_value76.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                        }
                                    } { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (({
                                        let source_ml_source2_nullish_field_receiver81 = (match call.clone() {
                                            self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value80, _) => {
                                                ((source_ml_source2_data_property_value80.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                            }
                                        },);
                                        if (match (source_ml_source2_nullish_field_receiver81.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) {
                                            {
                                                let source_ml_source2_nullish_field_present82 = ((source_ml_source2_nullish_field_receiver81.clone()).0).clone().expect("a proven present value");
                                                source_ml_source2_nullish_field_present82.length()
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                        }
                                    }) == 0f64)) || {
                                        let source_ml_source2_comparison_operand84 = match call.clone() {
                                            self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value83, _) => {
                                                ((source_ml_source2_data_property_value83.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_comparison_operand85 = root.clone();
                                            match (source_ml_source2_comparison_operand84, source_ml_source2_comparison_operand85) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        }
                                    } {
                                        self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 108, 111, 110, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110, 116, 32, 102, 114, 101, 115, 104, 32, 111, 117, 116, 112, 117, 116, 32, 98, 105, 110, 100, 105, 110, 103, 32, 114, 111, 111, 116]))
                                    } else {
                                        self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Next
                                    }
                            } else {
                                self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 108, 111, 110, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 67, 108, 111, 110, 101, 32, 116, 114, 97, 105, 116, 32, 100, 105, 115, 112, 97, 116, 99, 104]))
                            }
                        } else if (*method)
                            == (self::ml_string::Text::from_units(vec![
                                100, 101, 102, 97, 117, 108, 116,
                            ]))
                        {
                            if (self::ml_array::includes(
                                &vec![
                                    self::ml_string::Text::from_units(vec![
                                        115, 116, 100, 58, 58, 100, 101, 102, 97, 117, 108, 116,
                                        58, 58, 68, 101, 102, 97, 117, 108, 116,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        99, 111, 114, 101, 58, 58, 100, 101, 102, 97, 117, 108,
                                        116, 58, 58, 68, 101, 102, 97, 117, 108, 116,
                                    ]),
                                ],
                                &dispatch.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            )) {
                                self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Next
                            } else {
                                self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 100, 101, 102, 97, 117, 108, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 68, 101, 102, 97, 117, 108, 116, 32, 116, 114, 97, 105, 116, 32, 100, 105, 115, 112, 97, 116, 99, 104]))
                            }
                        } else if (*method)
                            == (self::ml_string::Text::from_units(vec![
                                105, 110, 116, 111, 95, 105, 116, 101, 114,
                            ]))
                        {
                            if (!(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![115, 116, 100, 58, 58, 105, 116, 101, 114, 58, 58, 73, 110, 116, 111, 73, 116, 101, 114, 97, 116, 111, 114]), self::ml_string::Text::from_units(vec![99, 111, 114, 101, 58, 58, 105, 116, 101, 114, 58, 58, 73, 110, 116, 111, 73, 116, 101, 114, 97, 116, 111, 114])], &dispatch.clone(), 0f64, false, |a, b| a == b)) || {
                                            let source_ml_source2_comparison_operand87 = match call.clone() {
                                                self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value86) => {
                                                    ((source_ml_source2_data_property_value86.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_comparison_operand88 = true;
                                                !(match (source_ml_source2_comparison_operand87, source_ml_source2_comparison_operand88) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                            }
                                        }) || match call.clone() {
                                            self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value89, _, _, _) => {
                                                source_ml_source2_data_property_value89
                                            }
                                        } {
                                            self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Return(self::ml_string::Text::from_units(vec![111, 114, 100, 101, 114, 101, 100, 32, 73, 110, 116, 111, 73, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 116, 114, 97, 110, 115, 102, 101, 114, 32, 97, 110, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 116, 114, 97, 105, 116, 32, 100, 105, 115, 112, 97, 116, 99, 104]))
                                        } else {
                                            self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Next
                                        }
                        } else {
                            let ml_library_inequality_1 =
                                (self::resolve_rust_standard_collection_owner(
                                    &(dispatch),
                                    true,
                                    false,
                                )) != owner;
                            if ml_library_inequality_1 {
                                self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 104, 101, 114, 101, 110, 116, 32, 99, 97, 108, 108, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114]))
                            } else {
                                self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Next
                            }
                        };
                        match source_ml_source2_s74 {
                                self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Next => {
                                    if self::test_rust_standard_collection_mutation(&(owner), method) && (!(match call.clone() {
                                        self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, source_ml_source2_data_property_value90, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value90
                                        }
                                    }) || match call.clone() {
                                        self::RustStandardCollectionCallOrigin::RustStandardCollectionCallOrigin(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value91, _, _, _) => {
                                            source_ml_source2_data_property_value91
                                        }
                                    }) {
                                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 109, 117, 116, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 114, 111, 111, 116, 32, 119, 105, 116, 104, 111, 117, 116, 32, 111, 118, 101, 114, 108, 97, 112, 112, 105, 110, 103, 32, 108, 105, 118, 101, 32, 108, 111, 97, 110, 115])
                                    } else {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                }
                                self::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9::MlSource1TemporaryReadRustStandardCollectionCallOriginErrorJoin9Return(source_ml_source1_temporary_result9) => {
                                    source_ml_source1_temporary_result9.clone()
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
pub fn read_rust_standard_collection_argument_origin_error(
    argument: Option<(
        bool,
        bool,
        self::ml_string::Text,
        bool,
        bool,
        (
            Vec<(self::ml_string::Text, Vec<self::ml_string::Text>)>,
            self::ml_string::Text,
            Vec<self::ml_string::Text>,
        ),
    )>,
    expected: &self::ml_string::Text,
    borrowed: bool,
) -> self::ml_string::Text {
    {
        let invalid = self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 114, 103, 117, 109, 101, 110,
            116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99,
            116, 117, 97, 108, 32, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 116, 121, 112, 101,
            100, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 98, 111, 114, 114, 111, 119,
        ]);
        if match (argument.clone(), None::<()>) {
            (Some(_ml_left), Some(_ml_right)) => false,
            (None, None) => true,
            _ => false,
        } {
            invalid.clone()
        } else if (!({
            let source_ml_source2_optional_chain_receiver92 =
                (argument.clone().expect("a proven present value"),);
            {
                let source_ml_source1_temporary_optional_chain10 =
                    (source_ml_source2_optional_chain_receiver92).0;
                (source_ml_source1_temporary_optional_chain10).4
            }
        }) || !((argument.clone().expect("a proven present value")).0))
            || (((argument.clone().expect("a proven present value")).1) != borrowed)
        {
            invalid.clone()
        } else {
            {
                let domain = (argument.clone().expect("a proven present value")).2;
                {
                    let owner = (argument.clone().expect("a proven present value")).5;
                    {
                        let path = {
                            let source_ml_source2_optional_chain_receiver93 = (owner.clone(),);
                            {
                                let source_ml_source1_temporary_optional_chain11 =
                                    (source_ml_source2_optional_chain_receiver93).0;
                                self::ml_string::join(
                                    &(source_ml_source1_temporary_optional_chain11).2,
                                    &self::ml_string::Text::from_units(vec![58, 58]),
                                )
                            }
                        };
                        if (({
                            let source_ml_source2_optional_chain_receiver95 = (owner.clone(),);
                            {
                                let source_ml_source1_temporary_optional_chain12 =
                                    (source_ml_source2_optional_chain_receiver95).0;
                                (((source_ml_source1_temporary_optional_chain12).0).len() as f64)
                            }
                        }) != 0f64)
                            || ((domain != (*expected))
                                && !((borrowed
                                    && ((*expected)
                                        == (self::ml_string::Text::from_units(vec![
                                            83, 116, 114, 105, 110, 103,
                                        ]))))
                                    && (domain
                                        == (self::ml_string::Text::from_units(vec![
                                            115, 116, 114,
                                        ])))))
                        {
                            self::ml_string::Text::from_units(vec![
                                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 114, 103,
                                117, 109, 101, 110, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32,
                                101, 120, 97, 99, 116, 32, 99, 108, 111, 115, 101, 100, 32, 100,
                                111, 109, 97, 105, 110, 32, 105, 100, 101, 110, 116, 105, 116, 105,
                                101, 115, 32, 119, 105, 116, 104, 111, 117, 116, 32, 99, 111, 101,
                                114, 99, 105, 111, 110,
                            ])
                        } else {
                            {
                                let source_ml_source2_s96 = if domain
                                    == (self::ml_string::Text::from_units(vec![
                                        83, 116, 114, 105, 110, 103,
                                    ])) {
                                    if (((owner.clone()).1)
                                        != (self::ml_string::Text::from_units(vec![
                                            108, 105, 98, 114, 97, 114, 121,
                                        ])))
                                        || !(self::ml_array::includes(
                                            &vec![
                                                self::ml_string::Text::from_units(vec![
                                                    115, 116, 100, 58, 58, 115, 116, 114, 105, 110,
                                                    103, 58, 58, 83, 116, 114, 105, 110, 103,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    97, 108, 108, 111, 99, 58, 58, 115, 116, 114,
                                                    105, 110, 103, 58, 58, 83, 116, 114, 105, 110,
                                                    103,
                                                ]),
                                            ],
                                            &path.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        ))
                                    {
                                        self::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 83, 116, 114, 105, 110, 103, 32, 116, 121, 112, 101]))
                                    } else if ((borrowed)
                                        || ((argument.clone().expect("a proven present value")).3))
                                    {
                                        self::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13Next
                                    } else {
                                        self::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 116, 114, 97, 110, 115, 102, 101, 114]))
                                    }
                                } else if (((owner.clone()).1)
                                    != (self::ml_string::Text::from_units(vec![
                                        112, 114, 105, 109, 105, 116, 105, 118, 101,
                                    ])))
                                    || !(self::ml_array::includes(
                                        &vec![
                                            domain.clone(),
                                            (self::ml_string::Text::from_units(vec![
                                                115, 116, 100, 58, 58, 112, 114, 105, 109, 105,
                                                116, 105, 118, 101, 58, 58,
                                            ]))
                                            .concat(&domain),
                                            (self::ml_string::Text::from_units(vec![
                                                99, 111, 114, 101, 58, 58, 112, 114, 105, 109, 105,
                                                116, 105, 118, 101, 58, 58,
                                            ]))
                                            .concat(&domain),
                                        ],
                                        &path.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ))
                                {
                                    self::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 99, 97, 108, 97, 114, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 115]))
                                } else {
                                    self::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13Next
                                };
                                match source_ml_source2_s96 {
                                        self::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13Next => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13::MlSource1TemporaryReadRustStandardCollectionArgumentOriginErrorJoin13Return(source_ml_source1_temporary_result13) => {
                                            source_ml_source1_temporary_result13.clone()
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
pub fn plan_rust_standard_collection_state_packet(roots: f64) -> Vec<f64> {
    if (!(self::ml_math::is_integer(roots)) || (roots < 0f64)) || (roots > 256f64) {
        Vec::<f64>::new()
    } else {
        {
            let positions = vec![0f64];
            {
                let index = 0f64;
                {
                    self::source_ml_source1_temporary_plan_rust_standard_collection_state_packet_loop14(roots, positions, index)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78,
            117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118,
            97, 108, 117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> self::ml_string::Text {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116,
            101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108,
            117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66,
            111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32,
            118, 97, 108, 117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110,
                32, 112, 111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(
    values: &[self::ml_string::Text],
    index: f64,
) -> self::ml_string::Text {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112,
                111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109,
                110, 32, 112, 111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_rust_standard_collection_scalars(
    left: self::RustStandardCollectionScalar,
    right: self::RustStandardCollectionScalar,
) -> f64 {
    match left.clone() {
        self::RustStandardCollectionScalar::RustCollectionBoolean(left_value) => {
            match right.clone() {
                self::RustStandardCollectionScalar::RustCollectionBoolean(value) => {
                    if left_value == value {
                        0f64
                    } else if left_value {
                        1f64
                    } else {
                        -1f64
                    }
                }
                self::RustStandardCollectionScalar::RustCollectionInteger(
                    source_ml_source2_f98,
                ) => 0f64,
                self::RustStandardCollectionScalar::RustCollectionString(source_ml_source2_f99) => {
                    0f64
                }
            }
        }
        self::RustStandardCollectionScalar::RustCollectionInteger(left_value_2) => {
            match right.clone() {
                self::RustStandardCollectionScalar::RustCollectionBoolean(
                    source_ml_source2_f100,
                ) => 0f64,
                self::RustStandardCollectionScalar::RustCollectionInteger(value_2) => {
                    if left_value_2 == value_2 {
                        0f64
                    } else if left_value_2 < value_2 {
                        -1f64
                    } else {
                        1f64
                    }
                }
                self::RustStandardCollectionScalar::RustCollectionString(
                    source_ml_source2_f101,
                ) => 0f64,
            }
        }
        self::RustStandardCollectionScalar::RustCollectionString(source_ml_source2_f97) => {
            let left_units = source_ml_source2_f97;
            match right.clone() {
                self::RustStandardCollectionScalar::RustCollectionBoolean(
                    source_ml_source2_f104,
                ) => 0f64,
                self::RustStandardCollectionScalar::RustCollectionInteger(
                    source_ml_source2_f105,
                ) => 0f64,
                self::RustStandardCollectionScalar::RustCollectionString(units) => {
                    self::compare_rust_standard_collection_text_units(&(left_units), &(units))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_rust_standard_collection_text_units(left: &[f64], right: &[f64]) -> f64 {
    {
        let first = 0f64;
        {
            let second = 0f64;
            {
                let source_ml_source2_s106 = self::source_ml_source1_temporary_compare_rust_standard_collection_text_units_loop15(left, right, first, second);
                match source_ml_source2_s106 {
                    self::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Result::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Done(first_2, second_2) => {
                        if first_2 == (left.len() as f64) {
                            if second_2 == (right.len() as f64) {
                                0f64
                            } else {
                                -1f64
                            }
                        } else {
                            1f64
                        }
                    }
                    self::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Result::MlSource1TemporaryCompareRustStandardCollectionTextUnitsLoop15Return(source_ml_source1_temporary_result15) => {
                        source_ml_source1_temporary_result15
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_local_state_model(model: &self::ml_string::Text) -> bool {
    (((*model) == (self::ml_string::Text::from_units(vec![])))
        || ((*model)
            == (self::ml_string::Text::from_units(vec![
                99, 108, 111, 115, 101, 100, 32, 111, 119, 110, 101, 100, 32, 116, 101, 120, 116,
                32, 108, 111, 99, 97, 108, 32, 115, 116, 97, 116, 101, 32, 49,
            ]))))
        || ((*model)
            == (self::ml_string::Text::from_units(vec![
                99, 97, 110, 111, 110, 105, 99, 97, 108, 32, 111, 119, 110, 101, 100, 32, 116, 101,
                120, 116, 32, 108, 111, 99, 97, 108, 32, 99, 111, 112, 105, 101, 115, 32, 49,
            ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_local_value_type(
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
) -> bool {
    (((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
        || ((((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![51, 50]))))
            && signed)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_local_expression(kind: &self::ml_string::Text) -> bool {
    ((((((((*kind) == (self::ml_string::Text::from_units(vec![108, 105, 116])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![
                116, 117, 112, 108, 101, 73, 110, 100, 101, 120,
            ]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![108, 101, 116]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![105, 102]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]))))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_local_binary_operation(operation: &self::ml_string::Text) -> bool {
    (*operation) == (self::ml_string::Text::from_units(vec![101, 113]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_local_source_operation(operation: &self::ml_string::Text) -> bool {
    (((*operation) != (self::ml_string::Text::from_units(vec![98, 111, 114, 114, 111, 119])))
        && ((*operation)
            != (self::ml_string::Text::from_units(vec![
                100, 101, 114, 101, 102, 101, 114, 101, 110, 99, 101,
            ]))))
        && ((*operation)
            != (self::ml_string::Text::from_units(vec![
                98, 111, 120, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
            ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_local_container(
    kind: &self::ml_string::Text,
    dense: bool,
    unrepresented: bool,
) -> bool {
    (((((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) && dense)
        && !unrepresented)
        || (((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && !unrepresented))
        || (((*kind) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])))
            && !unrepresented)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_owned_local_record_value(
    boolean_required: bool,
    boolean_value: bool,
) -> bool {
    !boolean_required || boolean_value
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_owned_local_source_span(
    present: bool,
    record_shape: bool,
    offset_shape: bool,
) -> bool {
    !present || (record_shape && offset_shape)
}
