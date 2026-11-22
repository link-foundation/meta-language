// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::ml_array;
use super::ml_math;

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_comparison(
    target: &str,
    form: &str,
    left: &str,
    right: &str,
    utf16: bool,
) -> String {
    if (form == "number-compare") {
        if (target == "JavaScript") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("ml_big_integer_conversion.compareNumber(")),
                            left.to_owned()
                        )),
                        (String::from(","))
                    )),
                    right.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            if (target == "Rust") {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (String::from(
                                    "crate::ml_big_integer_conversion::compare_java_script_number_big_integer("
                                )),
                                left.to_owned()
                            )),
                            (String::from(","))
                        )),
                        right.to_owned()
                    )),
                    (String::from(")"))
                )
            } else {
                if ((target == "Lean") || (target == "Rocq")) {
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (String::from("(MLBigIntegerConversion.compareNumber ")),
                                    left.to_owned()
                                )),
                                (String::from(" "))
                            )),
                            right.to_owned()
                        )),
                        (String::from(")"))
                    )
                } else {
                    String::from("")
                }
            }
        }
    } else {
        if (form != "string-compare") {
            String::from("")
        } else {
            if (target == "JavaScript") {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (String::from(
                                    "((ml_text,ml_integer)=>ml_big_integer_conversion.compareUnits(Array.from({length:ml_text.length},(_,index)=>ml_text.charCodeAt(index)),ml_integer))("
                                )),
                                left.to_owned()
                            )),
                            (String::from(","))
                        )),
                        right.to_owned()
                    )),
                    (String::from(")"))
                )
            } else {
                if (target == "Rust") {
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (String::from(
                                        "crate::ml_big_integer_conversion::compare_java_script_string_big_integer_units(&"
                                    )),
                                    (if utf16 {
                                        format!(
                                            "{}{}",
                                            (format!("{}{}", (String::from("(")), left.to_owned())),
                                            (String::from(").0.iter().copied()"))
                                        )
                                    } else {
                                        format!(
                                            "{}{}",
                                            (format!("{}{}", (String::from("(")), left.to_owned())),
                                            (String::from(").encode_utf16()"))
                                        )
                                    })
                                )),
                                (String::from(".map(f64::from).collect::<Vec<f64>>(),"))
                            )),
                            right.to_owned()
                        )),
                        (String::from(")"))
                    )
                } else {
                    if (target == "Lean") {
                        format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (String::from(
                                            "(MLBigIntegerConversion.compareUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] "
                                        )),
                                        left.to_owned()
                                    )),
                                    (String::from(") "))
                                )),
                                right.to_owned()
                            )),
                            (String::from(")"))
                        )
                    } else {
                        if (target == "Rocq") {
                            format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (format!(
                                            "{}{}",
                                            (String::from(
                                                "(MLBigIntegerConversion.compareUtf8 ((fix ml_compare_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_compare_bytes rest) end) "
                                            )),
                                            left.to_owned()
                                        )),
                                        (String::from(") "))
                                    )),
                                    right.to_owned()
                                )),
                                (String::from(")"))
                            )
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_number_power(target: &str, base: &str, exponent: &str) -> String {
    if (target == "JavaScript") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("ml_big_integer_conversion.powerNumber(")),
                        base.to_owned()
                    )),
                    (String::from(","))
                )),
                exponent.to_owned()
            )),
            (String::from(")"))
        )
    } else {
        if (target == "Rust") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from(
                                "crate::ml_big_integer_conversion::power_java_script_number("
                            )),
                            base.to_owned()
                        )),
                        (String::from(","))
                    )),
                    exponent.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            if ((target == "Lean") || (target == "Rocq")) {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (String::from("(MLBigIntegerConversion.powerNumber ")),
                                base.to_owned()
                            )),
                            (String::from(" "))
                        )),
                        exponent.to_owned()
                    )),
                    (String::from(")"))
                )
            } else {
                String::from("")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_result_expression(
    target: &str,
    form: &str,
    value: &str,
    utf16: bool,
) -> String {
    if (form == "boolean") {
        if (target == "JavaScript") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (String::from("ml_big_integer_conversion.convertBoolean(")),
                    value.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            if (target == "Rust") {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from(
                            "crate::ml_big_integer_conversion::convert_java_script_boolean_big_integer("
                        )),
                        value.to_owned()
                    )),
                    (String::from(")"))
                )
            } else {
                if ((target == "Lean") || (target == "Rocq")) {
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("(MLBigIntegerConversion.convertBoolean ")),
                            value.to_owned()
                        )),
                        (String::from(")"))
                    )
                } else {
                    String::from("")
                }
            }
        }
    } else {
        if ((form != "string") && (form != "number")) {
            String::from("")
        } else {
            if (target == "JavaScript") {
                if (form == "number") {
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from("ml_big_integer_conversion.convertNumber(")),
                            value.to_owned()
                        )),
                        (String::from(")"))
                    )
                } else {
                    format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from(
                                "((ml_text)=>ml_big_integer_conversion.convertUnits(Array.from({length:ml_text.length}, (_,index)=>ml_text.charCodeAt(index))))("
                            )),
                            value.to_owned()
                        )),
                        (String::from(")"))
                    )
                }
            } else {
                if (target == "Rust") {
                    if (form == "number") {
                        format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (String::from(
                                    "crate::ml_big_integer_conversion::convert_java_script_number_big_integer("
                                )),
                                value.to_owned()
                            )),
                            (String::from(")"))
                        )
                    } else {
                        {
                            let units = if utf16 {
                                format!(
                                    "{}{}",
                                    (format!("{}{}", (String::from("(")), value.to_owned())),
                                    (String::from(").0.iter().copied()"))
                                )
                            } else {
                                format!(
                                    "{}{}",
                                    (format!("{}{}", (String::from("(")), value.to_owned())),
                                    (String::from(").encode_utf16()"))
                                )
                            };
                            format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (String::from(
                                        "crate::ml_big_integer_conversion::convert_java_script_string_big_integer_units(&"
                                    )),
                                    units
                                )),
                                (String::from(".map(f64::from).collect::<Vec<f64>>())"))
                            )
                        }
                    }
                } else {
                    if (target == "Lean") {
                        if (form == "number") {
                            format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (String::from("(MLBigIntegerConversion.convertNumber ")),
                                    value.to_owned()
                                )),
                                (String::from(")"))
                            )
                        } else {
                            format!(
                                "{}{}",
                                (format!(
                                    "{}{}",
                                    (String::from(
                                        "(MLBigIntegerConversion.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] "
                                    )),
                                    value.to_owned()
                                )),
                                (String::from("))"))
                            )
                        }
                    } else {
                        if (target == "Rocq") {
                            if (form == "number") {
                                format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (String::from("(MLBigIntegerConversion.convertNumber ")),
                                        value.to_owned()
                                    )),
                                    (String::from(")"))
                                )
                            } else {
                                format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (String::from(
                                            "(MLBigIntegerConversion.convertUtf8 ((fix ml_integer_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_integer_bytes rest) end) "
                                        )),
                                        value.to_owned()
                                    )),
                                    (String::from("))"))
                                )
                            }
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(
    bytes: &[f64],
    mut index: f64,
    mut scalar: f64,
    mut remaining: f64,
) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next =
                    (self::ml_array::optional_at(bytes, self::ml_array::number_index(index)))
                        .clone()
                        .unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(vec![(-1f64)])
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop2(
    bytes: &[f64],
    mut units: Vec<f64>,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
    loop {
        return if (index < (bytes.len() as f64)) {
            {
                let scalar =
                    (self::ml_array::optional_at(bytes, self::ml_array::number_index(index)))
                        .clone()
                        .unwrap_or_else(|| (-1f64));
                {
                    let index_2 = (index + 1f64);
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source2_s36 = if ((scalar >= (194f64))
                                    && (scalar <= (223f64)))
                                {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_2, remaining_2, minimum_2)
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
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_3, remaining_3, minimum_3)
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
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Return(vec![(-1f64)])
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source2_s36.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s37 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s37.clone() {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(vec![(-1f64)])
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop1(
    bytes: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    loop {
        return if (index < (bytes.len() as f64)) {
            {
                let unit =
                    (self::ml_array::optional_at(bytes, self::ml_array::number_index(index)))
                        .clone()
                        .unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64)))
                    || (unit > (255f64)))
                {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Return(vec![(-1f64)])
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
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop5(
    mut exponent: f64,
    mut value: self::ml::Big,
    mut base: self::ml::Big,
) -> self::ml::Big {
    loop {
        return if (exponent > (0f64)) {
            {
                let value_3 = if ((exponent % 2f64) == (1f64)) {
                    {
                        let value_2 = value.mul(&base);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = (exponent / 2f64).floor();
                    if (exponent_2 > (0f64)) {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) =
                                    (exponent_2, value_3.clone(), base_2.clone());
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

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop6(
    mut value: self::ml::Big,
    mut result: f64,
    mut scale: f64,
) -> f64 {
    loop {
        return if (value > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(
                    &value,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    true,
                )) == (self::ml::Big::from_i128(1)))
                {
                    {
                        let result_2 = (result + scale);
                        result_2
                    }
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

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop8(
    mut exponent: f64,
    mut result: f64,
) -> f64 {
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
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop7Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop7(
    mut exponent: f64,
    mut result: f64,
) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result {
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop12(
    mut quotient: self::ml::Big,
    mut remainder: self::ml::Big,
    mut scale: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop12Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
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
                    if (remainder >= scale_2) {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (
                                        quotient_2.clone(),
                                        remainder_2.clone(),
                                        scale_2.clone(),
                                        weight_2.clone(),
                                    );
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
            self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop11Result {
    MlSource1TemporaryConvertStringNumberRationalLoop11Done(self::ml::Big, self::ml::Big),
}
