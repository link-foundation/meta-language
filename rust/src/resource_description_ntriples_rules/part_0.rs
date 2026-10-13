// Generated from resource-description-ntriples-rules.js and its dependencies by the checked translator.
// Do not edit by hand.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
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
        if x < 0.0 && x >= -0.5 {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.
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
    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
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
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
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
    fn integer(value: f64) -> f64 {
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

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_unit(units: &[f64], index: f64) -> f64 {
    if (!(super::ml_math::is_safe_integer(index)) || (index < (0f64))) {
        (-1f64)
    } else {
        (super::ml_array::optional_at(units, super::ml_array::number_index(index))).unwrap_or(-1f64)
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_letter(unit: f64) -> bool {
    (((unit >= (65f64)) && (unit <= (90f64))) || ((unit >= (97f64)) && (unit <= (122f64))))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_digit(unit: f64) -> bool {
    ((unit >= (48f64)) && (unit <= (57f64)))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_hexadecimal(unit: f64) -> bool {
    ((super::resource_description_digit(unit) || ((unit >= (65f64)) && (unit <= (70f64))))
        || ((unit >= (97f64)) && (unit <= (102f64))))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn accept_resource_description_unicode(units: &[f64]) -> bool {
    {
        let index = 0f64;
        {
            let ml_s1 = super::ml_accept_resource_description_unicode_loop1(units, index);
            match ml_s1 {
                super::MlAcceptResourceDescriptionUnicodeLoop1Result::MlAcceptResourceDescriptionUnicodeLoop1Done => {
                    true
                }
                super::MlAcceptResourceDescriptionUnicodeLoop1Result::MlAcceptResourceDescriptionUnicodeLoop1Return(ml_result1) => {
                    ml_result1
                }
            }
        }
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_n_triples_name(code: f64, first: bool) -> bool {
    if ((((((((((((((((((code >= (65f64)) && (code <= (90f64)))
        || ((code >= (97f64)) && (code <= (122f64))))
        || ((code >= (192f64)) && (code <= (214f64))))
        || ((code >= (216f64)) && (code <= (246f64))))
        || ((code >= (248f64)) && (code <= (767f64))))
        || ((code >= (880f64)) && (code <= (893f64))))
        || ((code >= (895f64)) && (code <= (8191f64))))
        || ((code >= (8204f64)) && (code <= (8205f64))))
        || ((code >= (8304f64)) && (code <= (8591f64))))
        || ((code >= (11264f64)) && (code <= (12271f64))))
        || ((code >= (12289f64)) && (code <= (55295f64))))
        || ((code >= (63744f64)) && (code <= (64975f64))))
        || ((code >= (65008f64)) && (code <= (65533f64))))
        || ((code >= (65536f64)) && (code <= (983_039f64))))
        || (code == (95f64)))
        || (code == (58f64)))
        || ((code >= (48f64)) && (code <= (57f64))))
    {
        true
    } else {
        (!first
            && (((((code == (45f64)) || (code == (46f64))) || (code == (183f64)))
                || ((code >= (768f64)) && (code <= (879f64))))
                || ((code >= (8255f64)) && (code <= (8256f64)))))
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_n_triples_escape(code: f64) -> f64 {
    if (code == (116f64)) {
        9f64
    } else if (code == (98f64)) {
        8f64
    } else if (code == (110f64)) {
        10f64
    } else if (code == (114f64)) {
        13f64
    } else if (code == (102f64)) {
        12f64
    } else if (code == (34f64)) {
        34f64
    } else if (code == (39f64)) {
        39f64
    } else if (code == (92f64)) {
        92f64
    } else {
        (-1f64)
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn read_resource_description_n_triples_transition(state: f64, kind: f64) -> f64 {
    if ((kind == (9f64))
        && (((((((state == (0f64)) || (state == (1f64))) || (state == (2f64)))
            || (state == (3f64)))
            || (state == (4f64)))
            || (state == (5f64)))
            || (state == (7f64))))
    {
        state
    } else if ((state == (0f64)) && (kind == (1f64))) {
        1f64
    } else if ((state == (0f64)) && (kind == (2f64))) {
        1f64
    } else if ((state == (1f64)) && (kind == (1f64))) {
        2f64
    } else if ((state == (2f64)) && (kind == (1f64))) {
        3f64
    } else if ((state == (2f64)) && (kind == (2f64))) {
        3f64
    } else if ((state == (2f64)) && (kind == (3f64))) {
        4f64
    } else if ((state == (4f64)) && (kind == (4f64))) {
        3f64
    } else if ((state == (4f64)) && (kind == (6f64))) {
        5f64
    } else if ((state == (5f64)) && (kind == (1f64))) {
        3f64
    } else if ((state == (3f64)) && (kind == (5f64))) {
        7f64
    } else if ((state == (4f64)) && (kind == (5f64))) {
        7f64
    } else if ((state == (0f64)) && (kind == (7f64))) {
        0f64
    } else if ((state == (7f64)) && (kind == (7f64))) {
        0f64
    } else if ((state == (0f64)) && (kind == (8f64))) {
        0f64
    } else if ((state == (7f64)) && (kind == (8f64))) {
        7f64
    } else {
        (-1f64)
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn read_resource_description_n_triples_role(state: f64, kind: f64) -> f64 {
    if ((state == (0f64)) && (kind == (1f64))) {
        1f64
    } else if ((state == (0f64)) && (kind == (2f64))) {
        1f64
    } else if ((state == (1f64)) && (kind == (1f64))) {
        2f64
    } else if ((state == (2f64)) && (kind == (1f64))) {
        3f64
    } else if ((state == (2f64)) && (kind == (2f64))) {
        3f64
    } else if ((state == (2f64)) && (kind == (3f64))) {
        3f64
    } else if ((state == (4f64)) && (kind == (4f64))) {
        4f64
    } else if ((state == (5f64)) && (kind == (1f64))) {
        5f64
    } else if ((state == (3f64)) && (kind == (5f64))) {
        6f64
    } else if ((state == (4f64)) && (kind == (5f64))) {
        6f64
    } else {
        0f64
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn accept_resource_description_n_triples_final_state(state: f64) -> bool {
    ((state == (0f64)) || (state == (7f64)))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn decode_resource_description_scalar_escape(units: &[f64], start: f64, width: f64) -> f64 {
    if ((((width != (4f64)) && (width != (8f64))) || !(super::ml_math::is_safe_integer(start)))
        || (start < (0f64)))
    {
        (-1f64)
    } else {
        {
            let value = 0f64;
            {
                let index = start;
                {
                    let ml_s2 = super::ml_decode_resource_description_scalar_escape_loop2(
                        units, start, width, value, index,
                    );
                    match ml_s2 {
                        super::MlDecodeResourceDescriptionScalarEscapeLoop2Result::MlDecodeResourceDescriptionScalarEscapeLoop2Done(value_2) => {
                            if ((value_2 > (1_114_111f64)) || ((value_2 >= (55296f64)) && (value_2 <= (57343f64)))) {
                                (-1f64)
                            } else {
                                value_2
                            }
                        }
                        super::MlDecodeResourceDescriptionScalarEscapeLoop2Result::MlDecodeResourceDescriptionScalarEscapeLoop2Return(ml_result2) => {
                            ml_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn read_resource_description_n_triples_scalar(
    units: &[f64],
    index: f64,
    kind: f64,
) -> Vec<f64> {
    {
        let code = super::resource_description_unit(units, index);
        if ((code < (0f64)) || (code > (65535f64))) {
            vec![0f64, (-1f64), 1f64]
        } else if ((code == (92f64)) && (kind != (0f64))) {
            {
                let escape = super::resource_description_unit(units, (index + 1f64));
                if ((escape == (117f64)) || (escape == (85f64))) {
                    {
                        let width = if (escape == (117f64)) { 4f64 } else { 8f64 };
                        {
                            let scalar = super::decode_resource_description_scalar_escape(
                                units,
                                (index + 2f64),
                                width,
                            );
                            if (scalar < (0f64)) {
                                vec![0f64, (-1f64), 4f64]
                            } else {
                                vec![(width + 2f64), scalar, 0f64]
                            }
                        }
                    }
                } else {
                    {
                        let scalar_2 = super::resource_description_n_triples_escape(escape);
                        if ((kind != (3f64)) || (scalar_2 < (0f64))) {
                            vec![0f64, (-1f64), 5f64]
                        } else {
                            vec![2f64, scalar_2, 0f64]
                        }
                    }
                }
            }
        } else if ((code >= (55296f64)) && (code <= (56319f64))) {
            {
                let low = super::resource_description_unit(units, (index + 1f64));
                if ((low < (56320f64)) || (low > (57343f64))) {
                    vec![0f64, (-1f64), 1f64]
                } else {
                    vec![
                        2f64,
                        (((65536f64 + ((code - 55296f64) * 1024f64)) + low) - 56320f64),
                        0f64,
                    ]
                }
            }
        } else if ((code >= (56320f64)) && (code <= (57343f64))) {
            vec![0f64, (-1f64), 1f64]
        } else if (((kind == (1f64))
            && ((((((((code <= (32f64)) || (code == (60f64))) || (code == (34f64)))
                || (code == (123f64)))
                || (code == (125f64)))
                || (code == (124f64)))
                || (code == (94f64)))
                || (code == (96f64))))
            || ((kind == (3f64)) && ((code == (10f64)) || (code == (13f64)))))
        {
            vec![0f64, (-1f64), 6f64]
        } else {
            vec![1f64, code, 0f64]
        }
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
// The checked continuation explicitly carries its lexical state.
#[allow(clippy::too_many_lines)]
pub fn read_resource_description_n_triples_token(units: &[f64], start: f64) -> Vec<f64> {
    {
        let index = start;
        {
            let code = super::resource_description_unit(units, index);
            if ((((code == (32f64)) || (code == (9f64))) || (code == (10f64))) || (code == (13f64)))
            {
                {
                    let kind = if ((code == (32f64)) || (code == (9f64))) {
                        9f64
                    } else {
                        7f64
                    };
                    {
                        let index_2 = super::ml_read_resource_description_n_triples_token_loop3(
                            units, index, kind,
                        );
                        vec![kind, index_2, 0f64]
                    }
                }
            } else if (code == (35f64)) {
                {
                    let ml_s3 =
                        super::ml_read_resource_description_n_triples_token_loop4(units, index);
                    match ml_s3 {
                            super::MlReadResourceDescriptionNTriplesTokenLoop4Result::MlReadResourceDescriptionNTriplesTokenLoop4Done(index_3) => {
                                vec![8f64, index_3, 0f64]
                            }
                            super::MlReadResourceDescriptionNTriplesTokenLoop4Result::MlReadResourceDescriptionNTriplesTokenLoop4Return(ml_result4) => {
                                ml_result4.clone()
                            }
                        }
                }
            } else if (code == (46f64)) {
                vec![5f64, (index + 1f64), 0f64]
            } else if ((code == (94f64))
                && ((super::resource_description_unit(units, (index + 1f64))) == (94f64)))
            {
                vec![6f64, (index + 2f64), 0f64]
            } else if (code == (64f64)) {
                {
                    let index_4 = (index + 1f64);
                    {
                        let ml_s4 = super::ml_read_resource_description_n_triples_token_loop5(
                            units, index_4, code,
                        );
                        match ml_s4 {
                                            super::MlReadResourceDescriptionNTriplesTokenLoop5Result::MlReadResourceDescriptionNTriplesTokenLoop5Done(index_5, code_2) => {
                                                if (index_5 == ((start + 1f64))) {
                                                    vec![0f64, start, 2f64]
                                                } else {
                                                    vec![4f64, index_5, 0f64]
                                                }
                                            }
                                        }
                    }
                }
            } else if ((code == (95f64))
                && ((super::resource_description_unit(units, (index + 1f64))) == (58f64)))
            {
                {
                    let index_6 = (index + 2f64);
                    {
                        let beginning = index_6;
                        {
                            let ml_s5 = super::ml_read_resource_description_n_triples_token_loop6(
                                units, index_6, beginning,
                            );
                            match ml_s5 {
                                                    super::MlReadResourceDescriptionNTriplesTokenLoop6Result::MlReadResourceDescriptionNTriplesTokenLoop6Done(index_7) => {
                                                        {
                                                            let index_8 = super::ml_read_resource_description_n_triples_token_loop7(units, index_7, beginning);
                                                            if (index_8 == beginning) {
                                                                vec![0f64, start, 3f64]
                                                            } else {
                                                                vec![2f64, index_8, 0f64]
                                                            }
                                                        }
                                                    }
                                                    super::MlReadResourceDescriptionNTriplesTokenLoop6Result::MlReadResourceDescriptionNTriplesTokenLoop6Return(ml_result7) => {
                                                        ml_result7.clone()
                                                    }
                                                }
                        }
                    }
                }
            } else if ((code == (60f64)) || (code == (34f64))) {
                {
                    let kind_2 = if (code == (60f64)) { 1f64 } else { 3f64 };
                    {
                        let close = if (code == (60f64)) { 62f64 } else { 34f64 };
                        {
                            let index_9 = (index + 1f64);
                            {
                                let ml_s6 =
                                    super::ml_read_resource_description_n_triples_token_loop8(
                                        units, index_9, kind_2, close,
                                    );
                                match ml_s6 {
                                                            super::MlReadResourceDescriptionNTriplesTokenLoop8Result::MlReadResourceDescriptionNTriplesTokenLoop8Done(index_10) => {
                                                                vec![0f64, start, 7f64]
                                                            }
                                                            super::MlReadResourceDescriptionNTriplesTokenLoop8Result::MlReadResourceDescriptionNTriplesTokenLoop8Return(ml_result8) => {
                                                                ml_result8.clone()
                                                            }
                                                        }
                            }
                        }
                    }
                }
            } else {
                {
                    let scalar =
                        super::read_resource_description_n_triples_scalar(units, index, 0f64);
                    vec![
                        0f64,
                        index,
                        if ((super::resource_description_unit(&scalar, 2f64)) != (0f64)) {
                            1f64
                        } else {
                            8f64
                        },
                    ]
                }
            }
        }
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn read_resource_description_n_triples_canonical_escape(code: f64) -> f64 {
    if ((code == (34f64)) || (code == (92f64))) {
        code
    } else if (code == (10f64)) {
        110f64
    } else if (code == (13f64)) {
        114f64
    } else {
        (-1f64)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptResourceDescriptionUnicodeLoop1Result {
    MlAcceptResourceDescriptionUnicodeLoop1Done,
    MlAcceptResourceDescriptionUnicodeLoop1Return(bool),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn ml_accept_resource_description_unicode_loop1(
    units: &[f64],
    mut index: f64,
) -> super::MlAcceptResourceDescriptionUnicodeLoop1Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let unit = super::resource_description_unit(units, index);
                if ((!(super::ml_math::is_safe_integer(unit)) || (unit < (0f64)))
                    || (unit > (65535f64)))
                {
                    super::MlAcceptResourceDescriptionUnicodeLoop1Result::MlAcceptResourceDescriptionUnicodeLoop1Return(false)
                } else if ((unit >= (55296f64)) && (unit <= (56319f64))) {
                    {
                        let index_2 = (index + 1f64);
                        if (((index_2 >= (units.len() as f64))
                            || ((super::resource_description_unit(units, index_2)) < (56320f64)))
                            || ((super::resource_description_unit(units, index_2)) > (57343f64)))
                        {
                            super::MlAcceptResourceDescriptionUnicodeLoop1Result::MlAcceptResourceDescriptionUnicodeLoop1Return(false)
                        } else {
                            {
                                let index_3 = (index_2 + 1f64);
                                {
                                    index = index_3;
                                    continue;
                                }
                            }
                        }
                    }
                } else if ((unit >= (56320f64)) && (unit <= (57343f64))) {
                    super::MlAcceptResourceDescriptionUnicodeLoop1Result::MlAcceptResourceDescriptionUnicodeLoop1Return(false)
                } else {
                    {
                        let index_4 = (index + 1f64);
                        {
                            index = index_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            super::MlAcceptResourceDescriptionUnicodeLoop1Result::MlAcceptResourceDescriptionUnicodeLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeResourceDescriptionScalarEscapeLoop2Result {
    MlDecodeResourceDescriptionScalarEscapeLoop2Done(f64),
    MlDecodeResourceDescriptionScalarEscapeLoop2Return(f64),
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn ml_decode_resource_description_scalar_escape_loop2(
    units: &[f64],
    mut start: f64,
    mut width: f64,
    mut value: f64,
    mut index: f64,
) -> super::MlDecodeResourceDescriptionScalarEscapeLoop2Result {
    loop {
        return if (index < (start + width)) {
            {
                let code = super::resource_description_unit(units, index);
                if !(super::resource_description_hexadecimal(code)) {
                    super::MlDecodeResourceDescriptionScalarEscapeLoop2Result::MlDecodeResourceDescriptionScalarEscapeLoop2Return((-1f64))
                } else {
                    {
                        let code_3 = if (code >= (97f64)) {
                            {
                                let code_2 = (code - 32f64);
                                code_2
                            }
                        } else {
                            code
                        };
                        {
                            let value_2 = ((value * 16f64)
                                + if (code_3 >= (65f64)) {
                                    (code_3 - 55f64)
                                } else {
                                    (code_3 - 48f64)
                                });
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (value, index) = (value_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            super::MlDecodeResourceDescriptionScalarEscapeLoop2Result::MlDecodeResourceDescriptionScalarEscapeLoop2Done(value)
        };
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn ml_read_resource_description_n_triples_token_loop3(
    units: &[f64],
    mut index: f64,
    mut kind: f64,
) -> f64 {
    loop {
        return if ((index < (units.len() as f64))
            && if (kind == (9f64)) {
                (((super::resource_description_unit(units, index)) == (32f64))
                    || ((super::resource_description_unit(units, index)) == (9f64)))
            } else {
                (((super::resource_description_unit(units, index)) == (10f64))
                    || ((super::resource_description_unit(units, index)) == (13f64)))
            }) {
            {
                let index_2 = (index + 1f64);
                {
                    index = index_2;
                    continue;
                }
            }
        } else {
            index
        };
    }
}
