// Generated from complete js/src/translation/rust-array-element-owner-rules.js. Do not edit by hand.
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
pub fn select_rust_array_element_owner_index(identities: &[String], admitted: &[bool]) -> f64 {
    if ((identities.len() as f64) == 0f64) || ((identities.len() as f64) != (admitted.len() as f64))
    {
        -1f64
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s1 =
                    self::source_ml_temporary_select_rust_array_element_owner_index_loop1(
                        identities, admitted, index,
                    );
                match source_ml_source1_s1 {
                    self::MlTemporarySelectRustArrayElementOwnerIndexLoop1Result::MlTemporarySelectRustArrayElementOwnerIndexLoop1Done => {
                        0f64
                    }
                    self::MlTemporarySelectRustArrayElementOwnerIndexLoop1Result::MlTemporarySelectRustArrayElementOwnerIndexLoop1Return(source_ml_temporary_result1) => {
                        source_ml_temporary_result1
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySelectRustArrayElementOwnerIndexLoop1Result {
    MlTemporarySelectRustArrayElementOwnerIndexLoop1Done,
    MlTemporarySelectRustArrayElementOwnerIndexLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_select_rust_array_element_owner_index_loop1(
    identities: &[String],
    admitted: &[bool],
    mut index: f64,
) -> self::MlTemporarySelectRustArrayElementOwnerIndexLoop1Result {
    loop {
        return if index < (identities.len() as f64) {
            if !((self::ml_array::optional_at(admitted, self::ml_array::number_index(index)))
                .unwrap_or_default())
                || {
                    let source_ml_source1_comparison_operand2 = self::ml_array::optional_at(
                        identities,
                        self::ml_array::number_index(index),
                    );
                    {
                        let source_ml_source1_comparison_operand3 = self::ml_array::optional_at(
                            identities,
                            self::ml_array::number_index(0f64),
                        );
                        !(match (
                            source_ml_source1_comparison_operand2,
                            source_ml_source1_comparison_operand3,
                        ) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        })
                    }
                }
            {
                self::MlTemporarySelectRustArrayElementOwnerIndexLoop1Result::MlTemporarySelectRustArrayElementOwnerIndexLoop1Return(-1f64)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporarySelectRustArrayElementOwnerIndexLoop1Result::MlTemporarySelectRustArrayElementOwnerIndexLoop1Done
        };
    }
}
