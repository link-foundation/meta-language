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
        || {
            let source_ml_source1_comparison_operand1 =
                ml_array::optional_at(units, ml_array::number_index(start));
            {
                let source_ml_source1_comparison_operand2 = 92f64;
                !(match (
                    source_ml_source1_comparison_operand1,
                    source_ml_source1_comparison_operand2,
                ) {
                    (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                    _ => false,
                })
            }
        })
        || {
            let source_ml_source1_comparison_operand3 =
                ml_array::optional_at(units, ml_array::number_index(start + 1f64));
            {
                let source_ml_source1_comparison_operand4 = 117f64;
                !(match (
                    source_ml_source1_comparison_operand3,
                    source_ml_source1_comparison_operand4,
                ) {
                    (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                    _ => false,
                })
            }
        })
        || {
            let source_ml_source1_comparison_operand5 =
                ml_array::optional_at(units, ml_array::number_index(start + 2f64));
            {
                let source_ml_source1_comparison_operand6 = 123f64;
                !(match (
                    source_ml_source1_comparison_operand5,
                    source_ml_source1_comparison_operand6,
                ) {
                    (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                    _ => false,
                })
            }
        }
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
                        let source_ml_source1_s7 =
                            crate::source_ml_temporary_decode_rust_unicode_escape_loop1(
                                units, start, index, code, digits,
                            );
                        match source_ml_source1_s7 {
                            crate::MlTemporaryDecodeRustUnicodeEscapeLoop1Result::MlTemporaryDecodeRustUnicodeEscapeLoop1Done => {
                                crate::translation::frontend_rules::UnicodeEscape::Malformed
                            }
                            crate::MlTemporaryDecodeRustUnicodeEscapeLoop1Result::MlTemporaryDecodeRustUnicodeEscapeLoop1Return(source_ml_temporary_result1) => {
                                *source_ml_temporary_result1
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDecodeRustUnicodeEscapeLoop1Result {
    MlTemporaryDecodeRustUnicodeEscapeLoop1Done,
    MlTemporaryDecodeRustUnicodeEscapeLoop1Return(
        Box<crate::translation::frontend_rules::UnicodeEscape>,
    ),
}

/// Decode the shared Rust Unicode scalar escape model.
#[must_use]
pub fn source_ml_temporary_decode_rust_unicode_escape_loop1(
    units: &[f64],
    start: f64,
    mut index: f64,
    mut code: f64,
    mut digits: f64,
) -> crate::MlTemporaryDecodeRustUnicodeEscapeLoop1Result {
    loop {
        return if index
            < (serde_json::Number::from(units.len())
                .as_f64()
                .expect("a source unit vector length has a finite binary64 representation"))
        {
            if {
                let source_ml_source1_comparison_operand8 =
                    ml_array::optional_at(units, ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand9 = 125f64;
                    match (
                        source_ml_source1_comparison_operand8,
                        source_ml_source1_comparison_operand9,
                    ) {
                        (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                        _ => false,
                    }
                }
            } {
                if digits < 1f64 {
                    crate::MlTemporaryDecodeRustUnicodeEscapeLoop1Result::MlTemporaryDecodeRustUnicodeEscapeLoop1Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Malformed))
                } else {
                    {
                        let end = (index + 1f64) - start;
                        if (code > 1_114_111f64) || ((55_296f64..=57_343f64).contains(&code)) {
                            crate::MlTemporaryDecodeRustUnicodeEscapeLoop1Result::MlTemporaryDecodeRustUnicodeEscapeLoop1Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Unsupported(end, end)))
                        } else {
                            crate::MlTemporaryDecodeRustUnicodeEscapeLoop1Result::MlTemporaryDecodeRustUnicodeEscapeLoop1Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Scalar(code, end)))
                        }
                    }
                }
            } else {
                if {
                    let source_ml_source1_comparison_operand10 =
                        ml_array::optional_at(units, ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand11 = 95f64;
                        match (
                            source_ml_source1_comparison_operand10,
                            source_ml_source1_comparison_operand11,
                        ) {
                            (Some(ml_left), ml_right) => ml_left.to_bits() == ml_right.to_bits(),
                            _ => false,
                        }
                    }
                } {
                    if digits < 1f64 {
                        crate::MlTemporaryDecodeRustUnicodeEscapeLoop1Result::MlTemporaryDecodeRustUnicodeEscapeLoop1Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Malformed))
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
                            crate::MlTemporaryDecodeRustUnicodeEscapeLoop1Result::MlTemporaryDecodeRustUnicodeEscapeLoop1Return(Box::new(crate::translation::frontend_rules::UnicodeEscape::Malformed))
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
            }
        } else {
            crate::MlTemporaryDecodeRustUnicodeEscapeLoop1Result::MlTemporaryDecodeRustUnicodeEscapeLoop1Done
        };
    }
}
