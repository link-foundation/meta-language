// Generated from complete js/src/translation/rust-primitive-try-from-rules.js. Do not edit by hand.
/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() { b } else { a }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() { a } else { b }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

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
pub fn accept_rust_scalar_predicate(
    kind: &str,
    parameters: f64,
    layers: f64,
    parameter: bool,
    result: bool,
) -> bool {
    ((((kind == "function") && (parameters == 1f64)) && (layers == 0f64)) && parameter) && result
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_version_default() -> String {
    String::from("1.99.0")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_version_error(
    major: f64,
    minor: f64,
    patch: f64,
    canonical: bool,
) -> String {
    if ((((((canonical && (major == 1f64)) && self::ml_math::is_integer(minor))
        && (minor >= 90f64))
        && (minor < 100f64))
        && self::ml_math::is_integer(patch))
        && (patch >= 0f64))
        && (patch <= 4_294_967_295f64)
    {
        String::new()
    } else {
        String::from("select a canonical stable Rust source version from 1.90.0 through 1.99.x")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_integer_conversion_error_debug_model(minor: f64) -> String {
    if (self::ml_math::is_integer(minor) && (minor >= 90f64)) && (minor < 97f64) {
        String::from("unit payload")
    } else if (self::ml_math::is_integer(minor) && (minor >= 97f64)) && (minor < 100f64) {
        String::from("overflow kind")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_version_text_limit() -> f64 {
    15f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_primitive_owner_path(
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
                            let source_ml_comparison_operand1 = self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(0f64),
                            );
                            {
                                let source_ml_comparison_operand2 = String::from("std");
                                match (source_ml_comparison_operand1, source_ml_comparison_operand2)
                                {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        } || {
                            let source_ml_comparison_operand3 = self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(0f64),
                            );
                            {
                                let source_ml_comparison_operand4 = String::from("core");
                                match (source_ml_comparison_operand3, source_ml_comparison_operand4)
                                {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }))
                        && {
                            let source_ml_comparison_operand5 = self::ml_array::optional_at(
                                segments,
                                self::ml_array::number_index(1f64),
                            );
                            {
                                let source_ml_comparison_operand6 = String::from("primitive");
                                match (source_ml_comparison_operand5, source_ml_comparison_operand6)
                                {
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
pub fn read_rust_primitive_integer_constant_owner(
    segments: &[String],
    ordinary: bool,
    shadowed: bool,
    absolute: bool,
) -> String {
    {
        let owner = self::read_rust_primitive_owner_path(segments, ordinary, shadowed, absolute);
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
        let source_ml_comparison_operand7 =
            self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
        {
            let source_ml_comparison_operand8 = String::from("f64");
            match (source_ml_comparison_operand7, source_ml_comparison_operand8) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || (((((segments.len() as f64) == 4f64)
        && ({
            let source_ml_comparison_operand9 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_comparison_operand10 = String::from("std");
                match (
                    source_ml_comparison_operand9,
                    source_ml_comparison_operand10,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } || {
            let source_ml_comparison_operand11 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(0f64));
            {
                let source_ml_comparison_operand12 = String::from("core");
                match (
                    source_ml_comparison_operand11,
                    source_ml_comparison_operand12,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }))
        && {
            let source_ml_comparison_operand13 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(1f64));
            {
                let source_ml_comparison_operand14 = String::from("primitive");
                match (
                    source_ml_comparison_operand13,
                    source_ml_comparison_operand14,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
        && {
            let source_ml_comparison_operand15 =
                self::ml_array::optional_at(segments, self::ml_array::number_index(2f64));
            {
                let source_ml_comparison_operand16 = String::from("f64");
                match (
                    source_ml_comparison_operand15,
                    source_ml_comparison_operand16,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        })
    {
        String::from("f64")
    } else {
        self::read_rust_primitive_owner_path(segments, ordinary, shadowed, absolute)
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
