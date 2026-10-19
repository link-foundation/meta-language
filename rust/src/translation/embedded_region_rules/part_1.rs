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
pub fn read_embedded_region_default_limit(kind: &str) -> f64 {
    if kind == "depth" {
        128f64
    } else if kind == "count" {
        4096f64
    } else if kind == "bytes" {
        16_777_216f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_embedded_region_limit(value: f64) -> bool {
    ((0f64..=9_007_199_254_740_991f64).contains(&value)) && ((value % 1f64) == 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_embedded_region_bounds(
    parent_start: f64,
    parent_end: f64,
    start: f64,
    end: f64,
) -> bool {
    (((((self::accept_embedded_region_limit(parent_start)
        && self::accept_embedded_region_limit(parent_end))
        && self::accept_embedded_region_limit(start))
        && self::accept_embedded_region_limit(end))
        && (parent_start <= start))
        && (start <= end))
        && (end <= parent_end)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_embedded_region_ancestor(
    parents: &[f64],
    languages: &[f64],
    starts: &[f64],
    ends: &[f64],
    parent: f64,
    language: f64,
    start: f64,
    end: f64,
) -> f64 {
    if ((((parents.len() as f64) != (languages.len() as f64))
        || ((starts.len() as f64) != (parents.len() as f64)))
        || ((ends.len() as f64) != (parents.len() as f64)))
        || (!(self::accept_embedded_region_limit(language))
            || !(self::accept_embedded_region_bounds(0f64, 9_007_199_254_740_991f64, start, end)))
    {
        -2f64
    } else {
        {
            let position = parent;
            {
                let ml_s1 = self::ml_read_embedded_region_ancestor_loop1(
                    parents, languages, starts, ends, language, start, end, position,
                );
                match ml_s1 {
                        self::MlReadEmbeddedRegionAncestorLoop1Result::MlReadEmbeddedRegionAncestorLoop1Done(position_2) => {
                            if position_2 == -1f64 {
                                -1f64
                            } else {
                                -2f64
                            }
                        }
                        self::MlReadEmbeddedRegionAncestorLoop1Result::MlReadEmbeddedRegionAncestorLoop1Return(ml_result1) => {
                            ml_result1
                        }
                    }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_embedded_region_disposition(
    parent_start: f64,
    parent_end: f64,
    parent_language: f64,
    language: f64,
    start: f64,
    end: f64,
    ancestor: f64,
    depth: f64,
    count: f64,
    bytes: f64,
    depth_limit: f64,
    count_limit: f64,
    byte_limit: f64,
    root_spelling_reuse: bool,
) -> String {
    if (self::accept_embedded_region_bounds(parent_start, parent_end, start, end)) {
        if ancestor == -2f64 {
            String::from("invalidEmbeddedRegionAncestry")
        } else {
            {
                let same_host = (((parent_language).partial_cmp(&(language))
                    == Some(std::cmp::Ordering::Equal))
                    && ((start).partial_cmp(&(parent_start)) == Some(std::cmp::Ordering::Equal)))
                    && ((end).partial_cmp(&(parent_end)) == Some(std::cmp::Ordering::Equal));
                if same_host && (depth != 1f64) {
                    String::from("reuseAttached")
                } else if same_host && root_spelling_reuse {
                    String::from("reuse")
                } else {
                    {
                        let legacy_root_alias =
                            (same_host && (depth == 1f64)) && !root_spelling_reuse;
                        if (ancestor >= 0f64) && !legacy_root_alias {
                            String::from("nonConsumingEmbeddingCycle")
                        } else if ((((!(self::accept_embedded_region_limit(depth))
                            || !(self::accept_embedded_region_limit(count)))
                            || !(self::accept_embedded_region_limit(bytes)))
                            || !(self::accept_embedded_region_limit(depth_limit)))
                            || !(self::accept_embedded_region_limit(count_limit)))
                            || !(self::accept_embedded_region_limit(byte_limit))
                        {
                            String::from("invalidEmbeddedRegionLimits")
                        } else if depth > depth_limit {
                            String::from("embeddedRegionDepthLimit")
                        } else if count >= count_limit {
                            String::from("embeddedRegionCountLimit")
                        } else if (bytes > byte_limit) || ((end - start) > (byte_limit - bytes)) {
                            String::from("embeddedRegionByteLimit")
                        } else {
                            String::from("parse")
                        }
                    }
                }
            }
        }
    } else {
        String::from("invalidEmbeddedRegionBounds")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn read_embedded_region_charge(
    current: f64,
    start: f64,
    end: f64,
    byte_charge: bool,
) -> f64 {
    current + if byte_charge { end - start } else { 1f64 }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_embedded_region_diagnostic_observed(
    reason: &str,
    depth: f64,
    count: f64,
    bytes: f64,
    start: f64,
    end: f64,
) -> f64 {
    if reason == "embeddedRegionDepthLimit" {
        depth
    } else if reason == "embeddedRegionCountLimit" {
        {
            let observed = self::read_embedded_region_charge(count, start, end, false);
            if self::accept_embedded_region_limit(observed) {
                observed
            } else {
                -1f64
            }
        }
    } else if reason == "embeddedRegionByteLimit" {
        {
            let observed_2 = self::read_embedded_region_charge(bytes, start, end, true);
            if self::accept_embedded_region_limit(observed_2) {
                observed_2
            } else {
                -1f64
            }
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_embedded_region_diagnostic_limit(
    reason: &str,
    depth_limit: f64,
    count_limit: f64,
    byte_limit: f64,
) -> f64 {
    if reason == "embeddedRegionDepthLimit" {
        depth_limit
    } else if reason == "embeddedRegionCountLimit" {
        count_limit
    } else if reason == "embeddedRegionByteLimit" {
        byte_limit
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_embedded_region_source_coverage(start: f64, covered_until: f64) -> bool {
    start >= covered_until
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadEmbeddedRegionAncestorLoop1Result {
    MlReadEmbeddedRegionAncestorLoop1Done(f64),
    MlReadEmbeddedRegionAncestorLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_embedded_region_ancestor_loop1(
    parents: &[f64],
    languages: &[f64],
    starts: &[f64],
    ends: &[f64],
    mut language: f64,
    mut start: f64,
    mut end: f64,
    mut position: f64,
) -> self::MlReadEmbeddedRegionAncestorLoop1Result {
    loop {
        return if position >= 0f64 {
            if !(self::accept_embedded_region_limit(position))
                || (position >= (parents.len() as f64))
            {
                self::MlReadEmbeddedRegionAncestorLoop1Result::MlReadEmbeddedRegionAncestorLoop1Return(-2f64)
            } else {
                {
                    let previous = (self::ml_array::optional_at(
                        parents,
                        self::ml_array::number_index(position),
                    ))
                    .unwrap_or(f64::NAN);
                    {
                        let selected_language = (self::ml_array::optional_at(
                            languages,
                            self::ml_array::number_index(position),
                        ))
                        .unwrap_or(f64::NAN);
                        {
                            let selected_start = (self::ml_array::optional_at(
                                starts,
                                self::ml_array::number_index(position),
                            ))
                            .unwrap_or(f64::NAN);
                            {
                                let selected_end = (self::ml_array::optional_at(
                                    ends,
                                    self::ml_array::number_index(position),
                                ))
                                .unwrap_or(f64::NAN);
                                if ((previous != -1f64)
                                    && (!(self::accept_embedded_region_limit(previous))
                                        || (previous >= position)))
                                    || (!(self::accept_embedded_region_limit(selected_language))
                                        || !(self::accept_embedded_region_bounds(
                                            0f64,
                                            9_007_199_254_740_991f64,
                                            selected_start,
                                            selected_end,
                                        )))
                                {
                                    self::MlReadEmbeddedRegionAncestorLoop1Result::MlReadEmbeddedRegionAncestorLoop1Return(-2f64)
                                } else if ((selected_language == language)
                                    && (selected_start == start))
                                    && (selected_end == end)
                                {
                                    self::MlReadEmbeddedRegionAncestorLoop1Result::MlReadEmbeddedRegionAncestorLoop1Return(position)
                                } else {
                                    {
                                        let position_2 = previous;
                                        {
                                            position = position_2;
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
            self::MlReadEmbeddedRegionAncestorLoop1Result::MlReadEmbeddedRegionAncestorLoop1Done(
                position,
            )
        };
    }
}
