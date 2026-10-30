pub mod ml_utf16_text {
#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]

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
pub fn read_java_script_utf8_string_units(bytes: &[f64]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source1_s1 = self::source_ml_temporary_read_java_script_utf8_string_units_loop1(bytes, index);
            match source_ml_source1_s1.clone() {
                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source1_s2 = self::source_ml_temporary_read_java_script_utf8_string_units_loop2(bytes, units, index_2);
                                match source_ml_source1_s2.clone() {
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Done(units_2) => {
                                        units_2
                                    }
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_temporary_result2) => {
                                        source_ml_temporary_result2
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Return(source_ml_temporary_result4) => {
                    source_ml_temporary_result4
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_unsigned_integer(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_integer(value: f64) -> f64 {
    if value.is_nan() {
        0f64
    } else {
        value.trunc()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_slice_position(value: f64, length: f64) -> f64 {
    {
        let integer = self::read_utf16_text_integer(value);
        if (integer < (0f64)) {
            self::ml_math::max((length + integer), 0f64)
        } else {
            self::ml_math::min(integer, length)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn slice_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::read_utf16_text_slice_position(start, (text.len() as f64));
        {
            let last = self::read_utf16_text_slice_position(end, (text.len() as f64));
            {
                let result = Vec::<f64>::new();
                {
                    let at = first;
                    {
                        let result_2 = self::source_ml_temporary_slice_utf16_text_loop5(text, last, result, at);
                        result_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn substring_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(start), 0f64), (text.len() as f64));
        {
            let last = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(end), 0f64), (text.len() as f64));
            self::slice_utf16_text(text, self::ml_math::min(first, last), self::ml_math::max(first, last))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn relative_character_utf16_text(text: &[f64], position: f64) -> Vec<Vec<f64>> {
    {
        let integer = self::read_utf16_text_integer(position);
        {
            let at = if (integer < (0f64)) {
                ((text.len() as f64) + integer)
            } else {
                integer
            };
            if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
                Vec::<Vec<f64>>::new()
            } else {
                vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    vec![if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        (((65536f64 + ((first - 55296f64) * 1024f64)) + second) - 56320f64)
                    } else {
                        first
                    }]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn points_utf16_text(text: &[f64]) -> Vec<Vec<f64>> {
    {
        let result = Vec::<Vec<f64>>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_points_utf16_text_loop6(text, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn split_utf16_text(text: &[f64], separator: &[f64], limit: f64, missing: bool) -> Vec<Vec<f64>> {
    {
        let count = self::normalize_java_script_unsigned_integer(limit);
        {
            let result = Vec::<Vec<f64>>::new();
            if (count == (0f64)) {
                result
            } else {
                if missing {
                    vec![self::slice_utf16_text(text, 0f64, (text.len() as f64))]
                } else {
                    if (((separator.len() as f64)) == (0f64)) {
                        {
                            let at = 0f64;
                            {
                                let result_2 = self::source_ml_temporary_split_utf16_text_loop7(text, count, result, at);
                                result_2
                            }
                        }
                    } else {
                        {
                            let start = 0f64;
                            {
                                let source_ml_source1_s3 = self::source_ml_temporary_split_utf16_text_loop8(text, separator, count, result, start);
                                match source_ml_source1_s3.clone() {
                                    self::MlTemporarySplitUtf16TextLoop8Result::MlTemporarySplitUtf16TextLoop8Done(result_3, start_2) => {
                                        {
                                            let result_4 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start_2, (text.len() as f64))], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                            result_4
                                        }
                                    }
                                    self::MlTemporarySplitUtf16TextLoop8Result::MlTemporarySplitUtf16TextLoop8Return(source_ml_temporary_result8) => {
                                        source_ml_temporary_result8
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
pub fn match_utf16_text(text: &[f64], search: &[f64], start: f64) -> bool {
    if ((start < (0f64)) || (((start + (search.len() as f64))) > ((text.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source1_s4 = self::source_ml_temporary_match_utf16_text_loop9(text, search, start, at);
                match source_ml_source1_s4.clone() {
                    self::MlTemporaryMatchUtf16TextLoop9Result::MlTemporaryMatchUtf16TextLoop9Done => {
                        true
                    }
                    self::MlTemporaryMatchUtf16TextLoop9Result::MlTemporaryMatchUtf16TextLoop9Return(source_ml_temporary_result9) => {
                        source_ml_temporary_result9
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn index_utf16_text(text: &[f64], search: &[f64], position: f64) -> f64 {
    {
        let start = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(position), 0f64), (text.len() as f64));
        {
            let at = start;
            {
                let source_ml_source1_s5 = self::source_ml_temporary_index_utf16_text_loop10(text, search, at);
                match source_ml_source1_s5.clone() {
                    self::MlTemporaryIndexUtf16TextLoop10Result::MlTemporaryIndexUtf16TextLoop10Done => {
                        (-1f64)
                    }
                    self::MlTemporaryIndexUtf16TextLoop10Result::MlTemporaryIndexUtf16TextLoop10Return(source_ml_temporary_result10) => {
                        source_ml_temporary_result10
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn last_index_utf16_text(text: &[f64], search: &[f64], position: f64) -> f64 {
    {
        let integer = if position.is_nan() {
            f64::INFINITY
        } else {
            self::read_utf16_text_integer(position)
        };
        {
            let start = self::ml_math::min(self::ml_math::max(integer, 0f64), ((text.len() as f64) - (search.len() as f64)));
            {
                let at = start;
                {
                    let source_ml_source1_s6 = self::source_ml_temporary_last_index_utf16_text_loop11(text, search, at);
                    match source_ml_source1_s6.clone() {
                        self::MlTemporaryLastIndexUtf16TextLoop11Result::MlTemporaryLastIndexUtf16TextLoop11Done => {
                            (-1f64)
                        }
                        self::MlTemporaryLastIndexUtf16TextLoop11Result::MlTemporaryLastIndexUtf16TextLoop11Return(source_ml_temporary_result11) => {
                            source_ml_temporary_result11
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_utf16_text(left: &[f64], right: &[f64]) -> f64 {
    {
        let count = self::ml_math::min((left.len() as f64), (right.len() as f64));
        {
            let at = 0f64;
            {
                let source_ml_source1_s7 = self::source_ml_temporary_compare_utf16_text_loop12(left, right, count, at);
                match source_ml_source1_s7.clone() {
                    self::MlTemporaryCompareUtf16TextLoop12Result::MlTemporaryCompareUtf16TextLoop12Done => {
                        if (((left.len() as f64)) == ((right.len() as f64))) {
                            0f64
                        } else {
                            if (((left.len() as f64)) < ((right.len() as f64))) {
                                (-1f64)
                            } else {
                                1f64
                            }
                        }
                    }
                    self::MlTemporaryCompareUtf16TextLoop12Result::MlTemporaryCompareUtf16TextLoop12Return(source_ml_temporary_result12) => {
                        source_ml_temporary_result12
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn starts_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ends_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, ((text.len() as f64) - (search.len() as f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn includes_utf16_text(text: &[f64], search: &[f64]) -> bool {
    ((self::index_utf16_text(text, search, 0f64)) >= (0f64))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn join_utf16_text(values: &[Vec<f64>], separator: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_join_utf16_text_loop13(values, separator, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn trim_utf16_text(text: &[f64], start: bool, end: bool) -> Vec<f64> {
    {
        let first = 0f64;
        {
            let last = (text.len() as f64);
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_temporary_trim_utf16_text_loop16(text, first, last);
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_temporary_trim_utf16_text_loop17(text, first_3, last);
                            last_2
                        }
                    } else {
                        last
                    };
                    self::slice_utf16_text(text, first_3, last_3)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_units_utf16_text(values: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_character_units_utf16_text_loop18(values, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_units_utf16_text(values: &[f64]) -> Vec<f64> {
    {
        let result = vec![(-1f64)];
        {
            let at = 0f64;
            {
                let source_ml_source1_s8 = self::source_ml_temporary_code_point_units_utf16_text_loop19(values, result, at);
                match source_ml_source1_s8.clone() {
                    self::MlTemporaryCodePointUnitsUtf16TextLoop19Result::MlTemporaryCodePointUnitsUtf16TextLoop19Done(result_2) => {
                        result_2
                    }
                    self::MlTemporaryCodePointUnitsUtf16TextLoop19Result::MlTemporaryCodePointUnitsUtf16TextLoop19Return(source_ml_temporary_result19) => {
                        source_ml_temporary_result19
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf8_text_units(bytes: &[f64]) -> Vec<f64> {
    self::read_java_script_utf8_string_units(bytes)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn display_points_utf16_text(text: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_temporary_display_points_utf16_text_loop20(text, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn display_utf8_from_utf16_text(text: &[f64]) -> Vec<f64> {
    {
        let bytes = Vec::<f64>::new();
        {
            let points = self::display_points_utf16_text(text);
            {
                let at = 0f64;
                {
                    let bytes_2 = self::source_ml_temporary_display_utf8_from_utf16_text_loop22(bytes, &(points), at);
                    bytes_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_utf16_text_substitution(replacement: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let start = 0f64;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s9 = self::source_ml_temporary_plan_utf16_text_substitution_loop23(replacement, result, start, at);
                    match source_ml_source1_s9.clone() {
                        self::MlTemporaryPlanUtf16TextSubstitutionLoop23Result::MlTemporaryPlanUtf16TextSubstitutionLoop23Done(result_2, start_2) => {
                            {
                                let result_4 = if (start_2 < ((replacement.len() as f64))) {
                                    {
                                        let result_3 = { let (ml_append_right, ml_append_left) = (vec![0f64, start_2, (replacement.len() as f64)], result_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                        result_3
                                    }
                                } else {
                                    result_2
                                };
                                result_4
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
pub fn replace_utf16_text(text: &[f64], search: &[f64], replacement: &[f64], all: bool) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let plan = self::plan_utf16_text_substitution(replacement);
            {
                let start = 0f64;
                {
                    let from = 0f64;
                    {
                        let searching = true;
                        {
                            let source_ml_source1_s10 = self::source_ml_temporary_replace_utf16_text_loop24(text, search, replacement, all, result, &(plan), start, from, searching);
                            match source_ml_source1_s10.clone() {
                                self::MlTemporaryReplaceUtf16TextLoop24Result::MlTemporaryReplaceUtf16TextLoop24Done(result_2, start_2) => {
                                    {
                                        let at = start_2;
                                        {
                                            let result_3 = self::source_ml_temporary_replace_utf16_text_loop25(text, result_2, at);
                                            result_3
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Done(Vec<f64>),
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done(f64, f64),
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop4(bytes: &[f64], mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(vec![(-1f64)])
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
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsJoin3 {
    MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(f64, f64, f64),
    MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop2(bytes: &[f64], mut units: Vec<f64>, mut index: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result {
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
                                let source_ml_source1_s11 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_2, remaining_2, minimum_2)
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
                                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_3, remaining_3, minimum_3)
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
                                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Return(vec![(-1f64)])
                                            } else {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source1_s11.clone() {
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source1_s12 = self::source_ml_temporary_read_java_script_utf8_string_units_loop4(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source1_s12.clone() {
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(vec![(-1f64)])
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
                                                self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop4Return(source_ml_temporary_result4) => {
                                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_temporary_result4)
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3::MlTemporaryReadJavaScriptUtf8StringUnitsJoin3Return(source_ml_temporary_result4_2) => {
                                        self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Return(source_ml_temporary_result4_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop2Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Done,
    MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_utf8_string_units_loop1(bytes: &[f64], mut index: f64) -> self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result {
    loop {
        return if (index < ((bytes.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Return(vec![(-1f64)])
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
            self::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Result::MlTemporaryReadJavaScriptUtf8StringUnitsLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_slice_utf16_text_loop5(text: &[f64], mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
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
pub fn source_ml_temporary_points_utf16_text_loop6(text: &[f64], mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![first, second]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    let at_3 = (at_2 + 1f64);
                                    {
                                        (result, at) = (result_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![vec![first]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_4 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_4);
                                    continue;
                                }
                            }
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
pub fn source_ml_temporary_split_utf16_text_loop7(text: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if ((at < ((text.len() as f64))) && (((result.len() as f64)) < count)) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
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
pub enum MlTemporarySplitUtf16TextLoop8Result {
    MlTemporarySplitUtf16TextLoop8Done(Vec<Vec<f64>>, f64),
    MlTemporarySplitUtf16TextLoop8Return(Vec<Vec<f64>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_split_utf16_text_loop8(text: &[f64], separator: &[f64], mut count: f64, mut result: Vec<Vec<f64>>, mut start: f64) -> self::MlTemporarySplitUtf16TextLoop8Result {
    loop {
        return {
            let found = self::index_utf16_text(text, separator, start);
            if (found < (0f64)) {
                self::MlTemporarySplitUtf16TextLoop8Result::MlTemporarySplitUtf16TextLoop8Done(result, start)
            } else {
                {
                    let result_2 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start, found)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                    if (((result_2.len() as f64)) == count) {
                        self::MlTemporarySplitUtf16TextLoop8Result::MlTemporarySplitUtf16TextLoop8Return(result_2)
                    } else {
                        {
                            let start_2 = (found + (separator.len() as f64));
                            {
                                (result, start) = (result_2, start_2);
                                continue;
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryMatchUtf16TextLoop9Result {
    MlTemporaryMatchUtf16TextLoop9Done,
    MlTemporaryMatchUtf16TextLoop9Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_match_utf16_text_loop9(text: &[f64], search: &[f64], mut start: f64, mut at: f64) -> self::MlTemporaryMatchUtf16TextLoop9Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source1_comparison_operand13 = self::ml_array::optional_at(text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source1_comparison_operand14 = self::ml_array::optional_at(search, self::ml_array::number_index(at));
                    !(match (source_ml_source1_comparison_operand13.clone(), source_ml_source1_comparison_operand14.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlTemporaryMatchUtf16TextLoop9Result::MlTemporaryMatchUtf16TextLoop9Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryMatchUtf16TextLoop9Result::MlTemporaryMatchUtf16TextLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryIndexUtf16TextLoop10Result {
    MlTemporaryIndexUtf16TextLoop10Done,
    MlTemporaryIndexUtf16TextLoop10Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_index_utf16_text_loop10(text: &[f64], search: &[f64], mut at: f64) -> self::MlTemporaryIndexUtf16TextLoop10Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryIndexUtf16TextLoop10Result::MlTemporaryIndexUtf16TextLoop10Return(at)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryIndexUtf16TextLoop10Result::MlTemporaryIndexUtf16TextLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLastIndexUtf16TextLoop11Result {
    MlTemporaryLastIndexUtf16TextLoop11Done,
    MlTemporaryLastIndexUtf16TextLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_last_index_utf16_text_loop11(text: &[f64], search: &[f64], mut at: f64) -> self::MlTemporaryLastIndexUtf16TextLoop11Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryLastIndexUtf16TextLoop11Result::MlTemporaryLastIndexUtf16TextLoop11Return(at)
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryLastIndexUtf16TextLoop11Result::MlTemporaryLastIndexUtf16TextLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareUtf16TextLoop12Result {
    MlTemporaryCompareUtf16TextLoop12Done,
    MlTemporaryCompareUtf16TextLoop12Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_utf16_text_loop12(left: &[f64], right: &[f64], mut count: f64, mut at: f64) -> self::MlTemporaryCompareUtf16TextLoop12Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlTemporaryCompareUtf16TextLoop12Result::MlTemporaryCompareUtf16TextLoop12Return(if (a < b) {
                            (-1f64)
                        } else {
                            1f64
                        })
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
            }
        } else {
            self::MlTemporaryCompareUtf16TextLoop12Result::MlTemporaryCompareUtf16TextLoop12Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop14(separator: &[f64], mut result: Vec<f64>, mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((separator.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(separator, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
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
pub fn source_ml_temporary_join_utf16_text_loop15(mut result: Vec<f64>, value: &[f64], mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((value.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(value, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
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
pub fn source_ml_temporary_join_utf16_text_loop13(values: &[Vec<f64>], separator: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let result_3 = if (at != (0f64)) {
                    {
                        let offset = 0f64;
                        {
                            let result_2 = self::source_ml_temporary_join_utf16_text_loop14(separator, result, offset);
                            result_2
                        }
                    }
                } else {
                    result
                };
                {
                    let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| Vec::<f64>::new());
                    {
                        let offset_2 = 0f64;
                        {
                            let result_4 = self::source_ml_temporary_join_utf16_text_loop15(result_3, &(value), offset_2);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_4, at_2);
                                    continue;
                                }
                            }
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
pub fn source_ml_temporary_trim_utf16_text_loop16(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((first < last) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index(first))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_trim_utf16_text_loop17(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((last > first) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let last_2 = (last - 1f64);
                {
                    last = last_2;
                    continue;
                }
            }
        } else {
            last
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_character_units_utf16_text_loop18(values: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = (integer % 65536f64);
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (residue < (0f64)) {
                                (residue + 65536f64)
                            } else {
                                if (residue == (0f64)) {
                                    0f64
                                } else {
                                    residue
                                }
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCodePointUnitsUtf16TextLoop19Result {
    MlTemporaryCodePointUnitsUtf16TextLoop19Done(Vec<f64>),
    MlTemporaryCodePointUnitsUtf16TextLoop19Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_code_point_units_utf16_text_loop19(values: &[f64], mut result: Vec<f64>, mut at: f64) -> self::MlTemporaryCodePointUnitsUtf16TextLoop19Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value > (1114111f64))) {
                    self::MlTemporaryCodePointUnitsUtf16TextLoop19Result::MlTemporaryCodePointUnitsUtf16TextLoop19Return(vec![at])
                } else {
                    if (value < (65536f64)) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (value == (0f64)) {
                                0f64
                            } else {
                                value
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![(55296f64 + (((value - 65536f64) / 1024f64)).floor()), (56320f64 + ((value - 65536f64) % 1024f64))], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCodePointUnitsUtf16TextLoop19Result::MlTemporaryCodePointUnitsUtf16TextLoop19Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDisplayPointsUtf16TextJoin21 {
    MlTemporaryDisplayPointsUtf16TextJoin21Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_points_utf16_text_loop20(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                {
                    let source_ml_source1_s15 = if ((point >= (55296f64)) && (point <= (56319f64))) {
                        {
                            let low = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                            if ((low >= (56320f64)) && (low <= (57343f64))) {
                                {
                                    let point_2 = (((65536f64 + ((point - 55296f64) * 1024f64)) + low) - 56320f64);
                                    {
                                        let at_2 = (at + 1f64);
                                        self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65533f64;
                                    self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at, point_3)
                                }
                            }
                        }
                    } else {
                        if ((point >= (56320f64)) && (point <= (57343f64))) {
                            {
                                let point_4 = 65533f64;
                                self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at, point_4)
                            }
                        } else {
                            self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at, point)
                        }
                    };
                    match source_ml_source1_s15.clone() {
                        self::MlTemporaryDisplayPointsUtf16TextJoin21::MlTemporaryDisplayPointsUtf16TextJoin21Next(at_3, point_5) => {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![point_5], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at_3 + 1f64);
                                    {
                                        (result, at) = (result_2, at_4);
                                        continue;
                                    }
                                }
                            }
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
pub fn source_ml_temporary_display_utf8_from_utf16_text_loop22(mut bytes: Vec<f64>, points: &[f64], mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((points.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(points, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                if (point < (128f64)) {
                    {
                        let bytes_2 = { let (ml_append_right, ml_append_left) = (vec![point], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (bytes, at) = (bytes_2, at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    if (point < (2048f64)) {
                        {
                            let bytes_3 = { let (ml_append_right, ml_append_left) = (vec![(192f64 + ((point / 64f64)).floor()), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (bytes, at) = (bytes_3, at_3);
                                    continue;
                                }
                            }
                        }
                    } else {
                        if (point < (65536f64)) {
                            {
                                let bytes_4 = { let (ml_append_right, ml_append_left) = (vec![(224f64 + ((point / 4096f64)).floor()), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_4, at_4);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let bytes_5 = { let (ml_append_right, ml_append_left) = (vec![(240f64 + ((point / 262144f64)).floor()), (128f64 + (((point / 4096f64)).floor() % 64f64)), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_5 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_5, at_5);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            bytes
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanUtf16TextSubstitutionLoop23Result {
    MlTemporaryPlanUtf16TextSubstitutionLoop23Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_utf16_text_substitution_loop23(replacement: &[f64], mut result: Vec<f64>, mut start: f64, mut at: f64) -> self::MlTemporaryPlanUtf16TextSubstitutionLoop23Result {
    loop {
        return if (at < ((replacement.len() as f64))) {
            {
                let next = (self::ml_array::optional_at(replacement, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if ((((self::ml_array::optional_at(replacement, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)) == (36f64)) && ((((next == (36f64)) || (next == (38f64))) || (next == (96f64))) || (next == (39f64)))) {
                    {
                        let result_3 = if (start < at) {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![0f64, start, at], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let result_6 = if (next == (36f64)) {
                                {
                                    let result_4 = { let (ml_append_right, ml_append_left) = (vec![0f64, at, (at + 1f64)], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_4
                                }
                            } else {
                                {
                                    let result_5 = { let (ml_append_right, ml_append_left) = (vec![if (next == (38f64)) {
                                        1f64
                                    } else {
                                        if (next == (96f64)) {
                                            2f64
                                        } else {
                                            3f64
                                        }
                                    }, 0f64, 0f64], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_5
                                }
                            };
                            {
                                let at_2 = (at + 2f64);
                                {
                                    let start_2 = at_2;
                                    {
                                        (result, start, at) = (result_6, start_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = (at + 1f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanUtf16TextSubstitutionLoop23Result::MlTemporaryPlanUtf16TextSubstitutionLoop23Done(result, start)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop25(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
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
pub enum MlTemporaryReplaceUtf16TextLoop24Result {
    MlTemporaryReplaceUtf16TextLoop24Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop26(text: &[f64], mut result: Vec<f64>, mut found: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < found) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
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
pub fn source_ml_temporary_replace_utf16_text_loop28(mut result: Vec<f64>, mut last: f64, source: &[f64], mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(source, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
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
pub fn source_ml_temporary_replace_utf16_text_loop27(text: &[f64], search: &[f64], replacement: &[f64], mut result: Vec<f64>, plan: &[f64], mut found: f64, mut instruction: f64) -> Vec<f64> {
    loop {
        return if (instruction < ((plan.len() as f64))) {
            {
                let kind = (self::ml_array::optional_at(plan, self::ml_array::number_index(instruction))).clone().unwrap_or_else(|| 0f64);
                {
                    let first = if (kind == (0f64)) {
                        (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 1f64)))).clone().unwrap_or_else(|| 0f64)
                    } else {
                        if (kind == (2f64)) {
                            0f64
                        } else {
                            if (kind == (1f64)) {
                                found
                            } else {
                                (found + (search.len() as f64))
                            }
                        }
                    };
                    {
                        let last = if (kind == (0f64)) {
                            (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 2f64)))).clone().unwrap_or_else(|| 0f64)
                        } else {
                            if (kind == (2f64)) {
                                found
                            } else {
                                if (kind == (1f64)) {
                                    (found + (search.len() as f64))
                                } else {
                                    (text.len() as f64)
                                }
                            }
                        };
                        {
                            let source = if (kind == (0f64)) {
                                replacement.to_vec()
                            } else {
                                text.to_vec()
                            };
                            {
                                let at = first;
                                {
                                    let result_2 = self::source_ml_temporary_replace_utf16_text_loop28(result, last, &(source), at);
                                    {
                                        let instruction_2 = (instruction + 3f64);
                                        {
                                            (result, instruction) = (result_2, instruction_2);
                                            continue;
                                        }
                                    }
                                }
                            }
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
pub fn source_ml_temporary_replace_utf16_text_loop24(text: &[f64], search: &[f64], replacement: &[f64], mut all: bool, mut result: Vec<f64>, plan: &[f64], mut start: f64, mut from: f64, mut searching: bool) -> self::MlTemporaryReplaceUtf16TextLoop24Result {
    loop {
        return if searching {
            {
                let found = self::index_utf16_text(text, search, from);
                if (found < (0f64)) {
                    {
                        let searching_2 = false;
                        {
                            searching = searching_2;
                            continue;
                        }
                    }
                } else {
                    {
                        let at = start;
                        {
                            let result_2 = self::source_ml_temporary_replace_utf16_text_loop26(text, result, found, at);
                            {
                                let instruction = 0f64;
                                {
                                    let result_3 = self::source_ml_temporary_replace_utf16_text_loop27(text, search, replacement, result_2, plan, found, instruction);
                                    {
                                        let start_2 = (found + (search.len() as f64));
                                        {
                                            let searching_3 = (all && !(((((search.len() as f64)) == (0f64)) && (found == ((text.len() as f64))))));
                                            {
                                                let from_2 = (start_2 + if (((search.len() as f64)) == (0f64)) {
                                                    1f64
                                                } else {
                                                    0f64
                                                });
                                                {
                                                    (result, start, from, searching) = (result_3, start_2, from_2, searching_3);
                                                    continue;
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
            self::MlTemporaryReplaceUtf16TextLoop24Result::MlTemporaryReplaceUtf16TextLoop24Done(result, start)
        };
    }
}
}
