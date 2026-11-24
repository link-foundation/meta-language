// Generated from complete js/src/translation/rust-standard-collection-value-packet.js. Do not edit by hand.
use super::MlOptional;
use super::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Result;
use super::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Result;
use super::RustStandardCollectionOrigin;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_rust_standard_collection_kind;
use super::read_rust_standard_collection_source_version_error;
use super::resolve_rust_standard_collection_owner;
use super::source_ml_source1_temporary_read_rust_standard_collection_source_origin_error_loop6;
use super::source_ml_source1_temporary_read_rust_standard_collection_source_origin_error_loop7;
use super::test_rust_standard_collection_ordered;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_collection_scalar_origin(
    origin: (
        bool,
        (
            Vec<(self::ml_string::Text, Vec<self::ml_string::Text>)>,
            self::ml_string::Text,
            Vec<self::ml_string::Text>,
        ),
        Option<(Option<self::ml_string::Text>,)>,
        Vec<self::ml_string::Text>,
    ),
    domain: &self::ml_string::Text,
) -> bool {
    if (!({
        let source_ml_source2_optional_chain_receiver5 = (origin.clone(),);
        {
            let source_ml_source1_temporary_optional_chain1 =
                (source_ml_source2_optional_chain_receiver5).0;
            (source_ml_source1_temporary_optional_chain1).0
        }
    }) || ((((origin.clone()).3).len() as f64) != 0f64))
        || (({
            let source_ml_source2_optional_chain_receiver7 = ((origin.clone()).1,);
            {
                let source_ml_source1_temporary_optional_chain2 =
                    (source_ml_source2_optional_chain_receiver7).0;
                (((source_ml_source1_temporary_optional_chain2).0).len() as f64)
            }
        }) != 0f64)
    {
        false
    } else {
        {
            let owner = (origin.clone()).1;
            {
                let path = self::ml_string::join(
                    &(owner.clone()).2,
                    &self::ml_string::Text::from_units(vec![58, 58]),
                );
                if (*domain)
                    == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))
                {
                    ((((owner.clone()).1)
                        == (self::ml_string::Text::from_units(vec![
                            108, 105, 98, 114, 97, 114, 121,
                        ])))
                        && self::ml_array::includes(
                            &vec![
                                self::ml_string::Text::from_units(vec![
                                    115, 116, 100, 58, 58, 115, 116, 114, 105, 110, 103, 58, 58,
                                    83, 116, 114, 105, 110, 103,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    97, 108, 108, 111, 99, 58, 58, 115, 116, 114, 105, 110, 103,
                                    58, 58, 83, 116, 114, 105, 110, 103,
                                ]),
                            ],
                            &path.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))
                        && {
                            let source_ml_source2_comparison_operand8 = ((((origin.clone()).2)
                                .clone()
                                .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                            .0)
                                .clone();
                            {
                                let source_ml_source2_comparison_operand9 =
                                    self::ml_string::Text::from_units(vec![
                                        111, 119, 110, 101, 100, 32, 115, 116, 114, 105, 110, 103,
                                    ]);
                                match (
                                    source_ml_source2_comparison_operand8,
                                    source_ml_source2_comparison_operand9,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }
                } else {
                    (((owner.clone()).1)
                        == (self::ml_string::Text::from_units(vec![
                            112, 114, 105, 109, 105, 116, 105, 118, 101,
                        ])))
                        && self::ml_array::includes(
                            &vec![
                                domain.clone(),
                                (self::ml_string::Text::from_units(vec![
                                    115, 116, 100, 58, 58, 112, 114, 105, 109, 105, 116, 105, 118,
                                    101, 58, 58,
                                ]))
                                .concat(domain),
                                (self::ml_string::Text::from_units(vec![
                                    99, 111, 114, 101, 58, 58, 112, 114, 105, 109, 105, 116, 105,
                                    118, 101, 58, 58,
                                ]))
                                .concat(domain),
                            ],
                            &path.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_source_origin_error(
    origin: self::RustStandardCollectionOrigin,
    key_domain: &self::ml_string::Text,
    value_domain: &self::ml_string::Text,
    pointer_width: f64,
) -> self::ml_string::Text {
    if (((((({
        let source_ml_source2_optional_chain_receiver11 = (origin.clone(),);
        {
            let source_ml_source1_temporary_optional_chain3 =
                (source_ml_source2_optional_chain_receiver11).0;
            match source_ml_source1_temporary_optional_chain3 {
                self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
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
                ) => self::ml_string::Text::from_units(vec![
                    114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108,
                    101, 99, 116, 105, 111, 110, 79, 114, 105, 103, 105, 110,
                ]),
            }
        }
    }) != (self::ml_string::Text::from_units(vec![
        114, 117, 115, 116, 83, 116, 97, 110, 100, 97, 114, 100, 67, 111, 108, 108, 101, 99, 116,
        105, 111, 110, 79, 114, 105, 103, 105, 110,
    ]))) || !(match origin.clone() {
        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
            source_ml_source2_data_property_value12,
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
        ) => source_ml_source2_data_property_value12,
    })) || !(match origin.clone() {
        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
            _,
            source_ml_source2_data_property_value13,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value13,
    })) || match origin.clone() {
        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
            _,
            _,
            source_ml_source2_data_property_value14,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value14,
    }) || (({
        let source_ml_source2_type_query_operand18 = match origin.clone() {
            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                _,
                _,
                _,
                source_ml_source2_data_property_value17,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source2_data_property_value17.clone(),
        };
        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
        || (((match origin.clone() {
            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                _,
                _,
                _,
                source_ml_source2_data_property_value20,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => source_ml_source2_data_property_value20.clone(),
        })
        .length())
            == 0f64)
    {
        self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114,
            101, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 118, 101, 110, 32, 99, 111, 110,
            99, 114, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32,
            97, 110, 100, 32, 117, 110, 115, 104, 97, 100, 111, 119, 101, 100, 32, 105, 109, 112,
            111, 114, 116, 45, 111, 119, 110, 101, 114, 32, 119, 105, 116, 110, 101, 115, 115, 101,
            115,
        ])
    } else {
        {
            let owner = self::resolve_rust_standard_collection_owner(
                &(match origin.clone() {
                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value21,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value21.clone(),
                }),
                match origin.clone() {
                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                        source_ml_source2_data_property_value22,
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
                    ) => source_ml_source2_data_property_value22,
                },
                match origin.clone() {
                    self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                        _,
                        _,
                        source_ml_source2_data_property_value23,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value23,
                },
            );
            {
                let kind = self::read_rust_standard_collection_kind(&(owner));
                {
                    let source = match origin.clone() {
                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value24,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value24.clone(),
                    };
                    if ((kind == (self::ml_string::Text::from_units(vec![])))
                        || (({
                            let source_ml_source2_optional_chain_receiver26 = (source.clone(),);
                            {
                                let source_ml_source1_temporary_optional_chain4 =
                                    (source_ml_source2_optional_chain_receiver26).0;
                                (source_ml_source1_temporary_optional_chain4).1
                            }
                        }) != (self::ml_string::Text::from_units(vec![
                            108, 105, 98, 114, 97, 114, 121,
                        ]))))
                        || ((self::resolve_rust_standard_collection_owner(
                            &(self::ml_string::join(
                                &(source.clone()).2,
                                &self::ml_string::Text::from_units(vec![58, 58]),
                            )),
                            true,
                            false,
                        )) != owner)
                    {
                        self::ml_string::Text::from_units(vec![
                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 111, 117, 114,
                            99, 101, 32, 84, 89, 80, 69, 32, 111, 119, 110, 101, 114, 32, 109, 117,
                            115, 116, 32, 109, 97, 116, 99, 104, 32, 116, 104, 101, 32, 97, 99,
                            116, 117, 97, 108, 32, 114, 101, 115, 111, 108, 118, 101, 100, 32, 115,
                            116, 97, 110, 100, 97, 114, 100, 32, 100, 101, 99, 108, 97, 114, 97,
                            116, 105, 111, 110,
                        ])
                    } else {
                        {
                            let count = if kind
                                == (self::ml_string::Text::from_units(vec![109, 97, 112]))
                            {
                                2f64
                            } else {
                                1f64
                            };
                            if (((((match &origin { self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, _, _, source_ml_source2_data_property_value28, _, _, _, _) => source_ml_source2_data_property_value28.as_slice() }).len() as f64) != count) || ((((source.clone()).0).len() as f64) != count)) || ((match origin.clone() {
                                self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value30, _) => {
                                    source_ml_source2_data_property_value30
                                }
                            }) != (false))) || ((match origin.clone() {
                                self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value32) => {
                                    source_ml_source2_data_property_value32
                                }
                            }) != (false)) {
                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 116, 121, 112, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 102, 97, 117, 108, 116, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 119, 105, 116, 104, 32, 99, 108, 111, 115, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 69, 113, 47, 72, 97, 115, 104, 47, 79, 114, 100])
                            } else {
                                {
                                    let version = self::read_rust_standard_collection_source_version_error(match origin.clone() {
                                        self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, _, _, _, source_ml_source2_data_property_value33, _, _, _) => {
                                            source_ml_source2_data_property_value33
                                        }
                                    });
                                    {
let ml_library_inequality_1 = version != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                        version.clone()
                                    } else if (match origin.clone() {
                                            self::RustStandardCollectionOrigin::RustStandardCollectionOrigin(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value35, _, _) => {
                                                source_ml_source2_data_property_value35
                                            }
                                        }) != pointer_width {
                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 97, 99, 116, 105, 118, 101, 32, 116, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 112, 111, 105, 110, 116, 101, 114, 32, 119, 105, 100, 116, 104])
                                        } else {
                                            {
                                                let source_ml_source1_temporary_values5 = if kind == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                                                    vec![key_domain.clone(), value_domain.clone()]
                                                } else {
                                                    vec![key_domain.clone()]
                                                };
                                                {
                                                    let source_ml_source1_temporary_index5 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s36 = self::source_ml_source1_temporary_read_rust_standard_collection_source_origin_error_loop6(pointer_width, &(source_ml_source1_temporary_values5), source_ml_source1_temporary_index5);
                                                        match source_ml_source2_s36 {
                                                            self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Done => {
                                                                if (kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) && ((*value_domain) != (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                                                                    self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 83, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 107, 101, 121, 32, 100, 111, 109, 97, 105, 110])
                                                                } else {
                                                                    {
                                                                        let invalid_argument = self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 100, 111, 109, 97, 105, 110, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 99, 97, 108, 97, 114, 32, 84, 89, 80, 69, 32, 111, 119, 110, 101, 114, 115]);
                                                                        {
                                                                            let index = 0f64;
                                                                            {
                                                                                let source_ml_source2_s37 = self::source_ml_source1_temporary_read_rust_standard_collection_source_origin_error_loop7(origin.clone(), key_domain, value_domain, self::MlOptional::Value(source.clone()), count, &(invalid_argument), index);
                                                                                match source_ml_source2_s37 {
                                                                                    self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Done => {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                    self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop7Return(source_ml_source1_temporary_result7) => {
                                                                                        source_ml_source1_temporary_result7.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Result::MlSource1TemporaryReadRustStandardCollectionSourceOriginErrorLoop6Return(source_ml_source1_temporary_result7_2) => {
                                                                source_ml_source1_temporary_result7_2.clone()
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_method_arity(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
) -> f64 {
    {
        let kind = self::read_rust_standard_collection_kind(owner);
        if kind == (self::ml_string::Text::from_units(vec![])) {
            -1f64
        } else if self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![110, 101, 119]),
                self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]),
                self::ml_string::Text::from_units(vec![108, 101, 110]),
                self::ml_string::Text::from_units(vec![105, 115, 95, 101, 109, 112, 116, 121]),
                self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114]),
                self::ml_string::Text::from_units(vec![99, 108, 111, 110, 101]),
            ],
            method,
            0f64,
            false,
            |a, b| a == b,
        ) {
            0f64
        } else if ((*method)
            == (self::ml_string::Text::from_units(vec![
                105, 110, 116, 111, 95, 105, 116, 101, 114,
            ])))
            && self::test_rust_standard_collection_ordered(owner)
        {
            0f64
        } else if self::test_rust_standard_collection_ordered(owner)
            && (((kind == (self::ml_string::Text::from_units(vec![115, 101, 116])))
                && ((*method) == (self::ml_string::Text::from_units(vec![105, 116, 101, 114]))))
                || ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![105, 116, 101, 114]),
                            self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]),
                        ],
                        method,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
        {
            0f64
        } else if (*method)
            == (self::ml_string::Text::from_units(vec![105, 110, 115, 101, 114, 116]))
        {
            if kind == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                2f64
            } else {
                1f64
            }
        } else if (((*method) == (self::ml_string::Text::from_units(vec![103, 101, 116])))
            || ((*method)
                == (self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101]))))
            || ((*method)
                == (if kind == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
                    self::ml_string::Text::from_units(vec![
                        99, 111, 110, 116, 97, 105, 110, 115, 95, 107, 101, 121,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![99, 111, 110, 116, 97, 105, 110, 115])
                }))
        {
            1f64
        } else {
            -1f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_collection_method_result(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (self::read_rust_standard_collection_method_arity(owner, method)) < 0f64 {
        self::ml_string::Text::from_units(vec![])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![110, 101, 119]),
            self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]),
            self::ml_string::Text::from_units(vec![99, 108, 111, 110, 101]),
        ],
        method,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![
            111, 119, 110, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110,
        ])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![105, 116, 101, 114]),
            self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]),
        ],
        method,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![
            111, 119, 110, 101, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 118, 97, 108, 117,
            101, 115,
        ])
    } else if (*method)
        == (self::ml_string::Text::from_units(vec![105, 110, 116, 111, 95, 105, 116, 101, 114]))
    {
        self::ml_string::Text::from_units(vec![
            111, 119, 110, 101, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 101, 110, 116, 114,
            105, 101, 115,
        ])
    } else if (*method) == (self::ml_string::Text::from_units(vec![108, 101, 110])) {
        self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101])
    } else if (*method) == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114])) {
        self::ml_string::Text::from_units(vec![117, 110, 105, 116])
    } else if (*method) == (self::ml_string::Text::from_units(vec![103, 101, 116])) {
        self::ml_string::Text::from_units(vec![
            98, 111, 114, 114, 111, 119, 101, 100, 32, 79, 112, 116, 105, 111, 110,
        ])
    } else if ((self::read_rust_standard_collection_kind(owner))
        == (self::ml_string::Text::from_units(vec![109, 97, 112])))
        && (((*method) == (self::ml_string::Text::from_units(vec![105, 110, 115, 101, 114, 116])))
            || ((*method)
                == (self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101]))))
    {
        self::ml_string::Text::from_units(vec![
            111, 119, 110, 101, 100, 32, 79, 112, 116, 105, 111, 110,
        ])
    } else {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_rust_standard_collection_mutation(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
) -> bool {
    ((self::read_rust_standard_collection_method_arity(owner, method)) >= 0f64)
        && self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![105, 110, 115, 101, 114, 116]),
                self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101]),
                self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114]),
            ],
            method,
            0f64,
            false,
            |a, b| a == b,
        )
}
