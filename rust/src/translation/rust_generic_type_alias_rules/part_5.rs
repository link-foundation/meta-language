// Generated from complete js/src/translation/rust-generic-type-alias-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Result;
use super::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Result;
use super::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Result;
use super::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::plan_rust_primitive_from;
use super::read_rust_function_pointer_header_type_error;
use super::read_rust_function_pointer_type_error;
use super::source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop11;
use super::source_ml_source1_temporary_read_rust_function_pointer_owned_domain_error_loop12;
use super::source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop8;
use super::source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop10;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_owner(
    path: &self::ml_string::Text,
    standard_definition: bool,
    layers: f64,
) -> self::ml_string::Text {
    if !standard_definition || (layers != 0f64) {
        self::ml_string::Text::from_units(vec![])
    } else if ((*path)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105, 111,
            110,
        ])))
        || ((*path)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110,
            ])))
    {
        self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105, 111,
            110,
        ])
    } else if ((*path)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117, 108,
            116,
        ])))
        || ((*path)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116,
            ])))
    {
        self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117, 108,
            116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_path_owner(
    path: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![79, 112, 116, 105, 111, 110]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110,
            ]),
            self::ml_string::Text::from_units(vec![78, 111, 110, 101]),
            self::ml_string::Text::from_units(vec![83, 111, 109, 101]),
            self::ml_string::Text::from_units(vec![
                79, 112, 116, 105, 111, 110, 58, 58, 78, 111, 110, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                79, 112, 116, 105, 111, 110, 58, 58, 83, 111, 109, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110, 58, 58, 78, 111, 110, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110, 58, 58, 83, 111, 109, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110, 58, 58, 78, 111, 110, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110, 58, 58, 83, 111, 109, 101,
            ]),
        ],
        path,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105, 111,
            110,
        ])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![82, 101, 115, 117, 108, 116]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116,
            ]),
            self::ml_string::Text::from_units(vec![79, 107]),
            self::ml_string::Text::from_units(vec![69, 114, 114]),
            self::ml_string::Text::from_units(vec![82, 101, 115, 117, 108, 116, 58, 58, 79, 107]),
            self::ml_string::Text::from_units(vec![
                82, 101, 115, 117, 108, 116, 58, 58, 69, 114, 114,
            ]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116, 58, 58, 79, 107,
            ]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116, 58, 58, 69, 114, 114,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116, 58, 58, 79, 107,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116, 58, 58, 69, 114, 114,
            ]),
        ],
        path,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117, 108,
            116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_path_constructor(
    path: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![78, 111, 110, 101]),
            self::ml_string::Text::from_units(vec![
                79, 112, 116, 105, 111, 110, 58, 58, 78, 111, 110, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110, 58, 58, 78, 111, 110, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110, 58, 58, 78, 111, 110, 101,
            ]),
        ],
        path,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![78, 111, 110, 101])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![83, 111, 109, 101]),
            self::ml_string::Text::from_units(vec![
                79, 112, 116, 105, 111, 110, 58, 58, 83, 111, 109, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110, 58, 58, 83, 111, 109, 101,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110, 58, 58, 83, 111, 109, 101,
            ]),
        ],
        path,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![83, 111, 109, 101])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![79, 107]),
            self::ml_string::Text::from_units(vec![82, 101, 115, 117, 108, 116, 58, 58, 79, 107]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116, 58, 58, 79, 107,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116, 58, 58, 79, 107,
            ]),
        ],
        path,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![79, 107])
    } else if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![69, 114, 114]),
            self::ml_string::Text::from_units(vec![
                82, 101, 115, 117, 108, 116, 58, 58, 69, 114, 114,
            ]),
            self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116, 58, 58, 69, 114, 114,
            ]),
            self::ml_string::Text::from_units(vec![
                99, 111, 114, 101, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116, 58, 58, 69, 114, 114,
            ]),
        ],
        path,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![69, 114, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_instantiation_limit() -> f64 {
    256f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_depth_limit() -> f64 {
    16f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_specialization_name(
    ordinal: f64,
    collision: f64,
) -> self::ml_string::Text {
    if ((((!(self::ml_math::is_integer(ordinal)) || (ordinal < 0f64)) || (ordinal >= 256f64))
        || !(self::ml_math::is_integer(collision)))
        || (collision < 0f64))
        || (collision >= 4096f64)
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        (((self::ml_string::Text::from_units(vec![
            109, 108, 95, 115, 116, 97, 110, 100, 97, 114, 100, 95, 115, 117, 109, 95, 118, 97,
            108, 117, 101,
        ]))
        .concat(&{
            let source_ml_source2_string_argument39 = ordinal;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source2_string_argument39,
            ))
        }))
        .concat(&self::ml_string::Text::from_units(vec![95])))
        .concat(&{
            let source_ml_source2_string_argument40 = collision;
            self::ml_string::Text::from_text(&self::ml_number::js_number(
                source_ml_source2_string_argument40,
            ))
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_phase(
    header: bool,
    complete: bool,
    copies: bool,
) -> self::ml_string::Text {
    if header {
        self::ml_string::Text::from_units(vec![
            112, 114, 111, 118, 105, 115, 105, 111, 110, 97, 108,
        ])
    } else if complete && copies {
        self::ml_string::Text::from_units(vec![97, 100, 109, 105, 116, 116, 101, 100])
    } else {
        self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_depth(children: &[f64], sum: bool) -> f64 {
    {
        let depth = 0f64;
        {
            let source_ml_source1_temporary_values7 = children.to_vec();
            {
                let source_ml_source1_temporary_index7 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s41 =
                        self::source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop8(
                            depth,
                            &(source_ml_source1_temporary_values7),
                            source_ml_source1_temporary_index7,
                        );
                    match source_ml_source2_s41 {
                        self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Done(depth_2) => {
                            {
                                let result = depth_2 + if sum {
                                    1f64
                                } else {
                                    0f64
                                };
                                if result <= 16f64 {
                                    result
                                } else {
                                    -1f64
                                }
                            }
                        }
                        self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Return(source_ml_source1_temporary_result8) => {
                            source_ml_source1_temporary_result8
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_materialization_error(
    nodes: f64,
    units: f64,
    additional_nodes: f64,
    additional_units: f64,
) -> self::ml_string::Text {
    {
        let source_ml_source1_temporary_values9 =
            vec![nodes, units, additional_nodes, additional_units];
        {
            let source_ml_source1_temporary_index9 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s42 = self::source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop10(&(source_ml_source1_temporary_values9), source_ml_source1_temporary_index9);
                match source_ml_source2_s42 {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Done => {
                        if (((nodes > 1_048_576f64) || (units > 8_388_608f64)) || (additional_nodes > (1_048_576f64 - nodes))) || (additional_units > (8_388_608f64 - units)) {
                            self::ml_string::Text::from_units(vec![116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 115, 117, 109, 32, 115, 111, 117, 114, 99, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 109, 97, 116, 101, 114, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116])
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Return(source_ml_source1_temporary_result10) => {
                        source_ml_source1_temporary_result10
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_argument_count(owner: &self::ml_string::Text) -> f64 {
    if (*owner)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105, 111,
            110,
        ]))
    {
        1f64
    } else if (*owner)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117, 108,
            116,
        ]))
    {
        2f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_source_arguments(
    owner: &self::ml_string::Text,
    identities: &[self::ml_string::Text],
    direct_copies: &[bool],
) -> bool {
    {
        let count = self::read_rust_standard_sum_source_argument_count(owner);
        if ((count < 0f64) || ((identities.len() as f64) != count))
            || ((direct_copies.len() as f64) != count)
        {
            false
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s43 = self::source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop11(identities, direct_copies, count, at);
                    match source_ml_source2_s43 {
                        self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Return(source_ml_source1_temporary_result11) => {
                            source_ml_source1_temporary_result11
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_constructors(
    owner: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    if (*owner)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105, 111,
            110,
        ]))
    {
        vec![
            self::ml_string::Text::from_units(vec![78, 111, 110, 101]),
            self::ml_string::Text::from_units(vec![83, 111, 109, 101]),
        ]
    } else if (*owner)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117, 108,
            116,
        ]))
    {
        vec![
            self::ml_string::Text::from_units(vec![79, 107]),
            self::ml_string::Text::from_units(vec![69, 114, 114]),
        ]
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_field_argument(
    owner: &self::ml_string::Text,
    constructor: &self::ml_string::Text,
) -> f64 {
    if (*owner)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105, 111,
            110,
        ]))
    {
        if (*constructor) == (self::ml_string::Text::from_units(vec![78, 111, 110, 101])) {
            -1f64
        } else if (*constructor) == (self::ml_string::Text::from_units(vec![83, 111, 109, 101])) {
            0f64
        } else {
            -2f64
        }
    } else if (*owner)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117, 108,
            116,
        ]))
    {
        if (*constructor) == (self::ml_string::Text::from_units(vec![79, 107])) {
            0f64
        } else if (*constructor) == (self::ml_string::Text::from_units(vec![69, 114, 114])) {
            1f64
        } else {
            -2f64
        }
    } else {
        -2f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_source_constructor_arity(
    owner: &self::ml_string::Text,
    constructor: &self::ml_string::Text,
) -> f64 {
    {
        let argument = self::read_rust_standard_sum_source_field_argument(owner, constructor);
        if argument == -2f64 {
            -1f64
        } else if argument == -1f64 {
            0f64
        } else {
            1f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_source_inferred_constructor(
    owner: &self::ml_string::Text,
    constructor: &self::ml_string::Text,
) -> bool {
    ((*owner)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105, 111,
            110,
        ])))
        && ((*constructor) == (self::ml_string::Text::from_units(vec![83, 111, 109, 101])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_question_context(
    operand_owner: &self::ml_string::Text,
    return_owner: &self::ml_string::Text,
    ordinary_callable: bool,
    direct_copy_operand: bool,
    operand_error: &self::ml_string::Text,
    return_error: &self::ml_string::Text,
) -> bool {
    (ordinary_callable && direct_copy_operand)
        && self::accept_rust_standard_sum_question_family(
            operand_owner,
            return_owner,
            operand_error,
            return_error,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_question_family(
    operand_owner: &self::ml_string::Text,
    return_owner: &self::ml_string::Text,
    operand_error: &self::ml_string::Text,
    return_error: &self::ml_string::Text,
) -> bool {
    {
        let ml_library_inequality_1 = (*operand_owner) != (*return_owner);
        if ml_library_inequality_1 {
            false
        } else if (*operand_owner)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105,
                111, 110,
            ]))
        {
            ((*operand_error) == (self::ml_string::Text::from_units(vec![])))
                && ((*return_error) == (self::ml_string::Text::from_units(vec![])))
        } else {
            (((*operand_owner)
                == (self::ml_string::Text::from_units(vec![
                    115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                    108, 116,
                ])))
                && ((*operand_error) != (self::ml_string::Text::from_units(vec![]))))
                && ((*operand_error) == (*return_error))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_owned_question_context(
    operand_owner: &self::ml_string::Text,
    return_owner: &self::ml_string::Text,
    ordinary: bool,
    role: &self::ml_string::Text,
    wrappers: f64,
    operand_error: &self::ml_string::Text,
    return_error: &self::ml_string::Text,
) -> bool {
    ((ordinary && ((*role) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]))))
        && (wrappers == 0f64))
        && self::accept_rust_standard_sum_question_family(
            operand_owner,
            return_owner,
            operand_error,
            return_error,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_rust_standard_sum_primitive_question_conversion_plan(
    operand_owner: &self::ml_string::Text,
    return_owner: &self::ml_string::Text,
    ordinary: bool,
    role: &self::ml_string::Text,
    wrappers: f64,
    direct_copy: bool,
    source_known: bool,
    target_known: bool,
    source: &self::ml_string::Text,
    target: &self::ml_string::Text,
    minor: f64,
) -> Vec<self::ml_string::Text> {
    if (((((((!ordinary
        || ((*operand_owner)
            != (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116,
            ]))))
        || ((*return_owner)
            != (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 114, 101, 115, 117, 108, 116, 58, 58, 82, 101, 115, 117,
                108, 116,
            ]))))
        || (wrappers != 0f64))
        || !source_known)
        || !target_known)
        || !(self::ml_math::is_integer(minor)))
        || (minor < 90f64))
        || (minor >= 100f64)
    {
        Vec::<self::ml_string::Text>::new()
    } else if ((*source) == (*target))
        || (((*role) != (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100])))
            && (((*role) != (self::ml_string::Text::from_units(vec![99, 111, 112, 121])))
                || !direct_copy))
    {
        Vec::<self::ml_string::Text>::new()
    } else {
        {
            let plan = self::plan_rust_primitive_from(target, source);
            if plan == (self::ml_string::Text::from_units(vec![])) {
                Vec::<self::ml_string::Text>::new()
            } else {
                vec![role.clone(), target.clone(), plan]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_standard_sum_primitive_question_completion(
    profile: &self::ml_string::Text,
    consumed: bool,
) -> bool {
    ((*profile) == (self::ml_string::Text::from_units(vec![99, 111, 112, 121])))
        || (((*profile) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100])))
            && consumed)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_question_branch(
    owner: &self::ml_string::Text,
    constructor: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (self::read_rust_standard_sum_source_constructor_arity(owner, constructor)) < 0f64 {
        self::ml_string::Text::from_units(vec![])
    } else if (*owner)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 100, 58, 58, 111, 112, 116, 105, 111, 110, 58, 58, 79, 112, 116, 105, 111,
            110,
        ]))
    {
        if (*constructor) == (self::ml_string::Text::from_units(vec![83, 111, 109, 101])) {
            self::ml_string::Text::from_units(vec![
                99, 111, 110, 116, 105, 110, 117, 101, 32, 102, 105, 101, 108, 100,
            ])
        } else {
            self::ml_string::Text::from_units(vec![
                114, 101, 116, 117, 114, 110, 32, 97, 98, 115, 101, 110, 116,
            ])
        }
    } else if (*constructor) == (self::ml_string::Text::from_units(vec![79, 107])) {
        self::ml_string::Text::from_units(vec![
            99, 111, 110, 116, 105, 110, 117, 101, 32, 102, 105, 101, 108, 100,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            114, 101, 116, 117, 114, 110, 32, 101, 114, 114, 111, 114, 32, 102, 105, 101, 108, 100,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_type_closing_token_parts(
    token: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    if (*token) == (self::ml_string::Text::from_units(vec![62, 62])) {
        vec![
            self::ml_string::Text::from_units(vec![62]),
            self::ml_string::Text::from_units(vec![62]),
        ]
    } else if (*token) == (self::ml_string::Text::from_units(vec![62, 61])) {
        vec![
            self::ml_string::Text::from_units(vec![62]),
            self::ml_string::Text::from_units(vec![61]),
        ]
    } else if (*token) == (self::ml_string::Text::from_units(vec![62, 62, 61])) {
        vec![
            self::ml_string::Text::from_units(vec![62]),
            self::ml_string::Text::from_units(vec![62]),
            self::ml_string::Text::from_units(vec![61]),
        ]
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_reference_token_parts(
    token: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    if (*token) == (self::ml_string::Text::from_units(vec![38, 38])) {
        vec![
            self::ml_string::Text::from_units(vec![38]),
            self::ml_string::Text::from_units(vec![38]),
        ]
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_pointer_parameter_limit() -> f64 {
    1024f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_function_pointer_safe_source(
    value: &self::ml_string::Text,
    raw_: &self::ml_string::Text,
) -> bool {
    ((*value) == (self::ml_string::Text::from_units(vec![102, 110])))
        && ((*raw_) == (self::ml_string::Text::from_units(vec![102, 110])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_pointer_owned_domain_error(
    ordinary: bool,
    parameters: &[self::ml_string::Text],
    result: &self::ml_string::Text,
    minor: f64,
) -> self::ml_string::Text {
    if ordinary {
        if (!(self::ml_math::is_integer(minor)) || (minor < 90f64)) || (minor >= 100f64) {
            self::ml_string::Text::from_units(vec![
                111, 119, 110, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 111,
                105, 110, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 115, 101, 109, 97,
                110, 116, 105, 99, 115, 32, 97, 114, 101, 32, 117, 110, 99, 111, 118, 101, 114,
                101, 100, 32, 111, 117, 116, 115, 105, 100, 101, 32, 82, 117, 115, 116, 32, 62, 61,
                49, 46, 57, 48, 46, 48, 32, 60, 49, 46, 49, 48, 48, 46, 48,
            ])
        } else if (parameters.len() as f64) > (self::read_rust_function_pointer_parameter_limit()) {
            self::ml_string::Text::from_units(vec![
                116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 111, 105, 110,
                116, 101, 114, 32, 115, 105, 103, 110, 97, 116, 117, 114, 101, 32, 101, 120, 99,
                101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 98,
                117, 100, 103, 101, 116,
            ])
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s44 = self::source_ml_source1_temporary_read_rust_function_pointer_owned_domain_error_loop12(parameters, at);
                    match source_ml_source2_s44 {
                            self::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Result::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Done => {
                                if ((*result) != (self::ml_string::Text::from_units(vec![99, 111, 112, 121]))) && ((*result) != (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]))) {
                                    self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 111, 105, 110, 116, 101, 114, 32, 114, 101, 115, 117, 108, 116, 115, 32, 110, 101, 101, 100, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 100, 105, 114, 101, 99, 116, 32, 67, 111, 112, 121, 32, 111, 114, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 111, 119, 110, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101])
                                } else if self::ml_array::includes(parameters, &self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]), 0f64, false, |a, b| a == b) || ((*result) == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]))) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        self::ml_string::Text::from_units(vec![116, 104, 101, 32, 111, 119, 110, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 111, 105, 110, 116, 101, 114, 32, 112, 114, 111, 102, 105, 108, 101, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 111, 114, 32, 114, 101, 115, 117, 108, 116])
                                    }
                            }
                            self::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Result::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Return(source_ml_source1_temporary_result12) => {
                                source_ml_source1_temporary_result12.clone()
                            }
                        }
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![
            111, 119, 110, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 111, 105,
            110, 116, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32,
            111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114,
            111, 102, 105, 108, 101,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_pointer_domain_error(
    ordinary: bool,
    copies: &[bool],
    result_copy: bool,
    roles: &[self::ml_string::Text],
    result_role: &self::ml_string::Text,
    minor: f64,
) -> self::ml_string::Text {
    {
        let copy_error = self::read_rust_function_pointer_type_error(ordinary, copies, result_copy);
        if copy_error == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            let ml_library_inequality_1 = (roles.len() as f64) != (copies.len() as f64);
            if ml_library_inequality_1 {
                copy_error.clone()
            } else {
                {
                    let owned_error = self::read_rust_function_pointer_owned_domain_error(
                        ordinary,
                        roles,
                        result_role,
                        minor,
                    );
                    if owned_error == (self::ml_string::Text::from_units(vec![])) {
                        self::ml_string::Text::from_units(vec![])
                    } else if self::ml_array::includes(
                        roles,
                        &self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]),
                        0f64,
                        false,
                        |a, b| a == b,
                    ) || ((*result_role)
                        == (self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100])))
                    {
                        owned_error.clone()
                    } else {
                        copy_error.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_pointer_header_domain_error(
    ordinary: bool,
    copies: &[bool],
    result_copy: bool,
    roles: &[self::ml_string::Text],
    result_role: &self::ml_string::Text,
    minor: f64,
    header: bool,
) -> self::ml_string::Text {
    {
        let actual = self::read_rust_function_pointer_domain_error(
            ordinary,
            copies,
            result_copy,
            roles,
            result_role,
            minor,
        );
        if actual == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else if header {
            self::read_rust_function_pointer_header_type_error(
                ordinary,
                copies,
                result_copy,
                header,
            )
        } else {
            actual
        }
    }
}
