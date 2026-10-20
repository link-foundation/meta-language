// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
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
pub fn accept_rust_literal_match_receiver(
    kind: &str,
    text_model: &str,
    proven: bool,
    wrappers: &str,
    shadowed: bool,
) -> bool {
    (((proven && !shadowed) && (kind == "string")) && (text_model == "string slice"))
        && (wrappers == "reference")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_literal_match_catalog_owner(kind: &str) -> bool {
    kind == "data"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_literal_match_pattern(
    kinds: &[String],
    values: &[String],
    start: f64,
    close: &str,
) -> Vec<f64> {
    if (((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64)))
        || (((close != ")") && (close != "]")) && (close != "}"))
    {
        Vec::<f64>::new()
    } else {
        {
            let at = start;
            {
                let positions = Vec::<f64>::new();
                {
                    let at_3 = if match (
                        self::ml_array::optional_at(kinds, self::ml_array::number_index(at)),
                        String::from("punct"),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    } && match (
                        self::ml_array::optional_at(values, self::ml_array::number_index(at)),
                        String::from("|"),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    } {
                        { at + 1f64 }
                    } else {
                        at
                    };
                    {
                        let ml_s1 = self::ml_plan_rust_literal_match_pattern_loop1(
                            kinds, values, close, at_3, positions,
                        );
                        match ml_s1 {
                            self::MlPlanRustLiteralMatchPatternLoop1Result::MlPlanRustLiteralMatchPatternLoop1Done => {
                                Vec::<f64>::new()
                            }
                            self::MlPlanRustLiteralMatchPatternLoop1Result::MlPlanRustLiteralMatchPatternLoop1Return(ml_result1) => {
                                ml_result1
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanRustLiteralMatchPatternLoop1Result {
    MlPlanRustLiteralMatchPatternLoop1Done,
    MlPlanRustLiteralMatchPatternLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_rust_literal_match_pattern_loop1(
    kinds: &[String],
    values: &[String],
    close: &str,
    mut at: f64,
    mut positions: Vec<f64>,
) -> self::MlPlanRustLiteralMatchPatternLoop1Result {
    loop {
        return if at < (kinds.len() as f64) {
            if !(match (
                self::ml_array::optional_at(kinds, self::ml_array::number_index(at)),
                String::from("string"),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }) || ((match self::ml_array::optional_at(values, self::ml_array::number_index(at)) {
                Some(ml_present) => {
                    let _ = ml_present;
                    String::from("string")
                }
                None => String::from("undefined"),
            }) != "string")
            {
                self::MlPlanRustLiteralMatchPatternLoop1Result::MlPlanRustLiteralMatchPatternLoop1Return(Vec::<f64>::new())
            } else {
                {
                    let positions_2 = {
                        let (ml_append_right, ml_append_left) = (vec![at], positions);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let at_2 = at + 1f64;
                        if match (
                            self::ml_array::optional_at(kinds, self::ml_array::number_index(at_2)),
                            String::from("punct"),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } && match (
                            self::ml_array::optional_at(values, self::ml_array::number_index(at_2)),
                            close.to_owned(),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } {
                            self::MlPlanRustLiteralMatchPatternLoop1Result::MlPlanRustLiteralMatchPatternLoop1Return(self::ml_array::append(vec![(at_2 + 1f64)], positions_2))
                        } else if ((match (
                            self::ml_array::optional_at(kinds, self::ml_array::number_index(at_2)),
                            String::from("punct"),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } && match (
                            self::ml_array::optional_at(values, self::ml_array::number_index(at_2)),
                            String::from(","),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) && match (
                            self::ml_array::optional_at(
                                kinds,
                                self::ml_array::number_index(at_2 + 1f64),
                            ),
                            String::from("punct"),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) && match (
                            self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(at_2 + 1f64),
                            ),
                            close.to_owned(),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } {
                            self::MlPlanRustLiteralMatchPatternLoop1Result::MlPlanRustLiteralMatchPatternLoop1Return(self::ml_array::append(vec![(at_2 + 2f64)], positions_2))
                        } else if !(match (
                            self::ml_array::optional_at(kinds, self::ml_array::number_index(at_2)),
                            String::from("punct"),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) || !(match (
                            self::ml_array::optional_at(values, self::ml_array::number_index(at_2)),
                            String::from("|"),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) {
                            self::MlPlanRustLiteralMatchPatternLoop1Result::MlPlanRustLiteralMatchPatternLoop1Return(Vec::<f64>::new())
                        } else {
                            {
                                let at_3 = at_2 + 1f64;
                                {
                                    (at, positions) = (at_3, positions_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlPlanRustLiteralMatchPatternLoop1Result::MlPlanRustLiteralMatchPatternLoop1Done
        };
    }
}
