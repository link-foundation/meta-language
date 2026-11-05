// Generated from complete js/src/translation/rust-formatting-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustFormattingFieldTermsJoin8;
use super::MlTemporaryPlanRustFormattingFieldTermsLoop4Result;
use super::MlTemporaryPlanRustFormattingFieldTermsLoop6Result;
use super::MlTemporaryPlanRustFormattingFieldTermsLoop9Result;
use super::ml_array;
use super::ml_math;
use super::source_ml_temporary_plan_rust_formatting_field_terms_loop5;
use super::source_ml_temporary_plan_rust_formatting_field_terms_loop6;
use super::source_ml_temporary_plan_rust_formatting_field_terms_loop7;
use super::source_ml_temporary_plan_rust_formatting_field_terms_loop9;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_formatting_field_terms_loop4(
    units: &[f64],
    mut count: f64,
    mut plan: Vec<f64>,
    mut at: f64,
    mut text_start: f64,
    mut next: f64,
) -> self::MlTemporaryPlanRustFormattingFieldTermsLoop4Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                {
                    let following = (self::ml_array::optional_at(
                        units,
                        self::ml_array::number_index(at + 1f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    if ((unit == 123f64) && (following == 123f64))
                        || ((unit == 125f64) && (following == 125f64))
                    {
                        {
                            let plan_5 = if text_start < at {
                                {
                                    let plan_2 = {
                                        let (ml_append_right, ml_append_left) = (vec![0f64], plan);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let plan_3 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![text_start], plan_2);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![at], plan_3);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            }
                                        }
                                    }
                                }
                            } else {
                                plan
                            };
                            {
                                let plan_6 = {
                                    let (ml_append_right, ml_append_left) = (vec![0f64], plan_5);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let plan_7 = {
                                        let (ml_append_right, ml_append_left) = (vec![at], plan_6);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let plan_8 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![(at + 1f64)], plan_7);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let at_2 = at + 2f64;
                                            {
                                                let text_start_2 = at_2;
                                                {
                                                    (plan, at, text_start) =
                                                        (plan_8, at_2, text_start_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {
                        if unit == 125f64 {
                            self::MlTemporaryPlanRustFormattingFieldTermsLoop4Result::MlTemporaryPlanRustFormattingFieldTermsLoop4Return(vec![-1f64, at, (at + 1f64)])
                        } else {
                            if unit != 123f64 {
                                {
                                    let at_3 = at + 1f64;
                                    {
                                        at = at_3;
                                        continue;
                                    }
                                }
                            }
                            {
                                {
                                    let plan_12 = if text_start < at {
                                        {
                                            let plan_9 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![0f64], plan);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let plan_10 = {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![text_start], plan_9);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                };
                                                {
                                                    {
                                                        let (ml_append_right, ml_append_left) =
                                                            (vec![at], plan_10);
                                                        self::ml_array::append(
                                                            ml_append_left,
                                                            ml_append_right,
                                                        )
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        plan
                                    };
                                    {
                                        let end = at + 1f64;
                                        {
                                            let end_2 = self::source_ml_temporary_plan_rust_formatting_field_terms_loop5(units, end);
                                            if end_2 >= (units.len() as f64) {
                                                self::MlTemporaryPlanRustFormattingFieldTermsLoop4Result::MlTemporaryPlanRustFormattingFieldTermsLoop4Return(vec![-2f64, at, (units.len() as f64)])
                                            } else {
                                                {
                                                    let index = at + 1f64;
                                                    {
                                                        let source_ml_source1_s10 = self::source_ml_temporary_plan_rust_formatting_field_terms_loop6(units, at, end_2, index);
                                                        match source_ml_source1_s10 {
                                                            self::MlTemporaryPlanRustFormattingFieldTermsLoop6Result::MlTemporaryPlanRustFormattingFieldTermsLoop6Done => {
                                                                {
                                                                    let last = end_2;
                                                                    {
                                                                        let last_2 = self::source_ml_temporary_plan_rust_formatting_field_terms_loop7(units, at, last);
                                                                        {
                                                                            let source_ml_source1_s12 = if last_2 == (at + 1f64) {
                                                                                if next >= count {
                                                                                    self::MlTemporaryPlanRustFormattingFieldTermsJoin8::MlTemporaryPlanRustFormattingFieldTermsJoin8Return(vec![-5f64, (at + 1f64), last_2])
                                                                                } else {
                                                                                    {
                                                                                        let plan_13 = { let (ml_append_right, ml_append_left) = (vec![1f64], plan_12); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                        {
                                                                                            let plan_14 = { let (ml_append_right, ml_append_left) = (vec![next], plan_13); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                            {
                                                                                                let plan_15 = { let (ml_append_right, ml_append_left) = (vec![0f64], plan_14); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                {
                                                                                                    let next_2 = next + 1f64;
                                                                                                    self::MlTemporaryPlanRustFormattingFieldTermsJoin8::MlTemporaryPlanRustFormattingFieldTermsJoin8Next(plan_15, next_2)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                {
                                                                                    let numeric = true;
                                                                                    {
                                                                                        let position = 0f64;
                                                                                        {
                                                                                            let index_2 = at + 1f64;
                                                                                            {
                                                                                                let source_ml_source1_s13 = self::source_ml_temporary_plan_rust_formatting_field_terms_loop9(units, count, last_2, numeric, position, index_2);
                                                                                                match source_ml_source1_s13 {
                                                                                                    self::MlTemporaryPlanRustFormattingFieldTermsLoop9Result::MlTemporaryPlanRustFormattingFieldTermsLoop9Done(numeric_2, position_2) => {
                                                                                                        if numeric_2 {
                                                                                                            if position_2 >= count {
                                                                                                                self::MlTemporaryPlanRustFormattingFieldTermsJoin8::MlTemporaryPlanRustFormattingFieldTermsJoin8Return(vec![-6f64, (at + 1f64), last_2])
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let plan_16 = { let (ml_append_right, ml_append_left) = (vec![1f64], plan_12); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                    {
                                                                                                                        let plan_17 = { let (ml_append_right, ml_append_left) = (vec![position_2], plan_16); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                        {
                                                                                                                            let plan_18 = { let (ml_append_right, ml_append_left) = (vec![0f64], plan_17); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                            self::MlTemporaryPlanRustFormattingFieldTermsJoin8::MlTemporaryPlanRustFormattingFieldTermsJoin8Next(plan_18, next)
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            {
                                                                                                                let plan_19 = { let (ml_append_right, ml_append_left) = (vec![2f64], plan_12); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                {
                                                                                                                    let plan_20 = { let (ml_append_right, ml_append_left) = (vec![(at + 1f64)], plan_19); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                    {
                                                                                                                        let plan_21 = { let (ml_append_right, ml_append_left) = (vec![last_2], plan_20); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                        self::MlTemporaryPlanRustFormattingFieldTermsJoin8::MlTemporaryPlanRustFormattingFieldTermsJoin8Next(plan_21, next)
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
                                                                            };
                                                                            match source_ml_source1_s12 {
                                                                                self::MlTemporaryPlanRustFormattingFieldTermsJoin8::MlTemporaryPlanRustFormattingFieldTermsJoin8Next(plan_22, next_3) => {
                                                                                    {
                                                                                        let at_4 = end_2 + 1f64;
                                                                                        {
                                                                                            let text_start_3 = at_4;
                                                                                            {
                                                                                                (plan, at, text_start, next) = (plan_22, at_4, text_start_3, next_3);
                                                                                                continue;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryPlanRustFormattingFieldTermsJoin8::MlTemporaryPlanRustFormattingFieldTermsJoin8Return(source_ml_temporary_result8) => {
                                                                                    self::MlTemporaryPlanRustFormattingFieldTermsLoop4Result::MlTemporaryPlanRustFormattingFieldTermsLoop4Return(source_ml_temporary_result8)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryPlanRustFormattingFieldTermsLoop6Result::MlTemporaryPlanRustFormattingFieldTermsLoop6Return(source_ml_temporary_result9) => {
                                                                self::MlTemporaryPlanRustFormattingFieldTermsLoop4Result::MlTemporaryPlanRustFormattingFieldTermsLoop4Return(source_ml_temporary_result9)
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
        } else {
            self::MlTemporaryPlanRustFormattingFieldTermsLoop4Result::MlTemporaryPlanRustFormattingFieldTermsLoop4Done(plan, text_start)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustFormattingFieldTermsLoop3Result {
    MlTemporaryPlanRustFormattingFieldTermsLoop3Done,
    MlTemporaryPlanRustFormattingFieldTermsLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_formatting_field_terms_loop3(
    units: &[f64],
    mut index: f64,
) -> self::MlTemporaryPlanRustFormattingFieldTermsLoop3Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let unit =
                    (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64)
                {
                    self::MlTemporaryPlanRustFormattingFieldTermsLoop3Result::MlTemporaryPlanRustFormattingFieldTermsLoop3Return(vec![-8f64, index, (index + 1f64)])
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustFormattingFieldTermsLoop3Result::MlTemporaryPlanRustFormattingFieldTermsLoop3Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_formatting_value_loop10(
    plan: &[f64],
    mut used: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        return if at < (plan.len() as f64) {
            if {
                let source_ml_source1_comparison_operand14 =
                    self::ml_array::optional_at(plan, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand15 = 1f64;
                    match (
                        source_ml_source1_comparison_operand14,
                        source_ml_source1_comparison_operand15,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } {
                {
                    let used_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![
                                (self::ml_array::optional_at(
                                    plan,
                                    self::ml_array::number_index(at + 1f64),
                                ))
                                .unwrap_or_else(|| -1f64),
                            ],
                            used,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let at_2 = at + 3f64;
                        {
                            (used, at) = (used_2, at_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let at_3 = at + 3f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            used
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFormattingUnusedPositionLoop11Result {
    MlTemporaryReadRustFormattingUnusedPositionLoop11Done(f64),
    MlTemporaryReadRustFormattingUnusedPositionLoop11Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_formatting_unused_position_loop11(
    ordered: &[f64],
    mut expected: f64,
    mut index: f64,
) -> self::MlTemporaryReadRustFormattingUnusedPositionLoop11Result {
    loop {
        return if index < (ordered.len() as f64) {
            if {
                let source_ml_source1_comparison_operand16 =
                    self::ml_array::optional_at(ordered, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand17 = expected;
                    !(match (
                        source_ml_source1_comparison_operand16,
                        source_ml_source1_comparison_operand17,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryReadRustFormattingUnusedPositionLoop11Result::MlTemporaryReadRustFormattingUnusedPositionLoop11Return(vec![-7f64, expected, (expected + 1f64)])
            } else {
                {
                    let expected_2 = expected + 1f64;
                    {
                        let index_2 = index + 1f64;
                        {
                            (expected, index) = (expected_2, index_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFormattingUnusedPositionLoop11Result::MlTemporaryReadRustFormattingUnusedPositionLoop11Done(expected)
        };
    }
}
