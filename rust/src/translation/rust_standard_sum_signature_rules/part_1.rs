// Generated from complete js/src/translation/rust-standard-sum-signature-rules.js. Do not edit by hand.
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
pub fn read_rust_standard_sum_signature_argument_constructors(
    owner: &str,
    names: &[String],
    fields: &[f64],
) -> Vec<f64> {
    if ((names.len() as f64) != 2f64) || ((fields.len() as f64) != 2f64) {
        Vec::<f64>::new()
    } else {
        if ((((owner == "std::option::Option") && {
            let source_ml_comparison_operand1 =
                self::ml_array::optional_at(names, self::ml_array::number_index(0f64));
            {
                let source_ml_comparison_operand2 = String::from("None");
                match (source_ml_comparison_operand1, source_ml_comparison_operand2) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }) && {
            let source_ml_comparison_operand3 =
                self::ml_array::optional_at(names, self::ml_array::number_index(1f64));
            {
                let source_ml_comparison_operand4 = String::from("Some");
                match (source_ml_comparison_operand3, source_ml_comparison_operand4) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }) && {
            let source_ml_comparison_operand5 =
                self::ml_array::optional_at(fields, self::ml_array::number_index(0f64));
            {
                let source_ml_comparison_operand6 = 0f64;
                match (source_ml_comparison_operand5, source_ml_comparison_operand6) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        }) && {
            let source_ml_comparison_operand7 =
                self::ml_array::optional_at(fields, self::ml_array::number_index(1f64));
            {
                let source_ml_comparison_operand8 = 1f64;
                match (source_ml_comparison_operand7, source_ml_comparison_operand8) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } {
            vec![1f64]
        } else {
            if ((((owner == "std::result::Result") && {
                let source_ml_comparison_operand9 =
                    self::ml_array::optional_at(names, self::ml_array::number_index(0f64));
                {
                    let source_ml_comparison_operand10 = String::from("Ok");
                    match (
                        source_ml_comparison_operand9,
                        source_ml_comparison_operand10,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }) && {
                let source_ml_comparison_operand11 =
                    self::ml_array::optional_at(names, self::ml_array::number_index(1f64));
                {
                    let source_ml_comparison_operand12 = String::from("Err");
                    match (
                        source_ml_comparison_operand11,
                        source_ml_comparison_operand12,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }) && {
                let source_ml_comparison_operand13 =
                    self::ml_array::optional_at(fields, self::ml_array::number_index(0f64));
                {
                    let source_ml_comparison_operand14 = 1f64;
                    match (
                        source_ml_comparison_operand13,
                        source_ml_comparison_operand14,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }) && {
                let source_ml_comparison_operand15 =
                    self::ml_array::optional_at(fields, self::ml_array::number_index(1f64));
                {
                    let source_ml_comparison_operand16 = 1f64;
                    match (
                        source_ml_comparison_operand15,
                        source_ml_comparison_operand16,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            } {
                vec![0f64, 1f64]
            } else {
                Vec::<f64>::new()
            }
        }
    }
}
