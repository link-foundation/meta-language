// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Result;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin39;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop50Result;
use super::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Result;
use super::MlSource1TemporaryNormalizeNumberPowerBoundLoop53Result;
use super::accept_string_number_infinity;
use super::convert_string_number_rational;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_java_script_utf8_string_units;
use super::read_string_number_decimal_power;
use super::source_ml_source1_temporary_convert_java_script_number_big_integer_loop48;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop42;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop43;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop44;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop47;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop30;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop31;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop32;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop35;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop37;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop38;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop40;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop41;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop49;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop50;
use super::source_ml_source1_temporary_normalize_number_power_bound_loop52;
use super::source_ml_source1_temporary_normalize_number_power_bound_loop53;
use super::source_ml_source1_temporary_power_java_script_big_integer_loop51;

pub fn convert_java_script_string_number_units(units: Vec<f64>) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s150 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop30(units.clone(), index);
            match source_ml_source2_s150.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop31(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop32(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        0f64
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(start_2))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s151 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s151.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Next(start_4, negative_3, signed_3) => {
                                                                if (start_4 == end_2) {
                                                                    f64::NAN
                                                                } else {
                                                                    if self::accept_string_number_infinity(units.clone(), start_4, end_2) {
                                                                        if negative_3 {
                                                                            f64::NEG_INFINITY
                                                                        } else {
                                                                            f64::INFINITY
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let integer_digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                            {
                                                                                let source_ml_source2_s153 = if ((!signed_3 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(&units, self::ml_array::number_index((start_4 + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                                                                                        {
                                                                                            let radix = if ((marker == (120f64)) || (marker == (88f64))) {
                                                                                                16f64
                                                                                            } else {
                                                                                                if ((marker == (98f64)) || (marker == (66f64))) {
                                                                                                    2f64
                                                                                                } else {
                                                                                                    if ((marker == (111f64)) || (marker == (79f64))) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            if (radix != (0f64)) {
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let overflow = false;
                                                                                                        {
                                                                                                            let limit = self::read_string_number_decimal_power(310f64);
                                                                                                            {
                                                                                                                let base = if (radix == (16f64)) {
                                                                                                                    self::ml::Big::from_i128(16)
                                                                                                                } else {
                                                                                                                    if (radix == (8f64)) {
                                                                                                                        self::ml::Big::from_i128(8)
                                                                                                                    } else {
                                                                                                                        self::ml::Big::from_i128(2)
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let start_5 = (start_4 + 2f64);
                                                                                                                    if (start_5 == end_2) {
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let index_2 = start_5;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s154 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop41(units.clone(), end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), index_2);
                                                                                                                                match source_ml_source2_s154.clone() {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Done(value_2, overflow_2) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34Return(if overflow_2 {
                                                                                                                                            f64::INFINITY
                                                                                                                                        } else {
                                                                                                                                            self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                        })
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop41Return(source_ml_source1_temporary_result41) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34Return(source_ml_source1_temporary_result41)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s153.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34Next(start_6) => {
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
                                                                                                                let source_ml_source2_s155 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop35(units.clone(), end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s155.clone() {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop35Done(at_2, fraction_2, digits_2, significant_2) => {
                                                                                                                        if (digits_2 == (0f64)) {
                                                                                                                            f64::NAN
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let decimal_end = at_2;
                                                                                                                                {
                                                                                                                                    let exponent = 0f64;
                                                                                                                                    {
                                                                                                                                        let exponent_limit = ((units.len() as f64) + 10000f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s158 = if ((at_2 < end_2) && ((((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || (((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = (at_2 + 1f64);
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s159 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = (at_3 + 1f64);
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin39::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin39Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin39::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin39Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s159.clone() {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin39::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin39Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s160 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop40(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s160.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Done(at_6, exponent_2) => {
                                                                                                                                                                                        if (at_6 == first) {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36Return(f64::NAN)
                                                                                                                                                                                        } else {
                                                                                                                                                                                            if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = (-exponent_2);
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36Next(at_6, exponent_2)
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
                                                                                                                                            } else {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s158.clone() {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36Next(at_7, exponent_4) => {
                                                                                                                                                    if (at_7 != end_2) {
                                                                                                                                                        f64::NAN
                                                                                                                                                    } else {
                                                                                                                                                        if (significant_2 == (0f64)) {
                                                                                                                                                            if negative_3 {
                                                                                                                                                                (-0f64)
                                                                                                                                                            } else {
                                                                                                                                                                0f64
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            {
                                                                                                                                                                let exponent_5 = (exponent_4 - fraction_2);
                                                                                                                                                                {
                                                                                                                                                                    let order = (significant_2 + exponent_5);
                                                                                                                                                                    if (order > (310f64)) {
                                                                                                                                                                        if negative_3 {
                                                                                                                                                                            f64::NEG_INFINITY
                                                                                                                                                                        } else {
                                                                                                                                                                            f64::INFINITY
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        if (order < ((-325f64))) {
                                                                                                                                                                            if negative_3 {
                                                                                                                                                                                (-0f64)
                                                                                                                                                                            } else {
                                                                                                                                                                                0f64
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            {
                                                                                                                                                                                let mantissa_end = decimal_end;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_s161 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop37(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s161.clone() {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop37Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let index_3 = start_6;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop38(units, integer_digits, mantissa_end_2, mantissa.clone(), index_3);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let numerator = if (exponent_6 >= (0f64)) {
                                                                                                                                                                                                                mantissa_2.mul(&self::read_string_number_decimal_power(exponent_6))
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                mantissa_2.clone()
                                                                                                                                                                                                            };
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let denominator = if (exponent_6 < (0f64)) {
                                                                                                                                                                                                                    self::read_string_number_decimal_power((-exponent_6))
                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                    self::ml::Big::from_i128(1)
                                                                                                                                                                                                                };
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let value_3 = self::convert_string_number_rational(numerator.clone(), denominator.clone());
                                                                                                                                                                                                                    if negative_3 {
                                                                                                                                                                                                                        (-value_3)
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
                                                                                                                                                }
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin36Return(source_ml_source1_temporary_result38) => {
                                                                                                                                                    source_ml_source1_temporary_result38
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin34Return(source_ml_source1_temporary_result40) => {
                                                                                        source_ml_source1_temporary_result40
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop30Return(source_ml_source1_temporary_result41_2) => {
                    source_ml_source1_temporary_result41_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_big_integer_number(value: self::ml::Big) -> f64 {
    if (value < (self::ml::Big::from_i128(0))) {
        (-(self::convert_string_number_rational(value.neg(), self::ml::Big::from_i128(1))))
    } else {
        self::convert_string_number_rational(value.clone(), self::ml::Big::from_i128(1))
    }
}

pub fn convert_java_script_utf8_string_number_units(bytes: Vec<f64>) -> f64 {
    self::convert_java_script_string_number_units(self::read_java_script_utf8_string_units(bytes))
}

pub fn read_java_script_number_builtin_operation(name: String) -> String {
    if (name == "Number") {
        String::from("convertJavaScriptNumber")
    } else {
        if (name == "isFinite") {
            String::from("testCoercedJavaScriptNumberFinite")
        } else {
            if (name == "isNaN") {
                String::from("testCoercedJavaScriptNumberNaN")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_java_script_number_conversion_result(operation: String) -> String {
    if ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber")) {
        String::from("float")
    } else {
        if ((operation == "testCoercedJavaScriptNumberFinite") || (operation == "testCoercedJavaScriptNumberNaN")) {
            String::from("bool")
        } else {
            String::from("")
        }
    }
}

pub fn read_java_script_number_conversion_form(kind: String) -> String {
    if (((((((kind == "float") || (kind == "bool")) || (kind == "string")) || (kind == "unit")) || (kind == "int")) || (kind == "nat")) || (kind == "option")) {
        kind.clone()
    } else {
        String::from("unsupported")
    }
}

pub fn accept_linked_java_script_number_conversion(kind: String, operation: String) -> bool {
    ((kind == "math") && ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber")))
}

pub fn read_java_script_number_conversion_failure(operation: String, kind: String) -> String {
    if ((operation == "coerceJavaScriptNumber") && ((kind == "int") || (kind == "nat"))) {
        String::from("Cannot convert a BigInt value to a number")
    } else {
        String::from("")
    }
}

pub fn require_java_script_number_conversion_units(kind: String, operation: String, argument: String) -> bool {
    (self::accept_linked_java_script_number_conversion(kind.clone(), operation.clone()) && (argument == "string"))
}

pub fn read_java_script_primitive_number_operand(language: String, kind: String, element: String) -> String {
    if (language != "JavaScript") {
        String::from("")
    } else {
        if (((kind == "bool") || (kind == "string")) || (kind == "unit")) {
            String::from("coerceJavaScriptNumber")
        } else {
            if ((kind == "option") && (element == "string")) {
                String::from("coerceJavaScriptNumber")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_java_script_primitive_numeric_inference_kind(kind: String) -> String {
    if (((kind == "bool") || (kind == "string")) || (kind == "unit")) {
        String::from("float")
    } else {
        String::from("")
    }
}

pub fn read_java_script_primitive_unary_operation(symbol: String) -> String {
    if (symbol == "+") {
        String::from("coerceJavaScriptNumber")
    } else {
        String::from("")
    }
}

pub fn read_java_script_mathematical_conversion_form(language: String, operation: String) -> String {
    if (language != "JavaScript") {
        String::from("")
    } else {
        if (operation == "powerJavaScriptNumberArguments") {
            String::from("binary")
        } else {
            if ((operation == "max") || (operation == "min")) {
                String::from("variadic")
            } else {
                if (((((((operation == "abs") || (operation == "floor")) || (operation == "ceil")) || (operation == "trunc")) || (operation == "round")) || (operation == "sign")) || (operation == "sqrt")) {
                    String::from("unary")
                } else {
                    String::from("")
                }
            }
        }
    }
}

pub fn render_java_script_number_scalar_conversion(format: String, operation: String, form: String, value: String, utf16: bool) -> String {
    if ((!(self::accept_linked_java_script_number_conversion(String::from("math"), operation.clone())) || ((self::read_java_script_number_conversion_form(form.clone())) == "unsupported")) || (form == "option")) {
        String::from("")
    } else {
        if (format == "JavaScript") {
            if (operation == "convertJavaScriptNumber") {
                format!("{}{}", (format!("{}{}", (String::from("Number(")), value)), (String::from(")")))
            } else {
                format!("{}{}", (format!("{}{}", (String::from("(+(")), value)), (String::from("))")))
            }
        } else {
            if ((format == "Lean") || (format == "Rocq")) {
                if !(self::read_java_script_number_conversion_failure(operation.clone(), form.clone())).is_empty() {
                    String::from("")
                } else {
                    if (form == "float") {
                        value.clone()
                    } else {
                        if (form == "unit") {
                            if (format == "Lean") {
                                format!("{}{}", (format!("{}{}", (String::from("(let _ := ")), value)), (String::from("; ((0 : Float) / 0))")))
                            } else {
                                format!("{}{}", (format!("{}{}", (String::from("(let _ := ")), value)), (String::from(" in PrimFloat.nan)")))
                            }
                        } else {
                            if (form == "bool") {
                                if (format == "Lean") {
                                    format!("{}{}", (format!("{}{}", (String::from("(if ")), value)), (String::from(" then (1 : Float) else (0 : Float))")))
                                } else {
                                    format!("{}{}", (format!("{}{}", (String::from("(if ")), value)), (String::from(" then 1%float else 0%float)")))
                                }
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertInteger ")), (if (form == "nat") {
                                        if (format == "Lean") {
                                            format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat ")), value)), (String::from(")")))
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(Z.of_N ")), value)), (String::from(")")))
                                        }
                                    } else {
                                        value.clone()
                                    }))), (String::from(")")))
                                } else {
                                    if (format == "Lean") {
                                        format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] ")), value)), (String::from("))")))
                                    } else {
                                        format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) ")), value)), (String::from("))")))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if (format != "Rust") {
                    String::from("")
                } else {
                    if (form == "float") {
                        value.clone()
                    } else {
                        if (form == "bool") {
                            format!("{}{}", (format!("{}{}", (String::from("(if ")), value)), (String::from(" { 1.0f64 } else { 0.0f64 })")))
                        } else {
                            if (form == "unit") {
                                format!("{}{}", (format!("{}{}", (String::from("{ let _ = ")), value)), (String::from("; f64::NAN }")))
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    if (operation == "coerceJavaScriptNumber") {
                                        format!("{}{}", (format!("{}{}", (String::from("{ let _ = ")), value)), (String::from("; crate::ml_source_exception::SourceException::from_text(\"TypeError\", \"Cannot convert a BigInt value to a number\").raise() }")))
                                    } else {
                                        format!("{}{}", (format!("{}{}", (String::from("crate::ml_string_number::convert_java_script_big_integer_number(")), value)), (String::from(")")))
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            format!("{}{}", (format!("{}{}", (String::from("(")), value)), (String::from(").0.iter().copied()")))
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(")), value)), (String::from(").encode_utf16()")))
                                        };
                                        format!("{}{}", (format!("{}{}", (String::from("crate::ml_string_number::convert_java_script_string_number_units(&")), units)), (String::from(".map(f64::from).collect::<Vec<f64>>())")))
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
pub fn read_java_script_number_conversion_statement() -> String {
    String::from("ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws")
}

pub fn convert_java_script_string_big_integer_units(units: Vec<f64>) -> Vec<self::ml::Big> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s162 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop42(units.clone(), index);
            match source_ml_source2_s162.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop43(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop44(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        vec![self::ml::Big::from_i128(0)]
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(start_2))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s163 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s163.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin45Next(start_4, negative_3, signed_3) => {
                                                                if (start_4 == end_2) {
                                                                    Vec::<self::ml::Big>::new()
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = self::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s164 = if ((!signed_3 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(&units, self::ml_array::number_index((start_4 + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                                                                                        {
                                                                                            let selected = if ((marker == (120f64)) || (marker == (88f64))) {
                                                                                                16f64
                                                                                            } else {
                                                                                                if ((marker == (98f64)) || (marker == (66f64))) {
                                                                                                    2f64
                                                                                                } else {
                                                                                                    if ((marker == (111f64)) || (marker == (79f64))) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            if (selected != (0f64)) {
                                                                                                {
                                                                                                    let radix_2 = selected;
                                                                                                    {
                                                                                                        let base_2 = if (selected == (16f64)) {
                                                                                                            self::ml::Big::from_i128(16)
                                                                                                        } else {
                                                                                                            if (selected == (8f64)) {
                                                                                                                self::ml::Big::from_i128(8)
                                                                                                            } else {
                                                                                                                self::ml::Big::from_i128(2)
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let start_5 = (start_4 + 2f64);
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46Next(start_5, radix_2, base_2.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46Next(start_4, radix, base.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s164.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin46Next(start_6, radix_3, base_3) => {
                                                                                        if (start_6 == end_2) {
                                                                                            Vec::<self::ml::Big>::new()
                                                                                        } else {
                                                                                            {
                                                                                                let digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let index_2 = start_6;
                                                                                                        {
                                                                                                            let source_ml_source2_s165 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop47(units, end_2, radix_3, base_3.clone(), digits, value.clone(), index_2);
                                                                                                            match source_ml_source2_s165.clone() {
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Done(value_2) => {
                                                                                                                    vec![if negative_3 {
                                                                                                                        value_2.neg()
                                                                                                                    } else {
                                                                                                                        value_2.clone()
                                                                                                                    }]
                                                                                                                }
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop47Return(source_ml_source1_temporary_result47) => {
                                                                                                                    source_ml_source1_temporary_result47
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
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop42Return(source_ml_source1_temporary_result47_2) => {
                    source_ml_source1_temporary_result47_2
                }
            }
        }
    }
}

pub fn convert_java_script_utf8_big_integer(bytes: Vec<f64>) -> Vec<self::ml::Big> {
    self::convert_java_script_string_big_integer_units(self::read_java_script_utf8_string_units(bytes))
}

pub fn convert_java_script_number_big_integer(value: f64) -> Vec<self::ml::Big> {
    if !(self::ml_math::is_integer(value)) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let negative = (value < (0f64));
            {
                let value_2 = value.abs();
                {
                    let integer = self::ml::Big::from_i128(0);
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop48(value_2, integer.clone(), weight.clone());
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
pub fn compare_java_script_number_big_integer(value: f64, integer: self::ml::Big) -> f64 {
    if value.is_nan() {
        2f64
    } else {
        if (value == (f64::INFINITY)) {
            1f64
        } else {
            if (value == (f64::NEG_INFINITY)) {
                (-1f64)
            } else {
                {
                    let floor = value.floor();
                    {
                        let lower = (self::ml_array::optional_at(&self::convert_java_script_number_big_integer(floor), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                        if (lower < integer) {
                            (-1f64)
                        } else {
                            if (lower > integer) {
                                1f64
                            } else {
                                if (value == floor) {
                                    0f64
                                } else {
                                    1f64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn compare_java_script_string_big_integer_units(units: Vec<f64>, integer: self::ml::Big) -> f64 {
    {
        let result = self::convert_java_script_string_big_integer_units(units);
        if (((result.len() as f64)) != (1f64)) {
            2f64
        } else {
            {
                let value = (self::ml_array::optional_at(&result, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                if (value < integer) {
                    (-1f64)
                } else {
                    if (value > integer) {
                        1f64
                    } else {
                        0f64
                    }
                }
            }
        }
    }
}

pub fn compare_java_script_utf8_big_integer(bytes: Vec<f64>, integer: self::ml::Big) -> f64 {
    self::compare_java_script_string_big_integer_units(self::read_java_script_utf8_string_units(bytes), integer.clone())
}

pub fn divide_java_script_big_integer(left: self::ml::Big, right: self::ml::Big) -> Vec<self::ml::Big> {
    if (right == (self::ml::Big::from_i128(0))) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let remaining = if (left < (self::ml::Big::from_i128(0))) {
                left.neg()
            } else {
                left.clone()
            };
            {
                let divisor = if (right < (self::ml::Big::from_i128(0))) {
                    right.neg()
                } else {
                    right.clone()
                };
                {
                    let shifted = divisor.clone();
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let quotient = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s166 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop49(remaining.clone(), shifted.clone(), weight.clone());
                                match source_ml_source2_s166.clone() {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop49Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s167 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop50(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient.clone());
                                            match source_ml_source2_s167.clone() {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop50Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop50Done(remaining_2, quotient_2) => {
                                                    vec![if (((left < (self::ml::Big::from_i128(0)))) != ((right < (self::ml::Big::from_i128(0))))) {
                                                        quotient_2.neg()
                                                    } else {
                                                        quotient_2.clone()
                                                    }, if (left < (self::ml::Big::from_i128(0))) {
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

pub fn power_java_script_big_integer(base: self::ml::Big, exponent: self::ml::Big) -> Vec<self::ml::Big> {
    if (exponent < (self::ml::Big::from_i128(0))) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let result = self::ml::Big::from_i128(1);
            {
                let weight = base.clone();
                {
                    let remaining = exponent.clone();
                    {
                        let result_2 = self::source_ml_source1_temporary_power_java_script_big_integer_loop51(result.clone(), weight.clone(), remaining.clone());
                        vec![result_2.clone()]
                    }
                }
            }
        }
    }
}

pub fn normalize_number_power_bound(value: self::ml::Big, exponent: self::ml::Big, scale: self::ml::Big, upward: bool) -> Vec<self::ml::Big> {
    if (value <= (self::ml::Big::from_i128(0))) {
        vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(0)]
    } else {
        {
            let source_ml_source2_s168 = self::source_ml_source1_temporary_normalize_number_power_bound_loop52(value.clone(), exponent.clone(), scale.clone(), upward);
            match source_ml_source2_s168.clone() {
                self::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop52Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s169 = self::source_ml_source1_temporary_normalize_number_power_bound_loop53(value_2.clone(), exponent_2.clone(), scale.clone());
                        match source_ml_source2_s169.clone() {
                            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop53Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop53Done(value_3, exponent_3) => {
                                vec![value_3.clone(), exponent_3.clone()]
                            }
                        }
                    }
                }
            }
        }
    }
}
