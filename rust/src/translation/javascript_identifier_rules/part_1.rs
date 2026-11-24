// Generated from complete js/src/translation/javascript-identifier-rules.js. Do not edit by hand.
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
pub fn decode_java_script_identifier_escape(units: &[f64], start: f64) -> Vec<f64> {
    if (((!(self::ml_math::is_safe_integer(start)) || (start < 0f64))
        || ((start + 2f64) > (units.len() as f64)))
        || !(match (
            self::ml_array::optional_at(units, self::ml_array::number_index(start)),
            92f64,
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        }))
        || !(match (
            self::ml_array::optional_at(units, self::ml_array::number_index(start + 1f64)),
            117f64,
        ) {
            (Some(ml_left), ml_right) => ml_left == ml_right,
            _ => false,
        })
    {
        vec![
            -1f64,
            self::ml_math::min((units.len() as f64), (start + 2f64)),
        ]
    } else {
        {
            let braced = match (
                self::ml_array::optional_at(units, self::ml_array::number_index(start + 2f64)),
                123f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            };
            {
                let index = start + if braced { 3f64 } else { 2f64 };
                {
                    let digits = 0f64;
                    {
                        let scalar = 0f64;
                        {
                            let ml_s1 = self::ml_decode_java_script_identifier_escape_loop1(
                                units, braced, index, digits, scalar,
                            );
                            match ml_s1 {
                                self::MlDecodeJavaScriptIdentifierEscapeLoop1Result::MlDecodeJavaScriptIdentifierEscapeLoop1Done(index_2, digits_2, scalar_2) => {
                                    {
                                        let ml_s2 = if braced {
                                            if (digits_2 == 0f64) || !(match (self::ml_array::optional_at(units, self::ml_array::number_index(index_2)), 125f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) {
                                                self::MlDecodeJavaScriptIdentifierEscapeJoin2::MlDecodeJavaScriptIdentifierEscapeJoin2Return(vec![-1f64, self::ml_math::min((units.len() as f64), (index_2 + 1f64))])
                                            } else {
                                                {
                                                    let index_3 = index_2 + 1f64;
                                                    self::MlDecodeJavaScriptIdentifierEscapeJoin2::MlDecodeJavaScriptIdentifierEscapeJoin2Next(index_3)
                                                }
                                            }
                                        } else {
let ml_library_inequality_1 = digits_2 != 4f64 ;
if ml_library_inequality_1 {
                                                self::MlDecodeJavaScriptIdentifierEscapeJoin2::MlDecodeJavaScriptIdentifierEscapeJoin2Return(vec![-1f64, self::ml_math::min((units.len() as f64), (index_2 + 1f64))])
                                            } else {
                                                self::MlDecodeJavaScriptIdentifierEscapeJoin2::MlDecodeJavaScriptIdentifierEscapeJoin2Next(index_2)
                                            }
};
                                        match ml_s2 {
                                            self::MlDecodeJavaScriptIdentifierEscapeJoin2::MlDecodeJavaScriptIdentifierEscapeJoin2Next(index_4) => {
                                                if (55_296f64..=57_343f64).contains(&scalar_2) {
                                                    vec![-1f64, index_4]
                                                } else {
                                                    vec![index_4, scalar_2]
                                                }
                                            }
                                            self::MlDecodeJavaScriptIdentifierEscapeJoin2::MlDecodeJavaScriptIdentifierEscapeJoin2Return(ml_result2) => {
                                                ml_result2
                                            }
                                        }
                                    }
                                }
                                self::MlDecodeJavaScriptIdentifierEscapeLoop1Result::MlDecodeJavaScriptIdentifierEscapeLoop1Return(ml_result2_2) => {
                                    ml_result2_2
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
pub fn accept_java_script_reserved_identifier(name: &str) -> bool {
    self::ml_array::includes(
        &vec![
            String::from("await"),
            String::from("break"),
            String::from("case"),
            String::from("catch"),
            String::from("class"),
            String::from("const"),
            String::from("continue"),
            String::from("debugger"),
            String::from("default"),
            String::from("delete"),
            String::from("do"),
            String::from("else"),
            String::from("enum"),
            String::from("export"),
            String::from("extends"),
            String::from("false"),
            String::from("finally"),
            String::from("for"),
            String::from("function"),
            String::from("if"),
            String::from("import"),
            String::from("in"),
            String::from("instanceof"),
            String::from("new"),
            String::from("null"),
            String::from("return"),
            String::from("super"),
            String::from("switch"),
            String::from("this"),
            String::from("throw"),
            String::from("true"),
            String::from("try"),
            String::from("typeof"),
            String::from("var"),
            String::from("void"),
            String::from("while"),
            String::from("with"),
            String::from("yield"),
            String::from("let"),
            String::from("static"),
            String::from("implements"),
            String::from("interface"),
            String::from("package"),
            String::from("private"),
            String::from("protected"),
            String::from("public"),
        ],
        &name.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_source_keyword_token(name: &str, raw_: &str) -> bool {
    !(raw_.contains("\\"))
        || !(self::accept_java_script_reserved_identifier(name)
            || self::ml_array::includes(
                &vec![
                    String::from("as"),
                    String::from("async"),
                    String::from("from"),
                    String::from("get"),
                    String::from("meta"),
                    String::from("of"),
                    String::from("set"),
                    String::from("target"),
                ],
                &name.to_owned(),
                0f64,
                false,
                |a, b| a == b,
            ))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_token_goal_value(kind: &str, name: &str, raw_: &str) -> String {
    if (kind == "identifier") && !(self::accept_java_script_source_keyword_token(name, raw_)) {
        String::new()
    } else {
        name.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_escaped_identifier_error(name: &str, raw_: &str, binding: bool) -> String {
    if (raw_.contains("\\")) {
        if self::accept_java_script_reserved_identifier(name) {
            format!(
                "escaped JavaScript identifier is reserved: {}",
                name.to_owned()
            )
        } else if binding && ((name == "arguments") || (name == "eval")) {
            format!(
                "invalid strict JavaScript binding identifier: {}",
                name.to_owned()
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_binding_identifier_context(context: &str) -> bool {
    self::ml_array::includes(
        &vec![
            String::from("function"),
            String::from("parameter"),
            String::from("closure parameter"),
            String::from("array binding"),
            String::from("namespace"),
            String::from("const"),
            String::from("let"),
            String::from("for…of"),
        ],
        &context.to_owned(),
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_number_identifier_boundary(identifier: bool, following: f64) -> bool {
    (!identifier && (following != 92f64)) && !((48f64..=57f64).contains(&following))
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeJavaScriptIdentifierEscapeJoin2 {
    MlDecodeJavaScriptIdentifierEscapeJoin2Next(f64),
    MlDecodeJavaScriptIdentifierEscapeJoin2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeJavaScriptIdentifierEscapeLoop1Result {
    MlDecodeJavaScriptIdentifierEscapeLoop1Done(f64, f64, f64),
    MlDecodeJavaScriptIdentifierEscapeLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_decode_java_script_identifier_escape_loop1(
    units: &[f64],
    mut braced: bool,
    mut index: f64,
    mut digits: f64,
    mut scalar: f64,
) -> self::MlDecodeJavaScriptIdentifierEscapeLoop1Result {
    loop {
        return if (index < (units.len() as f64)) && (braced || (digits < 4f64)) {
            {
                let unit =
                    (self::ml_array::optional_at(units, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                {
                    let digit = -1f64;
                    {
                        let digit_5 = if (48f64..=57f64).contains(&unit) {
                            { unit - 48f64 }
                        } else if (65f64..=70f64).contains(&unit) {
                            { unit - 55f64 }
                        } else if (97f64..=102f64).contains(&unit) {
                            { unit - 87f64 }
                        } else {
                            digit
                        };
                        if digit_5 < 0f64 {
                            self::MlDecodeJavaScriptIdentifierEscapeLoop1Result::MlDecodeJavaScriptIdentifierEscapeLoop1Done(index, digits, scalar)
                        } else {
                            {
                                let scalar_2 = (scalar * 16f64) + digit_5;
                                {
                                    let digits_2 = digits + 1f64;
                                    {
                                        let index_2 = index + 1f64;
                                        if scalar_2 > 1_114_111f64 {
                                            self::MlDecodeJavaScriptIdentifierEscapeLoop1Result::MlDecodeJavaScriptIdentifierEscapeLoop1Return(vec![-1f64, index_2])
                                        } else {
                                            {
                                                (index, digits, scalar) =
                                                    (index_2, digits_2, scalar_2);
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
        } else {
            self::MlDecodeJavaScriptIdentifierEscapeLoop1Result::MlDecodeJavaScriptIdentifierEscapeLoop1Done(index, digits, scalar)
        };
    }
}
