// Generated through the checked JavaScript Rust raw string decoder. Do not edit.
pub use crate::translation::frontend_rules::ml_array;

#[derive(Clone, Debug, PartialEq)]
pub enum RustRawString {
    NotRaw,
    Literal(f64, f64, f64),
    TooManyDelimiters,
    CarriageReturn(f64),
    NonScalar(f64),
    Unterminated,
}

/// Read the shared original-source Rust raw string model.
#[must_use]
pub fn decode_rust_raw_string(
    units: &[f64],
    start: f64,
) -> crate::translation::lexer::raw_strings::RustRawString {
    if (((start < 0f64)
        || (start
            >= (serde_json::Number::from(units.len())
                .as_f64()
                .expect("a source unit vector length has a finite binary64 representation"))))
        || !matches!(
            (start % 1f64).partial_cmp(&0f64),
            Some(std::cmp::Ordering::Equal)
        ))
        || !(match (
            ml_array::optional_at(units, ml_array::number_index(start)),
            114f64,
        ) {
            (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
            _ => false,
        })
    {
        crate::translation::lexer::raw_strings::RustRawString::NotRaw
    } else {
        {
            let opening = start + 1f64;
            {
                let opening_2 =
                    crate::translation::lexer::raw_strings::ml_decode_rust_raw_string_loop5(
                        units, opening,
                    );
                if match (
                    ml_array::optional_at(units, ml_array::number_index(opening_2)),
                    34f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                    _ => false,
                } {
                    {
                        let delimiters = (opening_2 - start) - 1f64;
                        if delimiters > 255f64 {
                            crate::translation::lexer::raw_strings::RustRawString::TooManyDelimiters
                        } else {
                            {
                                let cursor = opening_2 + 1f64;
                                {
                                    let ml_s4 = crate::translation::lexer::raw_strings::ml_decode_rust_raw_string_loop6(units, opening_2, delimiters, cursor);
                                    match ml_s4 {
                                        crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop6Result::Done => {
                                            crate::translation::lexer::raw_strings::RustRawString::Unterminated
                                        }
                                        crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop6Result::Return(ml_result6) => {
                                            *ml_result6
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    crate::translation::lexer::raw_strings::RustRawString::NotRaw
                }
            }
        }
    }
}

/// Read the shared original-source Rust raw string model.
#[must_use]
fn ml_decode_rust_raw_string_loop5(units: &[f64], mut opening: f64) -> f64 {
    loop {
        if (opening
            < (serde_json::Number::from(units.len())
                .as_f64()
                .expect("a source unit vector length has a finite binary64 representation")))
            && match (
                ml_array::optional_at(units, ml_array::number_index(opening)),
                35f64,
            ) {
                (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                _ => false,
            }
        {
            {
                let opening_2 = opening + 1f64;
                {
                    opening = opening_2;
                    continue;
                }
            }
        }
        return opening;
    }
}

#[derive(Clone, Debug, PartialEq)]
enum MlDecodeRustRawStringLoop6Result {
    Done,
    Return(Box<crate::translation::lexer::raw_strings::RustRawString>),
}

#[derive(Clone, Debug, PartialEq)]
enum MlDecodeRustRawStringJoin7 {
    Next,
    Return(Box<crate::translation::lexer::raw_strings::RustRawString>),
}

#[derive(Clone, Debug, PartialEq)]
enum MlDecodeRustRawStringLoop8Result {
    Done(f64, f64),
}

/// Read the shared original-source Rust raw string model.
#[must_use]
fn ml_decode_rust_raw_string_loop8(
    units: &[f64],
    delimiters: f64,
    mut closing: f64,
    mut matched: f64,
) -> crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop8Result {
    loop {
        if (matched < delimiters)
            && match (
                ml_array::optional_at(units, ml_array::number_index(closing)),
                35f64,
            ) {
                (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                _ => false,
            }
        {
            {
                let closing_2 = closing + 1f64;
                {
                    let matched_2 = matched + 1f64;
                    {
                        (closing, matched) = (closing_2, matched_2);
                        continue;
                    }
                }
            }
        }
        return {
            crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop8Result::Done(
                closing, matched,
            )
        };
    }
}

/// Read the shared original-source Rust raw string model.
#[must_use]
fn ml_decode_rust_raw_string_loop6(
    units: &[f64],
    opening: f64,
    delimiters: f64,
    mut cursor: f64,
) -> crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop6Result {
    loop {
        return if cursor
            < (serde_json::Number::from(units.len())
                .as_f64()
                .expect("a source unit vector length has a finite binary64 representation"))
        {
            {
                let ml_s6 = if match (
                    ml_array::optional_at(units, ml_array::number_index(cursor)),
                    34f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                    _ => false,
                } {
                    {
                        let closing = cursor + 1f64;
                        {
                            let matched = 0f64;
                            {
                                let ml_s7 = crate::translation::lexer::raw_strings::ml_decode_rust_raw_string_loop8(units, delimiters, closing, matched);
                                let crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop8Result::Done(closing_2, matched_2) = ml_s7;

                                if matches!(
                                    matched_2.partial_cmp(&delimiters),
                                    Some(std::cmp::Ordering::Equal)
                                ) {
                                    crate::translation::lexer::raw_strings::MlDecodeRustRawStringJoin7::Return(Box::new(crate::translation::lexer::raw_strings::RustRawString::Literal(opening + 1f64, cursor, closing_2)))
                                } else {
                                    crate::translation::lexer::raw_strings::MlDecodeRustRawStringJoin7::Next
                                }
                            }
                        }
                    }
                } else {
                    crate::translation::lexer::raw_strings::MlDecodeRustRawStringJoin7::Next
                };
                match ml_s6 {
                    crate::translation::lexer::raw_strings::MlDecodeRustRawStringJoin7::Next => {
                        if match (
                            ml_array::optional_at(units, ml_array::number_index(cursor)),
                            13f64,
                        ) {
                            (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                            _ => false,
                        } {
                            {
                                let end = crate::translation::frontend_rules::read_rust_source_line_break_end(units, cursor);
                                if end < 0f64 {
                                    crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop6Result::Return(Box::new(crate::translation::lexer::raw_strings::RustRawString::CarriageReturn(cursor)))
                                } else {
                                    {
                                        let cursor_2 = end;
                                        {
                                            cursor = cursor_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let scalar = crate::translation::source_text_byte_rules::read_source_text_scalar(units, cursor);
                                if scalar < 0f64 {
                                    crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop6Result::Return(Box::new(crate::translation::lexer::raw_strings::RustRawString::NonScalar(cursor)))
                                } else {
                                    {
                                        let cursor_3 =
                                            cursor + if scalar >= 65_536f64 { 2f64 } else { 1f64 };
                                        {
                                            cursor = cursor_3;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    crate::translation::lexer::raw_strings::MlDecodeRustRawStringJoin7::Return(
                        ml_result7,
                    ) => {
                        let ml_result7 = *ml_result7;
                        crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop6Result::Return(Box::new(ml_result7))
                    }
                }
            }
        } else {
            crate::translation::lexer::raw_strings::MlDecodeRustRawStringLoop6Result::Done
        };
    }
}
