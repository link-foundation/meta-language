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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn check_owned_outcome_source_types(
    types: &[f64],
    selectors: &[f64],
    offsets: &[f64],
    children: &[f64],
    modes: &[f64],
    patterns: &[f64],
    pattern_offsets: &[f64],
    pattern_types: &[f64],
    variant_owners: &[f64],
) -> f64 {
    {
        let count = types.len() as f64;
        {
            let type_count = modes.len() as f64;
            {
                let pattern_count = (pattern_offsets.len() as f64) - 1f64;
                if ((((((((count < 1f64) || ((selectors.len() as f64) != count))
                    || ((offsets.len() as f64) != (count + 1f64)))
                    || (type_count < 1f64))
                    || ((patterns.len() as f64) != type_count))
                    || (pattern_count < 1f64))
                    || ((variant_owners.len() as f64) != pattern_count))
                    || !(match (
                        self::ml_array::optional_at(
                            pattern_offsets,
                            self::ml_array::number_index(0f64),
                        ),
                        0f64,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }))
                    || !(match (
                        self::ml_array::optional_at(
                            pattern_offsets,
                            self::ml_array::number_index(pattern_count),
                        ),
                        (pattern_types.len() as f64),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                {
                    -1f64
                } else {
                    {
                        let at = 0f64;
                        {
                            let ml_s1 = self::ml_check_owned_outcome_source_types_loop1(
                                pattern_offsets,
                                pattern_types,
                                at,
                            );
                            match ml_s1 {
                                self::MlCheckOwnedOutcomeSourceTypesLoop1Result::MlCheckOwnedOutcomeSourceTypesLoop1Done => {
                                    {
                                        let at_2 = 0f64;
                                        {
                                            let ml_s2 = self::ml_check_owned_outcome_source_types_loop2(pattern_types, type_count, at_2);
                                            match ml_s2 {
                                                self::MlCheckOwnedOutcomeSourceTypesLoop2Result::MlCheckOwnedOutcomeSourceTypesLoop2Done => {
                                                    {
                                                        let at_3 = 0f64;
                                                        {
                                                            let ml_s3 = self::ml_check_owned_outcome_source_types_loop3(modes, variant_owners, type_count, pattern_count, at_3);
                                                            match ml_s3 {
                                                                self::MlCheckOwnedOutcomeSourceTypesLoop3Result::MlCheckOwnedOutcomeSourceTypesLoop3Done => {
                                                                    {
                                                                        let at_4 = 0f64;
                                                                        {
                                                                            let ml_s4 = self::ml_check_owned_outcome_source_types_loop4(modes, patterns, pattern_offsets, variant_owners, type_count, pattern_count, at_4);
                                                                            match ml_s4 {
                                                                                self::MlCheckOwnedOutcomeSourceTypesLoop4Result::MlCheckOwnedOutcomeSourceTypesLoop4Done => {
                                                                                    if !(match (self::ml_array::optional_at(offsets, self::ml_array::number_index(0f64)), 0f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) || !(match (self::ml_array::optional_at(offsets, self::ml_array::number_index(count)), (children.len() as f64)) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) {
                                                                                        -2f64
                                                                                    } else {
                                                                                        {
                                                                                            let at_5 = 0f64;
                                                                                            {
                                                                                                let ml_s5 = self::ml_check_owned_outcome_source_types_loop5(offsets, children, at_5);
                                                                                                match ml_s5 {
                                                                                                    self::MlCheckOwnedOutcomeSourceTypesLoop5Result::MlCheckOwnedOutcomeSourceTypesLoop5Done => {
                                                                                                        {
                                                                                                            let at_6 = 0f64;
                                                                                                            {
                                                                                                                let ml_s6 = self::ml_check_owned_outcome_source_types_loop6(types, selectors, offsets, children, modes, patterns, pattern_offsets, pattern_types, variant_owners, count, type_count, pattern_count, at_6);
                                                                                                                match ml_s6 {
                                                                                                                    self::MlCheckOwnedOutcomeSourceTypesLoop6Result::MlCheckOwnedOutcomeSourceTypesLoop6Done => {
                                                                                                                        0f64
                                                                                                                    }
                                                                                                                    self::MlCheckOwnedOutcomeSourceTypesLoop6Result::MlCheckOwnedOutcomeSourceTypesLoop6Return(ml_result6) => {
                                                                                                                        ml_result6
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlCheckOwnedOutcomeSourceTypesLoop5Result::MlCheckOwnedOutcomeSourceTypesLoop5Return(ml_result8) => {
                                                                                                        ml_result8
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlCheckOwnedOutcomeSourceTypesLoop4Result::MlCheckOwnedOutcomeSourceTypesLoop4Return(ml_result8_2) => {
                                                                                    ml_result8_2
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlCheckOwnedOutcomeSourceTypesLoop3Result::MlCheckOwnedOutcomeSourceTypesLoop3Return(ml_result8_3) => {
                                                                    ml_result8_3
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlCheckOwnedOutcomeSourceTypesLoop2Result::MlCheckOwnedOutcomeSourceTypesLoop2Return(ml_result8_4) => {
                                                    ml_result8_4
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlCheckOwnedOutcomeSourceTypesLoop1Result::MlCheckOwnedOutcomeSourceTypesLoop1Return(ml_result8_5) => {
                                    ml_result8_5
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomeSourceTypesLoop6Result {
    MlCheckOwnedOutcomeSourceTypesLoop6Done,
    MlCheckOwnedOutcomeSourceTypesLoop6Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomeSourceTypesLoop8Result {
    MlCheckOwnedOutcomeSourceTypesLoop8Done,
    MlCheckOwnedOutcomeSourceTypesLoop8Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_check_owned_outcome_source_types_loop8(
    types: &[f64],
    children: &[f64],
    pattern_types: &[f64],
    mut count: f64,
    mut type_: f64,
    mut mode: f64,
    mut start: f64,
    mut end: f64,
    mut first: f64,
    mut edge: f64,
) -> self::MlCheckOwnedOutcomeSourceTypesLoop8Result {
    loop {
        return if edge < end {
            {
                let child =
                    (self::ml_array::optional_at(children, self::ml_array::number_index(edge)))
                        .unwrap_or(0f64);
                if (!(self::ml_math::is_safe_integer(child)) || (child < 1f64)) || (child > count) {
                    self::MlCheckOwnedOutcomeSourceTypesLoop8Result::MlCheckOwnedOutcomeSourceTypesLoop8Return(-6f64)
                } else {
                    {
                        let expected = if mode == 5f64 {
                            type_
                        } else {
                            (self::ml_array::optional_at(
                                pattern_types,
                                self::ml_array::number_index(
                                    first
                                        + if (mode == 1f64) || (mode == 2f64) {
                                            0f64
                                        } else if mode == 3f64 {
                                            (edge - start) % 2f64
                                        } else {
                                            edge - start
                                        },
                                ),
                            ))
                            .unwrap_or(-1f64)
                        };
                        if (match (
                            self::ml_array::optional_at(
                                types,
                                self::ml_array::number_index(child - 1f64),
                            ),
                            expected,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }) {
                            {
                                let edge_2 = edge + 1f64;
                                {
                                    edge = edge_2;
                                    continue;
                                }
                            }
                        }
                        {
                            self::MlCheckOwnedOutcomeSourceTypesLoop8Result::MlCheckOwnedOutcomeSourceTypesLoop8Return(-6f64)
                        }
                    }
                }
            }
        } else {
            self::MlCheckOwnedOutcomeSourceTypesLoop8Result::MlCheckOwnedOutcomeSourceTypesLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomeSourceTypesJoin7 {
    MlCheckOwnedOutcomeSourceTypesJoin7Next(f64),
    MlCheckOwnedOutcomeSourceTypesJoin7Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_check_owned_outcome_source_types_loop6(
    types: &[f64],
    selectors: &[f64],
    offsets: &[f64],
    children: &[f64],
    modes: &[f64],
    patterns: &[f64],
    pattern_offsets: &[f64],
    pattern_types: &[f64],
    variant_owners: &[f64],
    mut count: f64,
    mut type_count: f64,
    mut pattern_count: f64,
    mut at: f64,
) -> self::MlCheckOwnedOutcomeSourceTypesLoop6Result {
    loop {
        return if at < count {
            {
                let type_ = (self::ml_array::optional_at(types, self::ml_array::number_index(at)))
                    .unwrap_or(-1f64);
                if (!(self::ml_math::is_safe_integer(type_)) || (type_ < 0f64))
                    || (type_ >= type_count)
                {
                    self::MlCheckOwnedOutcomeSourceTypesLoop6Result::MlCheckOwnedOutcomeSourceTypesLoop6Return(-3f64)
                } else {
                    {
                        let mode = (self::ml_array::optional_at(
                            modes,
                            self::ml_array::number_index(type_),
                        ))
                        .unwrap_or(-1f64);
                        if mode < 0f64 {
                            self::MlCheckOwnedOutcomeSourceTypesLoop6Result::MlCheckOwnedOutcomeSourceTypesLoop6Return(-3f64)
                        } else {
                            {
                                let selector = (self::ml_array::optional_at(
                                    selectors,
                                    self::ml_array::number_index(at),
                                ))
                                .unwrap_or(-2f64);
                                if (self::ml_math::is_safe_integer(selector)) {
                                    {
                                        let pattern = (self::ml_array::optional_at(
                                            patterns,
                                            self::ml_array::number_index(type_),
                                        ))
                                        .unwrap_or(-1f64);
                                        {
                                            let ml_s7 = if mode == 4f64 {
                                                if ((selector < 0f64)
                                                    || (selector >= pattern_count))
                                                    || !(match (
                                                        self::ml_array::optional_at(
                                                            variant_owners,
                                                            self::ml_array::number_index(selector),
                                                        ),
                                                        type_,
                                                    ) {
                                                        (Some(ml_left), ml_right) => {
                                                            ml_left == ml_right
                                                        }
                                                        _ => false,
                                                    })
                                                {
                                                    self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Return(-4f64)
                                                } else {
                                                    {
                                                        let pattern_2 = selector;
                                                        self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Next(pattern_2)
                                                    }
                                                }
                                            } else if mode == 1f64 {
                                                if (selector != 0f64) && (selector != 1f64) {
                                                    self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Return(-4f64)
                                                } else {
                                                    self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Next(pattern)
                                                }
                                            } else if mode == 5f64 {
                                                if (selector < 0f64) || (selector > 5f64) {
                                                    self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Return(-4f64)
                                                } else {
                                                    self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Next(pattern)
                                                }
                                            } else {
                                                let ml_library_inequality_1 = selector != -1f64;
                                                if ml_library_inequality_1 {
                                                    self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Return(-4f64)
                                                } else {
                                                    self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Next(pattern)
                                                }
                                            };
                                            match ml_s7 {
                                                self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Next(pattern_3) => {
                                                    {
                                                        let start = (self::ml_array::optional_at(offsets, self::ml_array::number_index(at))).unwrap_or(-1f64);
                                                        {
                                                            let end = (self::ml_array::optional_at(offsets, self::ml_array::number_index(at + 1f64))).unwrap_or(-1f64);
                                                            {
                                                                let first = (self::ml_array::optional_at(pattern_offsets, self::ml_array::number_index(pattern_3))).unwrap_or(-1f64);
                                                                {
                                                                    let size = (self::ml_array::optional_at(pattern_offsets, self::ml_array::number_index(pattern_3 + 1f64))).unwrap_or(-1f64) - first;
                                                                    if (((((mode == 0f64) || (mode == 4f64)) && ((end - start) != size)) || ((mode == 1f64) && ((end - start) != selector))) || ((mode == 3f64) && (((end - start) % 2f64) != 0f64))) || (((mode == 5f64) && (selector < 4f64)) && (end != start)) {
                                                                        self::MlCheckOwnedOutcomeSourceTypesLoop6Result::MlCheckOwnedOutcomeSourceTypesLoop6Return(-5f64)
                                                                    } else {
                                                                        {
                                                                            let edge = start;
                                                                            {
                                                                                let ml_s8 = self::ml_check_owned_outcome_source_types_loop8(types, children, pattern_types, count, type_, mode, start, end, first, edge);
                                                                                match ml_s8 {
                                                                                    self::MlCheckOwnedOutcomeSourceTypesLoop8Result::MlCheckOwnedOutcomeSourceTypesLoop8Done => {
                                                                                        {
                                                                                            let at_2 = at + 1f64;
                                                                                            {
                                                                                                at = at_2;
                                                                                                continue;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlCheckOwnedOutcomeSourceTypesLoop8Result::MlCheckOwnedOutcomeSourceTypesLoop8Return(ml_result8) => {
                                                                                        self::MlCheckOwnedOutcomeSourceTypesLoop6Result::MlCheckOwnedOutcomeSourceTypesLoop6Return(ml_result8)
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
                                                self::MlCheckOwnedOutcomeSourceTypesJoin7::MlCheckOwnedOutcomeSourceTypesJoin7Return(ml_result8_2) => {
                                                    self::MlCheckOwnedOutcomeSourceTypesLoop6Result::MlCheckOwnedOutcomeSourceTypesLoop6Return(ml_result8_2)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlCheckOwnedOutcomeSourceTypesLoop6Result::MlCheckOwnedOutcomeSourceTypesLoop6Return(-4f64)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlCheckOwnedOutcomeSourceTypesLoop6Result::MlCheckOwnedOutcomeSourceTypesLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomeSourceTypesLoop5Result {
    MlCheckOwnedOutcomeSourceTypesLoop5Done,
    MlCheckOwnedOutcomeSourceTypesLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_check_owned_outcome_source_types_loop5(
    offsets: &[f64],
    children: &[f64],
    mut at: f64,
) -> self::MlCheckOwnedOutcomeSourceTypesLoop5Result {
    loop {
        return if at < (offsets.len() as f64) {
            {
                let value =
                    (self::ml_array::optional_at(offsets, self::ml_array::number_index(at)))
                        .unwrap_or(-1f64);
                if ((!(self::ml_math::is_safe_integer(value)) || (value < 0f64))
                    || (value > (children.len() as f64)))
                    || ((at > 0f64)
                        && (value
                            < ((self::ml_array::optional_at(
                                offsets,
                                self::ml_array::number_index(at - 1f64),
                            ))
                            .unwrap_or(-1f64))))
                {
                    self::MlCheckOwnedOutcomeSourceTypesLoop5Result::MlCheckOwnedOutcomeSourceTypesLoop5Return(-2f64)
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
            self::MlCheckOwnedOutcomeSourceTypesLoop5Result::MlCheckOwnedOutcomeSourceTypesLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomeSourceTypesLoop4Result {
    MlCheckOwnedOutcomeSourceTypesLoop4Done,
    MlCheckOwnedOutcomeSourceTypesLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_check_owned_outcome_source_types_loop4(
    modes: &[f64],
    patterns: &[f64],
    pattern_offsets: &[f64],
    variant_owners: &[f64],
    mut type_count: f64,
    mut pattern_count: f64,
    mut at: f64,
) -> self::MlCheckOwnedOutcomeSourceTypesLoop4Result {
    loop {
        return if at < type_count {
            {
                let mode = (self::ml_array::optional_at(modes, self::ml_array::number_index(at)))
                    .unwrap_or(-2f64);
                {
                    let pattern =
                        (self::ml_array::optional_at(patterns, self::ml_array::number_index(at)))
                            .unwrap_or(-1f64);
                    if (((((!(self::ml_math::is_safe_integer(mode)) || (mode < -1f64))
                        || (mode > 5f64))
                        || !(self::ml_math::is_safe_integer(pattern)))
                        || (pattern < 0f64))
                        || (pattern >= pattern_count))
                        || !(match (
                            self::ml_array::optional_at(
                                variant_owners,
                                self::ml_array::number_index(pattern),
                            ),
                            -1f64,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    {
                        self::MlCheckOwnedOutcomeSourceTypesLoop4Result::MlCheckOwnedOutcomeSourceTypesLoop4Return(-1f64)
                    } else {
                        {
                            let size = (self::ml_array::optional_at(
                                pattern_offsets,
                                self::ml_array::number_index(pattern + 1f64),
                            ))
                            .unwrap_or(-1f64)
                                - (self::ml_array::optional_at(
                                    pattern_offsets,
                                    self::ml_array::number_index(pattern),
                                ))
                                .unwrap_or(-1f64);
                            if ((((mode == 1f64) || (mode == 2f64)) && (size != 1f64))
                                || ((mode == 3f64) && (size != 2f64)))
                                || ((((mode == -1f64) || (mode == 4f64)) || (mode == 5f64))
                                    && (size != 0f64))
                            {
                                self::MlCheckOwnedOutcomeSourceTypesLoop4Result::MlCheckOwnedOutcomeSourceTypesLoop4Return(-1f64)
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
                    }
                }
            }
        } else {
            self::MlCheckOwnedOutcomeSourceTypesLoop4Result::MlCheckOwnedOutcomeSourceTypesLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomeSourceTypesLoop3Result {
    MlCheckOwnedOutcomeSourceTypesLoop3Done,
    MlCheckOwnedOutcomeSourceTypesLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_check_owned_outcome_source_types_loop3(
    modes: &[f64],
    variant_owners: &[f64],
    mut type_count: f64,
    mut pattern_count: f64,
    mut at: f64,
) -> self::MlCheckOwnedOutcomeSourceTypesLoop3Result {
    loop {
        return if at < pattern_count {
            {
                let owner =
                    (self::ml_array::optional_at(variant_owners, self::ml_array::number_index(at)))
                        .unwrap_or(-2f64);
                if ((!(self::ml_math::is_safe_integer(owner)) || (owner < -1f64))
                    || (owner >= type_count))
                    || ((owner >= 0f64)
                        && !(match (
                            self::ml_array::optional_at(modes, self::ml_array::number_index(owner)),
                            4f64,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }))
                {
                    self::MlCheckOwnedOutcomeSourceTypesLoop3Result::MlCheckOwnedOutcomeSourceTypesLoop3Return(-1f64)
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
            self::MlCheckOwnedOutcomeSourceTypesLoop3Result::MlCheckOwnedOutcomeSourceTypesLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomeSourceTypesLoop2Result {
    MlCheckOwnedOutcomeSourceTypesLoop2Done,
    MlCheckOwnedOutcomeSourceTypesLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_check_owned_outcome_source_types_loop2(
    pattern_types: &[f64],
    mut type_count: f64,
    mut at: f64,
) -> self::MlCheckOwnedOutcomeSourceTypesLoop2Result {
    loop {
        return if at < (pattern_types.len() as f64) {
            {
                let value =
                    (self::ml_array::optional_at(pattern_types, self::ml_array::number_index(at)))
                        .unwrap_or(-1f64);
                if (!(self::ml_math::is_safe_integer(value)) || (value < 0f64))
                    || (value >= type_count)
                {
                    self::MlCheckOwnedOutcomeSourceTypesLoop2Result::MlCheckOwnedOutcomeSourceTypesLoop2Return(-1f64)
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
            self::MlCheckOwnedOutcomeSourceTypesLoop2Result::MlCheckOwnedOutcomeSourceTypesLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlCheckOwnedOutcomeSourceTypesLoop1Result {
    MlCheckOwnedOutcomeSourceTypesLoop1Done,
    MlCheckOwnedOutcomeSourceTypesLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_check_owned_outcome_source_types_loop1(
    pattern_offsets: &[f64],
    pattern_types: &[f64],
    mut at: f64,
) -> self::MlCheckOwnedOutcomeSourceTypesLoop1Result {
    loop {
        return if at < (pattern_offsets.len() as f64) {
            {
                let value = (self::ml_array::optional_at(
                    pattern_offsets,
                    self::ml_array::number_index(at),
                ))
                .unwrap_or(-1f64);
                if ((!(self::ml_math::is_safe_integer(value)) || (value < 0f64))
                    || (value > (pattern_types.len() as f64)))
                    || ((at > 0f64)
                        && (value
                            < ((self::ml_array::optional_at(
                                pattern_offsets,
                                self::ml_array::number_index(at - 1f64),
                            ))
                            .unwrap_or(-1f64))))
                {
                    self::MlCheckOwnedOutcomeSourceTypesLoop1Result::MlCheckOwnedOutcomeSourceTypesLoop1Return(-1f64)
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
            self::MlCheckOwnedOutcomeSourceTypesLoop1Result::MlCheckOwnedOutcomeSourceTypesLoop1Done
        };
    }
}
