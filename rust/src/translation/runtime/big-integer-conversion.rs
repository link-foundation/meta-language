pub mod ml_big_integer_conversion {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_utf8_string_units(bytes: &[f64]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s1 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop1(bytes, index);
            match source_ml_source2_s1.clone() {
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s2 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop2(bytes, units, index_2);
                                match source_ml_source2_s2.clone() {
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
pub fn convert_java_script_string_big_integer_units(units: &[f64]) -> Vec<crate::ml::Big> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s3 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop5(units, index);
            match source_ml_source2_s3.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop6(units, start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop7(units, start_2, end);
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
                                                        let source_ml_source2_s4 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s4.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8Next(start_4, negative_3, signed_3) => {
                                                                if (start_4 == end_2) {
                                                                    Vec::<crate::ml::Big>::new()
                                                                } else {
                                                                    {
                                                                        let radix = 10f64;
                                                                        {
                                                                            let base = crate::ml::Big::from_i128(10);
                                                                            {
                                                                                let source_ml_source2_s5 = if ((!signed_3 && (((self::ml_array::optional_at(units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
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
                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9Next(start_5, radix_2, base_2.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9Next(start_4, radix, base.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9Next(start_4, radix, base.clone())
                                                                                };
                                                                                match source_ml_source2_s5.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9Next(start_6, radix_3, base_3) => {
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
                                                                                                            let source_ml_source2_s6 = self::source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop10(units, end_2, radix_3, base_3.clone(), digits, value.clone(), index_2);
                                                                                                            match source_ml_source2_s6.clone() {
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Done(value_2) => {
                                                                                                                    vec![if negative_3 {
                                                                                                                        value_2.neg()
                                                                                                                    } else {
                                                                                                                        value_2.clone()
                                                                                                                    }]
                                                                                                                }
                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Return(source_ml_source1_temporary_result10) => {
                                                                                                                    source_ml_source1_temporary_result10
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
                self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Return(source_ml_source1_temporary_result10_2) => {
                    source_ml_source1_temporary_result10_2
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
                            let integer_2 = self::source_ml_source1_temporary_convert_java_script_number_big_integer_loop11(value_2, integer.clone(), weight.clone());
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
                                let source_ml_source2_s7 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop12(remaining.clone(), shifted.clone(), weight.clone());
                                match source_ml_source2_s7.clone() {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop12Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop12Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s8 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop13(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient.clone());
                                            match source_ml_source2_s8.clone() {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop13Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop13Done(remaining_2, quotient_2) => {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_big_integer_result(kind: &str, operation: &str) -> bool {
    ((kind == "math") && ((self::read_java_script_big_integer_result_operation(operation)) != ""))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_result_expression(target: &str, form: &str, value: &str, utf16: bool) -> String {
    if (form == "boolean") {
        if (target == "JavaScript") {
            format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.convertBoolean(")), value.to_owned())), (String::from(")")))
        } else {
            if (target == "Rust") {
                format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::convert_java_script_boolean_big_integer(")), value.to_owned())), (String::from(")")))
            } else {
                if ((target == "Lean") || (target == "Rocq")) {
                    format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertBoolean ")), value.to_owned())), (String::from(")")))
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
                    format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.convertNumber(")), value.to_owned())), (String::from(")")))
                } else {
                    format!("{}{}", (format!("{}{}", (String::from("((ml_text)=>ml_big_integer_conversion.convertUnits(Array.from({length:ml_text.length}, (_,index)=>ml_text.charCodeAt(index))))(")), value.to_owned())), (String::from(")")))
                }
            } else {
                if (target == "Rust") {
                    if (form == "number") {
                        format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::convert_java_script_number_big_integer(")), value.to_owned())), (String::from(")")))
                    } else {
                        {
                            let units = if utf16 {
                                format!("{}{}", (format!("{}{}", (String::from("(")), value.to_owned())), (String::from(").0.iter().copied()")))
                            } else {
                                format!("{}{}", (format!("{}{}", (String::from("(")), value.to_owned())), (String::from(").encode_utf16()")))
                            };
                            format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::convert_java_script_string_big_integer_units(&")), units)), (String::from(".map(f64::from).collect::<Vec<f64>>())")))
                        }
                    }
                } else {
                    if (target == "Lean") {
                        if (form == "number") {
                            format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertNumber ")), value.to_owned())), (String::from(")")))
                        } else {
                            format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] ")), value.to_owned())), (String::from("))")))
                        }
                    } else {
                        if (target == "Rocq") {
                            if (form == "number") {
                                format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertNumber ")), value.to_owned())), (String::from(")")))
                            } else {
                                format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertUtf8 ((fix ml_integer_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_integer_bytes rest) end) ")), value.to_owned())), (String::from("))")))
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
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes: &[f64], mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
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
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop2(bytes: &[f64], mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let scalar = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                {
                    let index_2 = (index + 1f64);
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source2_s9 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
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
                                match source_ml_source2_s9.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s10 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop4(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s10.clone() {
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
pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop1(bytes: &[f64], mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
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
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop6(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((start < end) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index(start))).clone().unwrap_or_else(|| (-1f64)))) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop7(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((end > start) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index((end - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Done(crate::ml::Big),
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Return(Vec<crate::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop10(units: &[f64], mut end: f64, mut radix: f64, mut base: crate::ml::Big, mut digits: Vec<crate::ml::Big>, mut value: crate::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_java_script_numeric_digit_unit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop10Return(Vec::<crate::ml::Big>::new())
                } else {
                    {
                        let value_2 = (value.mul(&base)).add(&(self::ml_array::optional_at(&digits, self::ml_array::number_index(digit))).clone().unwrap_or_else(|| crate::ml::Big::from_i128(0)));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (value, index) = (value_2.clone(), index_2);
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
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin9Next(f64, f64, crate::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin8Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Done,
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Return(Vec<crate::ml::Big>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop5(units: &[f64], mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Return(Vec::<crate::ml::Big>::new())
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
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop5Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_number_big_integer_loop11(mut value: f64, mut integer: crate::ml::Big, mut weight: crate::ml::Big) -> crate::ml::Big {
    loop {
        return if (value >= (1f64)) {
            {
                let integer_3 = if (((value % 2f64)) == (1f64)) {
                    {
                        let integer_2 = integer.add(&weight);
                        integer_2.clone()
                    }
                } else {
                    integer.clone()
                };
                {
                    let value_2 = ((value / 2f64)).floor();
                    {
                        let weight_2 = weight.mul(&crate::ml::Big::from_i128(2));
                        {
                            (value, integer, weight) = (value_2, integer_3.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            integer.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop13Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop13Done(crate::ml::Big, crate::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop13(mut remaining: crate::ml::Big, mut shifted: crate::ml::Big, mut weight: crate::ml::Big, mut quotient: crate::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop13Result {
    loop {
        return if (weight > (crate::ml::Big::from_i128(1))) {
            {
                let shifted_2 = crate::ml::divide(&shifted, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = crate::ml::divide(&weight, &crate::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (shifted_2 <= remaining) {
                        {
                            let remaining_2 = remaining.sub(&shifted_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (remaining, shifted, weight, quotient) = (remaining_2.clone(), shifted_2.clone(), weight_2.clone(), quotient_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop13Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop13Done(remaining.clone(), quotient.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop12Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop12Done(crate::ml::Big, crate::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop12(mut remaining: crate::ml::Big, mut shifted: crate::ml::Big, mut weight: crate::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop12Result {
    loop {
        return if (shifted <= remaining) {
            {
                let shifted_2 = shifted.mul(&crate::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&crate::ml::Big::from_i128(2));
                    {
                        (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop12Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop12Done(shifted.clone(), weight.clone())
        };
    }
}
}
