
#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(f64, f64, f64, f64),
}

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
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done(crate::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units: &[f64], mut end: f64, mut integer_digits: Vec<crate::ml::Big>, mut radix: f64, mut value: crate::ml::Big, mut overflow: bool, mut limit: crate::ml::Big, mut base: crate::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64::NAN)
                } else {
                    if !overflow {
                        {
                            let value_2 = (value.mul(&base)).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index(digit))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)));
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
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Done(crate::ml::Big),
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Return(Vec<crate::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop35(units: &[f64], mut end: f64, mut radix: f64, mut base: crate::ml::Big, mut digits: Vec<crate::ml::Big>, mut value: crate::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_java_script_numeric_digit_unit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Return(Vec::<crate::ml::Big>::new())
                } else {
                    {
                        let value_2 = (value.mul(&base)).add(&(self::ml_array::optional_at(&digits, self::ml_array::number_index(digit))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)));
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
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(f64, f64, crate::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Done,
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Return(Vec<crate::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop30(units: &[f64], mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Return(Vec::<crate::ml::Big>::new())
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
pub fn source_ml_source1_temporary_convert_java_script_number_big_integer_loop36(mut value: f64, mut integer: crate::ml::Big, mut weight: crate::ml::Big) -> crate::ml::Big {
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
                        let weight_2 = weight.mul(&crate::ml::Big::from_i128(2));
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
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Done(crate::ml::Big, crate::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop38(mut remaining: crate::ml::Big, mut shifted: crate::ml::Big, mut weight: crate::ml::Big, mut quotient: crate::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result {
    loop {
        return if (weight > (crate::ml::Big::from_i128(1))) {
            {
                let shifted_2 = crate::ml::divide(&shifted, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = crate::ml::divide(&weight, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
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
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Done(crate::ml::Big, crate::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop37(mut remaining: crate::ml::Big, mut shifted: crate::ml::Big, mut weight: crate::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result {
    loop {
        return if (shifted <= remaining) {
            {
                let shifted_2 = shifted.mul(&crate::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&crate::ml::Big::from_i128(2));
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
pub fn source_ml_source1_temporary_power_java_script_big_integer_loop39(mut result: crate::ml::Big, mut weight: crate::ml::Big, mut remaining: crate::ml::Big) -> crate::ml::Big {
    loop {
        return if (remaining > (crate::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((crate::ml::divide(&remaining, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (crate::ml::Big::from_i128(1))) {
                    {
                        let result_2 = result.mul(&weight);
                        result_2.clone()
                    }
                } else {
                    result.clone()
                };
                {
                    let remaining_2 = crate::ml::divide(&remaining, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remaining_2 > (crate::ml::Big::from_i128(0))) {
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
    MlSource1TemporaryNormalizeNumberPowerBoundLoop41Done(crate::ml::Big, crate::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_normalize_number_power_bound_loop41(mut value: crate::ml::Big, mut exponent: crate::ml::Big, mut scale: crate::ml::Big) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Result {
    loop {
        return if (value < scale) {
            {
                let value_2 = value.mul(&crate::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent.sub(&crate::ml::Big::from_i128(1));
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
    MlSource1TemporaryNormalizeNumberPowerBoundLoop40Done(crate::ml::Big, crate::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_normalize_number_power_bound_loop40(mut value: crate::ml::Big, mut exponent: crate::ml::Big, mut scale: crate::ml::Big, mut upward: bool) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Result {
    loop {
        return if (value >= (scale.mul(&crate::ml::Big::from_i128(2)))) {
            {
                let value_2 = crate::ml::divide(&value.add(&if upward {
                    crate::ml::Big::from_i128(1)
                } else {
                    crate::ml::Big::from_i128(0)
                }), &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let exponent_2 = exponent.add(&crate::ml::Big::from_i128(1));
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
pub fn source_ml_source1_temporary_root_number_power_integer_loop42(mut root: crate::ml::Big, mut remaining: crate::ml::Big) -> crate::ml::Big {
    loop {
        return if (remaining > (crate::ml::Big::from_i128(1))) {
            {
                let remaining_2 = crate::ml::divide(&remaining, &crate::ml::Big::from_i128(4), "trunc", Some("Division by zero"), false);
                {
                    let root_2 = root.mul(&crate::ml::Big::from_i128(2));
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
pub fn source_ml_source1_temporary_root_number_power_integer_loop43(mut value: crate::ml::Big, mut root: crate::ml::Big) -> crate::ml::Big {
    loop {
        return {
            let next = crate::ml::divide(&root.add(&self::divide_number_power_bound(value.clone(), root.clone(), false)), &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
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
    MlSource1TemporaryRootNumberPowerBoundJoin44Next(crate::ml::Big, crate::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result {
    MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Done(f64, crate::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop46(mut value: f64, mut exponent: crate::ml::Big) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result {
    loop {
        return if (value < (1f64)) {
            {
                let value_2 = (value * 2f64);
                {
                    let exponent_2 = exponent.sub(&crate::ml::Big::from_i128(1));
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
    MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Done(f64, crate::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop45(mut value: f64, mut exponent: crate::ml::Big) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result {
    loop {
        return if (value >= (2f64)) {
            {
                let value_2 = (value / 2f64);
                {
                    let exponent_2 = exponent.add(&crate::ml::Big::from_i128(1));
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
pub fn source_ml_source1_temporary_power_java_script_number_loop49(mut scale: crate::ml::Big, mut interval: Vec<crate::ml::Big>, mut shift: crate::ml::Big) -> Vec<crate::ml::Big> {
    loop {
        return if (shift < (crate::ml::Big::from_i128(0))) {
            {
                let interval_2 = self::root_number_power_interval(interval, scale.clone());
                {
                    let shift_2 = shift.add(&crate::ml::Big::from_i128(1));
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
pub fn source_ml_source1_temporary_power_java_script_number_loop50(mut scale: crate::ml::Big, mut interval: Vec<crate::ml::Big>, mut result: Vec<crate::ml::Big>, mut remaining: crate::ml::Big) -> Vec<crate::ml::Big> {
    loop {
        return if (remaining > (crate::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((crate::ml::divide(&remaining, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (crate::ml::Big::from_i128(1))) {
                    {
                        let result_2 = self::multiply_number_power_interval(result, interval.clone(), scale.clone());
                        result_2
                    }
                } else {
                    result
                };
                {
                    let remaining_2 = crate::ml::divide(&remaining, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remaining_2 > (crate::ml::Big::from_i128(0))) {
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
    MlSource1TemporaryPowerJavaScriptNumberLoop48Done(Vec<crate::ml::Big>, crate::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_number_loop48(mut scale: crate::ml::Big, mut interval: Vec<crate::ml::Big>, mut shift: crate::ml::Big) -> self::MlSource1TemporaryPowerJavaScriptNumberLoop48Result {
    loop {
        return if (shift > (crate::ml::Big::from_i128(0))) {
            {
                let interval_2 = self::multiply_number_power_interval(interval.clone(), interval, scale.clone());
                {
                    let shift_2 = shift.sub(&crate::ml::Big::from_i128(1));
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
pub fn source_ml_source1_temporary_power_java_script_number_loop47(mut exponent: f64, mut negative: bool, mut input: Vec<crate::ml::Big>, mut power: Vec<crate::ml::Big>, mut precision: crate::ml::Big) -> f64 {
    loop {
        return {
            let scale = (self::ml_array::optional_at(&self::power_java_script_big_integer(crate::ml::Big::from_i128(2), precision.clone()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0));
            {
                let value = ((self::ml_array::optional_at(&input, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))).mul(&(self::ml_array::optional_at(&self::power_java_script_big_integer(crate::ml::Big::from_i128(2), precision.sub(&crate::ml::Big::from_i128(52))), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)));
                {
                    let initial_exponent = ((self::ml_array::optional_at(&input, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))).add(&crate::ml::Big::from_i128(52));
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
                                let shift = (self::ml_array::optional_at(&power, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0));
                                {
                                    let source_ml_source2_s38 = self::source_ml_source1_temporary_power_java_script_number_loop48(scale.clone(), interval_3, shift.clone());
                                    match source_ml_source2_s38.clone() {
                                        self::MlSource1TemporaryPowerJavaScriptNumberLoop48Result::MlSource1TemporaryPowerJavaScriptNumberLoop48Done(interval_4, shift_2) => {
                                            {
                                                let interval_5 = self::source_ml_source1_temporary_power_java_script_number_loop49(scale.clone(), interval_4, shift_2.clone());
                                                {
                                                    let result = vec![scale.clone(), crate::ml::Big::from_i128(0), scale.clone(), crate::ml::Big::from_i128(0)];
                                                    {
                                                        let remaining = (self::ml_array::optional_at(&power, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0));
                                                        {
                                                            let result_2 = self::source_ml_source1_temporary_power_java_script_number_loop50(scale.clone(), interval_5, result, remaining.clone());
                                                            {
                                                                let lower = self::round_number_power_bound((self::ml_array::optional_at(&result_2, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&result_2, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), precision.clone());
                                                                {
                                                                    let upper = self::round_number_power_bound((self::ml_array::optional_at(&result_2, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&result_2, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), precision.clone());
                                                                    if (lower == upper) {
                                                                        if negative {
                                                                            (-lower)
                                                                        } else {
                                                                            lower
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let precision_2 = precision.mul(&crate::ml::Big::from_i128(2));
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
