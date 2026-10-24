// Generated from complete js/src/translation/big-integer-conversion-rules.js. Do not edit by hand.
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result;
use super::accept_string_number_infinity;
use super::convert_string_number_rational;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_java_script_utf8_string_units;
use super::read_string_number_decimal_power;
use super::source_ml_source1_temporary_convert_java_script_number_big_integer_loop36;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop30;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop31;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop32;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop35;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop18;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop19;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop20;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop23;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop25;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop26;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop28;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop29;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop37;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop38;
use super::source_ml_source1_temporary_power_java_script_big_integer_loop39;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_string_number_units(units: &[f64]) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s13 =
                self::source_ml_source1_temporary_convert_java_script_string_number_units_loop18(
                    units, index,
                );
            match source_ml_source2_s13 {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Done => {
                    {
                        let start = 0f64;
                        {
                            let end = units.len() as f64;
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop19(units, start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop20(units, start_2, end);
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
                                                        let source_ml_source2_s14 = if (sign == 43f64) || (sign == 45f64) {
                                                            {
                                                                let negative_2 = sign == 45f64;
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = start_2 + 1f64;
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s14 {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_4, negative_3, signed_3) => {
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
                                                                                let source_ml_source2_s16 = if (!signed_3 && (((self::ml_array::optional_at(units, self::ml_array::number_index(start_4))).unwrap_or_else(|| -1f64)) == 48f64)) && ((end_2 - start_4) >= 2f64) {
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
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let index_2 = start_5;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s17 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units, end_2, integer_digits.clone(), radix, value, overflow, limit, base, index_2);
                                                                                                                                match source_ml_source2_s17 {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done(value_2, overflow_2) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(if overflow_2 {
                                                                                                                                            f64::INFINITY
                                                                                                                                        } else {
                                                                                                                                            self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                        })
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(source_ml_source1_temporary_result29) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(source_ml_source1_temporary_result29)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_4)
                                                                                            }
}
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s16 {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_6) => {
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
                                                                                                                let source_ml_source2_s18 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop23(units, end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s18 {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(at_2, fraction_2, digits_2, significant_2) => {
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
                                                                                                                                            let source_ml_source2_s21 = if (at_2 < end_2) && ((((self::ml_array::optional_at(units, self::ml_array::number_index(at_2))).unwrap_or_else(|| -1f64)) == 101f64) || (((self::ml_array::optional_at(units, self::ml_array::number_index(at_2))).unwrap_or_else(|| -1f64)) == 69f64)) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = at_2 + 1f64;
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(units, self::ml_array::number_index(at_3))).unwrap_or_else(|| -1f64);
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s22 = if (exponent_sign == 43f64) || (exponent_sign == 45f64) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = exponent_sign == 45f64;
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = at_3 + 1f64;
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s22 {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s23 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop28(units, end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s23 {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(at_6, exponent_2) => {
                                                                                                                                                                                        if at_6 == first {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(f64::NAN)
                                                                                                                                                                                        } else if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = -exponent_2;
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_6, exponent_2)
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s21 {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_7, exponent_4) => {
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
                                                                                                                                                                                    let source_ml_source2_s24 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop25(units, start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s24 {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let index_3 = start_6;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop26(units, integer_digits, mantissa_end_2, mantissa, index_3);
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(source_ml_source1_temporary_result26) => {
                                                                                                                                                    source_ml_source1_temporary_result26
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(source_ml_source1_temporary_result28) => {
                                                                                        source_ml_source1_temporary_result28
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Return(source_ml_source1_temporary_result29_2) => {
                    source_ml_source1_temporary_result29_2
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
    self::convert_java_script_string_number_units(
        &(self::read_java_script_utf8_string_units(bytes)),
    )
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
    } else if (operation == "testCoercedJavaScriptNumberFinite")
        || (operation == "testCoercedJavaScriptNumberNaN")
    {
        String::from("bool")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_form(kind: &str) -> String {
    if ((((((kind == "float") || (kind == "bool")) || (kind == "string")) || (kind == "unit"))
        || (kind == "int"))
        || (kind == "nat"))
        || (kind == "option")
    {
        kind.to_owned()
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_number_conversion(kind: &str, operation: &str) -> bool {
    (kind == "math")
        && ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber"))
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
pub fn require_java_script_number_conversion_units(
    kind: &str,
    operation: &str,
    argument: &str,
) -> bool {
    self::accept_linked_java_script_number_conversion(kind, operation) && (argument == "string")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_number_operand(
    language: &str,
    kind: &str,
    element: &str,
) -> String {
    if language == "JavaScript" {
        if (((kind == "bool") || (kind == "string")) || (kind == "unit"))
            || ((kind == "option") && (element == "string"))
        {
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
        if operation == "powerJavaScriptNumberArguments" {
            String::from("binary")
        } else if (operation == "max") || (operation == "min") {
            String::from("variadic")
        } else if ((((((operation == "abs") || (operation == "floor")) || (operation == "ceil"))
            || (operation == "trunc"))
            || (operation == "round"))
            || (operation == "sign"))
            || (operation == "sqrt")
        {
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
pub fn render_java_script_number_scalar_conversion(
    format: &str,
    operation: &str,
    form: &str,
    value: &str,
    utf16: bool,
) -> String {
    if (!(self::accept_linked_java_script_number_conversion("math", operation))
        || ((self::read_java_script_number_conversion_form(form)) == "unsupported"))
        || (form == "option")
    {
        String::new()
    } else if format == "JavaScript" {
        if operation == "convertJavaScriptNumber" {
            format!("Number({})", value.to_owned())
        } else {
            format!("(+({}))", value.to_owned())
        }
    } else if (format == "Lean") || (format == "Rocq") {
        if (self::read_java_script_number_conversion_failure(operation, form)).is_empty() {
            if form == "float" {
                value.to_owned()
            } else if form == "unit" {
                if format == "Lean" {
                    format!("(let _ := {}; ((0 : Float) / 0))", value.to_owned())
                } else {
                    format!("(let _ := {} in PrimFloat.nan)", value.to_owned())
                }
            } else if form == "bool" {
                if format == "Lean" {
                    format!(
                        "(if {} then (1 : Float) else (0 : Float))",
                        value.to_owned()
                    )
                } else {
                    format!("(if {} then 1%float else 0%float)", value.to_owned())
                }
            } else if (form == "int") || (form == "nat") {
                format!(
                    "(MLStringNumber.convertInteger {})",
                    if form == "nat" {
                        if format == "Lean" {
                            format!(
                                "{}{}",
                                (format!("{}{}", (String::from("(Int.ofNat ")), value.to_owned())),
                                (String::from(")"))
                            )
                        } else {
                            format!(
                                "{}{}",
                                (format!("{}{}", (String::from("(Z.of_N ")), value.to_owned())),
                                (String::from(")"))
                            )
                        }
                    } else {
                        value.to_owned()
                    }
                )
            } else if format == "Lean" {
                format!(
                    "(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] {}))",
                    value.to_owned()
                )
            } else {
                format!(
                    "(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) {}))",
                    value.to_owned()
                )
            }
        } else {
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
                format!(
                    "{{ let _ = {}; crate::ml_source_exception::SourceException::from_text(\"TypeError\", \"Cannot convert a BigInt value to a number\").raise() }}",
                    value.to_owned()
                )
            } else {
                format!(
                    "crate::ml_string_number::convert_java_script_big_integer_number({})",
                    value.to_owned()
                )
            }
        } else {
            {
                let units = if utf16 {
                    format!("({}).0.iter().copied()", value.to_owned())
                } else {
                    format!("({}).encode_utf16()", value.to_owned())
                };
                format!(
                    "crate::ml_string_number::convert_java_script_string_number_units(&{units}.map(f64::from).collect::<Vec<f64>>())"
                )
            }
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_statement() -> String {
    String::from(
        "ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws",
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_string_big_integer_units(units: &[f64]) -> Vec<self::ml::Big> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s25 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop30(units, index);
            match source_ml_source2_s25 {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Done => {
                    {
                        let start = 0f64;
                        {
                            let end = units.len() as f64;
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop31(units, start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop32(units, start_2, end);
                                    if start_2 == end_2 {
                                        vec![self::ml::Big::from_i128(0)]
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(units, self::ml_array::number_index(start_2))).unwrap_or_else(|| -1f64);
                                                    {
                                                        let source_ml_source2_s26 = if (sign == 43f64) || (sign == 45f64) {
                                                            {
                                                                let negative_2 = sign == 45f64;
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = start_2 + 1f64;
                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s26 {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(start_4, negative_3, signed_3) => {
                                                                if start_4 == end_2 {
                                                                    Vec::<self::ml::Big>::new()
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = self::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s27 = if (!signed_3 && (((self::ml_array::optional_at(units, self::ml_array::number_index(start_4))).unwrap_or_else(|| -1f64)) == 48f64)) && ((end_2 - start_4) >= 2f64) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(units, self::ml_array::number_index(start_4 + 1f64))).unwrap_or_else(|| -1f64);
                                                                                        {
                                                                                            let selected = if (marker == 120f64) || (marker == 88f64) {
                                                                                                16f64
                                                                                            } else if (marker == 98f64) || (marker == 66f64) {
                                                                                                    2f64
                                                                                                } else if (marker == 111f64) || (marker == 79f64) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    };
                                                                                            {
let ml_library_inequality_1 = selected != 0f64 ;
if ml_library_inequality_1 {
                                                                                                {
                                                                                                    let radix_2 = selected;
                                                                                                    {
                                                                                                        let base_2 = if selected == 16f64 {
                                                                                                            self::ml::Big::from_i128(16)
                                                                                                        } else if selected == 8f64 {
                                                                                                                self::ml::Big::from_i128(8)
                                                                                                            } else {
                                                                                                                self::ml::Big::from_i128(2)
                                                                                                            };
                                                                                                        {
                                                                                                            let start_5 = start_4 + 2f64;
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_5, radix_2, base_2)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_4, radix, base.clone())
                                                                                            }
}
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s27 {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_6, radix_3, base_3) => {
                                                                                        if start_6 == end_2 {
                                                                                            Vec::<self::ml::Big>::new()
                                                                                        } else {
                                                                                            {
                                                                                                let digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let index_2 = start_6;
                                                                                                        {
                                                                                                            let source_ml_source2_s28 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop35(units, end_2, radix_3, base_3.clone(), digits, value, index_2);
                                                                                                            match source_ml_source2_s28 {
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Done(value_2) => {
                                                                                                                    vec![if negative_3 {
                                                                                                                        value_2.neg()
                                                                                                                    } else {
                                                                                                                        value_2.clone()
                                                                                                                    }]
                                                                                                                }
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop35Return(source_ml_source1_temporary_result35) => {
                                                                                                                    source_ml_source1_temporary_result35
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
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Return(source_ml_source1_temporary_result35_2) => {
                    source_ml_source1_temporary_result35_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_utf8_big_integer(bytes: &[f64]) -> Vec<self::ml::Big> {
    self::convert_java_script_string_big_integer_units(
        &(self::read_java_script_utf8_string_units(bytes)),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_number_big_integer(value: f64) -> Vec<self::ml::Big> {
    if (self::ml_math::is_integer(value)) {
        {
            let negative = value < 0f64;
            {
                let value_2 = value.abs();
                {
                    let integer = self::ml::Big::from_i128(0);
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop36(value_2, integer, weight);
                            vec![if negative {
                                integer_2.neg()
                            } else {
                                integer_2.clone()
                            }]
                        }
                    }
                }
            }
        }
    } else {
        Vec::<self::ml::Big>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_boolean_big_integer(value: bool) -> self::ml::Big {
    if value {
        self::ml::Big::from_i128(1)
    } else {
        self::ml::Big::from_i128(0)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn divide_java_script_big_integer(
    left: self::ml::Big,
    right: self::ml::Big,
) -> Vec<self::ml::Big> {
    if right == (self::ml::Big::from_i128(0)) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let remaining = if left < (self::ml::Big::from_i128(0)) {
                left.neg()
            } else {
                left.clone()
            };
            {
                let divisor = if right < (self::ml::Big::from_i128(0)) {
                    right.neg()
                } else {
                    right.clone()
                };
                {
                    let shifted = divisor;
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let quotient = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s29 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop37(remaining.clone(), shifted, weight);
                                match source_ml_source2_s29 {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s30 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop38(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient);
                                            match source_ml_source2_s30 {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Done(remaining_2, quotient_2) => {
                                                    vec![if (left < (self::ml::Big::from_i128(0))) != (right < (self::ml::Big::from_i128(0))) {
                                                        quotient_2.neg()
                                                    } else {
                                                        quotient_2.clone()
                                                    }, if left < (self::ml::Big::from_i128(0)) {
                                                        remaining_2.neg()
                                                    } else {
                                                        remaining_2.clone()
                                                    }]
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn power_java_script_big_integer(
    base: self::ml::Big,
    exponent: self::ml::Big,
) -> Vec<self::ml::Big> {
    if exponent < (self::ml::Big::from_i128(0)) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let result = self::ml::Big::from_i128(1);
            {
                let weight = base.clone();
                {
                    let remaining = exponent.clone();
                    {
                        let result_2 =
                            self::source_ml_source1_temporary_power_java_script_big_integer_loop39(
                                result, weight, remaining,
                            );
                        vec![result_2]
                    }
                }
            }
        }
    }
}
