// Generated from complete js/src/translation/rust-pattern-match-rules.js. Do not edit by hand.
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

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Number fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required text fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Boolean fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Number fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required text fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Boolean fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_pattern_match_macro_owner(owner: &str, ordinary: bool) -> bool {
    ordinary && (((owner == "matches") || (owner == "std::matches")) || (owner == "core::matches"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_pattern_match_rows(
    kinds: &[String],
    values: &[String],
    start: f64,
    close: &str,
) -> Vec<f64> {
    if (((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64)))
        || !(self::ml_array::includes(
            &vec![String::from(")"), String::from("]"), String::from("}")],
            &close.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        ))
    {
        Vec::<f64>::new()
    } else {
        {
            let stack = Vec::<String>::new();
            {
                let end = -1f64;
                {
                    let at = start;
                    {
                        let source_ml_source1_s1 =
                            self::source_ml_temporary_plan_rust_pattern_match_rows_loop1(
                                kinds, values, close, stack, end, at,
                            );
                        match source_ml_source1_s1 {
                            self::MlTemporaryPlanRustPatternMatchRowsLoop1Result::MlTemporaryPlanRustPatternMatchRowsLoop1Done(end_2) => {
                                if end_2 < 0f64 {
                                    Vec::<f64>::new()
                                } else {
                                    {
                                        let last = if ((end_2 > start) && ((self::read_required_text_fact(kinds, (end_2 - 1f64))) == "punct")) && ((self::read_required_text_fact(values, (end_2 - 1f64))) == ",") {
                                            end_2 - 1f64
                                        } else {
                                            end_2
                                        };
                                        if last == start {
                                            Vec::<f64>::new()
                                        } else {
                                            {
                                                let plan = vec![(end_2 + 1f64), -1f64];
                                                {
                                                    let at_2 = start;
                                                    {
                                                        let plan_2 = self::source_ml_temporary_plan_rust_pattern_match_rows_loop2(last, plan, at_2);
                                                        {
                                                            let source_ml_source1_concat_value2 = plan_2;
                                                            {
                                                                let source_ml_source1_concat_value3 = vec![-2f64, -3f64, -4f64, -5f64, -2f64, -6f64, -7f64];
                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value2), source_ml_source1_concat_value3)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryPlanRustPatternMatchRowsLoop1Result::MlTemporaryPlanRustPatternMatchRowsLoop1Return(source_ml_temporary_result2) => {
                                source_ml_temporary_result2
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
pub fn read_rust_pattern_match_token_value(code: f64) -> String {
    if code == -1f64 {
        String::from("{")
    } else if code == -2f64 {
        String::from("=>")
    } else if code == -3f64 {
        String::from("true")
    } else if code == -4f64 {
        String::from(",")
    } else if code == -5f64 {
        String::from("_")
    } else if code == -6f64 {
        String::from("false")
    } else if code == -7f64 {
        String::from("}")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_pattern_match_token_kind(code: f64) -> String {
    if ((code == -3f64) || (code == -5f64)) || (code == -6f64) {
        String::from("identifier")
    } else if (((code == -1f64) || (code == -2f64)) || (code == -4f64)) || (code == -7f64) {
        String::from("punct")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_pattern_match_rows_loop2(
    mut last: f64,
    mut plan: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < last {
            {
                let plan_2 = {
                    let (ml_append_right, ml_append_left) = (vec![at], plan);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (plan, at) = (plan_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { plan };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustPatternMatchRowsLoop1Result {
    MlTemporaryPlanRustPatternMatchRowsLoop1Done(f64),
    MlTemporaryPlanRustPatternMatchRowsLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_pattern_match_rows_loop1(
    kinds: &[String],
    values: &[String],
    close: &str,
    mut stack: Vec<String>,
    mut end: f64,
    mut at: f64,
) -> self::MlTemporaryPlanRustPatternMatchRowsLoop1Result {
    loop {
        return if at < (kinds.len() as f64) {
            if (self::read_required_text_fact(kinds, at)) != "punct" {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let value = self::read_required_text_fact(values, at);
                    if ((value == "(") || (value == "[")) || (value == "{") {
                        {
                            let stack_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![if value == "(" {
                                        String::from(")")
                                    } else if value == "[" {
                                        String::from("]")
                                    } else {
                                        String::from("}")
                                    }],
                                    stack,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_3 = at + 1f64;
                                {
                                    (stack, at) = (stack_2, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if ((value == ")") || (value == "]")) || (value == "}") {
                            if (stack.len() as f64) == 0f64 {
                                if value == close {
                                    {
                                        let end_2 = at;
                                        self::MlTemporaryPlanRustPatternMatchRowsLoop1Result::MlTemporaryPlanRustPatternMatchRowsLoop1Done(end_2)
                                    }
                                } else {
                                    self::MlTemporaryPlanRustPatternMatchRowsLoop1Result::MlTemporaryPlanRustPatternMatchRowsLoop1Return(Vec::<f64>::new())
                                }
                            } else if (self::read_required_text_fact(
                                &(stack),
                                ((stack.len() as f64) - 1f64),
                            )) == value
                            {
                                {
                                    let stack_3 = self::ml_array::slice(&stack, 0f64, -1f64);
                                    {
                                        let at_4 = at + 1f64;
                                        {
                                            (stack, at) = (stack_3, at_4);
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                self::MlTemporaryPlanRustPatternMatchRowsLoop1Result::MlTemporaryPlanRustPatternMatchRowsLoop1Return(Vec::<f64>::new())
                            }
                        } else {
                            {
                                let at_5 = at + 1f64;
                                {
                                    at = at_5;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanRustPatternMatchRowsLoop1Result::MlTemporaryPlanRustPatternMatchRowsLoop1Done(end)
        };
    }
}
