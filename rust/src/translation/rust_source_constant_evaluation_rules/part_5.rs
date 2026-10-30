// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::constant_bits;
use super::constant_domain;
use super::constant_expression;
use super::constant_field;
use super::constant_hint;
use super::constant_text;
use super::constant_type_owner;
use super::evaluate_rust_constant_integer_operation;
use super::ml_string;
use super::read_required_text_fact;
use super::read_rust_source_constant_integer_literal;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn constant_literal(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    expected: &self::ml_string::Text,
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
        if kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
            vec![
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]),
                self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
                self::constant_text(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                ),
            ]
        } else if kind == (self::ml_string::Text::from_units(vec![115, 116, 114])) {
            vec![
                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]),
                self::ml_string::Text::from_units(vec![115, 116, 114]),
                self::constant_text(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                ),
            ]
        } else {
            {
                let literal_type = self::constant_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                );
                {
                    let literal_kind = self::constant_text(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        literal_type,
                        &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
                    );
                    if (kind == (self::ml_string::Text::from_units(vec![108, 105, 116])))
                        && (literal_kind
                            == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))
                    {
                        vec![
                            self::ml_string::Text::from_units(vec![
                                98, 111, 111, 108, 101, 97, 110,
                            ]),
                            self::ml_string::Text::from_units(vec![98, 111, 111, 108]),
                            self::constant_text(
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                            ),
                        ]
                    } else if (kind == (self::ml_string::Text::from_units(vec![108, 105, 116])))
                        && (literal_kind
                            == (self::ml_string::Text::from_units(vec![
                                115, 116, 114, 105, 110, 103,
                            ])))
                    {
                        vec![
                            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]),
                            self::ml_string::Text::from_units(vec![115, 116, 114]),
                            self::constant_text(
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                            ),
                        ]
                    } else if (kind == (self::ml_string::Text::from_units(vec![110, 117, 109])))
                        || (kind == (self::ml_string::Text::from_units(vec![108, 105, 116])))
                    {
                        {
                            let owner = self::constant_type_owner(
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(if (self::constant_bits(expected)) > 0f64 {
                                    expected.clone()
                                } else {
                                    self::ml_string::Text::from_units(vec![105, 51, 50])
                                }),
                            );
                            {
                                let domain = self::constant_domain(&(owner));
                                {
                                    let literal = self::read_rust_source_constant_integer_literal(
                                        &(self::constant_text(
                                            texts,
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            node,
                                            &(self::ml_string::Text::from_units(vec![
                                                118, 97, 108, 117, 101,
                                            ])),
                                        )),
                                        ((self::constant_text(
                                            texts,
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            node,
                                            &(self::ml_string::Text::from_units(vec![
                                                110, 101, 103, 97, 116, 105, 118, 101,
                                            ])),
                                        )) == (self::ml_string::Text::from_units(vec![
                                            116, 114, 117, 101,
                                        ]))),
                                    );
                                    if (self::read_required_text_fact(&(literal), 0f64))
                                        == (self::ml_string::Text::from_units(vec![
                                            101, 114, 114, 111, 114,
                                        ]))
                                    {
                                        literal
                                    } else if (domain.len() as f64) == 2f64 {
                                        {
                                            let checked =
                                                self::evaluate_rust_constant_integer_operation(
                                                    &(self::ml_string::Text::from_units(vec![
                                                        97, 100, 100,
                                                    ])),
                                                    &(self::read_required_text_fact(
                                                        &(literal),
                                                        1f64,
                                                    )),
                                                    &(self::ml_string::Text::from_units(vec![48])),
                                                    &(self::read_required_text_fact(
                                                        &(domain),
                                                        0f64,
                                                    )),
                                                    &(self::read_required_text_fact(
                                                        &(domain),
                                                        1f64,
                                                    )),
                                                    self::constant_bits(&(owner)),
                                                );
                                            if (self::read_required_text_fact(&(checked), 0f64))
                                                == (self::ml_string::Text::from_units(vec![
                                                    101, 114, 114, 111, 114,
                                                ]))
                                            {
                                                checked
                                            } else {
                                                vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        105, 110, 116, 101, 103, 101, 114,
                                                    ]),
                                                    owner.clone(),
                                                    self::read_required_text_fact(&(checked), 1f64),
                                                ]
                                            }
                                        }
                                    } else {
                                        vec![
                                            self::ml_string::Text::from_units(vec![
                                                101, 114, 114, 111, 114,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32,
                                                110, 117, 109, 101, 114, 105, 99, 32, 101, 120,
                                                112, 114, 101, 115, 115, 105, 111, 110, 32, 110,
                                                101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99,
                                                116, 117, 97, 108, 32, 105, 110, 116, 101, 103,
                                                101, 114, 32, 111, 119, 110, 101, 114,
                                            ]),
                                        ]
                                    }
                                }
                            }
                        }
                    } else {
                        vec![
                            self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                            self::ml_string::Text::from_units(vec![
                                97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 108, 105, 116,
                                101, 114, 97, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115,
                                32, 97, 99, 116, 117, 97, 108, 32, 115, 99, 97, 108, 97, 114, 32,
                                115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101,
                            ]),
                        ]
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
pub fn constant_cast(
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
        let target =
            self::constant_type_owner(texts, starts, counts, labels, targets, node, expected);
        {
            let domain = self::constant_domain(&(target));
            {
                let arg = self::constant_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![97, 114, 103])),
                );
                {
                    let hint = self::constant_hint(
                        texts, starts, counts, labels, targets, rows, arg, names, values, 0f64,
                    );
                    {
                        let value = self::constant_expression(
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            rows,
                            arg,
                            &(if hint == (self::ml_string::Text::from_units(vec![])) {
                                self::ml_string::Text::from_units(vec![105, 51, 50])
                            } else {
                                hint.clone()
                            }),
                            names,
                            values,
                            visiting,
                            (depth + 1f64),
                            (fuel - 1f64),
                        );
                        if (self::read_required_text_fact(&(value), 0f64))
                            == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))
                        {
                            value
                        } else if (((self::read_required_text_fact(&(value), 0f64))
                            != (self::ml_string::Text::from_units(vec![
                                105, 110, 116, 101, 103, 101, 114,
                            ])))
                            && ((self::read_required_text_fact(&(value), 0f64))
                                != (self::ml_string::Text::from_units(vec![
                                    98, 111, 111, 108, 101, 97, 110,
                                ]))))
                            || ((domain.len() as f64) != 2f64)
                        {
                            vec![
                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 99, 97, 115,
                                    116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99,
                                    116, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32,
                                    115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 116, 97,
                                    114, 103, 101, 116, 32, 111, 119, 110, 101, 114, 115,
                                ]),
                            ]
                        } else {
                            {
                                let result = self::evaluate_rust_constant_integer_operation(
                                    &(self::ml_string::Text::from_units(vec![99, 97, 115, 116])),
                                    &(if (self::read_required_text_fact(&(value), 0f64))
                                        == (self::ml_string::Text::from_units(vec![
                                            98, 111, 111, 108, 101, 97, 110,
                                        ]))
                                    {
                                        if (self::read_required_text_fact(&(value), 2f64))
                                            == (self::ml_string::Text::from_units(vec![
                                                116, 114, 117, 101,
                                            ]))
                                        {
                                            self::ml_string::Text::from_units(vec![49])
                                        } else {
                                            self::ml_string::Text::from_units(vec![48])
                                        }
                                    } else {
                                        self::read_required_text_fact(&(value), 2f64)
                                    }),
                                    &(self::ml_string::Text::from_units(vec![48])),
                                    &(self::read_required_text_fact(&(domain), 0f64)),
                                    &(self::read_required_text_fact(&(domain), 1f64)),
                                    self::constant_bits(&(target)),
                                );
                                if (self::read_required_text_fact(&(result), 0f64))
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 114, 114, 111, 114,
                                    ]))
                                {
                                    result
                                } else {
                                    vec![
                                        self::ml_string::Text::from_units(vec![
                                            105, 110, 116, 101, 103, 101, 114,
                                        ]),
                                        target.clone(),
                                        self::read_required_text_fact(&(result), 1f64),
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn constant_optional_choice(
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
        let left = self::constant_expression(
            texts,
            starts,
            counts,
            labels,
            targets,
            rows,
            self::constant_field(
                starts,
                counts,
                labels,
                targets,
                node,
                &(self::ml_string::Text::from_units(vec![108, 101, 102, 116])),
            ),
            expected,
            names,
            values,
            visiting,
            (depth + 1f64),
            ((fuel - 1f64) / 2f64).floor(),
        );
        {
            let kind = self::read_required_text_fact(&(left), 0f64);
            if kind == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])) {
                left
            } else if kind == (self::ml_string::Text::from_units(vec![115, 111, 109, 101])) {
                {
                    let owner = self::read_required_text_fact(&(left), 1f64);
                    vec![
                        if owner == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                        } else if owner == (self::ml_string::Text::from_units(vec![115, 116, 114]))
                        {
                            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                        } else {
                            self::ml_string::Text::from_units(vec![
                                105, 110, 116, 101, 103, 101, 114,
                            ])
                        },
                        owner,
                        self::read_required_text_fact(&(left), 2f64),
                    ]
                }
            } else if kind == (self::ml_string::Text::from_units(vec![110, 111, 110, 101])) {
                self::constant_expression(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    rows,
                    self::constant_field(
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116])),
                    ),
                    expected,
                    names,
                    values,
                    visiting,
                    (depth + 1f64),
                    ((fuel - 1f64) / 2f64).floor(),
                )
            } else {
                vec![
                    self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                    self::ml_string::Text::from_units(vec![
                        97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 111, 112, 116, 105, 111,
                        110, 97, 108, 32, 99, 104, 111, 105, 99, 101, 32, 114, 101, 113, 117, 105,
                        114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111,
                        112, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101,
                    ]),
                ]
            }
        }
    }
}
