// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::MlTemporaryConstantRowLoop30Result;
use super::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Result;
use super::MlTemporaryReadRustSourceConstantIntegerLiteralJoin25;
use super::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26;
use super::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Result;
use super::constant_bits;
use super::constant_element;
use super::constant_field;
use super::constant_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_constant_primitive_alias_owner;
use super::source_ml_temporary_constant_row_loop30;
use super::source_ml_temporary_read_rust_source_constant_initializer_boundary_loop29;
use super::source_ml_temporary_read_rust_source_constant_integer_literal_loop27;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_constant_integer_literal(
    text: &self::ml_string::Text,
    negative: bool,
) -> Vec<self::ml_string::Text> {
    {
        let digits = text.clone();
        {
            let base = self::ml::Big::from_i128(10);
            {
                let limit = 10f64;
                {
                    let negated = negative;
                    {
                        let source_ml_source1_s49 = if digits
                            .starts_with(&self::ml_string::Text::from_units(vec![45]))
                        {
                            {
                                let digits_2 = {
                                    let source_ml_source1_string_range_operand50 = digits;
                                    {
                                        let source_ml_source1_string_range_operand51 = 1f64;
                                        {
                                            let source_ml_source1_string_range_position52 =
                                                source_ml_source1_string_range_operand51;
                                            {
                                                let source_ml_source1_string_range_position53 =
                                                    f64::INFINITY;
                                                source_ml_source1_string_range_operand50.slice(
                                                    source_ml_source1_string_range_position52,
                                                    source_ml_source1_string_range_position53,
                                                )
                                            }
                                        }
                                    }
                                };
                                {
                                    let negated_2 = !negated;
                                    self::MlTemporaryReadRustSourceConstantIntegerLiteralJoin25::MlTemporaryReadRustSourceConstantIntegerLiteralJoin25Next(digits_2, negated_2)
                                }
                            }
                        } else {
                            self::MlTemporaryReadRustSourceConstantIntegerLiteralJoin25::MlTemporaryReadRustSourceConstantIntegerLiteralJoin25Next(digits, negated)
                        };
                        match source_ml_source1_s49 {
                            self::MlTemporaryReadRustSourceConstantIntegerLiteralJoin25::MlTemporaryReadRustSourceConstantIntegerLiteralJoin25Next(digits_3, negated_3) => {
                                {
                                    let source_ml_source1_s66 = if digits_3.starts_with(&self::ml_string::Text::from_units(vec![48, 120])) || digits_3.starts_with(&self::ml_string::Text::from_units(vec![48, 88])) {
                                        {
                                            let digits_4 = {
                                                let source_ml_source1_string_range_operand67 = digits_3;
                                                {
                                                    let source_ml_source1_string_range_operand68 = 2f64;
                                                    {
                                                        let source_ml_source1_string_range_position69 = source_ml_source1_string_range_operand68;
                                                        {
                                                            let source_ml_source1_string_range_position70 = f64::INFINITY;
                                                            source_ml_source1_string_range_operand67.slice(source_ml_source1_string_range_position69, source_ml_source1_string_range_position70)
                                                        }
                                                    }
                                                }
                                            };
                                            {
                                                let base_2 = self::ml::Big::from_i128(16);
                                                {
                                                    let limit_2 = 16f64;
                                                    self::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26Next(digits_4, base_2, limit_2)
                                                }
                                            }
                                        }
                                    } else if digits_3.starts_with(&self::ml_string::Text::from_units(vec![48, 111])) || digits_3.starts_with(&self::ml_string::Text::from_units(vec![48, 79])) {
                                            {
                                                let digits_5 = {
                                                    let source_ml_source1_string_range_operand71 = digits_3;
                                                    {
                                                        let source_ml_source1_string_range_operand72 = 2f64;
                                                        {
                                                            let source_ml_source1_string_range_position73 = source_ml_source1_string_range_operand72;
                                                            {
                                                                let source_ml_source1_string_range_position74 = f64::INFINITY;
                                                                source_ml_source1_string_range_operand71.slice(source_ml_source1_string_range_position73, source_ml_source1_string_range_position74)
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let base_3 = self::ml::Big::from_i128(8);
                                                    {
                                                        let limit_3 = 8f64;
                                                        self::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26Next(digits_5, base_3, limit_3)
                                                    }
                                                }
                                            }
                                        } else if digits_3.starts_with(&self::ml_string::Text::from_units(vec![48, 98])) || digits_3.starts_with(&self::ml_string::Text::from_units(vec![48, 66])) {
                                                {
                                                    let digits_6 = {
                                                        let source_ml_source1_string_range_operand75 = digits_3;
                                                        {
                                                            let source_ml_source1_string_range_operand76 = 2f64;
                                                            {
                                                                let source_ml_source1_string_range_position77 = source_ml_source1_string_range_operand76;
                                                                {
                                                                    let source_ml_source1_string_range_position78 = f64::INFINITY;
                                                                    source_ml_source1_string_range_operand75.slice(source_ml_source1_string_range_position77, source_ml_source1_string_range_position78)
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let base_4 = self::ml::Big::from_i128(2);
                                                        {
                                                            let limit_4 = 2f64;
                                                            self::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26Next(digits_6, base_4, limit_4)
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26Next(digits_3, base, limit)
                                            };
                                    match source_ml_source1_s66 {
                                        self::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26::MlTemporaryReadRustSourceConstantIntegerLiteralJoin26Next(digits_7, base_5, limit_5) => {
                                            if digits_7 == (self::ml_string::Text::from_units(vec![])) {
                                                vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 97, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 108, 105, 116, 101, 114, 97, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 115, 112, 101, 108, 108, 105, 110, 103])]
                                            } else {
                                                {
                                                    let value = self::ml::Big::from_i128(0);
                                                    {
                                                        let at = 0f64;
                                                        {
                                                            let source_ml_source1_s79 = self::source_ml_temporary_read_rust_source_constant_integer_literal_loop27(&(digits_7), base_5.clone(), limit_5, value, at);
                                                            match source_ml_source1_s79 {
                                                                self::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Result::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Done(value_2) => {
                                                                    vec![self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]), {
                                                                        let source_ml_source1_string_argument80 = if negated_3 {
                                                                            value_2.neg()
                                                                        } else {
                                                                            value_2.clone()
                                                                        };
                                                                        self::ml_string::Text::from_text(&source_ml_source1_string_argument80.to_string())
                                                                    }]
                                                                }
                                                                self::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Result::MlTemporaryReadRustSourceConstantIntegerLiteralLoop27Return(source_ml_temporary_result27) => {
                                                                    source_ml_temporary_result27
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_constant_initializer_boundary(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    first: f64,
) -> f64 {
    if ((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(first)))
        || (first < 0f64))
        || (first >= (values.len() as f64))
    {
        -1f64
    } else {
        {
            let depth = 0f64;
            {
                let at = first;
                {
                    let source_ml_source1_s81 = self::source_ml_temporary_read_rust_source_constant_initializer_boundary_loop29(kinds, values, depth, at);
                    match source_ml_source1_s81 {
                        self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Result::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Done => {
                            -1f64
                        }
                        self::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Result::MlTemporaryReadRustSourceConstantInitializerBoundaryLoop29Return(source_ml_temporary_result29) => {
                            source_ml_temporary_result29
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn constant_operation(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if (*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 106, 111, 105, 110, 82, 117, 115, 116, 80, 114, 105, 109, 105, 116, 105,
            118, 101, 66, 105, 116, 115,
        ]))
    {
        self::ml_string::Text::from_units(vec![98, 105, 116, 97, 110, 100])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![
            100, 105, 115, 106, 111, 105, 110, 82, 117, 115, 116, 80, 114, 105, 109, 105, 116, 105,
            118, 101, 66, 105, 116, 115,
        ]))
    {
        self::ml_string::Text::from_units(vec![98, 105, 116, 111, 114])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![
            101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 82, 117, 115, 116, 80, 114,
            105, 109, 105, 116, 105, 118, 101, 66, 105, 116, 115,
        ]))
    {
        self::ml_string::Text::from_units(vec![98, 105, 116, 120, 111, 114])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 82, 117, 115, 116, 80, 114, 105, 109,
            105, 116, 105, 118, 101, 66, 105, 116, 115,
        ]))
    {
        self::ml_string::Text::from_units(vec![98, 105, 116, 110, 111, 116])
    } else if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 82, 117, 115, 116, 80, 114, 105, 109, 105, 116, 105, 118, 101,
            66, 105, 116, 115, 76, 101, 102, 116,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 97, 100, 67, 104, 101, 99, 107, 101, 100, 82, 117, 115, 116, 76, 101,
                102, 116, 83, 104, 105, 102, 116,
            ])))
    {
        self::ml_string::Text::from_units(vec![115, 104, 108])
    } else if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            115, 104, 105, 102, 116, 82, 117, 115, 116, 80, 114, 105, 109, 105, 116, 105, 118, 101,
            66, 105, 116, 115, 82, 105, 103, 104, 116,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 97, 100, 67, 104, 101, 99, 107, 101, 100, 82, 117, 115, 116, 82, 105,
                103, 104, 116, 83, 104, 105, 102, 116,
            ])))
    {
        self::ml_string::Text::from_units(vec![115, 104, 114])
    } else {
        operation.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn constant_row(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    rows: f64,
    owner: &self::ml_string::Text,
) -> f64 {
    if rows < 0f64 {
        -1f64
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source1_s82 = self::source_ml_temporary_constant_row_loop30(
                    texts, starts, counts, labels, targets, rows, owner, at,
                );
                match source_ml_source1_s82 {
                    self::MlTemporaryConstantRowLoop30Result::MlTemporaryConstantRowLoop30Done => {
                        -1f64
                    }
                    self::MlTemporaryConstantRowLoop30Result::MlTemporaryConstantRowLoop30Return(source_ml_temporary_result30) => {
                        source_ml_temporary_result30
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn constant_type_owner(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    fallback: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let explicit = self::constant_text(
            texts,
            starts,
            counts,
            labels,
            targets,
            node,
            &(self::ml_string::Text::from_units(vec![
                115, 111, 117, 114, 99, 101, 73, 110, 116, 101, 103, 101, 114, 79, 119, 110, 101,
                114,
            ])),
        );
        {
            let type_ = self::constant_field(
                starts,
                counts,
                labels,
                targets,
                node,
                &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
            );
            {
                let type_3 = if type_ < 0f64 {
                    {
                        self::constant_field(
                            starts,
                            counts,
                            labels,
                            targets,
                            node,
                            &(self::ml_string::Text::from_units(vec![116, 111])),
                        )
                    }
                } else {
                    type_
                };
                {
                    let kind = self::constant_text(
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        type_3,
                        &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
                    );
                    if kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
                    } else if kind
                        == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))
                    {
                        {
                            let bits = self::constant_text(
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                type_3,
                                &(self::ml_string::Text::from_units(vec![98, 105, 116, 115])),
                            );
                            {
                                let signed = (self::constant_text(
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    type_3,
                                    &(self::ml_string::Text::from_units(vec![
                                        115, 105, 103, 110, 101, 100,
                                    ])),
                                )) == (self::ml_string::Text::from_units(vec![
                                    116, 114, 117, 101,
                                ]));
                                if explicit == (self::ml_string::Text::from_units(vec![])) {
                                    {
                                        let inherited = self::constant_bits(fallback);
                                        {
                                            let width = if bits
                                                == (self::ml_string::Text::from_units(vec![56]))
                                            {
                                                8f64
                                            } else if bits
                                                == (self::ml_string::Text::from_units(vec![49, 54]))
                                            {
                                                16f64
                                            } else if bits
                                                == (self::ml_string::Text::from_units(vec![51, 50]))
                                            {
                                                32f64
                                            } else if bits
                                                == (self::ml_string::Text::from_units(vec![54, 52]))
                                            {
                                                64f64
                                            } else if bits
                                                == (self::ml_string::Text::from_units(vec![
                                                    49, 50, 56,
                                                ]))
                                            {
                                                128f64
                                            } else {
                                                0f64
                                            };
                                            if ((inherited == width) && (width > 0f64))
                                                && ((fallback.starts_with(
                                                    &self::ml_string::Text::from_units(vec![105]),
                                                )) == signed)
                                            {
                                                fallback.clone()
                                            } else {
                                                (if signed {
                                                    self::ml_string::Text::from_units(vec![105])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![117])
                                                })
                                                .concat(&bits)
                                            }
                                        }
                                    }
                                } else {
                                    if (self::read_rust_constant_primitive_alias_owner(
                                        &(kind),
                                        &(self::ml_string::Text::from_units(vec![
                                            112, 114, 105, 109, 105, 116, 105, 118, 101,
                                        ])),
                                        &(explicit),
                                        0f64,
                                        0f64,
                                        &(bits),
                                        signed,
                                    )) == (self::ml_string::Text::from_units(vec![]))
                                    {
                                        self::ml_string::Text::from_units(vec![
                                            64, 105, 110, 118, 97, 108, 105, 100,
                                        ])
                                    } else {
                                        explicit.clone()
                                    }
                                }
                            }
                        }
                    } else {
                        fallback.clone()
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
pub fn constant_hint(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    rows: f64,
    node: f64,
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    depth: f64,
) -> self::ml_string::Text {
    if (node < 0f64) || (depth > 256f64) {
        self::ml_string::Text::from_units(vec![])
    } else {
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
                let typed = self::constant_type_owner(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![])),
                );
                {
                    let ml_library_inequality_3 =
                        typed != (self::ml_string::Text::from_units(vec![]));
                    if ml_library_inequality_3 {
                        typed.clone()
                    } else if kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
                    } else if kind == (self::ml_string::Text::from_units(vec![115, 116, 114])) {
                        self::ml_string::Text::from_units(vec![115, 116, 114])
                    } else if ((kind
                        == (self::ml_string::Text::from_units(vec![110, 97, 109, 101])))
                        || (kind == (self::ml_string::Text::from_units(vec![118, 97, 114]))))
                        || (kind == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))
                    {
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
                                let head =
                                    self::constant_element(starts, counts, targets, path, 0f64);
                                {
                                    let name = if kind
                                        == (self::ml_string::Text::from_units(vec![118, 97, 114]))
                                    {
                                        self::constant_text(
                                            texts,
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            node,
                                            &(self::ml_string::Text::from_units(vec![
                                                110, 97, 109, 101,
                                            ])),
                                        )
                                    } else if head < 0f64 {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        self::read_required_text_fact(texts, head)
                                    };
                                    {
                                        let bound = if (kind
                                            == (self::ml_string::Text::from_units(vec![
                                                118, 97, 114,
                                            ])))
                                            || ((path >= 0f64)
                                                && ((self::read_required_number_fact(
                                                    counts, path,
                                                )) == 1f64))
                                        {
                                            self::ml_array::index_of(
                                                names,
                                                &name,
                                                f64::INFINITY,
                                                true,
                                                |a, b| a == b,
                                            )
                                        } else {
                                            -1f64
                                        };
                                        if bound >= 0f64 {
                                            self::read_required_text_fact(
                                                values,
                                                ((bound * 3f64) + 1f64),
                                            )
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
                                                        115, 111, 117, 114, 99, 101, 67, 111, 110,
                                                        115, 116, 97, 110, 116, 79, 119, 110, 101,
                                                        114,
                                                    ])),
                                                );
                                                {
                                                    let row = self::constant_row(
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        rows,
                                                        &({
                                                            let ml_library_inequality_2 = reference != (self::ml_string::Text::from_units(vec![])) ;
                                                            if ml_library_inequality_2 {
                                                                reference.clone()
                                                            } else if kind == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])) {
                                                                    self::constant_text(texts, starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![102, 110])))
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                        }),
                                                    );
                                                    self::constant_text(
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        row,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            100, 111, 109, 97, 105, 110,
                                                        ])),
                                                    )
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if kind
                        == (self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121]))
                    {
                        self::constant_hint(
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
                                &(self::ml_string::Text::from_units(vec![97, 114, 103])),
                            ),
                            names,
                            values,
                            (depth + 1f64),
                        )
                    } else if kind
                        == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121]))
                    {
                        {
                            let left = self::constant_hint(
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
                                names,
                                values,
                                (depth + 1f64),
                            );
                            {
                                let ml_library_inequality_1 =
                                    left != (self::ml_string::Text::from_units(vec![]));
                                if ml_library_inequality_1 {
                                    left.clone()
                                } else {
                                    self::constant_hint(
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
                                            &(self::ml_string::Text::from_units(vec![
                                                114, 105, 103, 104, 116,
                                            ])),
                                        ),
                                        names,
                                        values,
                                        (depth + 1f64),
                                    )
                                }
                            }
                        }
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    }
}
