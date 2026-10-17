// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::RustCharacterLiteral;
use super::RustUnicodeEscape;
use super::accept_rust_cooked_string_escape_prefix;
use super::decode_rust_unicode_escape;
use super::finish_rust_character_literal;
use super::ml_array;
use super::ml_math;
use super::read_rust_ascii_escape_digit;
use super::read_rust_ascii_escape_value;
use super::read_rust_character_byte_escape_value;
use super::read_rust_character_simple_escape_value;
use super::read_source_text_scalar;
use super::read_source_text_scalar_byte_width;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decode_rust_character_literal(units: &[f64], start: f64) -> self::RustCharacterLiteral {
    if (!(self::ml_math::is_integer(start)) || (start < 0f64)) || (start >= (units.len() as f64)) {
        self::RustCharacterLiteral::NotCharacter
    } else {
        {
            let byte = match (
                self::ml_array::optional_at(units, self::ml_array::number_index(start)),
                98f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } && match (
                self::ml_array::optional_at(units, self::ml_array::number_index(start + 1f64)),
                39f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            };
            if !byte
                && !(match (
                    self::ml_array::optional_at(units, self::ml_array::number_index(start)),
                    39f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })
            {
                self::RustCharacterLiteral::NotCharacter
            } else {
                {
                    let content_start = start + if byte { 2f64 } else { 1f64 };
                    if content_start >= (units.len() as f64) {
                        self::RustCharacterLiteral::Unterminated(start, (units.len() as f64))
                    } else {
                        {
                            let first = (self::ml_array::optional_at(
                                units,
                                self::ml_array::number_index(content_start),
                            ))
                            .unwrap_or_else(|| -1f64);
                            if (((first == 39f64) || (first == 9f64)) || (first == 10f64))
                                || (first == 13f64)
                            {
                                self::RustCharacterLiteral::Malformed(
                                    content_start,
                                    (content_start + 1f64),
                                )
                            } else {
                                let ml_library_inequality_1 = first != 92f64;
                                if ml_library_inequality_1 {
                                    {
                                        let scalar =
                                            self::read_source_text_scalar(units, content_start);
                                        if scalar < 0f64 {
                                            self::RustCharacterLiteral::NonScalar(
                                                content_start,
                                                (content_start + 1f64),
                                            )
                                        } else {
                                            {
                                                let content_end = content_start
                                                    + if scalar >= 65_536f64 { 2f64 } else { 1f64 };
                                                if !byte
                                                    && !(match (
                                                        self::ml_array::optional_at(
                                                            units,
                                                            self::ml_array::number_index(
                                                                content_end,
                                                            ),
                                                        ),
                                                        39f64,
                                                    ) {
                                                        (Some(ml_left), ml_right) => {
                                                            ml_left == ml_right
                                                        }
                                                        _ => false,
                                                    })
                                                {
                                                    self::RustCharacterLiteral::NotCharacter
                                                } else if byte && (scalar > 127f64) {
                                                    self::RustCharacterLiteral::NonAscii(
                                                        content_start,
                                                        content_end,
                                                    )
                                                } else {
                                                    self::finish_rust_character_literal(
                                                        units,
                                                        byte,
                                                        scalar,
                                                        content_start,
                                                        content_end,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let escaped = (self::ml_array::optional_at(
                                            units,
                                            self::ml_array::number_index(content_start + 1f64),
                                        ))
                                        .unwrap_or_else(|| -1f64);
                                        if (self::accept_rust_cooked_string_escape_prefix(escaped))
                                        {
                                            if escaped == 120f64 {
                                                {
                                                    let high = (self::ml_array::optional_at(
                                                        units,
                                                        self::ml_array::number_index(
                                                            content_start + 2f64,
                                                        ),
                                                    ))
                                                    .unwrap_or_else(|| -1f64);
                                                    {
                                                        let low = (self::ml_array::optional_at(
                                                            units,
                                                            self::ml_array::number_index(
                                                                content_start + 3f64,
                                                            ),
                                                        ))
                                                        .unwrap_or_else(|| -1f64);
                                                        {
                                                            let value = if byte {
                                                                self::read_rust_character_byte_escape_value(high, low)
                                                            } else {
                                                                self::read_rust_ascii_escape_value(
                                                                    high, low,
                                                                )
                                                            };
                                                            if value < 0f64 {
                                                                self::RustCharacterLiteral::Malformed(content_start, self::ml_math::min((content_start + 4f64), (units.len() as f64)))
                                                            } else {
                                                                self::finish_rust_character_literal(
                                                                    units,
                                                                    byte,
                                                                    value,
                                                                    content_start,
                                                                    (content_start + 4f64),
                                                                )
                                                            }
                                                        }
                                                    }
                                                }
                                            } else if escaped == 117f64 {
                                                if byte {
                                                    self::RustCharacterLiteral::UnicodeByteEscape(
                                                        content_start,
                                                        self::ml_math::min(
                                                            (content_start + 2f64),
                                                            (units.len() as f64),
                                                        ),
                                                    )
                                                } else {
                                                    {
                                                        let scalar_2 =
                                                            self::decode_rust_unicode_escape(
                                                                units,
                                                                content_start,
                                                            );
                                                        match scalar_2 {
                                                                self::RustUnicodeEscape::Scalar(code, end) => {
                                                                    self::finish_rust_character_literal(units, false, code, content_start, (content_start + end))
                                                                }
                                                                self::RustUnicodeEscape::Malformed => {
                                                                    self::RustCharacterLiteral::Malformed(content_start, self::ml_math::min((content_start + 2f64), (units.len() as f64)))
                                                                }
                                                                self::RustUnicodeEscape::Unsupported(end_2, ml_f5) => {
                                                                    self::RustCharacterLiteral::NonScalar(content_start, (content_start + end_2))
                                                                }
                                                            }
                                                    }
                                                }
                                            } else {
                                                {
                                                    let value_2 = self::read_rust_character_simple_escape_value(escaped);
                                                    if value_2 < 0f64 {
                                                        self::RustCharacterLiteral::Malformed(
                                                            content_start,
                                                            self::ml_math::min(
                                                                (content_start + 2f64),
                                                                (units.len() as f64),
                                                            ),
                                                        )
                                                    } else {
                                                        self::finish_rust_character_literal(
                                                            units,
                                                            byte,
                                                            value_2,
                                                            content_start,
                                                            (content_start + 2f64),
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            self::RustCharacterLiteral::Malformed(
                                                content_start,
                                                self::ml_math::min(
                                                    (content_start + 2f64),
                                                    (units.len() as f64),
                                                ),
                                            )
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadSourceTextByteLengthLoop1Result {
    MlReadSourceTextByteLengthLoop1Done(f64),
    MlReadSourceTextByteLengthLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_source_text_byte_length_loop1(
    units: &[f64],
    mut index: f64,
    mut bytes: f64,
) -> self::MlReadSourceTextByteLengthLoop1Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let scalar = self::read_source_text_scalar(units, index);
                {
                    let width = self::read_source_text_scalar_byte_width(scalar);
                    if width < 0f64 {
                        self::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Return(-1f64)
                    } else {
                        {
                            let bytes_2 = bytes + width;
                            {
                                let index_2 = index + if scalar >= 65_536f64 { 2f64 } else { 1f64 };
                                {
                                    (index, bytes) = (index_2, bytes_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Done(bytes)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadSourceTextCodeUnitOffsetLoop2Result {
    MlReadSourceTextCodeUnitOffsetLoop2Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_source_text_code_unit_offset_loop2(
    units: &[f64],
    mut offset: f64,
    mut index: f64,
    mut bytes: f64,
) -> self::MlReadSourceTextCodeUnitOffsetLoop2Result {
    loop {
        if bytes < offset {
            {
                let scalar = self::read_source_text_scalar(units, index);
                {
                    let bytes_2 = bytes + self::read_source_text_scalar_byte_width(scalar);
                    {
                        let index_2 = index + if scalar >= 65_536f64 { 2f64 } else { 1f64 };
                        {
                            (index, bytes) = (index_2, bytes_2);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlReadSourceTextCodeUnitOffsetLoop2Result::MlReadSourceTextCodeUnitOffsetLoop2Done(
                index, bytes,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadSourceTextByteLoop3Result {
    MlReadSourceTextByteLoop3Done,
    MlReadSourceTextByteLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_source_text_byte_loop3(
    units: &[f64],
    mut offset: f64,
    mut index: f64,
    mut bytes: f64,
) -> self::MlReadSourceTextByteLoop3Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let scalar = self::read_source_text_scalar(units, index);
                {
                    let width = self::read_source_text_scalar_byte_width(scalar);
                    {
                        let position = offset - bytes;
                        if position < width {
                            if width == 1f64 {
                                self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(scalar)
                            } else if position == 0f64 {
                                if width == 2f64 {
                                    self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(192f64 + (scalar / 64f64).floor())
                                } else if width == 3f64 {
                                    self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(224f64 + (scalar / 4096f64).floor())
                                } else {
                                    self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(240f64 + (scalar / 262_144f64).floor())
                                }
                            } else if position == (width - 1f64) {
                                self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(128f64 + (scalar % 64f64))
                            } else if (width == 4f64) && (position == 1f64) {
                                self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(128f64 + ((scalar / 4096f64).floor() % 64f64))
                            } else {
                                self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(128f64 + ((scalar / 64f64).floor() % 64f64))
                            }
                        } else {
                            {
                                let bytes_2 = bytes + width;
                                {
                                    let index_2 =
                                        index + if scalar >= 65_536f64 { 2f64 } else { 1f64 };
                                    {
                                        (index, bytes) = (index_2, bytes_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeRustUnicodeEscapeLoop4Result {
    MlDecodeRustUnicodeEscapeLoop4Done,
    MlDecodeRustUnicodeEscapeLoop4Return(Box<self::RustUnicodeEscape>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_decode_rust_unicode_escape_loop4(
    units: &[f64],
    mut start: f64,
    mut index: f64,
    mut code: f64,
    mut digits: f64,
) -> self::MlDecodeRustUnicodeEscapeLoop4Result {
    loop {
        return if index < (units.len() as f64) {
            if match (
                self::ml_array::optional_at(units, self::ml_array::number_index(index)),
                125f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } {
                if digits < 1f64 {
                    self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Return(
                        Box::new(self::RustUnicodeEscape::Malformed),
                    )
                } else {
                    {
                        let end = (index + 1f64) - start;
                        if (code > 1_114_111f64) || ((55_296f64..=57_343f64).contains(&code)) {
                            self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Return(Box::new(self::RustUnicodeEscape::Unsupported(end, end)))
                        } else {
                            self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Return(Box::new(self::RustUnicodeEscape::Scalar(code, end)))
                        }
                    }
                }
            } else if match (
                self::ml_array::optional_at(units, self::ml_array::number_index(index)),
                95f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } {
                if digits < 1f64 {
                    self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Return(
                        Box::new(self::RustUnicodeEscape::Malformed),
                    )
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            } else {
                {
                    let digit = self::read_rust_ascii_escape_digit(
                        (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                            .unwrap_or_else(|| -1f64),
                    );
                    if (digit < 0f64) || (digits >= 6f64) {
                        self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Return(Box::new(self::RustUnicodeEscape::Malformed))
                    } else {
                        {
                            let code_2 = (code * 16f64) + digit;
                            {
                                let digits_2 = digits + 1f64;
                                {
                                    let index_3 = index + 1f64;
                                    {
                                        (index, code, digits) = (index_3, code_2, digits_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Done
        };
    }
}
