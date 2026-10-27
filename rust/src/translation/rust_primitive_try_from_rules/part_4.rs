// Generated from complete js/src/translation/rust-primitive-try-from-rules.js. Do not edit by hand.
use super::accept_rust_primitive_from_method;
use super::accept_rust_primitive_selection_receiver;
use super::ml_array;
use super::ml_string;
use super::read_rust_primitive_associated_owner;
use super::read_rust_primitive_integer_constant_owner;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_from_owner(
    segments: &[self::ml_string::Text],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> self::ml_string::Text {
    if !ordinary || shadowed {
        self::ml_string::Text::from_units(vec![])
    } else if ((((segments.len() as f64) == 2f64) && !absolute) && {
        let source_ml_source1_comparison_operand47 =
            self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand48 =
                self::ml_string::Text::from_units(vec![102, 54, 52]);
            match (
                source_ml_source1_comparison_operand47,
                source_ml_source1_comparison_operand48,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || (((((segments.len() as f64) == 4f64)
        && ({
            let source_ml_source1_comparison_operand49 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand50 =
                    self::ml_string::Text::from_units(vec![115, 116, 100]);
                match (
                    source_ml_source1_comparison_operand49,
                    source_ml_source1_comparison_operand50,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } || {
            let source_ml_source1_comparison_operand51 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand52 =
                    self::ml_string::Text::from_units(vec![99, 111, 114, 101]);
                match (
                    source_ml_source1_comparison_operand51,
                    source_ml_source1_comparison_operand52,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }))
        && {
            let source_ml_source1_comparison_operand53 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(1f64));
            {
                let source_ml_source1_comparison_operand54 =
                    self::ml_string::Text::from_units(vec![
                        112, 114, 105, 109, 105, 116, 105, 118, 101,
                    ]);
                match (
                    source_ml_source1_comparison_operand53,
                    source_ml_source1_comparison_operand54,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source1_comparison_operand55 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(2f64));
            {
                let source_ml_source1_comparison_operand56 =
                    self::ml_string::Text::from_units(vec![102, 54, 52]);
                match (
                    source_ml_source1_comparison_operand55,
                    source_ml_source1_comparison_operand56,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
    {
        self::ml_string::Text::from_units(vec![102, 54, 52])
    } else {
        self::read_rust_primitive_associated_owner(segments, ordinary, shadowed, absolute)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_from_literal_owner(
    form: &self::ml_string::Text,
    typed: bool,
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*form) != (self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 97, 108])))
        && ((*form)
            != (self::ml_string::Text::from_units(vec![
                110, 101, 103, 97, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 97, 108,
            ])))
    {
        self::ml_string::Text::from_units(vec![])
    } else if (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
        self::ml_string::Text::from_units(vec![102, 54, 52])
    } else if typed {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![105, 51, 50])
    }
}

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
pub fn read_rust_primitive_try_from_owner(
    segments: &[self::ml_string::Text],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> self::ml_string::Text {
    self::read_rust_primitive_integer_constant_owner(segments, ordinary, shadowed, absolute)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_try_from_import(
    path: &self::ml_string::Text,
    ordinary: bool,
    shadowed: bool,
) -> bool {
    (ordinary && !shadowed)
        && (((*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 111, 110, 118, 101, 114, 116, 58, 58, 84, 114, 121, 70,
                114, 111, 109,
            ])))
            || ((*path)
                == (self::ml_string::Text::from_units(vec![
                    99, 111, 114, 101, 58, 58, 99, 111, 110, 118, 101, 114, 116, 58, 58, 84, 114,
                    121, 70, 114, 111, 109,
                ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_try_from_source(
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
        && self::accept_rust_primitive_from_method(
            path,
            &(self::ml_string::Text::from_units(vec![102, 114, 111, 109])),
        ))
        && self::accept_rust_primitive_from_source(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_consumer_arity(method: &self::ml_string::Text) -> f64 {
    if (((*method) == (self::ml_string::Text::from_units(vec![105, 115, 95, 111, 107])))
        || ((*method) == (self::ml_string::Text::from_units(vec![105, 115, 95, 101, 114, 114]))))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100, 101, 102, 97, 117, 108, 116,
            ])))
    {
        0f64
    } else if (*method)
        == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 95, 111, 114]))
    {
        1f64
    } else if (*method) == (self::ml_string::Text::from_units(vec![109, 97, 112, 95, 111, 114])) {
        2f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_guards(
    source_bits: f64,
    source_signed: bool,
    target_bits: f64,
    target_signed: bool,
) -> f64 {
    if !(self::ml_array::includes(
        &vec![8f64, 16f64, 32f64, 64f64, 128f64],
        &source_bits,
        0f64,
        false,
        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
    )) || !(self::ml_array::includes(
        &vec![8f64, 16f64, 32f64, 64f64, 128f64],
        &target_bits,
        0f64,
        false,
        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
    )) {
        -1f64
    } else {
        {
            let lower = source_signed && (!target_signed || (source_bits > target_bits));
            {
                let upper = if target_signed {
                    if source_signed {
                        source_bits > target_bits
                    } else {
                        source_bits >= target_bits
                    }
                } else {
                    source_bits > target_bits
                };
                (if lower { 1f64 } else { 0f64 } + if upper { 2f64 } else { 0f64 })
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_try_from_consumer(
    method: &self::ml_string::Text,
    guards: f64,
) -> Vec<self::ml_string::Text> {
    if ((self::read_rust_primitive_try_from_consumer_arity(method)) < 0f64)
        || !(self::ml_array::includes(
            &vec![0f64, 1f64, 2f64, 3f64],
            &guards,
            0f64,
            false,
            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
        ))
    {
        Vec::<self::ml_string::Text>::new()
    } else {
        {
            let lower = vec![
                self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116]),
                self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
                self::ml_string::Text::from_units(vec![103, 101]),
            ];
            {
                let upper = vec![
                    self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116]),
                    self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
                    self::ml_string::Text::from_units(vec![108, 101]),
                ];
                {
                    let condition = if guards == 0f64 {
                        vec![self::ml_string::Text::from_units(vec![116, 114, 117, 101])]
                    } else if guards == 1f64 {
                        lower
                    } else if guards == 2f64 {
                        upper
                    } else {
                        {
                            let source_ml_source1_concat_value57 = lower;
                            {
                                let source_ml_source1_concat_value58 = upper;
                                {
                                    let source_ml_source1_concat_value59 =
                                        vec![self::ml_string::Text::from_units(vec![97, 110, 100])];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source1_concat_value57,
                                            ),
                                            source_ml_source1_concat_value58,
                                        ),
                                        source_ml_source1_concat_value59,
                                    )
                                }
                            }
                        }
                    };
                    if (*method)
                        == (self::ml_string::Text::from_units(vec![105, 115, 95, 111, 107]))
                    {
                        condition
                    } else if (*method)
                        == (self::ml_string::Text::from_units(vec![105, 115, 95, 101, 114, 114]))
                    {
                        {
                            let source_ml_source1_concat_value60 = condition;
                            {
                                let source_ml_source1_concat_value61 =
                                    vec![self::ml_string::Text::from_units(vec![110, 111, 116])];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value60,
                                    ),
                                    source_ml_source1_concat_value61,
                                )
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_concat_value62 = condition;
                            {
                                let source_ml_source1_concat_value63 = vec![
                                    self::ml_string::Text::from_units(vec![
                                        115, 117, 99, 99, 101, 115, 115,
                                    ]),
                                    if (*method)
                                        == (self::ml_string::Text::from_units(vec![
                                            117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100,
                                            101, 102, 97, 117, 108, 116,
                                        ]))
                                    {
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 102, 97, 117, 108, 116,
                                        ])
                                    } else {
                                        self::ml_string::Text::from_units(vec![
                                            102, 97, 108, 108, 98, 97, 99, 107,
                                        ])
                                    },
                                    self::ml_string::Text::from_units(vec![105, 102]),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value62,
                                    ),
                                    source_ml_source1_concat_value63,
                                )
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
pub fn read_rust_primitive_try_from_result_owner(
    method: &self::ml_string::Text,
    target: &self::ml_string::Text,
    mapper: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*method) == (self::ml_string::Text::from_units(vec![105, 115, 95, 111, 107])))
        || ((*method) == (self::ml_string::Text::from_units(vec![105, 115, 95, 101, 114, 114])))
    {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 95, 111, 114])))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100, 101, 102, 97, 117, 108, 116,
            ])))
    {
        target.clone()
    } else if (*method) == (self::ml_string::Text::from_units(vec![109, 97, 112, 95, 111, 114])) {
        mapper.clone()
    } else {
        self::ml_string::Text::from_units(vec![])
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
                    let source_ml_source1_concat_value64 = plan;
                    {
                        let source_ml_source1_concat_value65 = vec![
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
                                source_ml_source1_concat_value64,
                            ),
                            source_ml_source1_concat_value65,
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
