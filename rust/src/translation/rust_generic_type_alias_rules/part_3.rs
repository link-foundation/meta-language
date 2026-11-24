// Generated from complete js/src/translation/rust-generic-type-alias-rules.js. Do not edit by hand.
use super::ml_array;
use super::ml_string;
use super::read_rust_primitive_integer_constant_owner;
use super::read_rust_primitive_owner_path;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_primitive(
    kind: &self::ml_string::Text,
    path: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
) -> bool {
    if (*path) == (self::ml_string::Text::from_units(vec![102, 54, 52])) {
        (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
    } else if (*path) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
        (*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))
    } else if (*path) == (self::ml_string::Text::from_units(vec![115, 116, 114])) {
        (*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![56]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![56]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 49, 54])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 54]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 49, 54])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 54]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 51, 50])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![51, 50]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 51, 50])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![51, 50]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 54, 52])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 54, 52])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 49, 50, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 50, 56]))))
            && (signed == (true))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 49, 50, 56])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![49, 50, 56]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (false))
    } else if (*path) == (self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101])) {
        (((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
            && ((*bits) == (self::ml_string::Text::from_units(vec![54, 52]))))
            && (signed == (true))
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_call_role(
    path: &self::ml_string::Text,
    ordinary: bool,
    shadowed: bool,
) -> self::ml_string::Text {
    if ordinary && !shadowed {
        if (*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 109, 112, 58, 58, 109, 105, 110,
            ]))
        {
            self::ml_string::Text::from_units(vec![109, 105, 110])
        } else if (*path)
            == (self::ml_string::Text::from_units(vec![
                115, 116, 100, 58, 58, 99, 109, 112, 58, 58, 109, 97, 120,
            ]))
        {
            self::ml_string::Text::from_units(vec![109, 97, 120])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_comparison(
    method: &self::ml_string::Text,
    ordinary: bool,
) -> self::ml_string::Text {
    if ordinary {
        if (*method) == (self::ml_string::Text::from_units(vec![109, 105, 110])) {
            self::ml_string::Text::from_units(vec![108, 101])
        } else if (*method) == (self::ml_string::Text::from_units(vec![109, 97, 120])) {
            self::ml_string::Text::from_units(vec![103, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_condition(
    method: &self::ml_string::Text,
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
        if (*method) == (self::ml_string::Text::from_units(vec![109, 105, 110])) {
            self::ml_string::Text::from_units(vec![110, 111, 116])
        } else if (*method) == (self::ml_string::Text::from_units(vec![109, 97, 120])) {
            self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else if (*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])) {
        self::read_rust_primitive_selection_comparison(method, true)
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_selection_receiver(
    kind: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    path: &self::ml_string::Text,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (((((((*kind) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))))
        && ((*owner)
            == (self::ml_string::Text::from_units(vec![
                112, 114, 105, 109, 105, 116, 105, 118, 101,
            ]))))
        && (children == 0f64))
        && proven)
        && (layers == 0f64))
        && self::accept_rust_source_signature_primitive(kind, path, bits, signed)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_selection_argument(
    expected: &self::ml_string::Text,
    bits: &self::ml_string::Text,
    signed: bool,
    owner: &self::ml_string::Text,
    actual: &self::ml_string::Text,
    actual_bits: &self::ml_string::Text,
    actual_signed: bool,
    actual_owner: &self::ml_string::Text,
    layers: f64,
    proven: bool,
    known: bool,
    literal: bool,
) -> bool {
    if layers == 0f64 {
        if ((literal
            && ((*expected) == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
            && ((*actual) == (self::ml_string::Text::from_units(vec![]))))
            && ((*actual_owner) == (self::ml_string::Text::from_units(vec![])))
        {
            true
        } else {
            ((((known && proven) && ((*expected) == (*actual))) && ((*bits) == (*actual_bits)))
                && (signed == actual_signed))
                && ((*owner) == (*actual_owner))
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_from_method(
    owner: &self::ml_string::Text,
    method: &self::ml_string::Text,
) -> bool {
    ((*method) == (self::ml_string::Text::from_units(vec![102, 114, 111, 109])))
        && self::ml_array::includes(
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
                self::ml_string::Text::from_units(vec![102, 54, 52]),
            ],
            owner,
            0f64,
            false,
            |a, b| a == b,
        )
}

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
        let source_ml_source2_comparison_operand7 =
            self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
        {
            let source_ml_source2_comparison_operand8 =
                self::ml_string::Text::from_units(vec![102, 54, 52]);
            match (
                source_ml_source2_comparison_operand7,
                source_ml_source2_comparison_operand8,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || (((((segments.len() as f64) == 4f64)
        && ({
            let source_ml_source2_comparison_operand9 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand10 =
                    self::ml_string::Text::from_units(vec![115, 116, 100]);
                match (
                    source_ml_source2_comparison_operand9,
                    source_ml_source2_comparison_operand10,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } || {
            let source_ml_source2_comparison_operand11 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source2_comparison_operand12 =
                    self::ml_string::Text::from_units(vec![99, 111, 114, 101]);
                match (
                    source_ml_source2_comparison_operand11,
                    source_ml_source2_comparison_operand12,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }))
        && {
            let source_ml_source2_comparison_operand13 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(1f64));
            {
                let source_ml_source2_comparison_operand14 =
                    self::ml_string::Text::from_units(vec![
                        112, 114, 105, 109, 105, 116, 105, 118, 101,
                    ]);
                match (
                    source_ml_source2_comparison_operand13,
                    source_ml_source2_comparison_operand14,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source2_comparison_operand15 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(2f64));
            {
                let source_ml_source2_comparison_operand16 =
                    self::ml_string::Text::from_units(vec![102, 54, 52]);
                match (
                    source_ml_source2_comparison_operand15,
                    source_ml_source2_comparison_operand16,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
    {
        self::ml_string::Text::from_units(vec![102, 54, 52])
    } else {
        self::read_rust_primitive_owner_path(segments, ordinary, shadowed, absolute)
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
    if ((((*method) == (self::ml_string::Text::from_units(vec![105, 115, 95, 111, 107])))
        || ((*method) == (self::ml_string::Text::from_units(vec![105, 115, 95, 101, 114, 114]))))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 119, 114, 97, 112, 95, 111, 114, 95, 100, 101, 102, 97, 117, 108, 116,
            ]))))
        || ((*method) == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112])))
    {
        0f64
    } else if (((*method)
        == (self::ml_string::Text::from_units(vec![117, 110, 119, 114, 97, 112, 95, 111, 114])))
        || ((*method) == (self::ml_string::Text::from_units(vec![101, 120, 112, 101, 99, 116]))))
        || ((*method)
            == (self::ml_string::Text::from_units(vec![105, 115, 95, 111, 107, 95, 97, 110, 100])))
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
