pub fn source_ml_source1_temporary_execute_java_script_call_text_conversion_loop3(mut text: self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (text.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand657 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand658 = index;
                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand664 = {
                                        let source_ml_source2_string_range_operand659 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand660 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand661 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position662 = source_ml_source2_string_range_operand660;
                                                    {
                                                        let source_ml_source2_string_range_position663 = source_ml_source2_string_range_operand661;
                                                        source_ml_source2_string_range_operand659.slice(source_ml_source2_string_range_position662, source_ml_source2_string_range_position663)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand665 = 0f64;
                                        source_ml_source2_string_unit_operand664.code_point_at(source_ml_source2_string_unit_operand665)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand657.clone(), source_ml_source2_string_unit_operand658)
                    }
                }], units); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (units, index) = (units_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2 {
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Next(self::ml_string::Text),
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin2Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Done(f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop7(mut bytes: Vec<f64>, mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(&bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Return(vec![(-1f64)])
                } else {
                    {
                        let scalar_2 = (((scalar * 64f64) + next) - 128f64);
                        {
                            let index_2 = (index + 1f64);
                            {
                                let remaining_2 = (remaining - 1f64);
                                {
                                    (index, scalar, remaining) = (index_2, scalar_2, remaining_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6 {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(f64, f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop5(mut bytes: Vec<f64>, mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let scalar = (self::ml_array::optional_at(&bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                {
                    let index_2 = (index + 1f64);
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source2_s666 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_2, remaining_2, minimum_2)
                                            }
                                        }
                                    }
                                } else {
                                    if ((scalar >= (224f64)) && (scalar <= (239f64))) {
                                        {
                                            let scalar_3 = (scalar - 224f64);
                                            {
                                                let remaining_3 = 2f64;
                                                {
                                                    let minimum_3 = 2048f64;
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_3, remaining_3, minimum_3)
                                                }
                                            }
                                        }
                                    } else {
                                        if ((scalar >= (240f64)) && (scalar <= (244f64))) {
                                            {
                                                let scalar_4 = (scalar - 240f64);
                                                {
                                                    let remaining_4 = 3f64;
                                                    {
                                                        let minimum_4 = 65536f64;
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Return(vec![(-1f64)])
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source2_s666.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s667 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop7(bytes.clone(), index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s667.clone() {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(vec![(-1f64)])
                                                    } else {
                                                        if (scalar_6 <= (65535f64)) {
                                                            {
                                                                let units_2 = { let (ml_append_right, ml_append_left) = (vec![scalar_6], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    (units, index) = (units_2, index_3);
                                                                    continue;
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let units_3 = { let (ml_append_right, ml_append_left) = (vec![(55296f64 + (((scalar_6 - 65536f64) / 1024f64)).floor())], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let units_4 = { let (ml_append_right, ml_append_left) = (vec![(56320f64 + ((scalar_6 - 65536f64) % 1024f64))], units_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        (units, index) = (units_4, index_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop7Return(source_ml_source1_temporary_result7) => {
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(source_ml_source1_temporary_result7)
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin6Return(source_ml_source1_temporary_result7_2) => {
                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(source_ml_source1_temporary_result7_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done,
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(mut bytes: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(vec![(-1f64)])
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done
        };
    }
}

pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop8(mut exponent: f64, mut value: self::ml::Big, mut base: self::ml::Big) -> self::ml::Big {
    loop {
        return if (exponent > (0f64)) {
            {
                let value_3 = if (((exponent % 2f64)) == (1f64)) {
                    {
                        let value_2 = value.mul(&base);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = ((exponent / 2f64)).floor();
                    if (exponent_2 > (0f64)) {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) = (exponent_2, value_3.clone(), base_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (exponent, value) = (exponent_2, value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop9(mut value: self::ml::Big, mut result: f64, mut scale: f64) -> f64 {
    loop {
        return if (value > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = (result + scale);
                        result_2
                    }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let scale_2 = (scale * 2f64);
                        {
                            (value, result, scale) = (value_2.clone(), result_3, scale_2);
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

pub fn source_ml_source1_temporary_read_string_number_binary_power_loop11(mut exponent: f64, mut result: f64) -> f64 {
    loop {
        return if (exponent < (0f64)) {
            {
                let result_2 = (result / 2f64);
                {
                    let exponent_2 = (exponent + 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop10Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop10Done(f64, f64),
}

pub fn source_ml_source1_temporary_read_string_number_binary_power_loop10(mut exponent: f64, mut result: f64) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Result {
    loop {
        return if (exponent > (0f64)) {
            {
                let result_2 = (result * 2f64);
                {
                    let exponent_2 = (exponent - 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop15Result {
    MlSource1TemporaryConvertStringNumberRationalLoop15Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop15(mut quotient: self::ml::Big, mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop15Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let scale_2 = self::ml::divide(&scale, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remainder >= scale_2) {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (quotient_2.clone(), remainder_2.clone(), scale_2.clone(), weight_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (scale, weight) = (scale_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop14Result {
    MlSource1TemporaryConvertStringNumberRationalLoop14Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop14(mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop14Result {
    loop {
        return if (scale <= remainder) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop14Result::MlSource1TemporaryConvertStringNumberRationalLoop14Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin13 {
    MlSource1TemporaryConvertStringNumberRationalJoin13Next(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop16(mut numerator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < shift) {
            {
                let numerator_2 = numerator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (numerator, index) = (numerator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            numerator.clone()
        };
    }
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop17(mut denominator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < ((-shift))) {
            {
                let denominator_2 = denominator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (denominator, index) = (denominator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            denominator.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin12 {
    MlSource1TemporaryConvertStringNumberRationalJoin12Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin12Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop18Result {
    MlSource1TemporaryConvertStringNumberRationalLoop18Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop18Return(f64),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop18(mut numerator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop18Result {
    loop {
        return if (numerator >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent + 1f64);
                    if (exponent_2 >= (1024f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop18Result::MlSource1TemporaryConvertStringNumberRationalLoop18Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop18Result::MlSource1TemporaryConvertStringNumberRationalLoop18Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop19Result {
    MlSource1TemporaryConvertStringNumberRationalLoop19Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop19Return(f64),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop19(mut denominator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop19Result {
    loop {
        return if (scale < denominator) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent - 1f64);
                    if (exponent_2 < ((-1075f64))) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop19Result::MlSource1TemporaryConvertStringNumberRationalLoop19Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop19Result::MlSource1TemporaryConvertStringNumberRationalLoop19Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop20Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop20Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop20Return(bool),
}

pub fn source_ml_source1_temporary_accept_string_number_infinity_loop20(mut units: Vec<f64>, mut start: f64, mut expected: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result {
    loop {
        return if (index < ((expected.len() as f64))) {
            if (((self::ml_array::optional_at(&units, self::ml_array::number_index((start + index)))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(&expected, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-2f64)))) {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result::MlSource1TemporaryAcceptStringNumberInfinityLoop20Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result::MlSource1TemporaryAcceptStringNumberInfinityLoop20Done
        };
    }
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop22(mut units: Vec<f64>, mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((start < end) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(&units, self::ml_array::number_index(start))).clone().unwrap_or_else(|| (-1f64)))) {
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

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop23(mut units: Vec<f64>, mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((end > start) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(&units, self::ml_array::number_index((end - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
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

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop29(mut units: Vec<f64>, mut integer_digits: Vec<self::ml::Big>, mut mantissa_end: f64, mut mantissa: self::ml::Big, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < mantissa_end) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (unit != (46f64)) {
                    {
                        let mantissa_2 = (mantissa.mul(&self::ml::Big::from_i128(10))).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index((unit - 48f64)))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (mantissa, index) = (mantissa_2.clone(), index_2);
                                continue;
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
        } else {
            mantissa.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop28(mut units: Vec<f64>, mut start: f64, mut exponent: f64, mut mantissa_end: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result {
    loop {
        return if (mantissa_end > start) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index((mantissa_end - 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if (unit == (46f64)) {
                    {
                        let mantissa_end_2 = (mantissa_end - 1f64);
                        {
                            mantissa_end = mantissa_end_2;
                            continue;
                        }
                    }
                } else {
                    if (unit != (48f64)) {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(exponent, mantissa_end)
                    } else {
                        {
                            let mantissa_end_3 = (mantissa_end - 1f64);
                            {
                                let exponent_2 = (exponent + 1f64);
                                {
                                    (exponent, mantissa_end) = (exponent_2, mantissa_end_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Done(f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop31(mut units: Vec<f64>, mut end: f64, mut at: f64, mut exponent: f64, mut exponent_limit: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result {
    loop {
        return if (at < end) {
            {
                let digit = ((self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)) - 48f64);
                if ((digit < (0f64)) || (digit > (9f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Done(at, exponent)
                } else {
                    {
                        let exponent_3 = if (exponent < exponent_limit) {
                            {
                                let exponent_2 = self::ml_math::min(exponent_limit, ((exponent * 10f64) + digit));
                                exponent_2
                            }
                        } else {
                            exponent
                        };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (at, exponent) = (at_2, exponent_3);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Done(f64, f64, f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop26(mut units: Vec<f64>, mut end: f64, mut at: f64, mut fraction: f64, mut digits: f64, mut significant: f64, mut dot: bool) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result {
    loop {
        return if (at < end) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
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
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop32(mut units: Vec<f64>, mut end: f64, mut integer_digits: Vec<self::ml::Big>, mut radix: f64, mut value: self::ml::Big, mut overflow: bool, mut limit: self::ml::Big, mut base: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit((self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Return(f64::NAN)
                } else {
                    if !overflow {
                        {
                            let value_2 = (value.mul(&base)).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index(digit))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop21(mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop33(mut units: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result {
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Done,
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Return(bool),
}
