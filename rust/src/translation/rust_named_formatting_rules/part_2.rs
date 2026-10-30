// Generated from complete js/src/translation/rust-named-formatting-rules.js. Do not edit by hand.
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
                                                        let source_ml_source1_s14 = self::source_ml_temporary_plan_rust_formatting_field_terms_loop6(units, at, end_2, index);
                                                        match source_ml_source1_s14 {
                                                            self::MlTemporaryPlanRustFormattingFieldTermsLoop6Result::MlTemporaryPlanRustFormattingFieldTermsLoop6Done => {
                                                                {
                                                                    let last = end_2;
                                                                    {
                                                                        let last_2 = self::source_ml_temporary_plan_rust_formatting_field_terms_loop7(units, at, last);
                                                                        {
                                                                            let source_ml_source1_s16 = if last_2 == (at + 1f64) {
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
                                                                                                let source_ml_source1_s17 = self::source_ml_temporary_plan_rust_formatting_field_terms_loop9(units, count, last_2, numeric, position, index_2);
                                                                                                match source_ml_source1_s17 {
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
                                                                            match source_ml_source1_s16 {
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
                let source_ml_source1_comparison_operand18 =
                    self::ml_array::optional_at(plan, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand19 = 1f64;
                    match (
                        source_ml_source1_comparison_operand18,
                        source_ml_source1_comparison_operand19,
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
                let source_ml_source1_comparison_operand20 =
                    self::ml_array::optional_at(ordered, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand21 = expected;
                    !(match (
                        source_ml_source1_comparison_operand20,
                        source_ml_source1_comparison_operand21,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustFormattingArgumentNameErrorLoop12Result {
    MlTemporaryReadRustFormattingArgumentNameErrorLoop12Done,
    MlTemporaryReadRustFormattingArgumentNameErrorLoop12Return(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFormattingArgumentNameErrorLoop13Result {
    MlTemporaryReadRustFormattingArgumentNameErrorLoop13Done(f64),
    MlTemporaryReadRustFormattingArgumentNameErrorLoop13Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_formatting_argument_name_error_loop13(
    names: &[String],
    mut work: f64,
    mut index: f64,
    name: &str,
    mut previous: f64,
) -> self::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Result {
    loop {
        return if previous < index {
            {
                let work_2 = work + 1f64;
                if work_2 > 1_000_000f64 {
                    self::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Result::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Return(String::from("named format arguments exceed their source budget"))
                } else if {
                    let source_ml_source1_comparison_operand22 =
                        self::ml_array::optional_at(names, self::ml_array::number_index(previous));
                    {
                        let source_ml_source1_comparison_operand23 = name.to_owned();
                        match (
                            source_ml_source1_comparison_operand22,
                            source_ml_source1_comparison_operand23,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } {
                    self::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Result::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Return(String::from("format argument name is repeated"))
                } else {
                    {
                        let previous_2 = previous + 1f64;
                        {
                            (work, previous) = (work_2, previous_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Result::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Done(work)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_formatting_argument_name_error_loop12(
    names: &[String],
    mut named: bool,
    mut work: f64,
    mut index: f64,
) -> self::MlTemporaryReadRustFormattingArgumentNameErrorLoop12Result {
    loop {
        return if index < (names.len() as f64) {
            {
                let name =
                    (self::ml_array::optional_at(names, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                if name == "" {
                    if named {
                        self::MlTemporaryReadRustFormattingArgumentNameErrorLoop12Result::MlTemporaryReadRustFormattingArgumentNameErrorLoop12Return(String::from("positional format arguments cannot follow named arguments"))
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
                        let named_2 = true;
                        {
                            let previous = 0f64;
                            {
                                let source_ml_source1_s24 = self::source_ml_temporary_read_rust_formatting_argument_name_error_loop13(names, work, index, &(name), previous);
                                match source_ml_source1_s24 {
                                    self::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Result::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Done(work_2) => {
                                        {
                                            let index_3 = index + 1f64;
                                            {
                                                (named, work, index) = (named_2, work_2, index_3);
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Result::MlTemporaryReadRustFormattingArgumentNameErrorLoop13Return(source_ml_temporary_result13) => {
                                        self::MlTemporaryReadRustFormattingArgumentNameErrorLoop12Result::MlTemporaryReadRustFormattingArgumentNameErrorLoop12Return(source_ml_temporary_result13.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFormattingArgumentNameErrorLoop12Result::MlTemporaryReadRustFormattingArgumentNameErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNamedFormattingValueLoop14Result {
    MlTemporaryPlanRustNamedFormattingValueLoop14Done(Vec<f64>, Vec<f64>, f64),
    MlTemporaryPlanRustNamedFormattingValueLoop14Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustNamedFormattingValueLoop15Result {
    MlTemporaryPlanRustNamedFormattingValueLoop15Done(f64, f64),
    MlTemporaryPlanRustNamedFormattingValueLoop15Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_named_formatting_value_loop15(
    names: &[String],
    mut work: f64,
    name: &str,
    mut position: f64,
    mut index: f64,
) -> self::MlTemporaryPlanRustNamedFormattingValueLoop15Result {
    loop {
        return if index < (names.len() as f64) {
            {
                let work_2 = work + 1f64;
                if work_2 > 1_000_000f64 {
                    self::MlTemporaryPlanRustNamedFormattingValueLoop15Result::MlTemporaryPlanRustNamedFormattingValueLoop15Return(vec![-8f64, 0f64, 0f64])
                } else if {
                    let source_ml_source1_comparison_operand25 =
                        self::ml_array::optional_at(names, self::ml_array::number_index(index));
                    {
                        let source_ml_source1_comparison_operand26 = name.to_owned();
                        match (
                            source_ml_source1_comparison_operand25,
                            source_ml_source1_comparison_operand26,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } {
                    {
                        let position_2 = index;
                        {
                            let index_2 = index + 1f64;
                            {
                                (work, position, index) = (work_2, position_2, index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = index + 1f64;
                        {
                            (work, index) = (work_2, index_3);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNamedFormattingValueLoop15Result::MlTemporaryPlanRustNamedFormattingValueLoop15Done(work, position)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_named_formatting_value_loop14(
    plan: &[f64],
    names: &[String],
    fields: &[String],
    mut result: Vec<f64>,
    mut used: Vec<f64>,
    mut named: f64,
    mut work: f64,
    mut at: f64,
) -> self::MlTemporaryPlanRustNamedFormattingValueLoop14Result {
    loop {
        return if at < (plan.len() as f64) {
            {
                let code = (self::ml_array::optional_at(plan, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                {
                    let first = (self::ml_array::optional_at(
                        plan,
                        self::ml_array::number_index(at + 1f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    {
                        let last = (self::ml_array::optional_at(
                            plan,
                            self::ml_array::number_index(at + 2f64),
                        ))
                        .unwrap_or_else(|| -1f64);
                        if ((!(self::ml_math::is_safe_integer(first))
                            || !(self::ml_math::is_safe_integer(last)))
                            || (first < 0f64))
                            || (last < 0f64)
                        {
                            self::MlTemporaryPlanRustNamedFormattingValueLoop14Result::MlTemporaryPlanRustNamedFormattingValueLoop14Return(vec![-8f64, 0f64, 0f64])
                        } else if code == 0f64 {
                            if last < first {
                                self::MlTemporaryPlanRustNamedFormattingValueLoop14Result::MlTemporaryPlanRustNamedFormattingValueLoop14Return(vec![-8f64, 0f64, 0f64])
                            } else {
                                {
                                    let result_2 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![0f64], result);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let result_3 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![first], result_2);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let result_4 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![last], result_3);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let at_2 = at + 3f64;
                                                {
                                                    (result, at) = (result_4, at_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if code == 1f64 {
                            if (first >= (names.len() as f64)) || (last != 0f64) {
                                self::MlTemporaryPlanRustNamedFormattingValueLoop14Result::MlTemporaryPlanRustNamedFormattingValueLoop14Return(vec![-8f64, 0f64, 0f64])
                            } else {
                                {
                                    let used_2 = {
                                        let (ml_append_right, ml_append_left) = (vec![first], used);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let result_5 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![1f64], result);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let result_6 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![first], result_5);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let result_7 = {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![0f64], result_6);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                };
                                                {
                                                    let at_3 = at + 3f64;
                                                    {
                                                        (result, used, at) =
                                                            (result_7, used_2, at_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if code == 2f64 {
                            if (last <= first) || (named >= (fields.len() as f64)) {
                                self::MlTemporaryPlanRustNamedFormattingValueLoop14Result::MlTemporaryPlanRustNamedFormattingValueLoop14Return(vec![-8f64, 0f64, 0f64])
                            } else {
                                {
                                    let name = (self::ml_array::optional_at(
                                        fields,
                                        self::ml_array::number_index(named),
                                    ))
                                    .unwrap_or_else(|| String::new());
                                    {
                                        let named_2 = named + 1f64;
                                        if name == "" {
                                            self::MlTemporaryPlanRustNamedFormattingValueLoop14Result::MlTemporaryPlanRustNamedFormattingValueLoop14Return(vec![-8f64, 0f64, 0f64])
                                        } else {
                                            {
                                                let position = -1f64;
                                                {
                                                    let index = 0f64;
                                                    {
                                                        let source_ml_source1_s27 = self::source_ml_temporary_plan_rust_named_formatting_value_loop15(names, work, &(name), position, index);
                                                        match source_ml_source1_s27 {
                                                                        self::MlTemporaryPlanRustNamedFormattingValueLoop15Result::MlTemporaryPlanRustNamedFormattingValueLoop15Done(work_2, position_2) => {
                                                                            if position_2 >= 0f64 {
                                                                                {
                                                                                    let used_3 = { let (ml_append_right, ml_append_left) = (vec![position_2], used); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                    {
                                                                                        let result_8 = { let (ml_append_right, ml_append_left) = (vec![1f64], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                        {
                                                                                            let result_9 = { let (ml_append_right, ml_append_left) = (vec![position_2], result_8); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                            {
                                                                                                let result_10 = { let (ml_append_right, ml_append_left) = (vec![0f64], result_9); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                {
                                                                                                    let at_4 = at + 3f64;
                                                                                                    {
                                                                                                        (result, used, named, work, at) = (result_10, used_3, named_2, work_2, at_4);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
{
                                                                                {
                                                                                    let result_11 = { let (ml_append_right, ml_append_left) = (vec![2f64], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                    {
                                                                                        let result_12 = { let (ml_append_right, ml_append_left) = (vec![first], result_11); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                        {
                                                                                            let result_13 = { let (ml_append_right, ml_append_left) = (vec![last], result_12); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                            {
                                                                                                let at_5 = at + 3f64;
                                                                                                {
                                                                                                    (result, named, work, at) = (result_13, named_2, work_2, at_5);
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlTemporaryPlanRustNamedFormattingValueLoop15Result::MlTemporaryPlanRustNamedFormattingValueLoop15Return(source_ml_temporary_result15) => {
                                                                            self::MlTemporaryPlanRustNamedFormattingValueLoop14Result::MlTemporaryPlanRustNamedFormattingValueLoop14Return(source_ml_temporary_result15)
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
                            self::MlTemporaryPlanRustNamedFormattingValueLoop14Result::MlTemporaryPlanRustNamedFormattingValueLoop14Return(vec![-8f64, 0f64, 0f64])
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustNamedFormattingValueLoop14Result::MlTemporaryPlanRustNamedFormattingValueLoop14Done(result, used, named)
        };
    }
}
