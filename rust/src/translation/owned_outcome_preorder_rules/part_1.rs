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
pub fn check_owned_outcome_preorder(
    offsets: &[f64],
    children: &[f64],
    node_limit: f64,
    edge_limit: f64,
    depth_limit: f64,
) -> Vec<f64> {
    if ((((!(self::ml_math::is_safe_integer(node_limit)) || (node_limit < 1f64))
        || !(self::ml_math::is_safe_integer(edge_limit)))
        || (edge_limit < 1f64))
        || !(self::ml_math::is_safe_integer(depth_limit)))
        || (depth_limit < 1f64)
    {
        vec![-1f64, 0f64, 0f64, 0f64]
    } else {
        {
            let count = (offsets.len() as f64) - 1f64;
            if ((((count < 1f64) || (count > node_limit))
                || ((children.len() as f64) > edge_limit))
                || !(match (
                    self::ml_array::optional_at(offsets, self::ml_array::number_index(0f64)),
                    0f64,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }))
                || !(match (
                    self::ml_array::optional_at(offsets, self::ml_array::number_index(count)),
                    (children.len() as f64),
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })
            {
                vec![-2f64, 0f64, 0f64, 0f64]
            } else {
                {
                    let at = 0f64;
                    {
                        let ml_s1 =
                            self::ml_check_owned_outcome_preorder_loop1(offsets, children, at);
                        match ml_s1 {
                            self::MlCheckOwnedOutcomePreorderLoop1Result::MlCheckOwnedOutcomePreorderLoop1Done => {
                                {
                                    let identities = vec![1f64];
                                    {
                                        let depths = vec![1f64];
                                        {
                                            let successors = vec![-1f64];
                                            {
                                                let head = 0f64;
                                                {
                                                    let visited = 0f64;
                                                    {
                                                        let maximum_depth = 0f64;
                                                        {
                                                            let ml_s2 = self::ml_check_owned_outcome_preorder_loop2(offsets, children, node_limit, depth_limit, count, identities, depths, successors, head, visited, maximum_depth);
                                                            match ml_s2 {
                                                                self::MlCheckOwnedOutcomePreorderLoop2Result::MlCheckOwnedOutcomePreorderLoop2Done(visited_2, maximum_depth_2) => {
                                                                    {
let ml_library_inequality_1 = visited_2 != count ;
if ml_library_inequality_1 {
                                                                        vec![-6f64, 0f64, 0f64, 0f64]
                                                                    } else {
                                                                        vec![0f64, visited_2, (children.len() as f64), maximum_depth_2]
                                                                    }
}
                                                                }
                                                                self::MlCheckOwnedOutcomePreorderLoop2Result::MlCheckOwnedOutcomePreorderLoop2Return(ml_result2) => {
                                                                    ml_result2
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
                            self::MlCheckOwnedOutcomePreorderLoop1Result::MlCheckOwnedOutcomePreorderLoop1Return(ml_result3) => {
                                ml_result3
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomePreorderLoop2Result {
    MlCheckOwnedOutcomePreorderLoop2Done(f64, f64),
    MlCheckOwnedOutcomePreorderLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomePreorderLoop3Result {
    MlCheckOwnedOutcomePreorderLoop3Done(Vec<f64>, Vec<f64>, Vec<f64>, f64),
    MlCheckOwnedOutcomePreorderLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_check_owned_outcome_preorder_loop3(
    children: &[f64],
    mut count: f64,
    mut identities: Vec<f64>,
    mut depths: Vec<f64>,
    mut successors: Vec<f64>,
    mut head: f64,
    mut identity: f64,
    mut depth: f64,
    mut start: f64,
    mut at: f64,
) -> self::MlCheckOwnedOutcomePreorderLoop3Result {
    loop {
        return if at >= start {
            {
                let child =
                    (self::ml_array::optional_at(children, self::ml_array::number_index(at)))
                        .unwrap_or(0f64);
                if (!(self::ml_math::is_safe_integer(child)) || (child <= identity))
                    || (child > count)
                {
                    self::MlCheckOwnedOutcomePreorderLoop3Result::MlCheckOwnedOutcomePreorderLoop3Return(vec![-4f64, 0f64, 0f64, 0f64])
                } else {
                    {
                        let identities_2 = {
                            let (ml_append_right, ml_append_left) = (vec![child], identities);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let depths_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![(depth + 1f64)], depths);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let successors_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![head], successors);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let head_2 = (identities_2.len() as f64) - 1f64;
                                    {
                                        let at_2 = at - 1f64;
                                        {
                                            (identities, depths, successors, head, at) = (
                                                identities_2,
                                                depths_2,
                                                successors_2,
                                                head_2,
                                                at_2,
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
        } else {
            self::MlCheckOwnedOutcomePreorderLoop3Result::MlCheckOwnedOutcomePreorderLoop3Done(
                identities, depths, successors, head,
            )
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_check_owned_outcome_preorder_loop2(
    offsets: &[f64],
    children: &[f64],
    mut node_limit: f64,
    mut depth_limit: f64,
    mut count: f64,
    mut identities: Vec<f64>,
    mut depths: Vec<f64>,
    mut successors: Vec<f64>,
    mut head: f64,
    mut visited: f64,
    mut maximum_depth: f64,
) -> self::MlCheckOwnedOutcomePreorderLoop2Result {
    loop {
        return if head >= 0f64 {
            {
                let identity =
                    (self::ml_array::optional_at(&identities, self::ml_array::number_index(head)))
                        .unwrap_or(0f64);
                {
                    let depth =
                        (self::ml_array::optional_at(&depths, self::ml_array::number_index(head)))
                            .unwrap_or(0f64);
                    {
                        let head_2 = (self::ml_array::optional_at(
                            &successors,
                            self::ml_array::number_index(head),
                        ))
                        .unwrap_or(-1f64);
                        if ((identity != (visited + 1f64)) || (identity > count)) || (depth < 1f64)
                        {
                            self::MlCheckOwnedOutcomePreorderLoop2Result::MlCheckOwnedOutcomePreorderLoop2Return(vec![-3f64, 0f64, 0f64, 0f64])
                        } else if depth > depth_limit {
                            self::MlCheckOwnedOutcomePreorderLoop2Result::MlCheckOwnedOutcomePreorderLoop2Return(vec![-5f64, 0f64, 0f64, 0f64])
                        } else {
                            {
                                let visited_2 = visited + 1f64;
                                {
                                    let maximum_depth_2 = self::ml_math::max(maximum_depth, depth);
                                    {
                                        let start = (self::ml_array::optional_at(
                                            offsets,
                                            self::ml_array::number_index(identity - 1f64),
                                        ))
                                        .unwrap_or(-1f64);
                                        {
                                            let end = (self::ml_array::optional_at(
                                                offsets,
                                                self::ml_array::number_index(identity),
                                            ))
                                            .unwrap_or(-1f64);
                                            if (((identities.len() as f64) + end) - start)
                                                > node_limit
                                            {
                                                self::MlCheckOwnedOutcomePreorderLoop2Result::MlCheckOwnedOutcomePreorderLoop2Return(vec![-4f64, 0f64, 0f64, 0f64])
                                            } else {
                                                {
                                                    let at = end - 1f64;
                                                    {
                                                        let ml_s3 = self::ml_check_owned_outcome_preorder_loop3(children, count, identities, depths, successors, head_2, identity, depth, start, at);
                                                        match ml_s3 {
                                                                self::MlCheckOwnedOutcomePreorderLoop3Result::MlCheckOwnedOutcomePreorderLoop3Done(identities_2, depths_2, successors_2, head_3) => {
                                                                    {
                                                                        (identities, depths, successors, head, visited, maximum_depth) = (identities_2, depths_2, successors_2, head_3, visited_2, maximum_depth_2);
                                                                        continue;
                                                                    }
                                                                }
                                                                self::MlCheckOwnedOutcomePreorderLoop3Result::MlCheckOwnedOutcomePreorderLoop3Return(ml_result3) => {
                                                                    self::MlCheckOwnedOutcomePreorderLoop2Result::MlCheckOwnedOutcomePreorderLoop2Return(ml_result3)
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
        } else {
            self::MlCheckOwnedOutcomePreorderLoop2Result::MlCheckOwnedOutcomePreorderLoop2Done(
                visited,
                maximum_depth,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomePreorderLoop1Result {
    MlCheckOwnedOutcomePreorderLoop1Done,
    MlCheckOwnedOutcomePreorderLoop1Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_check_owned_outcome_preorder_loop1(
    offsets: &[f64],
    children: &[f64],
    mut at: f64,
) -> self::MlCheckOwnedOutcomePreorderLoop1Result {
    loop {
        return if at < (offsets.len() as f64) {
            {
                let offset =
                    (self::ml_array::optional_at(offsets, self::ml_array::number_index(at)))
                        .unwrap_or(-1f64);
                if ((!(self::ml_math::is_safe_integer(offset)) || (offset < 0f64))
                    || (offset > (children.len() as f64)))
                    || ((at > 0f64)
                        && (offset
                            < ((self::ml_array::optional_at(
                                offsets,
                                self::ml_array::number_index(at - 1f64),
                            ))
                            .unwrap_or(-1f64))))
                {
                    self::MlCheckOwnedOutcomePreorderLoop1Result::MlCheckOwnedOutcomePreorderLoop1Return(vec![-2f64, 0f64, 0f64, 0f64])
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlCheckOwnedOutcomePreorderLoop1Result::MlCheckOwnedOutcomePreorderLoop1Done
        };
    }
}
