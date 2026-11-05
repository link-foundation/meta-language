// Generated from complete js/src/translation/javascript-string-unit-rules.js. Do not edit by hand.
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

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptStringEscape {
    Units(Vec<f64>, f64),
    Malformed,
}

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
pub fn decode_java_script_string_unicode_escape(
    source: &[f64],
    start: f64,
) -> self::JavaScriptStringEscape {
    if (!(self::ml_math::is_safe_integer(start)) || (start < 0f64))
        || !(match (
            self::ml_array::optional_at(source, self::ml_array::number_index(start)),
            92f64,
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
    {
        self::JavaScriptStringEscape::Malformed
    } else {
        {
            let marker =
                (self::ml_array::optional_at(source, self::ml_array::number_index(start + 1f64)))
                    .unwrap_or_else(|| -1f64);
            if (marker != 117f64) && (marker != 120f64) {
                self::JavaScriptStringEscape::Malformed
            } else {
                {
                    let braced = (marker == 117f64)
                        && match (
                            self::ml_array::optional_at(
                                source,
                                self::ml_array::number_index(start + 2f64),
                            ),
                            123f64,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        };
                    {
                        let count = if marker == 120f64 { 2f64 } else { 4f64 };
                        {
                            let at = start + if braced { 3f64 } else { 2f64 };
                            {
                                let digits = 0f64;
                                {
                                    let code = 0f64;
                                    {
                                        let source_ml_source2_s1 = self::source_ml_source1_temporary_decode_java_script_string_unicode_escape_loop1(source, braced, count, at, digits, code);
                                        match source_ml_source2_s1 {
                                            self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeLoop1Result::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeLoop1Done(at_2, digits_2, code_2) => {
                                                {
                                                    let source_ml_source2_s2 = if braced {
                                                        if (digits_2 == 0f64) || !(match (self::ml_array::optional_at(source, self::ml_array::number_index(at_2)), 125f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) {
                                                            self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2Return(Box::new(self::JavaScriptStringEscape::Malformed))
                                                        } else {
                                                            {
                                                                let at_3 = at_2 + 1f64;
                                                                self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2Next(at_3)
                                                            }
                                                        }
                                                    } else {
let ml_library_inequality_1 = digits_2 != count ;
if ml_library_inequality_1 {
                                                            self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2Return(Box::new(self::JavaScriptStringEscape::Malformed))
                                                        } else {
                                                            self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2Next(at_2)
                                                        }
};
                                                    match source_ml_source2_s2 {
                                                        self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2Next(at_4) => {
                                                            if code_2 > 1_114_111f64 {
                                                                self::JavaScriptStringEscape::Malformed
                                                            } else {
                                                                {
                                                                    let units = if code_2 < 65_536f64 {
                                                                        vec![code_2]
                                                                    } else {
                                                                        vec![(55_296f64 + ((code_2 - 65_536f64) / 1024f64).floor()), (56_320f64 + ((code_2 - 65_536f64) % 1024f64))]
                                                                    };
                                                                    self::JavaScriptStringEscape::Units(units, (at_4 - start))
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2Return(source_ml_source1_temporary_result2) => {
                                                            let source_ml_source1_temporary_result2 = *source_ml_source1_temporary_result2;
                                                            source_ml_source1_temporary_result2.clone()
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
pub fn accept_java_script_string_units(units: &[f64]) -> bool {
    {
        let at = 0f64;
        {
            let source_ml_source2_s3 =
                self::source_ml_source1_temporary_accept_java_script_string_units_loop3(units, at);
            match source_ml_source2_s3 {
                self::MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Result::MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Done => {
                    true
                }
                self::MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Result::MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Return(source_ml_source1_temporary_result3) => {
                    source_ml_source1_temporary_result3
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_string_unit_transport(units: &[f64]) -> bool {
    {
        let at = 0f64;
        {
            let source_ml_source2_s4 =
                self::source_ml_source1_temporary_require_java_script_string_unit_transport_loop4(
                    units, at,
                );
            match source_ml_source2_s4 {
                self::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Result::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Done => {
                    false
                }
                self::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Result::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Return(source_ml_source1_temporary_result4) => {
                    source_ml_source1_temporary_result4
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_string_unit_construction(kind: &str, operation: &str) -> bool {
    (kind == "math") && (operation == "createJavaScriptStringFromUnits")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_string_unit_construction(target: &str, values: &str) -> String {
    if target == "JavaScript" {
        format!(
            "(({}).map(ml_unit=>String.fromCharCode(ml_unit)).join(\"\"))",
            values.to_owned()
        )
    } else if target == "Rust" {
        format!(
            "crate::ml_string::Text::from_units(({}).into_iter().map(|ml_unit| ml_unit as u16).collect())",
            values.to_owned()
        )
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2 {
    MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2Next(f64),
    MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeJoin2Return(
        Box<self::JavaScriptStringEscape>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeLoop1Result {
    MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeLoop1Done(f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_decode_java_script_string_unicode_escape_loop1(
    source: &[f64],
    mut braced: bool,
    mut count: f64,
    mut at: f64,
    mut digits: f64,
    mut code: f64,
) -> self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeLoop1Result {
    loop {
        return if (at < (source.len() as f64)) && (braced || (digits < count)) {
            {
                let digit = self::read_java_script_numeric_digit_unit(
                    (self::ml_array::optional_at(source, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -1f64),
                );
                if (digit < 0f64) || (digit >= 16f64) {
                    self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeLoop1Result::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeLoop1Done(at, digits, code)
                } else {
                    {
                        let code_2 = self::ml_math::min(1_114_112f64, ((code * 16f64) + digit));
                        {
                            let digits_2 = digits + 1f64;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (at, digits, code) = (at_2, digits_2, code_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeLoop1Result::MlSource1TemporaryDecodeJavaScriptStringUnicodeEscapeLoop1Done(at, digits, code)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Result {
    MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Done,
    MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_string_units_loop3(
    units: &[f64],
    mut at: f64,
) -> self::MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64)
                {
                    self::MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Result::MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Return(false)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Result::MlSource1TemporaryAcceptJavaScriptStringUnitsLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Result {
    MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Done,
    MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_require_java_script_string_unit_transport_loop4(
    units: &[f64],
    mut at: f64,
) -> self::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (55_296f64..=56_319f64).contains(&unit) {
                    {
                        let low = (self::ml_array::optional_at(
                            units,
                            self::ml_array::number_index(at + 1f64),
                        ))
                        .unwrap_or_else(|| -1f64);
                        if (low < 56_320f64) || (low > 57_343f64) {
                            self::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Result::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Return(true)
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    let at_3 = at_2 + 1f64;
                                    {
                                        at = at_3;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else if (56_320f64..=57_343f64).contains(&unit) {
                    self::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Result::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Return(true)
                } else {
                    {
                        let at_4 = at + 1f64;
                        {
                            at = at_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Result::MlSource1TemporaryRequireJavaScriptStringUnitTransportLoop4Done
        };
    }
}
