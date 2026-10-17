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
pub fn accept_grammar_selection_field_range_mapping(mapping: &str) -> bool {
    mapping == "field-ranges"
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_field_range_candidate(kind: &str, mapping: &str) -> bool {
    (kind == "grammar-field-candidate")
        && self::accept_grammar_selection_field_range_mapping(mapping)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_field_range_selector(
    kind: &str,
    label: &str,
    mode: &str,
    first: f64,
    count: f64,
) -> bool {
    (((((((((kind == "source-field-selector") && ("string" == "string"))
        && (!(label).is_empty()))
        && ((mode == "include") || (mode == "exclude")))
        && self::ml_math::is_integer(first))
        && (first >= 0f64))
        && (first <= 4_294_967_295f64))
        && self::ml_math::is_integer(count))
        && (count > 0f64))
        && (count <= 4_294_967_295f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_field_range_context(mappings: &[String], in_token: bool) -> f64 {
    {
        let found = false;
        {
            let index = 0f64;
            {
                let found_2 = self::ml_read_grammar_selection_field_range_context_loop1(
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
                                    self::ml_read_grammar_selection_field_range_context_loop2(
                                        mappings, index_2,
                                    );
                                match ml_s1 {
                                    self::MlReadGrammarSelectionFieldRangeContextLoop2Result::MlReadGrammarSelectionFieldRangeContextLoop2Done => {
                                        1f64
                                    }
                                    self::MlReadGrammarSelectionFieldRangeContextLoop2Result::MlReadGrammarSelectionFieldRangeContextLoop2Return(ml_result2) => {
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
pub fn read_grammar_selection_field_range_field_order(role: &str) -> String {
    if role == "candidate" {
        String::from("kind|name|mapping|selector")
    } else if role == "selector" {
        String::from("kind|label|mode|first|count")
    } else if role == "occurrence" {
        String::from("path|start|end")
    } else if role == "derivation" {
        String::from("operation|input|start|end|selector|occurrences|ranges|witness")
    } else if role == "mapping" {
        String::from("kind|operation|input|output|selector|occurrences|ranges|intervals")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_field_range_match(
    actual_field: &str,
    label: &str,
    trivia: bool,
) -> bool {
    (((!trivia && ("string" == "string")) && ("string" == "string")) && (!(label).is_empty()))
        && (actual_field == label)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_field_range_plan(
    spans: &[f64],
    start: f64,
    end: f64,
    first: f64,
    count: f64,
    mode: &str,
) -> Vec<f64> {
    if (((((((((((((!(self::ml_math::is_integer(start))
        || !(self::ml_math::is_integer(end)))
        || (start < 0f64))
        || (start >= end))
        || (end > 4_294_967_295f64))
        || !(self::ml_math::is_integer(first)))
        || (first < 0f64))
        || (first > 4_294_967_295f64))
        || !(self::ml_math::is_integer(count)))
        || (count <= 0f64))
        || (count > 4_294_967_295f64))
        || ((mode != "include") && (mode != "exclude")))
        || ((spans.len() as f64) == 0f64))
        || (((spans.len() as f64) % 2f64) != 0f64))
        || ((spans.len() as f64) > (2f64 * (end - start)))
    {
        Vec::<f64>::new()
    } else {
        {
            let available = (spans.len() as f64) / 2f64;
            if (first >= available) || (count > (available - first)) {
                Vec::<f64>::new()
            } else {
                {
                    let following = first + count;
                    {
                        let ranges = Vec::<f64>::new();
                        {
                            let previous = start;
                            {
                                let cursor = start;
                                {
                                    let width = 0f64;
                                    {
                                        let index = 0f64;
                                        {
                                            let ml_s2 = self::ml_read_grammar_selection_field_range_plan_loop3(spans, start, end, first, mode, following, ranges, previous, cursor, width, index);
                                            match ml_s2 {
                                                self::MlReadGrammarSelectionFieldRangePlanLoop3Result::MlReadGrammarSelectionFieldRangePlanLoop3Done(ranges_2, cursor_2, width_2) => {
                                                    {
                                                        let ml_s3 = if (mode == "exclude") && (cursor_2 < end) {
                                                            {
                                                                let ranges_3 = { let (ml_append_right, ml_append_left) = (vec![(cursor_2 - start), (end - start)], ranges_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let width_3 = width_2 + (end - cursor_2);
                                                                    self::MlReadGrammarSelectionFieldRangePlanJoin4::MlReadGrammarSelectionFieldRangePlanJoin4Next(ranges_3, width_3)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlReadGrammarSelectionFieldRangePlanJoin4::MlReadGrammarSelectionFieldRangePlanJoin4Next(ranges_2, width_2)
                                                        };
                                                        match ml_s3 {
                                                            self::MlReadGrammarSelectionFieldRangePlanJoin4::MlReadGrammarSelectionFieldRangePlanJoin4Next(ranges_4, width_4) => {
                                                                if (width_4 <= 0f64) || (width_4 >= (end - start)) {
                                                                    Vec::<f64>::new()
                                                                } else {
                                                                    ranges_4
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlReadGrammarSelectionFieldRangePlanLoop3Result::MlReadGrammarSelectionFieldRangePlanLoop3Return(ml_result4) => {
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
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_field_range_children(type_: &str, source_children: bool) -> String {
    if type_ == "node" {
        String::from("children")
    } else if type_ == "embed" {
        if source_children {
            String::from("sourceChildren")
        } else {
            String::from("none")
        }
    } else if ((type_ == "token") || (type_ == "error")) || (type_ == "missing") {
        String::from("none")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_field_range_context_loop1(
    mappings: &[String],
    mut found: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (mappings.len() as f64) {
            if self::accept_grammar_selection_field_range_mapping(
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
pub enum MlReadGrammarSelectionFieldRangeContextLoop2Result {
    MlReadGrammarSelectionFieldRangeContextLoop2Done,
    MlReadGrammarSelectionFieldRangeContextLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_field_range_context_loop2(
    mappings: &[String],
    mut index: f64,
) -> self::MlReadGrammarSelectionFieldRangeContextLoop2Result {
    loop {
        return if index < (mappings.len() as f64) {
            if (self::accept_grammar_selection_field_range_mapping(
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
                self::MlReadGrammarSelectionFieldRangeContextLoop2Result::MlReadGrammarSelectionFieldRangeContextLoop2Return(2f64)
            }
        } else {
            self::MlReadGrammarSelectionFieldRangeContextLoop2Result::MlReadGrammarSelectionFieldRangeContextLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarSelectionFieldRangePlanJoin4 {
    MlReadGrammarSelectionFieldRangePlanJoin4Next(Vec<f64>, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarSelectionFieldRangePlanLoop3Result {
    MlReadGrammarSelectionFieldRangePlanLoop3Done(Vec<f64>, f64, f64),
    MlReadGrammarSelectionFieldRangePlanLoop3Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarSelectionFieldRangePlanJoin5 {
    MlReadGrammarSelectionFieldRangePlanJoin5Next(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_selection_field_range_plan_loop3(
    spans: &[f64],
    mut start: f64,
    mut end: f64,
    mut first: f64,
    mode: &str,
    mut following: f64,
    mut ranges: Vec<f64>,
    mut previous: f64,
    mut cursor: f64,
    mut width: f64,
    mut index: f64,
) -> self::MlReadGrammarSelectionFieldRangePlanLoop3Result {
    loop {
        return if index < (spans.len() as f64) {
            {
                let left =
                    (self::ml_array::optional_at(spans, self::ml_array::number_index(index)))
                        .unwrap_or(-1f64);
                {
                    let right = (self::ml_array::optional_at(
                        spans,
                        self::ml_array::number_index(index + 1f64),
                    ))
                    .unwrap_or(-1f64);
                    if ((((!(self::ml_math::is_integer(left))
                        || !(self::ml_math::is_integer(right)))
                        || (left < previous))
                        || (left < start))
                        || (left >= right))
                        || (right > end)
                    {
                        self::MlReadGrammarSelectionFieldRangePlanLoop3Result::MlReadGrammarSelectionFieldRangePlanLoop3Return(Vec::<f64>::new())
                    } else {
                        {
                            let previous_2 = right;
                            {
                                let occurrence = index / 2f64;
                                if (occurrence >= first) && (occurrence < following) {
                                    if mode == "include" {
                                        {
                                            let ranges_2 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![(left - start), (right - start)], ranges);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let width_2 = width + (right - left);
                                                {
                                                    let index_2 = index + 2f64;
                                                    {
                                                        (ranges, previous, width, index) = (
                                                            ranges_2, previous_2, width_2, index_2,
                                                        );
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    {
                                        {
                                            let ml_s4 = if cursor < left {
                                                {
                                                    let ranges_3 = {
                                                        let (ml_append_right, ml_append_left) = (
                                                            vec![(cursor - start), (left - start)],
                                                            ranges,
                                                        );
                                                        self::ml_array::append(
                                                            ml_append_left,
                                                            ml_append_right,
                                                        )
                                                    };
                                                    {
                                                        let width_3 = width + (left - cursor);
                                                        self::MlReadGrammarSelectionFieldRangePlanJoin5::MlReadGrammarSelectionFieldRangePlanJoin5Next(ranges_3, width_3)
                                                    }
                                                }
                                            } else {
                                                self::MlReadGrammarSelectionFieldRangePlanJoin5::MlReadGrammarSelectionFieldRangePlanJoin5Next(ranges, width)
                                            };
                                            match ml_s4 {
                                                self::MlReadGrammarSelectionFieldRangePlanJoin5::MlReadGrammarSelectionFieldRangePlanJoin5Next(ranges_4, width_4) => {
                                                    {
                                                        let cursor_2 = right;
                                                        {
                                                            let index_3 = index + 2f64;
                                                            {
                                                                (ranges, previous, cursor, width, index) = (ranges_4, previous_2, cursor_2, width_4, index_3);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let index_4 = index + 2f64;
                                        {
                                            (previous, index) = (previous_2, index_4);
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
            self::MlReadGrammarSelectionFieldRangePlanLoop3Result::MlReadGrammarSelectionFieldRangePlanLoop3Done(ranges, cursor, width)
        };
    }
}
