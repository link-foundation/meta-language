// Generated from js/src/translation/rust-string-rules.js by
// js/scripts/generate-rust-string-rules.mjs. Do not edit by hand.
/// Read the corresponding shared Rust string lexical decision.
#[must_use]
pub fn accept_rust_string_continuation_whitespace(unit: Option<f64>) -> bool {
    ((unit == Some(9f64) || unit == Some(10f64)) || unit == Some(13f64)) || unit == Some(32f64)
}

/// Read the corresponding shared Rust string lexical decision.
#[must_use]
pub fn read_rust_string_continuation_end(units: &[f64], start: f64) -> f64 {
    if (((start < (0f64)) || (start >= (units.len() as f64))) || ((start % 1f64) != (0f64)))
        || !(match (
            crate::translation::frontend_rules::ml_array::optional_at(
                units,
                crate::translation::frontend_rules::ml_array::number_index(start),
            ),
            92f64,
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
    {
        -1f64
    } else {
        {
            let end = start + 1f64;
            {
                let end_3 = if match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        units,
                        crate::translation::frontend_rules::ml_array::number_index(end),
                    ),
                    13f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                } && match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        units,
                        crate::translation::frontend_rules::ml_array::number_index(end + 1f64),
                    ),
                    10f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                } {
                    { end + 1f64 }
                } else {
                    end
                };
                if (match (
                    crate::translation::frontend_rules::ml_array::optional_at(
                        units,
                        crate::translation::frontend_rules::ml_array::number_index(end_3),
                    ),
                    10f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }) {
                    {
                        let end_4 = end_3 + 1f64;
                        {
                            crate::translation::frontend_rules::ml_read_rust_string_continuation_end_loop1(units, end_4)
                        }
                    }
                } else {
                    -1f64
                }
            }
        }
    }
}

/// Read the corresponding shared Rust string lexical decision.
#[must_use]
pub fn read_rust_source_line_break_end(units: &[f64], start: f64) -> f64 {
    if ((start < (0f64)) || (start >= (units.len() as f64))) || ((start % 1f64) != (0f64)) {
        -1f64
    } else if match (
        crate::translation::frontend_rules::ml_array::optional_at(
            units,
            crate::translation::frontend_rules::ml_array::number_index(start),
        ),
        13f64,
    ) {
        (Some(ml_left), ml_right) => ml_left == ml_right,
        _ => false,
    } && match (
        crate::translation::frontend_rules::ml_array::optional_at(
            units,
            crate::translation::frontend_rules::ml_array::number_index(start + 1f64),
        ),
        10f64,
    ) {
        (Some(ml_left), ml_right) => ml_left == ml_right,
        _ => false,
    } {
        start + 2f64
    } else {
        -1f64
    }
}

/// Read the corresponding shared Rust string lexical decision.
#[must_use]
pub fn ml_read_rust_string_continuation_end_loop1(units: &[f64], mut end: f64) -> f64 {
    loop {
        return if (end < (units.len() as f64))
            && crate::translation::frontend_rules::accept_rust_string_continuation_whitespace(
                crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                ),
            ) {
            {
                let end_2 = end + 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}
