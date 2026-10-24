// Generated from complete js/src/translation/rust-formatting-rules.js. Do not edit by hand.
use super::MlTemporaryPlanRustFormattingValueLoop3Result;
use super::source_ml_temporary_plan_rust_formatting_value_loop3;
use super::source_ml_temporary_plan_rust_formatting_value_loop4;

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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    (((((((((((9f64..=13f64).contains(&unit)) || (unit == 32f64)) || (unit == 160f64))
        || (unit == 5760f64))
        || ((8192f64..=8202f64).contains(&unit)))
        || (unit == 8232f64))
        || (unit == 8233f64))
        || (unit == 8239f64))
        || (unit == 8287f64))
        || (unit == 12_288f64))
        || (unit == 65_279f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_formatting_whitespace(unit: f64) -> bool {
    (unit == 133f64) || ((unit != 65_279f64) && self::accept_java_script_whitespace_unit(unit))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_formatting_operand_capture(count: f64) -> Vec<f64> {
    if !(self::ml_math::is_safe_integer(count)) || (count < 0f64) {
        Vec::<f64>::new()
    } else {
        {
            let result = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    self::source_ml_temporary_plan_rust_formatting_operand_capture_loop1(
                        count, result, index,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn sort_rust_formatting_positions(values: &[f64]) -> Vec<f64> {
    if (values.len() as f64) <= 1f64 {
        values.to_vec()
    } else {
        {
            let middle = ((values.len() as f64) / 2f64).floor();
            {
                let first = self::sort_rust_formatting_positions(
                    &(self::ml_array::slice(values, 0f64, middle)),
                );
                {
                    let second = self::sort_rust_formatting_positions(
                        &(self::ml_array::slice(values, middle, f64::INFINITY)),
                    );
                    {
                        let result = Vec::<f64>::new();
                        {
                            let left = 0f64;
                            {
                                let right = 0f64;
                                {
                                    self::source_ml_temporary_sort_rust_formatting_positions_loop2(
                                        &(first),
                                        &(second),
                                        result,
                                        left,
                                        right,
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_formatting_value(units: &[f64], count: f64) -> Vec<f64> {
    if !(self::ml_math::is_safe_integer(count)) || (count < 0f64) {
        vec![-8f64, 0f64, 0f64]
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s1 =
                    self::source_ml_temporary_plan_rust_formatting_value_loop3(units, index);
                match source_ml_source1_s1 {
                    self::MlTemporaryPlanRustFormattingValueLoop3Result::MlTemporaryPlanRustFormattingValueLoop3Done => {
                        {
                            let plan = vec![0f64];
                            {
                                let used = Vec::<f64>::new();
                                {
                                    let at = 0f64;
                                    {
                                        let text_start = 0f64;
                                        {
                                            let next = 0f64;
                                            {
                                                let source_ml_source1_s2 = self::source_ml_temporary_plan_rust_formatting_value_loop4(units, count, plan, used, at, text_start, next);
                                                match source_ml_source1_s2 {
                                                    self::MlTemporaryPlanRustFormattingValueLoop4Result::MlTemporaryPlanRustFormattingValueLoop4Done(plan_2, used_2, text_start_2) => {
                                                        {
                                                            let plan_6 = if (text_start_2 < (units.len() as f64)) || ((plan_2.len() as f64) == 1f64) {
                                                                {
                                                                    let plan_3 = { let (ml_append_right, ml_append_left) = (vec![0f64], plan_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        let plan_4 = { let (ml_append_right, ml_append_left) = (vec![text_start_2], plan_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            { let (ml_append_right, ml_append_left) = (vec![(units.len() as f64)], plan_4); self::ml_array::append(ml_append_left, ml_append_right) }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                plan_2
                                                            };
                                                            {
                                                                let ordered = self::sort_rust_formatting_positions(&(used_2));
                                                                {
                                                                    let expected = 0f64;
                                                                    {
                                                                        let index_2 = 0f64;
                                                                        {
                                                                            let source_ml_source1_s3 = self::source_ml_temporary_plan_rust_formatting_value_loop5(&(ordered), expected, index_2);
                                                                            match source_ml_source1_s3 {
                                                                                self::MlTemporaryPlanRustFormattingValueLoop5Result::MlTemporaryPlanRustFormattingValueLoop5Done(expected_2) => {
                                                                                    if expected_2 < count {
                                                                                        vec![-7f64, expected_2, (expected_2 + 1f64)]
                                                                                    } else {
                                                                                        plan_6
                                                                                    }
                                                                                }
                                                                                self::MlTemporaryPlanRustFormattingValueLoop5Result::MlTemporaryPlanRustFormattingValueLoop5Return(source_ml_temporary_result5) => {
                                                                                    source_ml_temporary_result5
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryPlanRustFormattingValueLoop4Result::MlTemporaryPlanRustFormattingValueLoop4Return(source_ml_temporary_result5_2) => {
                                                        source_ml_temporary_result5_2
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryPlanRustFormattingValueLoop3Result::MlTemporaryPlanRustFormattingValueLoop3Return(source_ml_temporary_result10) => {
                        source_ml_temporary_result10
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_formatting_operand_capture_loop1(
    mut count: f64,
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (vec![index], result);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_sort_rust_formatting_positions_loop2(
    first: &[f64],
    second: &[f64],
    mut result: Vec<f64>,
    mut left: f64,
    mut right: f64,
) -> Vec<f64> {
    loop {
        if (left < (first.len() as f64)) || (right < (second.len() as f64)) {
            {
                let one = (self::ml_array::optional_at(first, self::ml_array::number_index(left)))
                    .unwrap_or_else(|| -1f64);
                {
                    let two =
                        (self::ml_array::optional_at(second, self::ml_array::number_index(right)))
                            .unwrap_or_else(|| -1f64);
                    if left >= (first.len() as f64) {
                        {
                            let result_2 = {
                                let (ml_append_right, ml_append_left) = (vec![two], result);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let right_2 = right + 1f64;
                                {
                                    (result, right) = (result_2, right_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if right >= (second.len() as f64) {
                            {
                                let result_3 = {
                                    let (ml_append_right, ml_append_left) = (vec![one], result);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let left_2 = left + 1f64;
                                    {
                                        (result, left) = (result_3, left_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if one < two {
                                {
                                    let result_4 = {
                                        let (ml_append_right, ml_append_left) = (vec![one], result);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let left_3 = left + 1f64;
                                        {
                                            (result, left) = (result_4, left_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if one > two {
                                    {
                                        let result_5 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![two], result);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let right_3 = right + 1f64;
                                            {
                                                (result, right) = (result_5, right_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let result_6 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![one], result);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let left_4 = left + 1f64;
                                            {
                                                let right_4 = right + 1f64;
                                                {
                                                    (result, left, right) =
                                                        (result_6, left_4, right_4);
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
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustFormattingValueLoop5Result {
    MlTemporaryPlanRustFormattingValueLoop5Done(f64),
    MlTemporaryPlanRustFormattingValueLoop5Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_formatting_value_loop5(
    ordered: &[f64],
    mut expected: f64,
    mut index: f64,
) -> self::MlTemporaryPlanRustFormattingValueLoop5Result {
    loop {
        return if index < (ordered.len() as f64) {
            if {
                let source_ml_source1_comparison_operand4 =
                    self::ml_array::optional_at(ordered, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand5 = expected;
                    !(match (
                        source_ml_source1_comparison_operand4,
                        source_ml_source1_comparison_operand5,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryPlanRustFormattingValueLoop5Result::MlTemporaryPlanRustFormattingValueLoop5Return(vec![-7f64, expected, (expected + 1f64)])
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
            self::MlTemporaryPlanRustFormattingValueLoop5Result::MlTemporaryPlanRustFormattingValueLoop5Done(expected)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustFormattingValueLoop4Result {
    MlTemporaryPlanRustFormattingValueLoop4Done(Vec<f64>, Vec<f64>, f64),
    MlTemporaryPlanRustFormattingValueLoop4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_formatting_value_loop6(units: &[f64], mut end: f64) -> f64 {
    loop {
        return if (end < (units.len() as f64)) && {
            let source_ml_source1_comparison_operand6 =
                self::ml_array::optional_at(units, self::ml_array::number_index(end));
            {
                let source_ml_source1_comparison_operand7 = 125f64;
                !(match (
                    source_ml_source1_comparison_operand6,
                    source_ml_source1_comparison_operand7,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })
            }
        } {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_formatting_value_loop8(
    units: &[f64],
    mut at: f64,
    mut last: f64,
) -> f64 {
    loop {
        if (last > (at + 1f64))
            && self::accept_rust_formatting_whitespace(
                (self::ml_array::optional_at(units, self::ml_array::number_index(last - 1f64)))
                    .unwrap_or_else(|| 0f64),
            )
        {
            {
                let last_2 = last - 1f64;
                {
                    last = last_2;
                    continue;
                }
            }
        }
        return { last };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustFormattingValueJoin9 {
    MlTemporaryPlanRustFormattingValueJoin9Next(Vec<f64>, Vec<f64>, f64),
    MlTemporaryPlanRustFormattingValueJoin9Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustFormattingValueLoop10Result {
    MlTemporaryPlanRustFormattingValueLoop10Done(bool, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_formatting_value_loop10(
    units: &[f64],
    mut count: f64,
    mut last: f64,
    mut numeric: bool,
    mut position: f64,
    mut index: f64,
) -> self::MlTemporaryPlanRustFormattingValueLoop10Result {
    loop {
        return if index < last {
            {
                let digit =
                    (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| 0f64)
                        - 48f64;
                if (digit < 0f64) || (digit > 9f64) {
                    {
                        let numeric_2 = false;
                        self::MlTemporaryPlanRustFormattingValueLoop10Result::MlTemporaryPlanRustFormattingValueLoop10Done(numeric_2, position)
                    }
                } else {
                    {
                        let position_2 =
                            self::ml_math::min((count + 1f64), ((position * 10f64) + digit));
                        {
                            let index_2 = index + 1f64;
                            {
                                (position, index) = (position_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustFormattingValueLoop10Result::MlTemporaryPlanRustFormattingValueLoop10Done(numeric, position)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustFormattingValueLoop7Result {
    MlTemporaryPlanRustFormattingValueLoop7Done,
    MlTemporaryPlanRustFormattingValueLoop7Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_formatting_value_loop7(
    units: &[f64],
    mut at: f64,
    mut end: f64,
    mut index: f64,
) -> self::MlTemporaryPlanRustFormattingValueLoop7Result {
    loop {
        return if index < end {
            if {
                let source_ml_source1_comparison_operand8 =
                    self::ml_array::optional_at(units, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand9 = 58f64;
                    match (
                        source_ml_source1_comparison_operand8,
                        source_ml_source1_comparison_operand9,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } {
                self::MlTemporaryPlanRustFormattingValueLoop7Result::MlTemporaryPlanRustFormattingValueLoop7Return(vec![-3f64, (at + 1f64), end])
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
            self::MlTemporaryPlanRustFormattingValueLoop7Result::MlTemporaryPlanRustFormattingValueLoop7Done
        };
    }
}
