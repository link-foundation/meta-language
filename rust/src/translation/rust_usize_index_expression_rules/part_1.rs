// Generated from complete js/src/translation/rust-usize-index-expression-rules.js. Do not edit by hand.
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

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Number fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> String {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required text fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (String::from("missing required Boolean fact column value"))
            .encode_utf16()
            .collect(),
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Number fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required text fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (String::from("invalid required Boolean fact column position"))
                .encode_utf16()
                .collect(),
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_usize_index_expression_node(
    kind: &str,
    operator: &str,
    atom: bool,
    literal: bool,
    children: f64,
) -> bool {
    if kind == "binary" {
        (children == 2f64)
            && (((((operator == "add") || (operator == "sub")) || (operator == "mul"))
                || (operator == "div"))
                || (operator == "rem"))
    } else if kind == "if" {
        children == 2f64
    } else if kind == "match" {
        children > 0f64
    } else {
        (children == 0f64) && ((atom || literal) || (kind == "abort"))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_usize_index_expression_error(
    kinds: &[String],
    operators: &[String],
    atoms: &[bool],
    literals: &[bool],
    parents: &[f64],
) -> String {
    {
        let size = kinds.len() as f64;
        if (size < 1f64) || (size > 4096f64) {
            String::from("the index expression exceeds its source node limit")
        } else if ((((operators.len() as f64) != size) || ((atoms.len() as f64) != size))
            || ((literals.len() as f64) != size))
            || ((parents.len() as f64) != size)
        {
            String::from("unaligned index expression source columns")
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s1 =
                        self::source_ml_temporary_read_rust_usize_index_expression_error_loop1(
                            parents, size, index,
                        );
                    match source_ml_source1_s1 {
                            self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Done => {
                                {
                                    let accepted = Vec::<bool>::new();
                                    {
                                        let remaining = size;
                                        {
                                            let accepted_2 = self::source_ml_temporary_read_rust_usize_index_expression_error_loop2(kinds, operators, atoms, literals, parents, size, accepted, remaining);
                                            if self::read_required_boolean_fact(&(accepted_2), (size - 1f64)) {
                                                String::new()
                                            } else {
                                                String::from("the complete index expression needs actual usize atoms or contextually inferred integer operands")
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Return(source_ml_temporary_result3) => {
                                source_ml_temporary_result3
                            }
                        }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustUsizeIndexExpressionErrorLoop3Result {
    MlTemporaryReadRustUsizeIndexExpressionErrorLoop3Done(f64, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_usize_index_expression_error_loop3(
    parents: &[f64],
    mut size: f64,
    accepted: &[bool],
    mut index: f64,
    mut count: f64,
    mut children_accepted: bool,
    mut child: f64,
) -> self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop3Result {
    loop {
        if child < size {
            if (self::read_required_number_fact(parents, child)) == index {
                {
                    let count_2 = count + 1f64;
                    {
                        let children_accepted_2 = children_accepted
                            && self::read_required_boolean_fact(accepted, ((size - child) - 1f64));
                        {
                            let child_2 = child + 1f64;
                            {
                                (count, children_accepted, child) =
                                    (count_2, children_accepted_2, child_2);
                                continue;
                            }
                        }
                    }
                }
            }
            {
                {
                    let child_3 = child + 1f64;
                    {
                        child = child_3;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop3Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop3Done(count, children_accepted)
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
pub fn source_ml_temporary_read_rust_usize_index_expression_error_loop2(
    kinds: &[String],
    operators: &[String],
    atoms: &[bool],
    literals: &[bool],
    parents: &[f64],
    mut size: f64,
    mut accepted: Vec<bool>,
    mut remaining: f64,
) -> Vec<bool> {
    loop {
        return if remaining > 0f64 {
            {
                let index = remaining - 1f64;
                {
                    let count = 0f64;
                    {
                        let children_accepted = true;
                        {
                            let child = index + 1f64;
                            {
                                let source_ml_source1_s2 = self::source_ml_temporary_read_rust_usize_index_expression_error_loop3(parents, size, &(accepted), index, count, children_accepted, child);
                                match source_ml_source1_s2 {
                                    self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop3Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop3Done(count_2, children_accepted_2) => {
                                        {
                                            let accepted_2 = { let (ml_append_right, ml_append_left) = (vec![(children_accepted_2 && self::accept_rust_usize_index_expression_node(&(self::read_required_text_fact(kinds, index)), &(self::read_required_text_fact(operators, index)), self::read_required_boolean_fact(atoms, index), self::read_required_boolean_fact(literals, index), count_2))], accepted); self::ml_array::append(ml_append_left, ml_append_right) };
                                            {
                                                let remaining_2 = remaining - 1f64;
                                                {
                                                    (accepted, remaining) = (accepted_2, remaining_2);
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
            accepted
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Result {
    MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Done,
    MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Result {
    MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Done,
    MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_usize_index_expression_error_loop4(
    parents: &[f64],
    mut ancestor: f64,
    mut depth: f64,
) -> self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Result {
    loop {
        return if ancestor >= 0f64 {
            {
                let depth_2 = depth + 1f64;
                if depth_2 > 128f64 {
                    self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Return(String::from("the index expression exceeds its source depth limit"))
                } else {
                    {
                        let ancestor_2 = self::read_required_number_fact(parents, ancestor);
                        {
                            (ancestor, depth) = (ancestor_2, depth_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_usize_index_expression_error_loop1(
    parents: &[f64],
    mut size: f64,
    mut index: f64,
) -> self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Result {
    loop {
        return if index < size {
            {
                let parent = self::read_required_number_fact(parents, index);
                if (!(self::ml_math::is_integer(parent)) || ((index == 0f64) && (parent != -1f64)))
                    || ((index > 0f64) && ((parent < 0f64) || (parent >= index)))
                {
                    self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Return(String::from("an index expression needs one root and actual earlier parents"))
                } else {
                    {
                        let ancestor = parent;
                        {
                            let depth = 0f64;
                            {
                                let source_ml_source1_s3 = self::source_ml_temporary_read_rust_usize_index_expression_error_loop4(parents, ancestor, depth);
                                match source_ml_source1_s3 {
                                    self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Done => {
                                        {
                                            let index_2 = index + 1f64;
                                            {
                                                index = index_2;
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop4Return(source_ml_temporary_result4) => {
                                        self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Return(source_ml_temporary_result4)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Result::MlTemporaryReadRustUsizeIndexExpressionErrorLoop1Done
        };
    }
}
