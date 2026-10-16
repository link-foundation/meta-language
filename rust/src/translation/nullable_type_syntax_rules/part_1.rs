// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::*;

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
        match index.and_then(|index| values.get(index)) {
            Some(value) => value.clone(),
            None => panic!("array index out of range"),
        }
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
    fn integer(value: f64) -> f64 {
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
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Number fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_number_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required text fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Boolean fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_nullable_type_syntax(alternatives: &[String], first: f64) -> Vec<f64> {
    if (alternatives.len() as f64) > 1f64 {
        {
            let missing = 0f64;
            {
                let values = 0f64;
                {
                    let value = -1f64;
                    {
                        let absence = 0f64;
                        {
                            let at = 0f64;
                            {
                                let ml_s1 = self::ml_read_nullable_type_syntax_loop1(
                                    alternatives,
                                    missing,
                                    values,
                                    value,
                                    absence,
                                    at,
                                );
                                match ml_s1 {
                                    self::MlReadNullableTypeSyntaxLoop1Result::MlReadNullableTypeSyntaxLoop1Done(missing_2, values_2, value_2, absence_2) => {
                                        if (missing_2 > 0f64) && (values_2 == 1f64) {
                                            vec![1f64, value_2, if missing_2 > 1f64 {
                                                3f64
                                            } else {
                                                absence_2
                                            }]
                                        } else {
                                            vec![2f64, 0f64, 0f64]
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
        {
            let name = self::read_required_text_fact(alternatives, 0f64);
            if name == "null" {
                vec![3f64, 0f64, 1f64]
            } else if name == "undefined" {
                vec![3f64, 0f64, 2f64]
            } else if first == 63f64 {
                vec![4f64, 0f64, 1f64]
            } else {
                vec![0f64, 0f64, 0f64]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_nullable_type_absence(code: f64) -> String {
    if code == 1f64 {
        String::from("null")
    } else if code == 2f64 {
        String::from("undefined")
    } else if code == 3f64 {
        String::from("both")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_nullable_type_target_refusal(target: &str) -> String {
    format!(
        "{}{}",
        (format!("{}{}", (String::from("no ")), target.to_owned())),
        (String::from(" type for absence"))
    )
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadNullableTypeSyntaxLoop1Result {
    MlReadNullableTypeSyntaxLoop1Done(f64, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_nullable_type_syntax_loop1(
    alternatives: &[String],
    mut missing: f64,
    mut values: f64,
    mut value: f64,
    mut absence: f64,
    mut at: f64,
) -> self::MlReadNullableTypeSyntaxLoop1Result {
    loop {
        if at < (alternatives.len() as f64) {
            {
                let part = self::read_required_text_fact(alternatives, at);
                if (part == "null") || (part == "undefined") {
                    {
                        let missing_2 = missing + 1f64;
                        {
                            let absence_2 = if part == "null" { 1f64 } else { 2f64 };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (missing, absence, at) = (missing_2, absence_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    {
                        let values_2 = values + 1f64;
                        {
                            let value_2 = at;
                            {
                                let at_3 = at + 1f64;
                                {
                                    (values, value, at) = (values_2, value_2, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlReadNullableTypeSyntaxLoop1Result::MlReadNullableTypeSyntaxLoop1Done(
                missing, values, value, absence,
            )
        };
    }
}
