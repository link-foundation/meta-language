// Generated from the complete checked optional kernel and its recorded source dependencies.
// Do not edit by hand.
use super::ml_array;
use super::ml_source_exception;
use super::optional_fact_element;
use super::optional_fact_field;
use super::optional_fact_tag;
use super::plan_inference_term_identity;
use super::plan_optional_type_constraints;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_optional_annotation_constraints(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    actual: f64,
    declared: f64,
) -> Vec<f64> {
    {
        let optional = self::plan_optional_type_constraints(
            texts, starts, counts, labels, targets, actual, declared,
        );
        if (optional.len() as f64) > 0f64 {
            vec![
                1f64,
                self::read_required_number_fact(&(optional), 0f64),
                self::read_required_number_fact(&(optional), 1f64),
            ]
        } else {
            {
                let first =
                    self::optional_fact_tag(texts, starts, counts, labels, targets, actual, "kind");
                {
                    let second = self::optional_fact_tag(
                        texts, starts, counts, labels, targets, declared, "kind",
                    );
                    if (((first == "int") || (first == "nat")) || (first == "fixed"))
                        && (((second == "int") || (second == "nat")) || (second == "fixed"))
                    {
                        vec![1f64]
                    } else {
                        self::plan_inference_term_identity(
                            texts, starts, counts, labels, targets, actual, declared,
                        )
                    }
                }
            }
        }
    }
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOptionalFactFieldLoop1Result {
    MlTemporaryOptionalFactFieldLoop1Done,
    MlTemporaryOptionalFactFieldLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_optional_fact_field_loop1(
    labels: &[String],
    targets: &[f64],
    field: &str,
    mut at: f64,
    mut end: f64,
) -> self::MlTemporaryOptionalFactFieldLoop1Result {
    loop {
        return if at < end {
            if (self::read_required_text_fact(labels, at)) == field {
                self::MlTemporaryOptionalFactFieldLoop1Result::MlTemporaryOptionalFactFieldLoop1Return(self::read_required_number_fact(targets, at))
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
            self::MlTemporaryOptionalFactFieldLoop1Result::MlTemporaryOptionalFactFieldLoop1Done
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
pub fn source_ml_temporary_plan_optional_additional_absence_loop2(
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
                        && (!(self::optional_fact_tag(
                            texts, starts, counts, labels, targets, other, "kind",
                        ))
                        .is_empty())
                    {
                        {
                            {
                                let source_ml_source1_concat_value12 = pairs;
                                {
                                    let source_ml_source1_concat_value13 = vec![
                                        self::optional_fact_element(
                                            starts, counts, targets, values, at,
                                        ),
                                        other,
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value12,
                                        ),
                                        source_ml_source1_concat_value13,
                                    )
                                }
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
pub enum MlTemporaryPlanOptionalAdditionalAbsenceJoin5 {
    MlTemporaryPlanOptionalAdditionalAbsenceJoin5Next(Vec<f64>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_optional_additional_absence_loop4(
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
                    let source_ml_source1_s16 = if (self::optional_fact_tag(
                        texts, starts, counts, labels, targets, other, "name",
                    )) == name
                    {
                        {
                            let pairs_2 = {
                                let source_ml_source1_concat_value17 = pairs;
                                {
                                    let source_ml_source1_concat_value18 = vec![
                                        self::optional_fact_field(
                                            starts, counts, labels, targets, field, "type",
                                        ),
                                        self::optional_fact_field(
                                            starts, counts, labels, targets, other, "type",
                                        ),
                                    ];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value17,
                                        ),
                                        source_ml_source1_concat_value18,
                                    )
                                }
                            };
                            {
                                let matched_2 = true;
                                self::MlTemporaryPlanOptionalAdditionalAbsenceJoin5::MlTemporaryPlanOptionalAdditionalAbsenceJoin5Next(pairs_2, matched_2)
                            }
                        }
                    } else {
                        self::MlTemporaryPlanOptionalAdditionalAbsenceJoin5::MlTemporaryPlanOptionalAdditionalAbsenceJoin5Next(pairs, matched)
                    };
                    match source_ml_source1_s16 {
                        self::MlTemporaryPlanOptionalAdditionalAbsenceJoin5::MlTemporaryPlanOptionalAdditionalAbsenceJoin5Next(pairs_3, matched_3) => {
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
pub fn source_ml_temporary_plan_optional_additional_absence_loop3(
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
                                let pairs_2 = self::source_ml_temporary_plan_optional_additional_absence_loop4(texts, starts, counts, labels, targets, pairs, expected, field, &(name), candidate, matched);
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
pub enum MlTemporaryPlanInferenceTermIdentityLoop7Result {
    MlTemporaryPlanInferenceTermIdentityLoop7Done(f64, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_inference_term_identity_loop7(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut second: f64,
    mut second_absence: String,
) -> self::MlTemporaryPlanInferenceTermIdentityLoop7Result {
    loop {
        if (self::optional_fact_tag(texts, starts, counts, labels, targets, second, "kind"))
            == "option"
        {
            {
                let absence = self::optional_fact_tag(
                    texts, starts, counts, labels, targets, second, "absence",
                );
                {
                    let second_absence_3 = if absence == second_absence {
                        second_absence
                    } else {
                        { String::from("both") }
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
            self::MlTemporaryPlanInferenceTermIdentityLoop7Result::MlTemporaryPlanInferenceTermIdentityLoop7Done(second, second_absence)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanInferenceTermIdentityLoop6Result {
    MlTemporaryPlanInferenceTermIdentityLoop6Done(f64, String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_inference_term_identity_loop6(
    texts: &[String],
    starts: &[f64],
    counts: &[f64],
    labels: &[String],
    targets: &[f64],
    mut first: f64,
    mut first_absence: String,
) -> self::MlTemporaryPlanInferenceTermIdentityLoop6Result {
    loop {
        if (self::optional_fact_tag(texts, starts, counts, labels, targets, first, "kind"))
            == "option"
        {
            {
                let absence = self::optional_fact_tag(
                    texts, starts, counts, labels, targets, first, "absence",
                );
                {
                    let first_absence_3 = if absence == first_absence {
                        first_absence
                    } else {
                        { String::from("both") }
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
            self::MlTemporaryPlanInferenceTermIdentityLoop6Result::MlTemporaryPlanInferenceTermIdentityLoop6Done(first, first_absence)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanInferenceTermIdentityJoin9 {
    MlTemporaryPlanInferenceTermIdentityJoin9Next(Vec<f64>),
    MlTemporaryPlanInferenceTermIdentityJoin9Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanInferenceTermIdentityLoop10Result {
    MlTemporaryPlanInferenceTermIdentityLoop10Done(Vec<f64>),
    MlTemporaryPlanInferenceTermIdentityLoop10Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_inference_term_identity_loop11(
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
pub enum MlTemporaryPlanInferenceTermIdentityJoin13 {
    MlTemporaryPlanInferenceTermIdentityJoin13Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_inference_term_identity_loop12(
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
                    let source_ml_source1_s19 = if (self::optional_fact_tag(
                        texts, starts, counts, labels, targets, other, "name",
                    )) == name
                    {
                        if ordinal < 1f64 {
                            {
                                let matching_2 = other;
                                self::MlTemporaryPlanInferenceTermIdentityJoin13::MlTemporaryPlanInferenceTermIdentityJoin13Next(ordinal, matching_2)
                            }
                        } else {
                            {
                                let ordinal_2 = ordinal - 1f64;
                                self::MlTemporaryPlanInferenceTermIdentityJoin13::MlTemporaryPlanInferenceTermIdentityJoin13Next(ordinal_2, matching)
                            }
                        }
                    } else {
                        self::MlTemporaryPlanInferenceTermIdentityJoin13::MlTemporaryPlanInferenceTermIdentityJoin13Next(ordinal, matching)
                    };
                    match source_ml_source1_s19 {
                        self::MlTemporaryPlanInferenceTermIdentityJoin13::MlTemporaryPlanInferenceTermIdentityJoin13Next(ordinal_3, matching_3) => {
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
pub fn source_ml_temporary_plan_inference_term_identity_loop10(
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
) -> self::MlTemporaryPlanInferenceTermIdentityLoop10Result {
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
                                let ordinal_2 =
                                    self::source_ml_temporary_plan_inference_term_identity_loop11(
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
                                            let matching_2 = self::source_ml_temporary_plan_inference_term_identity_loop12(texts, starts, counts, labels, targets, second, length, &(name), ordinal_2, candidate, matching);
                                            if matching_2 < 0f64 {
                                                self::MlTemporaryPlanInferenceTermIdentityLoop10Result::MlTemporaryPlanInferenceTermIdentityLoop10Return(Vec::<f64>::new())
                                            } else {
                                                {
                                                    let pairs_2 = {
                                                        let source_ml_source1_concat_value20 =
                                                            pairs;
                                                        {
                                                            let source_ml_source1_concat_value21 = vec![
                                                                self::optional_fact_field(
                                                                    starts, counts, labels,
                                                                    targets, field, "type",
                                                                ),
                                                                self::optional_fact_field(
                                                                    starts, counts, labels,
                                                                    targets, matching_2, "type",
                                                                ),
                                                            ];
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value20), source_ml_source1_concat_value21)
                                                        }
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
            self::MlTemporaryPlanInferenceTermIdentityLoop10Result::MlTemporaryPlanInferenceTermIdentityLoop10Done(pairs)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanInferenceTermIdentityJoin8 {
    MlTemporaryPlanInferenceTermIdentityJoin8Next(Vec<f64>),
    MlTemporaryPlanInferenceTermIdentityJoin8Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_inference_term_identity_loop14(
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
                    let source_ml_source1_concat_value22 = pairs;
                    {
                        let source_ml_source1_concat_value23 = vec![
                            self::optional_fact_element(starts, counts, targets, first, at),
                            self::optional_fact_element(starts, counts, targets, second, at),
                        ];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source1_concat_value22,
                            ),
                            source_ml_source1_concat_value23,
                        )
                    }
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
