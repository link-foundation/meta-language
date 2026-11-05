// Generated from complete js/src/translation/rust-byte-string-rules.js. Do not edit by hand.
use super::RustCharacterLiteral;
use super::RustUnicodeEscape;
use super::accept_rust_cooked_string_escape_prefix;
use super::decode_rust_unicode_escape;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_rust_ascii_escape_digit;
use super::read_rust_ascii_escape_value;
use super::read_source_text_scalar;
use super::source_ml_temporary_read_rust_string_continuation_end_loop5;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_model(model: &self::ml_string::Text) -> bool {
    (*model)
        == (self::ml_string::Text::from_units(vec![
            112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 98, 111, 114, 114, 111, 119, 101, 100,
            32, 82, 117, 115, 116, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 104, 101, 97,
            100, 101, 114, 32, 49,
        ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_context(
    language: &self::ml_string::Text,
    forbidden: bool,
) -> bool {
    ((*language) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) && !forbidden
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_declaration(
    kind: &self::ml_string::Text,
    non_flat: bool,
    generated: bool,
) -> bool {
    (((*kind) == (self::ml_string::Text::from_units(vec![102, 110]))) && !non_flat) && !generated
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_source_header_emission_declaration_fields(unrepresented: bool) -> bool {
    !unrepresented
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_position(
    kind: &self::ml_string::Text,
    owner_kind: &self::ml_string::Text,
    owner_path: &self::ml_string::Text,
    invalid_arguments: bool,
    invalid_wrappers: bool,
    borrowed_result: bool,
) -> bool {
    (((!invalid_arguments && !invalid_wrappers) && !borrowed_result)
        && ((*owner_kind)
            == (self::ml_string::Text::from_units(vec![
                112, 114, 105, 109, 105, 116, 105, 118, 101,
            ]))))
        && ((((*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
            && ((*owner_path) == (self::ml_string::Text::from_units(vec![102, 54, 52]))))
            || (((*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))
                && ((*owner_path) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_expression(kind: &self::ml_string::Text) -> bool {
    ((((((*kind) == (self::ml_string::Text::from_units(vec![108, 105, 116])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![108, 101, 116]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![105, 102]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_expression_field(
    kind: &self::ml_string::Text,
    field: &self::ml_string::Text,
) -> bool {
    if (*kind) == (self::ml_string::Text::from_units(vec![108, 105, 116])) {
        ((((*field) == (self::ml_string::Text::from_units(vec![107])))
            || ((*field) == (self::ml_string::Text::from_units(vec![116, 121, 112, 101]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))
    } else if (*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
        ((((*field) == (self::ml_string::Text::from_units(vec![107])))
            || ((*field) == (self::ml_string::Text::from_units(vec![116, 121, 112, 101]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101])))
    } else if (*kind) == (self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121])) {
        ((((((*field) == (self::ml_string::Text::from_units(vec![107])))
            || ((*field) == (self::ml_string::Text::from_units(vec![116, 121, 112, 101]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![111, 112]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![97, 114, 103]))))
            || ((*field)
                == (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 115,
                ])))
    } else if (*kind) == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])) {
        ((((((((*field) == (self::ml_string::Text::from_units(vec![107])))
            || ((*field) == (self::ml_string::Text::from_units(vec![116, 121, 112, 101]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![111, 112]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![108, 101, 102, 116]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![100, 111, 109, 97, 105, 110]))))
            || ((*field)
                == (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 115,
                ])))
    } else if (*kind) == (self::ml_string::Text::from_units(vec![108, 101, 116])) {
        ((((((*field) == (self::ml_string::Text::from_units(vec![107])))
            || ((*field) == (self::ml_string::Text::from_units(vec![116, 121, 112, 101]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![110, 97, 109, 101]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![98, 111, 100, 121])))
    } else if (*kind) == (self::ml_string::Text::from_units(vec![105, 102])) {
        ((((((*field) == (self::ml_string::Text::from_units(vec![107])))
            || ((*field) == (self::ml_string::Text::from_units(vec![116, 121, 112, 101]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![115, 112, 97, 110]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![99, 111, 110, 100]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![116, 104, 101, 110]))))
            || ((*field) == (self::ml_string::Text::from_units(vec![101, 108, 115, 101])))
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_source_header_emission_semantics(semantics: &self::ml_string::Text) -> bool {
    ((((*semantics) == (self::ml_string::Text::from_units(vec![101, 120, 97, 99, 116])))
        || ((*semantics)
            == (self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100]))))
        || ((*semantics)
            == (self::ml_string::Text::from_units(vec![
                116, 114, 117, 110, 99, 97, 116, 101, 100,
            ]))))
        || ((*semantics) == (self::ml_string::Text::from_units(vec![105, 101, 101, 101])))
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
pub fn read_rust_source_header_primitive(kind: &self::ml_string::Text) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
        self::ml_string::Text::from_units(vec![102, 54, 52])
    } else if (*kind) == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])) {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_reference(
    lifetime: &self::ml_string::Text,
    inner: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*lifetime) == (self::ml_string::Text::from_units(vec![])) {
        (self::ml_string::Text::from_units(vec![38])).concat(inner)
    } else {
        (((self::ml_string::Text::from_units(vec![38, 39])).concat(lifetime))
            .concat(&self::ml_string::Text::from_units(vec![32])))
        .concat(inner)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_parameter(
    name: &self::ml_string::Text,
    type_: &self::ml_string::Text,
) -> self::ml_string::Text {
    (name.concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(type_)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_lifetime(name: &self::ml_string::Text) -> self::ml_string::Text {
    (self::ml_string::Text::from_units(vec![39])).concat(name)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_dereference(
    inner: &self::ml_string::Text,
) -> self::ml_string::Text {
    (self::ml_string::Text::from_units(vec![42])).concat(inner)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_copy(
    name: &self::ml_string::Text,
    dereferenced: &self::ml_string::Text,
) -> self::ml_string::Text {
    ((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(name))
        .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
    .concat(dereferenced))
    .concat(&self::ml_string::Text::from_units(vec![59]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_rust_source_header_function(
    name: &self::ml_string::Text,
    lifetimes: &self::ml_string::Text,
    parameters: &self::ml_string::Text,
    result: &self::ml_string::Text,
    body: &self::ml_string::Text,
) -> self::ml_string::Text {
    (((((((((self::ml_string::Text::from_units(vec![112, 117, 98, 32, 102, 110, 32]))
        .concat(name))
    .concat(
        &if (*lifetimes) == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            ((self::ml_string::Text::from_units(vec![60])).concat(lifetimes))
                .concat(&self::ml_string::Text::from_units(vec![62]))
        },
    ))
    .concat(&self::ml_string::Text::from_units(vec![40])))
    .concat(parameters))
    .concat(&self::ml_string::Text::from_units(vec![41, 32, 45, 62, 32])))
    .concat(result))
    .concat(&self::ml_string::Text::from_units(vec![32, 123, 10])))
    .concat(body))
    .concat(&self::ml_string::Text::from_units(vec![10, 125]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_number_digit_boundary(
    floating: bool,
    trailing_separator: bool,
    suffix: &self::ml_string::Text,
    continued_suffix: bool,
    following_code_unit: f64,
) -> bool {
    (floating || !trailing_separator)
        || ((!continued_suffix && (following_code_unit < 128f64))
            && (((((((((((((*suffix)
                == (self::ml_string::Text::from_units(vec![117, 56])))
                || ((*suffix) == (self::ml_string::Text::from_units(vec![105, 56]))))
                || ((*suffix) == (self::ml_string::Text::from_units(vec![117, 49, 54]))))
                || ((*suffix) == (self::ml_string::Text::from_units(vec![105, 49, 54]))))
                || ((*suffix) == (self::ml_string::Text::from_units(vec![117, 51, 50]))))
                || ((*suffix) == (self::ml_string::Text::from_units(vec![105, 51, 50]))))
                || ((*suffix) == (self::ml_string::Text::from_units(vec![117, 54, 52]))))
                || ((*suffix) == (self::ml_string::Text::from_units(vec![105, 54, 52]))))
                || ((*suffix) == (self::ml_string::Text::from_units(vec![117, 49, 50, 56]))))
                || ((*suffix) == (self::ml_string::Text::from_units(vec![105, 49, 50, 56]))))
                || ((*suffix)
                    == (self::ml_string::Text::from_units(vec![117, 115, 105, 122, 101]))))
                || ((*suffix)
                    == (self::ml_string::Text::from_units(vec![105, 115, 105, 122, 101])))))
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
    suffix: &self::ml_string::Text,
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
    } else if {
        let source_ml_source1_comparison_operand11 =
            self::ml_array::optional_at(units, self::ml_array::number_index(content_end));
        {
            let source_ml_source1_comparison_operand12 = 39f64;
            !(match (
                source_ml_source1_comparison_operand11,
                source_ml_source1_comparison_operand12,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    } {
        self::RustCharacterLiteral::Malformed(content_end, (content_end + 1f64))
    } else {
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decode_rust_character_literal(units: &[f64], start: f64) -> self::RustCharacterLiteral {
    if (!(self::ml_math::is_integer(start)) || (start < 0f64)) || (start >= (units.len() as f64)) {
        self::RustCharacterLiteral::NotCharacter
    } else {
        {
            let byte = {
                let source_ml_source1_comparison_operand13 =
                    self::ml_array::optional_at(units, self::ml_array::number_index(start));
                {
                    let source_ml_source1_comparison_operand14 = 98f64;
                    match (
                        source_ml_source1_comparison_operand13,
                        source_ml_source1_comparison_operand14,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } && {
                let source_ml_source1_comparison_operand15 =
                    self::ml_array::optional_at(units, self::ml_array::number_index(start + 1f64));
                {
                    let source_ml_source1_comparison_operand16 = 39f64;
                    match (
                        source_ml_source1_comparison_operand15,
                        source_ml_source1_comparison_operand16,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            };
            if !byte && {
                let source_ml_source1_comparison_operand17 =
                    self::ml_array::optional_at(units, self::ml_array::number_index(start));
                {
                    let source_ml_source1_comparison_operand18 = 39f64;
                    !(match (
                        source_ml_source1_comparison_operand17,
                        source_ml_source1_comparison_operand18,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
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
                                                if !byte && {
                                                    let source_ml_source1_comparison_operand19 =
                                                        self::ml_array::optional_at(
                                                            units,
                                                            self::ml_array::number_index(
                                                                content_end,
                                                            ),
                                                        );
                                                    {
                                                        let source_ml_source1_comparison_operand20 =
                                                            39f64;
                                                        !(match (
                                                            source_ml_source1_comparison_operand19,
                                                            source_ml_source1_comparison_operand20,
                                                        ) {
                                                            (Some(ml_left), ml_right) => {
                                                                ml_left == ml_right
                                                            }
                                                            _ => false,
                                                        })
                                                    }
                                                } {
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
                                                                self::RustUnicodeEscape::Unsupported(end_2, source_ml_source1_f21) => {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_character_expression_type(kind: &self::ml_string::Text) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![98, 121, 116, 101, 67, 104, 97, 114])) {
        self::ml_string::Text::from_units(vec![117, 56])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_numeric_literal_negation(
    language: &self::ml_string::Text,
    type_kind: &self::ml_string::Text,
    signed: bool,
) -> bool {
    (((*language) != (self::ml_string::Text::from_units(vec![82, 117, 115, 116])))
        || ((*type_kind) != (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
        || signed
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_string_continuation_whitespace(unit: Option<f64>) -> bool {
    (({
        let source_ml_source1_comparison_operand22 = unit.clone();
        {
            let source_ml_source1_comparison_operand23 = 9f64;
            match (
                source_ml_source1_comparison_operand22,
                source_ml_source1_comparison_operand23,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    } || {
        let source_ml_source1_comparison_operand24 = unit.clone();
        {
            let source_ml_source1_comparison_operand25 = 10f64;
            match (
                source_ml_source1_comparison_operand24,
                source_ml_source1_comparison_operand25,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || {
        let source_ml_source1_comparison_operand26 = unit.clone();
        {
            let source_ml_source1_comparison_operand27 = 13f64;
            match (
                source_ml_source1_comparison_operand26,
                source_ml_source1_comparison_operand27,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || {
        let source_ml_source1_comparison_operand28 = unit.clone();
        {
            let source_ml_source1_comparison_operand29 = 32f64;
            match (
                source_ml_source1_comparison_operand28,
                source_ml_source1_comparison_operand29,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_string_continuation_end(units: &[f64], start: f64) -> f64 {
    if (((start < 0f64) || (start >= (units.len() as f64))) || ((start % 1f64) != 0f64)) || {
        let source_ml_source1_comparison_operand30 =
            self::ml_array::optional_at(units, self::ml_array::number_index(start));
        {
            let source_ml_source1_comparison_operand31 = 92f64;
            !(match (
                source_ml_source1_comparison_operand30,
                source_ml_source1_comparison_operand31,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    } {
        -1f64
    } else {
        {
            let end = start + 1f64;
            {
                let end_3 = if {
                    let source_ml_source1_comparison_operand32 =
                        self::ml_array::optional_at(units, self::ml_array::number_index(end));
                    {
                        let source_ml_source1_comparison_operand33 = 13f64;
                        match (
                            source_ml_source1_comparison_operand32,
                            source_ml_source1_comparison_operand33,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } && {
                    let source_ml_source1_comparison_operand34 = self::ml_array::optional_at(
                        units,
                        self::ml_array::number_index(end + 1f64),
                    );
                    {
                        let source_ml_source1_comparison_operand35 = 10f64;
                        match (
                            source_ml_source1_comparison_operand34,
                            source_ml_source1_comparison_operand35,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } {
                    { end + 1f64 }
                } else {
                    end
                };
                if {
                    let source_ml_source1_comparison_operand36 =
                        self::ml_array::optional_at(units, self::ml_array::number_index(end_3));
                    {
                        let source_ml_source1_comparison_operand37 = 10f64;
                        !(match (
                            source_ml_source1_comparison_operand36,
                            source_ml_source1_comparison_operand37,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } {
                    -1f64
                } else {
                    {
                        let end_4 = end_3 + 1f64;
                        {
                            self::source_ml_temporary_read_rust_string_continuation_end_loop5(
                                units, end_4,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_line_break_end(units: &[f64], start: f64) -> f64 {
    if ((start < 0f64) || (start >= (units.len() as f64))) || ((start % 1f64) != 0f64) {
        -1f64
    } else {
        if {
            let source_ml_source1_comparison_operand38 =
                self::ml_array::optional_at(units, self::ml_array::number_index(start));
            {
                let source_ml_source1_comparison_operand39 = 13f64;
                match (
                    source_ml_source1_comparison_operand38,
                    source_ml_source1_comparison_operand39,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } && {
            let source_ml_source1_comparison_operand40 =
                self::ml_array::optional_at(units, self::ml_array::number_index(start + 1f64));
            {
                let source_ml_source1_comparison_operand41 = 10f64;
                match (
                    source_ml_source1_comparison_operand40,
                    source_ml_source1_comparison_operand41,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } {
            start + 2f64
        } else {
            -1f64
        }
    }
}
