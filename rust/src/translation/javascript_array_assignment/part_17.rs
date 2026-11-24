// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin173;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Result;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin161;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin167;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result;
use super::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Result;
use super::accept_string_number_infinity;
use super::convert_string_number_rational;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_string_number_decimal_power;
use super::source_ml_source1_temporary_convert_java_script_number_big_integer_loop176;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop170;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop171;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop172;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop175;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop158;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop159;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop160;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop163;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop165;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop166;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop168;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop169;
use super::source_ml_source1_temporary_read_java_script_call_integer_text_loop192;
use super::source_ml_source1_temporary_read_java_script_call_integer_width_loop191;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_string_number_units(
    units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s16615 =
                self::source_ml_source1_temporary_convert_java_script_string_number_units_loop158(
                    units.clone(),
                    index,
                );
            match source_ml_source2_s16615 {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Done => {
                    {
                        let start = 0f64;
                        {
                            let end = { let ml_reference_array199 = units.clone(); ml_reference_array199.borrow().len() as f64 };
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop159(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop160(units.clone(), start_2, end);
                                    if start_2 == end_2 {
                                        0f64
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = ({ let ml_reference_array200 = units.clone(); let ml_reference_position200 = start_2; self::ml_array::optional_at(&*ml_reference_array200.borrow(), self::ml_array::number_index(ml_reference_position200)) }).clone().unwrap_or_else(|| -1f64);
                                                    {
                                                        let source_ml_source2_s16618 = if (sign == 43f64) || (sign == 45f64) {
                                                            {
                                                                let negative_2 = sign == 45f64;
                                                                {
                                                                    let start_3 = start_2 + 1f64;
                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin161::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin161Next(start_3, negative_2, true)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin161::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin161Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s16618 {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin161::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin161Next(start_4, negative_3, signed_2) => {
                                                                if start_4 == end_2 {
                                                                    f64::NAN
                                                                } else if self::accept_string_number_infinity(units.clone(), start_4, end_2) {
                                                                        if negative_3 {
                                                                            f64::NEG_INFINITY
                                                                        } else {
                                                                            f64::INFINITY
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let integer_digits = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)]));
                                                                            {
                                                                                let source_ml_source2_s16626 = if (!signed_2 && ((({ let ml_reference_array201 = units.clone(); let ml_reference_position201 = start_4; self::ml_array::optional_at(&*ml_reference_array201.borrow(), self::ml_array::number_index(ml_reference_position201)) }).clone().unwrap_or_else(|| -1f64)) == 48f64)) && ((end_2 - start_4) >= 2f64) {
                                                                                    {
                                                                                        let marker = ({
                                                                                            let source_ml_source2_array_read_index16632 = start_4 + 1f64;
                                                                                            { let ml_reference_array202 = units.clone(); let ml_reference_position202 = source_ml_source2_array_read_index16632; self::ml_array::optional_at(&*ml_reference_array202.borrow(), self::ml_array::number_index(ml_reference_position202)) }
                                                                                        }).clone().unwrap_or_else(|| -1f64);
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
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let source_ml_source2_s16633 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop169(units.clone(), end_2, integer_digits.clone(), radix, value, overflow, limit, base, start_5);
                                                                                                                            match source_ml_source2_s16633 {
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Done(value_2, overflow_2) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Return(if overflow_2 {
                                                                                                                                        f64::INFINITY
                                                                                                                                    } else {
                                                                                                                                        self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                    })
                                                                                                                                }
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Return(source_ml_source1_temporary_result169) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Return(source_ml_source1_temporary_result169)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Next(start_4)
                                                                                            }
}
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s16626 {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Next(start_6) => {
                                                                                        {
                                                                                            let at = start_6;
                                                                                            {
                                                                                                let fraction = 0f64;
                                                                                                {
                                                                                                    let digits = 0f64;
                                                                                                    {
                                                                                                        let significant = 0f64;
                                                                                                        {
                                                                                                            let source_ml_source2_s16634 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop163(units.clone(), end_2, at, fraction, digits, significant, false);
                                                                                                            match source_ml_source2_s16634 {
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Done(at_2, fraction_2, digits_2, significant_2) => {
                                                                                                                    if digits_2 == 0f64 {
                                                                                                                        f64::NAN
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let decimal_end = at_2;
                                                                                                                            {
                                                                                                                                let exponent = 0f64;
                                                                                                                                {
                                                                                                                                    let exponent_limit = { let ml_reference_array203 = units.clone(); ml_reference_array203.borrow().len() as f64 } + 10_000f64;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_s16647 = if (at_2 < end_2) && (((({ let ml_reference_array204 = units.clone(); let ml_reference_position204 = at_2; self::ml_array::optional_at(&*ml_reference_array204.borrow(), self::ml_array::number_index(ml_reference_position204)) }).clone().unwrap_or_else(|| -1f64)) == 101f64) || ((({ let ml_reference_array205 = units.clone(); let ml_reference_position205 = at_2; self::ml_array::optional_at(&*ml_reference_array205.borrow(), self::ml_array::number_index(ml_reference_position205)) }).clone().unwrap_or_else(|| -1f64)) == 69f64)) {
                                                                                                                                            {
                                                                                                                                                let at_3 = at_2 + 1f64;
                                                                                                                                                {
                                                                                                                                                    let exponent_negative = false;
                                                                                                                                                    {
                                                                                                                                                        let exponent_sign = ({ let ml_reference_array206 = units.clone(); let ml_reference_position206 = at_3; self::ml_array::optional_at(&*ml_reference_array206.borrow(), self::ml_array::number_index(ml_reference_position206)) }).clone().unwrap_or_else(|| -1f64);
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_s16658 = if (exponent_sign == 43f64) || (exponent_sign == 45f64) {
                                                                                                                                                                {
                                                                                                                                                                    let exponent_negative_2 = exponent_sign == 45f64;
                                                                                                                                                                    {
                                                                                                                                                                        let at_4 = at_3 + 1f64;
                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin167::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin167Next(at_4, exponent_negative_2)
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin167::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin167Next(at_3, exponent_negative)
                                                                                                                                                            };
                                                                                                                                                            match source_ml_source2_s16658 {
                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin167::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin167Next(at_5, exponent_negative_3) => {
                                                                                                                                                                    {
                                                                                                                                                                        let first = at_5;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_s16659 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop168(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                            match source_ml_source2_s16659 {
                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Done(at_6, exponent_2) => {
                                                                                                                                                                                    if at_6 == first {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164Return(f64::NAN)
                                                                                                                                                                                    } else if exponent_negative_3 {
                                                                                                                                                                                            {
                                                                                                                                                                                                let exponent_3 = -exponent_2;
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164Next(at_6, exponent_3)
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164Next(at_6, exponent_2)
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
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164Next(at_2, exponent)
                                                                                                                                        };
                                                                                                                                        match source_ml_source2_s16647 {
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164Next(at_7, exponent_4) => {
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
                                                                                                                                                                                let source_ml_source2_s16660 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop165(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                match source_ml_source2_s16660 {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                        {
                                                                                                                                                                                            let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop166(units.clone(), integer_digits.clone(), mantissa_end_2, mantissa, start_6);
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
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164Return(source_ml_source1_temporary_result166) => {
                                                                                                                                                source_ml_source1_temporary_result166
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Return(source_ml_source1_temporary_result168) => {
                                                                                        source_ml_source1_temporary_result168
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Return(source_ml_source1_temporary_result169_2) => {
                    source_ml_source1_temporary_result169_2
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
pub fn convert_java_script_string_big_integer_units(
    units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s16661 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop170(units.clone(), index);
            match source_ml_source2_s16661 {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Done => {
                    {
                        let start = 0f64;
                        {
                            let end = { let ml_reference_array207 = units.clone(); ml_reference_array207.borrow().len() as f64 };
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop171(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop172(units.clone(), start_2, end);
                                    if start_2 == end_2 {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(0)]))
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = ({ let ml_reference_array208 = units.clone(); let ml_reference_position208 = start_2; self::ml_array::optional_at(&*ml_reference_array208.borrow(), self::ml_array::number_index(ml_reference_position208)) }).clone().unwrap_or_else(|| -1f64);
                                                    {
                                                        let source_ml_source2_s16664 = if (sign == 43f64) || (sign == 45f64) {
                                                            {
                                                                let negative_2 = sign == 45f64;
                                                                {
                                                                    let start_3 = start_2 + 1f64;
                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin173::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin173Next(start_3, negative_2, true)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin173::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin173Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s16664 {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin173::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin173Next(start_4, negative_3, signed_2) => {
                                                                if start_4 == end_2 {
                                                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = self::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s16671 = if (!signed_2 && ((({ let ml_reference_array209 = units.clone(); let ml_reference_position209 = start_4; self::ml_array::optional_at(&*ml_reference_array209.borrow(), self::ml_array::number_index(ml_reference_position209)) }).clone().unwrap_or_else(|| -1f64)) == 48f64)) && ((end_2 - start_4) >= 2f64) {
                                                                                    {
                                                                                        let marker = ({
                                                                                            let source_ml_source2_array_read_index16677 = start_4 + 1f64;
                                                                                            { let ml_reference_array210 = units.clone(); let ml_reference_position210 = source_ml_source2_array_read_index16677; self::ml_array::optional_at(&*ml_reference_array210.borrow(), self::ml_array::number_index(ml_reference_position210)) }
                                                                                        }).clone().unwrap_or_else(|| -1f64);
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
                                                                                                    let base_2 = if selected == 16f64 {
                                                                                                        self::ml::Big::from_i128(16)
                                                                                                    } else if selected == 8f64 {
                                                                                                            self::ml::Big::from_i128(8)
                                                                                                        } else {
                                                                                                            self::ml::Big::from_i128(2)
                                                                                                        };
                                                                                                    {
                                                                                                        let start_5 = start_4 + 2f64;
                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174Next(start_5, selected, base_2)
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174Next(start_4, radix, base.clone())
                                                                                            }
}
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s16671 {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174Next(start_6, radix_2, base_3) => {
                                                                                        if start_6 == end_2 {
                                                                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
                                                                                        } else {
                                                                                            {
                                                                                                let digits = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)]));
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let source_ml_source2_s16678 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop175(units.clone(), end_2, radix_2, base_3.clone(), digits, value, start_6);
                                                                                                        match source_ml_source2_s16678 {
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Done(value_2) => {
                                                                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![if negative_3 {
                                                                                                                    value_2.neg()
                                                                                                                } else {
                                                                                                                    value_2.clone()
                                                                                                                }]))
                                                                                                            }
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Return(source_ml_source1_temporary_result175) => {
                                                                                                                source_ml_source1_temporary_result175.clone()
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
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Return(source_ml_source1_temporary_result175_2) => {
                    source_ml_source1_temporary_result175_2.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_number_big_integer(
    value: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
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
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop176(value_2, integer, weight);
                            std::rc::Rc::new(std::cell::RefCell::new(vec![if negative {
                                integer_2.neg()
                            } else {
                                integer_2.clone()
                            }]))
                        }
                    }
                }
            }
        }
    } else {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
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
pub fn read_java_script_big_integer_conversion_error_class(
    form: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*form) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) {
        self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
    } else if (*form) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])) {
        self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
    } else if ((*form)
        == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])))
        || ((*form) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_message_pieces(
    form: &self::ml_string::Text,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    if (*form) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])) {
        std::rc::Rc::new(std::cell::RefCell::new(vec![
            self::ml_string::Text::from_units(vec![
                84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32,
            ]),
            self::ml_string::Text::from_units(vec![
                32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116,
                101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99,
                97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32,
                105, 110, 116, 101, 103, 101, 114,
            ]),
        ]))
    } else if (((*form) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
        || ((*form)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 100, 101, 102, 105, 110, 101, 100,
            ]))))
        || ((*form) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
    {
        std::rc::Rc::new(std::cell::RefCell::new(vec![
            self::ml_string::Text::from_units(vec![
                67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32,
            ]),
            self::ml_string::Text::from_units(vec![
                32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116,
            ]),
        ]))
    } else {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_width(value: self::ml::Big) -> f64 {
    {
        let rest = if value < (self::ml::Big::from_i128(0)) {
            value.neg()
        } else {
            value.clone()
        };
        {
            let width = 0f64;
            {
                let source_ml_source2_s16760 =
                    self::source_ml_source1_temporary_read_java_script_call_integer_width_loop191(
                        rest, width,
                    );
                match source_ml_source2_s16760 {
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Done(width_2) => {
                        width_2
                    }
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Return(source_ml_source1_temporary_result191) => {
                        source_ml_source1_temporary_result191
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_integer_protocol_operation_error(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105,
            110, 116, 101, 103, 101, 114,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                105, 110, 116, 101, 103, 101, 114,
            ])))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
            32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
            97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114,
            99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_call_integer_conversion(
    form: &self::ml_string::Text,
    value: &self::ml_string::Text,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let pieces = self::read_java_script_big_integer_conversion_message_pieces(form);
        self::JavaScriptCallPrimitiveResult::PrimitiveError(
            self::read_java_script_big_integer_conversion_error_class(form),
            ((({
                let source_ml_source2_array_read_index16762 = 0f64;
                {
                    let ml_reference_array211 = pieces.clone();
                    let ml_reference_position211 = source_ml_source2_array_read_index16762;
                    self::ml_array::optional_at(
                        &*ml_reference_array211.borrow(),
                        self::ml_array::number_index(ml_reference_position211),
                    )
                }
            })
            .clone()
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
            .concat(value))
            .concat(
                &({
                    let source_ml_source2_array_read_index16764 = 1f64;
                    {
                        let ml_reference_array212 = pieces.clone();
                        let ml_reference_position212 = source_ml_source2_array_read_index16764;
                        self::ml_array::optional_at(
                            &*ml_reference_array212.borrow(),
                            self::ml_array::number_index(ml_reference_position212),
                        )
                    }
                })
                .clone()
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
            ),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_text(
    units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> self::ml_string::Text {
    {
        let text = self::ml_string::Text::from_units(vec![]);
        {
            let index = 0f64;
            {
                self::source_ml_source1_temporary_read_java_script_call_integer_text_loop192(
                    units.clone(),
                    text,
                    index,
                )
            }
        }
    }
}
