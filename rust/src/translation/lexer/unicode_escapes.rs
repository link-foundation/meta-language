// Generated through the checked JavaScript Rust Unicode decoder. Do not edit.
pub use crate::translation::frontend_rules::ml_array;

/// Decode the shared Rust Unicode scalar escape model.
#[must_use]
pub fn decode_rust_unicode_escape(
    units: &[f64],
    start: f64,
) -> crate::translation::frontend_rules::UnicodeEscape {
    if (((((start < 0f64)
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
            92f64,
        ) {
            (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
            _ => false,
        }))
        || !(match (
            ml_array::optional_at(units, ml_array::number_index(start + 1f64)),
            117f64,
        ) {
            (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
            _ => false,
        }))
        || !(match (
            ml_array::optional_at(units, ml_array::number_index(start + 2f64)),
            123f64,
        ) {
            (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
            _ => false,
        })
    {
        crate::translation::frontend_rules::UnicodeEscape::Malformed
    } else {
        {
            let index = start + 3f64;
            {
                let code = 0f64;
                {
                    let digits = 0f64;
                    {
                        let ml_s1 = crate::translation::lexer::unicode_escapes::ml_decode_rust_unicode_escape_loop1(units, start, index, code, digits);
                        match ml_s1 {
                            crate::translation::lexer::unicode_escapes::MlDecodeRustUnicodeEscapeLoop1Result::Done => {
                                crate::translation::frontend_rules::UnicodeEscape::Malformed
                            }
                            crate::translation::lexer::unicode_escapes::MlDecodeRustUnicodeEscapeLoop1Result::Return(ml_result1) => {
                                *ml_result1
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum MlDecodeRustUnicodeEscapeLoop1Result {
    Done,
    Return(Box<crate::translation::frontend_rules::UnicodeEscape>),
}

/// Decode the shared Rust Unicode scalar escape model.
#[must_use]
fn ml_decode_rust_unicode_escape_loop1(
    units: &[f64],
    start: f64,
    mut index: f64,
    mut code: f64,
    mut digits: f64,
) -> crate::translation::lexer::unicode_escapes::MlDecodeRustUnicodeEscapeLoop1Result {
    loop {
        return if index
            < (serde_json::Number::from(units.len())
                .as_f64()
                .expect("a source unit vector length has a finite binary64 representation"))
        {
            if match (
                ml_array::optional_at(units, ml_array::number_index(index)),
                125f64,
            ) {
                (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                _ => false,
            } {
                if digits < 1f64 {
                    crate::translation::lexer::unicode_escapes::MlDecodeRustUnicodeEscapeLoop1Result::Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Malformed))
                } else {
                    {
                        let end = (index + 1f64) - start;
                        if (code > 1_114_111f64) || ((55_296f64..=57_343f64).contains(&code)) {
                            crate::translation::lexer::unicode_escapes::MlDecodeRustUnicodeEscapeLoop1Result::Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Unsupported(end, end)))
                        } else {
                            crate::translation::lexer::unicode_escapes::MlDecodeRustUnicodeEscapeLoop1Result::Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Scalar(code, end)))
                        }
                    }
                }
            } else if match (
                ml_array::optional_at(units, ml_array::number_index(index)),
                95f64,
            ) {
                (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                _ => false,
            } {
                if digits < 1f64 {
                    crate::translation::lexer::unicode_escapes::MlDecodeRustUnicodeEscapeLoop1Result::Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Malformed))
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
                    let digit =
                        crate::translation::lexer::cooked_strings::read_rust_ascii_escape_digit(
                            (ml_array::optional_at(units, ml_array::number_index(index)))
                                .unwrap_or(-1f64),
                        );
                    if (digit < 0f64) || (digits >= 6f64) {
                        crate::translation::lexer::unicode_escapes::MlDecodeRustUnicodeEscapeLoop1Result::Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Malformed))
                    } else {
                        {
                            let code_2 = {
                                let scaled = code * 16f64;
                                scaled + digit
                            };
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
            crate::translation::lexer::unicode_escapes::MlDecodeRustUnicodeEscapeLoop1Result::Done
        };
    }
}
