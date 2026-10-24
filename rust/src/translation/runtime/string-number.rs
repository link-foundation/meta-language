pub mod ml_string_number {
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

use crate::ml;

/// The Math functions that Rust's f64 methods do not share with JavaScript.
pub mod ml_math {
    /// Math.round: the nearest integer, the one towards +Infinity on a tie,
    /// with the sign of a zero result from the argument.
    pub fn round(x: f64) -> f64 {
        if !x.is_finite() || x == 0.0 {
            return x;
        }
        if x > 0.0 && x < 0.5 {
            return 0.0;
        }
        if x < 0.0 && x >= -0.5 {
            return -0.0;
        }
        let floor = x.floor();
        if x - floor >= 0.5 {
            floor + 1.0
        } else {
            floor
        }
    }

    /// Math.sign, which keeps -0, 0 and NaN, where f64::signum does not.
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
    pub fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else if b > a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                b
            } else {
                a
            }
        } else {
            f64::NAN
        }
    }

    /// Math.min of two Numbers: NaN when either is, and -0 below 0.
    pub fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else if b < a {
            b
        } else if a == b {
            if a.is_sign_negative() {
                a
            } else {
                b
            }
        } else {
            f64::NAN
        }
    }

    pub fn max_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::NEG_INFINITY, |a, &b| max(a, b))
    }

    pub fn min_of(values: &[f64]) -> f64 {
        values.iter().fold(f64::INFINITY, |a, &b| min(a, b))
    }

    pub fn is_integer(x: f64) -> bool {
        x.is_finite() && x.trunc() == x
    }

    pub fn is_safe_integer(x: f64) -> bool {
        is_integer(x) && x.abs() <= 9007199254740991.0
    }
}

/// Reads of JavaScript arrays, which the portable core never mutates.
pub mod ml_array {
    pub fn optional_at<T: Clone>(values: &[T], index: Option<usize>) -> Option<T> {
        index.and_then(|index| values.get(index)).cloned()
    }
    /// The element at an index; a read outside the array, undefined in
    /// JavaScript, aborts.
    pub fn at<T: Clone>(values: &[T], index: Option<usize>) -> T {
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
    }

    /// The index a Number names: a non-negative integer, -0 included.
    pub fn number_index(index: f64) -> Option<usize> {
        if index >= 0.0 && index.fract() == 0.0 && index < 9007199254740992.0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn append<T>(mut left: Vec<T>, right: Vec<T>) -> Vec<T> {
        left.extend(right);
        left
    }
    fn integer(value: f64) -> f64 { if value.is_nan() { 0.0 } else { value.trunc() } }
    fn relative(value: f64, length: usize) -> usize {
        let value = integer(value);
        if value < 0.0 { (length as f64 + value).max(0.0) as usize } else { value.min(length as f64) as usize }
    }
    pub fn slice<T: Clone>(values: &[T], start: f64, end: f64) -> Vec<T> {
        let start = relative(start, values.len()); let end = relative(end, values.len());
        values[start..end.max(start)].to_vec()
    }
    pub fn index_of<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> f64 {
        if values.is_empty() { return -1.0; }
        let from = integer(from);
        if reverse {
            if from < -(values.len() as f64) { return -1.0; }
            let start = if from < 0.0 { (values.len() as f64 + from) as usize } else { from.min((values.len() - 1) as f64) as usize };
            (0..=start).rev().find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        } else {
            let start = if from < 0.0 { (values.len() as f64 + from).max(0.0) as usize } else { from.min(values.len() as f64) as usize };
            (start..values.len()).find(|&index| equal(&values[index], search)).map_or(-1.0, |index| index as f64)
        }
    }
    pub fn includes<T, F: Fn(&T, &T) -> bool>(values: &[T], search: &T, from: f64, reverse: bool, equal: F) -> bool {
        index_of(values, search, from, reverse, equal) >= 0.0
    }
    pub fn set<T, F: Fn(&T, &T) -> bool>(values: Vec<T>, equal: F) -> Vec<T> {
        let mut out = Vec::new();
        for value in values { if !out.iter().any(|previous| equal(previous, &value)) { out.push(value); } }
        out
    }
    pub fn map<K, V, F: Fn(&K, &K) -> bool>(values: Vec<(K, V)>, equal: F) -> Vec<(K, V)> {
        let mut out: Vec<(K, V)> = Vec::new();
        for (key, value) in values {
            if let Some(index) = out.iter().position(|(previous, _)| equal(previous, &key)) { out[index].1 = value; }
            else { out.push((key, value)); }
        }
        out
    }
}

pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

pub fn read_string_number_digit(unit: f64) -> f64 {
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

pub fn read_string_number_decimal_power(exponent: f64) -> crate::ml::Big {
    {
        let value = crate::ml::Big::from_i128(1);
        {
            let base = crate::ml::Big::from_i128(10);
            {
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop1(exponent, value.clone(), base.clone());
                value_2.clone()
            }
        }
    }
}

pub fn read_string_number_exact_mantissa(value: crate::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop2(value.clone(), result, scale);
                result_2
            }
        }
    }
}

pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s1 = self::source_ml_source1_temporary_read_string_number_binary_power_loop3(exponent, result);
            match source_ml_source2_s1.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop3Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop3Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop4(exponent_2, result_2);
                        result_3
                    }
                }
            }
        }
    }
}

pub fn convert_string_number_rational(numerator: crate::ml::Big, denominator: crate::ml::Big) -> f64 {
    if ((numerator < (crate::ml::Big::from_i128(0))) || (denominator <= (crate::ml::Big::from_i128(0)))) {
        f64::NAN
    } else {
        if (numerator == (crate::ml::Big::from_i128(0))) {
            0f64
        } else {
            {
                let exponent = 0f64;
                {
                    let source_ml_source2_s4 = if (numerator >= denominator) {
                        {
                            let scale = denominator.clone();
                            {
                                let source_ml_source2_s5 = self::source_ml_source1_temporary_convert_string_number_rational_loop11(numerator.clone(), exponent, scale.clone());
                                match source_ml_source2_s5.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Done(exponent_2) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin5::MlSource1TemporaryConvertStringNumberRationalJoin5Next(exponent_2)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Return(source_ml_source1_temporary_result11) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin5::MlSource1TemporaryConvertStringNumberRationalJoin5Return(source_ml_source1_temporary_result11)
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let scale_2 = numerator.clone();
                            {
                                let source_ml_source2_s6 = self::source_ml_source1_temporary_convert_string_number_rational_loop12(denominator.clone(), exponent, scale_2.clone());
                                match source_ml_source2_s6.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(exponent_3) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin5::MlSource1TemporaryConvertStringNumberRationalJoin5Next(exponent_3)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Return(source_ml_source1_temporary_result12) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin5::MlSource1TemporaryConvertStringNumberRationalJoin5Return(source_ml_source1_temporary_result12)
                                    }
                                }
                            }
                        }
                    };
                    match source_ml_source2_s4.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin5::MlSource1TemporaryConvertStringNumberRationalJoin5Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s7 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop9(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin6::MlSource1TemporaryConvertStringNumberRationalJoin6Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop10(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin6::MlSource1TemporaryConvertStringNumberRationalJoin6Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s7.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin6::MlSource1TemporaryConvertStringNumberRationalJoin6Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = crate::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale_3 = denominator_3.clone();
                                                        {
                                                            let weight = crate::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s8 = self::source_ml_source1_temporary_convert_string_number_rational_loop7(remainder.clone(), scale_3.clone(), weight.clone());
                                                                match source_ml_source2_s8.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop7Result::MlSource1TemporaryConvertStringNumberRationalLoop7Done(scale_4, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s9 = self::source_ml_source1_temporary_convert_string_number_rational_loop8(quotient.clone(), remainder.clone(), scale_4.clone(), weight_2.clone());
                                                                            match source_ml_source2_s9.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop8Result::MlSource1TemporaryConvertStringNumberRationalLoop8Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&crate::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if ((twice > denominator_3) || ((twice == denominator_3) && ((crate::ml::divide(&quotient_2, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (crate::ml::Big::from_i128(1))))) {
                                                                                                {
                                                                                                    let quotient_3 = quotient_2.add(&crate::ml::Big::from_i128(1));
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
                        self::MlSource1TemporaryConvertStringNumberRationalJoin5::MlSource1TemporaryConvertStringNumberRationalJoin5Return(source_ml_source1_temporary_result10) => {
                            source_ml_source1_temporary_result10
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
                    let source_ml_source2_s10 = self::source_ml_source1_temporary_accept_string_number_infinity_loop13(units, start, expected, index);
                    match source_ml_source2_s10.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop13Result::MlSource1TemporaryAcceptStringNumberInfinityLoop13Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop13Result::MlSource1TemporaryAcceptStringNumberInfinityLoop13Return(source_ml_source1_temporary_result13) => {
                            source_ml_source1_temporary_result13
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
            let source_ml_source2_s11 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop14(units.clone(), index);
            match source_ml_source2_s11.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop15(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop16(units.clone(), start_2, end);
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
                                                        let source_ml_source2_s12 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s12.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17Next(start_4, negative_3, signed_3) => {
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
                                                                            let integer_digits = vec![crate::ml::Big::from_i128(0), crate::ml::Big::from_i128(1), crate::ml::Big::from_i128(2), crate::ml::Big::from_i128(3), crate::ml::Big::from_i128(4), crate::ml::Big::from_i128(5), crate::ml::Big::from_i128(6), crate::ml::Big::from_i128(7), crate::ml::Big::from_i128(8), crate::ml::Big::from_i128(9), crate::ml::Big::from_i128(10), crate::ml::Big::from_i128(11), crate::ml::Big::from_i128(12), crate::ml::Big::from_i128(13), crate::ml::Big::from_i128(14), crate::ml::Big::from_i128(15)];
                                                                            {
                                                                                let source_ml_source2_s14 = if ((!signed_3 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
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
                                                                                                    let value = crate::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let overflow = false;
                                                                                                        {
                                                                                                            let limit = self::read_string_number_decimal_power(310f64);
                                                                                                            {
                                                                                                                let base = if (radix == (16f64)) {
                                                                                                                    crate::ml::Big::from_i128(16)
                                                                                                                } else {
                                                                                                                    if (radix == (8f64)) {
                                                                                                                        crate::ml::Big::from_i128(8)
                                                                                                                    } else {
                                                                                                                        crate::ml::Big::from_i128(2)
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let start_5 = (start_4 + 2f64);
                                                                                                                    if (start_5 == end_2) {
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let index_2 = start_5;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s15 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop25(units.clone(), end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), index_2);
                                                                                                                                match source_ml_source2_s15.clone() {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(value_2, overflow_2) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(if overflow_2 {
                                                                                                                                            f64::INFINITY
                                                                                                                                        } else {
                                                                                                                                            self::convert_string_number_rational(value_2.clone(), crate::ml::Big::from_i128(1))
                                                                                                                                        })
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Return(source_ml_source1_temporary_result25) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(source_ml_source1_temporary_result25)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s14.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Next(start_6) => {
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
                                                                                                                let source_ml_source2_s16 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop19(units.clone(), end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s16.clone() {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Done(at_2, fraction_2, digits_2, significant_2) => {
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
                                                                                                                                            let source_ml_source2_s19 = if ((at_2 < end_2) && ((((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || (((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = (at_2 + 1f64);
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s20 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = (at_3 + 1f64);
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s20.clone() {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s21 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop24(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s21.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Done(at_6, exponent_2) => {
                                                                                                                                                                                        if (at_6 == first) {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Return(f64::NAN)
                                                                                                                                                                                        } else {
                                                                                                                                                                                            if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = (-exponent_2);
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(at_6, exponent_2)
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s19.clone() {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(at_7, exponent_4) => {
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
                                                                                                                                                                                    let source_ml_source2_s22 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop21(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s22.clone() {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = crate::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let index_3 = start_6;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop22(units, integer_digits, mantissa_end_2, mantissa.clone(), index_3);
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
                                                                                                                                                                                                                    crate::ml::Big::from_i128(1)
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Return(source_ml_source1_temporary_result22) => {
                                                                                                                                                    source_ml_source1_temporary_result22
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(source_ml_source1_temporary_result24) => {
                                                                                        source_ml_source1_temporary_result24
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Return(source_ml_source1_temporary_result25_2) => {
                    source_ml_source1_temporary_result25_2
                }
            }
        }
    }
}

pub fn convert_java_script_big_integer_number(value: crate::ml::Big) -> f64 {
    if (value < (crate::ml::Big::from_i128(0))) {
        (-(self::convert_string_number_rational(value.neg(), crate::ml::Big::from_i128(1))))
    } else {
        self::convert_string_number_rational(value.clone(), crate::ml::Big::from_i128(1))
    }
}

pub fn convert_java_script_utf8_string_number_units(bytes: Vec<f64>) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s23 = self::source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop26(bytes.clone(), index);
            match source_ml_source2_s23.clone() {
                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s24 = self::source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop27(bytes, units, index_2);
                                match source_ml_source2_s24.clone() {
                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Done(units_2) => {
                                        self::convert_java_script_string_number_units(units_2)
                                    }
                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(source_ml_source1_temporary_result27) => {
                                        source_ml_source1_temporary_result27
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Return(source_ml_source1_temporary_result29) => {
                    source_ml_source1_temporary_result29
                }
            }
        }
    }
}

pub fn read_java_script_number_builtin_operation(name: String) -> String {
    if (name == "Number") {
        String::from("convertJavaScriptNumber")
    } else {
        if (name == "isFinite") {
            String::from("testCoercedJavaScriptNumberFinite")
        } else {
            if (name == "isNaN") {
                String::from("testCoercedJavaScriptNumberNaN")
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_java_script_number_conversion_result(operation: String) -> String {
    if ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber")) {
        String::from("float")
    } else {
        if ((operation == "testCoercedJavaScriptNumberFinite") || (operation == "testCoercedJavaScriptNumberNaN")) {
            String::from("bool")
        } else {
            String::from("")
        }
    }
}

pub fn read_java_script_number_conversion_form(kind: String) -> String {
    if (((((((kind == "float") || (kind == "bool")) || (kind == "string")) || (kind == "unit")) || (kind == "int")) || (kind == "nat")) || (kind == "option")) {
        kind.clone()
    } else {
        String::from("unsupported")
    }
}

pub fn accept_linked_java_script_number_conversion(kind: String, operation: String) -> bool {
    ((kind == "math") && ((operation == "convertJavaScriptNumber") || (operation == "coerceJavaScriptNumber")))
}

pub fn read_java_script_number_conversion_failure(operation: String, kind: String) -> String {
    if ((operation == "coerceJavaScriptNumber") && ((kind == "int") || (kind == "nat"))) {
        String::from("Cannot convert a BigInt value to a number")
    } else {
        String::from("")
    }
}

pub fn require_java_script_number_conversion_units(kind: String, operation: String, argument: String) -> bool {
    (self::accept_linked_java_script_number_conversion(kind.clone(), operation.clone()) && (argument == "string"))
}

pub fn render_java_script_number_scalar_conversion(format: String, operation: String, form: String, value: String, utf16: bool) -> String {
    if ((!(self::accept_linked_java_script_number_conversion(String::from("math"), operation.clone())) || ((self::read_java_script_number_conversion_form(form.clone())) == "unsupported")) || (form == "option")) {
        String::from("")
    } else {
        if (format == "JavaScript") {
            if (operation == "convertJavaScriptNumber") {
                format!("{}{}", (format!("{}{}", (String::from("Number(")), value)), (String::from(")")))
            } else {
                format!("{}{}", (format!("{}{}", (String::from("(+(")), value)), (String::from("))")))
            }
        } else {
            if ((format == "Lean") || (format == "Rocq")) {
                if !(self::read_java_script_number_conversion_failure(operation.clone(), form.clone())).is_empty() {
                    String::from("")
                } else {
                    if (form == "float") {
                        value.clone()
                    } else {
                        if (form == "unit") {
                            if (format == "Lean") {
                                format!("{}{}", (format!("{}{}", (String::from("(let _ := ")), value)), (String::from("; ((0 : Float) / 0))")))
                            } else {
                                format!("{}{}", (format!("{}{}", (String::from("(let _ := ")), value)), (String::from(" in PrimFloat.nan)")))
                            }
                        } else {
                            if (form == "bool") {
                                if (format == "Lean") {
                                    format!("{}{}", (format!("{}{}", (String::from("(if ")), value)), (String::from(" then (1 : Float) else (0 : Float))")))
                                } else {
                                    format!("{}{}", (format!("{}{}", (String::from("(if ")), value)), (String::from(" then 1%float else 0%float)")))
                                }
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertInteger ")), (if (form == "nat") {
                                        if (format == "Lean") {
                                            format!("{}{}", (format!("{}{}", (String::from("(Int.ofNat ")), value)), (String::from(")")))
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(Z.of_N ")), value)), (String::from(")")))
                                        }
                                    } else {
                                        value.clone()
                                    }))), (String::from(")")))
                                } else {
                                    if (format == "Lean") {
                                        format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] ")), value)), (String::from("))")))
                                    } else {
                                        format!("{}{}", (format!("{}{}", (String::from("(MLStringNumber.convertUtf8 ((fix ml_numeric_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_numeric_bytes rest) end) ")), value)), (String::from("))")))
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
                        value.clone()
                    } else {
                        if (form == "bool") {
                            format!("{}{}", (format!("{}{}", (String::from("(if ")), value)), (String::from(" { 1.0f64 } else { 0.0f64 })")))
                        } else {
                            if (form == "unit") {
                                format!("{}{}", (format!("{}{}", (String::from("{ let _ = ")), value)), (String::from("; f64::NAN }")))
                            } else {
                                if ((form == "int") || (form == "nat")) {
                                    if (operation == "coerceJavaScriptNumber") {
                                        format!("{}{}", (format!("{}{}", (String::from("{ let _ = ")), value)), (String::from("; panic!(\"Cannot convert a BigInt value to a number\") }")))
                                    } else {
                                        format!("{}{}", (format!("{}{}", (String::from("crate::ml_string_number::convert_java_script_big_integer_number(")), value)), (String::from(")")))
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            format!("{}{}", (format!("{}{}", (String::from("(")), value)), (String::from(").0.iter().copied()")))
                                        } else {
                                            format!("{}{}", (format!("{}{}", (String::from("(")), value)), (String::from(").encode_utf16()")))
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

pub fn read_java_script_number_conversion_statement() -> String {
    String::from("ECMAScript Number and ToNumber preserve full UTF-16 StringNumericLiteral syntax, exact binary64 ties-to-even rounding, signed zero, Infinity, NaN, primitive absence and argument evaluation order; explicit BigInt conversion rounds once and implicit BigInt conversion throws")
}

pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop1(mut exponent: f64, mut value: crate::ml::Big, mut base: crate::ml::Big) -> crate::ml::Big {
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

pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop2(mut value: crate::ml::Big, mut result: f64, mut scale: f64) -> f64 {
    loop {
        return if (value > (crate::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((crate::ml::divide(&value, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (crate::ml::Big::from_i128(1))) {
                    {
                        let result_2 = (result + scale);
                        result_2
                    }
                } else {
                    result
                };
                {
                    let value_2 = crate::ml::divide(&value, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
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

pub fn source_ml_source1_temporary_read_string_number_binary_power_loop4(mut exponent: f64, mut result: f64) -> f64 {
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
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop3Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop3Done(f64, f64),
}

pub fn source_ml_source1_temporary_read_string_number_binary_power_loop3(mut exponent: f64, mut result: f64) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop3Result {
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
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop3Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop3Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop8Result {
    MlSource1TemporaryConvertStringNumberRationalLoop8Done(crate::ml::Big, crate::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop8(mut quotient: crate::ml::Big, mut remainder: crate::ml::Big, mut scale: crate::ml::Big, mut weight: crate::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop8Result {
    loop {
        return if (weight > (crate::ml::Big::from_i128(1))) {
            {
                let scale_2 = crate::ml::divide(&scale, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = crate::ml::divide(&weight, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
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
            self::MlSource1TemporaryConvertStringNumberRationalLoop8Result::MlSource1TemporaryConvertStringNumberRationalLoop8Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop7Result {
    MlSource1TemporaryConvertStringNumberRationalLoop7Done(crate::ml::Big, crate::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop7(mut remainder: crate::ml::Big, mut scale: crate::ml::Big, mut weight: crate::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop7Result {
    loop {
        return if (scale <= remainder) {
            {
                let scale_2 = scale.mul(&crate::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&crate::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop7Result::MlSource1TemporaryConvertStringNumberRationalLoop7Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin6 {
    MlSource1TemporaryConvertStringNumberRationalJoin6Next(crate::ml::Big, crate::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop9(mut numerator: crate::ml::Big, mut shift: f64, mut index: f64) -> crate::ml::Big {
    loop {
        return if (index < shift) {
            {
                let numerator_2 = numerator.mul(&crate::ml::Big::from_i128(2));
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

pub fn source_ml_source1_temporary_convert_string_number_rational_loop10(mut denominator: crate::ml::Big, mut shift: f64, mut index: f64) -> crate::ml::Big {
    loop {
        return if (index < ((-shift))) {
            {
                let denominator_2 = denominator.mul(&crate::ml::Big::from_i128(2));
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
pub enum MlSource1TemporaryConvertStringNumberRationalJoin5 {
    MlSource1TemporaryConvertStringNumberRationalJoin5Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin5Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop11Result {
    MlSource1TemporaryConvertStringNumberRationalLoop11Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop11Return(f64),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop11(mut numerator: crate::ml::Big, mut exponent: f64, mut scale: crate::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop11Result {
    loop {
        return if (numerator >= (scale.mul(&crate::ml::Big::from_i128(2)))) {
            {
                let scale_2 = scale.mul(&crate::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent + 1f64);
                    if (exponent_2 >= (1024f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop11Result::MlSource1TemporaryConvertStringNumberRationalLoop11Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop12Result {
    MlSource1TemporaryConvertStringNumberRationalLoop12Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop12Return(f64),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop12(mut denominator: crate::ml::Big, mut exponent: f64, mut scale: crate::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop12Result {
    loop {
        return if (scale < denominator) {
            {
                let scale_2 = scale.mul(&crate::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent - 1f64);
                    if (exponent_2 < ((-1075f64))) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop12Result::MlSource1TemporaryConvertStringNumberRationalLoop12Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop13Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop13Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop13Return(bool),
}

pub fn source_ml_source1_temporary_accept_string_number_infinity_loop13(mut units: Vec<f64>, mut start: f64, mut expected: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop13Result {
    loop {
        return if (index < ((expected.len() as f64))) {
            if (((self::ml_array::optional_at(&units, self::ml_array::number_index((start + index)))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(&expected, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-2f64)))) {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop13Result::MlSource1TemporaryAcceptStringNumberInfinityLoop13Return(false)
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
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop13Result::MlSource1TemporaryAcceptStringNumberInfinityLoop13Done
        };
    }
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop15(mut units: Vec<f64>, mut start: f64, mut end: f64) -> f64 {
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

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop16(mut units: Vec<f64>, mut start: f64, mut end: f64) -> f64 {
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

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop22(mut units: Vec<f64>, mut integer_digits: Vec<crate::ml::Big>, mut mantissa_end: f64, mut mantissa: crate::ml::Big, mut index: f64) -> crate::ml::Big {
    loop {
        return if (index < mantissa_end) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (unit != (46f64)) {
                    {
                        let mantissa_2 = (mantissa.mul(&crate::ml::Big::from_i128(10))).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index((unit - 48f64)))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)));
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
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done(f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop21(mut units: Vec<f64>, mut start: f64, mut exponent: f64, mut mantissa_end: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result {
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
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done(exponent, mantissa_end)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Done(f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop24(mut units: Vec<f64>, mut end: f64, mut at: f64, mut exponent: f64, mut exponent_limit: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result {
    loop {
        return if (at < end) {
            {
                let digit = ((self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)) - 48f64);
                if ((digit < (0f64)) || (digit > (9f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Done(at, exponent)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Done(f64, f64, f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop19(mut units: Vec<f64>, mut end: f64, mut at: f64, mut fraction: f64, mut digits: f64, mut significant: f64, mut dot: bool) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result {
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
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(crate::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop25(mut units: Vec<f64>, mut end: f64, mut integer_digits: Vec<crate::ml::Big>, mut radix: f64, mut value: crate::ml::Big, mut overflow: bool, mut limit: crate::ml::Big, mut base: crate::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit((self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Return(f64::NAN)
                } else {
                    if !overflow {
                        {
                            let value_2 = (value.mul(&base)).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index(digit))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)));
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop14(mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result {
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Done(Vec<f64>),
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result {
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Done(f64, f64),
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop29(mut bytes: Vec<f64>, mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(&bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28 {
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(f64, f64, f64),
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop27(mut bytes: Vec<f64>, mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result {
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
                                let source_ml_source2_s25 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar_2, remaining_2, minimum_2)
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
                                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar_3, remaining_3, minimum_3)
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
                                                        self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Return(f64::NAN)
                                            } else {
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source2_s25.clone() {
                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s26 = self::source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop29(bytes.clone(), index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s26.clone() {
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(f64::NAN)
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
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Return(source_ml_source1_temporary_result29) => {
                                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(source_ml_source1_temporary_result29)
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Return(source_ml_source1_temporary_result29_2) => {
                                        self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(source_ml_source1_temporary_result29_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result {
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Done,
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop26(mut bytes: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Done
        };
    }
}
}
