// Generated from complete js/src/translation/big-integer-conversion-rules.js. Do not edit by hand.
use super::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop6Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_java_script_numeric_digit_unit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_convert_java_script_string_big_integer_units_loop6(
    units: &[f64],
    mut end: f64,
    mut radix: f64,
    mut base: self::ml::Big,
    mut digits: Vec<self::ml::Big>,
    mut value: self::ml::Big,
    mut index: f64,
) -> self::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop6Result {
    loop {
        return if index < end {
            {
                let digit = self::read_java_script_numeric_digit_unit(
                    (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64),
                );
                if (digit < 0f64) || (digit >= radix) {
                    self::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop6Result::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop6Return(Vec::<self::ml::Big>::new())
                } else {
                    {
                        let value_2 = (value.mul(&base)).add(
                            &(self::ml_array::optional_at(
                                &digits,
                                self::ml_array::number_index(digit),
                            ))
                            .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                        );
                        {
                            let index_2 = index + 1f64;
                            {
                                (value, index) = (value_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop6Result::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop6Done(value.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryConvertJavaScriptStringBigIntegerUnitsJoin5 {
    MlTemporaryConvertJavaScriptStringBigIntegerUnitsJoin5Next(f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryConvertJavaScriptStringBigIntegerUnitsJoin4 {
    MlTemporaryConvertJavaScriptStringBigIntegerUnitsJoin4Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop1Result {
    MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop1Done,
    MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop1Return(Vec<self::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_convert_java_script_string_big_integer_units_loop1(
    units: &[f64],
    mut index: f64,
) -> self::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop1Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let unit =
                    (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64)
                {
                    self::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop1Result::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop1Return(Vec::<self::ml::Big>::new())
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
            self::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop1Result::MlTemporaryConvertJavaScriptStringBigIntegerUnitsLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_convert_java_script_number_big_integer_loop7(
    mut value: f64,
    mut integer: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::ml::Big {
    loop {
        if value >= 1f64 {
            {
                let integer_3 = if (value % 2f64) == 1f64 {
                    { integer.add(&weight) }
                } else {
                    integer.clone()
                };
                {
                    let value_2 = (value / 2f64).floor();
                    {
                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                        {
                            (value, integer, weight) = (value_2, integer_3, weight_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { integer.clone() };
    }
}
