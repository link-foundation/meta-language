// Generated from complete js/src/translation/rust-primitive-associated-call-rules.js. Do not edit by hand.
/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_model(model: &str) -> bool {
    ((model).is_empty()) || (model == "Rust source function signature 1")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_wrapper(wrapper: &str) -> bool {
    (wrapper == "reference") || (wrapper == "box")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_owner_kind(kind: &str) -> bool {
    (((((kind == "primitive") || (kind == "declaration")) || (kind == "library"))
        || (kind == "tuple"))
        || (kind == "unit"))
        || (kind == "slice")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_catalog_owner(kind: &str) -> bool {
    (((kind == "data") || (kind == "module")) || (kind == "externalCrate")) || (kind == "ctor")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_signature_primitive(
    kind: &str,
    path: &str,
    bits: &str,
    signed: bool,
) -> bool {
    if path == "f64" {
        kind == "float"
    } else if path == "bool" {
        kind == "bool"
    } else if path == "str" {
        kind == "string"
    } else if path == "i8" {
        ((kind == "fixed") && (bits == "8")) && (signed == (true))
    } else if path == "u8" {
        ((kind == "fixed") && (bits == "8")) && (signed == (false))
    } else if path == "i16" {
        ((kind == "fixed") && (bits == "16")) && (signed == (true))
    } else if path == "u16" {
        ((kind == "fixed") && (bits == "16")) && (signed == (false))
    } else if path == "i32" {
        ((kind == "fixed") && (bits == "32")) && (signed == (true))
    } else if path == "u32" {
        ((kind == "fixed") && (bits == "32")) && (signed == (false))
    } else if path == "i64" {
        ((kind == "fixed") && (bits == "64")) && (signed == (true))
    } else if path == "u64" {
        ((kind == "fixed") && (bits == "64")) && (signed == (false))
    } else if path == "i128" {
        ((kind == "fixed") && (bits == "128")) && (signed == (true))
    } else if path == "u128" {
        ((kind == "fixed") && (bits == "128")) && (signed == (false))
    } else if path == "usize" {
        ((kind == "fixed") && (bits == "64")) && (signed == (false))
    } else if path == "isize" {
        ((kind == "fixed") && (bits == "64")) && (signed == (true))
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_call_role(
    path: &str,
    ordinary: bool,
    shadowed: bool,
) -> String {
    if ordinary && !shadowed {
        if path == "std::cmp::min" {
            String::from("min")
        } else if path == "std::cmp::max" {
            String::from("max")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_comparison(method: &str, ordinary: bool) -> String {
    if ordinary {
        if method == "min" {
            String::from("le")
        } else if method == "max" {
            String::from("gt")
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_selection_condition(method: &str, kind: &str) -> String {
    if kind == "bool" {
        if method == "min" {
            String::from("not")
        } else if method == "max" {
            String::from("identity")
        } else {
            String::new()
        }
    } else if kind == "fixed" {
        self::read_rust_primitive_selection_comparison(method, true)
    } else {
        String::new()
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
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((((((kind == "fixed") || (kind == "bool")) && (owner == "primitive")) && (children == 0f64))
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
    expected: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    actual: &str,
    actual_bits: &str,
    actual_signed: bool,
    actual_owner: &str,
    layers: f64,
    proven: bool,
    known: bool,
    literal: bool,
) -> bool {
    if layers == 0f64 {
        if ((literal && (expected == "fixed")) && ((actual).is_empty()))
            && ((actual_owner).is_empty())
        {
            true
        } else {
            ((((known && proven) && (expected == actual)) && (bits == actual_bits))
                && (signed == actual_signed))
                && (owner == actual_owner)
        }
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_arity(method: &str, ordinary: bool) -> f64 {
    if ordinary {
        if self::ml_array::includes(
            &vec![
                String::from("checked_add"),
                String::from("checked_sub"),
                String::from("checked_mul"),
                String::from("checked_div"),
                String::from("checked_rem"),
            ],
            &method.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ) {
            1f64
        } else if self::ml_array::includes(
            &vec![String::from("checked_neg"), String::from("checked_abs")],
            &method.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ) {
            0f64
        } else {
            -1f64
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_checked_integer_receiver(
    method: &str,
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    ((((self::read_rust_checked_integer_arity(method, true)) >= 0f64) && (kind == "fixed"))
        && ((method != "checked_abs") || signed))
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_consumer_arity(consumer: &str) -> f64 {
    if (consumer == "is_some") || (consumer == "is_none") {
        0f64
    } else if consumer == "unwrap_or" {
        1f64
    } else if consumer == "map_or" {
        2f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_checked_integer_condition(method: &str, signed: bool) -> Vec<String> {
    if method == "checked_neg" {
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
                                    let source_ml_concat_value4 = vec![
                                        String::from("operand"),
                                        String::from("zero"),
                                        String::from("gt"),
                                        String::from("right"),
                                        String::from("zero"),
                                        String::from("gt"),
                                    ];
                                    {
                                        let source_ml_concat_value5 = positive_positive;
                                        {
                                            let source_ml_concat_value6 = positive_negative;
                                            {
                                                let source_ml_concat_value7 = vec![
                                                    String::from("if"),
                                                    String::from("right"),
                                                    String::from("zero"),
                                                    String::from("gt"),
                                                ];
                                                {
                                                    let source_ml_concat_value8 = negative_positive;
                                                    {
                                                        let source_ml_concat_value9 =
                                                            negative_negative;
                                                        {
                                                            let source_ml_concat_value10 = vec![
                                                                String::from("if"),
                                                                String::from("if"),
                                                            ];
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<String>::new(), source_ml_concat_value4), source_ml_concat_value5), source_ml_concat_value6), source_ml_concat_value7), source_ml_concat_value8), source_ml_concat_value9), source_ml_concat_value10)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                {
                                    let source_ml_concat_value11 = vec![
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
                                        let source_ml_concat_value12 = signs;
                                        {
                                            let source_ml_concat_value13 =
                                                vec![String::from("if"), String::from("if")];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<String>::new(),
                                                        source_ml_concat_value11,
                                                    ),
                                                    source_ml_concat_value12,
                                                ),
                                                source_ml_concat_value13,
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
                    let source_ml_concat_value1 = vec![
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
                        let source_ml_concat_value2 = positive_positive;
                        {
                            let source_ml_concat_value3 =
                                vec![String::from("if"), String::from("if")];
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_concat_value1,
                                    ),
                                    source_ml_concat_value2,
                                ),
                                source_ml_concat_value3,
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
    } else if consumer == "unwrap_or" {
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
                            let source_ml_comparison_operand14 = self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(0f64),
                            );
                            {
                                let source_ml_comparison_operand15 = String::from("std");
                                match (
                                    source_ml_comparison_operand14,
                                    source_ml_comparison_operand15,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        } || {
                            let source_ml_comparison_operand16 = self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(0f64),
                            );
                            {
                                let source_ml_comparison_operand17 = String::from("core");
                                match (
                                    source_ml_comparison_operand16,
                                    source_ml_comparison_operand17,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }))
                        && {
                            let source_ml_comparison_operand18 = self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(1f64),
                            );
                            {
                                let source_ml_comparison_operand19 = String::from("primitive");
                                match (
                                    source_ml_comparison_operand18,
                                    source_ml_comparison_operand19,
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
