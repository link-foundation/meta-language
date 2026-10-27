// Generated from complete js/src/translation/rust-primitive-try-from-rules.js. Do not edit by hand.
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
pub fn read_rust_primitive_try_from_owner(
    segments: &[String],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> String {
    self::read_rust_primitive_integer_constant_owner(segments, ordinary, shadowed, absolute)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_primitive_try_from_import(path: &str, ordinary: bool, shadowed: bool) -> bool {
    (ordinary && !shadowed)
        && ((path == "std::convert::TryFrom") || (path == "core::convert::TryFrom"))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_primitive_try_from_source(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((kind == "fixed") && self::accept_rust_primitive_from_method(path, "from"))
        && self::accept_rust_primitive_from_source(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_try_from_consumer_arity(method: &str) -> f64 {
    if ((method == "is_ok") || (method == "is_err")) || (method == "unwrap_or_default") {
        0f64
    } else if method == "unwrap_or" {
        1f64
    } else if method == "map_or" {
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
pub fn plan_rust_primitive_try_from_consumer(method: &str, guards: f64) -> Vec<String> {
    if ((self::read_rust_primitive_try_from_consumer_arity(method)) < 0f64)
        || !(self::ml_array::includes(
            &vec![0f64, 1f64, 2f64, 3f64],
            &guards,
            0f64,
            false,
            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
        ))
    {
        Vec::<String>::new()
    } else {
        {
            let lower = vec![
                String::from("input"),
                String::from("minimum"),
                String::from("ge"),
            ];
            {
                let upper = vec![
                    String::from("input"),
                    String::from("maximum"),
                    String::from("le"),
                ];
                {
                    let condition = if guards == 0f64 {
                        vec![String::from("true")]
                    } else if guards == 1f64 {
                        lower
                    } else if guards == 2f64 {
                        upper
                    } else {
                        {
                            let source_ml_source1_concat_value38 = lower;
                            {
                                let source_ml_source1_concat_value39 = upper;
                                {
                                    let source_ml_source1_concat_value40 =
                                        vec![String::from("and")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_source1_concat_value38,
                                            ),
                                            source_ml_source1_concat_value39,
                                        ),
                                        source_ml_source1_concat_value40,
                                    )
                                }
                            }
                        }
                    };
                    if method == "is_ok" {
                        condition
                    } else if method == "is_err" {
                        {
                            let source_ml_source1_concat_value41 = condition;
                            {
                                let source_ml_source1_concat_value42 = vec![String::from("not")];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source1_concat_value41,
                                    ),
                                    source_ml_source1_concat_value42,
                                )
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_concat_value43 = condition;
                            {
                                let source_ml_source1_concat_value44 = vec![
                                    String::from("success"),
                                    if method == "unwrap_or_default" {
                                        String::from("default")
                                    } else {
                                        String::from("fallback")
                                    },
                                    String::from("if"),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source1_concat_value43,
                                    ),
                                    source_ml_source1_concat_value44,
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
    method: &str,
    target: &str,
    mapper: &str,
) -> String {
    if (method == "is_ok") || (method == "is_err") {
        String::from("bool")
    } else if (method == "unwrap_or") || (method == "unwrap_or_default") {
        target.to_owned()
    } else if method == "map_or" {
        mapper.to_owned()
    } else {
        String::new()
    }
}
