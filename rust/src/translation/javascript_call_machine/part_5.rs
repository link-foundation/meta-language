// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result;
use super::accept_java_script_whitespace_unit;
use super::divide_number_power_bound;
use super::invert_number_power_interval;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::multiply_number_power_interval;
use super::power_java_script_big_integer;
use super::read_java_script_numeric_digit_unit;
use super::read_string_number_digit;
use super::root_number_power_interval;
use super::round_number_power_bound;

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop28(units: &[f64], mut end: f64, mut at: f64, mut exponent: f64, mut exponent_limit: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result {
    loop {
        return if (at < end) {
            {
                let digit = ((self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)) - 48f64);
                if ((digit < (0f64)) || (digit > (9f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(at, exponent)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(f64, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop23(units: &[f64], mut end: f64, mut at: f64, mut fraction: f64, mut digits: f64, mut significant: f64, mut dot: bool) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result {
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
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units: &[f64], mut end: f64, mut integer_digits: Vec<self::ml::Big>, mut radix: f64, mut value: self::ml::Big, mut overflow: bool, mut limit: self::ml::Big, mut base: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop18(units: &[f64], mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop31(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((start < end) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index(start))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let start_2 = (start + 1f64);
                {
                    start = start_2;
                    continue;
                }
            }
        } else {
            start
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop32(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((end > start) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index((end - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let end_2 = (end - 1f64);
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Done(self::ml::Big),
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Return(Vec<self::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop35(units: &[f64], mut end: f64, mut radix: f64, mut base: self::ml::Big, mut digits: Vec<self::ml::Big>, mut value: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_java_script_numeric_digit_unit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Return(Vec::<self::ml::Big>::new())
                } else {
                    {
                        let value_2 = (value.mul(&base)).add(&(self::ml_array::optional_at(&digits, self::ml_array::number_index(digit))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (value, index) = (value_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Done(value.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Done,
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Return(Vec<self::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop30(units: &[f64], mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Return(Vec::<self::ml::Big>::new())
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
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_number_big_integer_loop36(mut value: f64, mut integer: self::ml::Big, mut weight: self::ml::Big) -> self::ml::Big {
    loop {
        return if (value >= (1f64)) {
            {
                let integer_3 = if (((value % 2f64)) == (1f64)) {
                    {
                        let integer_2 = integer.add(&weight);
                        integer_2.clone()
                    }
                } else {
                    integer.clone()
                };
                {
                    let value_2 = ((value / 2f64)).floor();
                    {
                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                        {
                            (value, integer, weight) = (value_2, integer_3.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            integer.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop38(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big, mut quotient: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result {
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
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Done(remaining.clone(), quotient.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop37(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result {
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
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Done(shifted.clone(), weight.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_big_integer_loop39(mut result: self::ml::Big, mut weight: self::ml::Big, mut remaining: self::ml::Big) -> self::ml::Big {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryNormalizeNumberPowerBoundLoop41Result {
    MlSource1TemporaryNormalizeNumberPowerBoundLoop41Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_normalize_number_power_bound_loop41(mut value: self::ml::Big, mut exponent: self::ml::Big, mut scale: self::ml::Big) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Result {
    loop {
        return if (value < scale) {
            {
                let value_2 = value.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent.sub(&self::ml::Big::from_i128(1));
                    {
                        (value, exponent) = (value_2.clone(), exponent_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Done(value.clone(), exponent.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryNormalizeNumberPowerBoundLoop40Result {
    MlSource1TemporaryNormalizeNumberPowerBoundLoop40Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_normalize_number_power_bound_loop40(mut value: self::ml::Big, mut exponent: self::ml::Big, mut scale: self::ml::Big, mut upward: bool) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Result {
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
            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Done(value.clone(), exponent.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_root_number_power_integer_loop42(mut root: self::ml::Big, mut remaining: self::ml::Big) -> self::ml::Big {
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_root_number_power_integer_loop43(mut value: self::ml::Big, mut root: self::ml::Big) -> self::ml::Big {
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
pub enum MlSource1TemporaryRootNumberPowerBoundJoin44 {
    MlSource1TemporaryRootNumberPowerBoundJoin44Next(self::ml::Big, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result {
    MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Done(f64, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop46(mut value: f64, mut exponent: self::ml::Big) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result {
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
            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Done(value, exponent.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result {
    MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Done(f64, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop45(mut value: f64, mut exponent: self::ml::Big) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result {
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
            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Done(value, exponent.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_number_loop49(mut scale: self::ml::Big, mut interval: Vec<self::ml::Big>, mut shift: self::ml::Big) -> Vec<self::ml::Big> {
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_number_loop50(mut scale: self::ml::Big, mut interval: Vec<self::ml::Big>, mut result: Vec<self::ml::Big>, mut remaining: self::ml::Big) -> Vec<self::ml::Big> {
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
pub enum MlSource1TemporaryPowerJavaScriptNumberLoop48Result {
    MlSource1TemporaryPowerJavaScriptNumberLoop48Done(Vec<self::ml::Big>, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_number_loop48(mut scale: self::ml::Big, mut interval: Vec<self::ml::Big>, mut shift: self::ml::Big) -> self::MlSource1TemporaryPowerJavaScriptNumberLoop48Result {
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
            self::MlSource1TemporaryPowerJavaScriptNumberLoop48Result::MlSource1TemporaryPowerJavaScriptNumberLoop48Done(interval, shift.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_number_loop47(mut exponent: f64, mut negative: bool, mut input: Vec<self::ml::Big>, mut power: Vec<self::ml::Big>, mut precision: self::ml::Big) -> f64 {
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
                                    let source_ml_source2_s38 = self::source_ml_source1_temporary_power_java_script_number_loop48(scale.clone(), interval_3, shift.clone());
                                    match source_ml_source2_s38.clone() {
                                        self::MlSource1TemporaryPowerJavaScriptNumberLoop48Result::MlSource1TemporaryPowerJavaScriptNumberLoop48Done(interval_4, shift_2) => {
                                            {
                                                let interval_5 = self::source_ml_source1_temporary_power_java_script_number_loop49(scale.clone(), interval_4, shift_2.clone());
                                                {
                                                    let result = vec![scale.clone(), self::ml::Big::from_i128(0), scale.clone(), self::ml::Big::from_i128(0)];
                                                    {
                                                        let remaining = (self::ml_array::optional_at(&power, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                        {
                                                            let result_2 = self::source_ml_source1_temporary_power_java_script_number_loop50(scale.clone(), interval_5, result, remaining.clone());
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSemanticValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    Integer(self::ml::Big),
    Text(Vec<f64>),
    Symbol(self::ml::Big),
    Object(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallPrimitiveResult {
    PrimitiveValue(Box<self::JavaScriptSemanticValue>),
    PrimitiveInvalid(self::ml_string::Text),
    PrimitiveResourceLimit(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallInstruction {
    Constant(f64, Box<self::JavaScriptSemanticValue>),
    Copy(f64, f64),
    Argument(f64, f64),
    Jump(f64),
    Branch(f64, f64, f64),
    Call(f64, f64, f64, Vec<f64>),
    Return(f64),
    Throw(f64),
    PushHandler(f64, f64),
    PopHandler,
    NumberBinary(f64, f64, f64, self::ml_string::Text),
    SameValue(f64, f64, f64),
    StrictEqual(f64, f64, f64),
    RequireWrite(f64),
    Primitive(f64, self::ml_string::Text, Vec<f64>),
    OrdinaryGet(f64, f64, f64, f64),
    OrdinarySet(f64, f64, f64, f64, f64),
    PropertyKey(f64, f64, self::ml::Big),
    PropertyDefinition(f64, f64, f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallFunction {
    CallFunction(self::ml::Big, self::ml::Big, self::ml_string::Text, Box<self::JavaScriptSemanticValue>, f64, f64, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptCallInstruction>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptCallProgram {
    CallProgram(Vec<self::JavaScriptCallFunction>, Vec<self::ml::Big>, Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryProperty {
    OrdinaryProperty(Box<self::JavaScriptSemanticValue>, (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryObject {
    OrdinaryObject(self::ml::Big, Box<self::JavaScriptSemanticValue>, bool, bool, Vec<self::JavaScriptOrdinaryProperty>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptOrdinaryHeap {
    OrdinaryHeap(Vec<self::JavaScriptOrdinaryObject>, Vec<self::ml::Big>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptDescriptorConversionState {
    DescriptorConversion(Box<self::JavaScriptSemanticValue>, f64, (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptSemanticCompletion {
    NormalCompletion(Box<self::JavaScriptSemanticValue>),
    ThrowCompletion(Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptDescriptorConversionTransition {
    DescriptorContinuation(Box<self::JavaScriptDescriptorConversionState>),
    DescriptorCall(Box<self::JavaScriptDescriptorConversionState>, Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
    DescriptorCompleted((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)),
    DescriptorThrow(Box<self::JavaScriptSemanticValue>),
    DescriptorIntrinsicError(self::ml_string::Text, self::ml_string::Text),
    DescriptorInvalid(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveConversionState {
    PrimitiveConversion(Box<self::JavaScriptSemanticValue>, Box<self::JavaScriptSemanticValue>, self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptPrimitiveConversionCompletion {
    NormalCompletion(Box<self::JavaScriptSemanticValue>),
    ThrowCompletion(Box<self::JavaScriptSemanticValue>),
}
