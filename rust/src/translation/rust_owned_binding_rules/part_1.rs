// Generated from complete js/src/translation/rust-owned-binding-rules.js. Do not edit by hand.
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
pub const fn accept_rust_owned_binding_tracking(collecting: bool, excluded: bool) -> bool {
    !collecting && !excluded
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_binding_value(
    kind: &str,
    storage: &str,
    layers: f64,
    known: bool,
) -> bool {
    (((kind == "array") && (storage == "owned vector")) && (layers == 0f64)) && known
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_binding_read_error(state: &str) -> String {
    if state == "available" {
        String::new()
    } else if state == "moved" {
        String::from("a moved source binding cannot be read or consumed again")
    } else if state == "dropped" {
        String::from("an owned source binding does not outlive this use")
    } else if state == "possibly moved" {
        String::from("a source binding moved on a returning branch cannot be reused")
    } else {
        String::from("a source binding needs its actual ownership state")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn consume_rust_owned_binding_state(state: &str) -> String {
    if state == "available" {
        String::from("moved")
    } else {
        state.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn merge_rust_owned_binding_states(
    left: &str,
    right: &str,
    left_returns: bool,
    right_returns: bool,
) -> String {
    if ((left_returns) || (right_returns)) {
        if !left_returns || (left == "absent") {
            right.to_owned()
        } else if (!right_returns || (right == "absent")) || (left == right) {
            left.to_owned()
        } else {
            String::from("possibly moved")
        }
    } else {
        String::from("unreachable")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_owned_binding_guard(before: &[String], after: &[String]) -> bool {
    {
        let ml_library_inequality_1 = (before.len() as f64) != (after.len() as f64);
        if ml_library_inequality_1 {
            false
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_accept_rust_owned_binding_guard_loop1(
                            before, after, index,
                        );
                    match source_ml_source1_s1 {
                    self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Done => {
                        true
                    }
                    self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(source_ml_temporary_result1) => {
                        source_ml_temporary_result1
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn expire_rust_owned_binding_state(state: &str) -> String {
    if (state == "available") || (state == "possibly moved") {
        String::from("dropped")
    } else {
        state.to_owned()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_owned_result_transfer(
    binding: bool,
    available: bool,
    transferred: bool,
    fresh: bool,
) -> bool {
    if binding {
        available
    } else {
        transferred || fresh
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_owned_borrow_join_error(
    tracked: bool,
    borrowed: bool,
    named: bool,
    same: bool,
) -> String {
    if ((tracked && borrowed) && named) && !same {
        String::from("alternative borrowed owners need their complete loan union proof")
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustOwnedBindingGuardLoop1Result {
    MlTemporaryAcceptRustOwnedBindingGuardLoop1Done,
    MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_owned_binding_guard_loop1(
    before: &[String],
    after: &[String],
    mut index: f64,
) -> self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result {
    loop {
        return if index < (before.len() as f64) {
            if {
                let source_ml_source1_comparison_operand2 =
                    self::ml_array::optional_at(before, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand3 =
                        self::ml_array::optional_at(after, self::ml_array::number_index(index));
                    !(match (
                        source_ml_source1_comparison_operand2,
                        source_ml_source1_comparison_operand3,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Return(false)
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
            self::MlTemporaryAcceptRustOwnedBindingGuardLoop1Result::MlTemporaryAcceptRustOwnedBindingGuardLoop1Done
        };
    }
}
