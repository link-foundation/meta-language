// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
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
pub fn accept_owned_outcome_static_borrow_resources(entries: f64, bytes: f64) -> bool {
    ((((((entries.is_finite() && ((entries.trunc()) == entries)) && (entries >= 0f64))
        && (entries <= 128f64))
        && bytes.is_finite())
        && ((bytes.trunc()) == bytes))
        && (bytes >= 0f64))
        && (bytes <= 1_048_576f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_owned_outcome_static_borrow_index(text: &str, values: &[String], bytes: f64) -> f64 {
    if (self::accept_owned_outcome_static_borrow_resources((values.len() as f64), bytes)) {
        {
            let selected = -1f64;
            {
                let index = 0f64;
                {
                    let ml_s1 = self::ml_find_owned_outcome_static_borrow_index_loop1(
                        text, values, selected, index,
                    );
                    match ml_s1 {
                        self::MlFindOwnedOutcomeStaticBorrowIndexLoop1Result::MlFindOwnedOutcomeStaticBorrowIndexLoop1Done(selected_2) => {
                            selected_2
                        }
                        self::MlFindOwnedOutcomeStaticBorrowIndexLoop1Result::MlFindOwnedOutcomeStaticBorrowIndexLoop1Return(ml_result1) => {
                            ml_result1
                        }
                    }
                }
            }
        }
    } else {
        -3f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_owned_outcome_static_borrow_failure(code: f64) -> String {
    if code == -1f64 {
        String::from("no existing static reference matches the source value")
    } else if code == -2f64 {
        String::from("duplicate static reference labels are ambiguous")
    } else if code == -3f64 {
        String::from("static reference registry exceeds its resource limit")
    } else {
        String::from("invalid static reference selection")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindOwnedOutcomeStaticBorrowIndexLoop1Result {
    MlFindOwnedOutcomeStaticBorrowIndexLoop1Done(f64),
    MlFindOwnedOutcomeStaticBorrowIndexLoop1Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindOwnedOutcomeStaticBorrowIndexLoop2Result {
    MlFindOwnedOutcomeStaticBorrowIndexLoop2Done,
    MlFindOwnedOutcomeStaticBorrowIndexLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_find_owned_outcome_static_borrow_index_loop2(
    values: &[String],
    mut index: f64,
    mut previous: f64,
) -> self::MlFindOwnedOutcomeStaticBorrowIndexLoop2Result {
    loop {
        return if previous < index {
            if match (
                self::ml_array::optional_at(values, self::ml_array::number_index(previous)),
                self::ml_array::optional_at(values, self::ml_array::number_index(index)),
            ) {
                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                (None, None) => true,
                _ => false,
            } {
                self::MlFindOwnedOutcomeStaticBorrowIndexLoop2Result::MlFindOwnedOutcomeStaticBorrowIndexLoop2Return(-2f64)
            } else {
                {
                    let previous_2 = previous + 1f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlFindOwnedOutcomeStaticBorrowIndexLoop2Result::MlFindOwnedOutcomeStaticBorrowIndexLoop2Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_find_owned_outcome_static_borrow_index_loop1(
    text: &str,
    values: &[String],
    mut selected: f64,
    mut index: f64,
) -> self::MlFindOwnedOutcomeStaticBorrowIndexLoop1Result {
    loop {
        return if index < (values.len() as f64) {
            {
                let previous = 0f64;
                {
                    let ml_s2 = self::ml_find_owned_outcome_static_borrow_index_loop2(
                        values, index, previous,
                    );
                    match ml_s2 {
                        self::MlFindOwnedOutcomeStaticBorrowIndexLoop2Result::MlFindOwnedOutcomeStaticBorrowIndexLoop2Done => {
                            {
                                let selected_3 = if match (self::ml_array::optional_at(values, self::ml_array::number_index(index)), text.to_owned()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                    {
                                        index
                                    }
                                } else {
                                    selected
                                };
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (selected, index) = (selected_3, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        self::MlFindOwnedOutcomeStaticBorrowIndexLoop2Result::MlFindOwnedOutcomeStaticBorrowIndexLoop2Return(ml_result2) => {
                            self::MlFindOwnedOutcomeStaticBorrowIndexLoop1Result::MlFindOwnedOutcomeStaticBorrowIndexLoop1Return(ml_result2)
                        }
                    }
                }
            }
        } else {
            self::MlFindOwnedOutcomeStaticBorrowIndexLoop1Result::MlFindOwnedOutcomeStaticBorrowIndexLoop1Done(selected)
        };
    }
}
