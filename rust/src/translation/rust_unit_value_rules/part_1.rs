// Generated from complete js/src/translation/rust-unit-value-rules.js. Do not edit by hand.
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
pub fn read_rust_unit_statement_disposition(
    kind: &str,
    collecting: bool,
    proven: bool,
    type_: &str,
    indirected: bool,
) -> String {
    if kind == "abort" {
        String::from("abort")
    } else if ((proven && (type_ == "unit")) && !indirected) || (collecting && (kind == "app")) {
        String::from("bind")
    } else {
        String::from("refuse")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_function_macro_disposition(name: &str) -> String {
    if name == "println" {
        String::from("print")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_rust_terminal_return_boundary(values: &[String], start: f64) -> f64 {
    if (!(self::ml_math::is_safe_integer(start)) || (start < 0f64))
        || (start >= (values.len() as f64))
    {
        -1f64
    } else {
        {
            let stack = Vec::<String>::new();
            {
                let at = start + 1f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_find_rust_terminal_return_boundary_loop1(
                            values, stack, at,
                        );
                    match source_ml_source1_s1 {
                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop1Result::MlTemporaryFindRustTerminalReturnBoundaryLoop1Done => {
                            -2f64
                        }
                        self::MlTemporaryFindRustTerminalReturnBoundaryLoop1Result::MlTemporaryFindRustTerminalReturnBoundaryLoop1Return(source_ml_temporary_result1) => {
                            source_ml_temporary_result1
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindRustTerminalReturnBoundaryLoop1Result {
    MlTemporaryFindRustTerminalReturnBoundaryLoop1Done,
    MlTemporaryFindRustTerminalReturnBoundaryLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_rust_terminal_return_boundary_loop2(
    values: &[String],
    mut end: f64,
) -> f64 {
    loop {
        return if {
            let source_ml_source1_comparison_operand2 =
                self::ml_array::optional_at(values, self::ml_array::number_index(end));
            {
                let source_ml_source1_comparison_operand3 = String::from(";");
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
                let end_2 = end + 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_rust_terminal_return_boundary_loop1(
    values: &[String],
    mut stack: Vec<String>,
    mut at: f64,
) -> self::MlTemporaryFindRustTerminalReturnBoundaryLoop1Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| String::new());
                if ((value == "(") || (value == "[")) || (value == "{") {
                    {
                        let stack_2 = {
                            let (ml_append_right, ml_append_left) = (vec![value.clone()], stack);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (stack, at) = (stack_2, at_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    if ((stack.len() as f64) == 0f64) && ((value == ";") || (value == "}")) {
                        {
                            let end = at;
                            {
                                let end_2 = self::source_ml_temporary_find_rust_terminal_return_boundary_loop2(values, end);
                                if end_2 >= (values.len() as f64) {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop1Result::MlTemporaryFindRustTerminalReturnBoundaryLoop1Return(-2f64)
                                } else if {
                                    let source_ml_source1_comparison_operand4 =
                                        self::ml_array::optional_at(
                                            values,
                                            self::ml_array::number_index(end_2),
                                        );
                                    {
                                        let source_ml_source1_comparison_operand5 =
                                            String::from("}");
                                        match (
                                            source_ml_source1_comparison_operand4,
                                            source_ml_source1_comparison_operand5,
                                        ) {
                                            (Some(ml_left), ml_right) => ml_left == ml_right,
                                            _ => false,
                                        }
                                    }
                                } {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop1Result::MlTemporaryFindRustTerminalReturnBoundaryLoop1Return(end_2)
                                } else {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop1Result::MlTemporaryFindRustTerminalReturnBoundaryLoop1Return(-1f64)
                                }
                            }
                        }
                    } else if ((value == ")") || (value == "]")) || (value == "}") {
                        {
                            let previous = (self::ml_array::optional_at(
                                &stack,
                                self::ml_array::number_index((stack.len() as f64) - 1f64),
                            ))
                            .unwrap_or_else(|| String::new());
                            if (((value == ")") && (previous != "("))
                                || ((value == "]") && (previous != "[")))
                                || ((value == "}") && (previous != "{"))
                            {
                                self::MlTemporaryFindRustTerminalReturnBoundaryLoop1Result::MlTemporaryFindRustTerminalReturnBoundaryLoop1Return(-1f64)
                            } else {
                                {
                                    let stack_3 = self::ml_array::slice(&stack, 0f64, -1f64);
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (stack, at) = (stack_3, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let at_4 = at + 1f64;
                            {
                                at = at_4;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindRustTerminalReturnBoundaryLoop1Result::MlTemporaryFindRustTerminalReturnBoundaryLoop1Done
        };
    }
}
