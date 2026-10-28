// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin70;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin71;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop67Result;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop72Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop60Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop62Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop65Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop74Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop75Result;
use super::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop88Result;
use super::accept_string_number_infinity;
use super::apply_java_script_number_bitwise_operation;
use super::convert_string_number_rational;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_string_number_decimal_power;
use super::source_ml_source1_temporary_convert_java_script_number_big_integer_loop73;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop67;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop68;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop69;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop72;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop55;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop56;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop57;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop60;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop62;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop63;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop65;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop66;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop74;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop75;
use super::source_ml_source1_temporary_power_java_script_big_integer_loop76;
use super::source_ml_source1_temporary_read_java_script_call_integer_width_loop88;

pub fn convert_java_script_string_number_units(units: Vec<f64>) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s852 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop55(units.clone(), index);
            match source_ml_source2_s852.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop56(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop57(units.clone(), start_2, end);
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
                                                        let source_ml_source2_s853 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let start_3 = (start_2 + 1f64);
                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58Next(start_3, negative_2, true)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s853.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58Next(start_4, negative_3, signed_2) => {
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
                                                                                let source_ml_source2_s855 = if ((!signed_2 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
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
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let source_ml_source2_s856 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop66(units.clone(), end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), start_5);
                                                                                                                            match source_ml_source2_s856.clone() {
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Done(value_2, overflow_2) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Return(if overflow_2 {
                                                                                                                                        f64::INFINITY
                                                                                                                                    } else {
                                                                                                                                        self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                    })
                                                                                                                                }
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Return(source_ml_source1_temporary_result66) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Return(source_ml_source1_temporary_result66)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s855.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Next(start_6) => {
                                                                                        {
                                                                                            let at = start_6;
                                                                                            {
                                                                                                let fraction = 0f64;
                                                                                                {
                                                                                                    let digits = 0f64;
                                                                                                    {
                                                                                                        let significant = 0f64;
                                                                                                        {
                                                                                                            let source_ml_source2_s857 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop60(units.clone(), end_2, at, fraction, digits, significant, false);
                                                                                                            match source_ml_source2_s857.clone() {
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop60Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop60Done(at_2, fraction_2, digits_2, significant_2) => {
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
                                                                                                                                        let source_ml_source2_s860 = if ((at_2 < end_2) && ((((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || (((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                            {
                                                                                                                                                let at_3 = (at_2 + 1f64);
                                                                                                                                                {
                                                                                                                                                    let exponent_negative = false;
                                                                                                                                                    {
                                                                                                                                                        let exponent_sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_s861 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                {
                                                                                                                                                                    let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                    {
                                                                                                                                                                        let at_4 = (at_3 + 1f64);
                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64Next(at_4, exponent_negative_2)
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64Next(at_3, exponent_negative)
                                                                                                                                                            };
                                                                                                                                                            match source_ml_source2_s861.clone() {
                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64Next(at_5, exponent_negative_3) => {
                                                                                                                                                                    {
                                                                                                                                                                        let first = at_5;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_s862 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop65(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                            match source_ml_source2_s862.clone() {
                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop65Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop65Done(at_6, exponent_2) => {
                                                                                                                                                                                    if (at_6 == first) {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Return(f64::NAN)
                                                                                                                                                                                    } else {
                                                                                                                                                                                        if exponent_negative_3 {
                                                                                                                                                                                            {
                                                                                                                                                                                                let exponent_3 = (-exponent_2);
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Next(at_6, exponent_3)
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Next(at_6, exponent_2)
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
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Next(at_2, exponent)
                                                                                                                                        };
                                                                                                                                        match source_ml_source2_s860.clone() {
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Next(at_7, exponent_4) => {
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
                                                                                                                                                                                let source_ml_source2_s863 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop62(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                match source_ml_source2_s863.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop62Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop62Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                        {
                                                                                                                                                                                            let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop63(units, integer_digits, mantissa_end_2, mantissa.clone(), start_6);
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
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Return(source_ml_source1_temporary_result63) => {
                                                                                                                                                source_ml_source1_temporary_result63
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Return(source_ml_source1_temporary_result65) => {
                                                                                        source_ml_source1_temporary_result65
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Return(source_ml_source1_temporary_result66_2) => {
                    source_ml_source1_temporary_result66_2
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

pub fn convert_java_script_string_big_integer_units(units: Vec<f64>) -> Vec<self::ml::Big> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s864 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop67(units.clone(), index);
            match source_ml_source2_s864.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop67Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop67Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop68(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop69(units.clone(), start_2, end);
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
                                                        let source_ml_source2_s865 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let start_3 = (start_2 + 1f64);
                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin70::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin70Next(start_3, negative_2, true)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin70::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin70Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s865.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin70::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin70Next(start_4, negative_3, signed_2) => {
                                                                if (start_4 == end_2) {
                                                                    Vec::<self::ml::Big>::new()
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = self::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s866 = if ((!signed_2 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
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
                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin71::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin71Next(start_5, selected, base_2.clone())
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin71::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin71Next(start_4, radix, base.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin71::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin71Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s866.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin71::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin71Next(start_6, radix_2, base_3) => {
                                                                                        if (start_6 == end_2) {
                                                                                            Vec::<self::ml::Big>::new()
                                                                                        } else {
                                                                                            {
                                                                                                let digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let source_ml_source2_s867 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop72(units, end_2, radix_2, base_3.clone(), digits, value.clone(), start_6);
                                                                                                        match source_ml_source2_s867.clone() {
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop72Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop72Done(value_2) => {
                                                                                                                vec![if negative_3 {
                                                                                                                    value_2.neg()
                                                                                                                } else {
                                                                                                                    value_2.clone()
                                                                                                                }]
                                                                                                            }
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop72Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop72Return(source_ml_source1_temporary_result72) => {
                                                                                                                source_ml_source1_temporary_result72
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
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop67Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop67Return(source_ml_source1_temporary_result72_2) => {
                    source_ml_source1_temporary_result72_2
                }
            }
        }
    }
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
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop73(value_2, integer.clone(), weight.clone());
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
                                let source_ml_source2_s868 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop74(remaining.clone(), shifted.clone(), weight.clone());
                                match source_ml_source2_s868.clone() {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop74Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop74Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s869 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop75(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient.clone());
                                            match source_ml_source2_s869.clone() {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop75Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop75Done(remaining_2, quotient_2) => {
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
                let result_2 = self::source_ml_source1_temporary_power_java_script_big_integer_loop76(result.clone(), base.clone(), exponent.clone());
                vec![result_2.clone()]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_arity(operation: &self::ml_string::Text) -> f64 {
    if ((((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115])))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
        1f64
    } else {
        if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]), self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]), self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100])], operation, 0f64, false, |a, b| a == b) {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_width(value: self::ml::Big) -> f64 {
    {
        let rest = if (value < (self::ml::Big::from_i128(0))) {
            value.neg()
        } else {
            value.clone()
        };
        {
            let width = 0f64;
            {
                let source_ml_source2_s875 = self::source_ml_source1_temporary_read_java_script_call_integer_width_loop88(rest.clone(), width);
                match source_ml_source2_s875.clone() {
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop88Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop88Done(width_2) => {
                        width_2
                    }
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop88Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop88Return(source_ml_source1_temporary_result88) => {
                        source_ml_source1_temporary_result88
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_bit_code(operation: &self::ml_string::Text) -> f64 {
    if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
        0f64
    } else {
        if (((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
            1f64
        } else {
            if (((*operation) == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
                2f64
            } else {
                if (((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116])))) {
                    3f64
                } else {
                    if (((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116])))) {
                        4f64
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100]))) {
                            5f64
                        } else {
                            if (((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
                                6f64
                            } else {
                                (-1f64)
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
pub fn execute_java_script_call_number_bits(operation: &self::ml_string::Text, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let left = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let right = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Number(0f64));
            match left.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Boolean(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Number(left_value) => {
                    match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Number(value) => {
                            {
                                let code = self::read_java_script_call_integer_bit_code(operation);
                                if (code < (0f64)) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::apply_java_script_number_bitwise_operation(code, left_value, value))))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Text(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
            }
        }
    }
}
