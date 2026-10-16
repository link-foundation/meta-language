// Generated from the complete checked optional kernel and its recorded source dependencies.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(values: &[String], index: f64) -> String {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required text fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_text_fact())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        panic!("{}", "invalid required Boolean fact column position")
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(|| self::refuse_required_boolean_fact())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlOptionalFactFieldLoop1Result {
    MlOptionalFactFieldLoop1Done,
    MlOptionalFactFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_optional_fact_field_loop1(
    labels: &[String],
    targets: &[f64],
    field: &str,
    mut at: f64,
    mut end: f64,
) -> self::MlOptionalFactFieldLoop1Result {
    loop {
        return if at < end {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlOptionalFactFieldLoop1Result::MlOptionalFactFieldLoop1Return(
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
            self::MlOptionalFactFieldLoop1Result::MlOptionalFactFieldLoop1Done
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
pub fn ml_plan_optional_additional_absence_loop2(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut pairs: Vec<f64>,
    mut values: f64,
    mut expected: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (values >= 0f64) && (at < (self::read_required_number_fact(counts, values))) {
            {
                let other = self::optional_fact_element(starts, counts, targets, expected, at);
                {
                    let pairs_3 = if (other >= 0f64)
                        && ((self::optional_fact_tag(
                            texts, starts, counts, labels, targets, other, "kind",
                        )) != "")
                    {
                        {
                            {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        self::optional_fact_element(
                                            starts, counts, targets, values, at,
                                        ),
                                        other,
                                    ],
                                    pairs,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    } else {
                        pairs
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (pairs, at) = (pairs_3, at_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { pairs };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanOptionalAdditionalAbsenceJoin5 {
    MlPlanOptionalAdditionalAbsenceJoin5Next(Vec<f64>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_optional_additional_absence_loop4(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut pairs: Vec<f64>,
    mut expected: f64,
    mut field: f64,
    name: &str,
    mut candidate: f64,
    mut matched: bool,
) -> Vec<f64> {
    loop {
        return if (!matched && (expected >= 0f64))
            && (candidate < (self::read_required_number_fact(counts, expected)))
        {
            {
                let other =
                    self::optional_fact_element(starts, counts, targets, expected, candidate);
                {
                    let ml_s8 = if (self::optional_fact_tag(
                        texts, starts, counts, labels, targets, other, "name",
                    )) == name
                    {
                        {
                            let pairs_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        self::optional_fact_field(
                                            starts, counts, labels, targets, field, "type",
                                        ),
                                        self::optional_fact_field(
                                            starts, counts, labels, targets, other, "type",
                                        ),
                                    ],
                                    pairs,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let matched_2 = true;
                                self::MlPlanOptionalAdditionalAbsenceJoin5::MlPlanOptionalAdditionalAbsenceJoin5Next(pairs_2, matched_2)
                            }
                        }
                    } else {
                        self::MlPlanOptionalAdditionalAbsenceJoin5::MlPlanOptionalAdditionalAbsenceJoin5Next(pairs, matched)
                    };
                    match ml_s8 {
                        self::MlPlanOptionalAdditionalAbsenceJoin5::MlPlanOptionalAdditionalAbsenceJoin5Next(pairs_3, matched_3) => {
                            {
                                let candidate_2 = candidate + 1f64;
                                {
                                    (pairs, candidate, matched) = (pairs_3, candidate_2, matched_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            pairs
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
pub fn ml_plan_optional_additional_absence_loop3(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut pairs: Vec<f64>,
    mut fields: f64,
    mut expected: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if (fields >= 0f64) && (at < (self::read_required_number_fact(counts, fields))) {
            {
                let field = self::optional_fact_element(starts, counts, targets, fields, at);
                {
                    let name = self::optional_fact_tag(
                        texts, starts, counts, labels, targets, field, "name",
                    );
                    {
                        let candidate = 0f64;
                        {
                            let matched = false;
                            {
                                let pairs_2 = self::ml_plan_optional_additional_absence_loop4(
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    pairs,
                                    expected,
                                    field,
                                    &(name),
                                    candidate,
                                    matched,
                                );
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (pairs, at) = (pairs_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { pairs };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanInferenceTermIdentityLoop7Result {
    MlPlanInferenceTermIdentityLoop7Done(f64, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_inference_term_identity_loop7(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut second: f64,
    mut second_absence: String,
) -> self::MlPlanInferenceTermIdentityLoop7Result {
    loop {
        if (self::optional_fact_tag(texts, starts, counts, labels, targets, second, "kind"))
            == "option"
        {
            {
                let absence = self::optional_fact_tag(
                    texts, starts, counts, labels, targets, second, "absence",
                );
                {
                    let second_absence_3 = if absence != second_absence {
                        {
                            let second_absence_2 = String::from("both");
                            second_absence_2
                        }
                    } else {
                        second_absence.clone()
                    };
                    {
                        let second_2 = self::optional_fact_field(
                            starts, counts, labels, targets, second, "element",
                        );
                        {
                            (second, second_absence) = (second_2, second_absence_3);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlPlanInferenceTermIdentityLoop7Result::MlPlanInferenceTermIdentityLoop7Done(
                second,
                second_absence.clone(),
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanInferenceTermIdentityLoop6Result {
    MlPlanInferenceTermIdentityLoop6Done(f64, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_inference_term_identity_loop6(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut first: f64,
    mut first_absence: String,
) -> self::MlPlanInferenceTermIdentityLoop6Result {
    loop {
        if (self::optional_fact_tag(texts, starts, counts, labels, targets, first, "kind"))
            == "option"
        {
            {
                let absence = self::optional_fact_tag(
                    texts, starts, counts, labels, targets, first, "absence",
                );
                {
                    let first_absence_3 = if absence != first_absence {
                        {
                            let first_absence_2 = String::from("both");
                            first_absence_2
                        }
                    } else {
                        first_absence.clone()
                    };
                    {
                        let first_2 = self::optional_fact_field(
                            starts, counts, labels, targets, first, "element",
                        );
                        {
                            (first, first_absence) = (first_2, first_absence_3);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlPlanInferenceTermIdentityLoop6Result::MlPlanInferenceTermIdentityLoop6Done(
                first,
                first_absence.clone(),
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanInferenceTermIdentityJoin9 {
    MlPlanInferenceTermIdentityJoin9Next(Vec<f64>),
    MlPlanInferenceTermIdentityJoin9Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanInferenceTermIdentityLoop10Result {
    MlPlanInferenceTermIdentityLoop10Done(Vec<f64>),
    MlPlanInferenceTermIdentityLoop10Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_inference_term_identity_loop11(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut first: f64,
    mut at: f64,
    name: &str,
    mut ordinal: f64,
    mut prior: f64,
) -> f64 {
    loop {
        if prior < at {
            {
                let ordinal_3 = if (self::optional_fact_tag(
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    self::optional_fact_element(starts, counts, targets, first, prior),
                    "name",
                )) == name
                {
                    { ordinal + 1f64 }
                } else {
                    ordinal
                };
                {
                    let prior_2 = prior + 1f64;
                    {
                        (ordinal, prior) = (ordinal_3, prior_2);
                        continue;
                    }
                }
            }
        }
        return { ordinal };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanInferenceTermIdentityJoin13 {
    MlPlanInferenceTermIdentityJoin13Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_inference_term_identity_loop12(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut second: f64,
    mut length: f64,
    name: &str,
    mut ordinal: f64,
    mut candidate: f64,
    mut matching: f64,
) -> f64 {
    loop {
        return if (candidate < length) && (matching < 0f64) {
            {
                let other = self::optional_fact_element(starts, counts, targets, second, candidate);
                {
                    let ml_s9 = if (self::optional_fact_tag(
                        texts, starts, counts, labels, targets, other, "name",
                    )) == name
                    {
                        if ordinal < 1f64 {
                            {
                                let matching_2 = other;
                                self::MlPlanInferenceTermIdentityJoin13::MlPlanInferenceTermIdentityJoin13Next(ordinal, matching_2)
                            }
                        } else {
                            {
                                let ordinal_2 = ordinal - 1f64;
                                self::MlPlanInferenceTermIdentityJoin13::MlPlanInferenceTermIdentityJoin13Next(ordinal_2, matching)
                            }
                        }
                    } else {
                        self::MlPlanInferenceTermIdentityJoin13::MlPlanInferenceTermIdentityJoin13Next(ordinal, matching)
                    };
                    match ml_s9 {
                        self::MlPlanInferenceTermIdentityJoin13::MlPlanInferenceTermIdentityJoin13Next(ordinal_3, matching_3) => {
                            {
                                let candidate_2 = candidate + 1f64;
                                {
                                    (ordinal, candidate, matching) = (ordinal_3, candidate_2, matching_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            matching
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
pub fn ml_plan_inference_term_identity_loop10(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut pairs: Vec<f64>,
    mut first: f64,
    mut second: f64,
    mut length: f64,
    mut at: f64,
) -> self::MlPlanInferenceTermIdentityLoop10Result {
    loop {
        return if at < length {
            {
                let field = self::optional_fact_element(starts, counts, targets, first, at);
                {
                    let name = self::optional_fact_tag(
                        texts, starts, counts, labels, targets, field, "name",
                    );
                    {
                        let ordinal = 0f64;
                        {
                            let prior = 0f64;
                            {
                                let ordinal_2 = self::ml_plan_inference_term_identity_loop11(
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    first,
                                    at,
                                    &(name),
                                    ordinal,
                                    prior,
                                );
                                {
                                    let candidate = 0f64;
                                    {
                                        let matching = -1f64;
                                        {
                                            let matching_2 =
                                                self::ml_plan_inference_term_identity_loop12(
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    second,
                                                    length,
                                                    &(name),
                                                    ordinal_2,
                                                    candidate,
                                                    matching,
                                                );
                                            if matching_2 < 0f64 {
                                                self::MlPlanInferenceTermIdentityLoop10Result::MlPlanInferenceTermIdentityLoop10Return(Vec::<f64>::new())
                                            } else {
                                                {
                                                    let pairs_2 = {
                                                        let (ml_append_right, ml_append_left) = (
                                                            vec![
                                                                self::optional_fact_field(
                                                                    starts, counts, labels,
                                                                    targets, field, "type",
                                                                ),
                                                                self::optional_fact_field(
                                                                    starts, counts, labels,
                                                                    targets, matching_2, "type",
                                                                ),
                                                            ],
                                                            pairs,
                                                        );
                                                        self::ml_array::append(
                                                            ml_append_left,
                                                            ml_append_right,
                                                        )
                                                    };
                                                    {
                                                        let at_2 = at + 1f64;
                                                        {
                                                            (pairs, at) = (pairs_2, at_2);
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
        } else {
            self::MlPlanInferenceTermIdentityLoop10Result::MlPlanInferenceTermIdentityLoop10Done(
                pairs,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanInferenceTermIdentityJoin8 {
    MlPlanInferenceTermIdentityJoin8Next(Vec<f64>),
    MlPlanInferenceTermIdentityJoin8Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_plan_inference_term_identity_loop14(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut pairs: Vec<f64>,
    mut first: f64,
    mut second: f64,
    mut length: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < length {
            {
                let pairs_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::optional_fact_element(starts, counts, targets, first, at),
                            self::optional_fact_element(starts, counts, targets, second, at),
                        ],
                        pairs,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (pairs, at) = (pairs_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { pairs };
    }
}
