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
pub fn accept_grammar_selection_included_range_mapping(mapping: &str) -> bool {
    mapping == "included-ranges"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_included_range_context(mappings: &[String], in_token: bool) -> f64 {
    {
        let found = false;
        {
            let index = 0f64;
            {
                let found_2 = self::ml_read_grammar_selection_included_range_context_loop1(
                    mappings, found, index,
                );
                if found_2 {
                    if in_token {
                        2f64
                    } else {
                        {
                            let index_2 = 0f64;
                            {
                                let ml_s1 =
                                    self::ml_read_grammar_selection_included_range_context_loop2(
                                        mappings, index_2,
                                    );
                                match ml_s1 {
                                    self::MlReadGrammarSelectionIncludedRangeContextLoop2Result::MlReadGrammarSelectionIncludedRangeContextLoop2Done => {
                                        1f64
                                    }
                                    self::MlReadGrammarSelectionIncludedRangeContextLoop2Result::MlReadGrammarSelectionIncludedRangeContextLoop2Return(ml_result2) => {
                                        ml_result2
                                    }
                                }
                            }
                        }
                    }
                } else {
                    0f64
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_included_range_candidate(kind: &str, mapping: &str) -> bool {
    (kind == "grammar-range-candidate")
        && self::accept_grammar_selection_included_range_mapping(mapping)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_included_range_source_range(
    kind: &str,
    start: f64,
    end: f64,
) -> bool {
    (((((kind == "source-range") && self::ml_math::is_integer(start))
        && self::ml_math::is_integer(end))
        && (start >= 0f64))
        && (start < end))
        && (end <= 4_294_967_295f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_included_range_agreement(first: &[f64], next: &[f64]) -> bool {
    {
        let ml_library_inequality_1 = (first.len() as f64) != (next.len() as f64);
        if ml_library_inequality_1 {
            false
        } else {
            {
                let index = 0f64;
                {
                    let ml_s2 = self::ml_accept_grammar_selection_included_range_agreement_loop3(
                        first, next, index,
                    );
                    match ml_s2 {
                    self::MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Result::MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Done => {
                        true
                    }
                    self::MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Result::MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Return(ml_result3) => {
                        ml_result3
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_included_range_reservation(
    bytes: f64,
    cells: f64,
    steps: bool,
) -> f64 {
    if (((((!(self::ml_math::is_integer(bytes)) || (bytes <= 0f64))
        || (bytes > 4_294_967_295f64))
        || !(self::ml_math::is_integer(cells)))
        || (cells <= 0f64))
        || ((cells % 2f64) != 0f64))
        || (cells > (2f64 * bytes))
    {
        -1f64
    } else {
        (if steps { 64f64 } else { 32f64 } * bytes) + 64f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_included_range_plan(ranges: &[f64], length: f64) -> Vec<f64> {
    if (self::read_grammar_selection_included_range_reservation(
        length,
        (ranges.len() as f64),
        false,
    )) < 0f64
    {
        Vec::<f64>::new()
    } else {
        {
            let plan = vec![0f64, 0f64, 0f64, 0f64];
            {
                let cursor = 0f64;
                {
                    let output = 0f64;
                    {
                        let index = 0f64;
                        {
                            let ml_s3 = self::ml_read_grammar_selection_included_range_plan_loop4(
                                ranges, length, plan, cursor, output, index,
                            );
                            match ml_s3 {
                                self::MlReadGrammarSelectionIncludedRangePlanLoop4Result::MlReadGrammarSelectionIncludedRangePlanLoop4Done(plan_2, cursor_2, output_2) => {
                                    if output_2 >= length {
                                        Vec::<f64>::new()
                                    } else {
                                        {
                                            let plan_4 = if cursor_2 < length {
                                                {
                                                    { let (ml_append_right, ml_append_left) = (vec![cursor_2, length, output_2, output_2], plan_2); self::ml_array::append(ml_append_left, ml_append_right) }
                                                }
                                            } else {
                                                plan_2
                                            };
                                            {
                                                { let (ml_append_right, ml_append_left) = (vec![length, length, output_2, output_2], plan_4); self::ml_array::append(ml_append_left, ml_append_right) }
                                            }
                                        }
                                    }
                                }
                                self::MlReadGrammarSelectionIncludedRangePlanLoop4Result::MlReadGrammarSelectionIncludedRangePlanLoop4Return(ml_result4) => {
                                    ml_result4
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
pub fn read_grammar_selection_included_range_part(
    plan: &[f64],
    row: f64,
    position: f64,
    output: f64,
    width: f64,
) -> Vec<f64> {
    if ((((((plan.len() as f64) < 12f64) || (((plan.len() as f64) % 4f64) != 0f64))
        || !(self::ml_math::is_integer(row)))
        || (row < 0f64))
        || ((row % 4f64) != 0f64))
        || (row > (plan.len() as f64))
    {
        Vec::<f64>::new()
    } else if row == (plan.len() as f64) {
        vec![1f64]
    } else {
        {
            let start = (self::ml_array::optional_at(plan, self::ml_array::number_index(row)))
                .unwrap_or_else(|| -1f64);
            {
                let end =
                    (self::ml_array::optional_at(plan, self::ml_array::number_index(row + 1f64)))
                        .unwrap_or_else(|| -1f64);
                {
                    let output_start = (self::ml_array::optional_at(
                        plan,
                        self::ml_array::number_index(row + 2f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    {
                        let output_end = (self::ml_array::optional_at(
                            plan,
                            self::ml_array::number_index(row + 3f64),
                        ))
                        .unwrap_or_else(|| -1f64);
                        {
                            let following = row + 4f64;
                            {
                                let following_position = (self::ml_array::optional_at(
                                    plan,
                                    self::ml_array::number_index(following),
                                ))
                                .unwrap_or_else(|| end);
                                if output_start == output_end {
                                    if (position != start) || (output != output_start) {
                                        Vec::<f64>::new()
                                    } else {
                                        vec![
                                            0f64,
                                            following,
                                            following_position,
                                            output,
                                            start,
                                            end,
                                            output,
                                            output,
                                        ]
                                    }
                                } else if ((((((!(self::ml_math::is_integer(position))
                                    || (position < start))
                                    || (position >= end))
                                    || (output != ((output_start + position) - start)))
                                    || !(self::ml_math::is_integer(width)))
                                    || (width < 1f64))
                                    || (width > 4f64))
                                    || ((position + width) > end)
                                {
                                    Vec::<f64>::new()
                                } else {
                                    {
                                        let next = position + width;
                                        {
                                            let next_output = output + width;
                                            vec![
                                                0f64,
                                                if next == end { following } else { row },
                                                if next == end {
                                                    following_position
                                                } else {
                                                    next
                                                },
                                                next_output,
                                                position,
                                                next,
                                                output,
                                                next_output,
                                            ]
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
pub fn accept_grammar_selection_included_range_byte_boundary(byte: f64, at_end: bool) -> bool {
    at_end
        || (((self::ml_math::is_integer(byte) && (byte >= 0f64)) && (byte <= 255f64))
            && ((byte < 128f64) || (byte >= 192f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_included_range_field_order(role: &str) -> String {
    if role == "derivation" {
        String::from("operation|input|start|end|ranges")
    } else if role == "mapping" {
        String::from("kind|operation|input|output|ranges|intervals")
    } else if role == "candidate" {
        String::from("kind|name|mapping|ranges")
    } else if role == "range" {
        String::from("kind|start|end")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_included_range_derivation(
    operation: &str,
    input: f64,
    output: f64,
    start: f64,
    end: f64,
    input_length: f64,
) -> bool {
    (((((((self::accept_grammar_selection_included_range_mapping(operation)
        && self::ml_math::is_integer(input))
        && (input >= 0f64))
        && (input < output))
        && self::ml_math::is_integer(start))
        && self::ml_math::is_integer(end))
        && (start >= 0f64))
        && (start < end))
        && (end <= input_length)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_included_range_profile(
    mapping: &str,
    physical_children: bool,
    source_mapping: bool,
) -> bool {
    (self::accept_grammar_selection_included_range_mapping(mapping) && physical_children)
        && source_mapping
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_included_range_descriptor(kind: &str, operation: &str) -> bool {
    (kind == "source-mapping") && self::accept_grammar_selection_included_range_mapping(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_included_range_context_loop1(
    mappings: &[String],
    mut found: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (mappings.len() as f64) {
            if self::accept_grammar_selection_included_range_mapping(
                &((self::ml_array::optional_at(mappings, self::ml_array::number_index(index)))
                    .unwrap_or_else(|| String::from("identity"))),
            ) {
                {
                    let found_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (found, index) = (found_2, index_2);
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
        return { found };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarSelectionIncludedRangeContextLoop2Result {
    MlReadGrammarSelectionIncludedRangeContextLoop2Done,
    MlReadGrammarSelectionIncludedRangeContextLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_included_range_context_loop2(
    mappings: &[String],
    mut index: f64,
) -> self::MlReadGrammarSelectionIncludedRangeContextLoop2Result {
    loop {
        return if index < (mappings.len() as f64) {
            if (self::accept_grammar_selection_included_range_mapping(
                &((self::ml_array::optional_at(mappings, self::ml_array::number_index(index)))
                    .unwrap_or_else(|| String::from("identity"))),
            )) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlReadGrammarSelectionIncludedRangeContextLoop2Result::MlReadGrammarSelectionIncludedRangeContextLoop2Return(2f64)
            }
        } else {
            self::MlReadGrammarSelectionIncludedRangeContextLoop2Result::MlReadGrammarSelectionIncludedRangeContextLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Result {
    MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Done,
    MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_grammar_selection_included_range_agreement_loop3(
    first: &[f64],
    next: &[f64],
    mut index: f64,
) -> self::MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Result {
    loop {
        return if index < (first.len() as f64) {
            if (match (
                self::ml_array::optional_at(first, self::ml_array::number_index(index)),
                self::ml_array::optional_at(next, self::ml_array::number_index(index)),
            ) {
                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                (None, None) => true,
                _ => false,
            }) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Result::MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Return(false)
            }
        } else {
            self::MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Result::MlAcceptGrammarSelectionIncludedRangeAgreementLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarSelectionIncludedRangePlanLoop4Result {
    MlReadGrammarSelectionIncludedRangePlanLoop4Done(Vec<f64>, f64, f64),
    MlReadGrammarSelectionIncludedRangePlanLoop4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_included_range_plan_loop4(
    ranges: &[f64],
    mut length: f64,
    mut plan: Vec<f64>,
    mut cursor: f64,
    mut output: f64,
    mut index: f64,
) -> self::MlReadGrammarSelectionIncludedRangePlanLoop4Result {
    loop {
        return if index < (ranges.len() as f64) {
            {
                let start =
                    (self::ml_array::optional_at(ranges, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                {
                    let end = (self::ml_array::optional_at(
                        ranges,
                        self::ml_array::number_index(index + 1f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    if (!(self::accept_grammar_selection_included_range_source_range(
                        "source-range",
                        start,
                        end,
                    )) || (start < cursor))
                        || (end > length)
                    {
                        self::MlReadGrammarSelectionIncludedRangePlanLoop4Result::MlReadGrammarSelectionIncludedRangePlanLoop4Return(Vec::<f64>::new())
                    } else {
                        {
                            let plan_3 = if cursor < start {
                                {
                                    {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![cursor, start, output, output], plan);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    }
                                }
                            } else {
                                plan
                            };
                            {
                                let plan_4 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![start, end, output, ((output + end) - start)],
                                        plan_3,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let output_2 = output + (end - start);
                                    {
                                        let cursor_2 = end;
                                        {
                                            let index_2 = index + 2f64;
                                            {
                                                (plan, cursor, output, index) =
                                                    (plan_4, cursor_2, output_2, index_2);
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
        } else {
            self::MlReadGrammarSelectionIncludedRangePlanLoop4Result::MlReadGrammarSelectionIncludedRangePlanLoop4Done(plan, cursor, output)
        };
    }
}
