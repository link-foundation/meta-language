// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlDecodeRustUnicodeEscapeLoop4Result;
use super::MlReadSourceTextByteLengthLoop1Result;
use super::MlReadSourceTextByteLoop3Result;
use super::MlReadSourceTextCodeUnitOffsetLoop2Result;
use super::ml_decode_rust_unicode_escape_loop4;
use super::ml_read_source_text_byte_length_loop1;
use super::ml_read_source_text_byte_loop3;
use super::ml_read_source_text_code_unit_offset_loop2;

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

#[derive(Clone, Debug, PartialEq)]
pub enum RustUnicodeEscape {
    Scalar(f64, f64),
    Malformed,
    Unsupported(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RustCharacterLiteral {
    NotCharacter,
    Char(f64, f64, f64, f64),
    ByteChar(f64, f64, f64, f64),
    Malformed(f64, f64),
    Unterminated(f64, f64),
    NonScalar(f64, f64),
    NonAscii(f64, f64),
    UnicodeByteEscape(f64, f64),
    UnsupportedSuffix(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_scalar(units: &[f64], index: f64) -> f64 {
    if ((!(self::ml_math::is_integer(index)) || (index < 0f64)) || (index >= (units.len() as f64)))
        || ((units.len() as f64) > 4_294_967_295f64)
    {
        -1f64
    } else {
        {
            let first = (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                .unwrap_or_else(|| -1f64);
            if (!(self::ml_math::is_integer(first)) || (first < 0f64)) || (first > 65_535f64) {
                -1f64
            } else if (55_296f64..=56_319f64).contains(&first) {
                if (index + 1f64) >= (units.len() as f64) {
                    -1f64
                } else {
                    {
                        let second = (self::ml_array::optional_at(
                            units,
                            self::ml_array::number_index(index + 1f64),
                        ))
                        .unwrap_or_else(|| -1f64);
                        if (!(self::ml_math::is_integer(second)) || (second < 56_320f64))
                            || (second > 57_343f64)
                        {
                            -1f64
                        } else {
                            ((65_536f64 + ((first - 55_296f64) * 1024f64)) + second) - 56_320f64
                        }
                    }
                }
            } else if (56_320f64..=57_343f64).contains(&first) {
                -1f64
            } else {
                first
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_scalar_byte_width(scalar: f64) -> f64 {
    if ((!(self::ml_math::is_integer(scalar)) || (scalar < 0f64)) || (scalar > 1_114_111f64))
        || ((55_296f64..=57_343f64).contains(&scalar))
    {
        -1f64
    } else if scalar < 128f64 {
        1f64
    } else if scalar < 2048f64 {
        2f64
    } else if scalar < 65_536f64 {
        3f64
    } else {
        4f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_byte_length(units: &[f64]) -> f64 {
    if (units.len() as f64) > 4_294_967_295f64 {
        -1f64
    } else {
        {
            let index = 0f64;
            {
                let bytes = 0f64;
                {
                    let ml_s1 = self::ml_read_source_text_byte_length_loop1(units, index, bytes);
                    match ml_s1 {
                        self::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Done(bytes_2) => {
                            bytes_2
                        }
                        self::MlReadSourceTextByteLengthLoop1Result::MlReadSourceTextByteLengthLoop1Return(ml_result1) => {
                            ml_result1
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_text_scalar_units(units: &[f64]) -> bool {
    (self::read_source_text_byte_length(units)) >= 0f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_code_unit_offset(units: &[f64], offset: f64) -> f64 {
    {
        let length = self::read_source_text_byte_length(units);
        if (((length < 0f64) || !(self::ml_math::is_integer(offset))) || (offset < 0f64))
            || (offset > length)
        {
            -1f64
        } else {
            {
                let index = 0f64;
                {
                    let bytes = 0f64;
                    {
                        let ml_s2 = self::ml_read_source_text_code_unit_offset_loop2(
                            units, offset, index, bytes,
                        );
                        match ml_s2 {
                            self::MlReadSourceTextCodeUnitOffsetLoop2Result::MlReadSourceTextCodeUnitOffsetLoop2Done(index_2, bytes_2) => {
                                if bytes_2 == offset {
                                    index_2
                                } else {
                                    -1f64
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
pub fn accept_source_text_byte_slice(units: &[f64], start: f64, end: f64) -> bool {
    ((start <= end) && ((self::read_source_text_code_unit_offset(units, start)) >= 0f64))
        && ((self::read_source_text_code_unit_offset(units, end)) >= 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_text_byte(units: &[f64], offset: f64) -> f64 {
    {
        let length = self::read_source_text_byte_length(units);
        if (((length < 0f64) || !(self::ml_math::is_integer(offset))) || (offset < 0f64))
            || (offset >= length)
        {
            -1f64
        } else {
            {
                let index = 0f64;
                {
                    let bytes = 0f64;
                    {
                        let ml_s3 =
                            self::ml_read_source_text_byte_loop3(units, offset, index, bytes);
                        match ml_s3 {
                            self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Done => {
                                -1f64
                            }
                            self::MlReadSourceTextByteLoop3Result::MlReadSourceTextByteLoop3Return(ml_result3) => {
                                ml_result3
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
pub fn accept_source_text_unit_count(count: f64) -> bool {
    (self::ml_math::is_integer(count) && (count >= 0f64)) && (count <= 4_294_967_295f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_cooked_string_escape_prefix(unit: f64) -> bool {
    ((((((((unit == 34f64) || (unit == 39f64)) || (unit == 48f64)) || (unit == 92f64))
        || (unit == 110f64))
        || (unit == 114f64))
        || (unit == 116f64))
        || (unit == 117f64))
        || (unit == 120f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_ascii_escape_digit(unit: f64) -> f64 {
    if unit == 48f64 {
        0f64
    } else if unit == 49f64 {
        1f64
    } else if unit == 50f64 {
        2f64
    } else if unit == 51f64 {
        3f64
    } else if unit == 52f64 {
        4f64
    } else if unit == 53f64 {
        5f64
    } else if unit == 54f64 {
        6f64
    } else if unit == 55f64 {
        7f64
    } else if unit == 56f64 {
        8f64
    } else if unit == 57f64 {
        9f64
    } else if (unit == 65f64) || (unit == 97f64) {
        10f64
    } else if (unit == 66f64) || (unit == 98f64) {
        11f64
    } else if (unit == 67f64) || (unit == 99f64) {
        12f64
    } else if (unit == 68f64) || (unit == 100f64) {
        13f64
    } else if (unit == 69f64) || (unit == 101f64) {
        14f64
    } else if (unit == 70f64) || (unit == 102f64) {
        15f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_ascii_escape_value(high_unit: f64, low_unit: f64) -> f64 {
    {
        let high = self::read_rust_ascii_escape_digit(high_unit);
        {
            let low = self::read_rust_ascii_escape_digit(low_unit);
            if ((high < 0f64) || (high > 7f64)) || (low < 0f64) {
                -1f64
            } else {
                (high * 16f64) + low
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decode_rust_unicode_escape(units: &[f64], start: f64) -> self::RustUnicodeEscape {
    if (((((start < 0f64) || (start >= (units.len() as f64))) || ((start % 1f64) != 0f64))
        || !(match (
            self::ml_array::optional_at(units, self::ml_array::number_index(start)),
            92f64,
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        }))
        || !(match (
            self::ml_array::optional_at(units, self::ml_array::number_index(start + 1f64)),
            117f64,
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        }))
        || !(match (
            self::ml_array::optional_at(units, self::ml_array::number_index(start + 2f64)),
            123f64,
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
    {
        self::RustUnicodeEscape::Malformed
    } else {
        {
            let index = start + 3f64;
            {
                let code = 0f64;
                {
                    let digits = 0f64;
                    {
                        let ml_s4 = self::ml_decode_rust_unicode_escape_loop4(
                            units, start, index, code, digits,
                        );
                        match ml_s4 {
                            self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Done => {
                                self::RustUnicodeEscape::Malformed
                            }
                            self::MlDecodeRustUnicodeEscapeLoop4Result::MlDecodeRustUnicodeEscapeLoop4Return(ml_result4) => {
                                let ml_result4 = *ml_result4;
                                ml_result4.clone()
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
pub fn accept_rust_source_header_emission_model(model: &str) -> bool {
    model == "primitive borrowed Rust function header 1"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_context(language: &str, forbidden: bool) -> bool {
    (language == "Rust") && !forbidden
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_declaration(
    kind: &str,
    non_flat: bool,
    generated: bool,
) -> bool {
    ((kind == "fn") && !non_flat) && !generated
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_source_header_emission_declaration_fields(unrepresented: bool) -> bool {
    !unrepresented
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_position(
    kind: &str,
    owner_kind: &str,
    owner_path: &str,
    invalid_arguments: bool,
    invalid_wrappers: bool,
    borrowed_result: bool,
) -> bool {
    (((!invalid_arguments && !invalid_wrappers) && !borrowed_result) && (owner_kind == "primitive"))
        && (((kind == "float") && (owner_path == "f64"))
            || ((kind == "bool") && (owner_path == "bool")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_expression(kind: &str) -> bool {
    (((((kind == "lit") || (kind == "var")) || (kind == "let")) || (kind == "if"))
        || (kind == "unary"))
        || (kind == "binary")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_expression_field(kind: &str, field: &str) -> bool {
    if kind == "lit" {
        (((field == "k") || (field == "type")) || (field == "span")) || (field == "value")
    } else if kind == "var" {
        (((field == "k") || (field == "type")) || (field == "span")) || (field == "name")
    } else if kind == "unary" {
        (((((field == "k") || (field == "type")) || (field == "span")) || (field == "op"))
            || (field == "arg"))
            || (field == "semantics")
    } else if kind == "binary" {
        (((((((field == "k") || (field == "type")) || (field == "span")) || (field == "op"))
            || (field == "left"))
            || (field == "right"))
            || (field == "domain"))
            || (field == "semantics")
    } else if kind == "let" {
        (((((field == "k") || (field == "type")) || (field == "span")) || (field == "name"))
            || (field == "value"))
            || (field == "body")
    } else if kind == "if" {
        (((((field == "k") || (field == "type")) || (field == "span")) || (field == "cond"))
            || (field == "then"))
            || (field == "else")
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_semantics(semantics: &str) -> bool {
    (((semantics == "exact") || (semantics == "checked")) || (semantics == "truncated"))
        || (semantics == "ieee")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_source_header_emission_parameter(
    guarded: bool,
    defaulted: bool,
    unrepresented: bool,
) -> bool {
    (!guarded && !defaulted) && !unrepresented
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_lifetime_character(character: f64, first: bool) -> bool {
    ((((95f64..=95f64).contains(&character)) || ((65f64..=90f64).contains(&character)))
        || ((97f64..=122f64).contains(&character)))
        || (!first && ((48f64..=57f64).contains(&character)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_header_primitive(kind: &str) -> String {
    if kind == "float" {
        String::from("f64")
    } else if kind == "bool" {
        String::from("bool")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_reference(lifetime: &str, inner: &str) -> String {
    if (lifetime).is_empty() {
        format!("&{}", inner.to_owned())
    } else {
        format!("&'{} {}", lifetime.to_owned(), inner.to_owned())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_parameter(name: &str, type_: &str) -> String {
    format!("{}: {}", name.to_owned(), type_.to_owned())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_lifetime(name: &str) -> String {
    format!("'{}", name.to_owned())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_dereference(inner: &str) -> String {
    format!("*{}", inner.to_owned())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_copy(name: &str, dereferenced: &str) -> String {
    format!("let {} = {};", name.to_owned(), dereferenced.to_owned())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_function(
    name: &str,
    lifetimes: &str,
    parameters: &str,
    result: &str,
    body: &str,
) -> String {
    format!(
        "pub fn {}{}({}) -> {} {{\n{}\n}}",
        name.to_owned(),
        if (lifetimes).is_empty() {
            String::new()
        } else {
            format!(
                "{}{}",
                (format!("{}{}", (String::from("<")), lifetimes.to_owned())),
                (String::from(">"))
            )
        },
        parameters.to_owned(),
        result.to_owned(),
        body.to_owned()
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_number_digit_boundary(
    floating: bool,
    trailing_separator: bool,
    suffix: &str,
    continued_suffix: bool,
    following_code_unit: f64,
) -> bool {
    (floating || !trailing_separator)
        || ((!continued_suffix && (following_code_unit < 128f64))
            && ((((((((((((suffix == "u8") || (suffix == "i8")) || (suffix == "u16"))
                || (suffix == "i16"))
                || (suffix == "u32"))
                || (suffix == "i32"))
                || (suffix == "u64"))
                || (suffix == "i64"))
                || (suffix == "u128"))
                || (suffix == "i128"))
                || (suffix == "usize"))
                || (suffix == "isize")))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_whitespace(unit: f64) -> bool {
    ((((((((((unit == 9f64) || (unit == 10f64)) || (unit == 11f64)) || (unit == 12f64))
        || (unit == 13f64))
        || (unit == 32f64))
        || (unit == 133f64))
        || (unit == 8206f64))
        || (unit == 8207f64))
        || (unit == 8232f64))
        || (unit == 8233f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_byte_order_mark(unit: f64, first_position: bool) -> bool {
    first_position && (unit == 65_279f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_number_digit_boundary(
    floating: bool,
    trailing_separator: bool,
    suffix: &str,
    continued_suffix: bool,
    following_code_unit: f64,
) -> bool {
    self::accept_rust_number_digit_boundary(
        floating,
        trailing_separator,
        suffix,
        continued_suffix,
        following_code_unit,
    ) || (self::accept_rust_source_whitespace(following_code_unit)
        && self::accept_rust_number_digit_boundary(
            floating,
            trailing_separator,
            suffix,
            continued_suffix,
            32f64,
        ))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_character_byte_escape_value(high_unit: f64, low_unit: f64) -> f64 {
    {
        let high = self::read_rust_ascii_escape_digit(high_unit);
        {
            let low = self::read_rust_ascii_escape_digit(low_unit);
            if (high < 0f64) || (low < 0f64) {
                -1f64
            } else {
                (high * 16f64) + low
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_character_simple_escape_value(unit: f64) -> f64 {
    if unit == 110f64 {
        10f64
    } else if unit == 114f64 {
        13f64
    } else if unit == 116f64 {
        9f64
    } else if unit == 48f64 {
        0f64
    } else if ((unit == 92f64) || (unit == 39f64)) || (unit == 34f64) {
        unit
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_rust_character_literal(
    units: &[f64],
    byte: bool,
    value: f64,
    content_start: f64,
    content_end: f64,
) -> self::RustCharacterLiteral {
    if content_end >= (units.len() as f64) {
        self::RustCharacterLiteral::Unterminated(content_start, (units.len() as f64))
    } else if (match (
        self::ml_array::optional_at(units, self::ml_array::number_index(content_end)),
        39f64,
    ) {
        (Some(ml_left), ml_right) => ml_left == ml_right,
        _ => false,
    }) {
        {
            let end = content_end + 1f64;
            {
                let following =
                    (self::ml_array::optional_at(units, self::ml_array::number_index(end)))
                        .unwrap_or_else(|| -1f64);
                if self::accept_rust_source_header_lifetime_character(following, true) {
                    self::RustCharacterLiteral::UnsupportedSuffix(end, (end + 1f64))
                } else if (following >= 128f64) && !(self::accept_rust_source_whitespace(following))
                {
                    {
                        let scalar = self::read_source_text_scalar(units, end);
                        if scalar < 0f64 {
                            self::RustCharacterLiteral::NonScalar(end, (end + 1f64))
                        } else {
                            self::RustCharacterLiteral::UnsupportedSuffix(
                                end,
                                (end + if scalar >= 65_536f64 { 2f64 } else { 1f64 }),
                            )
                        }
                    }
                } else if byte {
                    self::RustCharacterLiteral::ByteChar(value, content_start, content_end, end)
                } else {
                    self::RustCharacterLiteral::Char(value, content_start, content_end, end)
                }
            }
        }
    } else {
        self::RustCharacterLiteral::Malformed(content_end, (content_end + 1f64))
    }
}
