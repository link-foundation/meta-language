// Generated from complete js/src/translation/rust-primitive-default-rules.js. Do not edit by hand.
use super::accept_rust_integer_bit_method;
use super::accept_rust_primitive_selection_receiver;
use super::ml_array;
use super::read_rust_checked_integer_arity;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_success(method: &str, signed: bool) -> Vec<String> {
    if (method == "checked_shl") || (method == "checked_shr") {
        vec![
            String::from("operand"),
            String::from("right"),
            if method == "checked_shl" {
                String::from("shift left")
            } else {
                String::from("shift right")
            },
        ]
    } else if method == "checked_neg" {
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
pub fn accept_rust_primitive_from_method(owner: &str, method: &str) -> bool {
    (method == "from")
        && self::ml_array::includes(
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
                String::from("f64"),
            ],
            &owner.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_from_owner(
    segments: &[String],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> String {
    if !ordinary || shadowed {
        String::new()
    } else if ((((segments.len() as f64) == 2f64) && !absolute) && {
        let source_ml_source1_comparison_operand28 =
            self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand29 = String::from("f64");
            match (
                source_ml_source1_comparison_operand28,
                source_ml_source1_comparison_operand29,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || (((((segments.len() as f64) == 4f64)
        && ({
            let source_ml_source1_comparison_operand30 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand31 = String::from("std");
                match (
                    source_ml_source1_comparison_operand30,
                    source_ml_source1_comparison_operand31,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } || {
            let source_ml_source1_comparison_operand32 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand33 = String::from("core");
                match (
                    source_ml_source1_comparison_operand32,
                    source_ml_source1_comparison_operand33,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }))
        && {
            let source_ml_source1_comparison_operand34 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(1f64));
            {
                let source_ml_source1_comparison_operand35 = String::from("primitive");
                match (
                    source_ml_source1_comparison_operand34,
                    source_ml_source1_comparison_operand35,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source1_comparison_operand36 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(2f64));
            {
                let source_ml_source1_comparison_operand37 = String::from("f64");
                match (
                    source_ml_source1_comparison_operand36,
                    source_ml_source1_comparison_operand37,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
    {
        String::from("f64")
    } else {
        self::read_rust_primitive_associated_owner(segments, ordinary, shadowed, absolute)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_from_literal_owner(form: &str, typed: bool, kind: &str) -> String {
    if (form != "numeral") && (form != "negative numeral") {
        String::new()
    } else if kind == "float" {
        String::from("f64")
    } else if typed {
        String::new()
    } else {
        String::from("i32")
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
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((((((((kind == "float") && ((bits).is_empty())) && !signed) && (owner == "primitive"))
        && (path == "f64"))
        && (children == 0f64))
        && proven)
        && (layers == 0f64))
        || self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_primitive_from(target: &str, source: &str) -> String {
    if !(self::accept_rust_primitive_from_method(target, "from"))
        || !(self::accept_rust_primitive_from_method(source, "from"))
    {
        String::new()
    } else if target == source {
        String::from("identity")
    } else if ((source == "bool") && (target != "bool")) && (target != "f64") {
        String::from("boolean integer")
    } else if (target == "f64")
        && self::ml_array::includes(
            &vec![
                String::from("i8"),
                String::from("i16"),
                String::from("i32"),
                String::from("u8"),
                String::from("u16"),
                String::from("u32"),
            ],
            &source.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )
    {
        String::from("floating exact")
    } else {
        {
            let admitted = ((((((((source == "u8")
                && self::ml_array::includes(
                    &vec![
                        String::from("u16"),
                        String::from("u32"),
                        String::from("u64"),
                        String::from("u128"),
                        String::from("usize"),
                        String::from("i16"),
                        String::from("i32"),
                        String::from("i64"),
                        String::from("i128"),
                        String::from("isize"),
                    ],
                    &target.to_owned(),
                    0f64,
                    false,
                    |a, b| a == b,
                ))
                || ((source == "u16")
                    && self::ml_array::includes(
                        &vec![
                            String::from("u32"),
                            String::from("u64"),
                            String::from("u128"),
                            String::from("usize"),
                            String::from("i32"),
                            String::from("i64"),
                            String::from("i128"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "u32")
                    && self::ml_array::includes(
                        &vec![
                            String::from("u64"),
                            String::from("u128"),
                            String::from("i64"),
                            String::from("i128"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "u64")
                    && self::ml_array::includes(
                        &vec![String::from("u128"), String::from("i128")],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i8")
                    && self::ml_array::includes(
                        &vec![
                            String::from("i16"),
                            String::from("i32"),
                            String::from("i64"),
                            String::from("i128"),
                            String::from("isize"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i16")
                    && self::ml_array::includes(
                        &vec![
                            String::from("i32"),
                            String::from("i64"),
                            String::from("i128"),
                            String::from("isize"),
                        ],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i32")
                    && self::ml_array::includes(
                        &vec![String::from("i64"), String::from("i128")],
                        &target.to_owned(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )))
                || ((source == "i64") && (target == "i128"));
            if admitted {
                String::from("integer exact")
            } else {
                String::new()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_default_owner(
    segments: &[String],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> String {
    if {
        let source_ml_source1_comparison_operand38 = self::ml_array::optional_at(
            segments,
            self::ml_array::number_index((segments.len() as f64) - 1f64),
        );
        {
            let source_ml_source1_comparison_operand39 = String::from("default");
            match (
                source_ml_source1_comparison_operand38,
                source_ml_source1_comparison_operand39,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    } {
        self::read_rust_primitive_from_owner(segments, ordinary, shadowed, absolute)
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_default_term(owner: &str) -> String {
    if owner == "bool" {
        String::from("false")
    } else if owner == "f64" {
        String::from("positive floating zero")
    } else if self::ml_array::includes(
        &vec![
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
    ) {
        String::from("integer zero")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_default_import(path: &str, ordinary: bool, shadowed: bool) -> bool {
    (ordinary && !shadowed)
        && ((path == "std::default::Default") || (path == "core::default::Default"))
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
                    let source_ml_source1_concat_value40 = plan;
                    {
                        let source_ml_source1_concat_value41 = vec![
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
                                source_ml_source1_concat_value40,
                            ),
                            source_ml_source1_concat_value41,
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
