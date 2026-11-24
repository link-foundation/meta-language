pub fn read_java_script_numeric_digit_unit(unit: f64) -> f64 {
    if ((unit >= (48f64)) && (unit <= (57f64))) {
        (unit - 48f64)
    } else {
        if ((unit >= (65f64)) && (unit <= (70f64))) {
            (unit - 55f64)
        } else {
            if ((unit >= (97f64)) && (unit <= (102f64))) {
                (unit - 87f64)
            } else {
                (-1f64)
            }
        }
    }
}

pub fn read_java_script_utf8_string_units(bytes: Vec<f64>) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s19 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes.clone(), index);
            match source_ml_source2_s19.clone() {
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s20 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop5(bytes, units, index_2);
                                match source_ml_source2_s20.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Done(units_2) => {
                                        units_2
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop5Return(source_ml_source1_temporary_result5) => {
                                        source_ml_source1_temporary_result5
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Return(source_ml_source1_temporary_result7) => {
                    source_ml_source1_temporary_result7
                }
            }
        }
    }
}

pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop8(exponent, value.clone(), base.clone());
                value_2.clone()
            }
        }
    }
}

pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop9(value.clone(), result, scale);
                result_2
            }
        }
    }
}

pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s21 = self::source_ml_source1_temporary_read_string_number_binary_power_loop10(exponent, result);
            match source_ml_source2_s21.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop10Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop11(exponent_2, result_2);
                        result_3
                    }
                }
            }
        }
    }
}

pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if ((numerator < (self::ml::Big::from_i128(0))) || (denominator <= (self::ml::Big::from_i128(0)))) {
        f64::NAN
    } else {
        if (numerator == (self::ml::Big::from_i128(0))) {
            0f64
        } else {
            {
                let exponent = 0f64;
                {
                    let source_ml_source2_s24 = if (numerator >= denominator) {
                        {
                            let scale = denominator.clone();
                            {
                                let source_ml_source2_s25 = self::source_ml_source1_temporary_convert_string_number_rational_loop18(numerator.clone(), exponent, scale.clone());
                                match source_ml_source2_s25.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop18Result::MlSource1TemporaryConvertStringNumberRationalLoop18Done(exponent_2) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Next(exponent_2)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop18Result::MlSource1TemporaryConvertStringNumberRationalLoop18Return(source_ml_source1_temporary_result18) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Return(source_ml_source1_temporary_result18)
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let scale_2 = numerator.clone();
                            {
                                let source_ml_source2_s26 = self::source_ml_source1_temporary_convert_string_number_rational_loop19(denominator.clone(), exponent, scale_2.clone());
                                match source_ml_source2_s26.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop19Result::MlSource1TemporaryConvertStringNumberRationalLoop19Done(exponent_3) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Next(exponent_3)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop19Result::MlSource1TemporaryConvertStringNumberRationalLoop19Return(source_ml_source1_temporary_result19) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Return(source_ml_source1_temporary_result19)
                                    }
                                }
                            }
                        }
                    };
                    match source_ml_source2_s24.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s27 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop16(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin13::MlSource1TemporaryConvertStringNumberRationalJoin13Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop17(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin13::MlSource1TemporaryConvertStringNumberRationalJoin13Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s27.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin13::MlSource1TemporaryConvertStringNumberRationalJoin13Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale_3 = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s28 = self::source_ml_source1_temporary_convert_string_number_rational_loop14(remainder.clone(), scale_3.clone(), weight.clone());
                                                                match source_ml_source2_s28.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop14Result::MlSource1TemporaryConvertStringNumberRationalLoop14Done(scale_4, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s29 = self::source_ml_source1_temporary_convert_string_number_rational_loop15(quotient.clone(), remainder.clone(), scale_4.clone(), weight_2.clone());
                                                                            match source_ml_source2_s29.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&self::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if ((twice > denominator_3) || ((twice == denominator_3) && ((self::ml::divide(&quotient_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))))) {
                                                                                                {
                                                                                                    let quotient_3 = quotient_2.add(&self::ml::Big::from_i128(1));
                                                                                                    quotient_3.clone()
                                                                                                }
                                                                                            } else {
                                                                                                quotient_2.clone()
                                                                                            };
                                                                                            (self::read_string_number_exact_mantissa(quotient_4.clone()) * self::read_string_number_binary_power((-shift)))
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
                        self::MlSource1TemporaryConvertStringNumberRationalJoin12::MlSource1TemporaryConvertStringNumberRationalJoin12Return(source_ml_source1_temporary_result17) => {
                            source_ml_source1_temporary_result17
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_string_number_infinity(units: Vec<f64>, start: f64, end: f64) -> bool {
    if (((end - start)) != (8f64)) {
        false
    } else {
        {
            let expected = vec![73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64];
            {
                let index = 0f64;
                {
                    let source_ml_source2_s30 = self::source_ml_source1_temporary_accept_string_number_infinity_loop20(units, start, expected, index);
                    match source_ml_source2_s30.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result::MlSource1TemporaryAcceptStringNumberInfinityLoop20Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop20Result::MlSource1TemporaryAcceptStringNumberInfinityLoop20Return(source_ml_source1_temporary_result20) => {
                            source_ml_source1_temporary_result20
                        }
                    }
                }
            }
        }
    }
}

pub fn convert_java_script_string_number_units(units: Vec<f64>) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s31 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop21(units.clone(), index);
            match source_ml_source2_s31.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop22(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop23(units.clone(), start_2, end);
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
                                                        let source_ml_source2_s32 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s32.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(start_4, negative_3, signed_3) => {
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
                                                                                let source_ml_source2_s34 = if ((!signed_3 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
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
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let index_2 = start_5;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s35 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop32(units.clone(), end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), index_2);
                                                                                                                                match source_ml_source2_s35.clone() {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Done(value_2, overflow_2) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(if overflow_2 {
                                                                                                                                            f64::INFINITY
                                                                                                                                        } else {
                                                                                                                                            self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                        })
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop32Return(source_ml_source1_temporary_result32) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(source_ml_source1_temporary_result32)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s34.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Next(start_6) => {
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
                                                                                                                let source_ml_source2_s36 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop26(units.clone(), end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s36.clone() {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop26Done(at_2, fraction_2, digits_2, significant_2) => {
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
                                                                                                                                            let source_ml_source2_s39 = if ((at_2 < end_2) && ((((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || (((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = (at_2 + 1f64);
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s40 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = (at_3 + 1f64);
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s40.clone() {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin30Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s41 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop31(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s41.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop31Done(at_6, exponent_2) => {
                                                                                                                                                                                        if (at_6 == first) {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Return(f64::NAN)
                                                                                                                                                                                        } else {
                                                                                                                                                                                            if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = (-exponent_2);
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_6, exponent_2)
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s39.clone() {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_7, exponent_4) => {
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
                                                                                                                                                                                    let source_ml_source2_s42 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop28(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s42.clone() {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let index_3 = start_6;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units, integer_digits, mantissa_end_2, mantissa.clone(), index_3);
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Return(source_ml_source1_temporary_result29) => {
                                                                                                                                                    source_ml_source1_temporary_result29
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin25Return(source_ml_source1_temporary_result31) => {
                                                                                        source_ml_source1_temporary_result31
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Return(source_ml_source1_temporary_result32_2) => {
                    source_ml_source1_temporary_result32_2
                }
            }
        }
    }
}

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

pub fn read_java_script_number_builtin_operation(name: self::ml_string::Text) -> self::ml_string::Text {
    if (name == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))) {
        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
    } else {
        if (name == (self::ml_string::Text::from_units(vec![105, 115, 70, 105, 110, 105, 116, 101]))) {
            self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 70, 105, 110, 105, 116, 101])
        } else {
            if (name == (self::ml_string::Text::from_units(vec![105, 115, 78, 97, 78]))) {
                self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 78, 97, 78])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_java_script_number_conversion_result(operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])))) {
        self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
    } else {
        if ((operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 70, 105, 110, 105, 116, 101]))) || (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 67, 111, 101, 114, 99, 101, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 78, 97, 78])))) {
            self::ml_string::Text::from_units(vec![98, 111, 111, 108])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_number_conversion_form(kind: self::ml_string::Text) -> self::ml_string::Text {
    if (((((((kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) || (kind == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) || (kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) {
        kind.clone()
    } else {
        self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])
    }
}

pub fn accept_linked_java_script_number_conversion(kind: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((kind == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])))))
}

pub fn read_java_script_number_conversion_failure(operation: self::ml_string::Text, kind: self::ml_string::Text) -> self::ml_string::Text {
    if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) && ((kind == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116]))))) {
        self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn require_java_script_number_conversion_units(kind: self::ml_string::Text, operation: self::ml_string::Text, argument: self::ml_string::Text) -> bool {
    (self::accept_linked_java_script_number_conversion(kind.clone(), operation.clone()) && (argument == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
}

pub fn read_java_script_primitive_number_operand(language: self::ml_string::Text, kind: self::ml_string::Text, element: self::ml_string::Text) -> self::ml_string::Text {
    if (language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
            self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (element == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_java_script_primitive_numeric_inference_kind(kind: self::ml_string::Text) -> self::ml_string::Text {
    if (((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
        self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_primitive_unary_operation(symbol: self::ml_string::Text) -> self::ml_string::Text {
    if (symbol == (self::ml_string::Text::from_units(vec![43]))) {
        self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_java_script_mathematical_conversion_form(language: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if (language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 65, 114, 103, 117, 109, 101, 110, 116, 115]))) {
            self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])
        } else {
            if ((operation == (self::ml_string::Text::from_units(vec![109, 97, 120]))) || (operation == (self::ml_string::Text::from_units(vec![109, 105, 110])))) {
                self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 100, 105, 99])
            } else {
                if (((((((operation == (self::ml_string::Text::from_units(vec![97, 98, 115]))) || (operation == (self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114])))) || (operation == (self::ml_string::Text::from_units(vec![99, 101, 105, 108])))) || (operation == (self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99])))) || (operation == (self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100])))) || (operation == (self::ml_string::Text::from_units(vec![115, 105, 103, 110])))) || (operation == (self::ml_string::Text::from_units(vec![115, 113, 114, 116])))) {
                    self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn render_java_script_number_scalar_conversion(format: self::ml_string::Text, operation: self::ml_string::Text, form: self::ml_string::Text, value: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    if ((!(self::accept_linked_java_script_number_conversion(self::ml_string::Text::from_units(vec![109, 97, 116, 104]), operation.clone())) || ((self::read_java_script_number_conversion_form(form.clone())) == (self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])))) || (form == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (format == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) {
                ((self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                ((self::ml_string::Text::from_units(vec![40, 43, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
            }
        } else {
            if ((format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (format == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                if !(self::read_java_script_number_conversion_failure(operation.clone(), form.clone())).0.is_empty() {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        value.clone()
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                            if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                ((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 95, 32, 58, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 40, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 47, 32, 48, 41, 41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 95, 32, 58, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 110, 97, 110, 41]))
                            }
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                                if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                    ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 40, 49, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 41]))
                                } else {
                                    ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 49, 37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101, 32, 48, 37, 102, 108, 111, 97, 116, 41]))
                                }
                            } else {
                                if ((form == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (form == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 73, 110, 116, 101, 103, 101, 114, 32])).concat(&if (form == (self::ml_string::Text::from_units(vec![110, 97, 116]))) {
                                        if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                            ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40, 90, 46, 111, 102, 95, 78, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                        }
                                    } else {
                                        value.clone()
                                    })).concat(&self::ml_string::Text::from_units(vec![41]))
                                } else {
                                    if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                        ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if (format != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        value.clone()
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                            ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 49, 46, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 46, 48, 102, 54, 52, 32, 125, 41]))
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                                ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 95, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 102, 54, 52, 58, 58, 78, 65, 78, 32, 125]))
                            } else {
                                if ((form == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (form == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) {
                                        ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 95, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 83, 111, 117, 114, 99, 101, 69, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 102, 114, 111, 109, 95, 116, 101, 120, 116, 40, 34, 84, 121, 112, 101, 69, 114, 114, 111, 114, 34, 44, 32, 34, 67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114, 34, 41, 46, 114, 97, 105, 115, 101, 40, 41, 32, 125]))
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 110, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41]))
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41]))
                                        };
                                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38])).concat(&units)).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 41]))
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

pub fn read_java_script_number_conversion_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![69, 67, 77, 65, 83, 99, 114, 105, 112, 116, 32, 78, 117, 109, 98, 101, 114, 32, 97, 110, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 112, 114, 101, 115, 101, 114, 118, 101, 32, 102, 117, 108, 108, 32, 85, 84, 70, 45, 49, 54, 32, 83, 116, 114, 105, 110, 103, 78, 117, 109, 101, 114, 105, 99, 76, 105, 116, 101, 114, 97, 108, 32, 115, 121, 110, 116, 97, 120, 44, 32, 101, 120, 97, 99, 116, 32, 98, 105, 110, 97, 114, 121, 54, 52, 32, 116, 105, 101, 115, 45, 116, 111, 45, 101, 118, 101, 110, 32, 114, 111, 117, 110, 100, 105, 110, 103, 44, 32, 115, 105, 103, 110, 101, 100, 32, 122, 101, 114, 111, 44, 32, 73, 110, 102, 105, 110, 105, 116, 121, 44, 32, 78, 97, 78, 44, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 97, 98, 115, 101, 110, 99, 101, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 101, 118, 97, 108, 117, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114, 59, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 111, 117, 110, 100, 115, 32, 111, 110, 99, 101, 32, 97, 110, 100, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 116, 104, 114, 111, 119, 115])
}

pub fn read_java_script_source_text_profile_error(profile: self::ml_string::Text, length: f64) -> self::ml_string::Text {
    if (profile != (self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]))) {
        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101])
    } else {
        if (((!(self::ml_math::is_safe_integer(length)) || (length < (0f64))) || (length > (16777216f64))) || ((length == (0f64)) && (((1f64 / length)) < (0f64)))) {
            self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 49, 54, 55, 55, 55, 50, 49, 54, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_semantic_value_error(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Null => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f43) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f44) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f45) => {
            self::ml_string::Text::from_units(vec![])
        }
        self::JavaScriptSemanticValue::Text(units) => {
            if (((units.len() as f64)) > (1048576f64)) {
                self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s47 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop33(units, at);
                        match source_ml_source2_s47.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop33Return(source_ml_source1_temporary_result33) => {
                                source_ml_source1_temporary_result33.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(profile, text) => {
            if ((self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 83, 116, 114, 105, 110, 103, 32, 118, 97, 108, 117, 101])
            } else {
                self::read_java_script_source_text_profile_error(profile.clone(), text.length())
            }
        }
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0))) && (identity <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f46) => {
            if ((identity_2 > (self::ml::Big::from_i128(0))) && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615)))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105, 110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121])
            }
        }
    }
}
