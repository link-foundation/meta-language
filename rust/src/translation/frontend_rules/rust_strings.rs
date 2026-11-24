// Generated from js/src/translation/rust-string-rules.js by
// js/scripts/generate-rust-string-rules.mjs. Do not edit by hand.
/// Read the corresponding shared Rust string lexical decision.
#[must_use]
pub fn accept_rust_string_continuation_whitespace(unit: Option<f64>) -> bool {
    ((({
        let source_ml_source1_comparison_operand1 = unit;
        {
            let source_ml_source1_comparison_operand2 = 9f64;
            match (
                source_ml_source1_comparison_operand1.clone(),
                source_ml_source1_comparison_operand2,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    } || {
        let source_ml_source1_comparison_operand3 = unit;
        {
            let source_ml_source1_comparison_operand4 = 10f64;
            match (
                source_ml_source1_comparison_operand3.clone(),
                source_ml_source1_comparison_operand4,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || {
        let source_ml_source1_comparison_operand5 = unit;
        {
            let source_ml_source1_comparison_operand6 = 13f64;
            match (
                source_ml_source1_comparison_operand5.clone(),
                source_ml_source1_comparison_operand6,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    }) || {
        let source_ml_source1_comparison_operand7 = unit;
        {
            let source_ml_source1_comparison_operand8 = 32f64;
            match (
                source_ml_source1_comparison_operand7.clone(),
                source_ml_source1_comparison_operand8,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }
        }
    })
}

/// Read the corresponding shared Rust string lexical decision.
#[must_use]
pub fn read_rust_string_continuation_end(units: &[f64], start: f64) -> f64 {
    if ((((start < (0f64)) || (start >= (units.len() as f64))) || ((start % 1f64) != (0f64))) || {
        let source_ml_source1_comparison_operand9 =
            crate::translation::frontend_rules::ml_array::optional_at(
                units,
                crate::translation::frontend_rules::ml_array::number_index(start),
            );
        {
            let source_ml_source1_comparison_operand10 = 92f64;
            !(match (
                source_ml_source1_comparison_operand9.clone(),
                source_ml_source1_comparison_operand10,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            })
        }
    }) {
        (-1f64)
    } else {
        {
            let end = (start + 1f64);
            {
                let end_3 = if ({
                    let source_ml_source1_comparison_operand11 =
                        crate::translation::frontend_rules::ml_array::optional_at(
                            units,
                            crate::translation::frontend_rules::ml_array::number_index(end),
                        );
                    {
                        let source_ml_source1_comparison_operand12 = 13f64;
                        match (
                            source_ml_source1_comparison_operand11.clone(),
                            source_ml_source1_comparison_operand12,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } && {
                    let source_ml_source1_comparison_operand13 =
                        crate::translation::frontend_rules::ml_array::optional_at(
                            units,
                            crate::translation::frontend_rules::ml_array::number_index(
                                (end + 1f64),
                            ),
                        );
                    {
                        let source_ml_source1_comparison_operand14 = 10f64;
                        match (
                            source_ml_source1_comparison_operand13.clone(),
                            source_ml_source1_comparison_operand14,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                }) {
                    {
                        let end_2 = (end + 1f64);
                        end_2
                    }
                } else {
                    end
                };
                if {
                    let source_ml_source1_comparison_operand15 =
                        crate::translation::frontend_rules::ml_array::optional_at(
                            units,
                            crate::translation::frontend_rules::ml_array::number_index(end_3),
                        );
                    {
                        let source_ml_source1_comparison_operand16 = 10f64;
                        !(match (
                            source_ml_source1_comparison_operand15.clone(),
                            source_ml_source1_comparison_operand16,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                } {
                    (-1f64)
                } else {
                    {
                        let end_4 = (end_3 + 1f64);
                        {
                            let end_5 =
                                crate::source_ml_temporary_read_rust_string_continuation_end_loop1(
                                    units, end_4,
                                );
                            end_5
                        }
                    }
                }
            }
        }
    }
}

/// Read the corresponding shared Rust string lexical decision.
#[must_use]
pub fn read_rust_source_line_break_end(units: &[f64], start: f64) -> f64 {
    if (((start < (0f64)) || (start >= (units.len() as f64))) || ((start % 1f64) != (0f64))) {
        (-1f64)
    } else {
        if ({
            let source_ml_source1_comparison_operand17 =
                crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index(start),
                );
            {
                let source_ml_source1_comparison_operand18 = 13f64;
                match (
                    source_ml_source1_comparison_operand17.clone(),
                    source_ml_source1_comparison_operand18,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } && {
            let source_ml_source1_comparison_operand19 =
                crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index((start + 1f64)),
                );
            {
                let source_ml_source1_comparison_operand20 = 10f64;
                match (
                    source_ml_source1_comparison_operand19.clone(),
                    source_ml_source1_comparison_operand20,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }) {
            (start + 2f64)
        } else {
            (-1f64)
        }
    }
}

pub fn source_ml_temporary_read_rust_string_continuation_end_loop1(
    units: &[f64],
    mut end: f64,
) -> f64 {
    loop {
        return if ((end < (units.len() as f64))
            && crate::translation::frontend_rules::accept_rust_string_continuation_whitespace(
                crate::translation::frontend_rules::ml_array::optional_at(
                    units,
                    crate::translation::frontend_rules::ml_array::number_index(end),
                ),
            )) {
            {
                let end_2 = (end + 1f64);
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
