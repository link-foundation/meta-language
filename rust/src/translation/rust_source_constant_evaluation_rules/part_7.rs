// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::constant_bits;
use super::constant_cast;
use super::constant_domain;
use super::constant_element;
use super::constant_expression;
use super::constant_field;
use super::constant_finish;
use super::constant_hint;
use super::constant_literal;
use super::constant_operation;
use super::constant_optional_choice;
use super::constant_reference;
use super::constant_remaining;
use super::constant_row;
use super::constant_text;
use super::constant_type_owner;
use super::evaluate_rust_constant_boolean_operation;
use super::evaluate_rust_constant_integer_operation;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_required_text_fact;
use super::read_rust_source_constant_integer_literal;
use super::require_rust_constant_boolean_right;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn constant_body(
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
        if kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])) {
            vec![
                self::ml_string::Text::from_units(vec![117, 110, 105, 116]),
                self::ml_string::Text::from_units(vec![117, 110, 105, 116]),
                self::ml_string::Text::from_units(vec![]),
            ]
        } else if (((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))
            || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114]))))
            || (kind == (self::ml_string::Text::from_units(vec![110, 117, 109]))))
            || (kind == (self::ml_string::Text::from_units(vec![108, 105, 116])))
        {
            self::constant_literal(texts, starts, counts, labels, targets, node, expected)
        } else if ((kind == (self::ml_string::Text::from_units(vec![110, 97, 109, 101])))
            || (kind == (self::ml_string::Text::from_units(vec![118, 97, 114]))))
            || (kind == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))
        {
            self::constant_reference(
                texts, starts, counts, labels, targets, rows, node, expected, names, values,
                visiting, depth, fuel,
            )
        } else if kind == (self::ml_string::Text::from_units(vec![108, 101, 116])) {
            {
                let value = self::constant_expression(
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
                        &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                    ),
                    expected,
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
                } else {
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
                            &(self::ml_string::Text::from_units(vec![98, 111, 100, 121])),
                        ),
                        expected,
                        &({
                            let source_ml_source1_concat_value85 = names.to_vec();
                            {
                                let source_ml_source1_concat_value86 = vec![self::constant_text(
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    node,
                                    &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value85,
                                    ),
                                    source_ml_source1_concat_value86,
                                )
                            }
                        }),
                        &({
                            let source_ml_source1_concat_value87 = values.to_vec();
                            {
                                let source_ml_source1_concat_value88 =
                                    self::ml_array::slice(&value, 0f64, 3f64);
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value87,
                                    ),
                                    source_ml_source1_concat_value88,
                                )
                            }
                        }),
                        visiting,
                        (depth + 1f64),
                        self::constant_remaining(&(value)),
                    )
                }
            }
        } else if kind == (self::ml_string::Text::from_units(vec![105, 102])) {
            {
                let condition = self::constant_expression(
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
                        &(self::ml_string::Text::from_units(vec![99, 111, 110, 100])),
                    ),
                    &(self::ml_string::Text::from_units(vec![98, 111, 111, 108])),
                    names,
                    values,
                    visiting,
                    (depth + 1f64),
                    fuel,
                );
                if (self::read_required_text_fact(&(condition), 0f64))
                    == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))
                {
                    condition
                } else if (self::read_required_text_fact(&(condition), 0f64))
                    == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))
                {
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
                            &(if (self::read_required_text_fact(&(condition), 2f64))
                                == (self::ml_string::Text::from_units(vec![116, 114, 117, 101]))
                            {
                                self::ml_string::Text::from_units(vec![116, 104, 101, 110])
                            } else {
                                self::ml_string::Text::from_units(vec![101, 108, 115, 101])
                            }),
                        ),
                        expected,
                        names,
                        values,
                        visiting,
                        (depth + 1f64),
                        self::constant_remaining(&(condition)),
                    )
                } else {
                    vec![
                        self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                        self::ml_string::Text::from_units(vec![
                            97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 99, 111, 110, 100,
                            105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                            32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111,
                            108, 101, 97, 110, 32, 118, 97, 108, 117, 101,
                        ]),
                    ]
                }
            }
        } else if kind
            == (self::ml_string::Text::from_units(vec![99, 111, 97, 108, 101, 115, 99, 101]))
        {
            self::constant_optional_choice(
                texts, starts, counts, labels, targets, rows, node, expected, names, values,
                visiting, depth, fuel,
            )
        } else if kind == (self::ml_string::Text::from_units(vec![97, 98, 111, 114, 116])) {
            vec![
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                (self::ml_string::Text::from_units(vec![
                    116, 104, 101, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 105,
                    116, 105, 97, 108, 105, 122, 101, 114, 32, 97, 98, 111, 114, 116, 115, 58, 32,
                ]))
                .concat(&self::constant_text(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101])),
                )),
            ]
        } else if kind == (self::ml_string::Text::from_units(vec![99, 97, 115, 116])) {
            self::constant_cast(
                texts, starts, counts, labels, targets, rows, node, expected, names, values,
                visiting, depth, fuel,
            )
        } else if ((kind == (self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121])))
            || (kind == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]))))
            || (kind == (self::ml_string::Text::from_units(vec![109, 97, 116, 104])))
        {
            {
                let args = self::constant_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![97, 114, 103, 115])),
                );
                {
                    let first = self::constant_element(starts, counts, targets, args, 0f64);
                    {
                        let second = self::constant_element(starts, counts, targets, args, 1f64);
                        {
                            let left = if kind
                                == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))
                            {
                                if (self::constant_text(
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
                                        &(self::ml_string::Text::from_units(vec![
                                            118, 97, 108, 117, 101,
                                        ])),
                                    )
                                } else {
                                    first
                                }
                            } else {
                                self::constant_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    node,
                                    &(if kind
                                        == (self::ml_string::Text::from_units(vec![
                                            117, 110, 97, 114, 121,
                                        ]))
                                    {
                                        self::ml_string::Text::from_units(vec![97, 114, 103])
                                    } else {
                                        self::ml_string::Text::from_units(vec![108, 101, 102, 116])
                                    }),
                                )
                            };
                            {
                                let right = if kind
                                    == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))
                                {
                                    if (self::constant_text(
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        second,
                                        &(self::ml_string::Text::from_units(vec![107])),
                                    )) == (self::ml_string::Text::from_units(vec![]))
                                    {
                                        self::constant_field(
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            second,
                                            &(self::ml_string::Text::from_units(vec![
                                                118, 97, 108, 117, 101,
                                            ])),
                                        )
                                    } else {
                                        second
                                    }
                                } else {
                                    self::constant_field(
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        node,
                                        &(self::ml_string::Text::from_units(vec![
                                            114, 105, 103, 104, 116,
                                        ])),
                                    )
                                };
                                {
                                    let operation = self::constant_operation(
                                        &(self::constant_text(
                                            texts,
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            node,
                                            &(self::ml_string::Text::from_units(vec![111, 112])),
                                        )),
                                    );
                                    {
                                        let left_hint = self::constant_hint(
                                            texts, starts, counts, labels, targets, rows, left,
                                            names, values, 0f64,
                                        );
                                        {
                                            let right_hint = self::constant_hint(
                                                texts, starts, counts, labels, targets, rows,
                                                right, names, values, 0f64,
                                            );
                                            {
                                                let operand = if left_hint
                                                    == (self::ml_string::Text::from_units(vec![]))
                                                {
                                                    if right_hint
                                                        == (self::ml_string::Text::from_units(
                                                            vec![],
                                                        ))
                                                    {
                                                        if (self::constant_bits(expected)) > 0f64 {
                                                            expected.clone()
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![
                                                                105, 51, 50,
                                                            ])
                                                        }
                                                    } else {
                                                        right_hint.clone()
                                                    }
                                                } else {
                                                    left_hint
                                                };
                                                if (operation
                                                    == (self::ml_string::Text::from_units(vec![
                                                        110, 101, 103,
                                                    ])))
                                                    && (((self::constant_text(
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        left,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            107,
                                                        ])),
                                                    )) == (self::ml_string::Text::from_units(
                                                        vec![110, 117, 109],
                                                    ))) || ((self::constant_text(
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        left,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            107,
                                                        ])),
                                                    ))
                                                        == (self::ml_string::Text::from_units(
                                                            vec![108, 105, 116],
                                                        ))))
                                                {
                                                    {
                                                        let owner = self::constant_type_owner(
                                                            texts,
                                                            starts,
                                                            counts,
                                                            labels,
                                                            targets,
                                                            left,
                                                            &(operand),
                                                        );
                                                        {
                                                            let domain =
                                                                self::constant_domain(&(owner));
                                                            if !(owner.starts_with(
                                                                &self::ml_string::Text::from_units(
                                                                    vec![105],
                                                                ),
                                                            )) || ((domain.len() as f64) != 2f64)
                                                            {
                                                                vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 97, 110, 110, 111, 116, 32, 117, 115, 101, 32, 117, 110, 97, 114, 121, 32, 110, 101, 103, 97, 116, 105, 111, 110])]
                                                            } else {
                                                                {
                                                                    let literal = self::read_rust_source_constant_integer_literal(&(self::constant_text(texts, starts, counts, labels, targets, left, &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))), ((self::constant_text(texts, starts, counts, labels, targets, left, &(self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 105, 118, 101])))) != (self::ml_string::Text::from_units(vec![116, 114, 117, 101]))));
                                                                    if (self::read_required_text_fact(&(literal), 0f64)) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])) {
                                                                                                        literal
                                                                                                    } else {
                                                                                                        {
                                                                                                            let result = self::evaluate_rust_constant_integer_operation(&(self::ml_string::Text::from_units(vec![97, 100, 100])), &(self::read_required_text_fact(&(literal), 1f64)), &(self::ml_string::Text::from_units(vec![48])), &(self::read_required_text_fact(&(domain), 0f64)), &(self::read_required_text_fact(&(domain), 1f64)), self::constant_bits(&(owner)));
                                                                                                            if (self::read_required_text_fact(&(result), 0f64)) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])) {
                                                                                                                result
                                                                                                            } else {
                                                                                                                vec![self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]), owner.clone(), self::read_required_text_fact(&(result), 1f64)]
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let unary_input = (kind == (self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121]))) || (operation == (self::ml_string::Text::from_units(vec![98, 105, 116, 110, 111, 116])));
                                                        {
                                                            let a = self::constant_expression(
                                                                texts,
                                                                starts,
                                                                counts,
                                                                labels,
                                                                targets,
                                                                rows,
                                                                left,
                                                                &(operand),
                                                                names,
                                                                values,
                                                                visiting,
                                                                (depth + 1f64),
                                                                fuel,
                                                            );
                                                            if (self::read_required_text_fact(&(a), 0f64)) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])) {
                                                                                                a
                                                                                            } else if ((self::read_required_text_fact(&(a), 0f64)) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) && !(self::require_rust_constant_boolean_right(&(operation), ((self::read_required_text_fact(&(a), 2f64)) == (self::ml_string::Text::from_units(vec![116, 114, 117, 101]))))) {
                                                                                                    a
                                                                                                } else {
                                                                                                    {
                                                                                                        let operation_3 = if (operation == (self::ml_string::Text::from_units(vec![110, 111, 116]))) && ((self::read_required_text_fact(&(a), 0f64)) == (self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]))) {
                                                                                                            {
                                                                                                                self::ml_string::Text::from_units(vec![98, 105, 116, 110, 111, 116])
                                                                                                            }
                                                                                                        } else {
                                                                                                            operation.clone()
                                                                                                        };
                                                                                                        {
                                                                                                            let unary = unary_input;
                                                                                                            {
                                                                                                                let b = if unary {
                                                                                                                    vec![self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]), self::read_required_text_fact(&(a), 1f64), self::ml_string::Text::from_units(vec![48]), {
                                                                                                                        let source_ml_source1_string_argument89 = self::constant_remaining(&(a));
                                                                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument89))
                                                                                                                    }]
                                                                                                                } else {
                                                                                                                    self::constant_expression(texts, starts, counts, labels, targets, rows, right, &(if (operation_3 == (self::ml_string::Text::from_units(vec![115, 104, 108]))) || (operation_3 == (self::ml_string::Text::from_units(vec![115, 104, 114]))) {
                                                                                                                        if right_hint == (self::ml_string::Text::from_units(vec![]))  {
                                                                                                                            self::ml_string::Text::from_units(vec![105, 51, 50])
                                                                                                                        } else {
                                                                                                                            right_hint.clone()
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        self::read_required_text_fact(&(a), 1f64)
                                                                                                                    }), names, values, visiting, (depth + 1f64), self::constant_remaining(&(a)))
                                                                                                                };
                                                                                                                if (self::read_required_text_fact(&(b), 0f64)) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])) {
                                                                                                                    b
                                                                                                                } else if ((self::read_required_text_fact(&(a), 0f64)) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) && (unary || ((self::read_required_text_fact(&(b), 0f64)) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) {
                                                                                                                        {
                                                                                                                            let result_2 = self::evaluate_rust_constant_boolean_operation(&(operation_3), ((self::read_required_text_fact(&(a), 2f64)) == (self::ml_string::Text::from_units(vec![116, 114, 117, 101]))), ((self::read_required_text_fact(&(b), 2f64)) == (self::ml_string::Text::from_units(vec![116, 114, 117, 101]))));
                                                                                                                            if (self::read_required_text_fact(&(result_2), 0f64)) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])) {
                                                                                                                                result_2
                                                                                                                            } else {
                                                                                                                                self::constant_finish(&(vec![self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![98, 111, 111, 108]), self::read_required_text_fact(&(result_2), 1f64)]), self::constant_remaining(&(b)))
                                                                                                                            }
                                                                                                                        }
                                                                                                                    } else if (((self::read_required_text_fact(&(a), 0f64)) != (self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]))) || ((self::read_required_text_fact(&(b), 0f64)) != (self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])))) || (((!unary && (operation_3 != (self::ml_string::Text::from_units(vec![115, 104, 108])))) && (operation_3 != (self::ml_string::Text::from_units(vec![115, 104, 114])))) && ((self::read_required_text_fact(&(a), 1f64)) != (self::read_required_text_fact(&(b), 1f64)))) {
                                                                                                                            vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 97, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 99, 111, 109, 112, 97, 116, 105, 98, 108, 101, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 111, 119, 110, 101, 114, 115])]
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let owner_2 = self::read_required_text_fact(&(a), 1f64);
                                                                                                                                {
                                                                                                                                    let domain_2 = self::constant_domain(&(owner_2));
                                                                                                                                    if (domain_2.len() as f64) == 2f64  {
                                                                                                                                        {
                                                                                                                                            let result_3 = self::evaluate_rust_constant_integer_operation(&(operation_3), &(self::read_required_text_fact(&(a), 2f64)), &(self::read_required_text_fact(&(b), 2f64)), &(self::read_required_text_fact(&(domain_2), 0f64)), &(self::read_required_text_fact(&(domain_2), 1f64)), self::constant_bits(&(owner_2)));
                                                                                                                                            {
                                                                                                                                                let original = self::constant_text(texts, starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![111, 112])));
                                                                                                                                                if (original == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 67, 104, 101, 99, 107, 101, 100, 82, 117, 115, 116, 76, 101, 102, 116, 83, 104, 105, 102, 116]))) || (original == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 67, 104, 101, 99, 107, 101, 100, 82, 117, 115, 116, 82, 105, 103, 104, 116, 83, 104, 105, 102, 116]))) {
                                                                                                                                                    if (self::read_required_text_fact(&(result_3), 0f64)) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])) {
                                                                                                                                                        if (self::read_required_text_fact(&(result_3), 1f64)) == (self::ml_string::Text::from_units(vec![111, 118, 101, 114, 102, 108, 111, 119, 32, 105, 110, 32, 97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 115, 104, 105, 102, 116])) {
                                                                                                                                                            self::constant_finish(&(vec![self::ml_string::Text::from_units(vec![110, 111, 110, 101]), owner_2.clone(), self::ml_string::Text::from_units(vec![])]), self::constant_remaining(&(b)))
                                                                                                                                                        } else {
                                                                                                                                                            result_3
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        self::constant_finish(&(vec![self::ml_string::Text::from_units(vec![115, 111, 109, 101]), owner_2.clone(), self::read_required_text_fact(&(result_3), 1f64)]), self::constant_remaining(&(b)))
                                                                                                                                                    }
                                                                                                                                                } else if (self::read_required_text_fact(&(result_3), 0f64)) == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])) {
                                                                                                                                                        result_3
                                                                                                                                                    } else {
                                                                                                                                                        self::constant_finish(&(vec![self::read_required_text_fact(&(result_3), 0f64), if (self::read_required_text_fact(&(result_3), 0f64)) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])) {
                                                                                                                                                            self::ml_string::Text::from_units(vec![98, 111, 111, 108])
                                                                                                                                                        } else {
                                                                                                                                                            owner_2.clone()
                                                                                                                                                        }, self::read_required_text_fact(&(result_3), 1f64)]), self::constant_remaining(&(b)))
                                                                                                                                                    }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 111, 119, 110, 101, 114])]
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
            vec![
                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                self::ml_string::Text::from_units(vec![
                    116, 104, 101, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 105,
                    116, 105, 97, 108, 105, 122, 101, 114, 32, 101, 120, 112, 114, 101, 115, 115,
                    105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 111,
                    117, 114, 99, 101, 32, 101, 118, 97, 108, 117, 97, 116, 105, 111, 110, 32, 105,
                    110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110,
                ]),
            ]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn evaluate_rust_source_constant(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    root: f64,
    owner: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    {
        let rows = self::constant_field(
            starts,
            counts,
            labels,
            targets,
            root,
            &(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 97, 110, 116, 115])),
        );
        {
            let row = self::constant_row(texts, starts, counts, labels, targets, rows, owner);
            if row < 0f64 {
                vec![
                    self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                    self::ml_string::Text::from_units(vec![
                        97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 112, 97, 116, 116, 101,
                        114, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                        117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 100, 101, 99, 108, 97,
                        114, 97, 116, 105, 111, 110,
                    ]),
                ]
            } else if (self::constant_text(
                texts,
                starts,
                counts,
                labels,
                targets,
                row,
                &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
            )) == (self::ml_string::Text::from_units(vec![
                102, 117, 110, 99, 116, 105, 111, 110,
            ])) {
                vec![
                    self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                    self::ml_string::Text::from_units(vec![
                        97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 112, 97, 116, 116, 101,
                        114, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                        117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116,
                        97, 110, 116, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
                    ]),
                ]
            } else {
                {
                    let domain = self::constant_text(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        row,
                        &(self::ml_string::Text::from_units(vec![100, 111, 109, 97, 105, 110])),
                    );
                    {
                        let result = self::constant_expression(
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
                                row,
                                &(self::ml_string::Text::from_units(vec![98, 111, 100, 121])),
                            ),
                            &(domain),
                            &(Vec::<self::ml_string::Text>::new()),
                            &(Vec::<self::ml_string::Text>::new()),
                            &(vec![owner.clone()]),
                            0f64,
                            65_536f64,
                        );
                        if (self::read_required_text_fact(&(result), 0f64))
                            == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))
                        {
                            result
                        } else if (self::read_required_text_fact(&(result), 1f64)) == domain {
                            self::ml_array::slice(&result, 0f64, 3f64)
                        } else {
                            vec![
                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110,
                                    105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 109, 117, 115,
                                    116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101,
                                    100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 111,
                                    119, 110, 101, 114,
                                ]),
                            ]
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result {
    MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Done,
    MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_rust_integer_range_canonical_decimal_loop1(
    digits: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result {
    loop {
        return if at < (digits.length()) {
            {
                let ml_product_condition_1 = {
                    let source_ml_source1_string_search_operand108 =
                        self::ml_string::Text::from_units(vec![
                            48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                        ]);
                    {
                        let source_ml_source1_string_search_operand109 = {
                            let source_ml_source1_string_unit_operand101 = digits.clone();
                            {
                                let source_ml_source1_string_unit_operand102 = at;
                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                let index = if position.is_nan() {
                                    0f64
                                } else {
                                    position.trunc()
                                };
                                if (index < 0f64) || (index >= (text.length())) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    {
                                        let source_ml_source1_string_range_operand103 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand104 = index;
                                            {
                                                let source_ml_source1_string_range_operand105 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position106 = source_ml_source1_string_range_operand104;
                                                    {
                                                        let source_ml_source1_string_range_position107 = source_ml_source1_string_range_operand105;
                                                        source_ml_source1_string_range_operand103.slice(source_ml_source1_string_range_position106, source_ml_source1_string_range_position107)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } }))(source_ml_source1_string_unit_operand101, source_ml_source1_string_unit_operand102)
                            }
                        };
                        {
                            let source_ml_source1_string_search_text110 =
                                source_ml_source1_string_search_operand109;
                            {
                                let source_ml_source1_string_search_position111 = 0f64;
                                source_ml_source1_string_search_operand108.index_of(
                                    &source_ml_source1_string_search_text110,
                                    source_ml_source1_string_search_position111,
                                )
                            }
                        }
                    }
                };
                if ml_product_condition_1 {
                    self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Return(false)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Result::MlTemporaryTestRustIntegerRangeCanonicalDecimalLoop1Done
        };
    }
}
