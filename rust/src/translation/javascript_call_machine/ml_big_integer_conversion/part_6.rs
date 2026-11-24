// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result;
use super::divide_number_power_bound;
use super::invert_number_power_interval;
use super::ml_array;
use super::multiply_number_power_interval;
use super::power_java_script_big_integer;
use super::root_number_power_interval;
use super::round_number_power_bound;

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop38(
    mut remaining: self::ml::Big,
    mut shifted: self::ml::Big,
    mut weight: self::ml::Big,
    mut quotient: self::ml::Big,
) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let shifted_2 = self::ml::divide(
                    &shifted,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    false,
                );
                {
                    let weight_2 = self::ml::divide(
                        &weight,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    if (shifted_2 <= remaining) {
                        {
                            let remaining_2 = remaining.sub(&shifted_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (remaining, shifted, weight, quotient) = (
                                        remaining_2.clone(),
                                        shifted_2.clone(),
                                        weight_2.clone(),
                                        quotient_2.clone(),
                                    );
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
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop37(
    mut remaining: self::ml::Big,
    mut shifted: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result {
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
pub fn source_ml_source1_temporary_power_java_script_big_integer_loop39(
    mut result: self::ml::Big,
    mut weight: self::ml::Big,
    mut remaining: self::ml::Big,
) -> self::ml::Big {
    loop {
        return if (remaining > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(
                    &remaining,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    true,
                )) == (self::ml::Big::from_i128(1)))
                {
                    {
                        let result_2 = result.mul(&weight);
                        result_2.clone()
                    }
                } else {
                    result.clone()
                };
                {
                    let remaining_2 = self::ml::divide(
                        &remaining,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let weight_2 = weight.mul(&weight);
                            {
                                (result, weight, remaining) =
                                    (result_3.clone(), weight_2.clone(), remaining_2.clone());
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
pub fn source_ml_source1_temporary_normalize_number_power_bound_loop41(
    mut value: self::ml::Big,
    mut exponent: self::ml::Big,
    mut scale: self::ml::Big,
) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Result {
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
pub fn source_ml_source1_temporary_normalize_number_power_bound_loop40(
    mut value: self::ml::Big,
    mut exponent: self::ml::Big,
    mut scale: self::ml::Big,
    mut upward: bool,
) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Result {
    loop {
        return if (value >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let value_2 = self::ml::divide(
                    &value.add(&if upward {
                        self::ml::Big::from_i128(1)
                    } else {
                        self::ml::Big::from_i128(0)
                    }),
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    false,
                );
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
pub fn source_ml_source1_temporary_root_number_power_integer_loop42(
    mut root: self::ml::Big,
    mut remaining: self::ml::Big,
) -> self::ml::Big {
    loop {
        return if (remaining > (self::ml::Big::from_i128(1))) {
            {
                let remaining_2 = self::ml::divide(
                    &remaining,
                    &self::ml::Big::from_i128(4),
                    "trunc",
                    Some("Division by zero"),
                    false,
                );
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
pub fn source_ml_source1_temporary_root_number_power_integer_loop43(
    mut value: self::ml::Big,
    mut root: self::ml::Big,
) -> self::ml::Big {
    loop {
        return {
            let next = self::ml::divide(
                &root.add(&self::divide_number_power_bound(
                    value.clone(),
                    root.clone(),
                    false,
                )),
                &self::ml::Big::from_i128(2),
                "trunc",
                Some("Division by zero"),
                false,
            );
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
pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop46(
    mut value: f64,
    mut exponent: self::ml::Big,
) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result {
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
pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop45(
    mut value: f64,
    mut exponent: self::ml::Big,
) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result {
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
pub fn source_ml_source1_temporary_power_java_script_number_loop49(
    mut scale: self::ml::Big,
    mut interval: Vec<self::ml::Big>,
    mut shift: self::ml::Big,
) -> Vec<self::ml::Big> {
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
pub fn source_ml_source1_temporary_power_java_script_number_loop50(
    mut scale: self::ml::Big,
    mut interval: Vec<self::ml::Big>,
    mut result: Vec<self::ml::Big>,
    mut remaining: self::ml::Big,
) -> Vec<self::ml::Big> {
    loop {
        return if (remaining > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(
                    &remaining,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    true,
                )) == (self::ml::Big::from_i128(1)))
                {
                    {
                        let result_2 = self::multiply_number_power_interval(
                            result,
                            interval.clone(),
                            scale.clone(),
                        );
                        result_2
                    }
                } else {
                    result
                };
                {
                    let remaining_2 = self::ml::divide(
                        &remaining,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let interval_2 = self::multiply_number_power_interval(
                                interval.clone(),
                                interval,
                                scale.clone(),
                            );
                            {
                                (interval, result, remaining) =
                                    (interval_2, result_3, remaining_2.clone());
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
pub fn source_ml_source1_temporary_power_java_script_number_loop48(
    mut scale: self::ml::Big,
    mut interval: Vec<self::ml::Big>,
    mut shift: self::ml::Big,
) -> self::MlSource1TemporaryPowerJavaScriptNumberLoop48Result {
    loop {
        return if (shift > (self::ml::Big::from_i128(0))) {
            {
                let interval_2 =
                    self::multiply_number_power_interval(interval.clone(), interval, scale.clone());
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
pub fn source_ml_source1_temporary_power_java_script_number_loop47(
    mut exponent: f64,
    mut negative: bool,
    mut input: Vec<self::ml::Big>,
    mut power: Vec<self::ml::Big>,
    mut precision: self::ml::Big,
) -> f64 {
    loop {
        return {
            let scale = (self::ml_array::optional_at(
                &self::power_java_script_big_integer(
                    self::ml::Big::from_i128(2),
                    precision.clone(),
                ),
                self::ml_array::number_index(0f64),
            ))
            .clone()
            .unwrap_or_else(|| self::ml::Big::from_i128(0));
            {
                let value =
                    ((self::ml_array::optional_at(&input, self::ml_array::number_index(0f64)))
                        .clone()
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)))
                    .mul(
                        &(self::ml_array::optional_at(
                            &self::power_java_script_big_integer(
                                self::ml::Big::from_i128(2),
                                precision.sub(&self::ml::Big::from_i128(52)),
                            ),
                            self::ml_array::number_index(0f64),
                        ))
                        .clone()
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                    );
                {
                    let initial_exponent =
                        ((self::ml_array::optional_at(&input, self::ml_array::number_index(1f64)))
                            .clone()
                            .unwrap_or_else(|| self::ml::Big::from_i128(0)))
                        .add(&self::ml::Big::from_i128(52));
                    {
                        let interval = vec![
                            value.clone(),
                            initial_exponent.clone(),
                            value.clone(),
                            initial_exponent.clone(),
                        ];
                        {
                            let interval_3 = if (exponent < (0f64)) {
                                {
                                    let interval_2 =
                                        self::invert_number_power_interval(interval, scale.clone());
                                    interval_2
                                }
                            } else {
                                interval
                            };
                            {
                                let shift = (self::ml_array::optional_at(
                                    &power,
                                    self::ml_array::number_index(1f64),
                                ))
                                .clone()
                                .unwrap_or_else(|| self::ml::Big::from_i128(0));
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
