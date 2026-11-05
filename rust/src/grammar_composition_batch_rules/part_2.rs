// Generated from complete js/src/grammar-composition-batch-rules.js. Do not edit by hand.
use super::contains_sorted;
use super::ml_array;
use super::ml_string;
use super::read_grammar_composition_batch_text_issue;
use super::read_required_text_fact;
use super::valid_name;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_composition_batch_field_order(
    role: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*role) == (self::ml_string::Text::from_units(vec![112, 111, 108, 105, 99, 121])) {
        self::ml_string::Text::from_units(vec![
            115, 111, 117, 114, 99, 101, 124, 114, 117, 108, 101, 124, 99, 111, 109, 112, 111, 115,
            105, 116, 105, 111, 110,
        ])
    } else if (*role)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            115, 99, 104, 101, 109, 97, 86, 101, 114, 115, 105, 111, 110, 124, 109, 111, 100, 101,
            108, 124, 112, 111, 108, 105, 99, 105, 101, 115,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_composition_batch_entry_name(
    role: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*role) == (self::ml_string::Text::from_units(vec![111, 114, 105, 103, 105, 110, 97, 108])) {
        self::ml_string::Text::from_units(vec![
            103, 114, 97, 109, 109, 97, 114, 45, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111,
            110, 45, 111, 114, 105, 103, 105, 110, 97, 108, 45, 100, 101, 102, 105, 110, 105, 116,
            105, 111, 110, 115,
        ])
    } else if (*role) == (self::ml_string::Text::from_units(vec![99, 117, 114, 114, 101, 110, 116]))
    {
        self::ml_string::Text::from_units(vec![
            103, 114, 97, 109, 109, 97, 114, 45, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111,
            110, 45, 99, 117, 114, 114, 101, 110, 116, 45, 100, 101, 102, 105, 110, 105, 116, 105,
            111, 110, 115,
        ])
    } else if (*role)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105, 111, 110,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            103, 114, 97, 109, 109, 97, 114, 45, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111,
            110, 45, 98, 97, 116, 99, 104, 45, 99, 111, 110, 102, 105, 103, 117, 114, 97, 116, 105,
            111, 110,
        ])
    } else if (*role)
        == (self::ml_string::Text::from_units(vec![97, 108, 103, 111, 114, 105, 116, 104, 109]))
    {
        self::ml_string::Text::from_units(vec![
            103, 114, 97, 109, 109, 97, 114, 45, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111,
            110, 45, 98, 97, 116, 99, 104, 45, 97, 108, 103, 111, 114, 105, 116, 104, 109,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_composition_batch_configuration(
    version: f64,
    model: &self::ml_string::Text,
) -> bool {
    (version == 1f64)
        && ((*model)
            == (self::ml_string::Text::from_units(vec![
                115, 111, 117, 114, 99, 101, 45, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32,
                103, 114, 97, 109, 109, 97, 114, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105,
                111, 110, 32, 98, 97, 116, 99, 104, 32, 49,
            ])))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryContainsSortedLoop1Result {
    MlTemporaryContainsSortedLoop1Done,
    MlTemporaryContainsSortedLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_contains_sorted_loop1(
    values: &[self::ml_string::Text],
    key: &self::ml_string::Text,
    mut first: f64,
    mut last: f64,
) -> self::MlTemporaryContainsSortedLoop1Result {
    loop {
        return if first < last {
            {
                let middle = ((first + last) / 2f64).floor();
                {
                    let value = self::read_required_text_fact(values, middle);
                    if value == (*key) {
                        self::MlTemporaryContainsSortedLoop1Result::MlTemporaryContainsSortedLoop1Return(true)
                    } else {
                        if <self::ml_string::Text as std::cmp::PartialOrd>::lt(&value, key) {
                            {
                                let first_2 = middle + 1f64;
                                {
                                    first = first_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let last_2 = middle;
                                {
                                    last = last_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryContainsSortedLoop1Result::MlTemporaryContainsSortedLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadGrammarCompositionBatchIssueLoop7Result {
    MlTemporaryReadGrammarCompositionBatchIssueLoop7Done,
    MlTemporaryReadGrammarCompositionBatchIssueLoop7Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_grammar_composition_batch_issue_loop7(
    sorted_targets: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlTemporaryReadGrammarCompositionBatchIssueLoop7Result {
    loop {
        return if index < (sorted_targets.len() as f64) {
            if (self::read_required_text_fact(sorted_targets, index))
                == (self::read_required_text_fact(sorted_targets, (index - 1f64)))
            {
                self::MlTemporaryReadGrammarCompositionBatchIssueLoop7Result::MlTemporaryReadGrammarCompositionBatchIssueLoop7Return(self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 101, 32, 112, 111, 108, 105, 99, 105, 101, 115, 32, 116, 97, 114, 103, 101, 116, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 115, 111, 117, 114, 99, 101, 45, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 114, 117, 108, 101]))
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
            self::MlTemporaryReadGrammarCompositionBatchIssueLoop7Result::MlTemporaryReadGrammarCompositionBatchIssueLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadGrammarCompositionBatchIssueLoop6Result {
    MlTemporaryReadGrammarCompositionBatchIssueLoop6Done(Vec<self::ml_string::Text>),
    MlTemporaryReadGrammarCompositionBatchIssueLoop6Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_grammar_composition_batch_issue_loop6(
    policy_sources: &[self::ml_string::Text],
    policy_rules: &[self::ml_string::Text],
    compositions: &[self::ml_string::Text],
    mut units: f64,
    sorted_keys: &[self::ml_string::Text],
    normal_keys: &[self::ml_string::Text],
    mut targets: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlTemporaryReadGrammarCompositionBatchIssueLoop6Result {
    loop {
        return if index < (policy_sources.len() as f64) {
            {
                let source = self::read_required_text_fact(policy_sources, index);
                {
                    let rule = self::read_required_text_fact(policy_rules, index);
                    {
                        let composition = self::read_required_text_fact(compositions, index);
                        if (!(self::valid_name(&(source))) || !(self::valid_name(&(rule))))
                            || (composition == (self::ml_string::Text::from_units(vec![])))
                        {
                            self::MlTemporaryReadGrammarCompositionBatchIssueLoop6Result::MlTemporaryReadGrammarCompositionBatchIssueLoop6Return(self::ml_string::Text::from_units(vec![112, 111, 108, 105, 99, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 114, 117, 108, 101, 32, 110, 97, 109, 101, 115, 32, 97, 110, 100, 32, 110, 111, 110, 101, 109, 112, 116, 121, 32, 76, 105, 110, 107, 115, 32, 99, 111, 109, 112, 111, 115, 105, 116, 105, 111, 110]))
                        } else {
                            {
                                let key = (source
                                    .concat(&self::ml_string::Text::from_units(vec![0])))
                                .concat(&rule);
                                if (self::contains_sorted(sorted_keys, &(key))) {
                                    if (self::contains_sorted(normal_keys, &(key))) {
                                        {
                                            let targets_2 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![key.clone()], targets);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let units_2 =
                                                    units + (key.length() + composition.length());
                                                if units_2 > 8_388_608f64 {
                                                    self::MlTemporaryReadGrammarCompositionBatchIssueLoop6Result::MlTemporaryReadGrammarCompositionBatchIssueLoop6Return(self::ml_string::Text::from_units(vec![98, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 32, 108, 105, 109, 105, 116]))
                                                } else {
                                                    {
                                                        let index_2 = index + 1f64;
                                                        {
                                                            (units, targets, index) =
                                                                (units_2, targets_2, index_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryReadGrammarCompositionBatchIssueLoop6Result::MlTemporaryReadGrammarCompositionBatchIssueLoop6Return(self::ml_string::Text::from_units(vec![97, 32, 112, 111, 108, 105, 99, 121, 32, 116, 97, 114, 103, 101, 116, 32, 110, 101, 101, 100, 115, 32, 97, 32, 110, 111, 114, 109, 97, 108, 32, 115, 121, 110, 116, 97, 99, 116, 105, 99, 32, 114, 117, 108, 101]))
                                    }
                                } else {
                                    self::MlTemporaryReadGrammarCompositionBatchIssueLoop6Result::MlTemporaryReadGrammarCompositionBatchIssueLoop6Return(self::ml_string::Text::from_units(vec![97, 32, 112, 111, 108, 105, 99, 121, 32, 116, 97, 114, 103, 101, 116, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 101, 120, 105, 115, 116, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 45, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 114, 117, 108, 101]))
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadGrammarCompositionBatchIssueLoop6Result::MlTemporaryReadGrammarCompositionBatchIssueLoop6Done(targets)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadGrammarCompositionBatchIssueLoop5Result {
    MlTemporaryReadGrammarCompositionBatchIssueLoop5Done,
    MlTemporaryReadGrammarCompositionBatchIssueLoop5Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_grammar_composition_batch_issue_loop5(
    sorted_keys: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlTemporaryReadGrammarCompositionBatchIssueLoop5Result {
    loop {
        return if index < (sorted_keys.len() as f64) {
            if (self::read_required_text_fact(sorted_keys, index))
                == (self::read_required_text_fact(sorted_keys, (index - 1f64)))
            {
                self::MlTemporaryReadGrammarCompositionBatchIssueLoop5Result::MlTemporaryReadGrammarCompositionBatchIssueLoop5Return(self::ml_string::Text::from_units(vec![100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 45, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 114, 117, 108, 101]))
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
            self::MlTemporaryReadGrammarCompositionBatchIssueLoop5Result::MlTemporaryReadGrammarCompositionBatchIssueLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadGrammarCompositionBatchIssueLoop4Result {
    MlTemporaryReadGrammarCompositionBatchIssueLoop4Done(
        f64,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
    ),
    MlTemporaryReadGrammarCompositionBatchIssueLoop4Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_grammar_composition_batch_issue_loop4(
    owners: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut units: f64,
    sorted_sources: &[self::ml_string::Text],
    mut keys: Vec<self::ml_string::Text>,
    mut normal: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlTemporaryReadGrammarCompositionBatchIssueLoop4Result {
    loop {
        return if index < (names.len() as f64) {
            {
                let owner = self::read_required_text_fact(owners, index);
                {
                    let name = self::read_required_text_fact(names, index);
                    {
                        let kind = self::read_required_text_fact(kinds, index);
                        if !(self::valid_name(&(name)))
                            || !(self::contains_sorted(sorted_sources, &(owner)))
                        {
                            self::MlTemporaryReadGrammarCompositionBatchIssueLoop4Result::MlTemporaryReadGrammarCompositionBatchIssueLoop4Return(self::ml_string::Text::from_units(vec![97, 32, 114, 117, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 101, 120, 105, 115, 116, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 110, 97, 109, 101]))
                        } else {
                            {
                                let key = (owner
                                    .concat(&self::ml_string::Text::from_units(vec![0])))
                                .concat(&name);
                                {
                                    let keys_2 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![key.clone()], keys);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let normal_3 = if kind
                                            == (self::ml_string::Text::from_units(vec![
                                                110, 111, 114, 109, 97, 108,
                                            ])) {
                                            {
                                                {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![key.clone()], normal);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                }
                                            }
                                        } else {
                                            normal
                                        };
                                        {
                                            let units_2 = units + key.length();
                                            if units_2 > 8_388_608f64 {
                                                self::MlTemporaryReadGrammarCompositionBatchIssueLoop4Result::MlTemporaryReadGrammarCompositionBatchIssueLoop4Return(self::ml_string::Text::from_units(vec![98, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 32, 108, 105, 109, 105, 116]))
                                            } else {
                                                {
                                                    let index_2 = index + 1f64;
                                                    {
                                                        (units, keys, normal, index) =
                                                            (units_2, keys_2, normal_3, index_2);
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
        } else {
            self::MlTemporaryReadGrammarCompositionBatchIssueLoop4Result::MlTemporaryReadGrammarCompositionBatchIssueLoop4Done(units, keys, normal)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadGrammarCompositionBatchIssueLoop3Result {
    MlTemporaryReadGrammarCompositionBatchIssueLoop3Done,
    MlTemporaryReadGrammarCompositionBatchIssueLoop3Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_grammar_composition_batch_issue_loop3(
    sorted_sources: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlTemporaryReadGrammarCompositionBatchIssueLoop3Result {
    loop {
        return if index < (sorted_sources.len() as f64) {
            {
                let name = self::read_required_text_fact(sorted_sources, index);
                if !(self::valid_name(&(name)))
                    || ((index > 0f64)
                        && (name
                            == (self::read_required_text_fact(sorted_sources, (index - 1f64)))))
                {
                    self::MlTemporaryReadGrammarCompositionBatchIssueLoop3Result::MlTemporaryReadGrammarCompositionBatchIssueLoop3Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 32, 100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadGrammarCompositionBatchIssueLoop3Result::MlTemporaryReadGrammarCompositionBatchIssueLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadGrammarCompositionBatchIssueLoop2Result {
    MlTemporaryReadGrammarCompositionBatchIssueLoop2Done(f64),
    MlTemporaryReadGrammarCompositionBatchIssueLoop2Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_grammar_composition_batch_issue_loop2(
    sources: &[self::ml_string::Text],
    mut units: f64,
    mut index: f64,
) -> self::MlTemporaryReadGrammarCompositionBatchIssueLoop2Result {
    loop {
        return if index < (sources.len() as f64) {
            {
                let units_2 = units + (self::read_required_text_fact(sources, index)).length();
                {
                    let issue = self::read_grammar_composition_batch_text_issue(units_2);
                    if issue.0.is_empty() {
                        {
                            let index_2 = index + 1f64;
                            {
                                (units, index) = (units_2, index_2);
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadGrammarCompositionBatchIssueLoop2Result::MlTemporaryReadGrammarCompositionBatchIssueLoop2Return(issue.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadGrammarCompositionBatchIssueLoop2Result::MlTemporaryReadGrammarCompositionBatchIssueLoop2Done(units)
        };
    }
}
