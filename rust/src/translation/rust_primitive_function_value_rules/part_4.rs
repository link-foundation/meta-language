// Generated from complete js/src/translation/rust-primitive-function-value-rules.js. Do not edit by hand.
use super::accept_rust_integer_bit_method;
use super::accept_rust_primitive_from_method;
use super::accept_rust_primitive_selection_receiver;
use super::accept_rust_wrapping_shift_method;
use super::ml_array;
use super::ml_string;
use super::read_rust_integer_bit_result_owner;
use super::read_rust_integer_magnitude_result_owner;
use super::read_rust_primitive_associated_arity;
use super::read_rust_primitive_from_owner;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_from_source(
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (((((((((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
        && ((*bits) == (self::ml_string::Text::from_units(vec![]))))
        && !signed)
        && ((*owner)
            == (self::ml_string::Text::from_units(vec![
                112, 114, 105, 109, 105, 116, 105, 118, 101,
            ]))))
        && ((*path) == (self::ml_string::Text::from_units(vec![102, 54, 52]))))
        && (children == 0f64))
        && proven)
        && (layers == 0f64))
        || self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_from(
    target: &self::ml_string::Text,
    source: &self::ml_string::Text,
) -> self::ml_string::Text {
    if !(self::accept_rust_primitive_from_method(
        target,
        &(self::ml_string::Text::from_units(vec![102, 114, 111, 109])),
    )) || !(self::accept_rust_primitive_from_method(
        source,
        &(self::ml_string::Text::from_units(vec![102, 114, 111, 109])),
    )) {
        self::ml_string::Text::from_units(vec![])
    } else if (*target) == (*source) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else if (((*source) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))
        && ((*target) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
        && ((*target) != (self::ml_string::Text::from_units(vec![102, 54, 52])))
    {
        self::ml_string::Text::from_units(vec![
            98, 111, 111, 108, 101, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114,
        ])
    } else if ((*target) == (self::ml_string::Text::from_units(vec![102, 54, 52])))
        && self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![105, 56]),
                self::ml_string::Text::from_units(vec![105, 49, 54]),
                self::ml_string::Text::from_units(vec![105, 51, 50]),
                self::ml_string::Text::from_units(vec![117, 56]),
                self::ml_string::Text::from_units(vec![117, 49, 54]),
                self::ml_string::Text::from_units(vec![117, 51, 50]),
            ],
            source,
            0f64,
            false,
            |a, b| a == b,
        )
    {
        self::ml_string::Text::from_units(vec![
            102, 108, 111, 97, 116, 105, 110, 103, 32, 101, 120, 97, 99, 116,
        ])
    } else {
        {
            let admitted = (((((((((*source)
                == (self::ml_string::Text::from_units(vec![117, 56])))
                && self::ml_array::includes(
                    &vec![
                        self::ml_string::Text::from_units(vec![117, 49, 54]),
                        self::ml_string::Text::from_units(vec![117, 51, 50]),
                        self::ml_string::Text::from_units(vec![117, 54, 52]),
                        self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                        self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
                        self::ml_string::Text::from_units(vec![105, 49, 54]),
                        self::ml_string::Text::from_units(vec![105, 51, 50]),
                        self::ml_string::Text::from_units(vec![105, 54, 52]),
                        self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
                    ],
                    target,
                    0f64,
                    false,
                    |a, b| a == b,
                ))
                || (((*source) == (self::ml_string::Text::from_units(vec![117, 49, 54])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![117, 51, 50]),
                            self::ml_string::Text::from_units(vec![117, 54, 52]),
                            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]),
                            self::ml_string::Text::from_units(vec![105, 51, 50]),
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![117, 51, 50])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![117, 54, 52]),
                            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![117, 54, 52])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![117, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![105, 56])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![105, 49, 54]),
                            self::ml_string::Text::from_units(vec![105, 51, 50]),
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![105, 49, 54])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![105, 51, 50]),
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                            self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![105, 51, 50])))
                    && self::ml_array::includes(
                        &vec![
                            self::ml_string::Text::from_units(vec![105, 54, 52]),
                            self::ml_string::Text::from_units(vec![105, 49, 50, 56]),
                        ],
                        target,
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || (((*source) == (self::ml_string::Text::from_units(vec![105, 54, 52])))
                    && ((*target) == (self::ml_string::Text::from_units(vec![105, 49, 50, 56]))));
            if admitted {
                self::ml_string::Text::from_units(vec![
                    105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 97, 99, 116,
                ])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_default_owner(
    segments: &[self::ml_string::Text],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> self::ml_string::Text {
    if {
        let source_ml_source1_comparison_operand57 = self::ml_array::optional_at(
            segments,
            self::ml_array::number_index((segments.len() as f64) - 1f64),
        );
        {
            let source_ml_source1_comparison_operand58 =
                self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]);
            match (
                source_ml_source1_comparison_operand57,
                source_ml_source1_comparison_operand58,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    } {
        self::read_rust_primitive_from_owner(segments, ordinary, shadowed, absolute)
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_default_term(owner: &self::ml_string::Text) -> self::ml_string::Text {
    if (*owner) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
        self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
    } else if (*owner) == (self::ml_string::Text::from_units(vec![102, 54, 52])) {
        self::ml_string::Text::from_units(vec![
            112, 111, 115, 105, 116, 105, 118, 101, 32, 102, 108, 111, 97, 116, 105, 110, 103, 32,
            122, 101, 114, 111,
        ])
    } else if self::ml_array::includes(
        &vec![
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
    ) {
        self::ml_string::Text::from_units(vec![
            105, 110, 116, 101, 103, 101, 114, 32, 122, 101, 114, 111,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_default_import(
    path: &self::ml_string::Text,
    ordinary: bool,
    shadowed: bool,
) -> bool {
    (ordinary && !shadowed)
        && (((*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 100, 101, 102, 97, 117, 108, 116, 58, 58, 68, 101, 102, 97,
                117, 108, 116,
            ])))
            || ((*path)
                == (self::ml_string::Text::from_units(vec![
                    99, 111, 114, 101, 58, 58, 100, 101, 102, 97, 117, 108, 116, 58, 58, 68, 101,
                    102, 97, 117, 108, 116,
                ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_function_value_arity(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
) -> f64 {
    if (*method) == (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])) {
        if (self::read_rust_primitive_default_term(owner))
            == (self::ml_string::Text::from_units(vec![]))
        {
            -1f64
        } else {
            0f64
        }
    } else if method.starts_with(&self::ml_string::Text::from_units(vec![
        99, 104, 101, 99, 107, 101, 100, 95,
    ])) {
        -1f64
    } else {
        self::read_rust_primitive_associated_arity(owner, method)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_function_value_parameter_owner(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
    index: f64,
) -> self::ml_string::Text {
    {
        let arity = self::read_rust_primitive_function_value_arity(owner, method);
        if ((index < 0f64) || (index >= arity)) || ((index % 1f64) != 0f64) {
            self::ml_string::Text::from_units(vec![])
        } else if (index == 1f64) && self::accept_rust_wrapping_shift_method(method) {
            self::ml_string::Text::from_units(vec![117, 51, 50])
        } else {
            owner.clone()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_function_value_result_owner(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (self::read_rust_primitive_function_value_arity(owner, method)) < 0f64 {
        self::ml_string::Text::from_units(vec![])
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![
            105, 115, 95, 110, 101, 103, 97, 116, 105, 118, 101,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                105, 115, 95, 112, 111, 115, 105, 116, 105, 118, 101,
            ])))
    {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else if self::accept_rust_integer_bit_method(method) {
        self::read_rust_integer_bit_result_owner(method, owner)
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![
            117, 110, 115, 105, 103, 110, 101, 100, 95, 97, 98, 115,
        ])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![97, 98, 115, 95, 100, 105, 102, 102])))
    {
        self::read_rust_integer_magnitude_result_owner(owner)
    } else {
        owner.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_integer_bits_loop3(
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
                    let source_ml_source1_concat_value59 = plan;
                    {
                        let source_ml_source1_concat_value60 = vec![
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
                                source_ml_source1_concat_value59,
                            ),
                            source_ml_source1_concat_value60,
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
pub enum MlTemporaryPlanRustIntegerBitsJoin2 {
    MlTemporaryPlanRustIntegerBitsJoin2Next(Vec<f64>, f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustIntegerBitsLoop1Result {
    MlTemporaryPlanRustIntegerBitsLoop1Done(Vec<f64>, Vec<f64>, f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustIntegerBitsJoin4 {
    MlTemporaryPlanRustIntegerBitsJoin4Next(Vec<f64>, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_integer_bits_loop1(
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
) -> self::MlTemporaryPlanRustIntegerBitsLoop1Result {
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
                                    let source_ml_source1_concat_value61 = plan;
                                    {
                                        let source_ml_source1_concat_value62 = vec![
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
                                                source_ml_source1_concat_value61,
                                            ),
                                            source_ml_source1_concat_value62,
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
                                                let source_ml_source1_s71 = if reverse {
                                                    {
                                                        let plan_3 = {
                                                            let source_ml_source1_concat_value72 =
                                                                plan_2;
                                                            {
                                                                let source_ml_source1_concat_value73 = vec![
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
                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value72), source_ml_source1_concat_value73)
                                                            }
                                                        };
                                                        {
                                                            let next_count_term_2 = at + 5f64;
                                                            {
                                                                let index_3 = index_2 + 2f64;
                                                                self::MlTemporaryPlanRustIntegerBitsJoin4::MlTemporaryPlanRustIntegerBitsJoin4Next(plan_3, index_3, next_count_term_2, next_active_term)
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let match_ = index_2;
                                                        {
                                                            let plan_4 = {
                                                                let source_ml_source1_concat_value74 =
                                                                    plan_2;
                                                                {
                                                                    let source_ml_source1_concat_value75 = vec![
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
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value74), source_ml_source1_concat_value75)
                                                                }
                                                            };
                                                            {
                                                                let index_4 = index_2 + 2f64;
                                                                if trailing {
                                                                    {
                                                                        let next_flag = index_4;
                                                                        {
                                                                            let plan_5 = {
                                                                                let source_ml_source1_concat_value76 =
                                                                                    plan_4;
                                                                                {
                                                                                    let source_ml_source1_concat_value77 = vec![
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
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value76), source_ml_source1_concat_value77)
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
                                                                                        self::MlTemporaryPlanRustIntegerBitsJoin4::MlTemporaryPlanRustIntegerBitsJoin4Next(plan_5, index_5, next_count_term_3, next_active_term_2)
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
                                                                                let source_ml_source1_concat_value78 =
                                                                                    plan_4;
                                                                                {
                                                                                    let source_ml_source1_concat_value79 = vec![
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
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value78), source_ml_source1_concat_value79)
                                                                                }
                                                                            };
                                                                            {
                                                                                let index_6 =
                                                                                    index_4 + 1f64;
                                                                                self::MlTemporaryPlanRustIntegerBitsJoin4::MlTemporaryPlanRustIntegerBitsJoin4Next(plan_6, index_6, next_count_term_4, next_active_term)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                match source_ml_source1_s71 {
                                                    self::MlTemporaryPlanRustIntegerBitsJoin4::MlTemporaryPlanRustIntegerBitsJoin4Next(plan_7, index_7, next_count_term_5, next_active_term_3) => {
                                                        {
                                                            let quotient = index_7;
                                                            {
                                                                let plan_8 = {
                                                                    let source_ml_source1_concat_value80 = plan_7;
                                                                    {
                                                                        let source_ml_source1_concat_value81 = vec![13f64, at, 3f64, 0f64, 0f64];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value80), source_ml_source1_concat_value81)
                                                                    }
                                                                };
                                                                {
                                                                    let index_8 = index_7 + 1f64;
                                                                    {
                                                                        let bindings_2 = {
                                                                            let source_ml_source1_concat_value82 = bindings;
                                                                            {
                                                                                let source_ml_source1_concat_value83 = vec![next_current, quotient, next_count, next_count_term_5];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value82), source_ml_source1_concat_value83)
                                                                            }
                                                                        };
                                                                        {
                                                                            let bindings_4 = if trailing {
                                                                                {
                                                                                    {
                                                                                        let source_ml_source1_concat_value84 = bindings_2;
                                                                                        {
                                                                                            let source_ml_source1_concat_value85 = vec![next_active, next_active_term_3];
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value84), source_ml_source1_concat_value85)
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
            self::MlTemporaryPlanRustIntegerBitsLoop1Result::MlTemporaryPlanRustIntegerBitsLoop1Done(
                plan, bindings, count, index,
            )
        };
    }
}
