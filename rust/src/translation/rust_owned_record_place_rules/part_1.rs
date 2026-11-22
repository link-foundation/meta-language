// Generated from complete js/src/translation/rust-owned-record-place-rules.js. Do not edit by hand.
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
pub fn read_rust_owned_record_place_depth_limit() -> f64 {
    256f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_record_place_token_count(forms: &[String]) -> f64 {
    {
        let depth = 0f64;
        {
            let atom = true;
            {
                let at = 0f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_read_rust_owned_record_place_token_count_loop1(
                            forms, depth, atom, at,
                        );
                    match source_ml_source1_s1 {
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Done => {
                            -1f64
                        }
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result {
    MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Done,
    MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_record_place_token_count_loop1(
    forms: &[String],
    mut depth: f64,
    mut atom: bool,
    mut at: f64,
) -> self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result {
    loop {
        return if at < (forms.len() as f64) {
            {
                let form = self::ml_array::optional_at(forms, self::ml_array::number_index(at));
                if atom {
                    if {
                        let source_ml_source1_comparison_operand2 = form.clone();
                        {
                            let source_ml_source1_comparison_operand3 = String::from("(");
                            match (
                                source_ml_source1_comparison_operand2,
                                source_ml_source1_comparison_operand3,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        }
                    } {
                        {
                            let depth_2 = depth + 1f64;
                            if depth_2 >= (self::read_rust_owned_record_place_depth_limit()) {
                                self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(-1f64)
                            } else {
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (depth, at) = (depth_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else if {
                        let source_ml_source1_comparison_operand4 = form.clone();
                        {
                            let source_ml_source1_comparison_operand5 = String::from("name");
                            match (
                                source_ml_source1_comparison_operand4,
                                source_ml_source1_comparison_operand5,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        }
                    } {
                        {
                            let atom_2 = false;
                            {
                                let at_3 = at + 1f64;
                                {
                                    (atom, at) = (atom_2, at_3);
                                    continue;
                                }
                            }
                        }
                    } else {
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(-1f64)
                    }
                } else if {
                    let source_ml_source1_comparison_operand6 = form.clone();
                    {
                        let source_ml_source1_comparison_operand7 = String::from(".");
                        match (
                            source_ml_source1_comparison_operand6,
                            source_ml_source1_comparison_operand7,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } {
                    {
                        let at_4 = at + 1f64;
                        if {
                            let source_ml_source1_comparison_operand8 = self::ml_array::optional_at(
                                forms,
                                self::ml_array::number_index(at_4),
                            );
                            {
                                let source_ml_source1_comparison_operand9 = String::from("name");
                                !(match (
                                    source_ml_source1_comparison_operand8,
                                    source_ml_source1_comparison_operand9,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                })
                            }
                        } && {
                            let source_ml_source1_comparison_operand10 =
                                self::ml_array::optional_at(
                                    forms,
                                    self::ml_array::number_index(at_4),
                                );
                            {
                                let source_ml_source1_comparison_operand11 = String::from("number");
                                !(match (
                                    source_ml_source1_comparison_operand10,
                                    source_ml_source1_comparison_operand11,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                })
                            }
                        } {
                            self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(-1f64)
                        } else {
                            {
                                let at_5 = at_4 + 1f64;
                                {
                                    at = at_5;
                                    continue;
                                }
                            }
                        }
                    }
                } else if {
                    let source_ml_source1_comparison_operand12 = form.clone();
                    {
                        let source_ml_source1_comparison_operand13 = String::from(")");
                        match (
                            source_ml_source1_comparison_operand12,
                            source_ml_source1_comparison_operand13,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } {
                    if depth == 0f64 {
                        self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(-1f64)
                    } else {
                        {
                            let depth_3 = depth - 1f64;
                            {
                                let at_6 = at + 1f64;
                                {
                                    (depth, at) = (depth_3, at_6);
                                    continue;
                                }
                            }
                        }
                    }
                } else if {
                    let source_ml_source1_comparison_operand14 = form.clone();
                    {
                        let source_ml_source1_comparison_operand15 = String::from("}");
                        match (
                            source_ml_source1_comparison_operand14,
                            source_ml_source1_comparison_operand15,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    }
                } {
                    self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(if depth == 0f64 {
                                    at
                                } else {
                                    -1f64
                                })
                } else {
                    self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Return(-1f64)
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Result::MlTemporaryReadRustOwnedRecordPlaceTokenCountLoop1Done
        };
    }
}
