// Generated from complete js/src/translation/utf16-text-expansion-rules.js. Do not edit by hand.
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
pub fn read_utf16_text_repetition_length(length: f64, count: f64) -> f64 {
    {
        let integer = if count.is_nan() { 0f64 } else { count.trunc() };
        if (integer < 0f64) || (integer == (f64::INFINITY)) {
            -1f64
        } else if (!(self::ml_math::is_safe_integer(length)) || (length < 0f64))
            || (length > 1_048_576f64)
        {
            -2f64
        } else if (length == 0f64) || (integer == 0f64) {
            0f64
        } else if integer > ((1_048_576f64 / length).floor()) {
            -2f64
        } else {
            length * integer
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_padding_length(length: f64, maximum: f64, filler_length: f64) -> f64 {
    if ((((!(self::ml_math::is_safe_integer(length)) || (length < 0f64))
        || (length > 1_048_576f64))
        || !(self::ml_math::is_safe_integer(filler_length)))
        || (filler_length < 0f64))
        || (filler_length > 1_048_576f64)
    {
        -2f64
    } else {
        {
            let integer = if maximum.is_nan() {
                0f64
            } else {
                maximum.trunc()
            };
            {
                let target =
                    self::ml_math::min(9_007_199_254_740_991f64, self::ml_math::max(0f64, integer));
                if (target <= length) || (filler_length == 0f64) {
                    length
                } else if target > 1_048_576f64 {
                    -2f64
                } else {
                    target
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_utf16_text_repetition(text: &[f64], length: f64) -> Vec<f64> {
    {
        assert!(
            !(((!(self::ml_math::is_safe_integer(length)) || (length < 0f64))
                || (length > 1_048_576f64))
                || ((length > 0f64)
                    && (((text.len() as f64) == 0f64)
                        || ((length % (text.len() as f64)) != 0f64)))),
            "{}",
            "text repetition requires its complete admitted length"
        );

        {
            let result = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    self::source_ml_temporary_construct_utf16_text_repetition_loop1(
                        text, length, result, index,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_utf16_text_padding(
    text: &[f64],
    filler: &[f64],
    length: f64,
    start: bool,
) -> Vec<f64> {
    {
        assert!(
            !(((!(self::ml_math::is_safe_integer(length)) || (length < (text.len() as f64)))
                || (length > 1_048_576f64))
                || ((length > (text.len() as f64)) && ((filler.len() as f64) == 0f64))),
            "{}",
            "text padding requires its complete admitted length"
        );

        {
            let added = length - (text.len() as f64);
            {
                let result = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        self::source_ml_temporary_construct_utf16_text_padding_loop2(
                            text, filler, length, start, added, result, index,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_construct_utf16_text_repetition_loop1(
    text: &[f64],
    mut length: f64,
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < length {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::optional_at(
                                text,
                                self::ml_array::number_index(index % (text.len() as f64)),
                            ))
                            .unwrap_or_else(|| 0f64),
                        ],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_construct_utf16_text_padding_loop2(
    text: &[f64],
    filler: &[f64],
    mut length: f64,
    mut start: bool,
    mut added: f64,
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < length {
            {
                let original = if start {
                    index >= added
                } else {
                    index < (text.len() as f64)
                };
                {
                    let position = if original {
                        if start { index - added } else { index }
                    } else if start {
                        index
                    } else {
                        index - (text.len() as f64)
                    };
                    {
                        let result_2 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![
                                    (if original {
                                        self::ml_array::optional_at(
                                            text,
                                            self::ml_array::number_index(position),
                                        )
                                    } else {
                                        self::ml_array::optional_at(
                                            filler,
                                            self::ml_array::number_index(
                                                position % (filler.len() as f64),
                                            ),
                                        )
                                    })
                                    .clone()
                                    .unwrap_or_else(|| 0f64),
                                ],
                                result,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (result, index) = (result_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}
