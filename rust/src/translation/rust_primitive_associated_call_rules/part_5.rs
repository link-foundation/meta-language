// Generated from complete js/src/translation/rust-primitive-associated-call-rules.js. Do not edit by hand.
use super::accept_rust_integer_bit_method;
use super::accept_rust_primitive_from_method;
use super::accept_rust_primitive_selection_receiver;
use super::accept_rust_scalar_fallback;
use super::accept_rust_wrapping_shift_method;
use super::ml_array;
use super::ml_string;
use super::read_rust_checked_integer_arity;
use super::read_rust_integer_magnitude_arity;
use super::read_rust_primitive_owner_path;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_mapped_fallback_error(
    kind: &self::ml_string::Text,
    parameters: f64,
    layers: f64,
    owner: &self::ml_string::Text,
    expected: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::accept_rust_scalar_fallback(
        kind,
        parameters,
        layers,
        (self::accept_rust_primitive_from_method(
            owner,
            &(self::ml_string::Text::from_units(vec![102, 114, 111, 109])),
        ) && ((*owner) == (*expected))),
    ) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            97, 32, 99, 104, 101, 99, 107, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32,
            109, 97, 112, 112, 101, 100, 32, 102, 97, 108, 108, 98, 97, 99, 107, 32, 110, 101, 101,
            100, 115, 32, 97, 32, 100, 105, 114, 101, 99, 116, 32, 122, 101, 114, 111, 45, 97, 114,
            103, 117, 109, 101, 110, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 119, 105,
            116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 99, 97, 108, 97,
            114, 32, 114, 101, 115, 117, 108, 116, 32, 111, 119, 110, 101, 114,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_associated_owner(
    segments: &[self::ml_string::Text],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> self::ml_string::Text {
    self::read_rust_primitive_owner_path(segments, ordinary, shadowed, absolute)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_associated_arity(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
) -> f64 {
    if (self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
            self::ml_string::Text::from_units(vec![105, 56]),
            self::ml_string::Text::from_units(vec![117, 56]),
            self::ml_string::Text::from_units(vec![105, 49, 54]),
            self::ml_string::Text::from_units(vec![117, 49, 54]),
            self::ml_string::Text::from_units(vec![105, 51, 50]),
            self::ml_string::Text::from_units(vec![117, 51, 50]),
            self::ml_string::Text::from_units(vec![105, 54, 52]),
            self::ml_string::Text::from_units(vec![117, 54, 52]),
            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
            self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
            self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
        ],
        owner,
        0f64,
        false,
        |a, b| a == b,
    )) {
        if ((*method) == (self::ml_string::Text::from_units(vec![109, 105, 110])))
            || ((*method) == (self::ml_string::Text::from_units(vec![109, 97, 120])))
        {
            2f64
        } else if (*owner) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
            -1f64
        } else {
            {
                let magnitude = self::read_rust_integer_magnitude_arity(method, true);
                if (magnitude >= 0f64)
                    && (((*method)
                        != (self::ml_string::Text::from_units(vec![
                            117, 110, 115, 105, 103, 110, 101, 100, 95, 97, 98, 115,
                        ])))
                        || owner.starts_with(&self::ml_string::Text::from_units(vec![105])))
                {
                    magnitude + 1f64
                } else if self::accept_rust_wrapping_shift_method(method) {
                    2f64
                } else if self::accept_rust_integer_bit_method(method) {
                    1f64
                } else {
                    {
                        let checked = self::read_rust_checked_integer_arity(method, true);
                        if (checked >= 0f64)
                            && (((*method)
                                != (self::ml_string::Text::from_units(vec![
                                    99, 104, 101, 99, 107, 101, 100, 95, 97, 98, 115,
                                ])))
                                || owner.starts_with(&self::ml_string::Text::from_units(vec![105])))
                        {
                            checked + 1f64
                        } else if self::ml_array::includes(
                            &vec![
                                self::ml_string::Text::from_units(vec![
                                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 100,
                                    100,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117,
                                    98,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 109, 117,
                                    108,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    119, 114, 97, 112, 112, 105, 110, 103, 95, 97, 100, 100,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    119, 114, 97, 112, 112, 105, 110, 103, 95, 115, 117, 98,
                                ]),
                            ],
                            method,
                            0f64,
                            false,
                            |a, b| a == b,
                        ) {
                            2f64
                        } else if (*method)
                            == (self::ml_string::Text::from_units(vec![
                                119, 114, 97, 112, 112, 105, 110, 103, 95, 110, 101, 103,
                            ]))
                        {
                            1f64
                        } else if (owner.starts_with(&self::ml_string::Text::from_units(vec![105])))
                        {
                            if self::ml_array::includes(
                                &vec![
                                    self::ml_string::Text::from_units(vec![
                                        105, 115, 95, 110, 101, 103, 97, 116, 105, 118, 101,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        105, 115, 95, 112, 111, 115, 105, 116, 105, 118, 101,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        115, 105, 103, 110, 117, 109,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 110,
                                        101, 103,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 98,
                                        115,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        119, 114, 97, 112, 112, 105, 110, 103, 95, 97, 98, 115,
                                    ]),
                                ],
                                method,
                                0f64,
                                false,
                                |a, b| a == b,
                            ) {
                                1f64
                            } else {
                                -1f64
                            }
                        } else {
                            -1f64
                        }
                    }
                }
            }
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_associated_literal(
    expected: &self::ml_string::Text,
    form: &self::ml_string::Text,
    typed: bool,
) -> bool {
    ((((*expected) != (self::ml_string::Text::from_units(vec![])))
        && ((*expected) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
        && !typed)
        && (((*form)
            == (self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 97, 108])))
            || ((*form)
                == (self::ml_string::Text::from_units(vec![
                    110, 101, 103, 97, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 97, 108,
                ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_associated_receiver(
    expected: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((*expected) == (*path))
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_integer_bits_loop3(
    mut plan: Vec<f64>,
    bindings: &[f64],
    mut index: f64,
    mut body: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at >= 0f64 {
            {
                let plan_2 = {
                    let source_ml_source2_concat_value72 = plan;
                    {
                        let source_ml_source2_concat_value73 = vec![
                            18f64,
                            (self::ml_array::optional_at(
                                bindings,
                                self::ml_array::number_index(at),
                            ))
                            .unwrap_or_else(|| -1f64),
                            (self::ml_array::optional_at(
                                bindings,
                                self::ml_array::number_index(at + 1f64),
                            ))
                            .unwrap_or_else(|| -1f64),
                            body,
                            0f64,
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value72,
                            ),
                            source_ml_source2_concat_value73,
                        )
                    }
                };
                {
                    let body_2 = index;
                    {
                        let index_2 = index + 1f64;
                        {
                            let at_2 = at - 2f64;
                            {
                                (plan, index, body, at) = (plan_2, index_2, body_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { plan };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustIntegerBitsJoin2 {
    MlSource1TemporaryPlanRustIntegerBitsJoin2Next(Vec<f64>, f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustIntegerBitsLoop1Result {
    MlSource1TemporaryPlanRustIntegerBitsLoop1Done(Vec<f64>, Vec<f64>, f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustIntegerBitsJoin4 {
    MlSource1TemporaryPlanRustIntegerBitsJoin4Next(Vec<f64>, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_rust_integer_bits_loop1(
    mut bits: f64,
    mut reverse: bool,
    mut trailing: bool,
    mut leading: bool,
    mut ones: bool,
    mut plan: Vec<f64>,
    mut bindings: Vec<f64>,
    mut current: f64,
    mut count: f64,
    mut active: f64,
    mut index: f64,
    mut bit: f64,
) -> self::MlSource1TemporaryPlanRustIntegerBitsLoop1Result {
    loop {
        return if bit < bits {
            {
                let at = index;
                {
                    let next_current = 3f64 + (bit * 3f64);
                    {
                        let next_count = 4f64 + (bit * 3f64);
                        {
                            let next_active = 5f64 + (bit * 3f64);
                            {
                                let plan_2 = {
                                    let source_ml_source2_concat_value74 = plan;
                                    {
                                        let source_ml_source2_concat_value75 = vec![
                                            8f64,
                                            current,
                                            0f64,
                                            0f64,
                                            0f64,
                                            if reverse { 8f64 } else { 9f64 },
                                            count,
                                            0f64,
                                            0f64,
                                            0f64,
                                            10f64,
                                            active,
                                            0f64,
                                            0f64,
                                            0f64,
                                            12f64,
                                            at,
                                            3f64,
                                            0f64,
                                            0f64,
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value74,
                                            ),
                                            source_ml_source2_concat_value75,
                                        )
                                    }
                                };
                                {
                                    let index_2 = index + 4f64;
                                    {
                                        let next_count_term = 0f64;
                                        {
                                            let next_active_term = 7f64;
                                            {
                                                let source_ml_source2_s84 = if reverse {
                                                    {
                                                        let plan_3 = {
                                                            let source_ml_source2_concat_value85 =
                                                                plan_2;
                                                            {
                                                                let source_ml_source2_concat_value86 = vec![
                                                                    16f64,
                                                                    (at + 1f64),
                                                                    3f64,
                                                                    0f64,
                                                                    0f64,
                                                                    15f64,
                                                                    (at + 4f64),
                                                                    (at + 3f64),
                                                                    0f64,
                                                                    0f64,
                                                                ];
                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value85), source_ml_source2_concat_value86)
                                                            }
                                                        };
                                                        {
                                                            let next_count_term_2 = at + 5f64;
                                                            {
                                                                let index_3 = index_2 + 2f64;
                                                                self::MlSource1TemporaryPlanRustIntegerBitsJoin4::MlSource1TemporaryPlanRustIntegerBitsJoin4Next(plan_3, index_3, next_count_term_2, next_active_term)
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let match_ = index_2;
                                                        {
                                                            let plan_4 = {
                                                                let source_ml_source2_concat_value87 =
                                                                    plan_2;
                                                                {
                                                                    let source_ml_source2_concat_value88 = vec![
                                                                        11f64,
                                                                        (at + 3f64),
                                                                        if ones {
                                                                            2f64
                                                                        } else {
                                                                            1f64
                                                                        },
                                                                        0f64,
                                                                        0f64,
                                                                        14f64,
                                                                        (at + 1f64),
                                                                        5f64,
                                                                        0f64,
                                                                        0f64,
                                                                    ];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value87), source_ml_source2_concat_value88)
                                                                }
                                                            };
                                                            {
                                                                let index_4 = index_2 + 2f64;
                                                                if trailing {
                                                                    {
                                                                        let next_flag = index_4;
                                                                        {
                                                                            let plan_5 = {
                                                                                let source_ml_source2_concat_value89 =
                                                                                    plan_4;
                                                                                {
                                                                                    let source_ml_source2_concat_value90 = vec![
                                                                                        17f64,
                                                                                        (at + 2f64),
                                                                                        match_,
                                                                                        6f64,
                                                                                        0f64,
                                                                                        17f64,
                                                                                        next_flag,
                                                                                        (match_
                                                                                            + 1f64),
                                                                                        (at + 1f64),
                                                                                        0f64,
                                                                                    ];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value89), source_ml_source2_concat_value90)
                                                                                }
                                                                            };
                                                                            {
                                                                                let next_active_term_2 =
                                                                                    next_flag;
                                                                                {
                                                                                    let next_count_term_3 =
                                                                                        next_flag
                                                                                            + 1f64;
                                                                                    {
                                                                                        let index_5 = index_4 + 2f64;
                                                                                        self::MlSource1TemporaryPlanRustIntegerBitsJoin4::MlSource1TemporaryPlanRustIntegerBitsJoin4Next(plan_5, index_5, next_count_term_3, next_active_term_2)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let next_count_term_4 =
                                                                            index_4;
                                                                        {
                                                                            let plan_6 = {
                                                                                let source_ml_source2_concat_value91 =
                                                                                    plan_4;
                                                                                {
                                                                                    let source_ml_source2_concat_value92 = vec![
                                                                                        17f64,
                                                                                        match_,
                                                                                        (match_
                                                                                            + 1f64),
                                                                                        if leading {
                                                                                            4f64
                                                                                        } else {
                                                                                            at + 1f64
                                                                                        },
                                                                                        0f64,
                                                                                    ];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value91), source_ml_source2_concat_value92)
                                                                                }
                                                                            };
                                                                            {
                                                                                let index_6 =
                                                                                    index_4 + 1f64;
                                                                                self::MlSource1TemporaryPlanRustIntegerBitsJoin4::MlSource1TemporaryPlanRustIntegerBitsJoin4Next(plan_6, index_6, next_count_term_4, next_active_term)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                match source_ml_source2_s84 {
                                                    self::MlSource1TemporaryPlanRustIntegerBitsJoin4::MlSource1TemporaryPlanRustIntegerBitsJoin4Next(plan_7, index_7, next_count_term_5, next_active_term_3) => {
                                                        {
                                                            let quotient = index_7;
                                                            {
                                                                let plan_8 = {
                                                                    let source_ml_source2_concat_value93 = plan_7;
                                                                    {
                                                                        let source_ml_source2_concat_value94 = vec![13f64, at, 3f64, 0f64, 0f64];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value93), source_ml_source2_concat_value94)
                                                                    }
                                                                };
                                                                {
                                                                    let index_8 = index_7 + 1f64;
                                                                    {
                                                                        let bindings_2 = {
                                                                            let source_ml_source2_concat_value95 = bindings;
                                                                            {
                                                                                let source_ml_source2_concat_value96 = vec![next_current, quotient, next_count, next_count_term_5];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value95), source_ml_source2_concat_value96)
                                                                            }
                                                                        };
                                                                        {
                                                                            let bindings_4 = if trailing {
                                                                                {
                                                                                    {
                                                                                        let source_ml_source2_concat_value97 = bindings_2;
                                                                                        {
                                                                                            let source_ml_source2_concat_value98 = vec![next_active, next_active_term_3];
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value97), source_ml_source2_concat_value98)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                bindings_2
                                                                            };
                                                                            {
                                                                                let current_2 = next_current;
                                                                                {
                                                                                    let count_2 = next_count;
                                                                                    if trailing {
                                                                                        {
                                                                                            let active_2 = next_active;
                                                                                            {
                                                                                                let bit_2 = bit + 1f64;
                                                                                                {
                                                                                                    (plan, bindings, current, count, active, index, bit) = (plan_8, bindings_4, current_2, count_2, active_2, index_8, bit_2);
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
{
                                                                                        {
                                                                                            let bit_3 = bit + 1f64;
                                                                                            {
                                                                                                (plan, bindings, current, count, index, bit) = (plan_8, bindings_4, current_2, count_2, index_8, bit_3);
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
            self::MlSource1TemporaryPlanRustIntegerBitsLoop1Result::MlSource1TemporaryPlanRustIntegerBitsLoop1Done(plan, bindings, count, index)
        };
    }
}
