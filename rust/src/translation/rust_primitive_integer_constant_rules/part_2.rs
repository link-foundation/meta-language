// Generated from complete js/src/translation/rust-primitive-integer-constant-rules.js. Do not edit by hand.
use super::accept_rust_integer_bit_method;
use super::accept_rust_primitive_selection_receiver;
use super::ml_array;
use super::read_rust_checked_integer_arity;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_success(method: &str, signed: bool) -> Vec<String> {
    if method == "checked_neg" {
        if signed {
            vec![String::from("operand"), String::from("neg")]
        } else {
            vec![String::from("zero")]
        }
    } else if method == "checked_abs" {
        if signed {
            vec![
                String::from("operand"),
                String::from("zero"),
                String::from("lt"),
                String::from("operand"),
                String::from("neg"),
                String::from("operand"),
                String::from("if"),
            ]
        } else {
            Vec::<String>::new()
        }
    } else {
        {
            let operation = if method == "checked_add" {
                String::from("add")
            } else if method == "checked_sub" {
                String::from("sub")
            } else if method == "checked_mul" {
                String::from("mul")
            } else if method == "checked_div" {
                String::from("div")
            } else if method == "checked_rem" {
                String::from("rem")
            } else {
                String::new()
            };
            if operation.is_empty() {
                Vec::<String>::new()
            } else {
                vec![String::from("operand"), String::from("right"), operation]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_consumer(consumer: &str) -> Vec<String> {
    if consumer == "is_some" {
        vec![String::from("condition")]
    } else if consumer == "is_none" {
        vec![String::from("condition"), String::from("not")]
    } else if consumer == "unwrap_or_default" {
        vec![
            String::from("condition"),
            String::from("success"),
            String::from("default"),
            String::from("if"),
        ]
    } else if (consumer == "unwrap_or") || (consumer == "map_or") {
        vec![
            String::from("condition"),
            String::from("success"),
            String::from("fallback"),
            String::from("if"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_result_owner(
    consumer: &str,
    receiver: &str,
    mapper: &str,
) -> String {
    if (consumer == "is_some") || (consumer == "is_none") {
        String::from("bool")
    } else if (consumer == "unwrap_or") || (consumer == "unwrap_or_default") {
        receiver.to_owned()
    } else if consumer == "map_or" {
        mapper.to_owned()
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_associated_owner(
    segments: &[String],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> String {
    if !ordinary || shadowed {
        String::new()
    } else {
        {
            let owner = String::new();
            {
                let owner_4 = if ((segments.len() as f64) == 2f64) && !absolute {
                    {
                        (self::ml_array::optional_at(segments, self::ml_array::number_index(0f64)))
                            .unwrap_or_else(|| String::new())
                    }
                } else {
                    if (((segments.len() as f64) == 4f64)
                        && ({
                            let source_ml_source1_comparison_operand22 =
                                self::ml_array::optional_at(
                                    segments,
                                    self::ml_array::number_index(0f64),
                                );
                            {
                                let source_ml_source1_comparison_operand23 = String::from("std");
                                match (
                                    source_ml_source1_comparison_operand22,
                                    source_ml_source1_comparison_operand23,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        } || {
                            let source_ml_source1_comparison_operand24 =
                                self::ml_array::optional_at(
                                    segments,
                                    self::ml_array::number_index(0f64),
                                );
                            {
                                let source_ml_source1_comparison_operand25 = String::from("core");
                                match (
                                    source_ml_source1_comparison_operand24,
                                    source_ml_source1_comparison_operand25,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }))
                        && {
                            let source_ml_source1_comparison_operand26 =
                                self::ml_array::optional_at(
                                    segments,
                                    self::ml_array::number_index(1f64),
                                );
                            {
                                let source_ml_source1_comparison_operand27 =
                                    String::from("primitive");
                                match (
                                    source_ml_source1_comparison_operand26,
                                    source_ml_source1_comparison_operand27,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }
                    {
                        {
                            (self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(2f64),
                            ))
                            .unwrap_or_else(|| String::new())
                        }
                    } else {
                        owner
                    }
                };
                if self::ml_array::includes(
                    &vec![
                        String::from("bool"),
                        String::from("i8"),
                        String::from("u8"),
                        String::from("i16"),
                        String::from("u16"),
                        String::from("i32"),
                        String::from("u32"),
                        String::from("i64"),
                        String::from("u64"),
                        String::from("i128"),
                        String::from("u128"),
                        String::from("isize"),
                        String::from("usize"),
                    ],
                    &owner_4.clone(),
                    0f64,
                    false,
                    |a, b| a == b,
                ) {
                    owner_4.clone()
                } else {
                    String::new()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_associated_arity(owner: &str, method: &str) -> f64 {
    if (self::ml_array::includes(
        &vec![
            String::from("bool"),
            String::from("i8"),
            String::from("u8"),
            String::from("i16"),
            String::from("u16"),
            String::from("i32"),
            String::from("u32"),
            String::from("i64"),
            String::from("u64"),
            String::from("i128"),
            String::from("u128"),
            String::from("isize"),
            String::from("usize"),
        ],
        &owner.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )) {
        if (method == "min") || (method == "max") {
            2f64
        } else if owner == "bool" {
            -1f64
        } else if self::accept_rust_integer_bit_method(method) {
            1f64
        } else {
            {
                let checked = self::read_rust_checked_integer_arity(method, true);
                if (checked >= 0f64) && ((method != "checked_abs") || owner.starts_with("i")) {
                    checked + 1f64
                } else if self::ml_array::includes(
                    &vec![
                        String::from("saturating_add"),
                        String::from("saturating_sub"),
                        String::from("saturating_mul"),
                        String::from("wrapping_add"),
                        String::from("wrapping_sub"),
                    ],
                    &method.to_owned(),
                    0f64,
                    false,
                    |a, b| a == b,
                ) {
                    2f64
                } else if method == "wrapping_neg" {
                    1f64
                } else if (owner.starts_with("i")) {
                    if self::ml_array::includes(
                        &vec![
                            String::from("is_negative"),
                            String::from("is_positive"),
                            String::from("signum"),
                            String::from("saturating_neg"),
                            String::from("saturating_abs"),
                            String::from("wrapping_abs"),
                        ],
                        &method.to_owned(),
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
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_associated_literal(expected: &str, form: &str, typed: bool) -> bool {
    (((!(expected).is_empty()) && (expected != "bool")) && !typed)
        && ((form == "numeral") || (form == "negative numeral"))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_associated_receiver(
    expected: &str,
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (expected == path)
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_integer_constant_owner(
    segments: &[String],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> String {
    {
        let owner =
            self::read_rust_primitive_associated_owner(segments, ordinary, shadowed, absolute);
        if owner == "bool" {
            String::new()
        } else {
            owner
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_integer_constant_term(name: &str) -> String {
    if name == "MIN" {
        String::from("minimum")
    } else if name == "MAX" {
        String::from("maximum")
    } else if name == "BITS" {
        String::from("width")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_integer_constant_result_owner(owner: &str, term: &str) -> String {
    if term == "width" {
        String::from("u32")
    } else if (term == "minimum") || (term == "maximum") {
        owner.to_owned()
    } else {
        String::new()
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
                    let source_ml_source1_concat_value28 = plan;
                    {
                        let source_ml_source1_concat_value29 = vec![
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
                                source_ml_source1_concat_value28,
                            ),
                            source_ml_source1_concat_value29,
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
                                    let source_ml_source1_concat_value30 = plan;
                                    {
                                        let source_ml_source1_concat_value31 = vec![
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
                                                source_ml_source1_concat_value30,
                                            ),
                                            source_ml_source1_concat_value31,
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
                                                let source_ml_source1_s40 = if reverse {
                                                    {
                                                        let plan_3 = {
                                                            let source_ml_source1_concat_value41 =
                                                                plan_2;
                                                            {
                                                                let source_ml_source1_concat_value42 = vec![
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
                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value41), source_ml_source1_concat_value42)
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
                                                                let source_ml_source1_concat_value43 =
                                                                    plan_2;
                                                                {
                                                                    let source_ml_source1_concat_value44 = vec![
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
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value43), source_ml_source1_concat_value44)
                                                                }
                                                            };
                                                            {
                                                                let index_4 = index_2 + 2f64;
                                                                if trailing {
                                                                    {
                                                                        let next_flag = index_4;
                                                                        {
                                                                            let plan_5 = {
                                                                                let source_ml_source1_concat_value45 =
                                                                                    plan_4;
                                                                                {
                                                                                    let source_ml_source1_concat_value46 = vec![
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
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value45), source_ml_source1_concat_value46)
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
                                                                                let source_ml_source1_concat_value47 =
                                                                                    plan_4;
                                                                                {
                                                                                    let source_ml_source1_concat_value48 = vec![
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
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value47), source_ml_source1_concat_value48)
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
                                                match source_ml_source1_s40 {
                                                    self::MlTemporaryPlanRustIntegerBitsJoin4::MlTemporaryPlanRustIntegerBitsJoin4Next(plan_7, index_7, next_count_term_5, next_active_term_3) => {
                                                        {
                                                            let quotient = index_7;
                                                            {
                                                                let plan_8 = {
                                                                    let source_ml_source1_concat_value49 = plan_7;
                                                                    {
                                                                        let source_ml_source1_concat_value50 = vec![13f64, at, 3f64, 0f64, 0f64];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value49), source_ml_source1_concat_value50)
                                                                    }
                                                                };
                                                                {
                                                                    let index_8 = index_7 + 1f64;
                                                                    {
                                                                        let bindings_2 = {
                                                                            let source_ml_source1_concat_value51 = bindings;
                                                                            {
                                                                                let source_ml_source1_concat_value52 = vec![next_current, quotient, next_count, next_count_term_5];
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value51), source_ml_source1_concat_value52)
                                                                            }
                                                                        };
                                                                        {
                                                                            let bindings_4 = if trailing {
                                                                                {
                                                                                    {
                                                                                        let source_ml_source1_concat_value53 = bindings_2;
                                                                                        {
                                                                                            let source_ml_source1_concat_value54 = vec![next_active, next_active_term_3];
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value53), source_ml_source1_concat_value54)
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
