// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::accept_java_script_whitespace_unit;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_java_script_numeric_digit_unit;
use super::read_string_number_digit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop18(mut value: self::ml::Big, mut result: f64, mut scale: f64) -> f64 {
    loop {
        return if (value > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = (result + scale);
                        result_2
                    }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let scale_2 = (scale * 2f64);
                        {
                            (value, result, scale) = (value_2.clone(), result_3, scale_2);
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop20(mut exponent: f64, mut result: f64) -> f64 {
    loop {
        return if (exponent < (0f64)) {
            {
                let result_2 = (result / 2f64);
                {
                    let exponent_2 = (exponent + 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
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
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop19Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop19Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop19(mut exponent: f64, mut result: f64) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop19Result {
    loop {
        return if (exponent > (0f64)) {
            {
                let result_2 = (result * 2f64);
                {
                    let exponent_2 = (exponent - 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop19Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop19Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop24Result {
    MlSource1TemporaryConvertStringNumberRationalLoop24Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop24(mut quotient: self::ml::Big, mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop24Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let scale_2 = self::ml::divide(&scale, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remainder >= scale_2) {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (quotient_2.clone(), remainder_2.clone(), scale_2.clone(), weight_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (scale, weight) = (scale_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop24Result::MlSource1TemporaryConvertStringNumberRationalLoop24Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop23Result {
    MlSource1TemporaryConvertStringNumberRationalLoop23Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop23(mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop23Result {
    loop {
        return if (scale <= remainder) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop23Result::MlSource1TemporaryConvertStringNumberRationalLoop23Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin22 {
    MlSource1TemporaryConvertStringNumberRationalJoin22Next(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop25(mut numerator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < shift) {
            {
                let numerator_2 = numerator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (numerator, index) = (numerator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            numerator.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop26(mut denominator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < ((-shift))) {
            {
                let denominator_2 = denominator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (denominator, index) = (denominator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            denominator.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin21 {
    MlSource1TemporaryConvertStringNumberRationalJoin21Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin21Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop27Result {
    MlSource1TemporaryConvertStringNumberRationalLoop27Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop27Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop27(mut numerator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop27Result {
    loop {
        return if (numerator >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent + 1f64);
                    if (exponent_2 >= (1024f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop28Result {
    MlSource1TemporaryConvertStringNumberRationalLoop28Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop28Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop28(mut denominator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop28Result {
    loop {
        return if (scale < denominator) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent - 1f64);
                    if (exponent_2 < ((-1075f64))) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop28Result::MlSource1TemporaryConvertStringNumberRationalLoop28Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop28Result::MlSource1TemporaryConvertStringNumberRationalLoop28Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop29Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop29Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop29Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_string_number_infinity_loop29(units: &[f64], mut start: f64, expected: &[f64], mut index: f64) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop29Result {
    loop {
        return if (index < ((expected.len() as f64))) {
            if (((self::ml_array::optional_at(units, self::ml_array::number_index((start + index)))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(expected, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-2f64)))) {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop29Result::MlSource1TemporaryAcceptStringNumberInfinityLoop29Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop29Result::MlSource1TemporaryAcceptStringNumberInfinityLoop29Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop31(units: &[f64], mut start: f64, mut end: f64) -> f64 {
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
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop32(units: &[f64], mut start: f64, mut end: f64) -> f64 {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop38(units: &[f64], mut integer_digits: Vec<self::ml::Big>, mut mantissa_end: f64, mut mantissa: self::ml::Big, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < mantissa_end) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (unit != (46f64)) {
                    {
                        let mantissa_2 = (mantissa.mul(&self::ml::Big::from_i128(10))).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index((unit - 48f64)))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (mantissa, index) = (mantissa_2.clone(), index_2);
                                continue;
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
        } else {
            mantissa.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop37(units: &[f64], mut start: f64, mut exponent: f64, mut mantissa_end: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Result {
    loop {
        return if (mantissa_end > start) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index((mantissa_end - 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if (unit == (46f64)) {
                    {
                        let mantissa_end_2 = (mantissa_end - 1f64);
                        {
                            mantissa_end = mantissa_end_2;
                            continue;
                        }
                    }
                } else {
                    if (unit != (48f64)) {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Done(exponent, mantissa_end)
                    } else {
                        {
                            let mantissa_end_3 = (mantissa_end - 1f64);
                            {
                                let exponent_2 = (exponent + 1f64);
                                {
                                    (exponent, mantissa_end) = (exponent_2, mantissa_end_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop40(units: &[f64], mut end: f64, mut at: f64, mut exponent: f64, mut exponent_limit: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result {
    loop {
        return if (at < end) {
            {
                let digit = ((self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)) - 48f64);
                if ((digit < (0f64)) || (digit > (9f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Done(at, exponent)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin39 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin39Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Done(f64, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop35(units: &[f64], mut end: f64, mut at: f64, mut fraction: f64, mut digits: f64, mut significant: f64, mut dot: bool) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Result {
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
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop41(units: &[f64], mut end: f64, mut integer_digits: Vec<self::ml::Big>, mut radix: f64, mut value: self::ml::Big, mut overflow: bool, mut limit: self::ml::Big, mut base: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop30(units: &[f64], mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop43(units: &[f64], mut start: f64, mut end: f64) -> f64 {
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
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop44(units: &[f64], mut start: f64, mut end: f64) -> f64 {
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
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Done(self::ml::Big),
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Return(Vec<self::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop47(units: &[f64], mut end: f64, mut radix: f64, mut base: self::ml::Big, mut digits: Vec<self::ml::Big>, mut value: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_java_script_numeric_digit_unit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Return(Vec::<self::ml::Big>::new())
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
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Done(value.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46Next(f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Done,
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Return(Vec<self::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop42(units: &[f64], mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Return(Vec::<self::ml::Big>::new())
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
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_number_big_integer_loop48(mut value: f64, mut integer: self::ml::Big, mut weight: self::ml::Big) -> self::ml::Big {
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
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop50Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop50Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop50(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big, mut quotient: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop50Result {
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
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop50Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop50Done(remaining.clone(), quotient.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop49(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result {
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
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Done(shifted.clone(), weight.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_big_integer_loop51(mut result: self::ml::Big, mut weight: self::ml::Big, mut remaining: self::ml::Big) -> self::ml::Big {
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
pub enum MlSource1TemporaryNormalizeNumberPowerBoundLoop53Result {
    MlSource1TemporaryNormalizeNumberPowerBoundLoop53Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_normalize_number_power_bound_loop53(mut value: self::ml::Big, mut exponent: self::ml::Big, mut scale: self::ml::Big) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop53Result {
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
            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop53Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop53Done(value.clone(), exponent.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryNormalizeNumberPowerBoundLoop52Result {
    MlSource1TemporaryNormalizeNumberPowerBoundLoop52Done(self::ml::Big, self::ml::Big),
}
