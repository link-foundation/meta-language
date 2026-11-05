// Generated from complete js/src/unsigned-link-identity-rules.js. Do not edit by hand.
use super::accept_unsigned_link_identity;
use super::compare_unsigned_link_identities;
use super::encode_unsigned_link_identity;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_unsigned_link_byte_factor;
use super::read_unsigned_link_identity;
use super::read_unsigned_link_limb;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_unsigned_link_envelope_error(code: f64) -> self::ml_string::Text {
    if code == -1f64 {
        self::ml_string::Text::from_units(vec![
            117, 110, 115, 105, 103, 110, 101, 100, 32, 108, 105, 110, 107, 32, 101, 110, 118, 101,
            108, 111, 112, 101, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 108, 105, 109, 105,
            116,
        ])
    } else if code == -2f64 {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 108,
            105, 110, 107, 32, 97, 108, 108, 111, 99, 97, 116, 111, 114,
        ])
    } else if code == -3f64 {
        self::ml_string::Text::from_units(vec![
            117, 110, 115, 105, 103, 110, 101, 100, 32, 108, 105, 110, 107, 32, 100, 105, 114, 101,
            99, 116, 111, 114, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 115, 116, 114, 105,
            99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100,
        ])
    } else if code == -4f64 {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 108,
            105, 110, 107, 32, 101, 110, 118, 101, 108, 111, 112, 101, 32, 98, 121, 116, 101,
        ])
    } else if code == -5f64 {
        self::ml_string::Text::from_units(vec![
            117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 117, 110, 115, 105, 103,
            110, 101, 100, 32, 108, 105, 110, 107, 32, 101, 110, 118, 101, 108, 111, 112, 101,
        ])
    } else if code == -6f64 {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 108,
            105, 110, 107, 32, 101, 120, 104, 97, 117, 115, 116, 105, 111, 110, 32, 102, 108, 97,
            103,
        ])
    } else if code == -7f64 {
        self::ml_string::Text::from_units(vec![
            110, 111, 110, 99, 97, 110, 111, 110, 105, 99, 97, 108, 32, 101, 120, 104, 97, 117,
            115, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 108, 105, 110, 107,
            32, 97, 108, 108, 111, 99, 97, 116, 111, 114,
        ])
    } else if code == -8f64 {
        self::ml_string::Text::from_units(vec![
            116, 114, 117, 110, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100,
            32, 108, 105, 110, 107, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121,
        ])
    } else if code == -9f64 {
        self::ml_string::Text::from_units(vec![
            117, 110, 115, 105, 103, 110, 101, 100, 32, 108, 105, 110, 107, 32, 101, 110, 118, 101,
            108, 111, 112, 101, 32, 98, 111, 100, 121, 32, 108, 101, 110, 103, 116, 104, 32, 109,
            105, 115, 109, 97, 116, 99, 104,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 108,
            105, 110, 107, 32, 101, 110, 118, 101, 108, 111, 112, 101, 32, 114, 101, 115, 117, 108,
            116,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_unsigned_link_byte_factor_loop1(
    mut exponent: f64,
    mut factor: f64,
    mut index: f64,
) -> f64 {
    loop {
        if index < exponent {
            {
                let factor_2 = factor * 256f64;
                {
                    let index_2 = index + 1f64;
                    {
                        (factor, index) = (factor_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { factor };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryParseUnsignedLinkIdentityLoop2Result {
    MlTemporaryParseUnsignedLinkIdentityLoop2Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_parse_unsigned_link_identity_loop2(
    text: &self::ml_string::Text,
    mut low: f64,
    mut high: f64,
    mut index: f64,
) -> self::MlTemporaryParseUnsignedLinkIdentityLoop2Result {
    loop {
        return if index < (text.length()) {
            {
                let digit = {
                    let source_ml_source1_string_unit_operand10 = text.clone();
                    {
                        let source_ml_source1_string_unit_operand11 = index;
                        (std::rc::Rc::new(
                            move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source1_string_unit_operand17 = {
                                        let source_ml_source1_string_range_operand12 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand13 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand14 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position15 = source_ml_source1_string_range_operand13;
                                                    {
                                                        let source_ml_source1_string_range_position16 = source_ml_source1_string_range_operand14;
                                                        source_ml_source1_string_range_operand12.slice(source_ml_source1_string_range_position15, source_ml_source1_string_range_position16)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand18 = 0f64;
                                        source_ml_source1_string_unit_operand17.code_point_at(source_ml_source1_string_unit_operand18)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand10,
                            source_ml_source1_string_unit_operand11,
                        )
                    }
                } - 48f64;
                if (digit < 0f64) || (digit > 9f64) {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("RangeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            105, 110, 118, 97, 108, 105, 100, 32, 117, 110, 115, 105, 103, 110,
                            101, 100, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105,
                            116, 121,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                } else {
                    {
                        let lower = (low * 10f64) + digit;
                        {
                            let upper = (high * 10f64) + (lower / 4_294_967_296f64).floor();
                            if upper >= 4_294_967_296f64 {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("RangeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        117, 110, 115, 105, 103, 110, 101, 100, 32, 108, 105, 110,
                                        107, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 111,
                                        118, 101, 114, 102, 108, 111, 119,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            } else {
                                {
                                    let low_2 = lower % 4_294_967_296f64;
                                    {
                                        let high_2 = upper;
                                        {
                                            let index_2 = index + 1f64;
                                            {
                                                (low, high, index) = (low_2, high_2, index_2);
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
        } else {
            self::MlTemporaryParseUnsignedLinkIdentityLoop2Result::MlTemporaryParseUnsignedLinkIdentityLoop2Done(low, high)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_render_unsigned_link_identity_loop3(
    mut low: f64,
    mut high: f64,
    mut text: self::ml_string::Text,
) -> self::ml_string::Text {
    loop {
        if (low != 0f64) || (high != 0f64) {
            {
                let upper = (high / 10f64).floor();
                {
                    let remainder = high % 10f64;
                    {
                        let lower = (remainder * 4_294_967_296f64) + low;
                        {
                            let text_2 = (self::ml_string::Text::from_text(
                                &self::ml_number::js_number(lower % 10f64),
                            ))
                            .concat(&text);
                            {
                                let low_2 = (lower / 10f64).floor();
                                {
                                    let high_2 = upper;
                                    {
                                        (low, high, text) = (low_2, high_2, text_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { text.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_encode_unsigned_link_identity_loop5(
    words: &[f64],
    mut bytes: Vec<f64>,
    mut limb: f64,
    mut byte: f64,
) -> Vec<f64> {
    loop {
        if byte < 4f64 {
            {
                let bytes_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            ((self::read_unsigned_link_limb(words, limb)
                                / self::read_unsigned_link_byte_factor(byte))
                            .floor()
                                % 256f64),
                        ],
                        bytes,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let byte_2 = byte + 1f64;
                    {
                        (bytes, byte) = (bytes_2, byte_2);
                        continue;
                    }
                }
            }
        }
        return { bytes };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_encode_unsigned_link_identity_loop4(
    words: &[f64],
    mut bytes: Vec<f64>,
    mut limb: f64,
) -> Vec<f64> {
    loop {
        if limb < 2f64 {
            {
                let byte = 0f64;
                {
                    let bytes_2 = self::source_ml_temporary_encode_unsigned_link_identity_loop5(
                        words, bytes, limb, byte,
                    );
                    {
                        let limb_2 = limb + 1f64;
                        {
                            (bytes, limb) = (bytes_2, limb_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { bytes };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadUnsignedLinkIdentityLoop6Result {
    MlTemporaryReadUnsignedLinkIdentityLoop6Done(f64, f64),
    MlTemporaryReadUnsignedLinkIdentityLoop6Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_unsigned_link_identity_loop6(
    bytes: &[f64],
    mut offset: f64,
    mut low: f64,
    mut high: f64,
    mut index: f64,
) -> self::MlTemporaryReadUnsignedLinkIdentityLoop6Result {
    loop {
        return if index < 8f64 {
            {
                let byte = self::read_unsigned_link_limb(bytes, (offset + index));
                if (!(self::ml_math::is_integer(byte)) || (byte < 0f64)) || (byte > 255f64) {
                    self::MlTemporaryReadUnsignedLinkIdentityLoop6Result::MlTemporaryReadUnsignedLinkIdentityLoop6Return(Vec::<f64>::new())
                } else {
                    if index < 4f64 {
                        {
                            let low_2 = low + (byte * self::read_unsigned_link_byte_factor(index));
                            {
                                let index_2 = index + 1f64;
                                {
                                    (low, index) = (low_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let high_2 =
                                high + (byte * self::read_unsigned_link_byte_factor(index - 4f64));
                            {
                                let index_3 = index + 1f64;
                                {
                                    (high, index) = (high_2, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadUnsignedLinkIdentityLoop6Result::MlTemporaryReadUnsignedLinkIdentityLoop6Done(low, high)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_encode_unsigned_link_envelope_loop10(
    mut output: Vec<f64>,
    source_ml_temporary_values7: &[f64],
    mut source_ml_temporary_index7: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_temporary_index7
            < (self::ml::Big::from_u128(source_ml_temporary_values7.len() as u128))
        {
            {
                let byte = self::ml_array::at(
                    source_ml_temporary_values7,
                    source_ml_temporary_index7.to_index(),
                );
                {
                    let output_2 = {
                        let (ml_append_right, ml_append_left) = (vec![byte], output);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let source_ml_temporary_index7_2 =
                            source_ml_temporary_index7.add(&self::ml::Big::from_i128(1));
                        {
                            (output, source_ml_temporary_index7) =
                                (output_2, source_ml_temporary_index7_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { output };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_encode_unsigned_link_envelope_loop11(
    identities: &[f64],
    mut output: Vec<f64>,
    mut byte: f64,
) -> Vec<f64> {
    loop {
        if byte < 4f64 {
            {
                let output_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            ((((identities.len() as f64) / 2f64)
                                / self::read_unsigned_link_byte_factor(byte))
                            .floor()
                                % 256f64),
                        ],
                        output,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let byte_2 = byte + 1f64;
                    {
                        (output, byte) = (output_2, byte_2);
                        continue;
                    }
                }
            }
        }
        return { output };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_encode_unsigned_link_envelope_loop13(
    body: &[f64],
    mut output: Vec<f64>,
    mut byte: f64,
) -> Vec<f64> {
    loop {
        if byte < 4f64 {
            {
                let output_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (((body.len() as f64) / self::read_unsigned_link_byte_factor(byte))
                                .floor()
                                % 256f64),
                        ],
                        output,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let byte_2 = byte + 1f64;
                    {
                        (output, byte) = (output_2, byte_2);
                        continue;
                    }
                }
            }
        }
        return { output };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Result {
    MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Done(Vec<f64>),
    MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_encode_unsigned_link_envelope_loop14(
    mut output: Vec<f64>,
    source_ml_temporary_values9: &[f64],
    mut source_ml_temporary_index9: self::ml::Big,
) -> self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Result {
    loop {
        return if source_ml_temporary_index9
            < (self::ml::Big::from_u128(source_ml_temporary_values9.len() as u128))
        {
            {
                let byte = self::ml_array::at(
                    source_ml_temporary_values9,
                    source_ml_temporary_index9.to_index(),
                );
                if (!(self::ml_math::is_integer(byte)) || (byte < 0f64)) || (byte > 255f64) {
                    self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Result::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Return(vec![-4f64])
                } else {
                    {
                        let output_2 = {
                            let (ml_append_right, ml_append_left) = (vec![byte], output);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let source_ml_temporary_index9_2 =
                                source_ml_temporary_index9.add(&self::ml::Big::from_i128(1));
                            {
                                (output, source_ml_temporary_index9) =
                                    (output_2, source_ml_temporary_index9_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Result::MlTemporaryEncodeUnsignedLinkEnvelopeLoop14Done(output)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Result {
    MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Done(Vec<f64>),
    MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_encode_unsigned_link_envelope_loop15(
    mut output: Vec<f64>,
    source_ml_temporary_values8: &[f64],
    mut source_ml_temporary_index8: self::ml::Big,
) -> Vec<f64> {
    loop {
        if source_ml_temporary_index8
            < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))
        {
            {
                let byte = self::ml_array::at(
                    source_ml_temporary_values8,
                    source_ml_temporary_index8.to_index(),
                );
                {
                    let output_2 = {
                        let (ml_append_right, ml_append_left) = (vec![byte], output);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let source_ml_temporary_index8_2 =
                            source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                        {
                            (output, source_ml_temporary_index8) =
                                (output_2, source_ml_temporary_index8_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { output };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_encode_unsigned_link_envelope_loop12(
    identities: &[f64],
    mut output: Vec<f64>,
    mut offset: f64,
) -> self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Result {
    loop {
        return if offset < (identities.len() as f64) {
            {
                let words = vec![
                    self::read_unsigned_link_limb(identities, offset),
                    self::read_unsigned_link_limb(identities, (offset + 1f64)),
                ];
                if !(self::accept_unsigned_link_identity(&(words)))
                    || ((offset > 0f64)
                        && ((self::compare_unsigned_link_identities(
                            &(vec![
                                self::read_unsigned_link_limb(identities, (offset - 2f64)),
                                self::read_unsigned_link_limb(identities, (offset - 1f64)),
                            ]),
                            &(words),
                        )) >= 0f64))
                {
                    self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Result::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Return(vec![-3f64])
                } else {
                    {
                        let source_ml_temporary_values8 =
                            self::encode_unsigned_link_identity(&(words));
                        {
                            let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                            {
                                let output_2 =
                                    self::source_ml_temporary_encode_unsigned_link_envelope_loop15(
                                        output,
                                        &(source_ml_temporary_values8),
                                        source_ml_temporary_index8,
                                    );
                                {
                                    let offset_2 = offset + 2f64;
                                    {
                                        (output, offset) = (output_2, offset_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Result::MlTemporaryEncodeUnsignedLinkEnvelopeLoop12Done(output)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Result {
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Done(Vec<f64>),
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_decode_unsigned_link_envelope_loop20(
    input: &[f64],
    mut output: Vec<f64>,
    mut at: f64,
) -> self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Result {
    loop {
        return if at < (input.len() as f64) {
            {
                let byte = self::read_unsigned_link_limb(input, at);
                if (!(self::ml_math::is_integer(byte)) || (byte < 0f64)) || (byte > 255f64) {
                    self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Return(vec![-4f64])
                } else {
                    {
                        let output_2 = {
                            let (ml_append_right, ml_append_left) = (vec![byte], output);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (output, at) = (output_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop20Done(output)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Result {
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Done(f64),
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_decode_unsigned_link_envelope_loop19(
    input: &[f64],
    mut position: f64,
    mut length: f64,
    mut byte: f64,
) -> self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Result {
    loop {
        return if byte < 4f64 {
            {
                let value = self::read_unsigned_link_limb(input, (position + byte));
                if (!(self::ml_math::is_integer(value)) || (value < 0f64)) || (value > 255f64) {
                    self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Return(vec![-4f64])
                } else {
                    {
                        let length_2 =
                            length + (value * self::read_unsigned_link_byte_factor(byte));
                        {
                            let byte_2 = byte + 1f64;
                            {
                                (length, byte) = (length_2, byte_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop19Done(length)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Result {
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Done(Vec<f64>),
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_decode_unsigned_link_envelope_loop18(
    input: &[f64],
    mut count: f64,
    mut output: Vec<f64>,
    mut previous: Vec<f64>,
    mut index: f64,
) -> self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Result {
    loop {
        return if index < count {
            {
                let words = self::read_unsigned_link_identity(input, (21f64 + (index * 8f64)));
                if (words.len() as f64) == 2f64 {
                    if ((previous.len() as f64) != 0f64)
                        && ((self::compare_unsigned_link_identities(&(previous), &(words))) >= 0f64)
                    {
                        self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Return(vec![-3f64])
                    } else {
                        {
                            let output_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        self::read_unsigned_link_limb(&(words), 0f64),
                                        self::read_unsigned_link_limb(&(words), 1f64),
                                    ],
                                    output,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let previous_2 = words;
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (output, previous, index) = (output_2, previous_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Return(vec![-3f64])
                }
            }
        } else {
            self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop18Done(output)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Result {
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Done(f64),
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_decode_unsigned_link_envelope_loop17(
    input: &[f64],
    mut count: f64,
    mut byte: f64,
) -> self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Result {
    loop {
        return if byte < 4f64 {
            {
                let value = self::read_unsigned_link_limb(input, (17f64 + byte));
                if (!(self::ml_math::is_integer(value)) || (value < 0f64)) || (value > 255f64) {
                    self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Return(vec![-4f64])
                } else {
                    {
                        let count_2 = count + (value * self::read_unsigned_link_byte_factor(byte));
                        {
                            let byte_2 = byte + 1f64;
                            {
                                (count, byte) = (count_2, byte_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop17Done(count)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Result {
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Done,
    MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_decode_unsigned_link_envelope_loop16(
    input: &[f64],
    header: &[f64],
    mut index: f64,
) -> self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Result {
    loop {
        return if index < 8f64 {
            if (self::read_unsigned_link_limb(input, index))
                == (self::read_unsigned_link_limb(header, index))
            {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Return(vec![-5f64])
            }
        } else {
            self::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Result::MlTemporaryDecodeUnsignedLinkEnvelopeLoop16Done
        };
    }
}
