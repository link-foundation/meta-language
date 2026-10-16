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
pub fn refuse_required_number_fact() -> f64 {
    panic!("{}", "missing required Number fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    panic!("{}", "missing required text fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    panic!("{}", "missing required Boolean fact column value")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Number fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required text fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    {
        assert!(
            !(((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64))),
            "{}",
            "invalid required Boolean fact column position"
        );

        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_owned_array_append_shape(
    statement: &str,
    expression: &str,
    operation: &str,
    receiver: &str,
    path_length: f64,
    arguments_count: f64,
) -> bool {
    (((((statement == "expr") && (expression == "method")) && (operation == "push"))
        && (receiver == "name"))
        && (path_length == 1f64))
        && (arguments_count > 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_owned_array_read(context: &str, operation: &str, reader: bool) -> bool {
    if (context == "index-receiver") || (context == "length-receiver") {
        true
    } else if context == "argument" {
        reader
    } else {
        (context == "method-receiver")
            && (((((operation == "slice") || (operation == "includes"))
                || (operation == "indexOf"))
                || (operation == "lastIndexOf"))
                || (operation == "join"))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_owned_array_reader_result(kind: &str) -> bool {
    ((((((kind == "float") || (kind == "fixed")) || (kind == "int")) || (kind == "nat"))
        || (kind == "bool"))
        || (kind == "string"))
        || (kind == "unit")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_owned_array_append_uses(
    allocation_loops: &[f64],
    append_loops: &[f64],
    escape_loops: &[f64],
    escape_starts: &[f64],
    escape_counts: &[f64],
    escape_positions: &[f64],
    position: f64,
) -> bool {
    {
        let escape = 0f64;
        {
            let ml_s1 = self::ml_accept_owned_array_append_uses_loop1(
                allocation_loops,
                append_loops,
                escape_loops,
                escape_starts,
                escape_counts,
                escape_positions,
                position,
                escape,
            );
            match ml_s1 {
                self::MlAcceptOwnedArrayAppendUsesLoop1Result::MlAcceptOwnedArrayAppendUsesLoop1Done => {
                    true
                }
                self::MlAcceptOwnedArrayAppendUsesLoop1Result::MlAcceptOwnedArrayAppendUsesLoop1Return(ml_result1) => {
                    ml_result1
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_owned_array_final_use(
    position: f64,
    bindings: &[f64],
    starts: &[f64],
    counts: &[f64],
    successors: &[f64],
) -> bool {
    {
        let binding = self::read_required_number_fact(bindings, position);
        {
            let pending = Vec::<f64>::new();
            {
                let seen = Vec::<f64>::new();
                {
                    let start = self::read_required_number_fact(starts, position);
                    {
                        let end = start + self::read_required_number_fact(counts, position);
                        {
                            let edge = start;
                            {
                                let pending_2 = self::ml_accept_owned_array_final_use_loop5(
                                    successors, pending, end, edge,
                                );
                                {
                                    let cursor = 0f64;
                                    {
                                        let ml_s2 = self::ml_accept_owned_array_final_use_loop6(
                                            bindings, starts, counts, successors, binding,
                                            pending_2, seen, cursor,
                                        );
                                        match ml_s2 {
                                            self::MlAcceptOwnedArrayFinalUseLoop6Result::MlAcceptOwnedArrayFinalUseLoop6Done => {
                                                true
                                            }
                                            self::MlAcceptOwnedArrayFinalUseLoop6Result::MlAcceptOwnedArrayFinalUseLoop6Return(ml_result6) => {
                                                ml_result6
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptOwnedArrayAppendUsesLoop1Result {
    MlAcceptOwnedArrayAppendUsesLoop1Done,
    MlAcceptOwnedArrayAppendUsesLoop1Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptOwnedArrayAppendUsesLoop2Result {
    MlAcceptOwnedArrayAppendUsesLoop2Done,
    MlAcceptOwnedArrayAppendUsesLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_owned_array_append_uses_loop3(
    append_loops: &[f64],
    mut loop_: f64,
    mut append_inside: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (append_loops.len() as f64) {
            if (self::read_required_number_fact(append_loops, index)) == loop_ {
                {
                    let append_inside_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (append_inside, index) = (append_inside_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { append_inside };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_owned_array_append_uses_loop4(
    allocation_loops: &[f64],
    mut loop_: f64,
    mut allocation_inside: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (allocation_loops.len() as f64) {
            if (self::read_required_number_fact(allocation_loops, index)) == loop_ {
                {
                    let allocation_inside_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (allocation_inside, index) = (allocation_inside_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { allocation_inside };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_owned_array_append_uses_loop2(
    allocation_loops: &[f64],
    append_loops: &[f64],
    escape_loops: &[f64],
    mut end: f64,
    mut edge: f64,
) -> self::MlAcceptOwnedArrayAppendUsesLoop2Result {
    loop {
        return if edge < end {
            {
                let loop_ = self::read_required_number_fact(escape_loops, edge);
                {
                    let append_inside = false;
                    {
                        let allocation_inside = false;
                        {
                            let index = 0f64;
                            {
                                let append_inside_2 = self::ml_accept_owned_array_append_uses_loop3(
                                    append_loops,
                                    loop_,
                                    append_inside,
                                    index,
                                );
                                {
                                    let index_2 = 0f64;
                                    {
                                        let allocation_inside_2 =
                                            self::ml_accept_owned_array_append_uses_loop4(
                                                allocation_loops,
                                                loop_,
                                                allocation_inside,
                                                index_2,
                                            );
                                        if append_inside_2 && !allocation_inside_2 {
                                            self::MlAcceptOwnedArrayAppendUsesLoop2Result::MlAcceptOwnedArrayAppendUsesLoop2Return(false)
                                        } else {
                                            {
                                                let edge_2 = edge + 1f64;
                                                {
                                                    edge = edge_2;
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
        } else {
            self::MlAcceptOwnedArrayAppendUsesLoop2Result::MlAcceptOwnedArrayAppendUsesLoop2Done
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
pub fn ml_accept_owned_array_append_uses_loop1(
    allocation_loops: &[f64],
    append_loops: &[f64],
    escape_loops: &[f64],
    escape_starts: &[f64],
    escape_counts: &[f64],
    escape_positions: &[f64],
    mut position: f64,
    mut escape: f64,
) -> self::MlAcceptOwnedArrayAppendUsesLoop1Result {
    loop {
        return if escape < (escape_positions.len() as f64) {
            if (self::read_required_number_fact(escape_positions, escape)) < position {
                self::MlAcceptOwnedArrayAppendUsesLoop1Result::MlAcceptOwnedArrayAppendUsesLoop1Return(false)
            } else {
                {
                    let start = self::read_required_number_fact(escape_starts, escape);
                    {
                        let end = start + self::read_required_number_fact(escape_counts, escape);
                        {
                            let edge = start;
                            {
                                let ml_s3 = self::ml_accept_owned_array_append_uses_loop2(
                                    allocation_loops,
                                    append_loops,
                                    escape_loops,
                                    end,
                                    edge,
                                );
                                match ml_s3 {
                                    self::MlAcceptOwnedArrayAppendUsesLoop2Result::MlAcceptOwnedArrayAppendUsesLoop2Done => {
                                        {
                                            let escape_2 = escape + 1f64;
                                            {
                                                escape = escape_2;
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlAcceptOwnedArrayAppendUsesLoop2Result::MlAcceptOwnedArrayAppendUsesLoop2Return(ml_result2) => {
                                        self::MlAcceptOwnedArrayAppendUsesLoop1Result::MlAcceptOwnedArrayAppendUsesLoop1Return(ml_result2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptOwnedArrayAppendUsesLoop1Result::MlAcceptOwnedArrayAppendUsesLoop1Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_owned_array_final_use_loop5(
    successors: &[f64],
    mut pending: Vec<f64>,
    mut end: f64,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        if edge < end {
            {
                let pending_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_required_number_fact(successors, edge)],
                        pending,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let edge_2 = edge + 1f64;
                    {
                        (pending, edge) = (pending_2, edge_2);
                        continue;
                    }
                }
            }
        }
        return { pending };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptOwnedArrayFinalUseLoop6Result {
    MlAcceptOwnedArrayFinalUseLoop6Done,
    MlAcceptOwnedArrayFinalUseLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_owned_array_final_use_loop7(
    successors: &[f64],
    mut pending: Vec<f64>,
    mut next_end: f64,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        if edge < next_end {
            {
                let pending_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_required_number_fact(successors, edge)],
                        pending,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let edge_2 = edge + 1f64;
                    {
                        (pending, edge) = (pending_2, edge_2);
                        continue;
                    }
                }
            }
        }
        return { pending };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_accept_owned_array_final_use_loop6(
    bindings: &[f64],
    starts: &[f64],
    counts: &[f64],
    successors: &[f64],
    mut binding: f64,
    mut pending: Vec<f64>,
    mut seen: Vec<f64>,
    mut cursor: f64,
) -> self::MlAcceptOwnedArrayFinalUseLoop6Result {
    loop {
        return if cursor < (pending.len() as f64) {
            {
                let event = self::read_required_number_fact(&(pending), cursor);
                {
                    let cursor_2 = cursor + 1f64;
                    if self::ml_array::includes(&seen, &event, 0f64, false, |a: &f64, b: &f64| {
                        a == b || (a.is_nan() && b.is_nan())
                    }) {
                        {
                            cursor = cursor_2;
                            continue;
                        }
                    }
                    {
                        {
                            let seen_2 = {
                                let (ml_append_right, ml_append_left) = (vec![event], seen);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            if (self::read_required_number_fact(bindings, event)) == binding {
                                self::MlAcceptOwnedArrayFinalUseLoop6Result::MlAcceptOwnedArrayFinalUseLoop6Return(false)
                            } else {
                                {
                                    let next_start = self::read_required_number_fact(starts, event);
                                    {
                                        let next_end = next_start
                                            + self::read_required_number_fact(counts, event);
                                        {
                                            let edge = next_start;
                                            {
                                                let pending_2 =
                                                    self::ml_accept_owned_array_final_use_loop7(
                                                        successors, pending, next_end, edge,
                                                    );
                                                {
                                                    (pending, seen, cursor) =
                                                        (pending_2, seen_2, cursor_2);
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
        } else {
            self::MlAcceptOwnedArrayFinalUseLoop6Result::MlAcceptOwnedArrayFinalUseLoop6Done
        };
    }
}
