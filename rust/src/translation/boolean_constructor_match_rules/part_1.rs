// Generated from complete js/src/translation/boolean-constructor-match-rules.js. Do not edit by hand.
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
pub fn plan_boolean_constructor_match_sharing(
    owner: &str,
    declared: &[String],
    owners: &[String],
    constructors: &[String],
    literals: &[f64],
) -> Vec<f64> {
    {
        let count = declared.len() as f64;
        if ((((((owner).is_empty()) || (count < 2f64)) || (count > 256f64))
            || ((owners.len() as f64) != count))
            || ((constructors.len() as f64) != count))
            || ((literals.len() as f64) != count)
        {
            Vec::<f64>::new()
        } else {
            {
                let covered = Vec::<String>::new();
                {
                    let zero = 0f64;
                    {
                        let one = 0f64;
                        {
                            let index = 0f64;
                            {
                                let source_ml_source1_s1 = self::source_ml_temporary_plan_boolean_constructor_match_sharing_loop1(declared, count, index);
                                match source_ml_source1_s1 {
                                    self::MlTemporaryPlanBooleanConstructorMatchSharingLoop1Result::MlTemporaryPlanBooleanConstructorMatchSharingLoop1Done => {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let source_ml_source1_s2 = self::source_ml_temporary_plan_boolean_constructor_match_sharing_loop2(owner, declared, owners, constructors, literals, count, covered, zero, one, index_2);
                                                match source_ml_source1_s2 {
                                                    self::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Result::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Done(zero_2, one_2) => {
                                                        {
                                                            let fallback = if zero_2 >= one_2 {
                                                                0f64
                                                            } else {
                                                                1f64
                                                            };
                                                            if (self::ml_math::max(zero_2, one_2)) < 2f64 {
                                                                Vec::<f64>::new()
                                                            } else {
                                                                {
                                                                    let plan = vec![fallback];
                                                                    {
                                                                        let index_3 = 0f64;
                                                                        {
                                                                            self::source_ml_temporary_plan_boolean_constructor_match_sharing_loop3(literals, count, fallback, plan, index_3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Result::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Return(source_ml_temporary_result3) => {
                                                        source_ml_temporary_result3
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanBooleanConstructorMatchSharingLoop1Result::MlTemporaryPlanBooleanConstructorMatchSharingLoop1Return(source_ml_temporary_result3_2) => {
                                        source_ml_temporary_result3_2
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
pub fn source_ml_temporary_plan_boolean_constructor_match_sharing_loop3(
    literals: &[f64],
    mut count: f64,
    mut fallback: f64,
    mut plan: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if index < count {
            if {
                let source_ml_source1_comparison_operand3 =
                    self::ml_array::optional_at(literals, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand4 = fallback;
                    !(match (
                        source_ml_source1_comparison_operand3,
                        source_ml_source1_comparison_operand4,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                {
                    let plan_2 = {
                        let (ml_append_right, ml_append_left) = (vec![index], plan);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            (plan, index) = (plan_2, index_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            plan
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanBooleanConstructorMatchSharingLoop2Result {
    MlTemporaryPlanBooleanConstructorMatchSharingLoop2Done(f64, f64),
    MlTemporaryPlanBooleanConstructorMatchSharingLoop2Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_boolean_constructor_match_sharing_loop2(
    owner: &str,
    declared: &[String],
    owners: &[String],
    constructors: &[String],
    literals: &[f64],
    mut count: f64,
    mut covered: Vec<String>,
    mut zero: f64,
    mut one: f64,
    mut index: f64,
) -> self::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Result {
    loop {
        return if index < count {
            {
                let constructor = (self::ml_array::optional_at(
                    constructors,
                    self::ml_array::number_index(index),
                ))
                .unwrap_or_else(|| String::new());
                {
                    let literal = (self::ml_array::optional_at(
                        literals,
                        self::ml_array::number_index(index),
                    ))
                    .unwrap_or_else(|| -1f64);
                    if ({
                        let source_ml_source1_comparison_operand5 = self::ml_array::optional_at(
                            owners,
                            self::ml_array::number_index(index),
                        );
                        {
                            let source_ml_source1_comparison_operand6 = owner.to_owned();
                            !(match (
                                source_ml_source1_comparison_operand5,
                                source_ml_source1_comparison_operand6,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    } || !(self::ml_array::includes(
                        declared,
                        &constructor.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    ))) || self::ml_array::includes(
                        &covered,
                        &constructor.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    ) {
                        self::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Result::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Return(Vec::<f64>::new())
                    } else {
                        {
                            let covered_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![constructor.clone()], covered);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            if literal == 0f64 {
                                {
                                    let zero_2 = zero + 1f64;
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (covered, zero, index) = (covered_2, zero_2, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                if literal == 1f64 {
                                    {
                                        let one_2 = one + 1f64;
                                        {
                                            let index_3 = index + 1f64;
                                            {
                                                (covered, one, index) = (covered_2, one_2, index_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    self::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Result::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Return(Vec::<f64>::new())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Result::MlTemporaryPlanBooleanConstructorMatchSharingLoop2Done(zero, one)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanBooleanConstructorMatchSharingLoop1Result {
    MlTemporaryPlanBooleanConstructorMatchSharingLoop1Done,
    MlTemporaryPlanBooleanConstructorMatchSharingLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_boolean_constructor_match_sharing_loop1(
    declared: &[String],
    mut count: f64,
    mut index: f64,
) -> self::MlTemporaryPlanBooleanConstructorMatchSharingLoop1Result {
    loop {
        return if index < count {
            {
                let declaration =
                    (self::ml_array::optional_at(declared, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                if (declaration == "")
                    || ((self::ml_array::index_of(
                        declared,
                        &declaration.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )) != index)
                {
                    self::MlTemporaryPlanBooleanConstructorMatchSharingLoop1Result::MlTemporaryPlanBooleanConstructorMatchSharingLoop1Return(Vec::<f64>::new())
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
            self::MlTemporaryPlanBooleanConstructorMatchSharingLoop1Result::MlTemporaryPlanBooleanConstructorMatchSharingLoop1Done
        };
    }
}
