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
pub fn read_grammar_composition_field_order(role: &str) -> String {
    if role == "selection" {
        String::from("kind|item|items")
    } else if role == "slot" {
        String::from("kind")
    } else if role == "json-candidate" {
        String::from("kind|name|mapping")
    } else if role == "field-candidate" {
        String::from("kind|name|mapping|selector")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_composition_context_issue(
    rule_kind: &str,
    mode: &str,
    slot_kind: &str,
    tree_decorators: bool,
) -> String {
    if rule_kind == "normal" {
        if (mode != "any-grammar") && (mode != "all-grammar") {
            String::from("composition requires any-grammar or all-grammar")
        } else if slot_kind == "empty" {
            if tree_decorators {
                String::from(
                    "composition with executor or recovery decorators requires a source-witness proof",
                )
            } else {
                String::new()
            }
        } else {
            String::from("composition requires one empty authoring slot")
        }
    } else {
        String::from("composition requires a normal syntactic rule")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_composition_candidate_profile(kind: &str, mapping: &str) -> bool {
    ((kind == "grammar-field-candidate") && (mapping == "field-ranges"))
        || ((kind == "grammar-candidate") && (mapping == "json-string"))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_composition_candidate_issue(
    kinds: &[String],
    mappings: &[String],
    names: &[String],
    selectors: &[String],
) -> String {
    if ((((kinds.len() as f64) == 0f64) || ((kinds.len() as f64) != (mappings.len() as f64)))
        || ((kinds.len() as f64) != (names.len() as f64)))
        || ((kinds.len() as f64) != (selectors.len() as f64))
    {
        String::from("composition requires a nonempty complete candidate list")
    } else {
        {
            let mapping =
                (self::ml_array::optional_at(mappings, self::ml_array::number_index(0f64)))
                    .unwrap_or_default();
            if (mapping != "field-ranges") && (mapping != "json-string") {
                String::from("composition uses an unsupported authoring profile")
            } else {
                {
                    let selector = (self::ml_array::optional_at(
                        selectors,
                        self::ml_array::number_index(0f64),
                    ))
                    .unwrap_or_default();
                    {
                        let index = 0f64;
                        {
                            let ml_s1 = self::ml_read_grammar_composition_candidate_issue_loop1(
                                kinds,
                                mappings,
                                names,
                                selectors,
                                &(mapping),
                                &(selector),
                                index,
                            );
                            match ml_s1 {
                                self::MlReadGrammarCompositionCandidateIssueLoop1Result::MlReadGrammarCompositionCandidateIssueLoop1Done => {
                                    String::new()
                                }
                                self::MlReadGrammarCompositionCandidateIssueLoop1Result::MlReadGrammarCompositionCandidateIssueLoop1Return(ml_result1) => {
                                    ml_result1
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
pub fn read_grammar_composition_disposition(
    actual_mode: &str,
    actual_candidates: &str,
    requested_mode: &str,
    requested_candidates: &str,
) -> f64 {
    if (actual_mode == requested_mode) && (actual_candidates == requested_candidates) {
        1f64
    } else {
        2f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_composition_publication_issue(
    mapped_sources: bool,
    tree_decorators: bool,
) -> String {
    if mapped_sources && tree_decorators {
        String::from(
            "a mapped grammar with executor or recovery decorators requires a source-witness proof",
        )
    } else {
        String::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarCompositionCandidateIssueLoop1Result {
    MlReadGrammarCompositionCandidateIssueLoop1Done,
    MlReadGrammarCompositionCandidateIssueLoop1Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarCompositionCandidateIssueLoop3Result {
    MlReadGrammarCompositionCandidateIssueLoop3Done,
    MlReadGrammarCompositionCandidateIssueLoop3Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_composition_candidate_issue_loop3(
    names: &[String],
    mut index: f64,
    name: &str,
    mut previous: f64,
) -> self::MlReadGrammarCompositionCandidateIssueLoop3Result {
    loop {
        return if previous < index {
            if ((self::ml_array::optional_at(names, self::ml_array::number_index(previous)))
                .unwrap_or_default())
                == name
            {
                self::MlReadGrammarCompositionCandidateIssueLoop3Result::MlReadGrammarCompositionCandidateIssueLoop3Return(String::from("composition candidate source names collide"))
            } else {
                {
                    let previous_2 = previous + 1f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlReadGrammarCompositionCandidateIssueLoop3Result::MlReadGrammarCompositionCandidateIssueLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarCompositionCandidateIssueJoin2 {
    MlReadGrammarCompositionCandidateIssueJoin2Next,
    MlReadGrammarCompositionCandidateIssueJoin2Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_composition_candidate_issue_loop1(
    kinds: &[String],
    mappings: &[String],
    names: &[String],
    selectors: &[String],
    mapping: &str,
    selector: &str,
    mut index: f64,
) -> self::MlReadGrammarCompositionCandidateIssueLoop1Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let kind =
                    (self::ml_array::optional_at(kinds, self::ml_array::number_index(index)))
                        .unwrap_or_default();
                {
                    let name =
                        (self::ml_array::optional_at(names, self::ml_array::number_index(index)))
                            .unwrap_or_default();
                    {
                        let current_selector = (self::ml_array::optional_at(
                            selectors,
                            self::ml_array::number_index(index),
                        ))
                        .unwrap_or_default();
                        if name.is_empty() {
                            self::MlReadGrammarCompositionCandidateIssueLoop1Result::MlReadGrammarCompositionCandidateIssueLoop1Return(String::from("composition candidates require nonempty source names"))
                        } else {
                            let ml_library_inequality_2 = ((self::ml_array::optional_at(
                                mappings,
                                self::ml_array::number_index(index),
                            ))
                            .unwrap_or_default())
                                != mapping;
                            if ml_library_inequality_2 {
                                self::MlReadGrammarCompositionCandidateIssueLoop1Result::MlReadGrammarCompositionCandidateIssueLoop1Return(String::from("composition requires one common source mapping"))
                            } else if (self::accept_grammar_composition_candidate_profile(
                                &(kind),
                                mapping,
                            )) {
                                {
                                    let ml_s2 = if mapping == "field-ranges" {
                                        if ((selector).is_empty()) || (current_selector != selector)
                                        {
                                            self::MlReadGrammarCompositionCandidateIssueJoin2::MlReadGrammarCompositionCandidateIssueJoin2Return(String::from("composition field candidates require one common exact selector"))
                                        } else {
                                            self::MlReadGrammarCompositionCandidateIssueJoin2::MlReadGrammarCompositionCandidateIssueJoin2Next
                                        }
                                    } else {
                                        let ml_library_inequality_1 = !current_selector.is_empty();
                                        if ml_library_inequality_1 {
                                            self::MlReadGrammarCompositionCandidateIssueJoin2::MlReadGrammarCompositionCandidateIssueJoin2Return(String::from("composition JSON candidates require their exact candidate form"))
                                        } else {
                                            self::MlReadGrammarCompositionCandidateIssueJoin2::MlReadGrammarCompositionCandidateIssueJoin2Next
                                        }
                                    };
                                    match ml_s2 {
                                            self::MlReadGrammarCompositionCandidateIssueJoin2::MlReadGrammarCompositionCandidateIssueJoin2Next => {
                                                {
                                                    let previous = 0f64;
                                                    {
                                                        let ml_s3 = self::ml_read_grammar_composition_candidate_issue_loop3(names, index, &(name), previous);
                                                        match ml_s3 {
                                                            self::MlReadGrammarCompositionCandidateIssueLoop3Result::MlReadGrammarCompositionCandidateIssueLoop3Done => {
                                                                {
                                                                    let index_2 = index + 1f64;
                                                                    {
                                                                        index = index_2;
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlReadGrammarCompositionCandidateIssueLoop3Result::MlReadGrammarCompositionCandidateIssueLoop3Return(ml_result3) => {
                                                                self::MlReadGrammarCompositionCandidateIssueLoop1Result::MlReadGrammarCompositionCandidateIssueLoop1Return(ml_result3)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlReadGrammarCompositionCandidateIssueJoin2::MlReadGrammarCompositionCandidateIssueJoin2Return(ml_result3_2) => {
                                                self::MlReadGrammarCompositionCandidateIssueLoop1Result::MlReadGrammarCompositionCandidateIssueLoop1Return(ml_result3_2)
                                            }
                                        }
                                }
                            } else {
                                self::MlReadGrammarCompositionCandidateIssueLoop1Result::MlReadGrammarCompositionCandidateIssueLoop1Return(String::from("composition needs its exact candidate form"))
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarCompositionCandidateIssueLoop1Result::MlReadGrammarCompositionCandidateIssueLoop1Done
        };
    }
}
