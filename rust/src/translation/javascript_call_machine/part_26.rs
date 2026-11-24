// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Result;
use super::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin168;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Result;
use super::MlSource1TemporaryDependencyFunction1Loop192Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop178Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop179Result;
use super::accept_string_number_infinity;
use super::convert_string_number_rational;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_string_number_decimal_power;
use super::source_ml_source1_temporary_convert_java_script_number_big_integer_loop177;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop171;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop172;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop173;
use super::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop176;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop159;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop160;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop161;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop164;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop166;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop167;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop169;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop170;
use super::source_ml_source1_temporary_dependency_function1_loop192;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop178;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop179;
use super::source_ml_source1_temporary_power_java_script_big_integer_loop180;
use super::source_ml_source1_temporary_read_java_script_call_integer_text_loop193;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_string_number_units(
    units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s3395 =
                self::source_ml_source1_temporary_convert_java_script_string_number_units_loop159(
                    units.clone(),
                    index,
                );
            match source_ml_source2_s3395.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Done => {
                    {
                        let start = 0f64;
                        {
                            let end = { let ml_reference_array306 = units.clone(); let ml_reference_projection306 = ml_reference_array306.borrow().len() as f64; ml_reference_projection306 };
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop160(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop161(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        0f64
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = ({ let ml_reference_array307 = units.clone(); let ml_reference_position307 = start_2; let ml_reference_projection307 = self::ml_array::optional_at(&*ml_reference_array307.borrow(), self::ml_array::number_index(ml_reference_position307)); ml_reference_projection307 }).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s3398 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let start_3 = (start_2 + 1f64);
                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Next(start_3, negative_2, true)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s3398.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Next(start_4, negative_3, signed_2) => {
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
                                                                            let integer_digits = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)]));
                                                                            {
                                                                                let source_ml_source2_s3406 = if ((!signed_2 && ((({ let ml_reference_array308 = units.clone(); let ml_reference_position308 = start_4; let ml_reference_projection308 = self::ml_array::optional_at(&*ml_reference_array308.borrow(), self::ml_array::number_index(ml_reference_position308)); ml_reference_projection308 }).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = ({
                                                                                            let source_ml_source2_array_read_index3412 = (start_4 + 1f64);
                                                                                            { let ml_reference_array309 = units.clone(); let ml_reference_position309 = source_ml_source2_array_read_index3412; let ml_reference_projection309 = self::ml_array::optional_at(&*ml_reference_array309.borrow(), self::ml_array::number_index(ml_reference_position309)); ml_reference_projection309 }
                                                                                        }).clone().unwrap_or_else(|| (-1f64));
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
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let source_ml_source2_s3413 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop170(units.clone(), end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), start_5);
                                                                                                                            match source_ml_source2_s3413.clone() {
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Done(value_2, overflow_2) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163Return(if overflow_2 {
                                                                                                                                        f64::INFINITY
                                                                                                                                    } else {
                                                                                                                                        self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                    })
                                                                                                                                }
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Return(source_ml_source1_temporary_result170) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163Return(source_ml_source1_temporary_result170)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s3406.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163Next(start_6) => {
                                                                                        {
                                                                                            let at = start_6;
                                                                                            {
                                                                                                let fraction = 0f64;
                                                                                                {
                                                                                                    let digits = 0f64;
                                                                                                    {
                                                                                                        let significant = 0f64;
                                                                                                        {
                                                                                                            let source_ml_source2_s3414 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop164(units.clone(), end_2, at, fraction, digits, significant, false);
                                                                                                            match source_ml_source2_s3414.clone() {
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Done(at_2, fraction_2, digits_2, significant_2) => {
                                                                                                                    if (digits_2 == (0f64)) {
                                                                                                                        f64::NAN
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let decimal_end = at_2;
                                                                                                                            {
                                                                                                                                let exponent = 0f64;
                                                                                                                                {
                                                                                                                                    let exponent_limit = ({ let ml_reference_array310 = units.clone(); let ml_reference_projection310 = ml_reference_array310.borrow().len() as f64; ml_reference_projection310 } + 10000f64);
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_s3427 = if ((at_2 < end_2) && (((({ let ml_reference_array311 = units.clone(); let ml_reference_position311 = at_2; let ml_reference_projection311 = self::ml_array::optional_at(&*ml_reference_array311.borrow(), self::ml_array::number_index(ml_reference_position311)); ml_reference_projection311 }).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || ((({ let ml_reference_array312 = units.clone(); let ml_reference_position312 = at_2; let ml_reference_projection312 = self::ml_array::optional_at(&*ml_reference_array312.borrow(), self::ml_array::number_index(ml_reference_position312)); ml_reference_projection312 }).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                            {
                                                                                                                                                let at_3 = (at_2 + 1f64);
                                                                                                                                                {
                                                                                                                                                    let exponent_negative = false;
                                                                                                                                                    {
                                                                                                                                                        let exponent_sign = ({ let ml_reference_array313 = units.clone(); let ml_reference_position313 = at_3; let ml_reference_projection313 = self::ml_array::optional_at(&*ml_reference_array313.borrow(), self::ml_array::number_index(ml_reference_position313)); ml_reference_projection313 }).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_s3438 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                {
                                                                                                                                                                    let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                    {
                                                                                                                                                                        let at_4 = (at_3 + 1f64);
                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin168::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin168Next(at_4, exponent_negative_2)
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin168::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin168Next(at_3, exponent_negative)
                                                                                                                                                            };
                                                                                                                                                            match source_ml_source2_s3438.clone() {
                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin168::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin168Next(at_5, exponent_negative_3) => {
                                                                                                                                                                    {
                                                                                                                                                                        let first = at_5;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_s3439 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop169(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                            match source_ml_source2_s3439.clone() {
                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Done(at_6, exponent_2) => {
                                                                                                                                                                                    if (at_6 == first) {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165Return(f64::NAN)
                                                                                                                                                                                    } else {
                                                                                                                                                                                        if exponent_negative_3 {
                                                                                                                                                                                            {
                                                                                                                                                                                                let exponent_3 = (-exponent_2);
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165Next(at_6, exponent_3)
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165Next(at_6, exponent_2)
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
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165Next(at_2, exponent)
                                                                                                                                        };
                                                                                                                                        match source_ml_source2_s3427.clone() {
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165Next(at_7, exponent_4) => {
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
                                                                                                                                                                                let source_ml_source2_s3440 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop166(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                match source_ml_source2_s3440.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                        {
                                                                                                                                                                                            let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop167(units.clone(), integer_digits.clone(), mantissa_end_2, mantissa.clone(), start_6);
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
                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165Return(source_ml_source1_temporary_result167) => {
                                                                                                                                                source_ml_source1_temporary_result167
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163Return(source_ml_source1_temporary_result169) => {
                                                                                        source_ml_source1_temporary_result169
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Return(source_ml_source1_temporary_result170_2) => {
                    source_ml_source1_temporary_result170_2
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_string_big_integer_units(
    units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s3441 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop171(units.clone(), index);
            match source_ml_source2_s3441.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Done => {
                    {
                        let start = 0f64;
                        {
                            let end = { let ml_reference_array314 = units.clone(); let ml_reference_projection314 = ml_reference_array314.borrow().len() as f64; ml_reference_projection314 };
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop172(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop173(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(0)]))
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = ({ let ml_reference_array315 = units.clone(); let ml_reference_position315 = start_2; let ml_reference_projection315 = self::ml_array::optional_at(&*ml_reference_array315.borrow(), self::ml_array::number_index(ml_reference_position315)); ml_reference_projection315 }).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s3444 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let start_3 = (start_2 + 1f64);
                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174Next(start_3, negative_2, true)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s3444.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174Next(start_4, negative_3, signed_2) => {
                                                                if (start_4 == end_2) {
                                                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = self::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s3451 = if ((!signed_2 && ((({ let ml_reference_array316 = units.clone(); let ml_reference_position316 = start_4; let ml_reference_projection316 = self::ml_array::optional_at(&*ml_reference_array316.borrow(), self::ml_array::number_index(ml_reference_position316)); ml_reference_projection316 }).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = ({
                                                                                            let source_ml_source2_array_read_index3457 = (start_4 + 1f64);
                                                                                            { let ml_reference_array317 = units.clone(); let ml_reference_position317 = source_ml_source2_array_read_index3457; let ml_reference_projection317 = self::ml_array::optional_at(&*ml_reference_array317.borrow(), self::ml_array::number_index(ml_reference_position317)); ml_reference_projection317 }
                                                                                        }).clone().unwrap_or_else(|| (-1f64));
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
                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175Next(start_5, selected, base_2.clone())
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175Next(start_4, radix, base.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s3451.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175Next(start_6, radix_2, base_3) => {
                                                                                        if (start_6 == end_2) {
                                                                                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
                                                                                        } else {
                                                                                            {
                                                                                                let digits = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)]));
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let source_ml_source2_s3458 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop176(units.clone(), end_2, radix_2, base_3.clone(), digits.clone(), value.clone(), start_6);
                                                                                                        match source_ml_source2_s3458.clone() {
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Done(value_2) => {
                                                                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![if negative_3 {
                                                                                                                    value_2.neg()
                                                                                                                } else {
                                                                                                                    value_2.clone()
                                                                                                                }]))
                                                                                                            }
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Return(source_ml_source1_temporary_result176) => {
                                                                                                                source_ml_source1_temporary_result176.clone()
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
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Return(source_ml_source1_temporary_result176_2) => {
                    source_ml_source1_temporary_result176_2.clone()
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
    if !(self::ml_math::is_integer(value)) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
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
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop177(value_2, integer.clone(), weight.clone());
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
                        let lower = ({
                            let source_ml_source2_array_read_value3459 =
                                self::convert_java_script_number_big_integer(floor);
                            {
                                let source_ml_source2_array_read_index3460 = 0f64;
                                {
                                    let ml_reference_array318 =
                                        source_ml_source2_array_read_value3459.clone();
                                    let ml_reference_position318 =
                                        source_ml_source2_array_read_index3460;
                                    let ml_reference_projection318 = self::ml_array::optional_at(
                                        &*ml_reference_array318.borrow(),
                                        self::ml_array::number_index(ml_reference_position318),
                                    );
                                    ml_reference_projection318
                                }
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::ml::Big::from_i128(0));
                        if (lower < integer) {
                            (-1f64)
                        } else {
                            if (lower > integer) {
                                1f64
                            } else {
                                if (value == floor) { 0f64 } else { 1f64 }
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
pub fn compare_java_script_string_big_integer_units(
    units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    integer: self::ml::Big,
) -> f64 {
    {
        let result = self::convert_java_script_string_big_integer_units(units.clone());
        if (({
            let ml_reference_array319 = result.clone();
            let ml_reference_projection319 = ml_reference_array319.borrow().len() as f64;
            ml_reference_projection319
        }) != (1f64))
        {
            2f64
        } else {
            {
                let value = ({
                    let source_ml_source2_array_read_index3462 = 0f64;
                    {
                        let ml_reference_array320 = result.clone();
                        let ml_reference_position320 = source_ml_source2_array_read_index3462;
                        let ml_reference_projection320 = self::ml_array::optional_at(
                            &*ml_reference_array320.borrow(),
                            self::ml_array::number_index(ml_reference_position320),
                        );
                        ml_reference_projection320
                    }
                })
                .clone()
                .unwrap_or_else(|| self::ml::Big::from_i128(0));
                if (value < integer) {
                    (-1f64)
                } else {
                    if (value > integer) { 1f64 } else { 0f64 }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn divide_java_script_big_integer(
    left: self::ml::Big,
    right: self::ml::Big,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    if (right == (self::ml::Big::from_i128(0))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
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
                                let source_ml_source2_s3463 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop178(remaining.clone(), shifted.clone(), weight.clone());
                                match source_ml_source2_s3463.clone() {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop178Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop178Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s3464 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop179(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient.clone());
                                            match source_ml_source2_s3464.clone() {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop179Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop179Done(remaining_2, quotient_2) => {
                                                    std::rc::Rc::new(std::cell::RefCell::new(vec![if (((left < (self::ml::Big::from_i128(0)))) != ((right < (self::ml::Big::from_i128(0))))) {
                                                        quotient_2.neg()
                                                    } else {
                                                        quotient_2.clone()
                                                    }, if (left < (self::ml::Big::from_i128(0))) {
                                                        remaining_2.neg()
                                                    } else {
                                                        remaining_2.clone()
                                                    }]))
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
) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    if (exponent < (self::ml::Big::from_i128(0))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()))
    } else {
        {
            let result = self::ml::Big::from_i128(1);
            {
                let result_2 =
                    self::source_ml_source1_temporary_power_java_script_big_integer_loop180(
                        result.clone(),
                        base.clone(),
                        exponent.clone(),
                    );
                std::rc::Rc::new(std::cell::RefCell::new(vec![result_2.clone()]))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_error_class(
    form: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*form) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
    } else {
        if ((*form) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
            self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
        } else {
            if (((*form)
                == (self::ml_string::Text::from_units(vec![
                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                ])))
                || ((*form) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108]))))
            {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_message_pieces(
    form: &self::ml_string::Text,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    if ((*form) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
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
    } else {
        if ((((*form) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
            || ((*form)
                == (self::ml_string::Text::from_units(vec![
                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                ]))))
            || ((*form) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108]))))
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function1(value: self::ml::Big) -> f64 {
    {
        let rest = if (value < (self::ml::Big::from_i128(0))) {
            value.neg()
        } else {
            value.clone()
        };
        {
            let width = 0f64;
            {
                let source_ml_source2_s3540 =
                    self::source_ml_source1_temporary_dependency_function1_loop192(
                        rest.clone(),
                        width,
                    );
                match source_ml_source2_s3540.clone() {
                    self::MlSource1TemporaryDependencyFunction1Loop192Result::MlSource1TemporaryDependencyFunction1Loop192Done(width_2) => {
                        width_2
                    }
                    self::MlSource1TemporaryDependencyFunction1Loop192Result::MlSource1TemporaryDependencyFunction1Loop192Return(source_ml_source1_temporary_result192) => {
                        source_ml_source1_temporary_result192
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
    if (((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105,
            110, 116, 101, 103, 101, 114,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                105, 110, 116, 101, 103, 101, 114,
            ]))))
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
                let source_ml_source2_array_read_index3542 = 0f64;
                {
                    let ml_reference_array321 = pieces.clone();
                    let ml_reference_position321 = source_ml_source2_array_read_index3542;
                    let ml_reference_projection321 = self::ml_array::optional_at(
                        &*ml_reference_array321.borrow(),
                        self::ml_array::number_index(ml_reference_position321),
                    );
                    ml_reference_projection321
                }
            })
            .clone()
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
            .concat(value))
            .concat(
                &({
                    let source_ml_source2_array_read_index3544 = 1f64;
                    {
                        let ml_reference_array322 = pieces.clone();
                        let ml_reference_position322 = source_ml_source2_array_read_index3544;
                        let ml_reference_projection322 = self::ml_array::optional_at(
                            &*ml_reference_array322.borrow(),
                            self::ml_array::number_index(ml_reference_position322),
                        );
                        ml_reference_projection322
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
                let text_2 =
                    self::source_ml_source1_temporary_read_java_script_call_integer_text_loop193(
                        units.clone(),
                        text.clone(),
                        index,
                    );
                text_2.clone()
            }
        }
    }
}
