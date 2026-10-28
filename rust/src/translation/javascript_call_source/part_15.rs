// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Result;
use super::accept_java_script_whitespace_unit;
use super::divide_number_power_bound;
use super::index_utf16_text;
use super::invert_number_power_interval;
use super::match_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::multiply_number_power_interval;
use super::power_java_script_big_integer;
use super::root_number_power_interval;
use super::round_number_power_bound;
use super::slice_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_normalize_number_power_bound_loop52(mut value: self::ml::Big, mut exponent: self::ml::Big, mut scale: self::ml::Big, mut upward: bool) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Result {
    loop {
        return if (value >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let value_2 = self::ml::divide(&value.add(&if upward {
                    self::ml::Big::from_i128(1)
                } else {
                    self::ml::Big::from_i128(0)
                }), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let exponent_2 = exponent.add(&self::ml::Big::from_i128(1));
                    {
                        (value, exponent) = (value_2.clone(), exponent_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Done(value.clone(), exponent.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_root_number_power_integer_loop54(mut root: self::ml::Big, mut remaining: self::ml::Big) -> self::ml::Big {
    loop {
        return if (remaining > (self::ml::Big::from_i128(1))) {
            {
                let remaining_2 = self::ml::divide(&remaining, &self::ml::Big::from_i128(4), "trunc", Some("Division by zero"), false);
                {
                    let root_2 = root.mul(&self::ml::Big::from_i128(2));
                    {
                        (root, remaining) = (root_2.clone(), remaining_2.clone());
                        continue;
                    }
                }
            }
        } else {
            root.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_root_number_power_integer_loop55(mut value: self::ml::Big, mut root: self::ml::Big) -> self::ml::Big {
    loop {
        return {
            let next = self::ml::divide(&root.add(&self::divide_number_power_bound(value.clone(), root.clone(), false)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
            if (next >= root) {
                root.clone()
            } else {
                {
                    let root_2 = next.clone();
                    {
                        root = root_2.clone();
                        continue;
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRootNumberPowerBoundJoin56 {
    MlSource1TemporaryRootNumberPowerBoundJoin56Next(self::ml::Big, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop58Result {
    MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop58Done(f64, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop58(mut value: f64, mut exponent: self::ml::Big) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop58Result {
    loop {
        return if (value < (1f64)) {
            {
                let value_2 = (value * 2f64);
                {
                    let exponent_2 = exponent.sub(&self::ml::Big::from_i128(1));
                    {
                        (value, exponent) = (value_2, exponent_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop58Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop58Done(value, exponent.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Result {
    MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Done(f64, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop57(mut value: f64, mut exponent: self::ml::Big) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Result {
    loop {
        return if (value >= (2f64)) {
            {
                let value_2 = (value / 2f64);
                {
                    let exponent_2 = exponent.add(&self::ml::Big::from_i128(1));
                    {
                        (value, exponent) = (value_2, exponent_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Done(value, exponent.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_number_loop61(mut scale: self::ml::Big, mut interval: Vec<self::ml::Big>, mut shift: self::ml::Big) -> Vec<self::ml::Big> {
    loop {
        return if (shift < (self::ml::Big::from_i128(0))) {
            {
                let interval_2 = self::root_number_power_interval(interval, scale.clone());
                {
                    let shift_2 = shift.add(&self::ml::Big::from_i128(1));
                    {
                        (interval, shift) = (interval_2, shift_2.clone());
                        continue;
                    }
                }
            }
        } else {
            interval
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_number_loop62(mut scale: self::ml::Big, mut interval: Vec<self::ml::Big>, mut result: Vec<self::ml::Big>, mut remaining: self::ml::Big) -> Vec<self::ml::Big> {
    loop {
        return if (remaining > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = self::multiply_number_power_interval(result, interval.clone(), scale.clone());
                        result_2
                    }
                } else {
                    result
                };
                {
                    let remaining_2 = self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let interval_2 = self::multiply_number_power_interval(interval.clone(), interval, scale.clone());
                            {
                                (interval, result, remaining) = (interval_2, result_3, remaining_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (result, remaining) = (result_3, remaining_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPowerJavaScriptNumberLoop60Result {
    MlSource1TemporaryPowerJavaScriptNumberLoop60Done(Vec<self::ml::Big>, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_number_loop60(mut scale: self::ml::Big, mut interval: Vec<self::ml::Big>, mut shift: self::ml::Big) -> self::MlSource1TemporaryPowerJavaScriptNumberLoop60Result {
    loop {
        return if (shift > (self::ml::Big::from_i128(0))) {
            {
                let interval_2 = self::multiply_number_power_interval(interval.clone(), interval, scale.clone());
                {
                    let shift_2 = shift.sub(&self::ml::Big::from_i128(1));
                    {
                        (interval, shift) = (interval_2, shift_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPowerJavaScriptNumberLoop60Result::MlSource1TemporaryPowerJavaScriptNumberLoop60Done(interval, shift.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_number_loop59(mut exponent: f64, mut negative: bool, mut input: Vec<self::ml::Big>, mut power: Vec<self::ml::Big>, mut precision: self::ml::Big) -> f64 {
    loop {
        return {
            let scale = (self::ml_array::optional_at(&self::power_java_script_big_integer(self::ml::Big::from_i128(2), precision.clone()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
            {
                let value = ((self::ml_array::optional_at(&input, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).mul(&(self::ml_array::optional_at(&self::power_java_script_big_integer(self::ml::Big::from_i128(2), precision.sub(&self::ml::Big::from_i128(52))), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                {
                    let initial_exponent = ((self::ml_array::optional_at(&input, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).add(&self::ml::Big::from_i128(52));
                    {
                        let interval = vec![value.clone(), initial_exponent.clone(), value.clone(), initial_exponent.clone()];
                        {
                            let interval_3 = if (exponent < (0f64)) {
                                {
                                    let interval_2 = self::invert_number_power_interval(interval, scale.clone());
                                    interval_2
                                }
                            } else {
                                interval
                            };
                            {
                                let shift = (self::ml_array::optional_at(&power, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                {
                                    let source_ml_source2_s592 = self::source_ml_source1_temporary_power_java_script_number_loop60(scale.clone(), interval_3, shift.clone());
                                    match source_ml_source2_s592.clone() {
                                        self::MlSource1TemporaryPowerJavaScriptNumberLoop60Result::MlSource1TemporaryPowerJavaScriptNumberLoop60Done(interval_4, shift_2) => {
                                            {
                                                let interval_5 = self::source_ml_source1_temporary_power_java_script_number_loop61(scale.clone(), interval_4, shift_2.clone());
                                                {
                                                    let result = vec![scale.clone(), self::ml::Big::from_i128(0), scale.clone(), self::ml::Big::from_i128(0)];
                                                    {
                                                        let remaining = (self::ml_array::optional_at(&power, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                        {
                                                            let result_2 = self::source_ml_source1_temporary_power_java_script_number_loop62(scale.clone(), interval_5, result, remaining.clone());
                                                            {
                                                                let lower = self::round_number_power_bound((self::ml_array::optional_at(&result_2, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&result_2, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), precision.clone());
                                                                {
                                                                    let upper = self::round_number_power_bound((self::ml_array::optional_at(&result_2, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&result_2, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), precision.clone());
                                                                    if (lower == upper) {
                                                                        if negative {
                                                                            (-lower)
                                                                        } else {
                                                                            lower
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let precision_2 = precision.mul(&self::ml::Big::from_i128(2));
                                                                            {
                                                                                precision = precision_2.clone();
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
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Result {
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Done(f64),
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_integer_width_loop63(mut rest: self::ml::Big, mut width: f64) -> self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Result {
    loop {
        return if (rest > (self::ml::Big::from_i128(0))) {
            if (width >= (4096f64)) {
                self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Return((-1f64))
            } else {
                {
                    let rest_2 = self::ml::divide(&rest, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let width_2 = (width + 1f64);
                        {
                            (rest, width) = (rest_2.clone(), width_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Done(width)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Next(self::ml::Big),
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next,
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_slice_utf16_text_loop66(text: &[f64], mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_points_utf16_text_loop67(text: &[f64], mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![first, second]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    let at_3 = (at_2 + 1f64);
                                    {
                                        (result, at) = (result_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![vec![first]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_4 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_4);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_split_utf16_text_loop68(text: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if ((at < ((text.len() as f64))) && (((result.len() as f64)) < count)) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporarySplitUtf16TextLoop69Result {
    MlSource1TemporarySplitUtf16TextLoop69Done(Vec<Vec<f64>>, f64),
    MlSource1TemporarySplitUtf16TextLoop69Return(Vec<Vec<f64>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_split_utf16_text_loop69(text: &[f64], separator: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut start: f64) -> self::MlSource1TemporarySplitUtf16TextLoop69Result {
    loop {
        return {
            let found = self::index_utf16_text(text, separator, start);
            if (found < (0f64)) {
                self::MlSource1TemporarySplitUtf16TextLoop69Result::MlSource1TemporarySplitUtf16TextLoop69Done(result, start)
            } else {
                {
                    let result_2 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start, found)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                    if (((result_2.len() as f64)) == count) {
                        self::MlSource1TemporarySplitUtf16TextLoop69Result::MlSource1TemporarySplitUtf16TextLoop69Return(result_2)
                    } else {
                        {
                            let start_2 = (found + (separator.len() as f64));
                            {
                                (result, start) = (result_2, start_2);
                                continue;
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMatchUtf16TextLoop70Result {
    MlSource1TemporaryMatchUtf16TextLoop70Done,
    MlSource1TemporaryMatchUtf16TextLoop70Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_match_utf16_text_loop70(text: &[f64], search: &[f64], mut start: f64, mut at: f64) -> self::MlSource1TemporaryMatchUtf16TextLoop70Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source2_comparison_operand593 = self::ml_array::optional_at(text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source2_comparison_operand594 = self::ml_array::optional_at(search, self::ml_array::number_index(at));
                    !(match (source_ml_source2_comparison_operand593.clone(), source_ml_source2_comparison_operand594.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlSource1TemporaryMatchUtf16TextLoop70Result::MlSource1TemporaryMatchUtf16TextLoop70Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryMatchUtf16TextLoop70Result::MlSource1TemporaryMatchUtf16TextLoop70Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryIndexUtf16TextLoop71Result {
    MlSource1TemporaryIndexUtf16TextLoop71Done,
    MlSource1TemporaryIndexUtf16TextLoop71Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_index_utf16_text_loop71(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryIndexUtf16TextLoop71Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text, search, at) {
                self::MlSource1TemporaryIndexUtf16TextLoop71Result::MlSource1TemporaryIndexUtf16TextLoop71Return(at)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryIndexUtf16TextLoop71Result::MlSource1TemporaryIndexUtf16TextLoop71Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLastIndexUtf16TextLoop72Result {
    MlSource1TemporaryLastIndexUtf16TextLoop72Done,
    MlSource1TemporaryLastIndexUtf16TextLoop72Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_last_index_utf16_text_loop72(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryLastIndexUtf16TextLoop72Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text, search, at) {
                self::MlSource1TemporaryLastIndexUtf16TextLoop72Result::MlSource1TemporaryLastIndexUtf16TextLoop72Return(at)
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLastIndexUtf16TextLoop72Result::MlSource1TemporaryLastIndexUtf16TextLoop72Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareUtf16TextLoop73Result {
    MlSource1TemporaryCompareUtf16TextLoop73Done,
    MlSource1TemporaryCompareUtf16TextLoop73Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compare_utf16_text_loop73(left: &[f64], right: &[f64], mut count: f64, mut at: f64) -> self::MlSource1TemporaryCompareUtf16TextLoop73Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlSource1TemporaryCompareUtf16TextLoop73Result::MlSource1TemporaryCompareUtf16TextLoop73Return(if (a < b) {
                            (-1f64)
                        } else {
                            1f64
                        })
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompareUtf16TextLoop73Result::MlSource1TemporaryCompareUtf16TextLoop73Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_join_utf16_text_loop75(separator: &[f64], mut result: Vec<f64>, mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((separator.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(separator, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_join_utf16_text_loop76(mut result: Vec<f64>, value: &[f64], mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((value.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(value, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_join_utf16_text_loop74(values: &[Vec<f64>], separator: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let result_3 = if (at != (0f64)) {
                    {
                        let offset = 0f64;
                        {
                            let result_2 = self::source_ml_source1_temporary_join_utf16_text_loop75(separator, result, offset);
                            result_2
                        }
                    }
                } else {
                    result
                };
                {
                    let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| Vec::<f64>::new());
                    {
                        let offset_2 = 0f64;
                        {
                            let result_4 = self::source_ml_source1_temporary_join_utf16_text_loop76(result_3, &(value), offset_2);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_4, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_trim_utf16_text_loop77(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((first < last) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index(first))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_trim_utf16_text_loop78(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((last > first) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let last_2 = (last - 1f64);
                {
                    last = last_2;
                    continue;
                }
            }
        } else {
            last
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_character_units_utf16_text_loop79(values: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = (integer % 65536f64);
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (residue < (0f64)) {
                                (residue + 65536f64)
                            } else {
                                if (residue == (0f64)) {
                                    0f64
                                } else {
                                    residue
                                }
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCodePointUnitsUtf16TextLoop80Result {
    MlSource1TemporaryCodePointUnitsUtf16TextLoop80Done(Vec<f64>),
    MlSource1TemporaryCodePointUnitsUtf16TextLoop80Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_code_point_units_utf16_text_loop80(values: &[f64], mut result: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value > (1114111f64))) {
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Return(vec![at])
                } else {
                    if (value < (65536f64)) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (value == (0f64)) {
                                0f64
                            } else {
                                value
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![(55296f64 + (((value - 65536f64) / 1024f64)).floor()), (56320f64 + ((value - 65536f64) % 1024f64))], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDisplayPointsUtf16TextJoin82 {
    MlSource1TemporaryDisplayPointsUtf16TextJoin82Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_display_points_utf16_text_loop81(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                {
                    let source_ml_source2_s595 = if ((point >= (55296f64)) && (point <= (56319f64))) {
                        {
                            let low = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                            if ((low >= (56320f64)) && (low <= (57343f64))) {
                                {
                                    let point_2 = (((65536f64 + ((point - 55296f64) * 1024f64)) + low) - 56320f64);
                                    {
                                        let at_2 = (at + 1f64);
                                        self::MlSource1TemporaryDisplayPointsUtf16TextJoin82::MlSource1TemporaryDisplayPointsUtf16TextJoin82Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65533f64;
                                    self::MlSource1TemporaryDisplayPointsUtf16TextJoin82::MlSource1TemporaryDisplayPointsUtf16TextJoin82Next(at, point_3)
                                }
                            }
                        }
                    } else {
                        if ((point >= (56320f64)) && (point <= (57343f64))) {
                            {
                                let point_4 = 65533f64;
                                self::MlSource1TemporaryDisplayPointsUtf16TextJoin82::MlSource1TemporaryDisplayPointsUtf16TextJoin82Next(at, point_4)
                            }
                        } else {
                            self::MlSource1TemporaryDisplayPointsUtf16TextJoin82::MlSource1TemporaryDisplayPointsUtf16TextJoin82Next(at, point)
                        }
                    };
                    match source_ml_source2_s595.clone() {
                        self::MlSource1TemporaryDisplayPointsUtf16TextJoin82::MlSource1TemporaryDisplayPointsUtf16TextJoin82Next(at_3, point_5) => {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![point_5], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at_3 + 1f64);
                                    {
                                        (result, at) = (result_2, at_4);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}
