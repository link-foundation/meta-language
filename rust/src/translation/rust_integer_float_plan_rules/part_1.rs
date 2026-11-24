// Generated from complete js/src/translation/rust-integer-float-plan-rules.js. Do not edit by hand.
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
pub fn read_rust_integer_float_plan_operation(code: f64) -> String {
    if code == 0f64 {
        String::from("input")
    } else if code == 1f64 {
        String::from("fixed zero")
    } else if code == 2f64 {
        String::from("fixed two")
    } else if code == 3f64 {
        String::from("floating zero")
    } else if code == 4f64 {
        String::from("floating one")
    } else if code == 5f64 {
        String::from("floating two")
    } else if code == 6f64 {
        String::from("fixed reference")
    } else if code == 7f64 {
        String::from("floating reference")
    } else if code == 8f64 {
        String::from("equal")
    } else if code == 9f64 {
        String::from("less")
    } else if code == 10f64 {
        String::from("remainder")
    } else if code == 11f64 {
        String::from("division")
    } else if code == 12f64 {
        String::from("negation")
    } else if code == 13f64 {
        String::from("addition")
    } else if code == 14f64 {
        String::from("multiplication")
    } else if code == 15f64 {
        String::from("conditional")
    } else if code == 16f64 {
        String::from("binding")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_integer_float_conversion(bits: f64) -> Vec<f64> {
    if ((bits != 8f64) && (bits != 16f64)) && (bits != 32f64) {
        Vec::<f64>::new()
    } else {
        {
            let plan = vec![
                0f64, 0f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64, 0f64, 2f64, 0f64, 0f64, 0f64,
                0f64, 3f64, 0f64, 0f64, 0f64, 0f64, 4f64, 0f64, 0f64, 0f64, 0f64, 5f64, 0f64, 0f64,
                0f64, 0f64,
            ];
            {
                let bindings = vec![0f64, 0f64, 1f64, 3f64, 2f64, 4f64];
                {
                    let current = 0f64;
                    {
                        let sum = 1f64;
                        {
                            let weight = 2f64;
                            {
                                let index = 6f64;
                                {
                                    let bit = 0f64;
                                    {
                                        let source_ml_source1_s1 = self::source_ml_temporary_plan_rust_integer_float_conversion_loop1(bits, plan, bindings, current, sum, weight, index, bit);
                                        match source_ml_source1_s1 {
                                            self::MlTemporaryPlanRustIntegerFloatConversionLoop1Result::MlTemporaryPlanRustIntegerFloatConversionLoop1Done(plan_2, bindings_2, sum_2, index_2) => {
                                                {
                                                    let plan_3 = {
                                                        let source_ml_source1_concat_value2 = plan_2;
                                                        {
                                                            let source_ml_source1_concat_value3 = vec![7f64, sum_2, 0f64, 0f64, 0f64];
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value2), source_ml_source1_concat_value3)
                                                        }
                                                    };
                                                    {
                                                        let body = index_2;
                                                        {
                                                            let index_3 = index_2 + 1f64;
                                                            {
                                                                let at = (bindings_2.len() as f64) - 2f64;
                                                                {
                                                                    self::source_ml_temporary_plan_rust_integer_float_conversion_loop2(plan_3, &(bindings_2), index_3, body, at)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
pub fn source_ml_temporary_plan_rust_integer_float_conversion_loop2(
    mut plan: Vec<f64>,
    bindings: &[f64],
    mut index: f64,
    mut body: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at >= 0f64 {
            {
                let plan_2 = {
                    let source_ml_source1_concat_value4 = plan;
                    {
                        let source_ml_source1_concat_value5 = vec![
                            16f64,
                            (self::ml_array::optional_at(
                                bindings,
                                self::ml_array::number_index(at),
                            ))
                            .unwrap_or_else(|| -1f64),
                            (self::ml_array::optional_at(
                                bindings,
                                self::ml_array::number_index(at + 1f64),
                            ))
                            .unwrap_or_else(|| -1f64),
                            body,
                            0f64,
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source1_concat_value4,
                            ),
                            source_ml_source1_concat_value5,
                        )
                    }
                };
                {
                    let body_2 = index;
                    {
                        let index_2 = index + 1f64;
                        {
                            let at_2 = at - 2f64;
                            {
                                (plan, index, body, at) = (plan_2, index_2, body_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { plan };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustIntegerFloatConversionLoop1Result {
    MlTemporaryPlanRustIntegerFloatConversionLoop1Done(Vec<f64>, Vec<f64>, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_rust_integer_float_conversion_loop1(
    mut bits: f64,
    mut plan: Vec<f64>,
    mut bindings: Vec<f64>,
    mut current: f64,
    mut sum: f64,
    mut weight: f64,
    mut index: f64,
    mut bit: f64,
) -> self::MlTemporaryPlanRustIntegerFloatConversionLoop1Result {
    loop {
        if bit < bits {
            {
                let at = index;
                {
                    let next_current = 3f64 + (bit * 3f64);
                    {
                        let next_sum = 4f64 + (bit * 3f64);
                        {
                            let next_weight = 5f64 + (bit * 3f64);
                            {
                                let plan_2 = {
                                    let source_ml_source1_concat_value6 = plan;
                                    {
                                        let source_ml_source1_concat_value7 = vec![
                                            6f64,
                                            current,
                                            0f64,
                                            0f64,
                                            0f64,
                                            7f64,
                                            sum,
                                            0f64,
                                            0f64,
                                            0f64,
                                            7f64,
                                            weight,
                                            0f64,
                                            0f64,
                                            0f64,
                                            10f64,
                                            at,
                                            2f64,
                                            0f64,
                                            0f64,
                                            8f64,
                                            (at + 3f64),
                                            1f64,
                                            0f64,
                                            0f64,
                                            9f64,
                                            at,
                                            1f64,
                                            0f64,
                                            0f64,
                                            12f64,
                                            (at + 2f64),
                                            0f64,
                                            0f64,
                                            0f64,
                                            15f64,
                                            (at + 5f64),
                                            (at + 6f64),
                                            (at + 2f64),
                                            0f64,
                                            15f64,
                                            (at + 4f64),
                                            3f64,
                                            (at + 7f64),
                                            0f64,
                                            13f64,
                                            (at + 1f64),
                                            (at + 8f64),
                                            0f64,
                                            0f64,
                                            11f64,
                                            at,
                                            2f64,
                                            0f64,
                                            0f64,
                                            14f64,
                                            (at + 2f64),
                                            5f64,
                                            0f64,
                                            0f64,
                                        ];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source1_concat_value6,
                                            ),
                                            source_ml_source1_concat_value7,
                                        )
                                    }
                                };
                                {
                                    let bindings_2 = {
                                        let source_ml_source1_concat_value8 = bindings;
                                        {
                                            let source_ml_source1_concat_value9 = vec![
                                                next_sum,
                                                (at + 9f64),
                                                next_current,
                                                (at + 10f64),
                                                next_weight,
                                                (at + 11f64),
                                            ];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source1_concat_value8,
                                                ),
                                                source_ml_source1_concat_value9,
                                            )
                                        }
                                    };
                                    {
                                        let current_2 = next_current;
                                        {
                                            let sum_2 = next_sum;
                                            {
                                                let weight_2 = next_weight;
                                                {
                                                    let index_2 = index + 12f64;
                                                    {
                                                        let bit_2 = bit + 1f64;
                                                        {
                                                            (
                                                                plan, bindings, current, sum,
                                                                weight, index, bit,
                                                            ) = (
                                                                plan_2, bindings_2, current_2,
                                                                sum_2, weight_2, index_2, bit_2,
                                                            );
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
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanRustIntegerFloatConversionLoop1Result::MlTemporaryPlanRustIntegerFloatConversionLoop1Done(plan, bindings, sum, index)
        };
    }
}
