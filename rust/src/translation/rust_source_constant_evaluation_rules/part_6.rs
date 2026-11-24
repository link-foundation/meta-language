// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::MlTemporaryConstantRemainingLoop32Result;
use super::accept_source_text_scalar_units;
use super::constant_body;
use super::constant_element;
use super::constant_field;
use super::constant_function;
use super::constant_row;
use super::constant_text;
use super::constant_type_owner;
use super::measure_rust_utf8_byte_length;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_constant_pattern_evaluation_error;
use super::source_ml_temporary_constant_byte_length_loop33;
use super::source_ml_temporary_constant_remaining_loop32;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn constant_reference(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    rows: f64,
    node: f64,
    expected: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    visiting: &[self::ml_string::Text],
    depth: f64,
    fuel: f64,
) -> Vec<self::ml_string::Text> {
    {
        let kind = self::constant_text(
            texts,
            starts,
            counts,
            labels,
            targets,
            node,
            &(self::ml_string::Text::from_units(vec![107])),
        );
        {
            let path = self::constant_field(
                starts,
                counts,
                labels,
                targets,
                node,
                &(self::ml_string::Text::from_units(vec![112, 97, 116, 104])),
            );
            {
                let head = self::constant_element(starts, counts, targets, path, 0f64);
                {
                    let name = if kind == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
                        self::constant_text(
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            node,
                            &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                        )
                    } else if head < 0f64 {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::read_required_text_fact(texts, head)
                    };
                    {
                        let bound = if (kind
                            == (self::ml_string::Text::from_units(vec![118, 97, 114])))
                            || ((path >= 0f64)
                                && ((self::read_required_number_fact(counts, path)) == 1f64))
                        {
                            self::ml_array::index_of(names, &name, f64::INFINITY, true, |a, b| {
                                a == b
                            })
                        } else {
                            -1f64
                        };
                        if bound >= 0f64 {
                            vec![
                                self::read_required_text_fact(values, (bound * 3f64)),
                                self::read_required_text_fact(values, ((bound * 3f64) + 1f64)),
                                self::read_required_text_fact(values, ((bound * 3f64) + 2f64)),
                            ]
                        } else {
                            {
                                let reference = self::constant_text(
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    node,
                                    &(self::ml_string::Text::from_units(vec![
                                        115, 111, 117, 114, 99, 101, 67, 111, 110, 115, 116, 97,
                                        110, 116, 79, 119, 110, 101, 114,
                                    ])),
                                );
                                {
                                    let owner = {
                                        let ml_library_inequality_2 = reference
                                            != (self::ml_string::Text::from_units(vec![]));
                                        if ml_library_inequality_2 {
                                            reference.clone()
                                        } else if kind
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 97, 108, 108,
                                            ]))
                                        {
                                            self::constant_text(
                                                texts,
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                node,
                                                &(self::ml_string::Text::from_units(vec![
                                                    102, 110,
                                                ])),
                                            )
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                    };
                                    {
                                        let row = self::constant_row(
                                            texts,
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            rows,
                                            &(owner),
                                        );
                                        {
                                            let args = self::constant_field(
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                node,
                                                &(self::ml_string::Text::from_units(vec![
                                                    97, 114, 103, 115,
                                                ])),
                                            );
                                            if (self::constant_text(
                                                texts,
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                row,
                                                &(self::ml_string::Text::from_units(vec![
                                                    107, 105, 110, 100,
                                                ])),
                                            )) == (self::ml_string::Text::from_units(vec![
                                                102, 117, 110, 99, 116, 105, 111, 110,
                                            ])) {
                                                if kind
                                                    == (self::ml_string::Text::from_units(vec![
                                                        99, 97, 108, 108,
                                                    ]))
                                                {
                                                    self::constant_function(
                                                        texts, starts, counts, labels, targets,
                                                        rows, node, row, names, values, visiting,
                                                        depth, fuel,
                                                    )
                                                } else {
                                                    vec![
                                                        self::ml_string::Text::from_units(vec![
                                                            101, 114, 114, 111, 114,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            97, 32, 99, 111, 110, 115, 116, 97,
                                                            110, 116, 32, 102, 117, 110, 99, 116,
                                                            105, 111, 110, 32, 105, 116, 101, 109,
                                                            32, 110, 101, 101, 100, 115, 32, 97,
                                                            110, 32, 97, 99, 116, 117, 97, 108, 32,
                                                            99, 97, 108, 108, 32, 101, 120, 112,
                                                            114, 101, 115, 115, 105, 111, 110,
                                                        ]),
                                                    ]
                                                }
                                            } else if ((kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    99, 97, 108, 108,
                                                ])))
                                                && (args >= 0f64))
                                                && ((self::read_required_number_fact(counts, args))
                                                    != 0f64)
                                            {
                                                vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        101, 114, 114, 111, 114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        99, 111, 110, 115, 116, 97, 110, 116, 32,
                                                        102, 117, 110, 99, 116, 105, 111, 110, 32,
                                                        99, 97, 108, 108, 115, 32, 110, 101, 101,
                                                        100, 32, 116, 104, 101, 105, 114, 32, 97,
                                                        99, 116, 117, 97, 108, 32, 99, 111, 110,
                                                        115, 116, 32, 115, 111, 117, 114, 99, 101,
                                                        32, 105, 110, 116, 101, 114, 112, 114, 101,
                                                        116, 97, 116, 105, 111, 110,
                                                    ]),
                                                ]
                                            } else if row < 0f64 {
                                                vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        101, 114, 114, 111, 114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        97, 32, 99, 111, 110, 115, 116, 97, 110,
                                                        116, 32, 101, 120, 112, 114, 101, 115, 115,
                                                        105, 111, 110, 32, 110, 97, 109, 101, 32,
                                                        110, 101, 101, 100, 115, 32, 105, 116, 115,
                                                        32, 97, 99, 116, 117, 97, 108, 32, 115,
                                                        111, 117, 114, 99, 101, 32, 99, 111, 110,
                                                        115, 116, 97, 110, 116, 32, 111, 114, 32,
                                                        108, 111, 99, 97, 108, 32, 98, 105, 110,
                                                        100, 105, 110, 103,
                                                    ]),
                                                ]
                                            } else if self::ml_array::includes(
                                                visiting,
                                                &owner.clone(),
                                                0f64,
                                                false,
                                                |a, b| a == b,
                                            ) {
                                                vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::read_rust_constant_pattern_evaluation_error(depth, 0f64, true)]
                                            } else {
                                                {
                                                    let domain = self::constant_text(
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        row,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            100, 111, 109, 97, 105, 110,
                                                        ])),
                                                    );
                                                    {
                                                        let result = self::constant_expression(texts, starts, counts, labels, targets, rows, self::constant_field(starts, counts, labels, targets, row, &(self::ml_string::Text::from_units(vec![98, 111, 100, 121]))), &(domain), &(Vec::<self::ml_string::Text>::new()), &(Vec::<self::ml_string::Text>::new()), &({
                                                                        let source_ml_source1_concat_value84 = visiting.to_vec();
                                                                        {
                                                                            let source_ml_source1_concat_value85 = vec![owner.clone()];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value84), source_ml_source1_concat_value85)
                                                                        }
                                                                    }), (depth + 1f64), fuel);
                                                        if (self::read_required_text_fact(
                                                            &(result),
                                                            0f64,
                                                        )) == (self::ml_string::Text::from_units(
                                                            vec![101, 114, 114, 111, 114],
                                                        )) {
                                                            result
                                                        } else {
                                                            let ml_library_inequality_1 =
                                                                (self::read_required_text_fact(
                                                                    &(result),
                                                                    1f64,
                                                                )) != domain;
                                                            if ml_library_inequality_1 {
                                                                vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 111, 119, 110, 101, 114])]
                                                            } else {
                                                                result
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
pub fn constant_finish(value: &[self::ml_string::Text], fuel: f64) -> Vec<self::ml_string::Text> {
    if (self::read_required_text_fact(value, 0f64))
        == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))
    {
        value.to_vec()
    } else {
        vec![
            self::read_required_text_fact(value, 0f64),
            self::read_required_text_fact(value, 1f64),
            self::read_required_text_fact(value, 2f64),
            {
                let source_ml_source1_string_argument86 = fuel;
                self::ml_string::Text::from_text(&self::ml_number::js_number(
                    source_ml_source1_string_argument86,
                ))
            },
        ]
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn constant_remaining(value: &[self::ml_string::Text]) -> f64 {
    {
        let text = self::read_required_text_fact(value, 3f64);
        {
            let result = 0f64;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s87 =
                        self::source_ml_temporary_constant_remaining_loop32(&(text), result, at);
                    match source_ml_source1_s87 {
                        self::MlTemporaryConstantRemainingLoop32Result::MlTemporaryConstantRemainingLoop32Done(result_2) => {
                            result_2
                        }
                        self::MlTemporaryConstantRemainingLoop32Result::MlTemporaryConstantRemainingLoop32Return(source_ml_temporary_result32) => {
                            source_ml_temporary_result32
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn constant_expression(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    rows: f64,
    node: f64,
    expected: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    visiting: &[self::ml_string::Text],
    depth: f64,
    fuel: f64,
) -> Vec<self::ml_string::Text> {
    {
        let budget =
            self::read_rust_constant_pattern_evaluation_error(depth, (65_536f64 - fuel), false);
        if ((node < 0f64) || (fuel <= 0f64))
            || (budget != (self::ml_string::Text::from_units(vec![])))
        {
            vec![
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                if budget != (self::ml_string::Text::from_units(vec![])) {
                    budget
                } else {
                    self::ml_string::Text::from_units(vec![
                        99, 111, 110, 115, 116, 97, 110, 116, 32, 112, 97, 116, 116, 101, 114, 110,
                        32, 101, 118, 97, 108, 117, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101,
                        101, 100, 115, 32, 105, 116, 115, 32, 115, 111, 117, 114, 99, 101, 32, 114,
                        101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116,
                    ])
                },
            ]
        } else {
            {
                let result = self::constant_body(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    rows,
                    node,
                    expected,
                    names,
                    values,
                    visiting,
                    depth,
                    (fuel - 1f64),
                );
                if (result.len() as f64) == 3f64 {
                    self::constant_finish(&(result), (fuel - 1f64))
                } else {
                    result
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn constant_byte_length(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    rows: f64,
    node: f64,
    expected: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    visiting: &[self::ml_string::Text],
    depth: f64,
    fuel: f64,
) -> Vec<self::ml_string::Text> {
    {
        let args = self::constant_field(
            starts,
            counts,
            labels,
            targets,
            node,
            &(self::ml_string::Text::from_units(vec![97, 114, 103, 115])),
        );
        if (args < 0f64) || ((self::read_required_number_fact(counts, args)) != 1f64) {
            vec![
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                self::ml_string::Text::from_units(vec![
                    97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 98, 121, 116, 101, 32, 108,
                    101, 110, 103, 116, 104, 32, 110, 101, 101, 100, 115, 32, 111, 110, 101, 32,
                    97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110, 103, 32, 111, 112, 101,
                    114, 97, 110, 100,
                ]),
            ]
        } else {
            {
                let first = self::constant_element(starts, counts, targets, args, 0f64);
                {
                    let input = if (self::constant_text(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        first,
                        &(self::ml_string::Text::from_units(vec![107])),
                    )) == (self::ml_string::Text::from_units(vec![]))
                    {
                        self::constant_field(
                            starts,
                            counts,
                            labels,
                            targets,
                            first,
                            &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                        )
                    } else {
                        first
                    };
                    {
                        let value = self::constant_expression(
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            rows,
                            input,
                            &(self::ml_string::Text::from_units(vec![115, 116, 114])),
                            names,
                            values,
                            visiting,
                            (depth + 1f64),
                            fuel,
                        );
                        if (self::read_required_text_fact(&(value), 0f64))
                            == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))
                        {
                            value
                        } else if ((self::read_required_text_fact(&(value), 0f64))
                            != (self::ml_string::Text::from_units(vec![
                                115, 116, 114, 105, 110, 103,
                            ])))
                            || ((self::read_required_text_fact(&(value), 1f64))
                                != (self::ml_string::Text::from_units(vec![115, 116, 114])))
                        {
                            vec![
                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 98, 121, 116,
                                    101, 32, 108, 101, 110, 103, 116, 104, 32, 110, 101, 101, 100,
                                    115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85,
                                    110, 105, 99, 111, 100, 101, 32, 115, 99, 97, 108, 97, 114, 32,
                                    115, 116, 114, 105, 110, 103,
                                ]),
                            ]
                        } else {
                            {
                                let text = self::read_required_text_fact(&(value), 2f64);
                                {
                                    let remaining = self::constant_remaining(&(value));
                                    if (text.length()) > remaining {
                                        vec![
                                            self::ml_string::Text::from_units(vec![
                                                101, 114, 114, 111, 114,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 115, 116, 97, 110, 116, 32, 112, 97,
                                                116, 116, 101, 114, 110, 32, 101, 118, 97, 108,
                                                117, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101,
                                                101, 100, 115, 32, 105, 116, 115, 32, 115, 111,
                                                117, 114, 99, 101, 32, 114, 101, 115, 111, 117,
                                                114, 99, 101, 32, 98, 117, 100, 103, 101, 116,
                                            ]),
                                        ]
                                    } else {
                                        {
                                            let units = Vec::<f64>::new();
                                            {
                                                let at = 0f64;
                                                {
                                                    let units_2 = self::source_ml_temporary_constant_byte_length_loop33(&(text), units, at);
                                                    if (self::accept_source_text_scalar_units(
                                                        &(units_2),
                                                    )) {
                                                        {
                                                            let owner = self::constant_type_owner(
                                                                texts, starts, counts, labels,
                                                                targets, node, expected,
                                                            );
                                                            if owner == (self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]))  {
                                                                    self::constant_finish(&(vec![self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]), owner.clone(), {
                                                                        let source_ml_source1_string_argument88 = self::measure_rust_utf8_byte_length(&(units_2));
                                                                        self::ml_string::Text::from_text(&source_ml_source1_string_argument88.to_string())
                                                                    }]), (remaining - text.length()))
                                                                } else {
                                                                    vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 98, 121, 116, 101, 32, 108, 101, 110, 103, 116, 104, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 115, 105, 122, 101, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114])]
                                                                }
                                                        }
                                                    } else {
                                                        vec![
                                                            self::ml_string::Text::from_units(
                                                                vec![101, 114, 114, 111, 114],
                                                            ),
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    97, 32, 99, 111, 110, 115, 116,
                                                                    97, 110, 116, 32, 98, 121, 116,
                                                                    101, 32, 108, 101, 110, 103,
                                                                    116, 104, 32, 110, 101, 101,
                                                                    100, 115, 32, 105, 116, 115,
                                                                    32, 97, 99, 116, 117, 97, 108,
                                                                    32, 85, 110, 105, 99, 111, 100,
                                                                    101, 32, 115, 99, 97, 108, 97,
                                                                    114, 32, 115, 116, 114, 105,
                                                                    110, 103,
                                                                ],
                                                            ),
                                                        ]
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
