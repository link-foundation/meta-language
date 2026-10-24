// Generated from complete js/src/translation/string-number-rules.js. Do not edit by hand.
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result;
use super::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result;
use super::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result;
use super::accept_string_number_infinity;
use super::convert_string_number_rational;
use super::ml;
use super::ml_array;
use super::read_string_number_decimal_power;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop14;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop15;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop16;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop19;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop21;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop22;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop24;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop25;
use super::source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop26;
use super::source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop27;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_string_number_units(units: &[f64]) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s11 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop14(units, index);
            match source_ml_source2_s11 {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Done => {
                    {
                        let start = 0f64;
                        {
                            let end = units.len() as f64;
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop15(units, start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop16(units, start_2, end);
                                    if start_2 == end_2 {
                                        0f64
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(units, self::ml_array::number_index(start_2))).unwrap_or_else(|| -1f64);
                                                    {
                                                        let source_ml_source2_s12 = if (sign == 43f64) || (sign == 45f64) {
                                                            {
                                                                let negative_2 = sign == 45f64;
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = start_2 + 1f64;
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s12 {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17Next(start_4, negative_3, signed_3) => {
                                                                if start_4 == end_2 {
                                                                    f64::NAN
                                                                } else if self::accept_string_number_infinity(units, start_4, end_2) {
                                                                        if negative_3 {
                                                                            f64::NEG_INFINITY
                                                                        } else {
                                                                            f64::INFINITY
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let integer_digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                            {
                                                                                let source_ml_source2_s14 = if (!signed_3 && (((self::ml_array::optional_at(units, self::ml_array::number_index(start_4))).unwrap_or_else(|| -1f64)) == 48f64)) && ((end_2 - start_4) >= 2f64) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(units, self::ml_array::number_index(start_4 + 1f64))).unwrap_or_else(|| -1f64);
                                                                                        {
                                                                                            let radix = if (marker == 120f64) || (marker == 88f64) {
                                                                                                16f64
                                                                                            } else if (marker == 98f64) || (marker == 66f64) {
                                                                                                    2f64
                                                                                                } else if (marker == 111f64) || (marker == 79f64) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    };
                                                                                            {
let ml_library_inequality_2 = radix != 0f64 ;
if ml_library_inequality_2 {
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let overflow = false;
                                                                                                        {
                                                                                                            let limit = self::read_string_number_decimal_power(310f64);
                                                                                                            {
                                                                                                                let base = if radix == 16f64 {
                                                                                                                    self::ml::Big::from_i128(16)
                                                                                                                } else if radix == 8f64 {
                                                                                                                        self::ml::Big::from_i128(8)
                                                                                                                    } else {
                                                                                                                        self::ml::Big::from_i128(2)
                                                                                                                    };
                                                                                                                {
                                                                                                                    let start_5 = start_4 + 2f64;
                                                                                                                    if start_5 == end_2 {
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let index_2 = start_5;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s15 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop25(units, end_2, integer_digits.clone(), radix, value, overflow, limit, base, index_2);
                                                                                                                                match source_ml_source2_s15 {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(value_2, overflow_2) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(if overflow_2 {
                                                                                                                                            f64::INFINITY
                                                                                                                                        } else {
                                                                                                                                            self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                        })
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Return(source_ml_source1_temporary_result25) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(source_ml_source1_temporary_result25)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Next(start_4)
                                                                                            }
}
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s14 {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Next(start_6) => {
                                                                                        {
                                                                                            let at = start_6;
                                                                                            {
                                                                                                let fraction = 0f64;
                                                                                                {
                                                                                                    let digits = 0f64;
                                                                                                    {
                                                                                                        let significant = 0f64;
                                                                                                        {
                                                                                                            let dot = false;
                                                                                                            {
                                                                                                                let source_ml_source2_s16 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop19(units, end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s16 {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Done(at_2, fraction_2, digits_2, significant_2) => {
                                                                                                                        if digits_2 == 0f64 {
                                                                                                                            f64::NAN
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let decimal_end = at_2;
                                                                                                                                {
                                                                                                                                    let exponent = 0f64;
                                                                                                                                    {
                                                                                                                                        let exponent_limit = (units.len() as f64) + 10_000f64;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s19 = if (at_2 < end_2) && ((((self::ml_array::optional_at(units, self::ml_array::number_index(at_2))).unwrap_or_else(|| -1f64)) == 101f64) || (((self::ml_array::optional_at(units, self::ml_array::number_index(at_2))).unwrap_or_else(|| -1f64)) == 69f64)) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = at_2 + 1f64;
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(units, self::ml_array::number_index(at_3))).unwrap_or_else(|| -1f64);
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s20 = if (exponent_sign == 43f64) || (exponent_sign == 45f64) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = exponent_sign == 45f64;
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = at_3 + 1f64;
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s20 {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s21 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop24(units, end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s21 {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Done(at_6, exponent_2) => {
                                                                                                                                                                                        if at_6 == first {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Return(f64::NAN)
                                                                                                                                                                                        } else if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = -exponent_2;
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(at_6, exponent_2)
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s19 {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(at_7, exponent_4) => {
                                                                                                                                                    {
let ml_library_inequality_1 = at_7 != end_2 ;
if ml_library_inequality_1 {
                                                                                                                                                        f64::NAN
                                                                                                                                                    } else if significant_2 == 0f64 {
                                                                                                                                                            if negative_3 {
                                                                                                                                                                -0f64
                                                                                                                                                            } else {
                                                                                                                                                                0f64
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            {
                                                                                                                                                                let exponent_5 = exponent_4 - fraction_2;
                                                                                                                                                                {
                                                                                                                                                                    let order = significant_2 + exponent_5;
                                                                                                                                                                    if order > 310f64 {
                                                                                                                                                                        if negative_3 {
                                                                                                                                                                            f64::NEG_INFINITY
                                                                                                                                                                        } else {
                                                                                                                                                                            f64::INFINITY
                                                                                                                                                                        }
                                                                                                                                                                    } else if order < -325f64 {
                                                                                                                                                                            if negative_3 {
                                                                                                                                                                                -0f64
                                                                                                                                                                            } else {
                                                                                                                                                                                0f64
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            {
                                                                                                                                                                                let mantissa_end = decimal_end;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_s22 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop21(units, start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s22 {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let index_3 = start_6;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop22(units, integer_digits, mantissa_end_2, mantissa, index_3);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let numerator = if exponent_6 >= 0f64 {
                                                                                                                                                                                                                mantissa_2.mul(&self::read_string_number_decimal_power(exponent_6))
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                mantissa_2.clone()
                                                                                                                                                                                                            };
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let denominator = if exponent_6 < 0f64 {
                                                                                                                                                                                                                    self::read_string_number_decimal_power(-exponent_6)
                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                    self::ml::Big::from_i128(1)
                                                                                                                                                                                                                };
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let value_3 = self::convert_string_number_rational(numerator, denominator);
                                                                                                                                                                                                                    if negative_3 {
                                                                                                                                                                                                                        -value_3
                                                                                                                                                                                                                    } else {
                                                                                                                                                                                                                        value_3
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Return(source_ml_source1_temporary_result22) => {
                                                                                                                                                    source_ml_source1_temporary_result22
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(source_ml_source1_temporary_result24) => {
                                                                                        source_ml_source1_temporary_result24
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Return(source_ml_source1_temporary_result25_2) => {
                    source_ml_source1_temporary_result25_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_big_integer_number(value: self::ml::Big) -> f64 {
    if value < (self::ml::Big::from_i128(0)) {
        -(self::convert_string_number_rational(value.neg(), self::ml::Big::from_i128(1)))
    } else {
        self::convert_string_number_rational(value.clone(), self::ml::Big::from_i128(1))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_utf8_string_number_units(bytes: &[f64]) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s23 = self::source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop26(bytes, index);
            match source_ml_source2_s23 {
                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s24 = self::source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop27(bytes, units, index_2);
                                match source_ml_source2_s24 {
                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Done(units_2) => {
                                        self::convert_java_script_string_number_units(&(units_2))
                                    }
                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(source_ml_source1_temporary_result27) => {
                                        source_ml_source1_temporary_result27
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Return(source_ml_source1_temporary_result29) => {
                    source_ml_source1_temporary_result29
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_builtin_operation(name: &str) -> String {
    if name == "Number" {
        String::from("convertJavaScriptNumber")
    } else if name == "isFinite" {
            String::from("testCoercedJavaScriptNumberFinite")
        } else if name == "isNaN" {
                String::from("testCoercedJavaScriptNumberNaN")
            } else {
                String::new()
            }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_result(operation: &str) -> String {
    if (operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber") {
        String::from("float")
    } else if (operation == "testCoercedJavaScriptNumberFinite") || (operation == "testCoercedJavaScriptNumberNaN") {
            String::from("bool")
        } else {
            String::new()
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_form(kind: &str) -> String {
    if ((((((kind == "float") || (kind == "bool")) || (kind == "string")) || (kind == "unit")) || (kind == "int")) || (kind == "nat")) || (kind == "option") {
        kind.to_owned()
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_number_conversion(kind: &str, operation: &str) -> bool {
    (kind == "math") && ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_failure(operation: &str, kind: &str) -> String {
    if (operation == "coerceJavaScriptNumber") && ((kind == "int") || (kind == "nat")) {
        String::from("Cannot convert a BigInt value to a number")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_number_conversion_units(kind: &str, operation: &str, argument: &str) -> bool {
    self::accept_linked_java_script_number_conversion(kind, operation) && (argument == "string")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_number_operand(language: &str, kind: &str, element: &str) -> String {
    if language == "JavaScript" {
 if (
 ((kind == "bool") || (kind == "string")) || (kind == "unit") 
) || (
 (kind == "option") && (element == "string") 
) {
                String::from("coerceJavaScriptNumber")
            } else {
                String::new()
            }
} else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_numeric_inference_kind(kind: &str) -> String {
    if ((kind == "bool") || (kind == "string")) || (kind == "unit") {
        String::from("float")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_unary_operation(symbol: &str) -> String {
    if symbol == "+" {
        String::from("coerceJavaScriptNumber")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_mathematical_conversion_form(language: &str, operation: &str) -> String {
    if language == "JavaScript" {
 if (operation == "max") || (operation == "min") {
            String::from("variadic")
        } else if ((((((operation == "abs") || (operation == "floor")) || (operation == "ceil")) || (operation == "trunc")) || (operation == "round")) || (operation == "sign")) || (operation == "sqrt") {
                String::from("unary")
            } else {
                String::new()
            }
} else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_number_scalar_conversion(format: &str, operation: &str, form: &str, value: &str, utf16: bool) -> String {
    if (!(self::accept_linked_java_script_number_conversion("math", operation)) || ((self::read_java_script_number_conversion_form(form)) == "unsupported")) || (form == "option") {
        String::new()
    } else if format == "JavaScript" {
            if operation == "convertJavaScriptNumber" {
                format!("Number({})", value.to_owned())
            } else {
                format!("(+({}))", value.to_owned())
            }
        } else if (format == "Lean") || (format == "Rocq") {
                if (self::read_java_script_number_conversion_failure(operation, form)).is_empty() {if form == "float" {
                        value.to_owned()
                    } else if form == "unit" {
                            if format == "Lean" {
                                format!("(let _ := {}; ((0 : Float) / 0))", value.to_owned())
                            } else {
                                format!("(let _ := {} in PrimFloat.nan)", value.to_owned())
                            }
                        } else if form == "bool" {
                                if format == "Lean" {
                                    format!("(if {} then (1 : Float) else (0 : Float))", value.to_owned())
                                } else {
                                    format!("(if {} then 1%float else 0%float)", value.to_owned())
                                }
                            } else if (form == "int") || (form == "nat") {
                                    format!("(MLStringNumber.convertInteger {})", if form == "nat" {
                                        if format == "Lean" {
                                            format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat ")), value.to_owned())), (String::from(")")))
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(Z.of_N ")), value.to_owned())), (String::from(")")))
                                        }
                                    } else {
                                        value.to_owned()
                                    })
                                } else if format == "Lean" {
                                        format!("(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] {}))", value.to_owned())
                                    } else {
                                        format!("(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) {}))", value.to_owned())
                                    }} else {
                    String::new()
                }
            } else if format == "Rust" {
 if form == "float" {
                        value.to_owned()
                    } else if form == "bool" {
                            format!("(if {} {{ 1.0f64 }} else {{ 0.0f64 }})", value.to_owned())
                        } else if form == "unit" {
                                format!("{{ let _ = {}; f64::NAN }}", value.to_owned())
                            } else if (form == "int") || (form == "nat") {
                                    if operation == "coerceJavaScriptNumber" {
                                        format!("{{ let _ = {}; panic!(\"Cannot convert a BigInt value to a number\") }}", value.to_owned())
                                    } else {
                                        format!("crate::ml_string_number::convert_java_script_big_integer_number({})", value.to_owned())
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            format!("({}).0.iter().copied()", value.to_owned())
                                        } else {
                                            format!("({}).encode_utf16()", value.to_owned())
                                        };
                                        format!("crate::ml_string_number::convert_java_script_string_number_units(&{units}.map(f64::from).collect::<Vec<f64>>())")
                                    }
                                }
} else {
                    String::new()
                }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_statement() -> String {
    String::from("ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop1(mut exponent: f64, mut value: self::ml::Big, mut base: self::ml::Big) -> self::ml::Big {
    loop {
        if exponent > 0f64 {
            {
                let value_3 = if (exponent % 2f64) == 1f64 {
                    {
                        value.mul(&base)
                    }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = (exponent / 2f64).floor();
                    if exponent_2 > 0f64 {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) = (exponent_2, value_3.clone(), base_2);
                                continue;
                            }
                        }
                    }
{
                        {
                            (exponent, value) = (exponent_2, value_3.clone());
                            continue;
                        }
                    }
                }
            }
        }
return {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop2(mut value: self::ml::Big, mut result: f64, mut scale: f64) -> f64 {
    loop {
        if value > (self::ml::Big::from_i128(0)) {
            {
                let result_3 = if (self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1)) {
                    {
                        result + scale
                    }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let scale_2 = scale * 2f64;
                        {
                            (value, result, scale) = (value_2, result_3, scale_2);
                            continue;
                        }
                    }
                }
            }
        }
return {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop4(mut exponent: f64, mut result: f64) -> f64 {
    loop {
        if exponent < 0f64 {
            {
                let result_2 = result / 2f64;
                {
                    let exponent_2 = exponent + 1f64;
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        }
return {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop3Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop3Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop3(mut exponent: f64, mut result: f64) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop3Result {
    loop {
        if exponent > 0f64 {
            {
                let result_2 = result * 2f64;
                {
                    let exponent_2 = exponent - 1f64;
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        }
return {
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop3Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop3Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop8Result {
    MlSource1TemporaryConvertStringNumberRationalLoop8Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop8(mut quotient: self::ml::Big, mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop8Result {
    loop {
        if weight > (self::ml::Big::from_i128(1)) {
            {
                let scale_2 = self::ml::divide(&scale, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if remainder >= scale_2 {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (quotient_2, remainder_2, scale_2.clone(), weight_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
{
                        {
                            (scale, weight) = (scale_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        }
return {
            self::MlSource1TemporaryConvertStringNumberRationalLoop8Result::MlSource1TemporaryConvertStringNumberRationalLoop8Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop7Result {
    MlSource1TemporaryConvertStringNumberRationalLoop7Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop7(mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop7Result {
    loop {
        if scale <= remainder {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2, weight_2);
                        continue;
                    }
                }
            }
        }
return {
            self::MlSource1TemporaryConvertStringNumberRationalLoop7Result::MlSource1TemporaryConvertStringNumberRationalLoop7Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin6 {
    MlSource1TemporaryConvertStringNumberRationalJoin6Next(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop9(mut numerator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        if index < shift {
            {
                let numerator_2 = numerator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = index + 1f64;
                    {
                        (numerator, index) = (numerator_2, index_2);
                        continue;
                    }
                }
            }
        }
return {
            numerator.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop10(mut denominator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        if index < (-shift) {
            {
                let denominator_2 = denominator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = index + 1f64;
                    {
                        (denominator, index) = (denominator_2, index_2);
                        continue;
                    }
                }
            }
        }
return {
            denominator.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin5 {
    MlSource1TemporaryConvertStringNumberRationalJoin5Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin5Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop11Result {
    MlSource1TemporaryConvertStringNumberRationalLoop11Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop11(mut numerator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop11Result {
    loop {
        return if numerator >= (scale.mul(&self::ml::Big::from_i128(2))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent + 1f64;
                    if exponent_2 >= 1024f64 {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop12Result {
    MlSource1TemporaryConvertStringNumberRationalLoop12Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop12Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop12(mut denominator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop12Result {
    loop {
        return if scale < denominator {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent - 1f64;
                    if exponent_2 < -1075f64 {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop13Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop13Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop13Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_string_number_infinity_loop13(units: &[f64], mut start: f64, expected: &[f64], mut index: f64) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop13Result {
    loop {
        return if index < (expected.len() as f64) {
            {
let ml_library_inequality_1 = ((self::ml_array::optional_at(units, self::ml_array::number_index(start + index))).unwrap_or_else(|| -1f64)) != ((self::ml_array::optional_at(expected, self::ml_array::number_index(index))).unwrap_or_else(|| -2f64)) ;
if ml_library_inequality_1 {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop13Result::MlSource1TemporaryAcceptStringNumberInfinityLoop13Return(false)
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
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop13Result::MlSource1TemporaryAcceptStringNumberInfinityLoop13Done
        };
    }
}
