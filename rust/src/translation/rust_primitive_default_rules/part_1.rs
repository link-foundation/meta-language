// Generated from complete js/src/translation/rust-primitive-default-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustIntegerBitsJoin2;
use super::MlTemporaryPlanRustIntegerBitsLoop1Result;
use super::source_ml_temporary_plan_rust_integer_bits_loop1;
use super::source_ml_temporary_plan_rust_integer_bits_loop3;

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
pub fn accept_rust_wrapping_shift_method(method: &str) -> bool {
    self::ml_array::includes(
        &vec![
            String::from("wrapping_shl"),
            String::from("wrapping_shr"),
            String::from("rotate_left"),
            String::from("rotate_right"),
        ],
        &method.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_wrapping_shift_receiver(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (kind == "fixed")
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_wrapping_shift_operation(term: &str) -> String {
    if term == "shift left" {
        String::from("shiftRustPrimitiveBitsLeft")
    } else if term == "shift right" {
        String::from("shiftRustPrimitiveBitsRight")
    } else if term == "bits or" {
        String::from("disjoinRustPrimitiveBits")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_wrapping_shift(method: &str) -> Vec<String> {
    if (self::accept_rust_wrapping_shift_method(method)) {
        {
            let count = vec![
                String::from("count"),
                String::from("width"),
                String::from("rem"),
            ];
            if (method == "wrapping_shl") || (method == "wrapping_shr") {
                {
                    let source_ml_source1_concat_value1 = vec![String::from("operand")];
                    {
                        let source_ml_source1_concat_value2 = count;
                        {
                            let source_ml_source1_concat_value3 =
                                vec![if method == "wrapping_shl" {
                                    String::from("shift left")
                                } else {
                                    String::from("shift right")
                                }];
                            self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
                                        source_ml_source1_concat_value1,
                                    ),
                                    source_ml_source1_concat_value2,
                                ),
                                source_ml_source1_concat_value3,
                            )
                        }
                    }
                }
            } else {
                {
                    let inverse = {
                        let source_ml_source1_concat_value4 = vec![String::from("width")];
                        {
                            let source_ml_source1_concat_value5 = count.clone();
                            {
                                let source_ml_source1_concat_value6 = vec![
                                    String::from("sub"),
                                    String::from("width"),
                                    String::from("rem"),
                                ];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
                                            source_ml_source1_concat_value4,
                                        ),
                                        source_ml_source1_concat_value5,
                                    ),
                                    source_ml_source1_concat_value6,
                                )
                            }
                        }
                    };
                    {
                        let source_ml_source1_concat_value7 =
                            vec![String::from("unsigned operand")];
                        {
                            let source_ml_source1_concat_value8 = count;
                            {
                                let source_ml_source1_concat_value9 = vec![
                                    if method == "rotate_left" {
                                        String::from("shift left")
                                    } else {
                                        String::from("shift right")
                                    },
                                    String::from("unsigned operand"),
                                ];
                                {
                                    let source_ml_source1_concat_value10 = inverse;
                                    {
                                        let source_ml_source1_concat_value11 = vec![
                                            if method == "rotate_left" {
                                                String::from("shift right")
                                            } else {
                                                String::from("shift left")
                                            },
                                            String::from("bits or"),
                                            String::from("source cast"),
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<String>::new(),
                                                            source_ml_source1_concat_value7,
                                                        ),
                                                        source_ml_source1_concat_value8,
                                                    ),
                                                    source_ml_source1_concat_value9,
                                                ),
                                                source_ml_source1_concat_value10,
                                            ),
                                            source_ml_source1_concat_value11,
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_integer_bit_method(method: &str) -> bool {
    self::ml_array::includes(
        &vec![
            String::from("count_ones"),
            String::from("count_zeros"),
            String::from("leading_ones"),
            String::from("leading_zeros"),
            String::from("trailing_ones"),
            String::from("trailing_zeros"),
            String::from("reverse_bits"),
        ],
        &method.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn accept_rust_integer_bit_receiver(
    kind: &str,
    bits: &str,
    signed: bool,
    owner: &str,
    path: &str,
    children: f64,
    proven: bool,
    layers: f64,
) -> bool {
    (kind == "fixed")
        && self::accept_rust_primitive_selection_receiver(
            kind, bits, signed, owner, path, children, proven, layers,
        )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_bit_result_owner(method: &str, receiver: &str) -> String {
    if (self::accept_rust_integer_bit_method(method)) {
        if method == "reverse_bits" {
            receiver.to_owned()
        } else {
            String::from("u32")
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_bit_plan_operation(code: f64) -> String {
    if code == 0f64 {
        String::from("unsigned input")
    } else if code == 1f64 {
        String::from("fixed zero")
    } else if code == 2f64 {
        String::from("fixed one")
    } else if code == 3f64 {
        String::from("fixed two")
    } else if code == 4f64 {
        String::from("count zero")
    } else if code == 5f64 {
        String::from("count one")
    } else if code == 6f64 {
        String::from("false")
    } else if code == 7f64 {
        String::from("true")
    } else if code == 8f64 {
        String::from("fixed reference")
    } else if code == 9f64 {
        String::from("count reference")
    } else if code == 10f64 {
        String::from("boolean reference")
    } else if code == 11f64 {
        String::from("equal")
    } else if code == 12f64 {
        String::from("remainder")
    } else if code == 13f64 {
        String::from("division")
    } else if code == 14f64 {
        String::from("count addition")
    } else if code == 15f64 {
        String::from("fixed addition")
    } else if code == 16f64 {
        String::from("fixed multiplication")
    } else if code == 17f64 {
        String::from("conditional")
    } else if code == 18f64 {
        String::from("binding")
    } else if code == 19f64 {
        String::from("source cast")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_integer_bits(method: &str, bits: f64) -> Vec<f64> {
    if !(self::accept_rust_integer_bit_method(method))
        || !(self::ml_array::includes(
            &vec![8f64, 16f64, 32f64, 64f64, 128f64],
            &bits,
            0f64,
            false,
            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
        ))
    {
        Vec::<f64>::new()
    } else {
        {
            let reverse = method == "reverse_bits";
            {
                let trailing = (method == "trailing_zeros") || (method == "trailing_ones");
                {
                    let leading = (method == "leading_zeros") || (method == "leading_ones");
                    {
                        let ones = method.ends_with("ones");
                        {
                            let plan = vec![
                                0f64, 0f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64, 0f64, 2f64,
                                0f64, 0f64, 0f64, 0f64, 3f64, 0f64, 0f64, 0f64, 0f64, 4f64, 0f64,
                                0f64, 0f64, 0f64, 5f64, 0f64, 0f64, 0f64, 0f64, 6f64, 0f64, 0f64,
                                0f64, 0f64, 7f64, 0f64, 0f64, 0f64, 0f64,
                            ];
                            {
                                let bindings = vec![
                                    0f64,
                                    0f64,
                                    1f64,
                                    if reverse { 1f64 } else { 4f64 },
                                    2f64,
                                    7f64,
                                ];
                                {
                                    let current = 0f64;
                                    {
                                        let count = 1f64;
                                        {
                                            let active = 2f64;
                                            {
                                                let index = 8f64;
                                                {
                                                    let bit = 0f64;
                                                    {
                                                        let source_ml_source1_s12 = self::source_ml_temporary_plan_rust_integer_bits_loop1(bits, reverse, trailing, leading, ones, plan, bindings, current, count, active, index, bit);
                                                        match source_ml_source1_s12 {
                                                            self::MlTemporaryPlanRustIntegerBitsLoop1Result::MlTemporaryPlanRustIntegerBitsLoop1Done(plan_2, bindings_2, count_2, index_2) => {
                                                                {
                                                                    let plan_3 = {
                                                                        let source_ml_source1_concat_value13 = plan_2;
                                                                        {
                                                                            let source_ml_source1_concat_value14 = vec![if reverse {
                                                                                8f64
                                                                            } else {
                                                                                9f64
                                                                            }, count_2, 0f64, 0f64, 0f64];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value13), source_ml_source1_concat_value14)
                                                                        }
                                                                    };
                                                                    {
                                                                        let body = index_2;
                                                                        {
                                                                            let index_3 = index_2 + 1f64;
                                                                            {
                                                                                let source_ml_source1_s17 = if reverse {
                                                                                    {
                                                                                        let plan_4 = {
                                                                                            let source_ml_source1_concat_value18 = plan_3;
                                                                                            {
                                                                                                let source_ml_source1_concat_value19 = vec![19f64, body, 0f64, 0f64, 0f64];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value18), source_ml_source1_concat_value19)
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let body_2 = index_3;
                                                                                            {
                                                                                                let index_4 = index_3 + 1f64;
                                                                                                self::MlTemporaryPlanRustIntegerBitsJoin2::MlTemporaryPlanRustIntegerBitsJoin2Next(plan_4, index_4, body_2)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlTemporaryPlanRustIntegerBitsJoin2::MlTemporaryPlanRustIntegerBitsJoin2Next(plan_3, index_3, body)
                                                                                };
                                                                                match source_ml_source1_s17 {
                                                                                    self::MlTemporaryPlanRustIntegerBitsJoin2::MlTemporaryPlanRustIntegerBitsJoin2Next(plan_5, index_5, body_3) => {
                                                                                        {
                                                                                            let at = (bindings_2.len() as f64) - 2f64;
                                                                                            {
                                                                                                self::source_ml_temporary_plan_rust_integer_bits_loop3(plan_5, &(bindings_2), index_5, body_3, at)
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
        }
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
                String::from("checked_shl"),
                String::from("checked_shr"),
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
    if ((consumer == "is_some") || (consumer == "is_none")) || (consumer == "unwrap_or_default") {
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
pub fn read_rust_checked_integer_argument_owner(method: &str, receiver: &str) -> String {
    if (method == "checked_shl") || (method == "checked_shr") {
        String::from("u32")
    } else {
        receiver.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_argument_reason(method: &str) -> String {
    if (method == "checked_shl") || (method == "checked_shr") {
        String::from("the actual shift count must retain standard u32 ownership")
    } else {
        String::from("the actual argument must retain the receiver type and source owner")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_checked_integer_shift_operation(term: &str) -> String {
    if term == "shift left" {
        String::from("shiftRustPrimitiveBitsLeft")
    } else if term == "shift right" {
        String::from("shiftRustPrimitiveBitsRight")
    } else {
        String::new()
    }
}
