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

pub use crate::translation::frontend_rules::ml_array;

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
    (((65f64..=90f64).contains(&unit)) || ((97f64..=122f64).contains(&unit)))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_digit(unit: f64) -> bool {
    ((48f64..=57f64).contains(&unit))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_hexadecimal(unit: f64) -> bool {
    ((super::resource_description_digit(unit) || ((65f64..=70f64).contains(&unit)))
        || ((97f64..=102f64).contains(&unit)))
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn accept_resource_description_unicode(units: &[f64]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s1 =
                crate::source_ml_temporary_accept_resource_description_unicode_loop1(units, index);
            match source_ml_source1_s1.clone() {
                crate::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Done => {
                    true
                }
                crate::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Return(source_ml_temporary_result1) => {
                    source_ml_temporary_result1
                }
            }
        }
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn resource_description_n_triples_name(code: f64, first: bool) -> bool {
    if ((((((((((((((((((65f64..=90f64).contains(&code))
        || ((97f64..=122f64).contains(&code)))
        || ((192f64..=214f64).contains(&code)))
        || ((216f64..=246f64).contains(&code)))
        || ((248f64..=767f64).contains(&code)))
        || ((880f64..=893f64).contains(&code)))
        || ((895f64..=8191f64).contains(&code)))
        || ((8204f64..=8205f64).contains(&code)))
        || ((8304f64..=8591f64).contains(&code)))
        || ((11264f64..=12271f64).contains(&code)))
        || ((12289f64..=55295f64).contains(&code)))
        || ((63744f64..=64975f64).contains(&code)))
        || ((65008f64..=65533f64).contains(&code)))
        || ((65536f64..=983_039f64).contains(&code)))
        || (code == (95f64)))
        || (code == (58f64)))
        || ((48f64..=57f64).contains(&code)))
    {
        true
    } else {
        (!first
            && (((((code == (45f64)) || (code == (46f64))) || (code == (183f64)))
                || ((768f64..=879f64).contains(&code)))
                || ((8255f64..=8256f64).contains(&code))))
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
    } else if (state == (0f64)) && ((kind == (1f64)) || (kind == (2f64))) {
        1f64
    } else if ((state == (1f64)) && (kind == (1f64))) {
        2f64
    } else if ((kind == (1f64)) && ((state == (2f64)) || (state == (5f64))))
        || ((kind == (2f64)) && (state == (2f64)) || (kind == (4f64)) && (state == (4f64)))
    {
        3f64
    } else if ((state == (2f64)) && (kind == (3f64))) {
        4f64
    } else if ((state == (4f64)) && (kind == (6f64))) {
        5f64
    } else if (((kind == (5f64)) && ((state == (3f64)) || (state == (4f64))))
        || ((kind == (8f64)) && (state == (7f64))))
    {
        7f64
    } else if (((kind == (7f64)) && ((state == (0f64)) || (state == (7f64))))
        || ((kind == (8f64)) && (state == (0f64))))
    {
        0f64
    } else {
        (-1f64)
    }
}

/// Execute the checked shared resource-description algorithm.
#[must_use]
pub fn read_resource_description_n_triples_role(state: f64, kind: f64) -> f64 {
    if (state == (0f64)) && ((kind == (1f64)) || (kind == (2f64))) {
        1f64
    } else if ((state == (1f64)) && (kind == (1f64))) {
        2f64
    } else if (state == (2f64)) && ((kind == (1f64)) || (kind == (2f64)) || (kind == (3f64))) {
        3f64
    } else if ((state == (4f64)) && (kind == (4f64))) {
        4f64
    } else if ((state == (5f64)) && (kind == (1f64))) {
        5f64
    } else if (kind == (5f64)) && ((state == (3f64)) || (state == (4f64))) {
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
                    let source_ml_source1_s2 =
                        crate::source_ml_temporary_decode_resource_description_scalar_escape_loop2(
                            units, start, width, value, index,
                        );
                    match source_ml_source1_s2.clone() {
                        crate::MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Result::MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Done(value_2) => {
                            if ((value_2 > (1_114_111f64)) || ((value_2 >= (55296f64)) && (value_2 <= (57343f64)))) {
                                (-1f64)
                            } else {
                                value_2
                            }
                        }
                        crate::MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Result::MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
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
                let escape = super::resource_description_unit(units, index + 1f64);
                if ((escape == (117f64)) || (escape == (85f64))) {
                    {
                        let width = if (escape == (117f64)) { 4f64 } else { 8f64 };
                        {
                            let scalar = super::decode_resource_description_scalar_escape(
                                units,
                                index + 2f64,
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
        } else if ((55296f64..=56319f64).contains(&code)) {
            {
                let low = super::resource_description_unit(units, index + 1f64);
                if ((low < (56320f64)) || (low > (57343f64))) {
                    vec![0f64, (-1f64), 1f64]
                } else {
                    vec![
                        2f64,
                        ((({
                            let separately_rounded_product = (code - 55296f64) * 1024f64;
                            65536f64 + separately_rounded_product
                        }) + low)
                            - 56320f64),
                        0f64,
                    ]
                }
            }
        } else if ((56320f64..=57343f64).contains(&code)) {
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
                        let index_2 = crate::source_ml_temporary_read_resource_description_n_triples_token_loop3(units, index, kind);
                        vec![kind, index_2, 0f64]
                    }
                }
            } else {
                if (code == (35f64)) {
                    {
                        let source_ml_source1_s3 = crate::source_ml_temporary_read_resource_description_n_triples_token_loop4(units, index);
                        match source_ml_source1_s3.clone() {
                            crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Done(index_3) => {
                                vec![8f64, index_3, 0f64]
                            }
                            crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Return(source_ml_temporary_result4) => {
                                source_ml_temporary_result4.clone()
                            }
                        }
                    }
                } else {
                    if (code == (46f64)) {
                        vec![5f64, (index + 1f64), 0f64]
                    } else {
                        if ((code == (94f64))
                            && ((super::resource_description_unit(units, (index + 1f64)))
                                == (94f64)))
                        {
                            vec![6f64, (index + 2f64), 0f64]
                        } else {
                            if (code == (64f64)) {
                                {
                                    let index_4 = (index + 1f64);
                                    {
                                        let source_ml_source1_s4 = crate::source_ml_temporary_read_resource_description_n_triples_token_loop5(units, index_4, code);
                                        match source_ml_source1_s4.clone() {
                                            crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop5Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop5Done(index_5, code_2) => {
                                                if (index_5 == ((start + 1f64))) {
                                                    vec![0f64, start, 2f64]
                                                } else {
                                                    vec![4f64, index_5, 0f64]
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((code == (95f64))
                                    && ((super::resource_description_unit(units, (index + 1f64)))
                                        == (58f64)))
                                {
                                    {
                                        let index_6 = (index + 2f64);
                                        {
                                            let beginning = index_6;
                                            {
                                                let source_ml_source1_s5 = crate::source_ml_temporary_read_resource_description_n_triples_token_loop6(units, index_6, beginning);
                                                match source_ml_source1_s5.clone() {
                                                    crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop6Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop6Done(index_7) => {
                                                        {
                                                            let index_8 = crate::source_ml_temporary_read_resource_description_n_triples_token_loop7(units, index_7, beginning);
                                                            if (index_8 == beginning) {
                                                                vec![0f64, start, 3f64]
                                                            } else {
                                                                vec![2f64, index_8, 0f64]
                                                            }
                                                        }
                                                    }
                                                    crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop6Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop6Return(source_ml_temporary_result7) => {
                                                        source_ml_temporary_result7.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if ((code == (60f64)) || (code == (34f64))) {
                                        {
                                            let kind_2 =
                                                if (code == (60f64)) { 1f64 } else { 3f64 };
                                            {
                                                let close =
                                                    if (code == (60f64)) { 62f64 } else { 34f64 };
                                                {
                                                    let index_9 = (index + 1f64);
                                                    {
                                                        let source_ml_source1_s6 = crate::source_ml_temporary_read_resource_description_n_triples_token_loop8(units, index_9, kind_2, close);
                                                        match source_ml_source1_s6.clone() {
                                                            crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop8Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop8Done(index_10) => {
                                                                vec![0f64, start, 7f64]
                                                            }
                                                            crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop8Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop8Return(source_ml_temporary_result8) => {
                                                                source_ml_temporary_result8.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let scalar =
                                                super::read_resource_description_n_triples_scalar(
                                                    units, index, 0f64,
                                                );
                                            vec![
                                                0f64,
                                                index,
                                                if ((super::resource_description_unit(
                                                    &scalar, 2f64,
                                                )) == (0f64))
                                                {
                                                    8f64
                                                } else {
                                                    1f64
                                                },
                                            ]
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
pub enum MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result {
    MlTemporaryAcceptResourceDescriptionUnicodeLoop1Done,
    MlTemporaryAcceptResourceDescriptionUnicodeLoop1Return(bool),
}

pub fn source_ml_temporary_accept_resource_description_unicode_loop1(
    units: &[f64],
    mut index: f64,
) -> crate::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let unit = super::resource_description_unit(units, index);
                if ((!(super::ml_math::is_safe_integer(unit)) || (unit < (0f64)))
                    || (unit > (65535f64)))
                {
                    crate::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Return(false)
                } else {
                    if ((unit >= (55296f64)) && (unit <= (56319f64))) {
                        {
                            let index_2 = (index + 1f64);
                            if (((index_2 >= (units.len() as f64))
                                || ((super::resource_description_unit(units, index_2))
                                    < (56320f64)))
                                || ((super::resource_description_unit(units, index_2))
                                    > (57343f64)))
                            {
                                crate::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Return(false)
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
                    } else {
                        if ((unit >= (56320f64)) && (unit <= (57343f64))) {
                            crate::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Return(false)
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
                }
            }
        } else {
            crate::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Result::MlTemporaryAcceptResourceDescriptionUnicodeLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Result {
    MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Done(f64),
    MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Return(f64),
}

pub fn source_ml_temporary_decode_resource_description_scalar_escape_loop2(
    units: &[f64],
    mut start: f64,
    mut width: f64,
    mut value: f64,
    mut index: f64,
) -> crate::MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Result {
    loop {
        return if (index < (start + width)) {
            {
                let code = super::resource_description_unit(units, index);
                if !(super::resource_description_hexadecimal(code)) {
                    crate::MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Result::MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Return((-1f64))
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
            crate::MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Result::MlTemporaryDecodeResourceDescriptionScalarEscapeLoop2Done(value)
        };
    }
}

pub fn source_ml_temporary_read_resource_description_n_triples_token_loop3(
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Result {
    MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Done(f64),
    MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Return(Vec<f64>),
}

pub fn source_ml_temporary_read_resource_description_n_triples_token_loop4(
    units: &[f64],
    mut index: f64,
) -> crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Result {
    loop {
        return if (((index < (units.len() as f64))
            && ((super::resource_description_unit(units, index)) != (10f64)))
            && ((super::resource_description_unit(units, index)) != (13f64)))
        {
            {
                let scalar = super::read_resource_description_n_triples_scalar(units, index, 0f64);
                if ((super::resource_description_unit(&scalar, 2f64)) != (0f64)) {
                    crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Return(vec![0f64, index, 1f64])
                } else {
                    {
                        let index_2 = (index + super::resource_description_unit(&scalar, 0f64));
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop4Done(index)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadResourceDescriptionNTriplesTokenLoop5Result {
    MlTemporaryReadResourceDescriptionNTriplesTokenLoop5Done(f64, f64),
}

pub fn source_ml_temporary_read_resource_description_n_triples_token_loop5(
    units: &[f64],
    mut index: f64,
    mut code: f64,
) -> crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop5Result {
    loop {
        return if (index < (units.len() as f64)) {
            {
                let code_2 = super::resource_description_unit(units, index);
                if !(((((code_2 >= (65f64)) && (code_2 <= (90f64)))
                    || ((code_2 >= (97f64)) && (code_2 <= (122f64))))
                    || ((code_2 >= (48f64)) && (code_2 <= (57f64))))
                    || (code_2 == (45f64)))
                {
                    crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop5Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop5Done(index, code_2)
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            (index, code) = (index_2, code_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            crate::MlTemporaryReadResourceDescriptionNTriplesTokenLoop5Result::MlTemporaryReadResourceDescriptionNTriplesTokenLoop5Done(index, code)
        };
    }
}

pub fn source_ml_temporary_read_resource_description_n_triples_token_loop7(
    units: &[f64],
    mut index: f64,
    mut beginning: f64,
) -> f64 {
    loop {
        return if ((index > beginning)
            && ((super::resource_description_unit(units, (index - 1f64))) == (46f64)))
        {
            {
                let index_2 = (index - 1f64);
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadResourceDescriptionNTriplesTokenLoop6Result {
    MlTemporaryReadResourceDescriptionNTriplesTokenLoop6Done(f64),
    MlTemporaryReadResourceDescriptionNTriplesTokenLoop6Return(Vec<f64>),
}
