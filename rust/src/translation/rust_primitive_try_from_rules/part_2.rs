// Generated from complete js/src/translation/rust-primitive-try-from-rules.js. Do not edit by hand.
use super::accept_rust_integer_bit_method;
use super::accept_rust_primitive_selection_receiver;
use super::accept_rust_wrapping_shift_method;
use super::ml_array;
use super::read_rust_checked_integer_arity;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_condition(method: &str, signed: bool) -> Vec<String> {
    if (method == "checked_shl") || (method == "checked_shr") {
        vec![
            String::from("right"),
            String::from("width"),
            String::from("lt"),
        ]
    } else if method == "checked_neg" {
        if signed {
            vec![
                String::from("operand"),
                String::from("minimum"),
                String::from("ne"),
            ]
        } else {
            vec![
                String::from("operand"),
                String::from("zero"),
                String::from("eq"),
            ]
        }
    } else if method == "checked_abs" {
        if signed {
            vec![
                String::from("operand"),
                String::from("minimum"),
                String::from("ne"),
            ]
        } else {
            Vec::<String>::new()
        }
    } else if method == "checked_add" {
        if signed {
            vec![
                String::from("right"),
                String::from("zero"),
                String::from("gt"),
                String::from("operand"),
                String::from("maximum"),
                String::from("right"),
                String::from("sub"),
                String::from("le"),
                String::from("right"),
                String::from("zero"),
                String::from("lt"),
                String::from("operand"),
                String::from("minimum"),
                String::from("right"),
                String::from("sub"),
                String::from("ge"),
                String::from("true"),
                String::from("if"),
                String::from("if"),
            ]
        } else {
            vec![
                String::from("operand"),
                String::from("maximum"),
                String::from("right"),
                String::from("sub"),
                String::from("le"),
            ]
        }
    } else if method == "checked_sub" {
        if signed {
            vec![
                String::from("right"),
                String::from("zero"),
                String::from("lt"),
                String::from("operand"),
                String::from("maximum"),
                String::from("right"),
                String::from("add"),
                String::from("le"),
                String::from("right"),
                String::from("zero"),
                String::from("gt"),
                String::from("operand"),
                String::from("minimum"),
                String::from("right"),
                String::from("add"),
                String::from("ge"),
                String::from("true"),
                String::from("if"),
                String::from("if"),
            ]
        } else {
            vec![
                String::from("operand"),
                String::from("right"),
                String::from("ge"),
            ]
        }
    } else if method == "checked_mul" {
        {
            let positive_positive = vec![
                String::from("operand"),
                String::from("maximum"),
                String::from("right"),
                String::from("div"),
                String::from("le"),
            ];
            if signed {
                {
                    let positive_negative = vec![
                        String::from("right"),
                        String::from("minimum"),
                        String::from("operand"),
                        String::from("div"),
                        String::from("ge"),
                    ];
                    {
                        let negative_positive = vec![
                            String::from("operand"),
                            String::from("minimum"),
                            String::from("right"),
                            String::from("div"),
                            String::from("ge"),
                        ];
                        {
                            let negative_negative = vec![
                                String::from("operand"),
                                String::from("maximum"),
                                String::from("right"),
                                String::from("div"),
                                String::from("ge"),
                            ];
                            {
                                let signs = {
                                    let source_ml_source1_concat_value23 = vec![
                                        String::from("operand"),
                                        String::from("zero"),
                                        String::from("gt"),
                                        String::from("right"),
                                        String::from("zero"),
                                        String::from("gt"),
                                    ];
                                    {
                                        let source_ml_source1_concat_value24 = positive_positive;
                                        {
                                            let source_ml_source1_concat_value25 =
                                                positive_negative;
                                            {
                                                let source_ml_source1_concat_value26 = vec![
                                                    String::from("if"),
                                                    String::from("right"),
                                                    String::from("zero"),
                                                    String::from("gt"),
                                                ];
                                                {
                                                    let source_ml_source1_concat_value27 =
                                                        negative_positive;
                                                    {
                                                        let source_ml_source1_concat_value28 =
                                                            negative_negative;
                                                        {
                                                            let source_ml_source1_concat_value29 = vec![
                                                                String::from("if"),
                                                                String::from("if"),
                                                            ];
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_source1_concat_value23), source_ml_source1_concat_value24), source_ml_source1_concat_value25), source_ml_source1_concat_value26), source_ml_source1_concat_value27), source_ml_source1_concat_value28), source_ml_source1_concat_value29)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                {
                                    let source_ml_source1_concat_value30 = vec![
                                        String::from("operand"),
                                        String::from("zero"),
                                        String::from("eq"),
                                        String::from("true"),
                                        String::from("right"),
                                        String::from("zero"),
                                        String::from("eq"),
                                        String::from("true"),
                                    ];
                                    {
                                        let source_ml_source1_concat_value31 = signs;
                                        {
                                            let source_ml_source1_concat_value32 =
                                                vec![String::from("if"), String::from("if")];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<String>::new(),
                                                        source_ml_source1_concat_value30,
                                                    ),
                                                    source_ml_source1_concat_value31,
                                                ),
                                                source_ml_source1_concat_value32,
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                {
                    let source_ml_source1_concat_value20 = vec![
                        String::from("operand"),
                        String::from("zero"),
                        String::from("eq"),
                        String::from("true"),
                        String::from("right"),
                        String::from("zero"),
                        String::from("eq"),
                        String::from("true"),
                    ];
                    {
                        let source_ml_source1_concat_value21 = positive_positive;
                        {
                            let source_ml_source1_concat_value22 =
                                vec![String::from("if"), String::from("if")];
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source1_concat_value20,
                                    ),
                                    source_ml_source1_concat_value21,
                                ),
                                source_ml_source1_concat_value22,
                            )
                        }
                    }
                }
            }
        }
    } else if (method == "checked_div") || (method == "checked_rem") {
        if signed {
            vec![
                String::from("right"),
                String::from("zero"),
                String::from("eq"),
                String::from("false"),
                String::from("operand"),
                String::from("minimum"),
                String::from("eq"),
                String::from("right"),
                String::from("negative unit"),
                String::from("ne"),
                String::from("true"),
                String::from("if"),
                String::from("if"),
            ]
        } else {
            vec![
                String::from("right"),
                String::from("zero"),
                String::from("ne"),
            ]
        }
    } else {
        Vec::<String>::new()
    }
}

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
                            let source_ml_source1_comparison_operand33 =
                                self::ml_array::optional_at(
                                    segments,
                                    self::ml_array::number_index(0f64),
                                );
                            {
                                let source_ml_source1_comparison_operand34 = String::from("std");
                                match (
                                    source_ml_source1_comparison_operand33,
                                    source_ml_source1_comparison_operand34,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        } || {
                            let source_ml_source1_comparison_operand35 =
                                self::ml_array::optional_at(
                                    segments,
                                    self::ml_array::number_index(0f64),
                                );
                            {
                                let source_ml_source1_comparison_operand36 = String::from("core");
                                match (
                                    source_ml_source1_comparison_operand35,
                                    source_ml_source1_comparison_operand36,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }))
                        && {
                            let source_ml_source1_comparison_operand37 =
                                self::ml_array::optional_at(
                                    segments,
                                    self::ml_array::number_index(1f64),
                                );
                            {
                                let source_ml_source1_comparison_operand38 =
                                    String::from("primitive");
                                match (
                                    source_ml_source1_comparison_operand37,
                                    source_ml_source1_comparison_operand38,
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
        } else if self::accept_rust_wrapping_shift_method(method) {
            2f64
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
        let source_ml_source1_comparison_operand39 =
            self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
        {
            let source_ml_source1_comparison_operand40 = String::from("f64");
            match (
                source_ml_source1_comparison_operand39,
                source_ml_source1_comparison_operand40,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || (((((segments.len() as f64) == 4f64)
        && ({
            let source_ml_source1_comparison_operand41 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand42 = String::from("std");
                match (
                    source_ml_source1_comparison_operand41,
                    source_ml_source1_comparison_operand42,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } || {
            let source_ml_source1_comparison_operand43 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand44 = String::from("core");
                match (
                    source_ml_source1_comparison_operand43,
                    source_ml_source1_comparison_operand44,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }))
        && {
            let source_ml_source1_comparison_operand45 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(1f64));
            {
                let source_ml_source1_comparison_operand46 = String::from("primitive");
                match (
                    source_ml_source1_comparison_operand45,
                    source_ml_source1_comparison_operand46,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_source1_comparison_operand47 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(2f64));
            {
                let source_ml_source1_comparison_operand48 = String::from("f64");
                match (
                    source_ml_source1_comparison_operand47,
                    source_ml_source1_comparison_operand48,
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
