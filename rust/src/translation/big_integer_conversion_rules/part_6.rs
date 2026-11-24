// Generated from complete js/src/translation/big-integer-conversion-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPowerJavaScriptNumberLoop48Result;
use super::invert_number_power_interval;
use super::ml;
use super::ml_array;
use super::power_java_script_big_integer;
use super::round_number_power_bound;
use super::source_ml_source1_temporary_power_java_script_number_loop48;
use super::source_ml_source1_temporary_power_java_script_number_loop49;
use super::source_ml_source1_temporary_power_java_script_number_loop50;

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
            .unwrap_or_else(|| self::ml::Big::from_i128(0));
            {
                let value =
                    ((self::ml_array::optional_at(&input, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)))
                    .mul(
                        &(self::ml_array::optional_at(
                            &self::power_java_script_big_integer(
                                self::ml::Big::from_i128(2),
                                precision.sub(&self::ml::Big::from_i128(52)),
                            ),
                            self::ml_array::number_index(0f64),
                        ))
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                    );
                {
                    let initial_exponent =
                        ((self::ml_array::optional_at(&input, self::ml_array::number_index(1f64)))
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
                            let interval_3 = if exponent < 0f64 {
                                { self::invert_number_power_interval(interval, scale.clone()) }
                            } else {
                                interval
                            };
                            {
                                let shift = (self::ml_array::optional_at(
                                    &power,
                                    self::ml_array::number_index(1f64),
                                ))
                                .unwrap_or_else(|| self::ml::Big::from_i128(0));
                                {
                                    let source_ml_source2_s38 = self::source_ml_source1_temporary_power_java_script_number_loop48(scale.clone(), interval_3, shift);
                                    match source_ml_source2_s38 {
                                        self::MlSource1TemporaryPowerJavaScriptNumberLoop48Result::MlSource1TemporaryPowerJavaScriptNumberLoop48Done(interval_4, shift_2) => {
                                            {
                                                let interval_5 = self::source_ml_source1_temporary_power_java_script_number_loop49(scale.clone(), interval_4, shift_2.clone());
                                                {
                                                    let result = vec![scale.clone(), self::ml::Big::from_i128(0), scale.clone(), self::ml::Big::from_i128(0)];
                                                    {
                                                        let remaining = (self::ml_array::optional_at(&power, self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                        {
                                                            let result_2 = self::source_ml_source1_temporary_power_java_script_number_loop50(scale.clone(), interval_5, result, remaining);
                                                            {
                                                                let lower = self::round_number_power_bound((self::ml_array::optional_at(&result_2, self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&result_2, self::ml_array::number_index(1f64))).unwrap_or_else(|| self::ml::Big::from_i128(0)), precision.clone());
                                                                {
                                                                    let upper = self::round_number_power_bound((self::ml_array::optional_at(&result_2, self::ml_array::number_index(2f64))).unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&result_2, self::ml_array::number_index(3f64))).unwrap_or_else(|| self::ml::Big::from_i128(0)), precision.clone());
                                                                    if lower == upper {
                                                                        if negative {
                                                                            -lower
                                                                        } else {
                                                                            lower
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let precision_2 = precision.mul(&self::ml::Big::from_i128(2));
                                                                            {
                                                                                precision = precision_2;
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
