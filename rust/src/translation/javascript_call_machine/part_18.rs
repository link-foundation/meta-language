// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result;
use super::accept_java_script_whitespace_unit;
use super::match_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::read_string_number_digit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop39(units: &[f64], mut end: f64, mut at: f64, mut exponent: f64, mut exponent_limit: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result {
    loop {
        return if (at < end) {
            {
                let digit = ((self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)) - 48f64);
                if ((digit < (0f64)) || (digit > (9f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Done(at, exponent)
                } else {
                    {
                        let exponent_3 = if (exponent < exponent_limit) {
                            {
                                let exponent_2 = self::ml_math::min(exponent_limit, ((exponent * 10f64) + digit));
                                exponent_2
                            }
                        } else {
                            exponent
                        };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (at, exponent) = (at_2, exponent_3);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Done(f64, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop34(units: &[f64], mut end: f64, mut at: f64, mut fraction: f64, mut digits: f64, mut significant: f64, mut dot: bool) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result {
    loop {
        return if (at < end) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((unit >= (48f64)) && (unit <= (57f64))) {
                    {
                        let digits_2 = (digits + 1f64);
                        {
                            let fraction_3 = if dot {
                                {
                                    let fraction_2 = (fraction + 1f64);
                                    fraction_2
                                }
                            } else {
                                fraction
                            };
                            {
                                let significant_3 = if ((unit != (48f64)) || (significant > (0f64))) {
                                    {
                                        let significant_2 = (significant + 1f64);
                                        significant_2
                                    }
                                } else {
                                    significant
                                };
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (at, fraction, digits, significant) = (at_2, fraction_3, digits_2, significant_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if ((unit == (46f64)) && !dot) {
                        {
                            let dot_2 = true;
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (at, dot) = (at_3, dot_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop40(units: &[f64], mut end: f64, mut integer_digits: Vec<self::ml::Big>, mut radix: f64, mut value: self::ml::Big, mut overflow: bool, mut limit: self::ml::Big, mut base: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Return(f64::NAN)
                } else {
                    if !overflow {
                        {
                            let value_2 = (value.mul(&base)).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index(digit))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                            {
                                let overflow_2 = (value_2 > limit);
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (value, overflow, index) = (value_2.clone(), overflow_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let index_3 = (index + 1f64);
                            {
                                index = index_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop29(mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64::NAN)
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop49(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big, mut quotient: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let shifted_2 = self::ml::divide(&shifted, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (shifted_2 <= remaining) {
                        {
                            let remaining_2 = remaining.sub(&shifted_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (remaining, shifted, weight, quotient) = (remaining_2.clone(), shifted_2.clone(), weight_2.clone(), quotient_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Done(remaining.clone(), quotient.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop48(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Result {
    loop {
        return if (shifted <= remaining) {
            {
                let shifted_2 = shifted.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop48Done(shifted.clone(), weight.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_big_integer_loop50(mut result: self::ml::Big, mut weight: self::ml::Big, mut remaining: self::ml::Big) -> self::ml::Big {
    loop {
        return if (remaining > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = result.mul(&weight);
                        result_2.clone()
                    }
                } else {
                    result.clone()
                };
                {
                    let remaining_2 = self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let weight_2 = weight.mul(&weight);
                            {
                                (result, weight, remaining) = (result_3.clone(), weight_2.clone(), remaining_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (result, remaining) = (result_3.clone(), remaining_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result {
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Done(f64),
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_integer_width_loop62(mut rest: self::ml::Big, mut width: f64) -> self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result {
    loop {
        return if (rest > (self::ml::Big::from_i128(0))) {
            if (width >= (4096f64)) {
                self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Return((-1f64))
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
            self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop62Done(width)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next(self::ml::Big),
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Next,
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin63Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_slice_utf16_text_loop65(text: &[f64], mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMatchUtf16TextLoop69Result {
    MlSource1TemporaryMatchUtf16TextLoop69Done,
    MlSource1TemporaryMatchUtf16TextLoop69Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_match_utf16_text_loop69(text: &[f64], search: &[f64], mut start: f64, mut at: f64) -> self::MlSource1TemporaryMatchUtf16TextLoop69Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source2_comparison_operand2146 = self::ml_array::optional_at(text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source2_comparison_operand2147 = self::ml_array::optional_at(search, self::ml_array::number_index(at));
                    !(match (source_ml_source2_comparison_operand2146.clone(), source_ml_source2_comparison_operand2147.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlSource1TemporaryMatchUtf16TextLoop69Result::MlSource1TemporaryMatchUtf16TextLoop69Return(false)
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
            self::MlSource1TemporaryMatchUtf16TextLoop69Result::MlSource1TemporaryMatchUtf16TextLoop69Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryIndexUtf16TextLoop70Result {
    MlSource1TemporaryIndexUtf16TextLoop70Done,
    MlSource1TemporaryIndexUtf16TextLoop70Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_index_utf16_text_loop70(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryIndexUtf16TextLoop70Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text, search, at) {
                self::MlSource1TemporaryIndexUtf16TextLoop70Result::MlSource1TemporaryIndexUtf16TextLoop70Return(at)
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
            self::MlSource1TemporaryIndexUtf16TextLoop70Result::MlSource1TemporaryIndexUtf16TextLoop70Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLastIndexUtf16TextLoop71Result {
    MlSource1TemporaryLastIndexUtf16TextLoop71Done,
    MlSource1TemporaryLastIndexUtf16TextLoop71Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_last_index_utf16_text_loop71(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryLastIndexUtf16TextLoop71Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text, search, at) {
                self::MlSource1TemporaryLastIndexUtf16TextLoop71Result::MlSource1TemporaryLastIndexUtf16TextLoop71Return(at)
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
            self::MlSource1TemporaryLastIndexUtf16TextLoop71Result::MlSource1TemporaryLastIndexUtf16TextLoop71Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareUtf16TextLoop72Result {
    MlSource1TemporaryCompareUtf16TextLoop72Done,
    MlSource1TemporaryCompareUtf16TextLoop72Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compare_utf16_text_loop72(left: &[f64], right: &[f64], mut count: f64, mut at: f64) -> self::MlSource1TemporaryCompareUtf16TextLoop72Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlSource1TemporaryCompareUtf16TextLoop72Result::MlSource1TemporaryCompareUtf16TextLoop72Return(if (a < b) {
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
            self::MlSource1TemporaryCompareUtf16TextLoop72Result::MlSource1TemporaryCompareUtf16TextLoop72Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_trim_utf16_text_loop76(text: &[f64], mut first: f64, mut last: f64) -> f64 {
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
pub fn source_ml_source1_temporary_trim_utf16_text_loop77(text: &[f64], mut first: f64, mut last: f64) -> f64 {
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
pub fn source_ml_source1_temporary_character_units_utf16_text_loop78(values: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
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
pub enum MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result {
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Done(Vec<f64>),
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop90(mut numbers: Vec<f64>, mut source_ml_source1_temporary_values89: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index89: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result {
    loop {
        return if (source_ml_source1_temporary_index89 < (self::ml::Big::from_u128(source_ml_source1_temporary_values89.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values89, source_ml_source1_temporary_index89.to_index());
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let numbers_2 = { let (ml_append_right, ml_append_left) = (vec![value_value], numbers); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index89_2 = source_ml_source1_temporary_index89.add(&self::ml::Big::from_i128(1));
                                {
                                    (numbers, source_ml_source1_temporary_index89) = (numbers_2, source_ml_source1_temporary_index89_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop90Done(numbers)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result {
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Done,
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_primitive_loop92(mut source_ml_source1_temporary_values91: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index91: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result {
    loop {
        return if (source_ml_source1_temporary_index91 < (self::ml::Big::from_u128(source_ml_source1_temporary_values91.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values91, source_ml_source1_temporary_index91.to_index());
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index91_2 = source_ml_source1_temporary_index91.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index91 = source_ml_source1_temporary_index91_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop92Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_text_conversion_loop94(text: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (text.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand2155 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand2156 = index;
                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand2162 = {
                                        let source_ml_source2_string_range_operand2157 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand2158 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand2159 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position2160 = source_ml_source2_string_range_operand2158;
                                                    {
                                                        let source_ml_source2_string_range_position2161 = source_ml_source2_string_range_operand2159;
                                                        source_ml_source2_string_range_operand2157.slice(source_ml_source2_string_range_position2160, source_ml_source2_string_range_position2161)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand2163 = 0f64;
                                        source_ml_source2_string_unit_operand2162.code_point_at(source_ml_source2_string_unit_operand2163)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand2155.clone(), source_ml_source2_string_unit_operand2156)
                    }
                }], units); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (units, index) = (units_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin93 {
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin93Next(self::ml_string::Text),
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin93Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Result {
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Done,
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_code_function_loop95(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2164, _, _) => source_ml_source2_data_property_value2164.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2166, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2166.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2168, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value2168.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop95Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96 {
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Next,
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin96Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Result {
    MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Done,
    MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_ordinary_object_loop98(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2170, _, _, _, _) => {
                                source_ml_source2_data_property_value2170.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2172, _, _, _, _) => {
                                    source_ml_source2_data_property_value2172.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop98Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Result {
    MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Done,
    MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_ordinary_symbol_loop99(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source2_comparison_operand2173 = symbol.clone();
                        {
                            let source_ml_source2_comparison_operand2174 = None::<()>;
                            match (source_ml_source2_comparison_operand2173.clone(), source_ml_source2_comparison_operand2174.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop99Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop100Result {
    MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop100Done,
    MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop100Return(f64),
}
