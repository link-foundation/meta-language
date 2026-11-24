// Generated from complete js/src/translation/rust-byte-string-rules.js. Do not edit by hand.
use super::RustUnicodeEscape;
use super::accept_rust_string_continuation_whitespace;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_rust_ascii_escape_digit;
use super::read_rust_character_byte_escape_value;
use super::read_rust_character_simple_escape_value;
use super::read_rust_source_line_break_end;
use super::read_rust_string_continuation_end;
use super::read_source_text_scalar;
use super::read_source_text_scalar_byte_width;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decode_rust_byte_string_literal(units: &[f64], start: f64) -> Vec<f64> {
    if ((!(self::ml_math::is_integer(start)) || (start < 0f64)) || (start >= (units.len() as f64)))
        || {
            let source_ml_source1_comparison_operand42 =
                self::ml_array::optional_at(units, self::ml_array::number_index(start));
            {
                let source_ml_source1_comparison_operand43 = 98f64;
                !(match (
                    source_ml_source1_comparison_operand42,
                    source_ml_source1_comparison_operand43,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })
            }
        }
    {
        vec![0f64]
    } else {
        {
            let raw_ = {
                let source_ml_source1_comparison_operand44 =
                    self::ml_array::optional_at(units, self::ml_array::number_index(start + 1f64));
                {
                    let source_ml_source1_comparison_operand45 = 114f64;
                    match (
                        source_ml_source1_comparison_operand44,
                        source_ml_source1_comparison_operand45,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            };
            {
                let opening = start + if raw_ { 2f64 } else { 1f64 };
                {
                    let opening_3 = if raw_ {
                        {
                            self::source_ml_temporary_decode_rust_byte_string_literal_loop6(
                                units, opening,
                            )
                        }
                    } else {
                        opening
                    };
                    if {
                        let source_ml_source1_comparison_operand46 = self::ml_array::optional_at(
                            units,
                            self::ml_array::number_index(opening_3),
                        );
                        {
                            let source_ml_source1_comparison_operand47 = 34f64;
                            !(match (
                                source_ml_source1_comparison_operand46,
                                source_ml_source1_comparison_operand47,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    } {
                        vec![0f64]
                    } else {
                        {
                            let hashes = if raw_ {
                                (opening_3 - start) - 2f64
                            } else {
                                0f64
                            };
                            if hashes > 255f64 {
                                vec![-7f64, start, (opening_3 + 1f64)]
                            } else {
                                {
                                    let position = opening_3 + 1f64;
                                    {
                                        let bytes = Vec::<f64>::new();
                                        {
                                            let source_ml_source1_s48 = self::source_ml_temporary_decode_rust_byte_string_literal_loop7(units, raw_, hashes, position, bytes);
                                            match source_ml_source1_s48 {
                                                self::MlTemporaryDecodeRustByteStringLiteralLoop7Result::MlTemporaryDecodeRustByteStringLiteralLoop7Done => {
                                                    vec![-2f64, start, (units.len() as f64)]
                                                }
                                                self::MlTemporaryDecodeRustByteStringLiteralLoop7Result::MlTemporaryDecodeRustByteStringLiteralLoop7Return(source_ml_temporary_result7) => {
                                                    source_ml_temporary_result7
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
pub fn read_rust_byte_string_literal_error(code: f64) -> self::ml_string::Text {
    if code == -2f64 {
        self::ml_string::Text::from_units(vec![
            117, 110, 116, 101, 114, 109, 105, 110, 97, 116, 101, 100, 32, 82, 117, 115, 116, 32,
            98, 121, 116, 101, 32, 115, 116, 114, 105, 110, 103, 32, 108, 105, 116, 101, 114, 97,
            108,
        ])
    } else if code == -3f64 {
        self::ml_string::Text::from_units(vec![
            110, 111, 110, 45, 65, 83, 67, 73, 73, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110,
            32, 97, 32, 82, 117, 115, 116, 32, 98, 121, 116, 101, 32, 115, 116, 114, 105, 110, 103,
            32, 108, 105, 116, 101, 114, 97, 108,
        ])
    } else if code == -4f64 {
        self::ml_string::Text::from_units(vec![
            105, 115, 111, 108, 97, 116, 101, 100, 32, 99, 97, 114, 114, 105, 97, 103, 101, 32,
            114, 101, 116, 117, 114, 110, 32, 105, 110, 32, 97, 32, 82, 117, 115, 116, 32, 98, 121,
            116, 101, 32, 115, 116, 114, 105, 110, 103, 32, 108, 105, 116, 101, 114, 97, 108,
        ])
    } else if code == -5f64 {
        self::ml_string::Text::from_units(vec![
            85, 110, 105, 99, 111, 100, 101, 32, 101, 115, 99, 97, 112, 101, 32, 105, 110, 32, 97,
            32, 82, 117, 115, 116, 32, 98, 121, 116, 101, 32, 115, 116, 114, 105, 110, 103, 32,
            108, 105, 116, 101, 114, 97, 108,
        ])
    } else if code == -6f64 {
        self::ml_string::Text::from_units(vec![
            109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 82, 117, 115, 116, 32, 98, 121, 116,
            101, 32, 115, 116, 114, 105, 110, 103, 32, 101, 115, 99, 97, 112, 101,
        ])
    } else if code == -7f64 {
        self::ml_string::Text::from_units(vec![
            82, 117, 115, 116, 32, 114, 97, 119, 32, 98, 121, 116, 101, 32, 115, 116, 114, 105,
            110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 50, 53, 53, 32, 100, 101, 108, 105,
            109, 105, 116, 101, 114, 32, 104, 97, 115, 104, 101, 115,
        ])
    } else {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116, 32, 98, 121, 116, 101, 32,
            115, 116, 114, 105, 110, 103, 32, 116, 111, 107, 101, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_byte_string_value(bytes: &[f64]) -> self::ml_string::Text {
    {
        let text = self::ml_string::Text::from_units(vec![]);
        {
            let index = 0f64;
            { self::source_ml_temporary_render_rust_byte_string_value_loop10(bytes, text, index) }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadSourceTextByteLengthLoop1Result {
    MlTemporaryReadSourceTextByteLengthLoop1Done(f64),
    MlTemporaryReadSourceTextByteLengthLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_source_text_byte_length_loop1(
    units: &[f64],
    mut index: f64,
    mut bytes: f64,
) -> self::MlTemporaryReadSourceTextByteLengthLoop1Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let scalar = self::read_source_text_scalar(units, index);
                {
                    let width = self::read_source_text_scalar_byte_width(scalar);
                    if width < 0f64 {
                        self::MlTemporaryReadSourceTextByteLengthLoop1Result::MlTemporaryReadSourceTextByteLengthLoop1Return(-1f64)
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
            self::MlTemporaryReadSourceTextByteLengthLoop1Result::MlTemporaryReadSourceTextByteLengthLoop1Done(bytes)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadSourceTextCodeUnitOffsetLoop2Result {
    MlTemporaryReadSourceTextCodeUnitOffsetLoop2Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_source_text_code_unit_offset_loop2(
    units: &[f64],
    mut offset: f64,
    mut index: f64,
    mut bytes: f64,
) -> self::MlTemporaryReadSourceTextCodeUnitOffsetLoop2Result {
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
            self::MlTemporaryReadSourceTextCodeUnitOffsetLoop2Result::MlTemporaryReadSourceTextCodeUnitOffsetLoop2Done(index, bytes)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadSourceTextByteLoop3Result {
    MlTemporaryReadSourceTextByteLoop3Done,
    MlTemporaryReadSourceTextByteLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_source_text_byte_loop3(
    units: &[f64],
    mut offset: f64,
    mut index: f64,
    mut bytes: f64,
) -> self::MlTemporaryReadSourceTextByteLoop3Result {
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
                                self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Return(scalar)
                            } else if position == 0f64 {
                                if width == 2f64 {
                                    self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Return(192f64 + (scalar / 64f64).floor())
                                } else if width == 3f64 {
                                    self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Return(224f64 + (scalar / 4096f64).floor())
                                } else {
                                    self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Return(240f64 + (scalar / 262_144f64).floor())
                                }
                            } else if position == (width - 1f64) {
                                self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Return(128f64 + (scalar % 64f64))
                            } else if (width == 4f64) && (position == 1f64) {
                                self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Return(128f64 + ((scalar / 4096f64).floor() % 64f64))
                            } else {
                                self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Return(128f64 + ((scalar / 64f64).floor() % 64f64))
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
            self::MlTemporaryReadSourceTextByteLoop3Result::MlTemporaryReadSourceTextByteLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeRustUnicodeEscapeLoop4Result {
    MlTemporaryDecodeRustUnicodeEscapeLoop4Done,
    MlTemporaryDecodeRustUnicodeEscapeLoop4Return(Box<self::RustUnicodeEscape>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_decode_rust_unicode_escape_loop4(
    units: &[f64],
    mut start: f64,
    mut index: f64,
    mut code: f64,
    mut digits: f64,
) -> self::MlTemporaryDecodeRustUnicodeEscapeLoop4Result {
    loop {
        return if index < (units.len() as f64) {
            if {
                let source_ml_source1_comparison_operand49 =
                    self::ml_array::optional_at(units, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand50 = 125f64;
                    match (
                        source_ml_source1_comparison_operand49,
                        source_ml_source1_comparison_operand50,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } {
                if digits < 1f64 {
                    self::MlTemporaryDecodeRustUnicodeEscapeLoop4Result::MlTemporaryDecodeRustUnicodeEscapeLoop4Return(Box::new(self::RustUnicodeEscape::Malformed))
                } else {
                    {
                        let end = (index + 1f64) - start;
                        if (code > 1_114_111f64) || ((55_296f64..=57_343f64).contains(&code)) {
                            self::MlTemporaryDecodeRustUnicodeEscapeLoop4Result::MlTemporaryDecodeRustUnicodeEscapeLoop4Return(Box::new(self::RustUnicodeEscape::Unsupported(end, end)))
                        } else {
                            self::MlTemporaryDecodeRustUnicodeEscapeLoop4Result::MlTemporaryDecodeRustUnicodeEscapeLoop4Return(Box::new(self::RustUnicodeEscape::Scalar(code, end)))
                        }
                    }
                }
            } else if {
                let source_ml_source1_comparison_operand51 =
                    self::ml_array::optional_at(units, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand52 = 95f64;
                    match (
                        source_ml_source1_comparison_operand51,
                        source_ml_source1_comparison_operand52,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } {
                if digits < 1f64 {
                    self::MlTemporaryDecodeRustUnicodeEscapeLoop4Result::MlTemporaryDecodeRustUnicodeEscapeLoop4Return(Box::new(self::RustUnicodeEscape::Malformed))
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
                        self::MlTemporaryDecodeRustUnicodeEscapeLoop4Result::MlTemporaryDecodeRustUnicodeEscapeLoop4Return(Box::new(self::RustUnicodeEscape::Malformed))
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
            self::MlTemporaryDecodeRustUnicodeEscapeLoop4Result::MlTemporaryDecodeRustUnicodeEscapeLoop4Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_string_continuation_end_loop5(
    units: &[f64],
    mut end: f64,
) -> f64 {
    loop {
        if (end < (units.len() as f64))
            && self::accept_rust_string_continuation_whitespace(self::ml_array::optional_at(
                units,
                self::ml_array::number_index(end),
            ))
        {
            {
                let end_2 = end + 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        }
        return { end };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_decode_rust_byte_string_literal_loop6(
    units: &[f64],
    mut opening: f64,
) -> f64 {
    loop {
        return if (opening < (units.len() as f64)) && {
            let source_ml_source1_comparison_operand53 =
                self::ml_array::optional_at(units, self::ml_array::number_index(opening));
            {
                let source_ml_source1_comparison_operand54 = 35f64;
                match (
                    source_ml_source1_comparison_operand53,
                    source_ml_source1_comparison_operand54,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } {
            {
                let opening_2 = opening + 1f64;
                {
                    opening = opening_2;
                    continue;
                }
            }
        } else {
            opening
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeRustByteStringLiteralLoop7Result {
    MlTemporaryDecodeRustByteStringLiteralLoop7Done,
    MlTemporaryDecodeRustByteStringLiteralLoop7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeRustByteStringLiteralJoin8 {
    MlTemporaryDecodeRustByteStringLiteralJoin8Next,
    MlTemporaryDecodeRustByteStringLiteralJoin8Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_decode_rust_byte_string_literal_loop9(
    units: &[f64],
    mut hashes: f64,
    mut position: f64,
    mut matches: bool,
    mut count: f64,
) -> bool {
    loop {
        return if count < hashes {
            if {
                let source_ml_source1_comparison_operand55 = self::ml_array::optional_at(
                    units,
                    self::ml_array::number_index((position + 1f64) + count),
                );
                {
                    let source_ml_source1_comparison_operand56 = 35f64;
                    !(match (
                        source_ml_source1_comparison_operand55,
                        source_ml_source1_comparison_operand56,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                {
                    let matches_2 = false;
                    {
                        let count_2 = count + 1f64;
                        {
                            (matches, count) = (matches_2, count_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let count_3 = count + 1f64;
                    {
                        count = count_3;
                        continue;
                    }
                }
            }
        } else {
            matches
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_decode_rust_byte_string_literal_loop7(
    units: &[f64],
    mut raw_: bool,
    mut hashes: f64,
    mut position: f64,
    mut bytes: Vec<f64>,
) -> self::MlTemporaryDecodeRustByteStringLiteralLoop7Result {
    loop {
        return if position < (units.len() as f64) {
            {
                let unit =
                    (self::ml_array::optional_at(units, self::ml_array::number_index(position)))
                        .unwrap_or_else(|| -1f64);
                {
                    let source_ml_source1_s59 = if unit == 34f64 {
                        {
                            let matches = true;
                            {
                                let count = 0f64;
                                {
                                    let matches_2 = self::source_ml_temporary_decode_rust_byte_string_literal_loop9(units, hashes, position, matches, count);
                                    if matches_2 {
                                        self::MlTemporaryDecodeRustByteStringLiteralJoin8::MlTemporaryDecodeRustByteStringLiteralJoin8Return({
                                            let source_ml_source1_concat_value60 = vec![1f64, ((position + 1f64) + hashes)];
                                            {
                                                let source_ml_source1_concat_value61 = bytes.clone();
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value60), source_ml_source1_concat_value61)
                                            }
                                        })
                                    } else {
                                        self::MlTemporaryDecodeRustByteStringLiteralJoin8::MlTemporaryDecodeRustByteStringLiteralJoin8Next
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlTemporaryDecodeRustByteStringLiteralJoin8::MlTemporaryDecodeRustByteStringLiteralJoin8Next
                    };
                    match source_ml_source1_s59 {
                        self::MlTemporaryDecodeRustByteStringLiteralJoin8::MlTemporaryDecodeRustByteStringLiteralJoin8Next => {
                            if !raw_ && (unit == 92f64) {
                                {
                                    let continuation = self::read_rust_string_continuation_end(units, position);
                                    if continuation >= 0f64 {
                                        {
                                            let position_2 = continuation;
                                            {
                                                position = position_2;
                                                continue;
                                            }
                                        }
                                    }
{
                                        {
                                            let escaped = (self::ml_array::optional_at(units, self::ml_array::number_index(position + 1f64))).unwrap_or_else(|| -1f64);
                                            if escaped == 117f64 {
                                                self::MlTemporaryDecodeRustByteStringLiteralLoop7Result::MlTemporaryDecodeRustByteStringLiteralLoop7Return(vec![-5f64, position, self::ml_math::min((position + 2f64), (units.len() as f64))])
                                            } else {
                                                {
                                                    let value = if escaped == 120f64 {
                                                        self::read_rust_character_byte_escape_value((self::ml_array::optional_at(units, self::ml_array::number_index(position + 2f64))).unwrap_or_else(|| -1f64), (self::ml_array::optional_at(units, self::ml_array::number_index(position + 3f64))).unwrap_or_else(|| -1f64))
                                                    } else {
                                                        self::read_rust_character_simple_escape_value(escaped)
                                                    };
                                                    if value < 0f64 {
                                                        self::MlTemporaryDecodeRustByteStringLiteralLoop7Result::MlTemporaryDecodeRustByteStringLiteralLoop7Return(vec![-6f64, position, self::ml_math::min((position + if escaped == 120f64 {
                                                            4f64
                                                        } else {
                                                            2f64
                                                        }), (units.len() as f64))])
                                                    } else {
                                                        {
                                                            let bytes_2 = {
                                                                let source_ml_source1_concat_value62 = bytes;
                                                                {
                                                                    let source_ml_source1_concat_value63 = vec![value];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value62), source_ml_source1_concat_value63)
                                                                }
                                                            };
                                                            {
                                                                let position_3 = position + if escaped == 120f64 {
                                                                    4f64
                                                                } else {
                                                                    2f64
                                                                };
                                                                {
                                                                    (position, bytes) = (position_3, bytes_2);
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
                                {
                                    let line_break = self::read_rust_source_line_break_end(units, position);
                                    if line_break >= 0f64 {
                                        {
                                            let bytes_3 = {
                                                let source_ml_source1_concat_value64 = bytes;
                                                {
                                                    let source_ml_source1_concat_value65 = vec![10f64];
                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value64), source_ml_source1_concat_value65)
                                                }
                                            };
                                            {
                                                let position_4 = line_break;
                                                {
                                                    (position, bytes) = (position_4, bytes_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
{
                                        if unit == 13f64 {
                                            self::MlTemporaryDecodeRustByteStringLiteralLoop7Result::MlTemporaryDecodeRustByteStringLiteralLoop7Return(vec![-4f64, position, (position + 1f64)])
                                        } else if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 127f64) {
                                                self::MlTemporaryDecodeRustByteStringLiteralLoop7Result::MlTemporaryDecodeRustByteStringLiteralLoop7Return(vec![-3f64, position, (position + 1f64)])
                                            } else {
                                                {
                                                    let bytes_4 = {
                                                        let source_ml_source1_concat_value66 = bytes;
                                                        {
                                                            let source_ml_source1_concat_value67 = vec![unit];
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value66), source_ml_source1_concat_value67)
                                                        }
                                                    };
                                                    {
                                                        let position_5 = position + 1f64;
                                                        {
                                                            (position, bytes) = (position_5, bytes_4);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                    }
                                }
                            }
                        }
                        self::MlTemporaryDecodeRustByteStringLiteralJoin8::MlTemporaryDecodeRustByteStringLiteralJoin8Return(source_ml_temporary_result8) => {
                            self::MlTemporaryDecodeRustByteStringLiteralLoop7Result::MlTemporaryDecodeRustByteStringLiteralLoop7Return(source_ml_temporary_result8)
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryDecodeRustByteStringLiteralLoop7Result::MlTemporaryDecodeRustByteStringLiteralLoop7Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_render_rust_byte_string_value_loop10(
    bytes: &[f64],
    mut text: self::ml_string::Text,
    mut index: f64,
) -> self::ml_string::Text {
    loop {
        return if index < (bytes.len() as f64) {
            {
                let byte =
                    (self::ml_array::optional_at(bytes, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(byte)) || (byte < 0f64)) || (byte > 255f64) {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("RangeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116, 32, 98, 121,
                            116, 101, 32, 115, 116, 114, 105, 110, 103, 32, 112, 97, 121, 108, 111,
                            97, 100,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                } else {
                    {
                        let text_2 = (text.concat(&{
                            let source_ml_source1_string_unit_operand68 = self::ml_string::Text::from_units(vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102]);
                            {
                                let source_ml_source1_string_unit_operand69 = (byte / 16f64).floor();
                                (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        {
                                            let source_ml_source1_string_range_operand70 = text_2.clone();
                                            {
                                                let source_ml_source1_string_range_operand71 = index_2;
                                                {
                                                    let source_ml_source1_string_range_operand72 = index_2 + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position73 = source_ml_source1_string_range_operand71;
                                                        {
                                                            let source_ml_source1_string_range_position74 = source_ml_source1_string_range_operand72;
                                                            source_ml_source1_string_range_operand70.slice(source_ml_source1_string_range_position73, source_ml_source1_string_range_position74)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand68, source_ml_source1_string_unit_operand69)
                            }
                        })).concat(&{
                            let source_ml_source1_string_unit_operand75 = self::ml_string::Text::from_units(vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102]);
                            {
                                let source_ml_source1_string_unit_operand76 = byte % 16f64;
                                (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        {
                                            let source_ml_source1_string_range_operand77 = text_2.clone();
                                            {
                                                let source_ml_source1_string_range_operand78 = index_2;
                                                {
                                                    let source_ml_source1_string_range_operand79 = index_2 + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position80 = source_ml_source1_string_range_operand78;
                                                        {
                                                            let source_ml_source1_string_range_position81 = source_ml_source1_string_range_operand79;
                                                            source_ml_source1_string_range_operand77.slice(source_ml_source1_string_range_position80, source_ml_source1_string_range_position81)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand75, source_ml_source1_string_unit_operand76)
                            }
                        });
                        {
                            let index_2 = index + 1f64;
                            {
                                (text, index) = (text_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}
