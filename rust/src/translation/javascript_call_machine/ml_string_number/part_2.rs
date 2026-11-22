// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::ml_array;
use super::ml_math;
use super::read_java_script_number_conversion_form;

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_number_conversion(kind: &str, operation: &str) -> bool {
    ((kind == "math")
        && ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber")))
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_failure(operation: &str, kind: &str) -> String {
    if ((operation == "coerceJavaScriptNumber") && ((kind == "int") || (kind == "nat"))) {
        String::from("Cannot convert a BigInt value to a number")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_number_conversion_units(
    kind: &str,
    operation: &str,
    argument: &str,
) -> bool {
    (self::accept_linked_java_script_number_conversion(kind, operation) && (argument == "string"))
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_number_operand(
    language: &str,
    kind: &str,
    element: &str,
) -> String {
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

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_numeric_inference_kind(kind: &str) -> String {
    if (((kind == "bool") || (kind == "string")) || (kind == "unit")) {
        String::from("float")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_unary_operation(symbol: &str) -> String {
    if (symbol == "+") {
        String::from("coerceJavaScriptNumber")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_mathematical_conversion_form(language: &str, operation: &str) -> String {
    if (language != "JavaScript") {
        String::from("")
    } else {
        if (operation == "powerJavaScriptNumberArguments") {
            String::from("binary")
        } else {
            if ((operation == "max") || (operation == "min")) {
                String::from("variadic")
            } else {
                if (((((((operation == "abs") || (operation == "floor"))
                    || (operation == "ceil"))
                    || (operation == "trunc"))
                    || (operation == "round"))
                    || (operation == "sign"))
                    || (operation == "sqrt"))
                {
                    String::from("unary")
                } else {
                    String::from("")
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_number_scalar_conversion(
    format: &str,
    operation: &str,
    form: &str,
    value: &str,
    utf16: bool,
) -> String {
    if ((!(self::accept_linked_java_script_number_conversion("math", operation))
        || ((self::read_java_script_number_conversion_form(form)) == "unsupported"))
        || (form == "option"))
    {
        String::from("")
    } else {
        if (format == "JavaScript") {
            if (operation == "convertJavaScriptNumber") {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("Number(")), value.to_owned())),
                    (String::from(")"))
                )
            } else {
                format!(
                    "{}{}",
                    (format!("{}{}", (String::from("(+(")), value.to_owned())),
                    (String::from("))"))
                )
            }
        } else {
            if ((format == "Lean") || (format == "Rocq")) {
                if !(self::read_java_script_number_conversion_failure(operation, form)).is_empty() {
                    String::from("")
                } else {
                    if (form == "float") {
                        value.to_owned()
                    } else {
                        if (form == "unit") {
                            if (format == "Lean") {
                                format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (String::from("(let _ := ")),
                                        value.to_owned()
                                    )),
                                    (String::from("; ((0 : Float) / 0))"))
                                )
                            } else {
                                format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (String::from("(let _ := ")),
                                        value.to_owned()
                                    )),
                                    (String::from(" in PrimFloat.nan)"))
                                )
                            }
                        } else {
                            if (form == "bool") {
                                if (format == "Lean") {
                                    format!(
                                        "{}{}",
                                        (format!("{}{}", (String::from("(if ")), value.to_owned())),
                                        (String::from(" then (1 : Float) else (0 : Float))"))
                                    )
                                } else {
                                    format!(
                                        "{}{}",
                                        (format!("{}{}", (String::from("(if ")), value.to_owned())),
                                        (String::from(" then 1%float else 0%float)"))
                                    )
                                }
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    format!(
                                        "{}{}",
                                        (format!(
                                            "{}{}",
                                            (String::from("(MLStringNumber.convertInteger ")),
                                            (if (form == "nat") {
                                                if (format == "Lean") {
                                                    format!(
                                                        "{}{}",
                                                        (format!(
                                                            "{}{}",
                                                            (String::from("(Int.ofNat ")),
                                                            value.to_owned()
                                                        )),
                                                        (String::from(")"))
                                                    )
                                                } else {
                                                    format!(
                                                        "{}{}",
                                                        (format!(
                                                            "{}{}",
                                                            (String::from("(Z.of_N ")),
                                                            value.to_owned()
                                                        )),
                                                        (String::from(")"))
                                                    )
                                                }
                                            } else {
                                                value.to_owned()
                                            })
                                        )),
                                        (String::from(")"))
                                    )
                                } else {
                                    if (format == "Lean") {
                                        format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (String::from(
                                                    "(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] "
                                                )),
                                                value.to_owned()
                                            )),
                                            (String::from("))"))
                                        )
                                    } else {
                                        format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (String::from(
                                                    "(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) "
                                                )),
                                                value.to_owned()
                                            )),
                                            (String::from("))"))
                                        )
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
                        value.to_owned()
                    } else {
                        if (form == "bool") {
                            format!(
                                "{}{}",
                                (format!("{}{}", (String::from("(if ")), value.to_owned())),
                                (String::from(" { 1.0f64 } else { 0.0f64 })"))
                            )
                        } else {
                            if (form == "unit") {
                                format!(
                                    "{}{}",
                                    (format!(
                                        "{}{}",
                                        (String::from("{ let _ = ")),
                                        value.to_owned()
                                    )),
                                    (String::from("; f64::NAN }"))
                                )
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    if (operation == "coerceJavaScriptNumber") {
                                        format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (String::from("{ let _ = ")),
                                                value.to_owned()
                                            )),
                                            (String::from(
                                                "; crate::ml_source_exception::SourceException::from_text(\"TypeError\", \"Cannot convert a BigInt value to a number\").raise() }"
                                            ))
                                        )
                                    } else {
                                        format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (String::from(
                                                    "crate::ml_string_number::convert_java_script_big_integer_number("
                                                )),
                                                value.to_owned()
                                            )),
                                            (String::from(")"))
                                        )
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            format!(
                                                "{}{}",
                                                (format!(
                                                    "{}{}",
                                                    (String::from("(")),
                                                    value.to_owned()
                                                )),
                                                (String::from(").0.iter().copied()"))
                                            )
                                        } else {
                                            format!(
                                                "{}{}",
                                                (format!(
                                                    "{}{}",
                                                    (String::from("(")),
                                                    value.to_owned()
                                                )),
                                                (String::from(").encode_utf16()"))
                                            )
                                        };
                                        format!(
                                            "{}{}",
                                            (format!(
                                                "{}{}",
                                                (String::from(
                                                    "crate::ml_string_number::convert_java_script_string_number_units(&"
                                                )),
                                                units
                                            )),
                                            (String::from(
                                                ".map(f64::from).collect::<Vec<f64>>())"
                                            ))
                                        )
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_statement() -> String {
    String::from(
        "ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws",
    )
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
                                let source_ml_source2_s25 = if ((scalar >= (194f64))
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
                                match source_ml_source2_s25.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s26 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s26.clone() {
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

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop11(
    mut remainder: self::ml::Big,
    mut scale: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop11Result {
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop13(
    mut numerator: self::ml::Big,
    mut shift: f64,
    mut index: f64,
) -> self::ml::Big {
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

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop14(
    mut denominator: self::ml::Big,
    mut shift: f64,
    mut index: f64,
) -> self::ml::Big {
    loop {
        return if (index < (-shift)) {
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
pub enum MlSource1TemporaryConvertStringNumberRationalJoin9 {
    MlSource1TemporaryConvertStringNumberRationalJoin9Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin9Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop15Result {
    MlSource1TemporaryConvertStringNumberRationalLoop15Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop15Return(f64),
}
