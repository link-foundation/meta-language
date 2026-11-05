// Generated from complete js/src/translation/source-parameter-binding-rules.js. Do not edit by hand.
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
pub fn accept_java_script_source_parameter_identifier_term(term: &str) -> bool {
    (term == "identifier") || (term == "shorthand_property_identifier_pattern")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_source_binding_root_term(term: &str) -> bool {
    ((term == "identifier") || (term == "object_pattern")) || (term == "array_pattern")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn contains_source_parameter_interval(firsts: &[f64], lasts: &[f64], at: f64) -> bool {
    {
        let slot = 0f64;
        {
            let source_ml_source1_s1 =
                self::source_ml_temporary_contains_source_parameter_interval_loop1(
                    firsts, lasts, at, slot,
                );
            match source_ml_source1_s1 {
                self::MlTemporaryContainsSourceParameterIntervalLoop1Result::MlTemporaryContainsSourceParameterIntervalLoop1Done => {
                    false
                }
                self::MlTemporaryContainsSourceParameterIntervalLoop1Result::MlTemporaryContainsSourceParameterIntervalLoop1Return(source_ml_temporary_result1) => {
                    source_ml_temporary_result1
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_source_parameter_default_separator(
    kinds: &[String],
    values: &[String],
    firsts: &[f64],
    lasts: &[f64],
    first: f64,
    last: f64,
) -> f64 {
    {
        let closes = Vec::<String>::new();
        {
            let at = first;
            {
                let source_ml_source1_s2 = self::source_ml_temporary_find_java_script_source_parameter_default_separator_loop2(kinds, values, firsts, lasts, last, closes, at);
                match source_ml_source1_s2 {
                    self::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Result::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Result::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Return(source_ml_temporary_result2) => {
                        source_ml_temporary_result2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_source_parameter_intervals(
    firsts: &[f64],
    lasts: &[f64],
    length: f64,
) -> bool {
    {
        let ml_library_inequality_1 = (firsts.len() as f64) != (lasts.len() as f64);
        if ml_library_inequality_1 {
            false
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source1_s3 = self::source_ml_temporary_accept_java_script_source_parameter_intervals_loop3(firsts, lasts, length, at);
                    match source_ml_source1_s3 {
                    self::MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Result::MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Done => {
                        true
                    }
                    self::MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Result::MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Return(source_ml_temporary_result3) => {
                        source_ml_temporary_result3
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_source_parameter_bindings(
    kinds: &[String],
    values: &[String],
    candidates: &[bool],
    default_firsts: &[f64],
    default_lasts: &[f64],
    computed_firsts: &[f64],
    computed_lasts: &[f64],
) -> Vec<f64> {
    {
        let length = kinds.len() as f64;
        if ((((values.len() as f64) != length) || ((candidates.len() as f64) != length))
            || !(self::accept_java_script_source_parameter_intervals(
                default_firsts,
                default_lasts,
                length,
            )))
            || !(self::accept_java_script_source_parameter_intervals(
                computed_firsts,
                computed_lasts,
                length,
            ))
        {
            vec![-1f64]
        } else {
            {
                let firsts = computed_firsts.to_vec();
                {
                    let lasts = computed_lasts.to_vec();
                    {
                        let previous = -1f64;
                        {
                            let slot = 0f64;
                            {
                                let source_ml_source1_s4 = self::source_ml_temporary_plan_java_script_source_parameter_bindings_loop4(kinds, values, default_firsts, default_lasts, firsts, lasts, previous, slot);
                                match source_ml_source1_s4 {
                                    self::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Result::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Done(firsts_2, lasts_2) => {
                                        {
                                            let positions = Vec::<f64>::new();
                                            {
                                                let at = 0f64;
                                                {
                                                    self::source_ml_temporary_plan_java_script_source_parameter_bindings_loop5(candidates, length, &(firsts_2), &(lasts_2), positions, at)
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Result::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Return(source_ml_temporary_result5) => {
                                        source_ml_temporary_result5
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
pub enum MlTemporaryContainsSourceParameterIntervalLoop1Result {
    MlTemporaryContainsSourceParameterIntervalLoop1Done,
    MlTemporaryContainsSourceParameterIntervalLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_contains_source_parameter_interval_loop1(
    firsts: &[f64],
    lasts: &[f64],
    mut at: f64,
    mut slot: f64,
) -> self::MlTemporaryContainsSourceParameterIntervalLoop1Result {
    loop {
        return if slot < (firsts.len() as f64) {
            if (at
                >= ((self::ml_array::optional_at(firsts, self::ml_array::number_index(slot)))
                    .unwrap_or_else(|| -1f64)))
                && (at
                    < ((self::ml_array::optional_at(lasts, self::ml_array::number_index(slot)))
                        .unwrap_or_else(|| -1f64)))
            {
                self::MlTemporaryContainsSourceParameterIntervalLoop1Result::MlTemporaryContainsSourceParameterIntervalLoop1Return(true)
            } else {
                {
                    let slot_2 = slot + 1f64;
                    {
                        slot = slot_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryContainsSourceParameterIntervalLoop1Result::MlTemporaryContainsSourceParameterIntervalLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Result {
    MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Done,
    MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_source_parameter_default_separator_loop2(
    kinds: &[String],
    values: &[String],
    firsts: &[f64],
    lasts: &[f64],
    mut last: f64,
    mut closes: Vec<String>,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Result {
    loop {
        return if at < last {
            if self::contains_source_parameter_interval(firsts, lasts, at)
                || (((self::ml_array::optional_at(kinds, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| String::new()))
                    != "punct")
            {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let value =
                        (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| String::new());
                    if (value == "=")
                        && !({
                            let ml_value = closes.len() as f64;
                            ml_value != 0f64 && !ml_value.is_nan()
                        })
                    {
                        self::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Result::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Return(at)
                    } else {
                        if ((value == "(") || (value == "[")) || (value == "{") {
                            {
                                let closes_2 = {
                                    let source_ml_source1_concat_value5 = closes;
                                    {
                                        let source_ml_source1_concat_value6 =
                                            vec![if value == "(" {
                                                String::from(")")
                                            } else if value == "[" {
                                                String::from("]")
                                            } else {
                                                String::from("}")
                                            }];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
                                                source_ml_source1_concat_value5,
                                            ),
                                            source_ml_source1_concat_value6,
                                        )
                                    }
                                };
                                {
                                    let at_3 = at + 1f64;
                                    {
                                        (closes, at) = (closes_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            if ((value == ")") || (value == "]")) || (value == "}") {
                                if !({
                                    let ml_value = closes.len() as f64;
                                    ml_value != 0f64 && !ml_value.is_nan()
                                }) || {
                                    let source_ml_source1_comparison_operand7 =
                                        self::ml_array::optional_at(
                                            &closes,
                                            self::ml_array::number_index(
                                                (closes.len() as f64) - 1f64,
                                            ),
                                        );
                                    {
                                        let source_ml_source1_comparison_operand8 = value.clone();
                                        !(match (
                                            source_ml_source1_comparison_operand7,
                                            source_ml_source1_comparison_operand8,
                                        ) {
                                            (Some(ml_left), ml_right) => ml_left == ml_right,
                                            _ => false,
                                        })
                                    }
                                } {
                                    self::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Result::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Return(-1f64)
                                } else {
                                    {
                                        let closes_3 = self::ml_array::slice(
                                            &closes,
                                            0f64,
                                            ((closes.len() as f64) - 1f64),
                                        );
                                        {
                                            let at_4 = at + 1f64;
                                            {
                                                (closes, at) = (closes_3, at_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at_5 = at + 1f64;
                                    {
                                        at = at_5;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Result::MlTemporaryFindJavaScriptSourceParameterDefaultSeparatorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Result {
    MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Done,
    MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_source_parameter_intervals_loop3(
    firsts: &[f64],
    lasts: &[f64],
    mut length: f64,
    mut at: f64,
) -> self::MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Result {
    loop {
        return if at < (firsts.len() as f64) {
            {
                let first = (self::ml_array::optional_at(firsts, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                {
                    let last =
                        (self::ml_array::optional_at(lasts, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| -1f64);
                    if (((!(self::ml_math::is_safe_integer(first))
                        || !(self::ml_math::is_safe_integer(last)))
                        || (first < 0f64))
                        || (last < first))
                        || (last > length)
                    {
                        self::MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Result::MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Return(false)
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
        } else {
            self::MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Result::MlTemporaryAcceptJavaScriptSourceParameterIntervalsLoop3Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_source_parameter_bindings_loop5(
    candidates: &[bool],
    mut length: f64,
    firsts: &[f64],
    lasts: &[f64],
    mut positions: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < length {
            if (self::ml_array::optional_at(candidates, self::ml_array::number_index(at)))
                .unwrap_or_else(|| false)
                && !(self::contains_source_parameter_interval(firsts, lasts, at))
            {
                {
                    let positions_2 = {
                        let source_ml_source1_concat_value9 = positions;
                        {
                            let source_ml_source1_concat_value10 = vec![at];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source1_concat_value9,
                                ),
                                source_ml_source1_concat_value10,
                            )
                        }
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (positions, at) = (positions_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 1f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { positions };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Result {
    MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Done(Vec<f64>, Vec<f64>),
    MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_source_parameter_bindings_loop6(
    firsts: &[f64],
    lasts: &[f64],
    mut first: f64,
    mut last: f64,
    mut masked: bool,
    mut range: f64,
) -> bool {
    loop {
        if range < (firsts.len() as f64) {
            if (first
                >= ((self::ml_array::optional_at(firsts, self::ml_array::number_index(range)))
                    .unwrap_or_else(|| -1f64)))
                && (last
                    <= ((self::ml_array::optional_at(lasts, self::ml_array::number_index(range)))
                        .unwrap_or_else(|| -1f64)))
            {
                {
                    let masked_2 = true;
                    {
                        let range_2 = range + 1f64;
                        {
                            (masked, range) = (masked_2, range_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let range_3 = range + 1f64;
                    {
                        range = range_3;
                        continue;
                    }
                }
            }
        }
        return { masked };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_java_script_source_parameter_bindings_loop4(
    kinds: &[String],
    values: &[String],
    default_firsts: &[f64],
    default_lasts: &[f64],
    mut firsts: Vec<f64>,
    mut lasts: Vec<f64>,
    mut previous: f64,
    mut slot: f64,
) -> self::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Result {
    loop {
        return if slot < (default_firsts.len() as f64) {
            {
                let first = (self::ml_array::optional_at(
                    default_firsts,
                    self::ml_array::number_index(slot),
                ))
                .unwrap_or_else(|| -1f64);
                {
                    let last = (self::ml_array::optional_at(
                        default_lasts,
                        self::ml_array::number_index(slot),
                    ))
                    .unwrap_or_else(|| -1f64);
                    {
                        let width = last - first;
                        if width < previous {
                            self::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Result::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Return(vec![-1f64])
                        } else {
                            {
                                let previous_2 = width;
                                {
                                    let masked = false;
                                    {
                                        let range = 0f64;
                                        {
                                            let masked_2 = self::source_ml_temporary_plan_java_script_source_parameter_bindings_loop6(&(firsts), &(lasts), first, last, masked, range);
                                            if masked_2 {
                                                {
                                                    let slot_2 = slot + 1f64;
                                                    {
                                                        (previous, slot) = (previous_2, slot_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                            {
                                                {
                                                    let equals = self::find_java_script_source_parameter_default_separator(kinds, values, &(firsts), &(lasts), first, last);
                                                    if equals < 0f64 {
                                                        self::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Result::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Return(vec![-1f64])
                                                    } else {
                                                        {
                                                            let firsts_2 = {
                                                                let source_ml_source1_concat_value11 =
                                                                    firsts;
                                                                {
                                                                    let source_ml_source1_concat_value12 =
                                                                        vec![(equals + 1f64)];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value11), source_ml_source1_concat_value12)
                                                                }
                                                            };
                                                            {
                                                                let lasts_2 = {
                                                                    let source_ml_source1_concat_value13 =
                                                                        lasts;
                                                                    {
                                                                        let source_ml_source1_concat_value14 =
                                                                            vec![last];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value13), source_ml_source1_concat_value14)
                                                                    }
                                                                };
                                                                {
                                                                    let slot_3 = slot + 1f64;
                                                                    {
                                                                        (
                                                                            firsts, lasts,
                                                                            previous, slot,
                                                                        ) = (
                                                                            firsts_2, lasts_2,
                                                                            previous_2, slot_3,
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
                }
            }
        } else {
            self::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Result::MlTemporaryPlanJavaScriptSourceParameterBindingsLoop4Done(firsts, lasts)
        };
    }
}
