// Generated from complete js/src/translation/big-integer-conversion-rules.js. Do not edit by hand.
use super::accept_java_script_whitespace_unit;
use super::ml;
use super::ml_array;
use super::ml_math;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_result_expression(
    target: &str,
    form: &str,
    value: &str,
    utf16: bool,
) -> String {
    if form == "boolean" {
        if target == "JavaScript" {
            format!(
                "ml_big_integer_conversion.convertBoolean({})",
                value.to_owned()
            )
        } else if target == "Rust" {
            format!(
                "crate::ml_big_integer_conversion::convert_java_script_boolean_big_integer({})",
                value.to_owned()
            )
        } else if (target == "Lean") || (target == "Rocq") {
            format!(
                "(MLBigIntegerConversion.convertBoolean {})",
                value.to_owned()
            )
        } else {
            String::new()
        }
    } else if (form != "string") && (form != "number") {
        String::new()
    } else if target == "JavaScript" {
        if form == "number" {
            format!(
                "ml_big_integer_conversion.convertNumber({})",
                value.to_owned()
            )
        } else {
            format!(
                "((ml_text)=>ml_big_integer_conversion.convertUnits(Array.from({{length:ml_text.length}}, (_,index)=>ml_text.charCodeAt(index))))({})",
                value.to_owned()
            )
        }
    } else if target == "Rust" {
        if form == "number" {
            format!(
                "crate::ml_big_integer_conversion::convert_java_script_number_big_integer({})",
                value.to_owned()
            )
        } else {
            {
                let units = if utf16 {
                    format!("({}).0.iter().copied()", value.to_owned())
                } else {
                    format!("({}).encode_utf16()", value.to_owned())
                };
                format!(
                    "crate::ml_big_integer_conversion::convert_java_script_string_big_integer_units(&{units}.map(f64::from).collect::<Vec<f64>>())"
                )
            }
        }
    } else if target == "Lean" {
        if form == "number" {
            format!(
                "(MLBigIntegerConversion.convertNumber {})",
                value.to_owned()
            )
        } else {
            format!(
                "(MLBigIntegerConversion.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] {}))",
                value.to_owned()
            )
        }
    } else if target == "Rocq" {
        if form == "number" {
            format!(
                "(MLBigIntegerConversion.convertNumber {})",
                value.to_owned()
            )
        } else {
            format!(
                "(MLBigIntegerConversion.convertUtf8 ((fix ml_integer_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_integer_bytes rest) end) {}))",
                value.to_owned()
            )
        }
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done(f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(
    bytes: &[f64],
    mut index: f64,
    mut scalar: f64,
    mut remaining: f64,
) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if remaining > 0f64 {
            {
                let next =
                    (self::ml_array::optional_at(bytes, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if (next < 128f64) || (next > 191f64) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(vec![-1f64])
                } else {
                    {
                        let scalar_2 = ((scalar * 64f64) + next) - 128f64;
                        {
                            let index_2 = index + 1f64;
                            {
                                let remaining_2 = remaining - 1f64;
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3 {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(f64, f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop2(
    bytes: &[f64],
    mut units: Vec<f64>,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
    loop {
        return if index < (bytes.len() as f64) {
            {
                let scalar =
                    (self::ml_array::optional_at(bytes, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                {
                    let index_2 = index + 1f64;
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source2_s36 = if (194f64..=223f64).contains(&scalar) {
                                    {
                                        let scalar_2 = scalar - 192f64;
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_2, remaining_2, minimum_2)
                                            }
                                        }
                                    }
                                } else if (224f64..=239f64).contains(&scalar) {
                                    {
                                        let scalar_3 = scalar - 224f64;
                                        {
                                            let remaining_3 = 2f64;
                                            {
                                                let minimum_3 = 2048f64;
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_3, remaining_3, minimum_3)
                                            }
                                        }
                                    }
                                } else if (240f64..=244f64).contains(&scalar) {
                                    {
                                        let scalar_4 = scalar - 240f64;
                                        {
                                            let remaining_4 = 3f64;
                                            {
                                                let minimum_4 = 65_536f64;
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_4, remaining_4, minimum_4)
                                            }
                                        }
                                    }
                                } else if scalar >= 128f64 {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Return(vec![-1f64])
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar, remaining, minimum)
                                };
                                match source_ml_source2_s36 {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s37 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s37 {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done(index_3, scalar_6) => {
                                                    if ((scalar_6 < minimum_5) || (scalar_6 > 1_114_111f64)) || ((55_296f64..=57_343f64).contains(&scalar_6)) {
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(vec![-1f64])
                                                    } else {
                                                        if scalar_6 <= 65_535f64 {
                                                            {
                                                                let units_2 = { let (ml_append_right, ml_append_left) = (vec![scalar_6], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    (units, index) = (units_2, index_3);
                                                                    continue;
                                                                }
                                                            }
                                                        }
{
                                                            {
                                                                let units_3 = { let (ml_append_right, ml_append_left) = (vec![(55_296f64 + ((scalar_6 - 65_536f64) / 1024f64).floor())], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let units_4 = { let (ml_append_right, ml_append_left) = (vec![(56_320f64 + ((scalar_6 - 65_536f64) % 1024f64))], units_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        (units, index) = (units_4, index_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(source_ml_source1_temporary_result4) => {
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_source1_temporary_result4)
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Return(source_ml_source1_temporary_result4_2) => {
                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_source1_temporary_result4_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Done,
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop1(
    bytes: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    loop {
        return if index < (bytes.len() as f64) {
            {
                let unit =
                    (self::ml_array::optional_at(bytes, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(unit)) || (unit < 0f64)) || (unit > 255f64) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Return(vec![-1f64])
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop5(
    mut exponent: f64,
    mut value: self::ml::Big,
    mut base: self::ml::Big,
) -> self::ml::Big {
    loop {
        if exponent > 0f64 {
            {
                let value_3 = if (exponent % 2f64) == 1f64 {
                    { value.mul(&base) }
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
        return { value.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop6(
    mut value: self::ml::Big,
    mut result: f64,
    mut scale: f64,
) -> f64 {
    loop {
        if value > (self::ml::Big::from_i128(0)) {
            {
                let result_3 = if (self::ml::divide(
                    &value,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    true,
                )) == (self::ml::Big::from_i128(1))
                {
                    { result + scale }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(
                        &value,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
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
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop8(
    mut exponent: f64,
    mut result: f64,
) -> f64 {
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
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop7Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop7(
    mut exponent: f64,
    mut result: f64,
) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result {
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
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop12Result {
    MlSource1TemporaryConvertStringNumberRationalLoop12Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop12(
    mut quotient: self::ml::Big,
    mut remainder: self::ml::Big,
    mut scale: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop12Result {
    loop {
        if weight > (self::ml::Big::from_i128(1)) {
            {
                let scale_2 = self::ml::divide(
                    &scale,
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
                    if remainder >= scale_2 {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (
                                        quotient_2,
                                        remainder_2,
                                        scale_2.clone(),
                                        weight_2.clone(),
                                    );
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
            self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop11Result {
    MlSource1TemporaryConvertStringNumberRationalLoop11Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop11(
    mut remainder: self::ml::Big,
    mut scale: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop11Result {
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
            self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin10 {
    MlSource1TemporaryConvertStringNumberRationalJoin10Next(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop13(
    mut numerator: self::ml::Big,
    mut shift: f64,
    mut index: f64,
) -> self::ml::Big {
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
        return { numerator.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop14(
    mut denominator: self::ml::Big,
    mut shift: f64,
    mut index: f64,
) -> self::ml::Big {
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
        return { denominator.clone() };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin9 {
    MlSource1TemporaryConvertStringNumberRationalJoin9Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin9Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop15Result {
    MlSource1TemporaryConvertStringNumberRationalLoop15Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop15Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop15(
    mut numerator: self::ml::Big,
    mut exponent: f64,
    mut scale: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop15Result {
    loop {
        return if numerator >= (scale.mul(&self::ml::Big::from_i128(2))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent + 1f64;
                    if exponent_2 >= 1024f64 {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop16Result {
    MlSource1TemporaryConvertStringNumberRationalLoop16Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop16Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop16(
    mut denominator: self::ml::Big,
    mut exponent: f64,
    mut scale: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop16Result {
    loop {
        return if scale < denominator {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent - 1f64;
                    if exponent_2 < -1075f64 {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop17Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop17Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop17Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_string_number_infinity_loop17(
    units: &[f64],
    mut start: f64,
    expected: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result {
    loop {
        return if index < (expected.len() as f64) {
            {
                let ml_library_inequality_1 = ((self::ml_array::optional_at(
                    units,
                    self::ml_array::number_index(start + index),
                ))
                .unwrap_or_else(|| -1f64))
                    != ((self::ml_array::optional_at(
                        expected,
                        self::ml_array::number_index(index),
                    ))
                    .unwrap_or_else(|| -2f64));
                if ml_library_inequality_1 {
                    self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Return(false)
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
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop19(
    units: &[f64],
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        if (start < end)
            && self::accept_java_script_whitespace_unit(
                (self::ml_array::optional_at(units, self::ml_array::number_index(start)))
                    .unwrap_or_else(|| -1f64),
            )
        {
            {
                let start_2 = start + 1f64;
                {
                    start = start_2;
                    continue;
                }
            }
        }
        return { start };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop20(
    units: &[f64],
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        if (end > start)
            && self::accept_java_script_whitespace_unit(
                (self::ml_array::optional_at(units, self::ml_array::number_index(end - 1f64)))
                    .unwrap_or_else(|| -1f64),
            )
        {
            {
                let end_2 = end - 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        }
        return { end };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop26(
    units: &[f64],
    mut integer_digits: Vec<self::ml::Big>,
    mut mantissa_end: f64,
    mut mantissa: self::ml::Big,
    mut index: f64,
) -> self::ml::Big {
    loop {
        if index < mantissa_end {
            {
                let unit =
                    (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if unit != 46f64 {
                    {
                        let mantissa_2 = (mantissa.mul(&self::ml::Big::from_i128(10))).add(
                            &(self::ml_array::optional_at(
                                &integer_digits,
                                self::ml_array::number_index(unit - 48f64),
                            ))
                            .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                        );
                        {
                            let index_2 = index + 1f64;
                            {
                                (mantissa, index) = (mantissa_2, index_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let index_3 = index + 1f64;
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { mantissa.clone() };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop25(
    units: &[f64],
    mut start: f64,
    mut exponent: f64,
    mut mantissa_end: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result {
    loop {
        return if mantissa_end > start {
            {
                let unit = (self::ml_array::optional_at(
                    units,
                    self::ml_array::number_index(mantissa_end - 1f64),
                ))
                .unwrap_or_else(|| -1f64);
                if unit == 46f64 {
                    {
                        let mantissa_end_2 = mantissa_end - 1f64;
                        {
                            mantissa_end = mantissa_end_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let ml_library_inequality_1 = unit != 48f64;
                        if ml_library_inequality_1 {
                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(exponent, mantissa_end)
                        } else {
                            {
                                let mantissa_end_3 = mantissa_end - 1f64;
                                {
                                    let exponent_2 = exponent + 1f64;
                                    {
                                        (exponent, mantissa_end) = (exponent_2, mantissa_end_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(f64, f64),
}
