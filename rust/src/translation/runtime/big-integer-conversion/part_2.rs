                format!("{}{}", (format!("{}{}", (String::from("(+(")), value.to_owned())), (String::from("))")))
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
                                format!("{}{}", (format!("{}{}", (String::from("(let _ := ")), value.to_owned())), (String::from("; ((0 : Float) / 0))")))
                            } else {
                                format!("{}{}", (format!("{}{}", (String::from("(let _ := ")), value.to_owned())), (String::from(" in PrimFloat.nan)")))
                            }
                        } else {
                            if (form == "bool") {
                                if (format == "Lean") {
                                    format!("{}{}", (format!("{}{}", (String::from("(if ")), value.to_owned())), (String::from(" then (1 : Float) else (0 : Float))")))
                                } else {
                                    format!("{}{}", (format!("{}{}", (String::from("(if ")), value.to_owned())), (String::from(" then 1%float else 0%float)")))
                                }
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertInteger ")), (if (form == "nat") {
                                        if (format == "Lean") {
                                            format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat ")), value.to_owned())), (String::from(")")))
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(Z.of_N ")), value.to_owned())), (String::from(")")))
                                        }
                                    } else {
                                        value.to_owned()
                                    }))), (String::from(")")))
                                } else {
                                    if (format == "Lean") {
                                        format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] ")), value.to_owned())), (String::from("))")))
                                    } else {
                                        format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) ")), value.to_owned())), (String::from("))")))
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
                            format!("{}{}", (format!("{}{}", (String::from("(if ")), value.to_owned())), (String::from(" { 1.0f64 } else { 0.0f64 })")))
                        } else {
                            if (form == "unit") {
                                format!("{}{}", (format!("{}{}", (String::from("{ let _ = ")), value.to_owned())), (String::from("; f64::NAN }")))
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    if (operation == "coerceJavaScriptNumber") {
                                        format!("{}{}", (format!("{}{}", (String::from("{ let _ = ")), value.to_owned())), (String::from("; crate::ml_source_exception::SourceException::from_text(\"TypeError\", \"Cannot convert a BigInt value to a number\").raise() }")))
                                    } else {
                                        format!("{}{}", (format!("{}{}", (String::from("crate::ml_string_number::convert_java_script_big_integer_number(")), value.to_owned())), (String::from(")")))
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            format!("{}{}", (format!("{}{}", (String::from("(")), value.to_owned())), (String::from(").0.iter().copied()")))
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(")), value.to_owned())), (String::from(").encode_utf16()")))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_string_big_integer_units(units: &[f64]) -> Vec<crate::ml::Big> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s25 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop30(units, index);
            match source_ml_source2_s25.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop30Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop31(units, start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop32(units, start_2, end);
                                    if (start_2 == end_2) {
                                        vec![crate::ml::Big::from_i128(0)]
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(units, self::ml_array::number_index(start_2))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s26 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s26.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin33Next(start_4, negative_3, signed_3) => {
                                                                if (start_4 == end_2) {
                                                                    Vec::<crate::ml::Big>::new()
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = crate::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s27 = if ((!signed_3 && (((self::ml_array::optional_at(units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(units, self::ml_array::number_index((start_4 + 1f64)))).clone().unwrap_or_else(|| (-1f64));
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
                                                                                                            crate::ml::Big::from_i128(16)
                                                                                                        } else {
                                                                                                            if (selected == (8f64)) {
                                                                                                                crate::ml::Big::from_i128(8)
                                                                                                            } else {
                                                                                                                crate::ml::Big::from_i128(2)
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let start_5 = (start_4 + 2f64);
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_5, radix_2, base_2.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_4, radix, base.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s27.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin34Next(start_6, radix_3, base_3) => {
                                                                                        if (start_6 == end_2) {
                                                                                            Vec::<crate::ml::Big>::new()
                                                                                        } else {
                                                                                            {
                                                                                                let digits = vec![crate::ml::Big::from_i128(0), crate::ml::Big::from_i128(1), crate::ml::Big::from_i128(2), crate::ml::Big::from_i128(3), crate::ml::Big::from_i128(4), crate::ml::Big::from_i128(5), crate::ml::Big::from_i128(6), crate::ml::Big::from_i128(7), crate::ml::Big::from_i128(8), crate::ml::Big::from_i128(9), crate::ml::Big::from_i128(10), crate::ml::Big::from_i128(11), crate::ml::Big::from_i128(12), crate::ml::Big::from_i128(13), crate::ml::Big::from_i128(14), crate::ml::Big::from_i128(15)];
                                                                                                {
                                                                                                    let value = crate::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let index_2 = start_6;
                                                                                                        {
                                                                                                            let source_ml_source2_s28 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop35(units, end_2, radix_3, base_3.clone(), digits, value.clone(), index_2);
                                                                                                            match source_ml_source2_s28.clone() {
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
pub fn convert_java_script_utf8_big_integer(bytes: &[f64]) -> Vec<crate::ml::Big> {
    self::convert_java_script_string_big_integer_units(&(self::read_java_script_utf8_string_units(bytes)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_number_big_integer(value: f64) -> Vec<crate::ml::Big> {
    if !(self::ml_math::is_integer(value)) {
        Vec::<crate::ml::Big>::new()
    } else {
        {
            let negative = (value < (0f64));
            {
                let value_2 = value.abs();
                {
                    let integer = crate::ml::Big::from_i128(0);
                    {
                        let weight = crate::ml::Big::from_i128(1);
                        {
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop36(value_2, integer.clone(), weight.clone());
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
pub fn convert_java_script_boolean_big_integer(value: bool) -> crate::ml::Big {
    if value {
        crate::ml::Big::from_i128(1)
    } else {
        crate::ml::Big::from_i128(0)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_java_script_number_big_integer(value: f64, integer: crate::ml::Big) -> f64 {
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
                        let lower = (self::ml_array::optional_at(&self::convert_java_script_number_big_integer(floor), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0));
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_java_script_string_big_integer_units(units: &[f64], integer: crate::ml::Big) -> f64 {
    {
        let result = self::convert_java_script_string_big_integer_units(units);
        if (((result.len() as f64)) != (1f64)) {
            2f64
        } else {
            {
                let value = (self::ml_array::optional_at(&result, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0));
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_java_script_utf8_big_integer(bytes: &[f64], integer: crate::ml::Big) -> f64 {
    self::compare_java_script_string_big_integer_units(&(self::read_java_script_utf8_string_units(bytes)), integer.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn divide_java_script_big_integer(left: crate::ml::Big, right: crate::ml::Big) -> Vec<crate::ml::Big> {
    if (right == (crate::ml::Big::from_i128(0))) {
        Vec::<crate::ml::Big>::new()
    } else {
        {
            let remaining = if (left < (crate::ml::Big::from_i128(0))) {
                left.neg()
            } else {
                left.clone()
            };
            {
                let divisor = if (right < (crate::ml::Big::from_i128(0))) {
                    right.neg()
                } else {
                    right.clone()
                };
                {
                    let shifted = divisor.clone();
                    {
                        let weight = crate::ml::Big::from_i128(1);
                        {
                            let quotient = crate::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s29 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop37(remaining.clone(), shifted.clone(), weight.clone());
                                match source_ml_source2_s29.clone() {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s30 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop38(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient.clone());
                                            match source_ml_source2_s30.clone() {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Done(remaining_2, quotient_2) => {
                                                    vec![if (((left < (crate::ml::Big::from_i128(0)))) != ((right < (crate::ml::Big::from_i128(0))))) {
                                                        quotient_2.neg()
                                                    } else {
                                                        quotient_2.clone()
                                                    }, if (left < (crate::ml::Big::from_i128(0))) {
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
pub fn power_java_script_big_integer(base: crate::ml::Big, exponent: crate::ml::Big) -> Vec<crate::ml::Big> {
    if (exponent < (crate::ml::Big::from_i128(0))) {
        Vec::<crate::ml::Big>::new()
    } else {
        {
            let result = crate::ml::Big::from_i128(1);
            {
                let weight = base.clone();
                {
                    let remaining = exponent.clone();
                    {
                        let result_2 = self::source_ml_source1_temporary_power_java_script_big_integer_loop39(result.clone(), weight.clone(), remaining.clone());
                        vec![result_2.clone()]
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_number_power_bound(value: crate::ml::Big, exponent: crate::ml::Big, scale: crate::ml::Big, upward: bool) -> Vec<crate::ml::Big> {
    if (value <= (crate::ml::Big::from_i128(0))) {
        vec![crate::ml::Big::from_i128(0), crate::ml::Big::from_i128(0)]
    } else {
        {
            let source_ml_source2_s31 = self::source_ml_source1_temporary_normalize_number_power_bound_loop40(value.clone(), exponent.clone(), scale.clone(), upward);
            match source_ml_source2_s31.clone() {
                self::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s32 = self::source_ml_source1_temporary_normalize_number_power_bound_loop41(value_2.clone(), exponent_2.clone(), scale.clone());
                        match source_ml_source2_s32.clone() {
                            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Done(value_3, exponent_3) => {
                                vec![value_3.clone(), exponent_3.clone()]
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
pub fn divide_number_power_bound(numerator: crate::ml::Big, denominator: crate::ml::Big, upward: bool) -> crate::ml::Big {
    {
        let result = self::divide_java_script_big_integer(numerator.clone(), denominator.clone());
        {
            let quotient = (self::ml_array::optional_at(&result, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0));
            {
                let remainder = (self::ml_array::optional_at(&result, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0));
                quotient.add(&if (upward && (remainder != (crate::ml::Big::from_i128(0)))) {
                    crate::ml::Big::from_i128(1)
                } else {
                    crate::ml::Big::from_i128(0)
                })
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_integer(value: crate::ml::Big) -> crate::ml::Big {
    if (value <= (crate::ml::Big::from_i128(0))) {
        crate::ml::Big::from_i128(0)
    } else {
        {
            let root = crate::ml::Big::from_i128(1);
            {
                let remaining = value.clone();
                {
                    let root_2 = self::source_ml_source1_temporary_root_number_power_integer_loop42(root.clone(), remaining.clone());
                    {
                        let root_3 = root_2.mul(&crate::ml::Big::from_i128(2));
                        self::source_ml_source1_temporary_root_number_power_integer_loop43(value.clone(), root_3.clone())
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn multiply_number_power_interval(left: Vec<crate::ml::Big>, right: Vec<crate::ml::Big>, scale: crate::ml::Big) -> Vec<crate::ml::Big> {
    {
        let lower = self::normalize_number_power_bound(self::divide_number_power_bound(((self::ml_array::optional_at(&left, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))).mul(&(self::ml_array::optional_at(&right, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))), scale.clone(), false), ((self::ml_array::optional_at(&left, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))).add(&(self::ml_array::optional_at(&right, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))), scale.clone(), false);
        {
            let upper = self::normalize_number_power_bound(self::divide_number_power_bound(((self::ml_array::optional_at(&left, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))).mul(&(self::ml_array::optional_at(&right, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))), scale.clone(), true), ((self::ml_array::optional_at(&left, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))).add(&(self::ml_array::optional_at(&right, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))), scale.clone(), true);
            vec![(self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_bound(value: crate::ml::Big, exponent: crate::ml::Big, scale: crate::ml::Big, upward: bool) -> Vec<crate::ml::Big> {
    {
        let source_ml_source2_s33 = if ((crate::ml::divide(&exponent, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) != (crate::ml::Big::from_i128(0))) {
            {
                let value_2 = value.mul(&crate::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent.sub(&crate::ml::Big::from_i128(1));
                    self::MlSource1TemporaryRootNumberPowerBoundJoin44::MlSource1TemporaryRootNumberPowerBoundJoin44Next(value_2.clone(), exponent_2.clone())
                }
            }
        } else {
            self::MlSource1TemporaryRootNumberPowerBoundJoin44::MlSource1TemporaryRootNumberPowerBoundJoin44Next(value.clone(), exponent.clone())
        };
        match source_ml_source2_s33.clone() {
            self::MlSource1TemporaryRootNumberPowerBoundJoin44::MlSource1TemporaryRootNumberPowerBoundJoin44Next(value_3, exponent_3) => {
                {
                    let square = value_3.mul(&scale);
                    {
                        let root = self::root_number_power_integer(square.clone());
                        {
                            let root_3 = if (upward && ((root.mul(&root)) < square)) {
                                {
                                    let root_2 = root.add(&crate::ml::Big::from_i128(1));
                                    root_2.clone()
                                }
                            } else {
                                root.clone()
                            };
                            self::normalize_number_power_bound(root_3.clone(), crate::ml::divide(&exponent_3, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false), scale.clone(), upward)
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_interval(interval: Vec<crate::ml::Big>, scale: crate::ml::Big) -> Vec<crate::ml::Big> {
    {
        let lower = self::root_number_power_bound((self::ml_array::optional_at(&interval, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), scale.clone(), false);
        {
            let upper = self::root_number_power_bound((self::ml_array::optional_at(&interval, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), scale.clone(), true);
            vec![(self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn invert_number_power_interval(interval: Vec<crate::ml::Big>, scale: crate::ml::Big) -> Vec<crate::ml::Big> {
    {
        let lower = self::normalize_number_power_bound(self::divide_number_power_bound(scale.mul(&scale), (self::ml_array::optional_at(&interval, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), false), ((self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))).neg(), scale.clone(), false);
        {
            let upper = self::normalize_number_power_bound(self::divide_number_power_bound(scale.mul(&scale), (self::ml_array::optional_at(&interval, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), true), ((self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))).neg(), scale.clone(), true);
            vec![(self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decompose_number_power_magnitude(value: f64) -> Vec<crate::ml::Big> {
    {
        let exponent = crate::ml::Big::from_i128(0);
        {
            let source_ml_source2_s34 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop45(value, exponent.clone());
            match source_ml_source2_s34.clone() {
                self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s35 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop46(value_2, exponent_2.clone());
                        match source_ml_source2_s35.clone() {
                            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Done(value_3, exponent_3) => {
                                vec![(self::ml_array::optional_at(&self::convert_java_script_number_big_integer((value_3 * 4503599627370496f64)), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)), exponent_3.sub(&crate::ml::Big::from_i128(52))]
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
pub fn round_number_power_bound(value: crate::ml::Big, exponent: crate::ml::Big, precision: crate::ml::Big) -> f64 {
    if ((value == (crate::ml::Big::from_i128(0))) || (exponent < (crate::ml::Big::from_i128(-1076)))) {
        0f64
    } else {
        if (exponent > (crate::ml::Big::from_i128(1024))) {
            f64::INFINITY
        } else {
            {
                let shift = exponent.sub(&precision);
                if (shift >= (crate::ml::Big::from_i128(0))) {
                    self::convert_string_number_rational(value.mul(&(self::ml_array::optional_at(&self::power_java_script_big_integer(crate::ml::Big::from_i128(2), shift.clone()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0))), crate::ml::Big::from_i128(1))
                } else {
                    self::convert_string_number_rational(value.clone(), (self::ml_array::optional_at(&self::power_java_script_big_integer(crate::ml::Big::from_i128(2), shift.neg()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn power_java_script_number(base: f64, exponent: f64) -> f64 {
    if exponent.is_nan() {
        f64::NAN
    } else {
        if (exponent == (0f64)) {
            1f64
        } else {
            if base.is_nan() {
                f64::NAN
            } else {
                {
                    let magnitude = base.abs();
                    if !(exponent.is_finite()) {
                        if (magnitude == (1f64)) {
                            f64::NAN
                        } else {
                            if (magnitude > (1f64)) {
                                if (exponent > (0f64)) {
                                    f64::INFINITY
                                } else {
                                    0f64
                                }
                            } else {
                                if (exponent > (0f64)) {
                                    0f64
                                } else {
                                    f64::INFINITY
                                }
                            }
                        }
                    } else {
                        {
                            let odd = (self::ml_math::is_integer(exponent) && (((exponent.abs() % 2f64)) == (1f64)));
                            {
                                let negative = (((base < (0f64)) || ((base == (0f64)) && (((1f64 / base)) < (0f64)))) && odd);
                                if (base == (0f64)) {
                                    {
                                        let value = if (exponent > (0f64)) {
                                            0f64
                                        } else {
                                            f64::INFINITY
                                        };
                                        if negative {
                                            (-value)
                                        } else {
                                            value
                                        }
                                    }
                                } else {
                                    if !(base.is_finite()) {
                                        {
                                            let value_2 = if (exponent > (0f64)) {
                                                f64::INFINITY
                                            } else {
                                                0f64
                                            };
                                            if negative {
                                                (-value_2)
                                            } else {
                                                value_2
                                            }
                                        }
                                    } else {
                                        if ((base < (0f64)) && !(self::ml_math::is_integer(exponent))) {
                                            f64::NAN
                                        } else {
                                            if (magnitude == (1f64)) {
                                                if negative {
                                                    (-1f64)
                                                } else {
                                                    1f64
                                                }
                                            } else {
                                                {
                                                    let input = self::decompose_number_power_magnitude(magnitude);
                                                    {
                                                        let power = self::decompose_number_power_magnitude(exponent.abs());
                                                        {
                                                            let precision = crate::ml::Big::from_i128(192);
                                                            self::source_ml_source1_temporary_power_java_script_number_loop47(exponent, negative, input, power, precision.clone())
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_exponentiation_base(prefix: &str, parenthesized: bool) -> bool {
    (parenthesized || !(self::ml_array::includes(&vec![String::from("+"), String::from("-"), String::from("!"), String::from("~"), String::from("typeof"), String::from("void"), String::from("delete"), String::from("await")], &prefix.to_owned(), 0f64, false, |a, b| a == b)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_exponent_message() -> String {
    String::from("Exponent must be positive")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_power(target: &str, base: &str, exponent: &str) -> String {
    if (target == "JavaScript") {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.power(")), base.to_owned())), (String::from(",")))), exponent.to_owned())), (String::from(")")))
    } else {
        if (target == "Rust") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::power_java_script_big_integer(")), base.to_owned())), (String::from(",")))), exponent.to_owned())), (String::from(")")))
        } else {
            if ((target == "Lean") || (target == "Rocq")) {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.power ")), base.to_owned())), (String::from(" ")))), exponent.to_owned())), (String::from(")")))
            } else {
                String::from("")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_nonzero_java_script_integer_literal(value: &str) -> bool {
    (((value != "") && (value != "0")) && (value != "-0"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_big_integer_division(language: &str, operation: &str, kind: &str, literal: &str) -> bool {
    ((((language == "JavaScript") && (kind == "int")) && ((operation == "div") || (operation == "rem"))) && !(self::accept_nonzero_java_script_integer_literal(literal)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_division_message() -> String {
    String::from("Division by zero")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_division(target: &str, left: &str, right: &str) -> String {
    if (target == "JavaScript") {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.divide(")), left.to_owned())), (String::from(",")))), right.to_owned())), (String::from(")")))
    } else {
        if (target == "Rust") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::divide_java_script_big_integer(")), left.to_owned())), (String::from(",")))), right.to_owned())), (String::from(")")))
        } else {
            if ((target == "Lean") || (target == "Rocq")) {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.divide ")), left.to_owned())), (String::from(" ")))), right.to_owned())), (String::from(")")))
            } else {
                String::from("")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_form(kind: &str) -> String {
    if ((kind == "int") || (kind == "nat")) {
        String::from("identity")
    } else {
        if (kind == "float") {
            String::from("number")
        } else {
            if (kind == "string") {
                String::from("string")
            } else {
                if (kind == "bool") {
                    String::from("boolean")
                } else {
                    if (kind == "unit") {
                        String::from("undefined")
                    } else {
                        if (kind == "option") {
                            String::from("optional")
                        } else {
                            String::from("unsupported")
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_error_class(form: &str) -> String {
    if (form == "string") {
        String::from("SyntaxError")
    } else {
        if (form == "number") {
            String::from("RangeError")
        } else {
            if ((form == "undefined") || (form == "null")) {
                String::from("TypeError")
            } else {
                String::from("")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_conversion_message(form: &str, value: &str) -> String {
    {
        let pieces = self::read_java_script_big_integer_conversion_message_pieces(form);
        if (((pieces.len() as f64)) == (2f64)) {
            format!("{}{}", (format!("{}{}", ((self::ml_array::optional_at(&pieces, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| String::from(""))), value.to_owned())), ((self::ml_array::optional_at(&pieces, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| String::from(""))))
        } else {
            String::from("")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_message_pieces(form: &str) -> Vec<String> {
    if (form == "number") {
        vec![String::from("The number "), String::from(" cannot be converted to a BigInt because it is not an integer")]
    } else {
        if (((form == "string") || (form == "undefined")) || (form == "null")) {
            vec![String::from("Cannot convert "), String::from(" to a BigInt")]
        } else {
            Vec::<String>::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_builtin_operation(name: &str) -> String {
    if (name == "BigInt") {
        String::from("convertJavaScriptBigInteger")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_result_operation(operation: &str) -> String {
    if (operation == "compareJavaScriptNumberBigIntegerValue") {
        String::from("number-compare")
    } else {
        if (operation == "compareJavaScriptStringBigIntegerValue") {
            String::from("string-compare")
        } else {
            if (operation == "powerJavaScriptNumberValue") {
                String::from("number-power")
            } else {
                if (operation == "powerJavaScriptBigIntegerResult") {
                    String::from("power")
                } else {
                    if (operation == "divideJavaScriptBigIntegerResult") {
                        String::from("division")
                    } else {
                        if (operation == "parseJavaScriptStringBigInteger") {
                            String::from("string")
                        } else {
                            if (operation == "convertJavaScriptBooleanBigIntegerValue") {
                                String::from("boolean")
                            } else {
                                if (operation == "convertJavaScriptNumberBigIntegerResult") {
                                    String::from("number")
                                } else {
                                    String::from("")
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
