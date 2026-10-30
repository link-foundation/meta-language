// Generated from complete js/src/translation/javascript-ordinary-heap.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin9;
use super::MlSource1TemporaryConvertStringNumberRationalJoin10;
use super::MlSource1TemporaryConvertStringNumberRationalLoop11Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop12Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop15Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop16Result;
use super::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result;
use super::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop17;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop18;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop19;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop20;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop23;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop25;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop26;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop28;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop29;
use super::source_ml_source1_temporary_convert_string_number_rational_loop11;
use super::source_ml_source1_temporary_convert_string_number_rational_loop12;
use super::source_ml_source1_temporary_convert_string_number_rational_loop13;
use super::source_ml_source1_temporary_convert_string_number_rational_loop14;
use super::source_ml_source1_temporary_convert_string_number_rational_loop15;
use super::source_ml_source1_temporary_convert_string_number_rational_loop16;
use super::source_ml_source1_temporary_read_java_script_utf8_string_units_loop1;
use super::source_ml_source1_temporary_read_java_script_utf8_string_units_loop2;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop7;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop8;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop5;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop6;

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    #[must_use]
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if (-0.5..0.0).contains(&x) {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 { floor + 1.0 } else { floor }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where `f64::signum` does not.
    #[must_use]
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            x
        }
    }

    /// Math.max of two Numbers: NaN when either is, and 0 above -0.
    #[must_use]
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() { b } else { a }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    #[must_use]
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() { a } else { b }
        } else {
            f64::NAN
        }
    }

    #[must_use]
    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    #[must_use]
    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    #[must_use]
    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    #[must_use]
    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9_007_199_254_740_991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    #[must_use]
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    #[must_use]
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        index
            .and_then(|index| values.get(index))
            .map_or_else(|| panic!("array index out of range"), Clone::clone)
    }

    /// The index a Number names: a non-negative integer, -0 included.
    #[must_use]
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9_007_199_254_740_992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    #[must_use]
    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    const fn integer(value: f64) -> f64 {
        if value.is_nan() { 0.0 } else { value.trunc() }
    }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 {
            (length as f64 + value).max(0.0) as usize
        } else {
            value.min(length as f64) as usize
        }
    }
    #[must_use]
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len());
        let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    #[must_use]
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> f64 {
        if values.is_empty() {
            return -1.0;
        }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) {
                return -1.0;
            }
            let start = if from < 0.0 {
                (values.len() as f64 + from) as usize
            } else {
                from.min((values.len() - 1) as f64) as usize
            };
            (0..=start)
                .rev()
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 {
                (values.len() as f64 + from).max(0.0) as usize
            } else {
                from.min(values.len() as f64) as usize
            };
            (start..values.len())
                .find(|&index| equal(&values[index], search))
                .map_or(-1.0, |index| index as f64)
        }
    }
    #[must_use]
    pub fn includes<T, F: Fn(&T, &T) -> bool>(
        values: &[T],
        search: &T,
        from: f64,
        reverse: bool,
        equal: F,
    ) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    #[must_use]
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values {
            if !out.iter().any(|previous| equal(previous, &value)) {
                out.push(value);
            }
        }
        out
    }
    #[must_use]
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) {
                out[index].1 = value;
            } else {
                out.push((key, value));
            }
        }
        out
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    (((((((((((9f64..=13f64).contains(&unit)) || (unit == 32f64)) || (unit == 160f64))
        || (unit == 5760f64))
        || ((8192f64..=8202f64).contains(&unit)))
        || (unit == 8232f64))
        || (unit == 8233f64))
        || (unit == 8239f64))
        || (unit == 8287f64))
        || (unit == 12_288f64))
        || (unit == 65_279f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_digit_unit(unit: f64) -> f64 {
    if (48f64..=57f64).contains(&unit) {
        unit - 48f64
    } else if (65f64..=70f64).contains(&unit) {
        unit - 55f64
    } else if (97f64..=102f64).contains(&unit) {
        unit - 87f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_utf8_string_units(bytes: &[f64]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s1 =
                self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop1(
                    bytes, index,
                );
            match source_ml_source2_s1 {
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s2 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop2(bytes, units, index_2);
                                match source_ml_source2_s2 {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Done(units_2) => {
                                        units_2
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_source1_temporary_result2) => {
                                        source_ml_source1_temporary_result2
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Return(source_ml_source1_temporary_result4) => {
                    source_ml_source1_temporary_result4
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                self::source_ml_source1_temporary_read_string_number_decimal_power_loop5(
                    exponent, value, base,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop6(
                    value.clone(),
                    result,
                    scale,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s3 =
                self::source_ml_source1_temporary_read_string_number_binary_power_loop7(
                    exponent, result,
                );
            match source_ml_source2_s3 {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop7Done(exponent_2, result_2) => {
                    {
                        self::source_ml_source1_temporary_read_string_number_binary_power_loop8(exponent_2, result_2)
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
pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if (numerator < (self::ml::Big::from_i128(0))) || (denominator <= (self::ml::Big::from_i128(0)))
    {
        f64::NAN
    } else if numerator == (self::ml::Big::from_i128(0)) {
        0f64
    } else {
        {
            let exponent = 0f64;
            {
                let source_ml_source2_s6 = if numerator >= denominator {
                    {
                        let scale = denominator.clone();
                        {
                            let source_ml_source2_s7 = self::source_ml_source1_temporary_convert_string_number_rational_loop15(numerator.clone(), exponent, scale);
                            match source_ml_source2_s7 {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Done(exponent_2) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_2)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop15Result::MlSource1TemporaryConvertStringNumberRationalLoop15Return(source_ml_source1_temporary_result15) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result15)
                                    }
                                }
                        }
                    }
                } else {
                    {
                        let scale_2 = numerator.clone();
                        {
                            let source_ml_source2_s8 = self::source_ml_source1_temporary_convert_string_number_rational_loop16(denominator.clone(), exponent, scale_2);
                            match source_ml_source2_s8 {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Done(exponent_3) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_3)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop16Result::MlSource1TemporaryConvertStringNumberRationalLoop16Return(source_ml_source1_temporary_result16) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result16)
                                    }
                                }
                        }
                    }
                };
                match source_ml_source2_s6 {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Next(exponent_4) => {
                            {
                                let shift = if exponent_4 >= -1022f64 {
                                    52f64 - exponent_4
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s9 = if shift >= 0f64 {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop13(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator_2, denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop14(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator.clone(), denominator_2)
                                            }
                                        }
                                    };
                                    match source_ml_source2_s9 {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin10::MlSource1TemporaryConvertStringNumberRationalJoin10Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale_3 = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s10 = self::source_ml_source1_temporary_convert_string_number_rational_loop11(remainder.clone(), scale_3, weight);
                                                                match source_ml_source2_s10 {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Done(scale_4, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s11 = self::source_ml_source1_temporary_convert_string_number_rational_loop12(quotient, remainder.clone(), scale_4.clone(), weight_2.clone());
                                                                            match source_ml_source2_s11 {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&self::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if (twice > denominator_3) || ((twice == denominator_3) && ((self::ml::divide(&quotient_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1)))) {
                                                                                                {
                                                                                                    quotient_2.add(&self::ml::Big::from_i128(1))
                                                                                                }
                                                                                            } else {
                                                                                                quotient_2.clone()
                                                                                            };
                                                                                            (self::read_string_number_exact_mantissa(quotient_4) * self::read_string_number_binary_power(-shift))
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
                        self::MlSource1TemporaryConvertStringNumberRationalJoin9::MlSource1TemporaryConvertStringNumberRationalJoin9Return(source_ml_source1_temporary_result14) => {
                            source_ml_source1_temporary_result14
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
pub fn accept_string_number_infinity(units: &[f64], start: f64, end: f64) -> bool {
    {
        let ml_library_inequality_1 = (end - start) != 8f64;
        if ml_library_inequality_1 {
            false
        } else {
            {
                let expected = vec![
                    73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64,
                ];
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s12 =
                            self::source_ml_source1_temporary_accept_string_number_infinity_loop17(
                                units,
                                start,
                                &(expected),
                                index,
                            );
                        match source_ml_source2_s12 {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop17Result::MlSource1TemporaryAcceptStringNumberInfinityLoop17Return(source_ml_source1_temporary_result17) => {
                            source_ml_source1_temporary_result17
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
pub fn convert_java_script_string_number_units(units: &[f64]) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s13 =
                self::source_ml_source1_temporary_convert_java_script_string_number_units_loop18(
                    units, index,
                );
            match source_ml_source2_s13 {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Done => {
                    {
                        let start = 0f64;
                        {
                            let end = units.len() as f64;
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop19(units, start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop20(units, start_2, end);
                                    if start_2 == end_2 {
                                        0f64
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(units, self::ml_array::number_index(start_2))).unwrap_or_else(|| -1f64);
                                                    {
                                                        let source_ml_source2_s14 = if (sign == 43f64) || (sign == 45f64) {
                                                            {
                                                                let negative_2 = sign == 45f64;
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = start_2 + 1f64;
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s14 {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin21Next(start_4, negative_3, signed_3) => {
                                                                if start_4 == end_2 {
                                                                    f64::NAN
                                                                } else if self::accept_string_number_infinity(units, start_4, end_2) {
                                                                        if negative_3 {
                                                                            f64::NEG_INFINITY
                                                                        } else {
                                                                            f64::INFINITY
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let integer_digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                            {
                                                                                let source_ml_source2_s16 = if (!signed_3 && (((self::ml_array::optional_at(units, self::ml_array::number_index(start_4))).unwrap_or_else(|| -1f64)) == 48f64)) && ((end_2 - start_4) >= 2f64) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(units, self::ml_array::number_index(start_4 + 1f64))).unwrap_or_else(|| -1f64);
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
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let index_2 = start_5;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s17 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units, end_2, integer_digits.clone(), radix, value, overflow, limit, base, index_2);
                                                                                                                                match source_ml_source2_s17 {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done(value_2, overflow_2) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(if overflow_2 {
                                                                                                                                            f64::INFINITY
                                                                                                                                        } else {
                                                                                                                                            self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                        })
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(source_ml_source1_temporary_result29) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(source_ml_source1_temporary_result29)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_4)
                                                                                            }
}
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s16 {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Next(start_6) => {
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
                                                                                                                let source_ml_source2_s18 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop23(units, end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s18 {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop23Done(at_2, fraction_2, digits_2, significant_2) => {
                                                                                                                        if digits_2 == 0f64 {
                                                                                                                            f64::NAN
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let decimal_end = at_2;
                                                                                                                                {
                                                                                                                                    let exponent = 0f64;
                                                                                                                                    {
                                                                                                                                        let exponent_limit = (units.len() as f64) + 10_000f64;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s21 = if (at_2 < end_2) && ((((self::ml_array::optional_at(units, self::ml_array::number_index(at_2))).unwrap_or_else(|| -1f64)) == 101f64) || (((self::ml_array::optional_at(units, self::ml_array::number_index(at_2))).unwrap_or_else(|| -1f64)) == 69f64)) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = at_2 + 1f64;
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(units, self::ml_array::number_index(at_3))).unwrap_or_else(|| -1f64);
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s22 = if (exponent_sign == 43f64) || (exponent_sign == 45f64) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = exponent_sign == 45f64;
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = at_3 + 1f64;
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s22 {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin27Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s23 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop28(units, end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s23 {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop28Done(at_6, exponent_2) => {
                                                                                                                                                                                        if at_6 == first {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(f64::NAN)
                                                                                                                                                                                        } else if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = -exponent_2;
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_6, exponent_2)
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s21 {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Next(at_7, exponent_4) => {
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
                                                                                                                                                                                    let source_ml_source2_s24 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop25(units, start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s24 {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let index_3 = start_6;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop26(units, integer_digits, mantissa_end_2, mantissa, index_3);
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
                                                                                                                                                }
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin24Return(source_ml_source1_temporary_result26) => {
                                                                                                                                                    source_ml_source1_temporary_result26
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin22Return(source_ml_source1_temporary_result28) => {
                                                                                        source_ml_source1_temporary_result28
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop18Return(source_ml_source1_temporary_result29_2) => {
                    source_ml_source1_temporary_result29_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_utf8_string_number_units(bytes: &[f64]) -> f64 {
    self::convert_java_script_string_number_units(
        &(self::read_java_script_utf8_string_units(bytes)),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_builtin_operation(name: &str) -> String {
    if name == "Number" {
        String::from("convertJavaScriptNumber")
    } else if name == "isFinite" {
        String::from("testCoercedJavaScriptNumberFinite")
    } else if name == "isNaN" {
        String::from("testCoercedJavaScriptNumberNaN")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_result(operation: &str) -> String {
    if (operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber") {
        String::from("float")
    } else if (operation == "testCoercedJavaScriptNumberFinite")
        || (operation == "testCoercedJavaScriptNumberNaN")
    {
        String::from("bool")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_conversion_form(kind: &str) -> String {
    if ((((((kind == "float") || (kind == "bool")) || (kind == "string")) || (kind == "unit"))
        || (kind == "int"))
        || (kind == "nat"))
        || (kind == "option")
    {
        kind.to_owned()
    } else {
        String::from("unsupported")
    }
}
