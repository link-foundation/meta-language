// Generated from complete js/src/translation/big-integer-conversion-rules.js. Do not edit by hand.
use super::accept_java_script_whitespace_unit;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_java_script_numeric_digit_unit;

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
                                let source_ml_source2_s7 = if (194f64..=223f64).contains(&scalar) {
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
                                match source_ml_source2_s7 {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s8 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s8 {
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
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop6(
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
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop7(
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Done(self::ml::Big),
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Return(Vec<self::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop10(
    units: &[f64],
    mut end: f64,
    mut radix: f64,
    mut base: self::ml::Big,
    mut digits: Vec<self::ml::Big>,
    mut value: self::ml::Big,
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Result {
    loop {
        return if index < end {
            {
                let digit = self::read_java_script_numeric_digit_unit(
                    (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64),
                );
                if (digit < 0f64) || (digit >= radix) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Return(Vec::<self::ml::Big>::new())
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
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Done(value.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9Next(f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Done,
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Return(Vec<self::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop5(
    units: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let unit =
                    (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64)
                {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Return(Vec::<self::ml::Big>::new())
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
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_number_big_integer_loop11(
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
