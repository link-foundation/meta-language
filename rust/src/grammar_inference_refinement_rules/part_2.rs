// Generated from complete js/src/grammar-inference-refinement-rules.js. Do not edit by hand.
use super::accept_regular_grammar_refinement_count;
use super::appended_refinement_values;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::refinement_field;
use super::same_refinement_values;
use super::source_ml_temporary_accept_regular_grammar_refinement_identifier_loop17;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn validate_regular_grammar_refinement_history(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    models: &[f64],
    steps: &[f64],
) -> f64 {
    if !(self::accept_regular_grammar_refinement_count(steps.len() as f64))
        || ((models.len() as f64) != ((steps.len() as f64) + 1f64))
    {
        -1f64
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s6 =
                    self::source_ml_temporary_validate_regular_grammar_refinement_history_loop10(
                        forms, texts, starts, counts, labels, targets, models, steps, index,
                    );
                match source_ml_source1_s6 {
                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Done => {
                        1f64
                    }
                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(source_ml_temporary_result10) => {
                        source_ml_temporary_result10
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_regular_grammar_refinement_identifier(identifier: &str, existing: &[String]) -> bool {
    if (identifier).is_empty() {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s7 =
                    self::source_ml_temporary_accept_regular_grammar_refinement_identifier_loop17(
                        identifier, existing, index,
                    );
                match source_ml_source1_s7 {
                    self::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Result::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Done => {
                        true
                    }
                    self::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Result::MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Return(source_ml_temporary_result17) => {
                        source_ml_temporary_result17
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRefinementFieldLoop1Result {
    MlTemporaryRefinementFieldLoop1Done,
    MlTemporaryRefinementFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_refinement_field_loop1(
    labels: &[String],
    targets: &[f64],
    name: &str,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlTemporaryRefinementFieldLoop1Result {
    loop {
        return if at < (start + count) {
            if (self::read_required_text_fact(labels, at)) == name {
                self::MlTemporaryRefinementFieldLoop1Result::MlTemporaryRefinementFieldLoop1Return(
                    self::read_required_number_fact(targets, at),
                )
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryRefinementFieldLoop1Result::MlTemporaryRefinementFieldLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporarySameRefinementValuesLoop2Result {
    MlTemporarySameRefinementValuesLoop2Done,
    MlTemporarySameRefinementValuesLoop2Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySameRefinementValuesLoop3Result {
    MlTemporarySameRefinementValuesLoop3Done(Vec<f64>, Vec<f64>),
    MlTemporarySameRefinementValuesLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_same_refinement_values_loop3(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut left: Vec<f64>,
    mut right: Vec<f64>,
    mut other: f64,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlTemporarySameRefinementValuesLoop3Result {
    loop {
        return if at < (start + count) {
            {
                let label = self::read_required_text_fact(labels, at);
                {
                    let target =
                        self::refinement_field(labels, starts, counts, targets, other, &(label));
                    if target < 0f64 {
                        self::MlTemporarySameRefinementValuesLoop3Result::MlTemporarySameRefinementValuesLoop3Return(false)
                    } else {
                        {
                            let left_2 = {
                                let source_ml_source1_concat_value8 = left;
                                {
                                    let source_ml_source1_concat_value9 =
                                        vec![self::read_required_number_fact(targets, at)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value8,
                                        ),
                                        source_ml_source1_concat_value9,
                                    )
                                }
                            };
                            {
                                let right_2 = {
                                    let source_ml_source1_concat_value10 = right;
                                    {
                                        let source_ml_source1_concat_value11 = vec![target];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source1_concat_value10,
                                            ),
                                            source_ml_source1_concat_value11,
                                        )
                                    }
                                };
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (left, right, at) = (left_2, right_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporarySameRefinementValuesLoop3Result::MlTemporarySameRefinementValuesLoop3Done(left, right)
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
pub fn source_ml_temporary_same_refinement_values_loop2(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut left: Vec<f64>,
    mut right: Vec<f64>,
    mut index: f64,
) -> self::MlTemporarySameRefinementValuesLoop2Result {
    loop {
        return if index < (left.len() as f64) {
            if (index >= 100_000f64) || ((left.len() as f64) > 100_000f64) {
                self::MlTemporarySameRefinementValuesLoop2Result::MlTemporarySameRefinementValuesLoop2Return(false)
            } else {
                {
                    let one = self::read_required_number_fact(&(left), index);
                    {
                        let other = self::read_required_number_fact(&(right), index);
                        if (((one < 0f64) || (other < 0f64)) || (one >= (forms.len() as f64)))
                            || (other >= (forms.len() as f64))
                        {
                            self::MlTemporarySameRefinementValuesLoop2Result::MlTemporarySameRefinementValuesLoop2Return(false)
                        } else {
                            {
                                let form = self::read_required_text_fact(forms, one);
                                if ((form != (self::read_required_text_fact(forms, other)))
                                    || ((self::read_required_text_fact(texts, one))
                                        != (self::read_required_text_fact(texts, other))))
                                    || ((self::read_required_number_fact(counts, one))
                                        != (self::read_required_number_fact(counts, other)))
                                {
                                    self::MlTemporarySameRefinementValuesLoop2Result::MlTemporarySameRefinementValuesLoop2Return(false)
                                } else {
                                    {
                                        let start = self::read_required_number_fact(starts, one);
                                        {
                                            let count =
                                                self::read_required_number_fact(counts, one);
                                            {
                                                let at = start;
                                                {
                                                    let source_ml_source1_s12 = self::source_ml_temporary_same_refinement_values_loop3(starts, counts, labels, targets, left, right, other, start, count, at);
                                                    match source_ml_source1_s12 {
                                                        self::MlTemporarySameRefinementValuesLoop3Result::MlTemporarySameRefinementValuesLoop3Done(left_2, right_2) => {
                                                            {
                                                                let index_2 = index + 1f64;
                                                                {
                                                                    (left, right, index) = (left_2, right_2, index_2);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::MlTemporarySameRefinementValuesLoop3Result::MlTemporarySameRefinementValuesLoop3Return(source_ml_temporary_result3) => {
                                                            self::MlTemporarySameRefinementValuesLoop2Result::MlTemporarySameRefinementValuesLoop2Return(source_ml_temporary_result3)
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
            self::MlTemporarySameRefinementValuesLoop2Result::MlTemporarySameRefinementValuesLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAppendedRefinementValuesLoop4Result {
    MlTemporaryAppendedRefinementValuesLoop4Done,
    MlTemporaryAppendedRefinementValuesLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_appended_refinement_values_loop4(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut before: f64,
    mut added: f64,
    mut after: f64,
    mut previous: f64,
    mut extra: f64,
    mut index: f64,
) -> self::MlTemporaryAppendedRefinementValuesLoop4Result {
    loop {
        return if index < (previous + extra) {
            {
                let source = if index < previous { before } else { added };
                {
                    let position = if index < previous {
                        index
                    } else {
                        index - previous
                    };
                    {
                        let first = self::refinement_field(
                            labels,
                            starts,
                            counts,
                            targets,
                            source,
                            &({
                                let source_ml_source1_string_argument13 = position;
                                self::ml_number::js_number(source_ml_source1_string_argument13)
                            }),
                        );
                        {
                            let second = self::refinement_field(
                                labels,
                                starts,
                                counts,
                                targets,
                                after,
                                &({
                                    let source_ml_source1_string_argument14 = index;
                                    self::ml_number::js_number(source_ml_source1_string_argument14)
                                }),
                            );
                            if (self::same_refinement_values(
                                forms, texts, starts, counts, labels, targets, first, second,
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
                                self::MlTemporaryAppendedRefinementValuesLoop4Result::MlTemporaryAppendedRefinementValuesLoop4Return(false)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAppendedRefinementValuesLoop4Result::MlTemporaryAppendedRefinementValuesLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result {
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Done,
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Result {
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Done,
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_validate_regular_grammar_refinement_history_loop16(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut current: f64,
    mut before: f64,
    mut step: f64,
    source_ml_temporary_values9: &[String],
    mut source_ml_temporary_index9: self::ml::Big,
) -> self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Result {
    loop {
        return if source_ml_temporary_index9
            < (self::ml::Big::from_u128(source_ml_temporary_values9.len() as u128))
        {
            {
                let field = self::ml_array::at(
                    source_ml_temporary_values9,
                    source_ml_temporary_index9.to_index(),
                );
                if (self::appended_refinement_values(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    self::refinement_field(labels, starts, counts, targets, before, &(field)),
                    self::refinement_field(labels, starts, counts, targets, step, &(field)),
                    self::refinement_field(labels, starts, counts, targets, current, &(field)),
                )) {
                    {
                        let source_ml_temporary_index9_2 =
                            source_ml_temporary_index9.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index9 = source_ml_temporary_index9_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Return(-4f64)
                }
            }
        } else {
            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Result {
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Done,
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_validate_regular_grammar_refinement_history_loop15(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut current: f64,
    mut before: f64,
    source_ml_temporary_values8: &[String],
    mut source_ml_temporary_index8: self::ml::Big,
) -> self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Result {
    loop {
        return if source_ml_temporary_index8
            < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))
        {
            {
                let field = self::ml_array::at(
                    source_ml_temporary_values8,
                    source_ml_temporary_index8.to_index(),
                );
                if (self::same_refinement_values(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    self::refinement_field(labels, starts, counts, targets, before, &(field)),
                    self::refinement_field(labels, starts, counts, targets, current, &(field)),
                )) {
                    {
                        let source_ml_temporary_index8_2 =
                            source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index8 = source_ml_temporary_index8_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Return(-3f64)
                }
            }
        } else {
            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Result {
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Done,
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_validate_regular_grammar_refinement_history_loop14(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut step: f64,
    source_ml_temporary_values7: &[String],
    mut source_ml_temporary_index7: self::ml::Big,
) -> self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Result {
    loop {
        return if source_ml_temporary_index7
            < (self::ml::Big::from_u128(source_ml_temporary_values7.len() as u128))
        {
            {
                let field = self::ml_array::at(
                    source_ml_temporary_values7,
                    source_ml_temporary_index7.to_index(),
                );
                if (self::refinement_field(labels, starts, counts, targets, step, &(field))) < 0f64
                {
                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Return(-5f64)
                } else {
                    {
                        let source_ml_temporary_index7_2 =
                            source_ml_temporary_index7.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index7 = source_ml_temporary_index7_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Result {
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Done,
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_validate_regular_grammar_refinement_history_loop13(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    models: &[f64],
    mut index: f64,
    mut identity: f64,
    mut previous: f64,
) -> self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Result {
    loop {
        return if previous < index {
            {
                let other = self::refinement_field(
                    labels,
                    starts,
                    counts,
                    targets,
                    self::read_required_number_fact(models, previous),
                    "identifier",
                );
                if (self::read_required_text_fact(texts, other))
                    == (self::read_required_text_fact(texts, identity))
                {
                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Return(-2f64)
                } else {
                    {
                        let previous_2 = previous + 1f64;
                        {
                            previous = previous_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Result {
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Done,
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_validate_regular_grammar_refinement_history_loop12(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut options: f64,
    source_ml_temporary_values6: &[String],
    mut source_ml_temporary_index6: self::ml::Big,
) -> self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Result {
    loop {
        return if source_ml_temporary_index6
            < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128))
        {
            {
                let field = self::ml_array::at(
                    source_ml_temporary_values6,
                    source_ml_temporary_index6.to_index(),
                );
                if (self::refinement_field(labels, starts, counts, targets, options, &(field)))
                    < 0f64
                {
                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Return(-6f64)
                } else {
                    {
                        let source_ml_temporary_index6_2 =
                            source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index6 = source_ml_temporary_index6_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Result {
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Done,
    MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_validate_regular_grammar_refinement_history_loop11(
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut current: f64,
    source_ml_temporary_values5: &[String],
    mut source_ml_temporary_index5: self::ml::Big,
) -> self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Result {
    loop {
        return if source_ml_temporary_index5
            < (self::ml::Big::from_u128(source_ml_temporary_values5.len() as u128))
        {
            {
                let field = self::ml_array::at(
                    source_ml_temporary_values5,
                    source_ml_temporary_index5.to_index(),
                );
                if (self::refinement_field(labels, starts, counts, targets, current, &(field)))
                    < 0f64
                {
                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Return(-6f64)
                } else {
                    {
                        let source_ml_temporary_index5_2 =
                            source_ml_temporary_index5.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index5 = source_ml_temporary_index5_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Done
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
pub fn source_ml_temporary_validate_regular_grammar_refinement_history_loop10(
    forms: &[String],
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    models: &[f64],
    steps: &[f64],
    mut index: f64,
) -> self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result {
    loop {
        return if index < (models.len() as f64) {
            {
                let current = self::read_required_number_fact(models, index);
                if ((current < 0f64)
                    || ((self::read_required_text_fact(forms, current)) != "object"))
                    || ((self::read_required_number_fact(counts, current)) != 10f64)
                {
                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(-6f64)
                } else {
                    {
                        let source_ml_temporary_values5 = vec![
                            String::from("schemaVersion"),
                            String::from("algorithm"),
                            String::from("identifier"),
                            String::from("language"),
                            String::from("edition"),
                            String::from("options"),
                            String::from("positives"),
                            String::from("negatives"),
                            String::from("alphabet"),
                            String::from("table"),
                        ];
                        {
                            let source_ml_temporary_index5 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s15 = self::source_ml_temporary_validate_regular_grammar_refinement_history_loop11(starts, counts, labels, targets, current, &(source_ml_temporary_values5), source_ml_temporary_index5);
                                match source_ml_source1_s15 {
                                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Done => {
                                        {
                                            let options = self::refinement_field(labels, starts, counts, targets, current, "options");
                                            if ((self::read_required_text_fact(forms, options)) != "object") || ((self::read_required_number_fact(counts, options)) != 7f64) {
                                                self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(-6f64)
                                            } else {
                                                {
                                                    let source_ml_temporary_values6 = vec![String::from("strategy"), String::from("alpha"), String::from("stateLimit"), String::from("pairLimit"), String::from("attemptLimit"), String::from("symbolLimit"), String::from("byteLimit")];
                                                    {
                                                        let source_ml_temporary_index6 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s16 = self::source_ml_temporary_validate_regular_grammar_refinement_history_loop12(starts, counts, labels, targets, options, &(source_ml_temporary_values6), source_ml_temporary_index6);
                                                            match source_ml_source1_s16 {
                                                                self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Done => {
                                                                    {
                                                                        let identity = self::refinement_field(labels, starts, counts, targets, current, "identifier");
                                                                        if (identity < 0f64) || ((self::read_required_text_fact(forms, identity)) != "string") {
                                                                            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(-2f64)
                                                                        } else {
                                                                            {
                                                                                let previous = 0f64;
                                                                                {
                                                                                    let source_ml_source1_s17 = self::source_ml_temporary_validate_regular_grammar_refinement_history_loop13(texts, starts, counts, labels, targets, models, index, identity, previous);
                                                                                    match source_ml_source1_s17 {
                                                                                        self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Done => {
                                                                                            if index == 0f64 {
                                                                                                {
                                                                                                    let index_2 = index + 1f64;
                                                                                                    {
                                                                                                        index = index_2;
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
{
                                                                                                {
                                                                                                    let before = self::read_required_number_fact(models, (index - 1f64));
                                                                                                    {
                                                                                                        let step = self::read_required_number_fact(steps, (index - 1f64));
                                                                                                        if ((step < 0f64) || ((self::read_required_text_fact(forms, step)) != "object")) || ((self::read_required_number_fact(counts, step)) != 4f64) {
                                                                                                            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(-5f64)
                                                                                                        } else {
                                                                                                            {
                                                                                                                let source_ml_temporary_values7 = vec![String::from("identifier"), String::from("nextIdentifier"), String::from("positives"), String::from("negatives")];
                                                                                                                {
                                                                                                                    let source_ml_temporary_index7 = self::ml::Big::from_i128(0);
                                                                                                                    {
                                                                                                                        let source_ml_source1_s18 = self::source_ml_temporary_validate_regular_grammar_refinement_history_loop14(starts, counts, labels, targets, step, &(source_ml_temporary_values7), source_ml_temporary_index7);
                                                                                                                        match source_ml_source1_s18 {
                                                                                                                            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Done => {
                                                                                                                                {
                                                                                                                                    let source_ml_temporary_values8 = vec![String::from("language"), String::from("edition"), String::from("options"), String::from("algorithm"), String::from("schemaVersion")];
                                                                                                                                    {
                                                                                                                                        let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_s19 = self::source_ml_temporary_validate_regular_grammar_refinement_history_loop15(forms, texts, starts, counts, labels, targets, current, before, &(source_ml_temporary_values8), source_ml_temporary_index8);
                                                                                                                                            match source_ml_source1_s19 {
                                                                                                                                                self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Done => {
                                                                                                                                                    if !(self::same_refinement_values(forms, texts, starts, counts, labels, targets, self::refinement_field(labels, starts, counts, targets, before, "identifier"), self::refinement_field(labels, starts, counts, targets, step, "identifier"))) || !(self::same_refinement_values(forms, texts, starts, counts, labels, targets, identity, self::refinement_field(labels, starts, counts, targets, step, "nextIdentifier"))) {
                                                                                                                                                        self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(-2f64)
                                                                                                                                                    } else {
                                                                                                                                                        {
                                                                                                                                                            let source_ml_temporary_values9 = vec![String::from("positives"), String::from("negatives")];
                                                                                                                                                            {
                                                                                                                                                                let source_ml_temporary_index9 = self::ml::Big::from_i128(0);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_s20 = self::source_ml_temporary_validate_regular_grammar_refinement_history_loop16(forms, texts, starts, counts, labels, targets, current, before, step, &(source_ml_temporary_values9), source_ml_temporary_index9);
                                                                                                                                                                    match source_ml_source1_s20 {
                                                                                                                                                                        self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Done => {
                                                                                                                                                                            {
                                                                                                                                                                                let index_3 = index + 1f64;
                                                                                                                                                                                {
                                                                                                                                                                                    index = index_3;
                                                                                                                                                                                    continue;
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop16Return(source_ml_temporary_result16) => {
                                                                                                                                                                            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(source_ml_temporary_result16)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop15Return(source_ml_temporary_result16_2) => {
                                                                                                                                                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(source_ml_temporary_result16_2)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop14Return(source_ml_temporary_result16_3) => {
                                                                                                                                self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(source_ml_temporary_result16_3)
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
                                                                                        self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop13Return(source_ml_temporary_result16_4) => {
                                                                                            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(source_ml_temporary_result16_4)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop12Return(source_ml_temporary_result16_5) => {
                                                                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(source_ml_temporary_result16_5)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop11Return(source_ml_temporary_result16_6) => {
                                        self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Return(source_ml_temporary_result16_6)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Result::MlTemporaryValidateRegularGrammarRefinementHistoryLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Result {
    MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Done,
    MlTemporaryAcceptRegularGrammarRefinementIdentifierLoop17Return(bool),
}
