// Generated from complete js/src/translation/javascript-nominal-spread-rules.js. Do not edit by hand.
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_nominal_spread_error(
    language: &str,
    constructors: f64,
    declared: &[String],
    kinds: &[String],
    names: &[String],
    owners: &[String],
    owner: &str,
    prototype: bool,
) -> String {
    {
        let ml_library_inequality_1 = language != "JavaScript";
        if ml_library_inequality_1 {
            String::from("nominal object copies require their actual JavaScript source owner")
        } else if (constructors != 1f64) || ((owner.encode_utf16().count() as f64) == 0f64) {
            String::from("nominal object copies need one complete actual constructor")
        } else if prototype {
            String::from(
                "nominal object copies cannot replace their complete prototype interpretation",
            )
        } else {
            if (((((kinds.len() as f64) == 0f64) || ((kinds.len() as f64) > 65_536f64))
                || ((kinds.len() as f64) != (names.len() as f64)))
                || ((kinds.len() as f64) != (owners.len() as f64)))
                || {
                    let source_ml_source1_comparison_operand1 =
                        self::ml_array::optional_at(kinds, self::ml_array::number_index(0f64));
                    {
                        let source_ml_source1_comparison_operand2 = String::from("spread");
                        !(match (
                            source_ml_source1_comparison_operand1,
                            source_ml_source1_comparison_operand2,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                }
            {
                String::from("nominal object copies start with a complete actual source value")
            } else if (declared.len() as f64) > 65_536f64 {
                String::from("nominal object field directories exceed the explicit source budget")
            } else {
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s3 =
                            self::source_ml_temporary_read_java_script_nominal_spread_error_loop1(
                                declared, index,
                            );
                        match source_ml_source1_s3 {
                                    self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Result::MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Done => {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let source_ml_source1_s4 = self::source_ml_temporary_read_java_script_nominal_spread_error_loop2(declared, kinds, names, owners, owner, index_2);
                                                match source_ml_source1_s4 {
                                                    self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Result::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Done => {
                                                        String::new()
                                                    }
                                                    self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Result::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Return(source_ml_temporary_result2) => {
                                                        source_ml_temporary_result2.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Result::MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Return(source_ml_temporary_result2_2) => {
                                        source_ml_temporary_result2_2.clone()
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
pub fn read_java_script_nominal_spread_guard_arm(
    kind: &str,
    operation: &str,
    absence: &str,
) -> f64 {
    if (kind != "binary") || ((absence != "undefined") && (absence != "null")) {
        0f64
    } else if operation == "ne" {
        1f64
    } else if operation == "eq" {
        2f64
    } else {
        0f64
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Result {
    MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Done,
    MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_nominal_spread_error_loop2(
    declared: &[String],
    kinds: &[String],
    names: &[String],
    owners: &[String],
    owner: &str,
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let kind =
                    (self::ml_array::optional_at(kinds, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                {
                    let name =
                        (self::ml_array::optional_at(names, self::ml_array::number_index(index)))
                            .unwrap_or_else(|| String::new());
                    {
                        let source = (self::ml_array::optional_at(
                            owners,
                            self::ml_array::number_index(index),
                        ))
                        .unwrap_or_else(|| String::new());
                        if kind == "spread" {
                            if (name != "") || (source != owner) {
                                self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Result::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Return(String::from("nominal object copies retain their actual complete source owner"))
                            } else {
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        index = index_2;
                                        continue;
                                    }
                                }
                            }
                        } else if ((kind != "field") || (source != ""))
                            || !(self::ml_array::includes(
                                declared,
                                &name.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            ))
                        {
                            self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Result::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Return(String::from("nominal object copies cannot add fields or replace their constructor tag"))
                        } else {
                            {
                                let index_3 = index + 1f64;
                                {
                                    index = index_3;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Result::MlTemporaryReadJavaScriptNominalSpreadErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Result {
    MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Done,
    MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_nominal_spread_error_loop1(
    declared: &[String],
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Result {
    loop {
        return if index < (declared.len() as f64) {
            {
                let name =
                    (self::ml_array::optional_at(declared, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                if (((name.encode_utf16().count() as f64) == 0f64) || (name == "$"))
                    || ((self::ml_array::index_of(declared, &name.clone(), 0f64, false, |a, b| {
                        a == b
                    })) != index)
                {
                    self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Result::MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Return(String::from("nominal object copies require complete unique declared fields"))
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Result::MlTemporaryReadJavaScriptNominalSpreadErrorLoop1Done
        };
    }
}
